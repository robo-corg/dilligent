#[derive(Debug)]
pub enum Op {
    Proto(u8),
    Appends,
    EmptyDict,
    EmptyList,
    Mark,
    BInput(u8),
    LongBInput(u32),
    Binunicode(String),
    Global(String, String),
    BinInt(i32),
    BinInt1(i8),
    BinInt2(i16),
    BinGet(u8),
    BinPersId,
    LongBinGet(u32),
    Tuple,
    TupleN(u8),
    True,
    False,
    Reduce,
    SetItems,
    SetItem,
    Build,
    Stop
}