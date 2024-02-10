use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use byteorder::{LittleEndian, ReadBytesExt};
use eyre::{eyre, Result};

use crate::ast::Op;
use crate::opcodes::OpCode;


fn read_byte<R: io::Read>(mut r: R) -> io::Result<Option<u8>> {
    let mut buffer = [0];

    r.read(&mut buffer).map(|count| match count { 1 => Some(buffer[0]), _ => None})
}

fn read_bytes<R: io::Read>(mut r: R, amount: usize) -> io::Result<Vec<u8>> {
    let mut buf = vec![0; amount];
    r.read_exact(&mut buf)?;
    Ok(buf)
}

fn read_line_string<R: io::BufRead>(mut r: R) -> Result<String> {
    let mut buf = String::new();
    r.read_line(&mut buf)?;

    if !buf.ends_with('\n') {
        return Err(eyre!("End of file encountered before end of line"));
    }
    buf.truncate(buf.len()-1);
    Ok(buf)
}

pub struct PickleReader<R: BufRead> {
    pickle_file: R
}

impl <R: BufRead> PickleReader<R> {
    pub fn new(pickle_file: R) -> Self {
        PickleReader { pickle_file }
    }

    fn try_pase_op(&mut self, op_code: OpCode) -> Result<Op> {
        let maybe_parsed_op = match op_code {
            OpCode::Proto => {
                let version = read_byte(&mut self.pickle_file)?.ok_or(
                    eyre!("Protocol version expected")
                )?;

                Op::Proto(version)
            },
            OpCode::EmptyDict => Op::EmptyDict,
            OpCode::Binput => {
                let val = read_byte(&mut self.pickle_file)?.ok_or(
                    eyre!("Byte value expected")
                )?;

                Op::BInput(val)
            },
            OpCode::LongBinput => {
                let val = self.pickle_file.read_u32::<LittleEndian>()?;
                Op::LongBInput(val)
            },
            OpCode::Binunicode => {
                let len = self.pickle_file.read_u32::<LittleEndian>()?;
                let data = read_bytes(&mut self.pickle_file, len as usize)?;
                let s: String = String::from_utf8(data)?;
                Op::Binunicode(s)
            },
            OpCode::Global => {
                let module = read_line_string(&mut self.pickle_file)?;
                let name = read_line_string(&mut self.pickle_file)?;

                Op::Global(module, name)
            },
            OpCode::Binint => {
                let value = self.pickle_file.read_i32::<LittleEndian>()?;
                Op::BinInt(value)
            },
            OpCode::Binint1 => {
                let value = self.pickle_file.read_i8()?;
                Op::BinInt1(value)
            },
            OpCode::Binint2 => {
                let value = self.pickle_file.read_i16::<LittleEndian>()?;
                Op::BinInt2(value)
            },
            OpCode::Binget => {
                let value = self.pickle_file.read_u8()?;
                Op::BinGet(value)
            }
            OpCode::LongBinget => {
                let value = self.pickle_file.read_u32::<LittleEndian>()?;
                Op::LongBinGet(value)
            },
            OpCode::Mark => Op::Mark,
            OpCode::Tuple => Op::Tuple,
            OpCode::EmptyTuple => Op::TupleN(0),
            OpCode::EmptyList => Op::EmptyList,
            OpCode::Tuple1 => Op::TupleN(1),
            OpCode::Tuple2 => Op::TupleN(2),
            OpCode::Tuple3 => Op::TupleN(3),
            OpCode::Newfalse => Op::False,
            OpCode::Newtrue => Op::True,
            OpCode::Binpersid => Op::BinPersId,
            OpCode::Reduce => Op::Reduce,
            OpCode::Setitems => Op::SetItems,
            OpCode::Appends => Op::Appends,
            OpCode::Stop => Op::Stop,
            OpCode::Pop => todo!(),
            OpCode::PopMark => todo!(),
            OpCode::Dup => todo!(),
            OpCode::Float => todo!(),
            OpCode::Int => todo!(),
            OpCode::Long => todo!(),
            OpCode::None => todo!(),
            OpCode::Persid => todo!(),
            OpCode::String => todo!(),
            OpCode::Binstring => todo!(),
            OpCode::ShortBinstring => todo!(),
            OpCode::Unicode => todo!(),
            OpCode::Append => todo!(),
            OpCode::Build => Op::Build,
            OpCode::Dict => todo!(),
            OpCode::Get => todo!(),
            OpCode::Inst => todo!(),
            OpCode::List => todo!(),
            OpCode::Obj => todo!(),
            OpCode::Put => todo!(),
            OpCode::Setitem => Op::SetItem,
            OpCode::Binfloat => todo!(),
            OpCode::Newobj => todo!(),
            OpCode::Ext1 => todo!(),
            OpCode::Ext2 => todo!(),
            OpCode::Ext4 => todo!(),
            OpCode::Long1 => todo!(),
            OpCode::Long4 => todo!(),
            OpCode::Binbytes => todo!(),
            OpCode::ShortBinbytes => todo!(),
            OpCode::ShortBinunicode => todo!(),
            OpCode::Binunicode8 => todo!(),
            OpCode::Binbytes8 => todo!(),
            OpCode::EmptySet => todo!(),
            OpCode::Additems => todo!(),
            OpCode::Frozenset => todo!(),
            OpCode::NewobjEx => todo!(),
            OpCode::StackGlobal => todo!(),
            OpCode::Memoize => todo!(),
            OpCode::Frame => todo!(),
            OpCode::Bytearray8 => todo!(),
            OpCode::NextBuffer => todo!(),
            OpCode::ReadonlyBuffer => todo!(),
        };
        Ok(maybe_parsed_op)
    }

    fn get_next_op(&mut self) -> Result<Option<Op>> {
        let maybe_op = read_byte(&mut self.pickle_file)?;

        maybe_op.map(|op| {
            let op_code: OpCode = op.try_into()?;
            self.try_pase_op(op_code)
        }).transpose()
    }


}

impl <R: BufRead> Iterator for PickleReader<R> {
    type Item = Result<Op>;

    fn next(&mut self) -> Option<Self::Item> {
        self.get_next_op().transpose()
    }
}