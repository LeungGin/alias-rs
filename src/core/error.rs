pub struct AliasError {
    pub kind: ErrorKind,
    pub msg: String,
}

#[derive(Debug)]
pub enum ErrorKind {
    Unkonw = 0,
}
