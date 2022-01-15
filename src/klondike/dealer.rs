use crate::model;

use super::game;

#[derive(Debug, Clone)]
pub struct KlondikeDealer;

impl model::dealer::Dealer<model::table::Action> for KlondikeDealer {
    type Context<'a> = KlondikeDealerContext;
    type Iter = KlondikeDealerIter;

    fn deal(&self, context: Self::Context<'_>) -> Self::Iter {
        KlondikeDealerIter::new(context.tableaux_width)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KlondikeDealerContext {
    tableaux_width: usize,
}

impl<'a, S> From<&'a game::KlondikeGame<S>> for KlondikeDealerContext
where
    S: model::deck::Shuffle,
{
    fn from(game: &'a game::KlondikeGame<S>) -> Self {
        Self {
            tableaux_width: game.settings().tableaux_width,
        }
    }
}

#[derive(Debug, Clone)]
pub struct KlondikeDealerIter {
    tableaux_width: usize,
    state: DealerIterState,
}

impl KlondikeDealerIter {
    fn new(tableaux_width: usize) -> Self {
        let state = DealerIterState::init(tableaux_width);

        Self {
            tableaux_width,
            state,
        }
    }
}

impl Iterator for KlondikeDealerIter {
    type Item = model::table::Action;

    fn next(&mut self) -> Option<Self::Item> {
        let action = self.state.action();
        self.state = self.state.next(self.tableaux_width);
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
enum DealerIterState {
    Deal(position::Dealing),
    Reveal(position::Revealing),
    Done,
}

impl DealerIterState {
    fn init(tableaux_width: usize) -> Self {
        if tableaux_width > 0 {
            Self::Deal(Default::default())
        } else {
            Self::Done
        }
    }

    fn action(&self) -> Option<model::table::Action> {
        match self {
            &Self::Deal(current_position) => {
                let pile_id = model::table::PileId::Tableaux(current_position.tableaux_index());
                Some(model::table::Action::Deal(pile_id))
            }
            &Self::Reveal(current_position) => {
                let pile_id = model::table::PileId::Tableaux(current_position.tableaux_index());
                Some(model::table::Action::Reveal(pile_id))
            }
            Self::Done => None,
        }
    }

    fn next(&self, tableaux_width: usize) -> Self {
        match self {
            Self::Deal(current_position) => current_position
                .step(tableaux_width)
                .map(Self::Deal)
                .unwrap_or_else(|| Self::Reveal(Default::default())),
            Self::Reveal(current_position) => current_position
                .step(tableaux_width)
                .map(Self::Reveal)
                .unwrap_or(Self::Done),
            Self::Done => Self::Done,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::dealer::Dealer as _;

    use assert_matches::assert_matches;
    use test_case::test_case;

    use super::*;

    #[test]
    fn dealer_should_deal_next_card() {
        let dealer = KlondikeDealer;
        let context = KlondikeDealerContext { tableaux_width: 7 };
        let mut dealer_iter = dealer.deal(context);

        assert_matches!(
            dealer_iter.next(),
            Some(model::table::Action::Deal(pile_id)) => {
                assert_eq!(pile_id, model::table::PileId::Tableaux(0));
            }
        );
    }

    #[test]
    fn dealer_should_eventually_finish() {
        let dealer = KlondikeDealer;
        let context = KlondikeDealerContext { tableaux_width: 7 };

        // Expected to deal 7 + 6 + ... + 1, then reveal 7.
        let expected_length = (1..=7).sum::<usize>() + 7;

        // Skip ahead the expected amount.
        let mut dealer_iter = dealer.deal(context).skip(expected_length);

        // Iterator should end on the next call.
        assert_matches!(dealer_iter.next(), None);
    }

    #[test]
    fn dealer_with_no_tableaux_should_finish() {
        let dealer = KlondikeDealer;
        let context = KlondikeDealerContext { tableaux_width: 0 };
        let mut dealer_iter = dealer.deal(context);

        assert_matches!(dealer_iter.next(), None);
    }

    #[test_case(1; "tableaux_width = 1")]
    #[test_case(4; "tableaux_width = 4")]
    #[test_case(7; "tableaux_width = 7")]
    #[test_case(52; "tableaux_width = 52")]
    #[test_case(1000; "tableaux_width = 1000")]
    fn dealer_should_deal_first_row(tableaux_width: usize) {
        let dealer = KlondikeDealer;
        let context = KlondikeDealerContext { tableaux_width };
        let mut dealer_iter = dealer.deal(context);

        // Should deal 0 through (tableaux_width - 1)
        for expected_index in 0..tableaux_width {
            assert_matches!(
                dealer_iter.next(),
                Some(model::table::Action::Deal(pile_id)) => {
                    assert_eq!(pile_id, model::table::PileId::Tableaux(expected_index));
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
        let dealer = KlondikeDealer;
        let context = KlondikeDealerContext { tableaux_width };
        let mut dealer_iter = dealer.deal(context);

        for expected_index in expected_indices {
            assert_matches!(
                dealer_iter.next(),
                Some(model::table::Action::Deal(pile_id)) => {
                    assert_eq!(pile_id, model::table::PileId::Tableaux(expected_index));
                }
            );
        }
    }

    #[test_case(1; "tableaux_width = 1")]
    #[test_case(4; "tableaux_width = 4")]
    #[test_case(7; "tableaux_width = 7")]
    #[test_case(52; "tableaux_width = 52")]
    #[test_case(1000; "tableaux_width = 1000")]
    fn dealer_should_reveal_top_cards(tableaux_width: usize) {
        let dealer = KlondikeDealer;
        let context = KlondikeDealerContext { tableaux_width };

        let expected_cards_dealt = (1..=tableaux_width).sum();

        // Skip the actions already matched in `dealer_should_deal_full_tableaux`.
        let mut dealer_iter = dealer.deal(context).skip(expected_cards_dealt);

        for expected_index in 0..tableaux_width {
            assert_matches!(
                dealer_iter.next(),
                Some(model::table::Action::Reveal(pile_id)) => {
                    assert_eq!(pile_id, model::table::PileId::Tableaux(expected_index));
                }
            );
        }
    }
}
