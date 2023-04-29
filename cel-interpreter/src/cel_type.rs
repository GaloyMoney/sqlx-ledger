#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CelType {
    Map,
    Array,
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
