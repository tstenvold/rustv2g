use core::result::Result;
use heapless::String;

use crate::common::exi_basetypes::{ExiSigned, ExiUnsigned, EXI_UNSIGNED_MAX_OCTETS};
use crate::common::exi_bitstream::ExiBitstream;
use crate::common::exi_error_codes::ExiError;

fn exi_basetypes_decoder_read_unsigned(
    stream: &mut ExiBitstream,
    exi_unsigned: &mut ExiUnsigned,
) -> Result<(), ExiError> {
    let msb: u8 = 1 << 7;
    exi_unsigned
        .octets
        .resize(EXI_UNSIGNED_MAX_OCTETS, 0)
        .map_err(|_| ExiError::OctetCountLargerThanTypeSupports)?;

    for octet in exi_unsigned.octets.iter_mut() {
        *octet = stream.read_octet()?;
        if *octet & msb == 0 {
            return Ok(());
        }
    }

    Err(ExiError::SupportedMaxOctetsOverrun)
}

pub fn exi_basetypes_decoder_bool(
    stream: &mut ExiBitstream,
    value: &mut i32,
) -> Result<(), ExiError> {
    let bit: u32 = stream.read_bits(1)?;
    *value = if bit == 0 { 0 } else { 1 };
    Ok(())
}

pub fn exi_basetypes_decoder_bytes(
    stream: &mut ExiBitstream,
    bytes_len: usize,
    bytes: &mut [u8],
) -> Result<(), ExiError> {
    if bytes_len > bytes.len() {
        return Err(ExiError::ByteBufferTooSmall);
    }
    for b in bytes.iter_mut().take(bytes_len) {
        *b = stream.read_octet()?;
    }
    Ok(())
}

pub fn exi_basetypes_decoder_nbit_uint(
    stream: &mut ExiBitstream,
    bit_count: usize,
    value: &mut u32,
) -> Result<(), ExiError> {
    *value = stream.read_bits(bit_count)?;
    Ok(())
}

pub fn exi_basetypes_decoder_uint_8(
    stream: &mut ExiBitstream,
    value: &mut u8,
) -> Result<(), ExiError> {
    let mut exi_unsigned = ExiUnsigned::default();
    let mut result: u32 = 0;
    exi_basetypes_decoder_read_unsigned(stream, &mut exi_unsigned)?;
    exi_unsigned.convert_from_unsigned(&mut result, 2)?;
    *value = u8::try_from(result).map_err(|_| ExiError::InvalidValue)?;
    Ok(())
}

pub fn exi_basetypes_decoder_uint_16(
    stream: &mut ExiBitstream,
    value: &mut u16,
) -> Result<(), ExiError> {
    let mut exi_unsigned = ExiUnsigned::default();
    let mut result: u32 = 0;
    exi_basetypes_decoder_read_unsigned(stream, &mut exi_unsigned)?;
    exi_unsigned.convert_from_unsigned(&mut result, 3)?;
    *value = u16::try_from(result).map_err(|_| ExiError::InvalidValue)?;
    Ok(())
}

pub fn exi_basetypes_decoder_uint_32(
    stream: &mut ExiBitstream,
    value: &mut u32,
) -> Result<(), ExiError> {
    let mut exi_unsigned = ExiUnsigned::default();
    exi_basetypes_decoder_read_unsigned(stream, &mut exi_unsigned)?;
    exi_unsigned.convert_from_unsigned(value, 5)?;
    Ok(())
}

pub fn exi_basetypes_decoder_uint_64(
    stream: &mut ExiBitstream,
    value: &mut u64,
) -> Result<(), ExiError> {
    let mut exi_unsigned = ExiUnsigned::default();
    exi_basetypes_decoder_read_unsigned(stream, &mut exi_unsigned)?;
    exi_unsigned.convert_64_from_unsigned(value)?;
    Ok(())
}

pub fn exi_basetypes_decoder_unsigned(
    stream: &mut ExiBitstream,
    value: &mut ExiUnsigned,
) -> Result<(), ExiError> {
    exi_basetypes_decoder_read_unsigned(stream, value)
}

pub fn exi_basetypes_decoder_integer_8(
    stream: &mut ExiBitstream,
    value: &mut i8,
) -> Result<(), ExiError> {
    let mut sign: i32 = 0;
    exi_basetypes_decoder_bool(stream, &mut sign)?;
    let mut tmp: u8 = 0;
    exi_basetypes_decoder_uint_8(stream, &mut tmp)?;
    *value = i8::try_from(tmp).map_err(|_| ExiError::InvalidValue)?;
    if sign != 0 {
        *value = -(*value + 1);
    }
    Ok(())
}

pub fn exi_basetypes_decoder_integer_16(
    stream: &mut ExiBitstream,
    value: &mut i16,
) -> Result<(), ExiError> {
    let mut sign: i32 = 0;
    exi_basetypes_decoder_bool(stream, &mut sign)?;
    let mut tmp: u16 = 0;
    exi_basetypes_decoder_uint_16(stream, &mut tmp)?;
    *value = i16::try_from(tmp).map_err(|_| ExiError::InvalidValue)?;
    if sign != 0 {
        *value = -(*value + 1);
    }
    Ok(())
}

pub fn exi_basetypes_decoder_integer_32(
    stream: &mut ExiBitstream,
    value: &mut i32,
) -> Result<(), ExiError> {
    let mut sign: i32 = 0;
    exi_basetypes_decoder_bool(stream, &mut sign)?;
    let mut tmp: u32 = 0;
    exi_basetypes_decoder_uint_32(stream, &mut tmp)?;
    *value = i32::try_from(tmp).map_err(|_| ExiError::InvalidValue)?;
    if sign != 0 {
        *value = -(*value + 1);
    }
    Ok(())
}

pub fn exi_basetypes_decoder_integer_64(
    stream: &mut ExiBitstream,
    value: &mut i64,
) -> Result<(), ExiError> {
    let mut sign: i32 = 0;
    exi_basetypes_decoder_bool(stream, &mut sign)?;
    let mut tmp: u64 = 0;
    exi_basetypes_decoder_uint_64(stream, &mut tmp)?;
    *value = i64::try_from(tmp).map_err(|_| ExiError::InvalidValue)?;
    if sign != 0 {
        *value = -(*value + 1);
    }
    Ok(())
}

pub fn exi_basetypes_decoder_signed(
    stream: &mut ExiBitstream,
    value: &mut ExiSigned,
) -> Result<(), ExiError> {
    let mut sign: i32 = 0;
    exi_basetypes_decoder_bool(stream, &mut sign)?;
    value.is_negative = if sign == 0 { 0 } else { 1 };
    let mut raw_value = ExiUnsigned::default();
    exi_basetypes_decoder_unsigned(stream, &mut raw_value)?;
    exi_basetypes_decoder_unsigned(stream, &mut raw_value)?;
    let mut len = value.data.octets.len();
    raw_value.convert_bytes_from_unsigned(&mut value.data.octets, &mut len)?;

    Ok(())
}

pub fn exi_basetypes_decoder_characters(
    stream: &mut ExiBitstream,
    characters_len: usize,
    characters: &mut String<100>,
    characters_size: usize,
) -> Result<(), ExiError> {
    let ascii_max_value: u8 = 127;
    if characters_len + 1 > characters_size {
        return Err(ExiError::CharacterBufferTooSmall);
    }
    for _ in 0..characters_len {
        let n: u8 = stream.read_octet()?;
        if n > ascii_max_value {
            return Err(ExiError::UnsupportedCharacterValue);
        }
        characters
            .push(n as char)
            .map_err(|_| ExiError::CharacterBufferTooSmall)?;
    }
    Ok(())
}
