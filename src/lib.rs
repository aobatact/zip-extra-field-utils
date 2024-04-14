#[cfg(feature = "zip_crate")]
pub use zip_crate::*;
#[cfg(feature = "zip_crate")]
mod zip_crate;

pub mod extra_fields;

pub trait ExtraFieldWriteUtils {
    type Error;
    fn add_extra_field<T: ExtraFieldValue>(&mut self, value: T) -> Result<(), Self::Error>;
}

pub trait ExtraFieldValue {
    fn info(&self) -> ExtraFieldInfo;
    fn write_data<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error>;
    fn write<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        let info = self.info();
        w.write_all(&info.header_id())?;
        w.write_all(&info.data_size())?;
        self.write_data(w)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct ExtraFieldInfo {
    header_id: [u8; 2],
    data_size: [u8; 2],
}

impl ExtraFieldInfo {
    pub fn new(header_id: u16, data_size: u16) -> Self {
        Self {
            header_id: header_id.to_le_bytes(),
            data_size: data_size.to_le_bytes(),
        }
    }

    pub fn header_id(&self) -> [u8; 2] {
        self.header_id
    }

    pub fn data_size(&self) -> [u8; 2] {
        self.data_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
