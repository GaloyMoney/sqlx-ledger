#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CelType {
    Map,
    Int,
    UInt,
    Double,
    String,
    Bytes,
    Bool,
    Null,

    Date,
    Uuid,
}
