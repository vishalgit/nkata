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
