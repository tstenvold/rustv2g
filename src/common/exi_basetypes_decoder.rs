use core::result::Result;

use crate::common::exi_basetypes::*;
use crate::common::exi_bitstream::*;
use crate::common::exi_error_codes::*;

fn exi_basetypes_decoder_read_unsigned(
    stream: &mut ExiBitstream,
    exi_unsigned: &mut ExiUnsigned,
) -> Result<u8, i16> {
    let msb: u8 = 1 << 7;
    let mut found_sequence_end = false;
    exi_unsigned.octets_count = 0;
    while exi_unsigned.octets_count < 29 {
        let octet = exi_unsigned
            .octets
            .get_mut(exi_unsigned.octets_count)
            .ok_or(-30 as i16)?;
        exi_bitstream_read_octet(stream, octet).map_err(|e| e)?;
        exi_unsigned.octets_count += 1;
        if *octet & msb == 0 {
            found_sequence_end = true;
            break;
        }
    }
    if found_sequence_end {
        Ok(NO_ERROR)
    } else {
        Err(SUPPORTED_MAX_OCTETS_OVERRUN)
    }
}

pub fn exi_basetypes_decoder_bool(stream: &mut ExiBitstream, value: &mut i32) -> Result<u8, i16> {
    let mut bit: u32 = 0;
    exi_bitstream_read_bits(stream, 1, &mut bit).map_err(|e| e)?;
    *value = if bit != 0 { 1 } else { 0 };
    Ok(NO_ERROR)
}

pub fn exi_basetypes_decoder_bytes(
    stream: &mut ExiBitstream,
    bytes_len: usize,
    bytes: &mut [u8],
) -> Result<u8, i16> {
    if bytes_len > bytes.len() {
        return Err(BYTE_BUFFER_TOO_SMALL);
    }
    for i in 0..bytes_len {
        exi_bitstream_read_octet(stream, &mut bytes[i]).map_err(|e| e)?;
    }
    Ok(NO_ERROR)
}

pub fn exi_basetypes_decoder_nbit_uint(
    stream: &mut ExiBitstream,
    bit_count: usize,
    value: &mut u32,
) -> Result<u8, i16> {
    exi_bitstream_read_bits(stream, bit_count, value)
        .map(|_| 0)
        .map_err(|e| e)
}

pub fn exi_basetypes_decoder_uint_8(stream: &mut ExiBitstream, value: &mut u8) -> Result<u8, i16> {
    let mut exi_unsigned = ExiUnsigned {
        octets: [0; 29],
        octets_count: 0,
    };
    let mut result: u32 = 0;
    exi_basetypes_decoder_read_unsigned(stream, &mut exi_unsigned)?;
    exi_basetypes_convert_from_unsigned(&mut exi_unsigned, &mut result, 2).map_err(|e| e)?;
    *value = result as u8;
    Ok(NO_ERROR)
}

pub fn exi_basetypes_decoder_uint_16(
    stream: &mut ExiBitstream,
    value: &mut u16,
) -> Result<u8, i16> {
    let mut exi_unsigned = ExiUnsigned {
        octets: [0; 29],
        octets_count: 0,
    };
    let mut result: u32 = 0;
    exi_basetypes_decoder_read_unsigned(stream, &mut exi_unsigned)?;
    exi_basetypes_convert_from_unsigned(&mut exi_unsigned, &mut result, 3).map_err(|e| e)?;
    *value = result as u16;
    Ok(NO_ERROR)
}

pub fn exi_basetypes_decoder_uint_32(
    stream: &mut ExiBitstream,
    value: &mut u32,
) -> Result<u8, i16> {
    let mut exi_unsigned = ExiUnsigned {
        octets: [0; 29],
        octets_count: 0,
    };
    exi_basetypes_decoder_read_unsigned(stream, &mut exi_unsigned)?;
    exi_basetypes_convert_from_unsigned(&mut exi_unsigned, value, 5).map_err(|e| e)?;
    Ok(NO_ERROR)
}

pub fn exi_basetypes_decoder_uint_64(
    stream: &mut ExiBitstream,
    value: &mut u64,
) -> Result<u8, i16> {
    let mut exi_unsigned = ExiUnsigned {
        octets: [0; 29],
        octets_count: 0,
    };
    exi_basetypes_decoder_read_unsigned(stream, &mut exi_unsigned)?;
    exi_basetypes_convert_64_from_unsigned(&mut exi_unsigned, value).map_err(|e| e)?;
    Ok(NO_ERROR)
}

pub fn exi_basetypes_decoder_unsigned(
    stream: &mut ExiBitstream,
    value: &mut ExiUnsigned,
) -> Result<u8, i16> {
    exi_basetypes_decoder_read_unsigned(stream, value)
}

pub fn exi_basetypes_decoder_integer_8(
    stream: &mut ExiBitstream,
    value: &mut i8,
) -> Result<u8, i16> {
    let mut sign: i32 = 0;
    exi_basetypes_decoder_bool(stream, &mut sign)?;
    let mut tmp: u8 = 0;
    exi_basetypes_decoder_uint_8(stream, &mut tmp)?;
    *value = tmp as i8;
    if sign != 0 {
        *value = -(*value as i32 + 1) as i8;
    }
    Ok(NO_ERROR)
}

pub fn exi_basetypes_decoder_integer_16(
    stream: &mut ExiBitstream,
    value: &mut i16,
) -> Result<u8, i16> {
    let mut sign: i32 = 0;
    exi_basetypes_decoder_bool(stream, &mut sign)?;
    let mut tmp: u16 = 0;
    exi_basetypes_decoder_uint_16(stream, &mut tmp)?;
    *value = tmp as i16;
    if sign != 0 {
        *value = -(*value as i32 + 1) as i16;
    }
    Ok(NO_ERROR)
}

pub fn exi_basetypes_decoder_integer_32(
    stream: &mut ExiBitstream,
    value: &mut i32,
) -> Result<u8, i16> {
    let mut sign: i32 = 0;
    exi_basetypes_decoder_bool(stream, &mut sign)?;
    let mut tmp: u32 = 0;
    exi_basetypes_decoder_uint_32(stream, &mut tmp)?;
    *value = tmp as i32;
    if sign != 0 {
        *value = -(*value + 1);
    }
    Ok(NO_ERROR)
}

pub fn exi_basetypes_decoder_integer_64(
    stream: &mut ExiBitstream,
    value: &mut i64,
) -> Result<u8, i16> {
    let mut sign: i32 = 0;
    exi_basetypes_decoder_bool(stream, &mut sign)?;
    let mut tmp: u64 = 0;
    exi_basetypes_decoder_uint_64(stream, &mut tmp)?;
    *value = tmp as i64;
    if sign != 0 {
        *value = -(*value + 1);
    }
    Ok(NO_ERROR)
}

pub fn exi_basetypes_decoder_signed(
    stream: &mut ExiBitstream,
    value: &mut ExiSigned,
) -> Result<u8, i16> {
    let mut sign: i32 = 0;
    exi_basetypes_decoder_bool(stream, &mut sign)?;
    value.is_negative = if sign == 0 { 0 } else { 1 };
    let mut raw_value = ExiUnsigned {
        octets: [0; 29],
        octets_count: 0,
    };
    exi_basetypes_decoder_unsigned(stream, &mut raw_value)?;
    exi_basetypes_convert_bytes_from_unsigned(
        &mut raw_value,
        &mut value.data.octets,
        &mut value.data.octets_count,
    )
    .map_err(|e| e)?;
    Ok(NO_ERROR)
}

pub fn exi_basetypes_decoder_characters(
    stream: &mut ExiBitstream,
    characters_len: usize,
    characters: &mut [u8],
    characters_size: usize,
) -> Result<u8, i16> {
    let ascii_max_value: u8 = 127;
    if characters_len + 1 > characters_size {
        return Err(CHARACTER_BUFFER_TOO_SMALL);
    }
    for n in 0..characters_len {
        exi_bitstream_read_octet(stream, &mut characters[n]).map_err(|e| e)?;
        if characters[n] > ascii_max_value {
            return Err(UNSUPPORTED_CHARACTER_VALUE);
        }
    }
    characters[characters_len] = 0;
    Ok(NO_ERROR)
}
