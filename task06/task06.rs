//! Simple char device

use kernel::{
    file::{File, Operations},
    io_buffer::{IoBufferReader, IoBufferWriter},
    module_misc_device,
    prelude::*,
};

module_misc_device! {
    type: Task06,
    name: "eudyptula",
    license: "GPL",
}

struct Task06;

const ASSIGNED_ID: &'static [u8] = b"123456789012\n";

#[vtable]
impl Operations for Task06 {
    type Data = ();

    fn open(_context: &Self::OpenData, _file: &File) -> Result<Self::Data> {
        Ok(())
    }

    fn read(_data: (), _f: &File, writer: &mut impl IoBufferWriter, offset: u64) -> Result<usize> {
        let s = &ASSIGNED_ID[ASSIGNED_ID.len().min(offset as usize)..];
        writer.write_slice(s)?;
        Ok(s.len())
    }

    fn write(_data: (), _f: &File, reader: &mut impl IoBufferReader, _off: u64) -> Result<usize> {
        let mut buf = [0u8; 13]; // ID + newline
        reader.read_slice(&mut buf)?;

        if buf == ASSIGNED_ID {
            Ok(buf.len())
        } else {
            Err(EINVAL)
        }
    }
}
