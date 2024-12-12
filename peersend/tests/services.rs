use services::help::HelpService;
use services::version::VersionService;
use services::jwt::TokenHandler;

#[cfg(test)]
pub mod help_service_tests {
    use super::*;

    #[test]
    pub fn run_noparamater_helpnotavailable() {
        let result = HelpService::run().is_ok();
        assert_eq!(result, true);
    }
}

#[cfg(test)]
pub mod version_service_tests {
    use super::*;

    #[test]
    pub fn run_noparamater_crateversionreturned() {
        let result = VersionService::run();
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), "v0.1.0".to_string())
    }
}

#[cfg(test)]
pub mod token_handler_tests {
    use super::*;

    #[test]
    pub fn generate_withuserid_tokenreturned() {
        let result = TokenHandler::generate("test".to_string());
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3MzQwODYwNTAsImlhdCI6MTczMzk5OTY1MCwiaXNzIjoicGVlcnNlbmQiLCJzdWIiOiIxMjM0NTYifQ.cQ0IIwCtB59KQBTr5x44pWV_dyJgqowJuJPR9BrZ00Q".to_string())
    }
}
