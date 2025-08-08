use core::result::Result;

use heapless::Vec;

use crate::common::exi_basetypes_decoder::{
    exi_basetypes_decoder_bytes, exi_basetypes_decoder_integer_16,
    exi_basetypes_decoder_integer_32, exi_basetypes_decoder_integer_64,
    exi_basetypes_decoder_integer_8, exi_basetypes_decoder_nbit_uint,
    exi_basetypes_decoder_uint_16, exi_basetypes_decoder_uint_32, exi_basetypes_decoder_uint_64,
    exi_basetypes_decoder_uint_8,
};
use crate::common::exi_bitstream::ExiBitstream;
use crate::common::exi_error_codes::ExiError;

pub fn decode_exi_type_hex_binary<const N: usize>(
    stream: &mut ExiBitstream,
) -> Result<Vec<u8, N>, ExiError> {
    let mut event_code: u32 = 0;

    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code == 0 {
        let val_len = exi_basetypes_decoder_uint_16(stream)? as usize;
        if val_len > N {
            return Err(ExiError::ByteBufferTooSmall);
        }
        let buf = exi_basetypes_decoder_bytes::<N>(stream, val_len)?;
        exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
        if event_code != 0 {
            return Err(ExiError::DeviantsNotSupported);
        }
        return Ok(buf);
    }

    Err(ExiError::UnsupportedSubEvent)
}

pub fn decode_exi_type_integer8(stream: &mut ExiBitstream, value: &mut i8) -> Result<(), ExiError> {
    let mut event_code: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code == 0 {
        exi_basetypes_decoder_integer_8(stream, value)?;
    } else {
        return Err(ExiError::UnsupportedSubEvent);
    }
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code != 0 {
        return Err(ExiError::DeviantsNotSupported);
    }
    Ok(())
}

pub fn decode_exi_type_integer16(
    stream: &mut ExiBitstream,
    value: &mut i16,
) -> Result<(), ExiError> {
    let mut event_code: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code == 0 {
        exi_basetypes_decoder_integer_16(stream, value)?;
    } else {
        return Err(ExiError::UnsupportedSubEvent);
    }
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code != 0 {
        return Err(ExiError::DeviantsNotSupported);
    }
    Ok(())
}

pub fn decode_exi_type_integer32(
    stream: &mut ExiBitstream,
    value: &mut i32,
) -> Result<(), ExiError> {
    let mut event_code: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code == 0 {
        exi_basetypes_decoder_integer_32(stream, value)?;
    } else {
        return Err(ExiError::UnsupportedSubEvent);
    }
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code != 0 {
        return Err(ExiError::DeviantsNotSupported);
    }
    Ok(())
}

pub fn decode_exi_type_integer64(
    stream: &mut ExiBitstream,
    value: &mut i64,
) -> Result<(), ExiError> {
    let mut event_code: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code == 0 {
        exi_basetypes_decoder_integer_64(stream, value)?;
    } else {
        return Err(ExiError::UnsupportedSubEvent);
    }
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code != 0 {
        return Err(ExiError::DeviantsNotSupported);
    }
    Ok(())
}

pub fn decode_exi_type_uint8(stream: &mut ExiBitstream, value: &mut u8) -> Result<(), ExiError> {
    let mut event_code: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code == 0 {
        exi_basetypes_decoder_uint_8(stream, value)?;
    } else {
        return Err(ExiError::UnsupportedSubEvent);
    }
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code != 0 {
        return Err(ExiError::DeviantsNotSupported);
    }
    Ok(())
}

pub fn decode_exi_type_uint16(stream: &mut ExiBitstream, value: &mut u16) -> Result<(), ExiError> {
    let mut event_code: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code == 0 {
        *value = exi_basetypes_decoder_uint_16(stream)?;
    } else {
        return Err(ExiError::UnsupportedSubEvent);
    }
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code != 0 {
        return Err(ExiError::DeviantsNotSupported);
    }
    Ok(())
}

pub fn decode_exi_type_uint32(stream: &mut ExiBitstream, value: &mut u32) -> Result<(), ExiError> {
    let mut event_code: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code == 0 {
        exi_basetypes_decoder_uint_32(stream, value)?;
    } else {
        return Err(ExiError::UnsupportedSubEvent);
    }
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code != 0 {
        return Err(ExiError::DeviantsNotSupported);
    }
    Ok(())
}

pub fn decode_exi_type_uint64(stream: &mut ExiBitstream, value: &mut u64) -> Result<(), ExiError> {
    let mut event_code: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code == 0 {
        exi_basetypes_decoder_uint_64(stream, value)?;
    } else {
        return Err(ExiError::UnsupportedSubEvent);
    }
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code != 0 {
        return Err(ExiError::DeviantsNotSupported);
    }
    Ok(())
}
