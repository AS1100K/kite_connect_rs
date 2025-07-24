use serde::{Deserialize, Serialize};

/// Represents a generic API response that can either be a success or an error.
///
/// # Variants
///
/// - `Success { data }`: Indicates a successful response containing the data of type `T`.
/// - `Error { message, error_type }`: Indicates an error response with a message and an error type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "status")]
pub enum Response<T> {
    /// Success response containing the data.
    Success { data: T },
    /// Error response containing an error message and error type.
    Error { message: String, error_type: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize, PartialEq)]
    struct EmptyType {}

    #[test]
    fn test_error_response() -> Result<(), Box<dyn std::error::Error>> {
        let err_str = r#"{
            "status": "error",
            "message": "Error message",
            "error_type": "GeneralException"
            }"#;

        let res: Response<EmptyType> = serde_json::from_str(err_str)?;
        let expected = Response::Error {
            message: "Error message".to_string(),
            error_type: "GeneralException".to_string(),
        };

        assert_eq!(res, expected);

        Ok(())
    }

    #[test]
    fn test_success_response() -> Result<(), Box<dyn std::error::Error>> {
        let err_str = r#"{
            "status": "success",
            "data": {}
            }"#;

        let res: Response<EmptyType> = serde_json::from_str(err_str)?;
        let expected = Response::Success { data: EmptyType {} };

        assert_eq!(res, expected);

        Ok(())
    }
}
