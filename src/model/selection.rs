use super::action;
use super::pile;

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

impl Default for Selection {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Action {
    GoTo(pile::PileId),
    Hold(pile::PileId, usize),
    Resize(usize),
    Return,
}

#[derive(Debug, snafu::Snafu)]
pub enum Error {}

impl action::Action<Selection> for Action {
    // TODO: Use Error = ! once RFC 1216 is stabilized (rust-lang/rust#35121).
    type Error = Error;

    fn apply_to(self, selection: &mut Selection) -> Result<(), Self::Error> {
        match self {
            Self::GoTo(target) => {
                selection.target = target;
            }
            Self::Hold(new_source, count) => {
                // First set the new size, in case we're told to hold zero cards.
                selection.state.resize(count, selection.target);

                // Then, if we're holding cards, override the source.
                if let State::Held { ref mut source, .. } = selection.state {
                    *source = new_source;
                }
            }
            Self::Resize(count) => {
                selection.state = selection.state.resize(count, selection.target);
            }
            Self::Return => {
                if let Some(source) = selection.state.source() {
                    selection.target = source;
                }

                selection.state = State::Visual;
            }
        }

        Ok(())
    }
}
