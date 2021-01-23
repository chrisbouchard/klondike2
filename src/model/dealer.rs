use std::iter;

use super::card;
use super::game;
use super::table;

#[derive(Debug)]
enum DealerState {
    Deal {
        current_index: usize,
        current_row: usize,
    },
    Reveal {
        current_index: usize,
    },
    Stock,
    Done,
}

impl DealerState {
    fn init<C>(tableaux_width: usize, card_iter: &mut iter::Peekable<C>) -> Self
    where
        C: Iterator<Item = card::Card>,
    {
        if card_iter.peek().is_some() {
            if tableaux_width > 0 {
                Self::Deal {
                    current_index: 0,
                    current_row: 0,
                }
            } else {
                Self::Stock
            }
        } else {
            Self::Done
        }
    }

    fn action<C>(&self, card_iter: &mut iter::Peekable<C>) -> Option<game::Action>
    where
        C: Iterator<Item = card::Card>,
    {
        match self {
            &Self::Deal { current_index, .. } => {
                let pile_id = table::PileId::Tableaux(current_index);
                let next_card = card_iter.next().expect("card_iter was unexpectedly empty");
                Some(game::Action::Deal(pile_id, next_card))
            }
            &Self::Reveal { current_index, .. } => {
                let pile_id = table::PileId::Tableaux(current_index);
                Some(game::Action::RevealAt(pile_id))
            }
            Self::Stock => Some(game::Action::Stock(card_iter.collect())),
            Self::Done => None,
        }
    }

    fn next<C>(&self, tableaux_width: usize, card_iter: &mut iter::Peekable<C>) -> Self
    where
        C: Iterator<Item = card::Card>,
    {
        match self {
            &Self::Deal {
                current_index,
                current_row,
                ..
            } => {
                let maybe_next_index_and_row = {
                    if card_iter.peek().is_none() {
                        None
                    } else {
                        let next_index = current_index + 1;

                        if next_index < tableaux_width {
                            Some((next_index, current_row))
                        } else {
                            let next_row = current_row + 1;

                            if next_row < tableaux_width {
                                Some((next_row, next_row))
                            } else {
                                None
                            }
                        }
                    }
                };

                if let Some((next_index, next_row)) = maybe_next_index_and_row {
                    Self::Deal {
                        current_index: next_index,
                        current_row: next_row,
                    }
                } else {
                    Self::Reveal { current_index: 0 }
                }
            }
            &Self::Reveal { current_index } => {
                let next_index = current_index + 1;

                if next_index < tableaux_width {
                    Self::Reveal {
                        current_index: next_index,
                    }
                } else {
                    Self::Stock
                }
            }
            Self::Stock => Self::Done,
            Self::Done => Self::Done,
        }
    }
}

#[derive(Debug)]
pub struct Dealer {
    tableaux_width: usize,
}

impl Dealer {
    pub fn with_tableaux_width(tableaux_width: usize) -> Self {
        Self { tableaux_width }
    }

    pub fn deal_cards<I>(&self, cards: I) -> Iter<I::IntoIter>
    where
        I: IntoIterator<Item = card::Card>,
    {
        let mut card_iter = cards.into_iter().peekable();
        let state = DealerState::init(self.tableaux_width, &mut card_iter);

        Iter {
            dealer: self,
            card_iter,
            state,
        }
    }
}

pub struct Iter<'d, C>
where
    C: Iterator,
{
    dealer: &'d Dealer,
    card_iter: iter::Peekable<C>,
    state: DealerState,
}

impl<'d, C> Iterator for Iter<'d, C>
where
    C: Iterator<Item = card::Card>,
{
    type Item = game::Action;

    fn next(&mut self) -> Option<Self::Item> {
        let action = self.state.action(&mut self.card_iter);
        self.state = self
            .state
            .next(self.dealer.tableaux_width, &mut self.card_iter);
        action
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use test_case::test_case;

    use super::super::pile;
    use super::*;

    #[test]
    fn dealer_with_nonempty_iterator_should_deal() {
        let dealer = Dealer::with_tableaux_width(7);

        let expected_card = card::Rank::Ace.of(card::Suit::Spades).face_down();

        let mut card_iter = velcro::iter![expected_card.clone()];
        let mut dealer_iter = dealer.deal_cards(&mut card_iter);

        assert_matches!(
            dealer_iter.next(),
            Some(game::Action::Deal(pile_id, actual_card)) => {
                assert_eq!(pile_id, table::PileId::Tableaux(0));
                assert_eq!(actual_card, expected_card);
            }
        );
    }

    #[test]
    fn dealer_with_empty_iterator_should_finish() {
        let dealer = Dealer::with_tableaux_width(7);
        let mut card_iter = velcro::iter![];
        let mut dealer_iter = dealer.deal_cards(&mut card_iter);
        assert_matches!(dealer_iter.next(), None);
    }

    #[test]
    fn dealer_with_no_tableaux_should_place_stock_and_finish() {
        let dealer = Dealer::with_tableaux_width(0);
        let mut card_iter = card::Card::values_face_down();
        let mut dealer_iter = dealer.deal_cards(&mut card_iter);

        let expected_stock = card::Card::values_face_down().collect::<pile::Pile>();

        assert_matches!(
            dealer_iter.next(),
            Some(game::Action::Stock(actual_stock)) => {
                assert_eq!(actual_stock, expected_stock);
            }
        );

        assert_matches!(dealer_iter.next(), None);
    }

    #[test_case(1; "tableaux_width = 1")]
    #[test_case(4; "tableaux_width = 4")]
    #[test_case(7; "tableaux_width = 7")]
    #[test_case(52; "tableaux_width = 52")]
    #[test_case(1000; "tableaux_width = 1000")]
    fn dealer_should_deal_first_row(tableaux_width: usize) {
        let dealer = Dealer::with_tableaux_width(tableaux_width);

        let mut card_iter = card::Card::values_face_down();
        let mut dealer_iter = dealer.deal_cards(&mut card_iter);

        let expected_cards = card::Card::values_face_down();

        // Should deal 0 through (tableaux_width - 1), but will stop short if we run out of cards.
        for (expected_card, expected_index) in expected_cards.zip(0..tableaux_width) {
            assert_matches!(
                dealer_iter.next(),
                Some(game::Action::Deal(pile_id, actual_card)) => {
                    assert_eq!(pile_id, table::PileId::Tableaux(expected_index));
                    assert_eq!(actual_card, expected_card);
                }
            );
        }
    }

    #[test_case(1, vec![0]; "tableaux_width = 1")]
    #[test_case(4, velcro::vec![
        0, 1, 2, 3, 1, 2, 3, 2, 3, 3,
    ]; "tableaux_width = 4")]
    #[test_case(7, velcro::vec![
        ..(0..=6), ..(1..=6), ..(2..=6), ..(3..=6), ..(4..=6), ..(5..=6), 6,
    ]; "tableaux_width = 7")]
    #[test_case(25, velcro::vec![
        ..(0..=24),  //   25 cards
        ..(1..=24),  // + 24 cards
        2, 3, 4,     // +  3 cards  =  52 cards total
    ]; "tableaux_width = 25")]
    #[test_case(52, velcro::vec![..(0..=51)]; "tableaux_width = 52")]
    #[test_case(1000, velcro::vec![..(0..=51)]; "tableaux_width = 1000")]
    fn dealer_should_deal_full_tableaux(tableaux_width: usize, expected_indices: Vec<usize>) {
        let dealer = Dealer::with_tableaux_width(tableaux_width);

        let mut card_iter = card::Card::values_face_down();
        let mut dealer_iter = dealer.deal_cards(&mut card_iter);

        let expected_cards = card::Card::values_face_down();

        for (expected_card, expected_index) in expected_cards.zip(expected_indices) {
            assert_matches!(
                dealer_iter.next(),
                Some(game::Action::Deal(pile_id, actual_card)) => {
                    assert_eq!(pile_id, table::PileId::Tableaux(expected_index));
                    assert_eq!(actual_card, expected_card);
                }
            );
        }
    }

    #[test_case(1, 1; "tableaux_width = 1")]
    #[test_case(4, 10; "tableaux_width = 4")]
    #[test_case(7, 28; "tableaux_width = 7")]
    #[test_case(25, 52; "tableaux_width = 25")]
    #[test_case(52, 52; "tableaux_width = 52")]
    #[test_case(1000, 52; "tableaux_width = 1000")]
    fn dealer_should_reveal_top_cards(
        tableaux_width: usize,
        // If there are enough cards for a full tableaux, the number of dealt cards should be
        // (tableaux) * (tableaux_width + 1) / 2 (the sum from 1 to tableaux_width). Otherwise it
        // should cap out at the deck size (52).
        dealt_card_count: usize,
    ) {
        let dealer = Dealer::with_tableaux_width(tableaux_width);

        let mut card_iter = card::Card::values_face_down();
        // Skip the actions already matched in `dealer_should_deal_full_tableaux`.
        let mut dealer_iter = dealer.deal_cards(&mut card_iter).skip(dealt_card_count);

        for expected_index in 0..tableaux_width {
            assert_matches!(
                dealer_iter.next(),
                Some(game::Action::RevealAt(pile_id)) => {
                    assert_eq!(pile_id, table::PileId::Tableaux(expected_index));
                }
            );
        }
    }

    #[test_case(1, 1; "tableaux_width = 1")]
    #[test_case(4, 10; "tableaux_width = 4")]
    #[test_case(7, 28; "tableaux_width = 7")]
    #[test_case(25, 52; "tableaux_width = 25")]
    #[test_case(52, 52; "tableaux_width = 52")]
    #[test_case(1000, 52; "tableaux_width = 1000")]
    fn dealer_should_place_remaining_stock_and_finish(
        tableaux_width: usize,
        // If there are enough cards for a full tableaux, the number of dealt cards should be
        // (tableaux) * (tableaux_width + 1) / 2 (the sum from 1 to tableaux_width). Otherwise it
        // should cap out at the deck size (52).
        dealt_card_count: usize,
    ) {
        let dealer = Dealer::with_tableaux_width(tableaux_width);

        let mut card_iter = card::Card::values_face_down();
        let mut dealer_iter = dealer
            .deal_cards(&mut card_iter)
            // Skip the Deal actions already matched in `dealer_should_deal_full_tableaux` and
            // the RevealAt actions matched in `dealer_should_deal_reveal_top_cards`.
            .skip(dealt_card_count + tableaux_width);

        // Skip the cards already dealt in `dealer_should_deal_full_tableaux`.
        let expected_stock = card::Card::values_face_down()
            .skip(dealt_card_count)
            .collect::<pile::Pile>();

        assert_matches!(
            dealer_iter.next(),
            Some(game::Action::Stock(actual_stock)) => {
                assert_eq!(actual_stock, expected_stock);
            }
        );

        assert_matches!(dealer_iter.next(), None);
    }
}
