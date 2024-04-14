use crate::ExtraFieldWriteUtils;
use std::io::{Seek, Write};
use zip::{
    result::{ZipError, ZipResult},
    ZipWriter,
};

pub mod safe_wrapper;

impl<W: Write + Seek> ExtraFieldWriteUtils for ZipWriter<W> {
    type Error = ZipError;
    fn add_extra_field<T: crate::ExtraFieldValue>(&mut self, value: T) -> ZipResult<()> {
        value.write(self)?;
        Ok(())
    }
}
