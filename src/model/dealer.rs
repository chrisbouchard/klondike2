use std::iter;

use super::card;
use super::game;
use super::table;

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
        Iter::new(self.tableaux_width, cards)
    }
}

pub struct Iter<C>
where
    C: Iterator,
{
    tableaux_width: usize,
    card_iter: iter::Peekable<C>,
    state: DealerState,
}

impl<C> Iter<C>
where
    C: Iterator<Item = card::Card>,
{
    fn new<I>(tableaux_width: usize, cards: I) -> Self
    where
        I: IntoIterator<Item = card::Card, IntoIter = C>,
    {
        let mut card_iter = cards.into_iter().peekable();
        let state = DealerState::init(tableaux_width, &mut card_iter);

        Self {
            tableaux_width,
            card_iter,
            state,
        }
    }
}

impl<C> Iterator for Iter<C>
where
    C: Iterator<Item = card::Card>,
{
    type Item = game::Action;

    fn next(&mut self) -> Option<Self::Item> {
        let action = self.state.action(&mut self.card_iter);
        self.state = self.state.next(self.tableaux_width, &mut self.card_iter);
        action
    }
}

mod position {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
    pub struct Dealing {
        column: usize,
        row: usize,
    }

    impl Dealing {
        pub fn step(self, tableaux_width: usize) -> Option<Self> {
            let row_width = tableaux_width - self.row;
            let next_column = (self.column + 1) % row_width;
            let next_row = self.row + (self.column + 1) / row_width;

            if next_row < tableaux_width {
                Some(Self {
                    column: next_column,
                    row: next_row,
                })
            } else {
                None
            }
        }

        pub fn tableaux_index(self) -> usize {
            self.column + self.row
        }
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
    pub struct Revealing {
        index: usize,
    }

    impl Revealing {
        pub fn step(self, tableaux_width: usize) -> Option<Self> {
            let next_index = self.index + 1;

            if next_index < tableaux_width {
                Some(Self { index: next_index })
            } else {
                None
            }
        }

        pub fn tableaux_index(self) -> usize {
            self.index
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;

        use super::*;

        #[test_case(Dealing { column: 0, row: 0}, 4 => Some(Dealing { column: 1, row: 0}))]
        #[test_case(Dealing { column: 1, row: 0}, 4 => Some(Dealing { column: 2, row: 0}))]
        #[test_case(Dealing { column: 2, row: 0}, 4 => Some(Dealing { column: 3, row: 0}))]
        #[test_case(Dealing { column: 3, row: 0}, 4 => Some(Dealing { column: 0, row: 1}))]
        #[test_case(Dealing { column: 0, row: 1}, 4 => Some(Dealing { column: 1, row: 1}))]
        #[test_case(Dealing { column: 1, row: 1}, 4 => Some(Dealing { column: 2, row: 1}))]
        #[test_case(Dealing { column: 2, row: 1}, 4 => Some(Dealing { column: 0, row: 2}))]
        #[test_case(Dealing { column: 0, row: 2}, 4 => Some(Dealing { column: 1, row: 2}))]
        #[test_case(Dealing { column: 1, row: 2}, 4 => Some(Dealing { column: 0, row: 3}))]
        #[test_case(Dealing { column: 0, row: 3}, 4 => None)]
        fn dealing_step(position: Dealing, tableaux_width: usize) -> Option<Dealing> {
            position.step(tableaux_width)
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum DealerState {
    Deal(position::Dealing),
    Reveal(position::Revealing),
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
                Self::Deal(Default::default())
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
            &Self::Deal(current_position) => {
                let pile_id = table::PileId::Tableaux(current_position.tableaux_index());
                let next_card = card_iter.next().expect("card_iter was unexpectedly empty");
                Some(game::Action::Deal(pile_id, next_card))
            }
            &Self::Reveal(current_position) => {
                let pile_id = table::PileId::Tableaux(current_position.tableaux_index());
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
            Self::Deal(current_position) => card_iter
                .peek()
                .and_then(|_| current_position.step(tableaux_width))
                .map(Self::Deal)
                .unwrap_or_else(|| Self::Reveal(Default::default())),
            Self::Reveal(current_position) => current_position
                .step(tableaux_width)
                .map(Self::Reveal)
                .unwrap_or(Self::Stock),
            Self::Stock => Self::Done,
            Self::Done => Self::Done,
        }
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
