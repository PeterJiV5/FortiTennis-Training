#[cfg(test)]
mod tests {
    use tui_coach::utils::error::{AppError, Result};
    use std::io;

    #[test]
    fn test_app_error_not_found_display() {
        let error = AppError::NotFound("user".to_string());
        assert_eq!(error.to_string(), "Not found: user");
    }

    #[test]
    fn test_app_error_unauthorized_display() {
        let error = AppError::Unauthorized("invalid credentials".to_string());
        assert_eq!(
            error.to_string(),
            "Unauthorized: invalid credentials"
        );
    }

    #[test]
    fn test_app_error_validation_display() {
        let error = AppError::Validation("email format is invalid".to_string());
        assert_eq!(
            error.to_string(),
            "Validation error: email format is invalid"
        );
    }

    #[test]
    fn test_app_error_other_display() {
        let error = AppError::Other("something went wrong".to_string());
        assert_eq!(error.to_string(), "Error: something went wrong");
    }

    #[test]
    fn test_app_error_io_from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let error: AppError = io_err.into();

        match error {
            AppError::Io(_) => assert!(true),
            _ => panic!("Expected Io error variant"),
        }
    }

    #[test]
    fn test_app_error_debug_format() {
        let error = AppError::NotFound("resource".to_string());
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("NotFound"));
        assert!(debug_str.contains("resource"));
    }

    #[test]
    fn test_result_type_ok() {
        let result: Result<i32> = Ok(42);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_result_type_err() {
        let result: Result<i32> = Err(AppError::Validation("test error".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_error_trait_implementation() {
        let error = AppError::Other("test error".to_string());
        let _: &dyn std::error::Error = &error;
    }
}
