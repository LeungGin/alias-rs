pub struct AliasError {
    pub err: AliasErrorCode,
    pub msg: String,
}

impl AliasError {
    pub fn new(err: AliasErrorCode, msg: String) -> Self {
        Self { err, msg }
    }
}

#[derive(Debug)]
pub enum AliasErrorCode {
    Unkonw = 0,
}
