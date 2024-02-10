// https://github.com/python/cpython/blob/main/Lib/pickle.py

// // macro_rules! opcodes {
// //     (($name:ident, $value:expr)) => {
// //         pub const $name: u8 = $value;
// //     };
// //     (($name:ident, $value:expr), $(($rest_name:ident, $rest_value:expr)),+) => {
// //         opcodes!(($name, $value));
// //         opcodes!($(($rest_name, $rest_value)),+);
// //     };
// // }

// macro_rules! opcodes {
//     ($($e:tt),+) => {
//         //opcodes_consts!($($e),+);
//         enum OpCode {
//             opcodes_enum!($($e),+);
//         }
//     };
// }

// macro_rules! opcodes_enum {
//     (($name:ident, $value:expr)) => {
//         $name = $value,
//     };
//     (($name:ident, $value:expr), $($e:tt),+) => {
//         opcodes_enum!(($name, $value));
//         opcodes_enum!($($e),+);
//     };
// }

// macro_rules! opcodes_consts {
//     (($name:ident, $value:expr)) => {
//         pub const $name: u8 = $value;
//     };
//     (($name:ident, $value:expr), $($e:tt),+) => {
//         opcodes_consts!(($name, $value));
//         opcodes_consts!($($e),+);
//     };
// }

// opcodes!(
//     (MARK, b'('),
//     (STOP, b'.'),
//     (POP, b'0'),
//     (POP_MARK, b'1'),
//     (DUP, b'2'),
//     (FLOAT, b'F'),
//     (INT, b'I'),
//     (BININT, b'J'),
//     (BININT1, b'K'),
//     (LONG, b'L'),
//     (BININT2, b'M'),
//     (NONE, b'N'),
//     (PERSID, b'P'),
//     (BINPERSID, b'Q'),
//     (REDUCE, b'R'),
//     (STRING, b'S'),
//     (BINSTRING, b'T'),
//     (SHORT_BINSTRING, b'U'),
//     (UNICODE, b'V'),
//     (BINUNICODE, b'X'),
//     (APPEND, b'a'),
//     (BUILD, b'b'),
//     (GLOBAL, b'c'),
//     (DICT, b'd'),
//     (EMPTY_DICT, b'}'),
//     (APPENDS, b'e'),
//     (GET, b'g'),
//     (BINGET, b'h'),
//     (INST, b'i'),
//     (LONG_BINGET, b'j'),
//     (LIST, b'l'),
//     (EMPTY_LIST, b']'),
//     (OBJ, b'o'),
//     (PUT, b'p'),
//     (BINPUT, b'q'),
//     (LONG_BINPUT, b'r'),
//     (SETITEM, b's'),
//     (TUPLE, b't'),
//     (EMPTY_TUPLE, b')'),
//     (SETITEMS, b'u'),
//     (BINFLOAT, b'G'),
//     (PROTO, b'\x80'),
//     (NEWOBJ, b'\x81'),
//     (EXT1, b'\x82'),
//     (EXT2, b'\x83'),
//     (EXT4, b'\x84'),
//     (TUPLE1, b'\x85'),
//     (TUPLE2, b'\x86'),
//     (TUPLE3, b'\x87'),
//     (NEWTRUE, b'\x88'),
//     (NEWFALSE, b'\x89'),
//     (LONG1, b'\x8a'),
//     (LONG4, b'\x8b'),
//     (BINBYTES, b'B'),
//     (SHORT_BINBYTES, b'C'),
//     (SHORT_BINUNICODE, b'\x8c'),
//     (BINUNICODE8, b'\x8d'),
//     (BINBYTES8, b'\x8e'),
//     (EMPTY_SET, b'\x8f'),
//     (ADDITEMS, b'\x90'),
//     (FROZENSET, b'\x91'),
//     (NEWOBJ_EX, b'\x92'),
//     (STACK_GLOBAL, b'\x93'),
//     (MEMOIZE, b'\x94'),
//     (FRAME, b'\x95'),
//     (BYTEARRAY8, b'\x96'),
//     (NEXT_BUFFER, b'\x97'),
//     (READONLY_BUFFER, b'\x98')
// );

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub(crate) enum OpCode {
    Mark = b'(',
    Stop = b'.',
    Pop = b'0',
    PopMark = b'1',
    Dup = b'2',
    Float = b'F',
    Int = b'I',
    Binint = b'J',
    Binint1 = b'K',
    Long = b'L',
    Binint2 = b'M',
    None = b'N',
    Persid = b'P',
    Binpersid = b'Q',
    Reduce = b'R',
    String = b'S',
    Binstring = b'T',
    ShortBinstring = b'U',
    Unicode = b'V',
    Binunicode = b'X',
    Append = b'a',
    Build = b'b',
    Global = b'c',
    Dict = b'd',
    EmptyDict = b'}',
    Appends = b'e',
    Get = b'g',
    Binget = b'h',
    Inst = b'i',
    LongBinget = b'j',
    List = b'l',
    EmptyList = b']',
    Obj = b'o',
    Put = b'p',
    Binput = b'q',
    LongBinput = b'r',
    Setitem = b's',
    Tuple = b't',
    EmptyTuple = b')',
    Setitems = b'u',
    Binfloat = b'G',
    Proto = b'\x80',
    Newobj = b'\x81',
    Ext1 = b'\x82',
    Ext2 = b'\x83',
    Ext4 = b'\x84',
    Tuple1 = b'\x85',
    Tuple2 = b'\x86',
    Tuple3 = b'\x87',
    Newtrue = b'\x88',
    Newfalse = b'\x89',
    Long1 = b'\x8a',
    Long4 = b'\x8b',
    Binbytes = b'B',
    ShortBinbytes = b'C',
    ShortBinunicode = b'\x8c',
    Binunicode8 = b'\x8d',
    Binbytes8 = b'\x8e',
    EmptySet = b'\x8f',
    Additems = b'\x90',
    Frozenset = b'\x91',
    NewobjEx = b'\x92',
    StackGlobal = b'\x93',
    Memoize = b'\x94',
    Frame = b'\x95',
    Bytearray8 = b'\x96',
    NextBuffer = b'\x97',
    ReadonlyBuffer = b'\x98',
}
