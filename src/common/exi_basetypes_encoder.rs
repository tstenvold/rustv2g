use core::result::Result;
use heapless::String;

use crate::common::exi_basetypes::{ExiSigned, ExiUnsigned};
use crate::common::exi_bitstream::ExiBitstream;
use crate::common::exi_error_codes::ExiError;

fn exi_basetypes_encoder_write_unsigned(
    stream: &mut ExiBitstream,
    exi_unsigned: &ExiUnsigned,
) -> Result<(), ExiError> {
    for &octet in &exi_unsigned.octets {
        stream.write_octet(octet)?;
    }
    Ok(())
}

pub fn exi_basetypes_encoder_bool(stream: &mut ExiBitstream, value: i32) -> Result<(), ExiError> {
    let bit: u32 = u32::from(value != 0);
    stream.write_bits(1, bit)?;
    Ok(())
}

pub fn exi_basetypes_encoder_bytes(
    stream: &mut ExiBitstream,
    bytes: &[u8],
) -> Result<(), ExiError> {
    for &byte in bytes {
        stream.write_octet(byte)?;
    }
    Ok(())
}

pub fn exi_basetypes_encoder_nbit_uint(
    stream: &mut ExiBitstream,
    bit_count: usize,
    value: u32,
) -> Result<(), ExiError> {
    stream.write_bits(bit_count, value)?;
    Ok(())
}

pub fn exi_basetypes_encoder_uint_8(stream: &mut ExiBitstream, value: u8) -> Result<(), ExiError> {
    let mut exi_unsigned = ExiUnsigned::default();
    let result = u32::from(value);
    exi_unsigned.convert_to_unsigned(result, 2)?;
    exi_basetypes_encoder_write_unsigned(stream, &exi_unsigned)
}

pub fn exi_basetypes_encoder_uint_16(
    stream: &mut ExiBitstream,
    value: u16,
) -> Result<(), ExiError> {
    let mut exi_unsigned = ExiUnsigned::default();
    let result = u32::from(value);
    exi_unsigned.convert_to_unsigned(result, 3)?;
    exi_basetypes_encoder_write_unsigned(stream, &exi_unsigned)
}

pub fn exi_basetypes_encoder_uint_32(
    stream: &mut ExiBitstream,
    value: u32,
) -> Result<(), ExiError> {
    let mut exi_unsigned = ExiUnsigned::default();
    exi_unsigned.convert_to_unsigned(value, 5)?;
    exi_basetypes_encoder_write_unsigned(stream, &exi_unsigned)
}

pub fn exi_basetypes_encoder_uint_64(
    stream: &mut ExiBitstream,
    value: u64,
) -> Result<(), ExiError> {
    let mut exi_unsigned = ExiUnsigned::default();
    exi_unsigned.convert_64_to_unsigned(value)?;
    exi_basetypes_encoder_write_unsigned(stream, &exi_unsigned)
}

pub fn exi_basetypes_encoder_unsigned(
    stream: &mut ExiBitstream,
    value: &ExiUnsigned,
) -> Result<(), ExiError> {
    let mut raw_exi_unsigned = ExiUnsigned::default();
    raw_exi_unsigned.convert_bytes_to_unsigned(&value.octets)?;
    exi_basetypes_encoder_write_unsigned(stream, &raw_exi_unsigned)
}

pub fn exi_basetypes_encoder_integer_8(
    stream: &mut ExiBitstream,
    value: i8,
) -> Result<(), ExiError> {
    let sign = i32::from(value < 0);
    exi_basetypes_encoder_bool(stream, sign)?;
    let result = if value >= 0 {
        value.unsigned_abs()
    } else {
        value.unsigned_abs().wrapping_sub(1)
    };
    exi_basetypes_encoder_uint_8(stream, result)
}

pub fn exi_basetypes_encoder_integer_16(
    stream: &mut ExiBitstream,
    value: i16,
) -> Result<(), ExiError> {
    let sign = i32::from(value < 0);
    exi_basetypes_encoder_bool(stream, sign)?;
    let result = if value >= 0 {
        value.unsigned_abs()
    } else {
        value.unsigned_abs().wrapping_sub(1)
    };
    exi_basetypes_encoder_uint_16(stream, result)
}

pub fn exi_basetypes_encoder_integer_32(
    stream: &mut ExiBitstream,
    value: i32,
) -> Result<(), ExiError> {
    let sign = i32::from(value < 0);
    exi_basetypes_encoder_bool(stream, sign)?;
    let result = if value >= 0 {
        value.unsigned_abs()
    } else {
        value.unsigned_abs().wrapping_sub(1)
    };
    exi_basetypes_encoder_uint_32(stream, result)
}

pub fn exi_basetypes_encoder_integer_64(
    stream: &mut ExiBitstream,
    value: i64,
) -> Result<(), ExiError> {
    let sign: i32 = i32::from(value < 0);
    exi_basetypes_encoder_bool(stream, sign)?;
    let result = if value >= 0 {
        value.unsigned_abs()
    } else {
        value.unsigned_abs().wrapping_sub(1)
    };
    exi_basetypes_encoder_uint_64(stream, result)
}

pub fn exi_basetypes_encoder_signed(
    stream: &mut ExiBitstream,
    value: &ExiSigned,
) -> Result<(), ExiError> {
    exi_basetypes_encoder_bool(stream, i32::from(value.is_negative))?;
    exi_basetypes_encoder_unsigned(stream, &value.data)
}

pub fn exi_basetypes_encoder_characters(
    stream: &mut ExiBitstream,
    characters: &String<100>,
) -> Result<(), ExiError> {
    const ASCII_MAX_VALUE: u8 = 127;
    for &ch in characters.as_bytes() {
        let byte = ch;
        if byte > ASCII_MAX_VALUE {
            return Err(ExiError::UnsupportedCharacterValue);
        }
        stream.write_octet(byte)?;
    }
    Ok(())
}
