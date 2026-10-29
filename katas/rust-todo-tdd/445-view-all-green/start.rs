use thiserror::Error;

use super::{Actor, Role};
use crate::shared::UserId;

/// Everything that needs permission, carrying the facts the decision depends on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    EditTodo { owner: UserId },
    DeleteTodo { owner: UserId },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[error("not allowed")]
pub struct Forbidden;

pub fn authorize(actor: &Actor, action: Action) -> Result<(), Forbidden> {
    let owns = |owner: UserId| owner == actor.id;
    let allowed = match (actor.role, action) {
        (_, Action::EditTodo { owner }) => owns(owner),
        (Role::Admin, Action::DeleteTodo { .. }) => true,
        (Role::User, Action::DeleteTodo { owner }) => owns(owner),
    };
    if allowed { Ok(()) } else { Err(Forbidden) }
}

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

    #[test]
    fn deleting_is_for_owners_and_admins() {
        let (user, admin) = (actor(1, Role::User), actor(2, Role::Admin));
        assert!(authorize(&user, Action::DeleteTodo { owner: user.id }).is_ok());
        assert!(
            authorize(
                &user,
                Action::DeleteTodo {
                    owner: someone_else()
                }
            )
            .is_err()
        );
        assert!(
            authorize(
                &admin,
                Action::DeleteTodo {
                    owner: someone_else()
                }
            )
            .is_ok()
        );
    }

    #[test]
    fn only_admins_may_view_all_todos() {
        assert!(authorize(&actor(1, Role::User), Action::ViewAllTodos).is_err());
        assert!(authorize(&actor(2, Role::Admin), Action::ViewAllTodos).is_ok());
    }
}
