#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roles_round_trip_through_their_storage_names() {
        for role in [Role::User, Role::Admin] {
            assert_eq!(Role::parse(role.as_str()), Some(role));
        }
        assert_eq!(Role::parse("superuser"), None);
    }
}
