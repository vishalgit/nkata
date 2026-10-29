#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn actor(n: u128, role: Role) -> Actor {
        Actor {
            id: UserId::from_uuid(Uuid::from_u128(n)),
            role,
        }
    }
    fn someone_else() -> UserId {
        UserId::from_uuid(Uuid::from_u128(99))
    }

    #[test]
    fn only_owners_may_edit_whatever_their_role() {
        for who in [actor(1, Role::User), actor(2, Role::Admin)] {
            assert!(
                authorize(&who, Action::EditTodo { owner: who.id }).is_ok(),
                "{:?} own",
                who.role
            );
            assert!(
                authorize(
                    &who,
                    Action::EditTodo {
                        owner: someone_else()
                    }
                )
                .is_err(),
                "{:?} other",
                who.role
            );
        }
    }
}
