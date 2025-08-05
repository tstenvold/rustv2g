use core::result::Result;
use heapless::String;

use crate::common::exi_basetypes::{
    exi_basetypes_convert_64_from_unsigned, exi_basetypes_convert_bytes_from_unsigned,
    exi_basetypes_convert_from_unsigned, ExiSigned, ExiUnsigned,
};
use crate::common::exi_bitstream::{
    exi_bitstream_read_bits, exi_bitstream_read_octet, ExiBitstream,
};
use crate::common::exi_error_codes::ExiError;

fn exi_basetypes_decoder_read_unsigned(
    stream: &mut ExiBitstream,
    exi_unsigned: &mut ExiUnsigned,
) -> Result<(), ExiError> {
    let msb: u8 = 1 << 7;
    let mut found_sequence_end = false;
    exi_unsigned.octets_count = 0;
    while exi_unsigned.octets_count < 29 {
        let octet = exi_unsigned
            .octets
            .get_mut(exi_unsigned.octets_count)
            .ok_or(ExiError::SupportedMaxOctetsOverrun)?;
        exi_bitstream_read_octet(stream, octet)?;
        exi_unsigned.octets_count += 1;
        if *octet & msb == 0 {
            found_sequence_end = true;
            break;
        }
    }
    if found_sequence_end {
        Ok(())
    } else {
        Err(ExiError::SupportedMaxOctetsOverrun)
    }
}

pub fn exi_basetypes_decoder_bool(
    stream: &mut ExiBitstream,
    value: &mut i32,
) -> Result<(), ExiError> {
    let mut bit: u32 = 0;
    exi_bitstream_read_bits(stream, 1, &mut bit)?;
    *value = if bit != 0 { 1 } else { 0 };
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
        exi_bitstream_read_octet(stream, b)?;
    }
    Ok(())
}

pub fn exi_basetypes_decoder_nbit_uint(
    stream: &mut ExiBitstream,
    bit_count: usize,
    value: &mut u32,
) -> Result<(), ExiError> {
    exi_bitstream_read_bits(stream, bit_count, value).map(|_| ())
}

pub fn exi_basetypes_decoder_uint_8(
    stream: &mut ExiBitstream,
    value: &mut u8,
) -> Result<(), ExiError> {
    let mut exi_unsigned = ExiUnsigned {
        octets: [0; 29],
        octets_count: 0,
    };
    let mut result: u32 = 0;
    exi_basetypes_decoder_read_unsigned(stream, &mut exi_unsigned)?;
    exi_basetypes_convert_from_unsigned(&exi_unsigned, &mut result, 2)?;
    *value = result as u8;
    Ok(())
}

pub fn exi_basetypes_decoder_uint_16(
    stream: &mut ExiBitstream,
    value: &mut u16,
) -> Result<(), ExiError> {
    let mut exi_unsigned = ExiUnsigned {
        octets: [0; 29],
        octets_count: 0,
    };
    let mut result: u32 = 0;
    exi_basetypes_decoder_read_unsigned(stream, &mut exi_unsigned)?;
    exi_basetypes_convert_from_unsigned(&exi_unsigned, &mut result, 3)?;
    *value = result as u16;
    Ok(())
}

pub fn exi_basetypes_decoder_uint_32(
    stream: &mut ExiBitstream,
    value: &mut u32,
) -> Result<(), ExiError> {
    let mut exi_unsigned = ExiUnsigned {
        octets: [0; 29],
        octets_count: 0,
    };
    exi_basetypes_decoder_read_unsigned(stream, &mut exi_unsigned)?;
    exi_basetypes_convert_from_unsigned(&exi_unsigned, value, 5)?;
    Ok(())
}

pub fn exi_basetypes_decoder_uint_64(
    stream: &mut ExiBitstream,
    value: &mut u64,
) -> Result<(), ExiError> {
    let mut exi_unsigned = ExiUnsigned {
        octets: [0; 29],
        octets_count: 0,
    };
    exi_basetypes_decoder_read_unsigned(stream, &mut exi_unsigned)?;
    exi_basetypes_convert_64_from_unsigned(&exi_unsigned, value)?;
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
    *value = tmp as i8;
    if sign != 0 {
        *value = -(i32::from(*value) + 1) as i8;
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
    *value = tmp as i16;
    if sign != 0 {
        *value = -(i32::from(*value) + 1) as i16;
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
    *value = tmp as i32;
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
    *value = tmp as i64;
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
    let mut raw_value = ExiUnsigned {
        octets: [0; 29],
        octets_count: 0,
    };
    exi_basetypes_decoder_unsigned(stream, &mut raw_value)?;
    exi_basetypes_convert_bytes_from_unsigned(
        &raw_value,
        &mut value.data.octets,
        &mut value.data.octets_count,
    )?;
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
        let mut n: u8 = 0;
        exi_bitstream_read_octet(stream, &mut n)?;
        match n > ascii_max_value {
            true => return Err(ExiError::UnsupportedCharacterValue),
            false => characters
                .push(n as char)
                .map_err(|_| ExiError::CharacterBufferTooSmall)?,
        }
    }
    Ok(())
}
