use super::action;
use super::pile;

#[derive(Debug, Copy, Clone)]
pub enum Action {
    Cancel,
    Replace(pile::PileId, usize),
    Resize(usize),
    Move(pile::PileId),
    Place,
}

#[derive(Debug, Copy, Clone)]
enum State {
    Held {
        source: pile::PileId,
        extra_count: usize,
    },
    Visual,
}

impl State {
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
    state: State,
}

impl Selection {
    pub const fn new() -> Self {
        Self {
            target: pile::PileId::Stock,
            state: State::Visual,
        }
    }

    pub fn target(&self) -> pile::PileId {
        self.target
    }

    pub fn source(&self) -> pile::PileId {
        self.state.source().unwrap_or(self.target)
    }

    pub fn count(&self) -> usize {
        self.state.count()
    }
}

impl action::Actionable for Selection {
    type Action = Action;

    fn apply(&mut self, action: Self::Action) {
        match action {
            Self::Action::Cancel => {
                if let Some(source) = self.state.source() {
                    self.target = source;
                }

                self.state = State::Visual;
            }
            Self::Action::Replace(new_source, count) => {
                // First set the new size, in case we're told to hold zero cards.
                self.state.resize(count, self.target);

                // Then, if we're holding cards, override the source.
                if let State::Held { ref mut source, .. } = self.state {
                    *source = new_source;
                }
            }
            Self::Action::Resize(count) => {
                self.state = self.state.resize(count, self.target);
            }
            Self::Action::Move(target) => {
                self.target = target;
            }
            Self::Action::Place => {
                self.state = State::Visual;
            }
        }
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self::new()
    }
}
