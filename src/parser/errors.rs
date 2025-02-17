//! Contains the error types used by the parser.

#[derive(thiserror::Error, Debug)]
pub enum NatspecParseError {
    #[error("Missing natspec tag")]
    MissingNatspecTag,
    #[error("Missing natspec description")]
    MissingNatspecDesc,
    #[error("Missing parameter name")]
    MissingParameterName,
    #[error("Missing parameter description")]
    MissingParameterDesc,
    #[error("Missing return variable name")]
    MissingReturnVarName,
    #[error("Missing return variable description")]
    MissingReturnVarDesc,
    #[error("Missing custom tag")]
    MissingCustomTag,
    #[error("Unknown tag: {0}")]
    UnknownTag(String),
}
