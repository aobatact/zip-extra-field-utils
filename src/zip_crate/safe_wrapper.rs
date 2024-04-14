use crate::ExtraFieldWriteUtils;
use std::io::{Seek, Write};
use zip::{
    result::{ZipError, ZipResult},
    ZipWriter,
};

/// Wrapper for [`ZipWriter`] to prevent illegal operation.
pub trait ZipSafeExt {
    type W: Write + Seek;
    fn start_file_with_extra_data_guard(&mut self) -> ZipExtraDataWriter<Self::W, false>;
}

/// Wrapper for [`ZipWriter`] to write extra data.
pub struct ZipExtraDataWriter<'a, W: Write + Seek, const IS_CENTRAL: bool> {
    writer: &'a mut ZipWriter<W>,
}

impl<W: Write + Seek> ZipSafeExt for ZipWriter<W> {
    type W = W;
    fn start_file_with_extra_data_guard(&mut self) -> ZipExtraDataWriter<W, false> {
        ZipExtraDataWriter { writer: self }
    }
}

impl<W: Write + Seek, const IS_CENTRAL: bool> ExtraFieldWriteUtils
    for ZipExtraDataWriter<'_, W, IS_CENTRAL>
{
    type Error = ZipError;

    fn add_extra_field<T: crate::ExtraFieldValue>(&mut self, value: T) -> Result<(), Self::Error> {
        self.writer.add_extra_field(value)
    }
}

impl<'a, W: Write + Seek> ZipExtraDataWriter<'a, W, false> {
    pub fn end_local_start_central_extra_data(
        &'a mut self,
    ) -> ZipResult<ZipExtraDataWriter<'a, W, true>> {
        self.writer.end_local_start_central_extra_data()?;
        Ok(ZipExtraDataWriter {
            writer: self.writer,
        })
    }
}

impl<'a, W: Write + Seek, const IS_CENTRAL: bool> ZipExtraDataWriter<'a, W, IS_CENTRAL> {
    pub fn end_extra_data(&'a mut self) -> ZipResult<&'a mut ZipWriter<W>> {
        let writer = &mut self.writer;
        writer.end_extra_data()?;
        Ok(writer)
    }
}
