
pub mod session {
    use chrono::{DateTime, TimeZone, Utc};

    pub enum SessionStatus {
        Active,
        Expired,
        Invalid,
        RequiresReauth
    }

    pub enum SessionError {
        ExpiredSession,
        InvalidUser,
        FilePermissionError,
        SessionCreationError,
        AuthenticationRequired
    }

    pub struct Session {
        session_id: String,
        user_login: String,
        // created_at: chrono::DatetTime,
        // expires_at: chrono::DatetTime,
        // last_activity: chrono::DatetTime,
        is_active: bool
    }


    impl Session {
        fn new() -> Self {
            todo!();
        }

        fn is_valid(&self) {
            todo!();
        }

        fn refresh(&self) {
            todo!();
        }

        fn expire(&self) {
            todo!();
        }

        fn save(&self) {
            todo!();
        }

        fn load(&self) {
            todo!();
        }


        fn validate_user(&self) {
            todo!();
        }

        fn check_expiry(&self) {
            todo!();
        }

        fn update_last_activity(&self) {
            todo!();
        }

    }

}