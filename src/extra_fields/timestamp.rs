use crate::{ExtraFieldInfo, ExtraFieldValue};
use seal::NullOrTimestamp;
pub use seal::ToTimpStamp;

mod seal {
    pub trait Sealed {}
    pub trait NullOrTimestamp: Sealed + Copy {
        const HAS_VALUE: bool;
        const FLAG: u8 = Self::HAS_VALUE as u8;
        fn value(self) -> u64;
    }
    pub trait ToTimpStamp: Copy {
        fn value(self) -> u64;
    }
    impl ToTimpStamp for u64 {
        #[inline]
        fn value(self) -> u64 {
            self
        }
    }
    #[cfg(feature = "chrono")]
    impl<Tz> ToTimpStamp for chrono::DateTime<Tz>
    where
        Tz: chrono::TimeZone,
        chrono::DateTime<Tz>: Copy,
    {
        fn value(self) -> u64 {
            self.timestamp() as u64
        }
    }

    impl Sealed for () {}
    impl<T: ToTimpStamp> Sealed for T {}
    impl NullOrTimestamp for () {
        const HAS_VALUE: bool = false;
        fn value(self) -> u64 {
            0
        }
    }
    impl<T: ToTimpStamp> NullOrTimestamp for T {
        const HAS_VALUE: bool = true;
        #[inline]
        fn value(self) -> u64 {
            self.value()
        }
    }
}

#[derive(Debug)]
pub struct ExtendedTimestampExtraField<Mod, Ac, Cr> {
    modification: Mod,
    access: Ac,
    creation: Cr,
}

impl<Mod, Ac, Cr> ExtendedTimestampExtraField<Mod, Ac, Cr>
where
    Mod: NullOrTimestamp,
    Ac: NullOrTimestamp,
    Cr: NullOrTimestamp,
{
    const FLAG: u8 = (Mod::FLAG/* << 0 */) + (Ac::FLAG << 1) + (Cr::FLAG << 2);
    const SIZE: u16 = (Mod::FLAG + Ac::FLAG + Cr::FLAG) as u16 * 4 + 1;

    pub fn modification<Mod2: ToTimpStamp>(
        self,
        value: Mod2,
    ) -> ExtendedTimestampExtraField<Mod2, Ac, Cr> {
        ExtendedTimestampExtraField {
            modification: value,
            access: self.access,
            creation: self.creation,
        }
    }

    pub fn access<Ac2: ToTimpStamp>(self, value: Ac2) -> ExtendedTimestampExtraField<Mod, Ac2, Cr> {
        ExtendedTimestampExtraField {
            modification: self.modification,
            access: value,
            creation: self.creation,
        }
    }

    pub fn creation<Cr2: ToTimpStamp>(
        self,
        value: Cr2,
    ) -> ExtendedTimestampExtraField<Mod, Ac, Cr2> {
        ExtendedTimestampExtraField {
            modification: self.modification,
            access: self.access,
            creation: value,
        }
    }
}

impl ExtendedTimestampExtraField<(), (), ()> {
    pub fn new() -> Self {
        Self {
            modification: (),
            access: (),
            creation: (),
        }
    }
}

impl ExtendedTimestampExtraField<u64, (), ()> {
    pub fn new_central_header(modification: u64) -> Self {
        Self {
            modification,
            access: (),
            creation: (),
        }
    }
}

impl ExtendedTimestampExtraField<u64, u64, u64> {
    pub fn new_local_header(modification: u64, access: u64, creation: u64) -> Self {
        Self {
            modification,
            access,
            creation,
        }
    }
}

impl<Mod, Ac, Cr> ExtraFieldValue for ExtendedTimestampExtraField<Mod, Ac, Cr>
where
    Mod: NullOrTimestamp,
    Ac: NullOrTimestamp,
    Cr: NullOrTimestamp,
{
    fn info(&self) -> ExtraFieldInfo {
        ExtraFieldInfo::new(0x5455, Self::SIZE)
    }

    fn write_data<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        w.write_all(&[Self::FLAG])?;
        if Mod::HAS_VALUE {
            w.write_all(&self.modification.value().to_le_bytes())?;
        }
        if Ac::HAS_VALUE {
            w.write_all(&self.access.value().to_le_bytes())?;
        }
        if Cr::HAS_VALUE {
            w.write_all(&self.creation.value().to_le_bytes())?;
        }

        Ok(())
    }
}
