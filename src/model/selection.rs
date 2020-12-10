use super::action;
use super::pile;

#[derive(Debug, Copy, Clone)]
pub enum SelectionAction {
    Cancel,
    Resize(usize),
    Move(pile::PileId),
    Place,
}

#[derive(Debug, Copy, Clone)]
enum SelectionState {
    Held {
        source: pile::PileId,
        extra_count: usize,
    },
    Visual,
}

impl SelectionState {
    fn source(&self) -> Option<pile::PileId> {
        match self {
            Self::Held { source, .. } => Some(*source),
            Self::Visual => None,
        }
    }

    fn count(&self) -> usize {
        match self {
            Self::Held { extra_count, .. } => *extra_count + 1,
            Self::Visual => 0,
        }
    }

    fn resize(self, new_count: usize, current_target: pile::PileId) -> Self {
        if new_count == 0 {
            Self::Visual
        } else {
            Self::Held {
                source: self.source().unwrap_or(current_target),
                extra_count: new_count - 1,
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Selection {
    target: pile::PileId,
    state: SelectionState,
}

impl Selection {
    pub const fn new() -> Self {
        Self {
            target: pile::PileId::Stock,
            state: SelectionState::Visual,
        }
    }

    pub fn target(&self) -> pile::PileId {
        self.target
    }

    pub fn source(&self) -> Option<pile::PileId> {
        self.state.source()
    }

    pub fn count(&self) -> usize {
        self.state.count()
    }
}

impl action::Actionable for Selection {
    type Action = SelectionAction;

    fn apply(&mut self, action: Self::Action) {
        match action {
            Self::Action::Cancel => {
                if let Some(source) = self.source() {
                    self.target = source;
                }

                self.state = SelectionState::Visual;
            }
            Self::Action::Resize(count) => {
                self.state = self.state.resize(count, self.target);
            }
            Self::Action::Move(target) => self.target = target,
            Self::Action::Place => self.state = SelectionState::Visual,
        }
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self::new()
    }
}
