use uuid::Uuid;

/// Identifies one todo item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TodoId(Uuid);

impl TodoId {
    pub const fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    pub const fn into_uuid(self) -> Uuid {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn an_id_round_trips_its_uuid() {
        let uuid = Uuid::from_u128(42);
        assert_eq!(TodoId::from_uuid(uuid).into_uuid(), uuid);
    }
}
