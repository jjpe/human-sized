//! For more information see the [`wikipedia`] article on byte sizes.
//!
//! [`wikipedia`]: https://en.wikipedia.org/wiki/Byte#Multiple-byte_units
#![allow(non_upper_case_globals)]

use std::fmt::Write as _;


// Binary constants
/// Kibibyte
pub const KiB: i128 = 1024;
/// Mebibyte
pub const MiB: i128 = 1024 * KiB;
/// Gibibyte
pub const GiB: i128 = 1024 * MiB;
/// Tebibyte
pub const TiB: i128 = 1024 * GiB;
/// Pebibyte
pub const PiB: i128 = 1024 * TiB;
/// Exbibyte
pub const EiB: i128 = 1024 * PiB;
/// Zebibyte
pub const ZiB: i128 = 1024 * EiB;
/// Yobibyte
pub const YiB: i128 = 1024 * ZiB;
/// Robibyte
pub const RiB: i128 = 1024 * YiB;
/// Quebibyte
pub const QiB: i128 = 1024 * RiB;

// Decimal constants
/// Kilobyte
pub const KB: i128 = 1000;
/// Megabyte
pub const MB: i128 = 1000 * KB;
/// Gigabyte
pub const GB: i128 = 1000 * MB;
/// Terabyte
pub const TB: i128 = 1000 * GB;
/// Petabyte
pub const PB: i128 = 1000 * TB;
/// Exabyte
pub const EB: i128 = 1000 * PB;
/// Zettabyte
pub const ZB: i128 = 1000 * EB;
/// Yottabyte
pub const YB: i128 = 1000 * ZB;
/// Ronnabyte
pub const RB: i128 = 1000 * YB;
/// Quettabyte
pub const QB: i128 = 1000 * RB;

pub fn binary(byte_size: impl Into<i128>) -> Result<String, std::fmt::Error> {
    let byte_size = byte_size.into();
    let mut buffer = String::new();
    match byte_size {
        ..KiB => write!(buffer, "{} B",   byte_size)?,
        KiB..MiB => write!(buffer, "{} KiB", byte_size / KiB)?,
        MiB..GiB => write!(buffer, "{} MiB", byte_size / MiB)?,
        GiB..TiB => write!(buffer, "{} GiB", byte_size / GiB)?,
        TiB..PiB => write!(buffer, "{} TiB", byte_size / TiB)?,
        PiB..EiB => write!(buffer, "{} PiB", byte_size / PiB)?,
        EiB..ZiB => write!(buffer, "{} EiB", byte_size / EiB)?,
        ZiB..YiB => write!(buffer, "{} ZiB", byte_size / ZiB)?,
        YiB..RiB => write!(buffer, "{} YiB", byte_size / YiB)?,
        RiB..QiB => write!(buffer, "{} RiB", byte_size / RiB)?,
        QiB..    => write!(buffer, "{} QiB", byte_size / QiB)?,
    }
    Ok(buffer)
}

pub fn decimal(byte_size: impl Into<i128>) -> Result<String, std::fmt::Error> {
    let byte_size = byte_size.into();
    let mut buffer = String::new();
    match byte_size {
        ..KB => write!(buffer, "{} B",  byte_size)?,
        KB..MB => write!(buffer, "{} KB", byte_size / KB)?,
        MB..GB => write!(buffer, "{} MB", byte_size / MB)?,
        GB..TB => write!(buffer, "{} GB", byte_size / GB)?,
        TB..PB => write!(buffer, "{} TB", byte_size / TB)?,
        PB..EB => write!(buffer, "{} PB", byte_size / PB)?,
        EB..ZB => write!(buffer, "{} EB", byte_size / EB)?,
        ZB..YB => write!(buffer, "{} ZB", byte_size / ZB)?,
        YB..RB => write!(buffer, "{} YB", byte_size / YB)?,
        RB..QB => write!(buffer, "{} RB", byte_size / RB)?,
        QB..   => write!(buffer, "{} QB", byte_size / QB)?,
    }
    Ok(buffer)
}


#[cfg(test)]
mod tests {

    #[test]
    fn binary() -> std::fmt::Result {
        let hsize = super::binary(4_400)?;
        println!("hsize={hsize}");
        assert_eq!(hsize, "4 KiB");
        Ok(())
    }

    #[test]
    fn decimal() -> std::fmt::Result {
        let hsize = super::decimal(4_812_935)?;
        println!("hsize={hsize}");
        assert_eq!(hsize, "4 MB");
        Ok(())
    }
}
