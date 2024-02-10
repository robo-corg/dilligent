use byteorder::{LittleEndian, ReadBytesExt};
use clap::Parser;
use eyre::{eyre, Result};
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;
use std::{fs, io};

mod ast;
mod decoder;
mod interpreter;
mod opcodes;

use opcodes::OpCode;

use crate::ast::Op;
use crate::decoder::PickleReader;
use crate::interpreter::Interpreter;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Model file to load
    model_file: PathBuf,
}

fn dump_pickle(r: &mut dyn Read) -> Result<()> {
    let pickle_file: BufReader<_> = BufReader::new(r);
    let reader = PickleReader::new(pickle_file);
    let mut interp = Interpreter::new();

    for maybe_op in reader {
        let op = maybe_op?;
        //println!("{:?}", op);
        if interp.exec_op(op)? {
            break;
        }
    }

    if let Some(value) = interp.into_stop_value() {
        println!("{:#?}", value);
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let file = fs::File::open(args.model_file)?;

    let mut zip_file = zip::ZipArchive::new(file)?;

    let pickle_filenames: Vec<String> = zip_file
        .file_names()
        .filter(|name| name.ends_with(".pkl"))
        .map(|s| s.to_string())
        .collect();

    for name in pickle_filenames.into_iter() {
        println!("Found pkl: {:?}", name);
        let mut f = zip_file.by_name(&name)?;
        dump_pickle(&mut f)?;
    }

    Ok(())
}
