use super::action;
use super::pile;

#[derive(Debug, Copy, Clone)]
pub enum SelectionAction {
    Cancel,
    DecreaseCount(usize),
    IncreaseCount(usize),
    Move(pile::PileId),
    Place,
}

#[derive(Debug, Copy, Clone)]
pub enum SelectionMode {
    Held {
        source: pile::PileId,
        extra_count: usize,
    },
    Visual,
}

#[derive(Debug, Clone)]
pub struct Selection {
    target: pile::PileId,
    mode: SelectionMode,
}

impl Selection {
    pub const fn new() -> Self {
        Self {
            target: pile::PileId::Stock,
            mode: SelectionMode::Visual,
        }
    }

    pub fn target(&self) -> pile::PileId {
        self.target
    }

    pub fn source(&self) -> Option<pile::PileId> {
        match self.mode {
            SelectionMode::Held { source, .. } => Some(source),
            SelectionMode::Visual => None,
        }
    }

    pub fn count(&self) -> usize {
        match self.mode {
            SelectionMode::Held { extra_count, .. } => extra_count + 1,
            SelectionMode::Visual => 0,
        }
    }
}

impl action::Actionable for Selection {
    type Action = SelectionAction;

    fn apply(&mut self, action: Self::Action) {
        match action {
            Self::Action::Cancel => todo!(),
            Self::Action::DecreaseCount(count) => todo!(),
            Self::Action::IncreaseCount(count) => todo!(),
            Self::Action::Move(target) => todo!(),
            Self::Action::Place => todo!(),
        }
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self::new()
    }
}
