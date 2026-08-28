use uuid::Uuid;

use crate::current_user::CurrentUser::{Guest, User};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentUser {
    Guest,
    User(Uuid),
}

impl CurrentUser {
    pub fn new(current_user_id: &str) -> Self {
        match current_user_id.parse().ok() {
            Some(user_id) => User(user_id),
            None => Guest,
        }
    }

    pub fn get_user_id<E>(&self, err: E) -> Result<Uuid, E> {
        match self {
            Guest => Err(err),
            User(uuid) => Ok(*uuid),
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::current_user::CurrentUser;

    #[test]
    fn test_parse_random_string() {
        let current_user = CurrentUser::new("aaaa");

        assert_eq!(current_user, CurrentUser::Guest);
    }

    #[test]
    fn test_parse_uuid_string() {
        let current_user = CurrentUser::new(&Uuid::nil().to_string());

        assert_eq!(current_user, CurrentUser::User(Uuid::nil()));
    }

    #[test]
    fn test_parse_empty_string() {
        let current_user = CurrentUser::new("");

        assert_eq!(current_user, CurrentUser::Guest);
    }
}
