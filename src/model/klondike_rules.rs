use enum_like::EnumValues as _;
use itertools::Itertools as _;

use super::card;
use super::game;
use super::rules;
use super::table;

#[derive(Clone, Copy, Debug)]
pub struct KlondikeRuleOptions {
    pub allow_move_from_foundation: bool,
    pub tableaux_width: usize,
}

#[derive(Clone, Debug)]
pub struct KlondikeRules {
    options: KlondikeRuleOptions,
}

impl KlondikeRules {
    pub const fn new(options: KlondikeRuleOptions) -> Self {
        Self { options }
    }
}

impl rules::Rules<table::Action, table::Table> for KlondikeRules {
    type State<'a> = game::GameState<'a>;

    fn valid_actions(&self, state: Self::State<'_>) -> Vec<table::Action> {
        if state.started {
            vec![]
        } else {
            velcro::vec![..self.pregame_actions()]
        }
    }
}

impl KlondikeRules {
    fn pregame_actions(&self) -> impl IntoIterator<Item = table::Action> {
        (0..self.options.tableaux_width)
            .map(table::PileId::Tableaux)
            .cartesian_product(card::Facing::values())
            .map(|(target_pile_id, facing)| table::Action::Deal(target_pile_id, facing))
    }
}
