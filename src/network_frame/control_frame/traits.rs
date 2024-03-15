/// Constantly offer length of data fragment for serialization constant
pub trait Len {
    fn len() -> u16;
}

/// Check response status, true for success
pub trait CheckStatus {
    /// Check response status, true for success
    fn check_status(&self) -> anyhow::Result<()>;
}
