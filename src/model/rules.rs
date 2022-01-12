use super::table;

#[derive(Clone, Copy, Debug)]
pub struct RuleOptions {
    pub allow_move_from_foundation: bool,
    pub tableaux_width: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct RuleState<'a> {
    pub started: bool,
    pub table: &'a table::Table,
}

#[derive(Clone, Debug)]
pub struct Rules {
    options: RuleOptions,
}

impl Rules {
    pub const fn new(options: RuleOptions) -> Self {
        Self { options }
    }

    pub fn valid_actions(&self, state: RuleState<'_>) -> Vec<table::Action> {
        if state.started {
            vec![]
        } else {
            velcro::vec![..self.pregame_actions()]
        }
    }

    pub fn is_valid_action(&self, state: RuleState<'_>, action: &table::Action) -> bool {
        self.valid_actions(state).into_iter().any(|a| a.eq(action))
    }

    fn pregame_actions(&self) -> impl IntoIterator<Item = table::Action> {
        (0..self.options.tableaux_width)
            .map(table::PileId::Tableaux)
            .map(table::Action::Deal)
    }
}
