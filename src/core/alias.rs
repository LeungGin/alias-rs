use super::{alias_setting::AliasSetting, error::AliasError};

pub trait Alias {
    fn init(&self) -> Result<(), AliasError>;
    fn setting(&self) -> AliasSetting;
    fn set(&self, alias: String, command: String) -> Result<(), AliasError>;
    fn remove(&self, alias: String) -> Result<(), AliasError>;
    fn list(&self) -> Result<Option<Vec<String>>, AliasError>;
}
