#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CelType {
    // Builtins
    Map,
    Array,
    Int,
    UInt,
    Double,
    String,
    Bytes,
    Bool,
    Null,

    // Addons
    Date,
    Uuid,
    Decimal,
}
