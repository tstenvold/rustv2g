use core::result::Result;
use heapless::String;

use crate::common::exi_basetypes::{ExiSigned, ExiUnsigned, exi_basetypes_convert_64_to_unsigned, exi_basetypes_convert_bytes_to_unsigned, exi_basetypes_convert_to_unsigned};
use crate::common::exi_bitstream::{ExiBitstream, exi_bitstream_write_bits, exi_bitstream_write_octet};
use crate::common::exi_error_codes::ExiError;

fn exi_basetypes_encoder_write_unsigned(
    stream: &mut ExiBitstream,
    exi_unsigned: &ExiUnsigned,
) -> Result<(), ExiError> {
    for &octet in exi_unsigned.octets[..exi_unsigned.octets_count].iter() {
        exi_bitstream_write_octet(stream, octet)?;
    }
    Ok(())
}

pub fn exi_basetypes_encoder_bool(stream: &mut ExiBitstream, value: i32) -> Result<(), ExiError> {
    let bit: u32 = if value != 0 { 1 } else { 0 };
    exi_bitstream_write_bits(stream, 1, bit)?;
    Ok(())
}

pub fn exi_basetypes_encoder_bytes(
    stream: &mut ExiBitstream,
    bytes: &[u8],
) -> Result<(), ExiError> {
    for &byte in bytes.iter() {
        exi_bitstream_write_octet(stream, byte)?;
    }
    Ok(())
}

pub fn exi_basetypes_encoder_nbit_uint(
    stream: &mut ExiBitstream,
    bit_count: usize,
    value: u32,
) -> Result<(), ExiError> {
    exi_bitstream_write_bits(stream, bit_count, value)?;
    Ok(())
}

pub fn exi_basetypes_encoder_uint_8(stream: &mut ExiBitstream, value: u8) -> Result<(), ExiError> {
    let mut exi_unsigned = ExiUnsigned {
        octets: [0; 29],
        octets_count: 0,
    };
    let result = u32::from(value);
    exi_basetypes_convert_to_unsigned(&mut exi_unsigned, result, 2)?;
    exi_basetypes_encoder_write_unsigned(stream, &exi_unsigned)
}

pub fn exi_basetypes_encoder_uint_16(
    stream: &mut ExiBitstream,
    value: u16,
) -> Result<(), ExiError> {
    let mut exi_unsigned = ExiUnsigned {
        octets: [0; 29],
        octets_count: 0,
    };
    let result = u32::from(value);
    exi_basetypes_convert_to_unsigned(&mut exi_unsigned, result, 3)?;
    exi_basetypes_encoder_write_unsigned(stream, &exi_unsigned)
}

pub fn exi_basetypes_encoder_uint_32(
    stream: &mut ExiBitstream,
    value: u32,
) -> Result<(), ExiError> {
    let mut exi_unsigned = ExiUnsigned {
        octets: [0; 29],
        octets_count: 0,
    };
    exi_basetypes_convert_to_unsigned(&mut exi_unsigned, value, 5)?;
    exi_basetypes_encoder_write_unsigned(stream, &exi_unsigned)
}

pub fn exi_basetypes_encoder_uint_64(
    stream: &mut ExiBitstream,
    value: u64,
) -> Result<(), ExiError> {
    let mut exi_unsigned = ExiUnsigned {
        octets: [0; 29],
        octets_count: 0,
    };
    exi_basetypes_convert_64_to_unsigned(&mut exi_unsigned, value)?;
    exi_basetypes_encoder_write_unsigned(stream, &exi_unsigned)
}

pub fn exi_basetypes_encoder_unsigned(
    stream: &mut ExiBitstream,
    value: &ExiUnsigned,
) -> Result<(), ExiError> {
    let mut raw_exi_unsigned = ExiUnsigned {
        octets: [0; 29],
        octets_count: 0,
    };
    exi_basetypes_convert_bytes_to_unsigned(
        &mut raw_exi_unsigned,
        &value.octets[..value.octets_count],
    )?;
    exi_basetypes_encoder_write_unsigned(stream, &raw_exi_unsigned)
}

pub fn exi_basetypes_encoder_integer_8(
    stream: &mut ExiBitstream,
    value: i8,
) -> Result<(), ExiError> {
    let sign = if value < 0 { 1 } else { 0 };
    exi_basetypes_encoder_bool(stream, sign)?;
    let mut result = value as u8;
    if sign != 0 {
        result = (-i32::from(value) - 1) as u8;
    }
    exi_basetypes_encoder_uint_8(stream, result)
}

pub fn exi_basetypes_encoder_integer_16(
    stream: &mut ExiBitstream,
    value: i16,
) -> Result<(), ExiError> {
    let sign = if value < 0 { 1 } else { 0 };
    exi_basetypes_encoder_bool(stream, sign)?;
    let mut result = value as u16;
    if sign != 0 {
        result = (-i32::from(value) - 1) as u16;
    }
    exi_basetypes_encoder_uint_16(stream, result)
}

pub fn exi_basetypes_encoder_integer_32(
    stream: &mut ExiBitstream,
    value: i32,
) -> Result<(), ExiError> {
    let sign = if value < 0 { 1 } else { 0 };
    exi_basetypes_encoder_bool(stream, sign)?;
    let mut result = value as u32;
    if sign != 0 {
        result = (-value - 1) as u32;
    }
    exi_basetypes_encoder_uint_32(stream, result)
}

pub fn exi_basetypes_encoder_integer_64(
    stream: &mut ExiBitstream,
    value: i64,
) -> Result<(), ExiError> {
    let sign = if value < 0 { 1 } else { 0 };
    exi_basetypes_encoder_bool(stream, sign)?;
    let result = if sign != 0 { (-value - 1) as u64 } else { value as u64 };
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
    for &ch in characters.as_bytes().iter() {
        let byte = ch as u8;
        if byte > ASCII_MAX_VALUE {
            return Err(ExiError::UnsupportedCharacterValue);
        }
        exi_bitstream_write_octet(stream, byte)?;
    }
    Ok(())
}
