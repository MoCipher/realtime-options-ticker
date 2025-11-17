
enum OptionsError {
    ApiError(String),
    ParseError(String),
    NetworkError(String),
    Other(String),
}

impl std::fmt::Display for OptionsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptionsError::ApiError(msg) => write!(f, "API Error: {}", msg),
            OptionsError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            OptionsError::NetworkError(msg) => write!(f, "Network Error: {}", msg),
            OptionsError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for OptionsError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn display_variants() {
        let api = OptionsError::ApiError("bad".into());
        assert_eq!(api.to_string(), "API Error: bad");

        let parse = OptionsError::ParseError("bad parse".into());
        assert_eq!(parse.to_string(), "Parse Error: bad parse");

        let net = OptionsError::NetworkError("timeout".into());
        assert_eq!(net.to_string(), "Network Error: timeout");

        let other = OptionsError::Other("oops".into());
        assert_eq!(other.to_string(), "Error: oops");
    }

    #[test]
    fn implements_error_trait_and_has_no_source() {
        let e = OptionsError::Other("x".into());
        let _err_obj: &dyn Error = &e;
        assert!(e.source().is_none());
    }

    fn assert_send_sync<T: Send + Sync>() {}
    #[test]
    fn is_send_and_sync() {
        assert_send_sync::<OptionsError>();
    }
}