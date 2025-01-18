use crate::core::{alias::Alias, error::AliasError};
use std::rc::Rc;

pub trait AliasImport {
    fn import(&self) -> Result<(), AliasError>;
}

pub struct AliasImporter {
    alias: Rc<Box<dyn Alias>>,
}

impl AliasImporter {
    pub fn new(alias: Rc<Box<dyn Alias>>) -> Result<Self, AliasError> {
        Ok(Self {
            alias: alias.clone(),
        })
    }
}

impl AliasImport for AliasImporter {
    fn import(&self) -> Result<(), AliasError> {
        for (alias, command) in &self.alias.setting().aliases {
            self.alias.set(alias.clone(), command.clone())?;
        }
        Ok(())
    }
}
