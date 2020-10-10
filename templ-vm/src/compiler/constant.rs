#[derive(Clone, Debug, PartialEq)]
pub enum Constant {
    Raw(String),
    String(String),
}
