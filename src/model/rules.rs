use super::{settings, table};

#[derive(Debug, Clone, Copy)]
pub struct RuleState<'a> {
    pub settings: &'a settings::Settings,
    pub started: bool,
    pub table: &'a table::Table,
}

#[derive(Debug, Clone, Default)]
pub struct Rules;

impl Rules {
    pub fn valid_actions(&self, state: RuleState<'_>) -> Vec<table::Action> {
        if state.started {
            vec![]
        } else {
            velcro::vec![..self.pregame_actions(state)]
        }
    }

    pub fn is_valid_action(&self, state: RuleState<'_>, action: &table::Action) -> bool {
        self.valid_actions(state).into_iter().any(|a| a.eq(action))
    }

    fn pregame_actions(&self, state: RuleState<'_>) -> impl IntoIterator<Item = table::Action> {
        (0..state.settings.tableaux_width)
            .map(table::PileId::Tableaux)
            .map(table::Action::Deal)
    }
}
