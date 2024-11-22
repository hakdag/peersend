use services::help::HelpService;
use services::version::VersionService;

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
