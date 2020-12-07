use super::pile;

pub enum Selection {
    Held {
        count: usize,
        source_id: pile::PileId,
        target_id: pile::PileId,
    },
    Visual {
        target_id: pile::PileId,
    },
}

impl Selection {
    pub fn is_held(&self) -> bool {
        matches!(self, Self::Held { .. })
    }

    pub fn is_visual(&self) -> bool {
        matches!(self, Self::Visual { .. })
    }
}
