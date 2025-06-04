use core::result::Result;

use crate::common::exi_basetypes_decoder::*;
use crate::common::exi_bitstream::*;
use crate::common::exi_error_codes::*;

pub fn decode_exi_type_hex_binary(
    stream: &mut ExiBitstream,
    value_len: &mut usize,
    value_buffer: &mut [u8],
    value_max_len: usize,
) -> Result<u8, i16> {
    let mut event_code: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code == 0 {
        exi_basetypes_decoder_uint_16(stream, &mut(*value_len as u16))?;
        let len = *value_len as usize;
        if len > value_max_len {
            return Err(-152); // buffer overflow
        }
        exi_basetypes_decoder_bytes(stream, len, &mut value_buffer[..len])?;
    } else {
        return Err(UNSUPPORTED_SUB_EVENT);
    }
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code != 0 {
        return Err(DEVIANTS_NOT_SUPPORTED);
    }
    Ok(NO_ERROR)
}

pub fn decode_exi_type_integer8(stream: &mut ExiBitstream, value: &mut i8) -> Result<u8, i16> {
    let mut event_code: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code == 0 {
        exi_basetypes_decoder_integer_8(stream, value)?;
    } else {
        return Err(UNSUPPORTED_SUB_EVENT);
    }
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code != 0 {
        return Err(DEVIANTS_NOT_SUPPORTED);
    }
    Ok(NO_ERROR)
}

pub fn decode_exi_type_integer16(stream: &mut ExiBitstream, value: &mut i16) -> Result<u8, i16> {
    let mut event_code: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code == 0 {
        exi_basetypes_decoder_integer_16(stream, value)?;
    } else {
        return Err(UNSUPPORTED_SUB_EVENT);
    }
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code != 0 {
        return Err(DEVIANTS_NOT_SUPPORTED);
    }
    Ok(NO_ERROR)
}

pub fn decode_exi_type_integer32(stream: &mut ExiBitstream, value: &mut i32) -> Result<u8, i16> {
    let mut event_code: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code == 0 {
        exi_basetypes_decoder_integer_32(stream, value)?;
    } else {
        return Err(UNSUPPORTED_SUB_EVENT);
    }
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code != 0 {
        return Err(DEVIANTS_NOT_SUPPORTED);
    }
    Ok(NO_ERROR)
}

pub fn decode_exi_type_integer64(stream: &mut ExiBitstream, value: &mut i64) -> Result<u8, i16> {
    let mut event_code: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code == 0 {
        exi_basetypes_decoder_integer_64(stream, value)?;
    } else {
        return Err(UNSUPPORTED_SUB_EVENT);
    }
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code != 0 {
        return Err(DEVIANTS_NOT_SUPPORTED);
    }
    Ok(NO_ERROR)
}

pub fn decode_exi_type_uint8(stream: &mut ExiBitstream, value: &mut u8) -> Result<u8, i16> {
    let mut event_code: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code == 0 {
        exi_basetypes_decoder_uint_8(stream, value)?;
    } else {
        return Err(UNSUPPORTED_SUB_EVENT);
    }
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code != 0 {
        return Err(DEVIANTS_NOT_SUPPORTED);
    }
    Ok(NO_ERROR)
}

pub fn decode_exi_type_uint16(stream: &mut ExiBitstream, value: &mut u16) -> Result<u8, i16> {
    let mut event_code: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code == 0 {
        exi_basetypes_decoder_uint_16(stream, value)?;
    } else {
        return Err(UNSUPPORTED_SUB_EVENT);
    }
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code != 0 {
        return Err(DEVIANTS_NOT_SUPPORTED);
    }
    Ok(NO_ERROR)
}

pub fn decode_exi_type_uint32(stream: &mut ExiBitstream, value: &mut u32) -> Result<u8, i16> {
    let mut event_code: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code == 0 {
        exi_basetypes_decoder_uint_32(stream, value)?;
    } else {
        return Err(UNSUPPORTED_SUB_EVENT);
    }
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code != 0 {
        return Err(DEVIANTS_NOT_SUPPORTED);
    }
    Ok(NO_ERROR)
}

pub fn decode_exi_type_uint64(stream: &mut ExiBitstream, value: &mut u64) -> Result<u8, i16> {
    let mut event_code: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code == 0 {
        exi_basetypes_decoder_uint_64(stream, value)?;
    } else {
        return Err(UNSUPPORTED_SUB_EVENT);
    }
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code != 0 {
        return Err(DEVIANTS_NOT_SUPPORTED);
    }
    Ok(NO_ERROR)
}
