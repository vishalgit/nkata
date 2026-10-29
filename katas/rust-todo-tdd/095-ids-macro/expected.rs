use uuid::Uuid;

macro_rules! id_type {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(Uuid);

        impl $name {
            pub const fn from_uuid(id: Uuid) -> Self {
                Self(id)
            }

            pub const fn into_uuid(self) -> Uuid {
                self.0
            }
        }

        impl std::str::FromStr for $name {
            type Err = uuid::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Uuid::parse_str(s).map(Self)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

id_type! {
    /// Identifies one todo item.
    TodoId
}

id_type! {
    /// Identifies one registered user.
    UserId
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

    #[test]
    fn an_id_parses_from_its_string_form() {
        let raw = "67e55044-10b1-426f-9247-bb680e5fe0c8";
        let id: TodoId = raw.parse().unwrap();
        assert_eq!(id.to_string(), raw);
    }

    #[test]
    fn user_ids_behave_the_same_way() {
        let raw = "67e55044-10b1-426f-9247-bb680e5fe0c8";
        let id: UserId = raw.parse().unwrap();
        assert_eq!(id.to_string(), raw);
    }
}
