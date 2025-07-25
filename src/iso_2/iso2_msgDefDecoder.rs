use crate::common::exi_basetypes_decoder::*;
use crate::common::exi_bitstream::*;
use crate::common::exi_error_codes::*;
use crate::common::exi_types_decoder::*;
use crate::common::exi_header::*;
use crate::iso_2::iso2_msgDefDatatypes::*;

pub fn decode_iso2_CostType(
    stream: &mut ExiBitstream,
    mut cost_type: &mut Iso2CostType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            0 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*cost_type).costKind = value as Iso2CostKindType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 1 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            1 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_uint32(stream, &mut (*cost_type).amount)?;
                            if error == 0 as i32 {
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            2 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        3 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*cost_type).amountMultiplier =
                                            Some(value_0.wrapping_sub(3) as i8);
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}

pub fn decode_iso2_TransformType(
    stream: &mut ExiBitstream,
    mut transform_type: &mut Iso2TransformType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 5 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            5 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*transform_type).Algorithm.len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*transform_type).Algorithm.len as i32 >= 2 as i32 {
                                    (*transform_type).Algorithm.len =
                                        ((*transform_type).Algorithm.len as i32 - 2 as i32) as usize;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*transform_type).Algorithm.len as usize,
                                        &mut ((*transform_type).Algorithm.data),
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 6 as i32;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            6 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*transform_type).XPath.unwrap().len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*transform_type).XPath.unwrap().len as i32 >= 2 as i32
                                        {
                                            (*transform_type).XPath.unwrap().len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*transform_type).XPath.unwrap().len,
                                                &mut (*transform_type).XPath.unwrap().data,
                                                (64 as i32 + 1 as i32) as usize,
                                            )?;
                                        } else {
                                            return Err(STRINGVALUES_NOT_SUPPORTED);
                                        }
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        1 => {
                            return Err(UNKNOWN_EVENT_FOR_DECODING);
                        }
                        2 => {
                            return Ok(NO_ERROR);
                        }
                        3 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*transform_type).ANY.unwrap().len,
                                &mut (*transform_type).ANY.unwrap().data,
                                4 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_IntervalType(
    stream: &mut ExiBitstream,
    mut _interval_type: &mut Iso2IntervalType,
) -> Result<u8, i16> {
    let mut eventCode: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
    if eventCode != 0 as i32 as u32 {
        return Err(UNKNOWN_EVENT_CODE);
    }

    return Ok(NO_ERROR);
}
pub fn decode_iso2_TransformsType(
    stream: &mut ExiBitstream,
    mut transform_type: &mut Iso2TransformsType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 7 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            7 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_TransformType(stream, &mut (*transform_type).Transform)?;
                            grammar_id = 8;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            8 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Err(ARRAY_OUT_OF_BOUNDS);
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_DSAKeyValueType(
    stream: &mut ExiBitstream,
    mut DSA_key_value: &mut Iso2DSAKeyValueType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 9 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            9 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*DSA_key_value).P.unwrap().len,
                                &mut (*DSA_key_value).P.unwrap().data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 10 as i32;
                            }
                        }
                        1 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*DSA_key_value).G.unwrap().len,
                                &mut (*DSA_key_value).G.unwrap().data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 12 as i32;
                            }
                        }
                        2 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*DSA_key_value).Y.len,
                                &mut (*DSA_key_value).Y.data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 13 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            10 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*DSA_key_value).Q.unwrap().len,
                                &mut (*DSA_key_value).Q.unwrap().data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 11 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            11 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*DSA_key_value).G.unwrap().len,
                                &mut (*DSA_key_value).G.unwrap().data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 12 as i32;
                            }
                        }
                        1 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*DSA_key_value).Y.len,
                                &mut (*DSA_key_value).Y.data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 13 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            12 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*DSA_key_value).Y.len,
                                &mut (*DSA_key_value).Y.data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 13 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            13 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*DSA_key_value).J.unwrap().len,
                                &mut (*DSA_key_value).J.unwrap().data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 14 as i32;
                            }
                        }
                        1 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*DSA_key_value).Seed.unwrap().len,
                                &mut (*DSA_key_value).Seed.unwrap().data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 15 as i32;
                            }
                        }
                        2 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            14 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*DSA_key_value).Seed.unwrap().len,
                                &mut (*DSA_key_value).Seed.unwrap().data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 15 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            15 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*DSA_key_value).PgenCounter.unwrap().len,
                                &mut (*DSA_key_value).PgenCounter.unwrap().data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_X509IssuerSerialType(
    stream: &mut ExiBitstream,
    mut x509_issuer_serial: &mut Iso2X509IssuerSerialType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 16 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            16 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*x509_issuer_serial).X509IssuerName.len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*x509_issuer_serial).X509IssuerName.len
                                            as i32
                                            >= 2 as i32
                                        {
                                            (*x509_issuer_serial).X509IssuerName.len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*x509_issuer_serial).X509IssuerName.len
                                                    as usize,
                                                &mut (*x509_issuer_serial).X509IssuerName.data,
                                                (64 as i32 + 1 as i32) as usize,
                                            )?;
                                        } else {
                                            return Err(STRINGVALUES_NOT_SUPPORTED);
                                        }
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 17 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            17 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                exi_basetypes_decoder_signed(
                                    stream,
                                    &mut (*x509_issuer_serial).X509SerialNumber,
                                )?;
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_RelativeTimeIntervalType(
    stream: &mut ExiBitstream,
    mut relative_time_interval: &mut Iso2RelativeTimeIntervalType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 18 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            18 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_uint32(
                                stream,
                                &mut (*relative_time_interval).start,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 19 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            19 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_uint32(
                                stream,
                                &mut (*relative_time_interval).duration.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_DigestMethodType(
    stream: &mut ExiBitstream,
    mut digest_method: &mut Iso2DigestMethodType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 20 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            20 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*digest_method).Algorithm.len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*digest_method).Algorithm.len as i32 >= 2 as i32 {
                                    (*digest_method).Algorithm.len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*digest_method).Algorithm.len as usize,
                                        &mut (*digest_method).Algorithm.data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 21 as i32;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            21 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Err(UNKNOWN_EVENT_FOR_DECODING);
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        2 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*digest_method).ANY.unwrap().len,
                                &mut (*digest_method).ANY.unwrap().data,
                                4 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_RSAKeyValueType(
    stream: &mut ExiBitstream,
    mut RSA_key_value: &mut Iso2RSAKeyValueType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 22 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            22 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                                decode_exi_type_hex_binary(
                                    stream,
                                    &mut RSA_key_value.Modulus.len,
                                    &mut RSA_key_value.Modulus.data,
                                    350 as i32 as usize,
                                )?;
                                grammar_id = 23 as i32;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            23 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*RSA_key_value).Exponent.len,
                                &mut (*RSA_key_value).Exponent.data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_CanonicalizationMethodType(
    stream: &mut ExiBitstream,
    mut canonicalization_method: &mut Iso2CanonicalizationMethodType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 24 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            24 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*canonicalization_method).Algorithm.len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*canonicalization_method).Algorithm.len as i32
                                    >= 2 as i32
                                {
                                    (*canonicalization_method).Algorithm.len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*canonicalization_method).Algorithm.len
                                            as usize,
                                        &mut (*canonicalization_method).Algorithm.data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 25 as i32;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            25 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Err(UNKNOWN_EVENT_FOR_DECODING);
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        2 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*canonicalization_method).ANY.unwrap().len,
                                &mut (*canonicalization_method).ANY.unwrap().data,
                                4 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_SignatureMethodType(
    stream: &mut ExiBitstream,
    mut signature_method: &mut Iso2SignatureMethodType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 26 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            26 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*signature_method).Algorithm.len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*signature_method).Algorithm.len as i32 >= 2 as i32
                                {
                                    (*signature_method).Algorithm.len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*signature_method).Algorithm.len as usize,
                                        &mut (*signature_method).Algorithm.data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 27 as i32;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            27 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                exi_basetypes_decoder_signed(
                                    stream,
                                    &mut (*signature_method).HMACOutputLength.unwrap(),
                                )?;
                                if error == 0 as i32 {
                                    grammar_id = 28 as i32;
                                }
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                            }
                        }
                        1 => {
                            return Err(UNKNOWN_EVENT_FOR_DECODING);
                        }
                        2 => {
                            return Ok(NO_ERROR);
                        }
                        3 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*signature_method).ANY.unwrap().len,
                                &mut (*signature_method).ANY.unwrap().data,
                                4 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            28 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Err(UNKNOWN_EVENT_FOR_DECODING);
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        2 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*signature_method).ANY.unwrap().len,
                                &mut (*signature_method).ANY.unwrap().data,
                                4 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_KeyValueType(
    stream: &mut ExiBitstream,
    mut key_value: &mut Iso2KeyValueType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 29 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            29 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_DSAKeyValueType(
                                stream,
                                &mut (*key_value).DSAKeyValue.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_RSAKeyValueType(
                                stream,
                                &mut (*key_value).RSAKeyValue.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*key_value).ANY.unwrap().len,
                                &mut (*key_value).ANY.unwrap().data,
                                4 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_PhysicalValueType(
    stream: &mut ExiBitstream,
    mut physical_value: &mut Iso2PhysicalValueType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 30 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            30 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        3 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*physical_value).Multiplier =
                                            value.wrapping_add(-(3 as i32) as u32) as i8;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 31 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            31 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        3 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*physical_value).Unit = value_0 as Iso2UnitSymbolType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 32 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            32 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_integer16(stream, &mut (*physical_value).Value)?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_ConsumptionCostType(
    stream: &mut ExiBitstream,
    mut ConsumptionCostType: &mut Iso2ConsumptionCostType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 33 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            33 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*ConsumptionCostType).startValue,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 34 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            34 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ConsumptionCostType).Cost.arrayLen as i32) < 3 as i32 {
                                let fresh0 = (*ConsumptionCostType).Cost.arrayLen;
                                (*ConsumptionCostType).Cost.arrayLen =
                                    ((*ConsumptionCostType).Cost.arrayLen).wrapping_add(1);
                                if let Some(cost) = (*ConsumptionCostType)
                                    .Cost
                                    .array
                                    .get_mut(fresh0 as usize)
                                {
                                    decode_iso2_CostType(stream, cost)?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            grammar_id = 35 as i32;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            35 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ConsumptionCostType).Cost.arrayLen as i32) < 3 as i32 {
                                let fresh1 = (*ConsumptionCostType).Cost.arrayLen;
                                (*ConsumptionCostType).Cost.arrayLen =
                                    ((*ConsumptionCostType).Cost.arrayLen).wrapping_add(1);
                                if let Some(cost) = (*ConsumptionCostType)
                                    .Cost
                                    .array
                                    .get_mut(fresh1 as usize)
                                {
                                    decode_iso2_CostType(stream, cost)?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            if ((*ConsumptionCostType).Cost.arrayLen as i32) < 3 as i32 {
                                grammar_id = 35 as i32;
                            } else {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_PMaxScheduleEntryType(
    stream: &mut ExiBitstream,
    mut PMaxScheduleEntryType: &mut Iso2PMaxScheduleEntryType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 36 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            36 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_RelativeTimeIntervalType(
                                stream,
                                &mut (*PMaxScheduleEntryType).RelativeTimeInterval.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 37 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_IntervalType(
                                stream,
                                &mut (*PMaxScheduleEntryType).TimeInterval.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 37 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            37 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*PMaxScheduleEntryType).PMax,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_SalesTariffEntryType(
    stream: &mut ExiBitstream,
    mut SalesTariffEntryType: &mut Iso2SalesTariffEntryType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 38 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            38 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_RelativeTimeIntervalType(
                                stream,
                                &mut (*SalesTariffEntryType).RelativeTimeInterval.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 39 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_IntervalType(
                                stream,
                                &mut (*SalesTariffEntryType).TimeInterval.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 39 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            39 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        8 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*SalesTariffEntryType).EPriceLevel = Some(value as u8);
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 41 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        1 => {
                            if ((*SalesTariffEntryType).ConsumptionCost.arrayLen as i32) < 3 as i32
                            {
                                let fresh2 = (*SalesTariffEntryType).ConsumptionCost.arrayLen;
                                (*SalesTariffEntryType).ConsumptionCost.arrayLen =
                                    ((*SalesTariffEntryType).ConsumptionCost.arrayLen)
                                        .wrapping_add(1);
                                if let Some(consumption_cost) = (*SalesTariffEntryType).ConsumptionCost.array.get_mut(fresh2 as usize) {
                                    decode_iso2_ConsumptionCostType(
                                        stream,
                                        consumption_cost,
                                    )?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            grammar_id = 40 as i32;
                        }
                        2 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            40 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SalesTariffEntryType).ConsumptionCost.arrayLen as i32) < 3 as i32
                            {
                                let fresh3 = (*SalesTariffEntryType).ConsumptionCost.arrayLen;
                                (*SalesTariffEntryType).ConsumptionCost.arrayLen =
                                    ((*SalesTariffEntryType).ConsumptionCost.arrayLen)
                                        .wrapping_add(1);
                                if let Some(consumption_cost) = (*SalesTariffEntryType)
                                    .ConsumptionCost
                                    .array
                                    .get_mut(fresh3 as usize)
                                {
                                    decode_iso2_ConsumptionCostType(
                                        stream,
                                        consumption_cost,
                                    )?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            if ((*SalesTariffEntryType).ConsumptionCost.arrayLen as i32) < 3 as i32
                            {
                                grammar_id = 40 as i32;
                            } else {
                                grammar_id = 41 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            41 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SalesTariffEntryType).ConsumptionCost.arrayLen as i32) < 3 as i32
                            {
                                let fresh4 = (*SalesTariffEntryType).ConsumptionCost.arrayLen;
                                (*SalesTariffEntryType).ConsumptionCost.arrayLen =
                                    ((*SalesTariffEntryType).ConsumptionCost.arrayLen)
                                        .wrapping_add(1);
                                if let Some(consumption_cost) = (*SalesTariffEntryType)
                                    .ConsumptionCost
                                    .array
                                    .get_mut(fresh4 as usize)
                                {
                                    decode_iso2_ConsumptionCostType(
                                        stream,
                                        consumption_cost,
                                    )?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            grammar_id = 42 as i32;
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            42 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SalesTariffEntryType).ConsumptionCost.arrayLen as i32) < 3 as i32
                            {
                                let fresh5 = (*SalesTariffEntryType).ConsumptionCost.arrayLen;
                                (*SalesTariffEntryType).ConsumptionCost.arrayLen =
                                    ((*SalesTariffEntryType).ConsumptionCost.arrayLen)
                                        .wrapping_add(1);
                                if let Some(consumption_cost) = (*SalesTariffEntryType)
                                    .ConsumptionCost
                                    .array
                                    .get_mut(fresh5 as usize)
                                {
                                    decode_iso2_ConsumptionCostType(
                                        stream,
                                        consumption_cost,
                                    )?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            if ((*SalesTariffEntryType).ConsumptionCost.arrayLen as i32) < 3 as i32
                            {
                                grammar_id = 42 as i32;
                            } else {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_ParameterType(
    stream: &mut ExiBitstream,
    mut ParameterType: &mut Iso2ParameterType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 43 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            43 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*ParameterType).Name.len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*ParameterType).Name.len as i32 >= 2 as i32 {
                                    (*ParameterType).Name.len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*ParameterType).Name.len as usize,
                                        &mut (*ParameterType).Name.data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 44 as i32;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            44 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ParameterType).boolValue = Some(value as i32);
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        8 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ParameterType).byteValue =
                                            Some(value_0.wrapping_sub(128) as i8);
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        2 => {
                            decode_exi_type_integer16(stream, &mut (*ParameterType).shortValue.unwrap())?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        3 => {
                            decode_exi_type_integer32(stream, &mut (*ParameterType).intValue.unwrap())?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        4 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*ParameterType).physicalValue.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        5 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*ParameterType).stringValue.unwrap().len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*ParameterType).stringValue.unwrap().len as i32
                                            >= 2 as i32
                                        {
                                            (*ParameterType).stringValue.unwrap().len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*ParameterType).stringValue.unwrap().len as usize,
                                                &mut (*ParameterType).stringValue.unwrap().data,
                                                (64 as i32 + 1 as i32) as usize,
                                            )?;
                                        } else {
                                            return Err(STRINGVALUES_NOT_SUPPORTED);
                                        }
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_PMaxScheduleType(
    stream: &mut ExiBitstream,
    mut PMaxScheduleType: &mut Iso2PMaxScheduleType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 45 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            45 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*PMaxScheduleType).PMaxScheduleEntry.arrayLen as i32) < 12 as i32 {
                                let fresh6 = (*PMaxScheduleType).PMaxScheduleEntry.arrayLen;
                                (*PMaxScheduleType).PMaxScheduleEntry.arrayLen =
                                    ((*PMaxScheduleType).PMaxScheduleEntry.arrayLen)
                                        .wrapping_add(1);
                                if let Some(entry) = (*PMaxScheduleType)
                                    .PMaxScheduleEntry
                                    .array
                                    .get_mut(fresh6 as usize)
                                {
                                    decode_iso2_PMaxScheduleEntryType(stream, entry)?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            grammar_id = 46 as i32;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            46 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*PMaxScheduleType).PMaxScheduleEntry.arrayLen as i32) < 12 as i32 {
                                let fresh7 = (*PMaxScheduleType).PMaxScheduleEntry.arrayLen;
                                (*PMaxScheduleType).PMaxScheduleEntry.arrayLen =
                                    ((*PMaxScheduleType).PMaxScheduleEntry.arrayLen)
                                        .wrapping_add(1);
                                if let Some(entry) = (*PMaxScheduleType)
                                    .PMaxScheduleEntry
                                    .array
                                    .get_mut(fresh7 as usize)
                                {
                                    decode_iso2_PMaxScheduleEntryType(stream, entry)?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            if ((*PMaxScheduleType).PMaxScheduleEntry.arrayLen as i32) < 1024 as i32
                            {
                                grammar_id = 46 as i32;
                            } else {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_ReferenceType(
    stream: &mut ExiBitstream,
    mut ReferenceType: &mut Iso2ReferenceType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 47 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            47 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*ReferenceType).Id.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*ReferenceType).Id.unwrap().len as i32 >= 2 as i32 {
                                    (*ReferenceType).Id.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*ReferenceType).Id.unwrap().len as usize,
                                        &mut (*ReferenceType).Id.unwrap().data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 48 as i32;
                        }
                        1 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*ReferenceType).Type.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*ReferenceType).Type.unwrap().len as i32 >= 2 as i32 {
                                    (*ReferenceType).Type.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*ReferenceType).Type.unwrap().len as usize,
                                        &mut (*ReferenceType).Type.unwrap().data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 49 as i32;
                        }
                        2 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*ReferenceType).URI.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*ReferenceType).URI.unwrap().len as i32 >= 2 as i32 {
                                    (*ReferenceType).URI.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*ReferenceType).URI.unwrap().len as usize,
                                        &mut (*ReferenceType).URI.unwrap().data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 50 as i32;
                        }
                        3 => {
                            decode_iso2_TransformsType(
                                stream,
                                &mut (*ReferenceType).Transforms.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 51 as i32;
                            }
                        }
                        4 => {
                            decode_iso2_DigestMethodType(
                                stream,
                                &mut (*ReferenceType).DigestMethod,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 52 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            48 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*ReferenceType).Type.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*ReferenceType).Type.unwrap().len as i32 >= 2 as i32 {
                                    (*ReferenceType).Type.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*ReferenceType).Type.unwrap().len as usize,
                                        &mut (*ReferenceType).Type.unwrap().data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 49 as i32;
                        }
                        1 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*ReferenceType).URI.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*ReferenceType).URI.unwrap().len as i32 >= 2 as i32 {
                                    (*ReferenceType).URI.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*ReferenceType).URI.unwrap().len as usize,
                                        &mut (*ReferenceType).URI.unwrap().data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 50 as i32;
                        }
                        2 => {
                            decode_iso2_TransformsType(
                                stream,
                                &mut (*ReferenceType).Transforms.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 51 as i32;
                            }
                        }
                        3 => {
                            decode_iso2_DigestMethodType(
                                stream,
                                &mut (*ReferenceType).DigestMethod,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 52 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            49 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*ReferenceType).URI.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*ReferenceType).URI.unwrap().len as i32 >= 2 as i32 {
                                    (*ReferenceType).URI.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*ReferenceType).URI.unwrap().len as usize,
                                        &mut (*ReferenceType).URI.unwrap().data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 50 as i32;
                        }
                        1 => {
                            decode_iso2_TransformsType(
                                stream,
                                &mut (*ReferenceType).Transforms.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 51 as i32;
                            }
                        }
                        2 => {
                            decode_iso2_DigestMethodType(
                                stream,
                                &mut (*ReferenceType).DigestMethod,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 52 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            50 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_TransformsType(
                                stream,
                                &mut (*ReferenceType).Transforms.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 51 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_DigestMethodType(
                                stream,
                                &mut (*ReferenceType).DigestMethod,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 52 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            51 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_DigestMethodType(
                                stream,
                                &mut (*ReferenceType).DigestMethod,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 52 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            52 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*ReferenceType).DigestValue.unwrap().len,
                                &mut (*ReferenceType).DigestValue.unwrap().data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_RetrievalMethodType(
    stream: &mut ExiBitstream,
    mut RetrievalMethodType: &mut Iso2RetrievalMethodType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 53 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            53 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*RetrievalMethodType).Type.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*RetrievalMethodType).Type.unwrap().len as i32 >= 2 as i32 {
                                    (*RetrievalMethodType).Type.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*RetrievalMethodType).Type.unwrap().len as usize,
                                        &mut (*RetrievalMethodType).Type.unwrap().data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 54 as i32;
                        }
                        1 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*RetrievalMethodType).URI.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*RetrievalMethodType).URI.unwrap().len as i32 >= 2 as i32 {
                                    (*RetrievalMethodType).URI.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*RetrievalMethodType).URI.unwrap().len as usize,
                                        &mut (*RetrievalMethodType).URI.unwrap().data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 55 as i32;
                        }
                        2 => {
                            decode_iso2_TransformsType(
                                stream,
                                &mut (*RetrievalMethodType).Transforms.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        3 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            54 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*RetrievalMethodType).URI.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*RetrievalMethodType).URI.unwrap().len as i32 >= 2 as i32 {
                                    (*RetrievalMethodType).URI.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*RetrievalMethodType).URI.unwrap().len as usize,
                                        &mut (*RetrievalMethodType).URI.unwrap().data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 55 as i32;
                        }
                        1 => {
                            decode_iso2_TransformsType(
                                stream,
                                &mut (*RetrievalMethodType).Transforms.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            55 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_TransformsType(
                                stream,
                                &mut (*RetrievalMethodType).Transforms.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_SalesTariffType(
    stream: &mut ExiBitstream,
    mut SalesTariffType: &mut Iso2SalesTariffType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 56 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            56 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*SalesTariffType).Id.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*SalesTariffType).Id.unwrap().len as i32 >= 2 as i32 {
                                    (*SalesTariffType).Id.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*SalesTariffType).Id.unwrap().len as usize,
                                        &mut (*SalesTariffType).Id.unwrap().data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 57 as i32;
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        8 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*SalesTariffType).SalesTariffID =
                                            value.wrapping_add(1 as i32 as u32) as u8;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 58 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            57 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        8 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*SalesTariffType).SalesTariffID =
                                            value_0.wrapping_add(1 as i32 as u32) as u8;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 58 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            58 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*SalesTariffType)
                                            .SalesTariffDescription
                                            .unwrap().len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*SalesTariffType).SalesTariffDescription.unwrap().len
                                            as i32
                                            >= 2 as i32
                                        {
                                            (*SalesTariffType)
                                                .SalesTariffDescription
                                                .unwrap().len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*SalesTariffType)
                                                    .SalesTariffDescription
                                                    .unwrap().len,
                                                &mut (*SalesTariffType)
                                                    .SalesTariffDescription
                                                    .unwrap().data,
                                                (32 as i32 + 1 as i32) as usize,
                                            )?;
                                        } else {
                                            return Err(STRINGVALUES_NOT_SUPPORTED);
                                        }
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 60 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_1: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        8 as i32 as usize,
                                        &mut value_1,
                                    )?;
                                    if error == 0 as i32 {
                                        (*SalesTariffType).NumEPriceLevels = Some(value_1 as u8);
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 62 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        2 => {
                            if ((*SalesTariffType).SalesTariffEntry.arrayLen as i32) < 12 as i32 {
                                let fresh8 = (*SalesTariffType).SalesTariffEntry.arrayLen;
                                (*SalesTariffType).SalesTariffEntry.arrayLen =
                                    ((*SalesTariffType).SalesTariffEntry.arrayLen).wrapping_add(1);
                                if let Some(entry) = (*SalesTariffType)
                                    .SalesTariffEntry
                                    .array
                                    .get_mut(fresh8 as usize)
                                {
                                    decode_iso2_SalesTariffEntryType(stream, entry)?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            grammar_id = 59 as i32;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            59 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SalesTariffType).SalesTariffEntry.arrayLen as i32) < 12 as i32 {
                                let fresh9 = (*SalesTariffType).SalesTariffEntry.arrayLen;
                                (*SalesTariffType).SalesTariffEntry.arrayLen =
                                    ((*SalesTariffType).SalesTariffEntry.arrayLen).wrapping_add(1);
                                if let Some(entry) = (*SalesTariffType)
                                    .SalesTariffEntry
                                    .array
                                    .get_mut(fresh9 as usize)
                                {
                                    decode_iso2_SalesTariffEntryType(stream, entry)?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            if ((*SalesTariffType).SalesTariffEntry.arrayLen as i32) < 1024 as i32 {
                                grammar_id = 59 as i32;
                            } else {
                                grammar_id = 60 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            60 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_2: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        8 as i32 as usize,
                                        &mut value_2,
                                    )?;
                                    if error == 0 as i32 {
                                        (*SalesTariffType).NumEPriceLevels = Some(value_2 as u8);
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 62 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        1 => {
                            if ((*SalesTariffType).SalesTariffEntry.arrayLen as i32) < 12 as i32 {
                                let fresh10 = (*SalesTariffType).SalesTariffEntry.arrayLen;
                                (*SalesTariffType).SalesTariffEntry.arrayLen =
                                    ((*SalesTariffType).SalesTariffEntry.arrayLen).wrapping_add(1);
                                if let Some(entry) = (*SalesTariffType)
                                    .SalesTariffEntry
                                    .array
                                    .get_mut(fresh10 as usize)
                                {
                                    decode_iso2_SalesTariffEntryType(stream, entry)?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            grammar_id = 61 as i32;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            61 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SalesTariffType).SalesTariffEntry.arrayLen as i32) < 12 as i32 {
                                let fresh11 = (*SalesTariffType).SalesTariffEntry.arrayLen;
                                (*SalesTariffType).SalesTariffEntry.arrayLen =
                                    ((*SalesTariffType).SalesTariffEntry.arrayLen).wrapping_add(1);
                                if let Some(entry) = (*SalesTariffType)
                                    .SalesTariffEntry
                                    .array
                                    .get_mut(fresh11 as usize)
                                {
                                    decode_iso2_SalesTariffEntryType(stream, entry)?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            if ((*SalesTariffType).SalesTariffEntry.arrayLen as i32) < 1024 as i32 {
                                grammar_id = 61 as i32;
                            } else {
                                grammar_id = 62 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            62 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SalesTariffType).SalesTariffEntry.arrayLen as i32) < 12 as i32 {
                                let fresh12 = (*SalesTariffType).SalesTariffEntry.arrayLen;
                                (*SalesTariffType).SalesTariffEntry.arrayLen =
                                    ((*SalesTariffType).SalesTariffEntry.arrayLen).wrapping_add(1);
                                if let Some(entry) = (*SalesTariffType)
                                    .SalesTariffEntry
                                    .array
                                    .get_mut(fresh12 as usize)
                                {
                                    decode_iso2_SalesTariffEntryType(stream, entry)?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            grammar_id = 63 as i32;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            63 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SalesTariffType).SalesTariffEntry.arrayLen as i32) < 12 as i32 {
                                let fresh13 = (*SalesTariffType).SalesTariffEntry.arrayLen;
                                (*SalesTariffType).SalesTariffEntry.arrayLen =
                                    ((*SalesTariffType).SalesTariffEntry.arrayLen).wrapping_add(1);
                                if let Some(entry) = (*SalesTariffType)
                                    .SalesTariffEntry
                                    .array
                                    .get_mut(fresh13 as usize)
                                {
                                    decode_iso2_SalesTariffEntryType(stream, entry)?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            if ((*SalesTariffType).SalesTariffEntry.arrayLen as i32) < 1024 as i32 {
                                grammar_id = 63 as i32;
                            } else {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_X509DataType(
    stream: &mut ExiBitstream,
    mut X509DataType: &mut Iso2X509DataType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 64 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            64 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_X509IssuerSerialType(
                                stream,
                                &mut (*X509DataType).X509IssuerSerial.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*X509DataType).X509SKI.unwrap().len,
                                &mut (*X509DataType).X509SKI.unwrap().data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*X509DataType).X509SubjectName.unwrap().len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*X509DataType).X509SubjectName.unwrap().len as i32
                                            >= 2 as i32
                                        {
                                            (*X509DataType).X509SubjectName.unwrap().len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*X509DataType).X509SubjectName.unwrap().len
                                                    as usize,
                                                &mut (*X509DataType).X509SubjectName.unwrap().data,
                                                (64 as i32 + 1 as i32) as usize,
                                            )?;
                                        } else {
                                            return Err(STRINGVALUES_NOT_SUPPORTED);
                                        }
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        3 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*X509DataType).X509Certificate.unwrap().len,
                                &mut (*X509DataType).X509Certificate.unwrap().data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        4 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*X509DataType).X509CRL.unwrap().len,
                                &mut (*X509DataType).X509CRL.unwrap().data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        5 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*X509DataType).ANY.unwrap().len,
                                &mut (*X509DataType).ANY.unwrap().data,
                                4 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_PGPDataType(
    stream: &mut ExiBitstream,
    mut PGPDataType: &mut Iso2PGPDataType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 65 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            65 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            let pgp_data = match &mut (*PGPDataType).PGPComponent {
                                Iso2PGPComponentType::Choice1(ref mut c1) => c1,
                                _ => return Err(UNKNOWN_EVENT_CODE),
                            };
                            decode_exi_type_hex_binary(
                                stream,
                                &mut pgp_data.PGPKeyID.len,
                                &mut pgp_data.PGPKeyID.data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 66 as i32;
                            }
                        }
                        1 => {
                            let pgp_data = match &mut (*PGPDataType).PGPComponent {
                                Iso2PGPComponentType::Choice1(ref mut c1) => c1,
                                _ => return Err(UNKNOWN_EVENT_CODE),
                            };
                            decode_exi_type_hex_binary(
                                stream,
                                &mut pgp_data.PGPKeyPacket.unwrap().len,
                                &mut pgp_data.PGPKeyPacket.unwrap().data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 67 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            66 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            let pgp_data = match &mut (*PGPDataType).PGPComponent {
                                Iso2PGPComponentType::Choice1(ref mut c1) => c1,
                                _ => return Err(UNKNOWN_EVENT_CODE),
                            };
                            decode_exi_type_hex_binary(
                                stream,
                                &mut pgp_data.PGPKeyPacket.unwrap().len,
                                &mut pgp_data.PGPKeyPacket.unwrap().data,
                                350 as i32 as usize,
                            )?;
                            grammar_id = 67 as i32;
                        }
                        1 => {
                            return Err(UNKNOWN_EVENT_FOR_DECODING);
                        }
                        2 => {
                            return Ok(NO_ERROR);
                        }
                        3 => {
                            let pgp_data = match &mut (*PGPDataType).PGPComponent {
                                Iso2PGPComponentType::Choice1(ref mut c1) => c1,
                                _ => return Err(UNKNOWN_EVENT_CODE),
                            };
                            decode_exi_type_hex_binary(
                                stream,
                                &mut pgp_data.ANY.unwrap().len,
                                &mut pgp_data.ANY.unwrap().data,
                                4 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 68 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            67 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Err(UNKNOWN_EVENT_FOR_DECODING);
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        2 => {
                            return Ok(NO_ERROR);
                        }
                        3 => {
                            let pgp_data = match &mut (*PGPDataType).PGPComponent {
                                Iso2PGPComponentType::Choice1(ref mut c1) => c1,
                                _ => return Err(UNKNOWN_EVENT_CODE),
                            };
                            decode_exi_type_hex_binary(
                                stream,
                                &mut pgp_data.ANY.unwrap().len,
                                &mut pgp_data.ANY.unwrap().data,
                                4 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 68 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            68 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            // Ensure we are using PGPChoice2Type
                            let pgp_data = match &mut (*PGPDataType).PGPComponent {
                                Iso2PGPComponentType::Choice2(ref mut c2) => c2,
                                _ => return Err(UNKNOWN_EVENT_CODE),
                            };

                            decode_exi_type_hex_binary(
                                stream,
                                &mut pgp_data.PGPKeyPacket.len,
                                &mut pgp_data.PGPKeyPacket.data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 69 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            69 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Err(UNKNOWN_EVENT_FOR_DECODING);
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        2 => {
                            let pgp_data = match &mut (*PGPDataType).PGPComponent {
                                Iso2PGPComponentType::Choice2(ref mut c2) => c2,
                                _ => return Err(UNKNOWN_EVENT_CODE),
                            };
                            decode_exi_type_hex_binary(
                                stream,
                                &mut pgp_data.ANY.unwrap().len,
                                &mut pgp_data.ANY.unwrap().data,
                                4
                            )?;
                            grammar_id = 68 as i32;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_SPKIDataType(
    stream: &mut ExiBitstream,
    mut SPKIDataType: &mut Iso2SPKIDataType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 70 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            70 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*SPKIDataType).SPKISexp.len,
                                &mut (*SPKIDataType).SPKISexp.data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 71 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            71 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Err(UNKNOWN_EVENT_FOR_DECODING);
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        2 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*SPKIDataType).ANY.unwrap().len,
                                &mut (*SPKIDataType).ANY.unwrap().data,
                                4 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_SignedInfoType(
    stream: &mut ExiBitstream,
    mut SignedInfoType: &mut Iso2SignedInfoType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 72 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            72 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*SignedInfoType).Id.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*SignedInfoType).Id.unwrap().len as i32 >= 2 as i32 {
                                    (*SignedInfoType).Id.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*SignedInfoType).Id.unwrap().len,
                                        &mut (*SignedInfoType).Id.unwrap().data,
                                        65,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 73 as i32;
                        }
                        1 => {
                            decode_iso2_CanonicalizationMethodType(
                                stream,
                                &mut (*SignedInfoType).CanonicalizationMethod,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 74 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            73 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_CanonicalizationMethodType(
                                stream,
                                &mut (*SignedInfoType).CanonicalizationMethod,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 74 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            74 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_SignatureMethodType(
                                stream,
                                &mut (*SignedInfoType).SignatureMethod,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 75 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            75 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if (*SignedInfoType).ReferenceLen < 4 {
                                let fresh14 = (*SignedInfoType).ReferenceLen;
                                if let Some(reference) = (*SignedInfoType)
                                    .Reference
                                    .get_mut(fresh14 as usize)
                                {
                                    decode_iso2_ReferenceType(stream, reference)?;
                                    (*SignedInfoType).ReferenceLen += 1;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            grammar_id = 76 as i32;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            76 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SignedInfoType).ReferenceLen as i32) < 4 as i32 {
                                let fresh15 = (*SignedInfoType).ReferenceLen;
                                (*SignedInfoType).ReferenceLen =
                                    ((*SignedInfoType).ReferenceLen).wrapping_add(1);
                                if let Some(reference) = (*SignedInfoType)
                                    .Reference
                                    .get_mut(fresh15 as usize)
                                {
                                    decode_iso2_ReferenceType(stream, reference)?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            grammar_id = 76 as i32;
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_ProfileEntryType(
    stream: &mut ExiBitstream,
    mut ProfileEntryType: &mut Iso2ProfileEntryType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 77 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            77 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_uint32(
                                stream,
                                &mut (*ProfileEntryType).ChargingProfileEntryStart,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 78 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            78 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*ProfileEntryType).ChargingProfileEntryMaxPower,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 79 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            79 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ProfileEntryType)
                                            .ChargingProfileEntryMaxNumberOfPhasesInUse =
                                            Some(value.wrapping_add(1) as i8);
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_DC_EVStatusType(
    stream: &mut ExiBitstream,
    mut DC_EVStatusType: &mut Iso2DCEVStatusType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 80 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            80 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*DC_EVStatusType).EVReady = value as i32;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 81 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            81 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        4 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*DC_EVStatusType).EVErrorCode =
                                            value_0 as Iso2DcEvErrorCodeType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 82 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            82 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_1: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_1,
                                    )?;
                                    if error == 0 as i32 {
                                        (*DC_EVStatusType).EVRESSSOC = value_1 as i8;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_ParameterSetType(
    stream: &mut ExiBitstream,
    mut ParameterSetType: &mut Iso2ParameterSetType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 83 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            83 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_integer16(
                                stream,
                                &mut (*ParameterSetType).ParameterSetID,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 84 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            84 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ParameterSetType).Parameter.arrayLen as i32) < 16 as i32 {
                                let fresh16 = (*ParameterSetType).Parameter.arrayLen;
                                (*ParameterSetType).Parameter.arrayLen =
                                    ((*ParameterSetType).Parameter.arrayLen).wrapping_add(1);
                                if let Some(param) = (*ParameterSetType).Parameter.array.get_mut(fresh16 as usize) {
                                    decode_iso2_ParameterType(
                                        stream,
                                        param,
                                    )?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            grammar_id = 85 as i32;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            85 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ParameterSetType).Parameter.arrayLen as i32) < 16 as i32 {
                                let fresh17 = (*ParameterSetType).Parameter.arrayLen;
                                (*ParameterSetType).Parameter.arrayLen =
                                    ((*ParameterSetType).Parameter.arrayLen).wrapping_add(1);
                                if let Some(param) = (*ParameterSetType).Parameter.array.get_mut(fresh17 as usize) {
                                    decode_iso2_ParameterType(
                                        stream,
                                        param,
                                    )?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            if ((*ParameterSetType).Parameter.arrayLen as i32) < 16 as i32 {
                                grammar_id = 85 as i32;
                            } else {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_SAScheduleTupleType(
    stream: &mut ExiBitstream,
    mut SAScheduleTupleType: &mut Iso2SAScheduleTupleType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 86 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            86 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        8 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*SAScheduleTupleType).SAScheduleTupleID =
                                            value.wrapping_add(1 as i32 as u32) as u8;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 87 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            87 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PMaxScheduleType(
                                stream,
                                &mut (*SAScheduleTupleType).PMaxSchedule,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 88 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            88 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_SalesTariffType(
                                stream,
                                &mut (*SAScheduleTupleType).SalesTariff.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_SelectedServiceType(
    stream: &mut ExiBitstream,
    mut SelectedServiceType: &mut Iso2SelectedServiceType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 89 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            89 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_uint16(
                                stream,
                                &mut (*SelectedServiceType).ServiceID,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 90 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            90 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_integer16(
                                stream,
                                &mut (*SelectedServiceType).ParameterSetID.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_ServiceType(
    stream: &mut ExiBitstream,
    mut ServiceType: &mut Iso2ServiceType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 91 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            91 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_uint16(stream, &mut (*ServiceType).ServiceID)?;
                            if error == 0 as i32 {
                                grammar_id = 92 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            92 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*ServiceType).ServiceName.unwrap().len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*ServiceType).ServiceName.unwrap().len as i32
                                            >= 2 as i32
                                        {
                                            (*ServiceType).ServiceName.unwrap().len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*ServiceType).ServiceName.unwrap().len,
                                                &mut (*ServiceType).ServiceName.unwrap().data,
                                                33,
                                            )?;
                                        } else {
                                            return Err(STRINGVALUES_NOT_SUPPORTED);
                                        }
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 93 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ServiceType).ServiceCategory =
                                            value as Iso2ServiceCategoryType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 94 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            93 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ServiceType).ServiceCategory =
                                            value_0 as Iso2ServiceCategoryType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 94 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            94 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*ServiceType).ServiceScope.unwrap().len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*ServiceType).ServiceScope.unwrap().len as i32
                                            >= 2 as i32
                                        {
                                            (*ServiceType).ServiceScope.unwrap().len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*ServiceType).ServiceScope.unwrap().len,
                                                &mut (*ServiceType).ServiceScope.unwrap().data,
                                                65,
                                            )?;
                                        } else {
                                            return Err(STRINGVALUES_NOT_SUPPORTED);
                                        }
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 95 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_1: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_1,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ServiceType).FreeService = value_1 as i32;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            95 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_2: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_2,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ServiceType).FreeService = value_2 as i32;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_SignatureValueType(
    stream: &mut ExiBitstream,
    mut SignatureValueType: &mut Iso2SignatureValueType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 96 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            96 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*SignatureValueType).Id.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*SignatureValueType).Id.unwrap().len as i32 >= 2 as i32 {
                                    (*SignatureValueType).Id.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*SignatureValueType).Id.unwrap().len as usize,
                                        &mut (*SignatureValueType).Id.unwrap().data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 97 as i32;
                        }
                        1 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*SignatureValueType).CONTENT.len as u16),
                            )?;
                            if error == 0 as i32 {
                                exi_basetypes_decoder_bytes(
                                    stream,
                                    (*SignatureValueType).CONTENT.len as usize,
                                    &mut (*SignatureValueType).CONTENT.data,
                                )?;
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            97 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*SignatureValueType).CONTENT.len as u16),
                            )?;
                            if error == 0 as i32 {
                                exi_basetypes_decoder_bytes(
                                    stream,
                                    (*SignatureValueType).CONTENT.len as usize,
                                    &mut (*SignatureValueType).CONTENT.data,
                                )?;
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_SubCertificatesType(
    stream: &mut ExiBitstream,
    mut SubCertificatesType: &mut Iso2SubCertificatesType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 98 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            98 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SubCertificatesType).Certificate.arrayLen as i32) < 4 as i32 {
                                let idx = (*SubCertificatesType).Certificate.arrayLen as usize;
                                if let Some(cert) = (*SubCertificatesType).Certificate.array.get_mut(idx) {
                                    decode_exi_type_hex_binary(
                                        stream,
                                        &mut cert.len,
                                        &mut cert.data,
                                        800 as usize,
                                    )?;
                                    if error == 0 as i32 {
                                        (*SubCertificatesType).Certificate.arrayLen =
                                            ((*SubCertificatesType).Certificate.arrayLen)
                                                .wrapping_add(1);
                                        grammar_id = 99 as i32;
                                    }
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            99 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SubCertificatesType).Certificate.arrayLen as i32) < 4 as i32 {
                                let idx = (*SubCertificatesType).Certificate.arrayLen as usize;
                                if let Some(cert) = (*SubCertificatesType).Certificate.array.get_mut(idx) {
                                    decode_exi_type_hex_binary(
                                        stream,
                                        &mut cert.len,
                                        &mut cert.data,
                                        800 as usize,
                                    )?;
                                    if error == 0 as i32 {
                                        (*SubCertificatesType).Certificate.arrayLen =
                                            ((*SubCertificatesType).Certificate.arrayLen)
                                                .wrapping_add(1);
                                        grammar_id = 99 as i32;
                                    }
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_KeyInfoType(
    stream: &mut ExiBitstream,
    mut KeyInfoType: &mut Iso2KeyInfoType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 100 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            100 => {
                exi_basetypes_decoder_nbit_uint(stream, 4 as i32 as usize, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*KeyInfoType).Id.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*KeyInfoType).Id.unwrap().len as i32 >= 2 as i32 {
                                    (*KeyInfoType).Id.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*KeyInfoType).Id.unwrap().len,
                                        &mut (*KeyInfoType).Id.unwrap().data,
                                        65,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 101 as i32;
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*KeyInfoType).KeyName.unwrap().len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*KeyInfoType).KeyName.unwrap().len as i32 >= 2 as i32 {
                                            (*KeyInfoType).KeyName.unwrap().len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*KeyInfoType).KeyName.unwrap().len as usize,
                                                &mut (*KeyInfoType).KeyName.unwrap().data,
                                                (64 as i32 + 1 as i32) as usize,
                                            )?;
                                        } else {
                                            return Err(STRINGVALUES_NOT_SUPPORTED);
                                        }
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        2 => {
                            decode_iso2_KeyValueType(stream, &mut (*KeyInfoType).KeyValue.unwrap())?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        3 => {
                            decode_iso2_RetrievalMethodType(
                                stream,
                                &mut (*KeyInfoType).RetrievalMethod.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        4 => {
                            decode_iso2_X509DataType(stream, &mut (*KeyInfoType).X509Data.unwrap())?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        5 => {
                            decode_iso2_PGPDataType(stream, &mut (*KeyInfoType).PGPData.unwrap())?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        6 => {
                            decode_iso2_SPKIDataType(stream, &mut (*KeyInfoType).SPKIData.unwrap())?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        7 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mgmt_data = (*KeyInfoType).MgmtData.as_mut().unwrap();
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut (mgmt_data.len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if mgmt_data.len >= 2 {
                                            mgmt_data.len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                mgmt_data.len as usize,
                                                &mut mgmt_data.data,
                                                65,
                                            )?;
                                        } else {
                                            return Err(STRINGVALUES_NOT_SUPPORTED);
                                        }
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        8 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*KeyInfoType).ANY.unwrap().len,
                                &mut (*KeyInfoType).ANY.unwrap().data,
                                4,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            101 => {
                exi_basetypes_decoder_nbit_uint(stream, 4 as i32 as usize, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*KeyInfoType).KeyName.unwrap().len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*KeyInfoType).KeyName.unwrap().len as i32 >= 2 as i32 {
                                            (*KeyInfoType).KeyName.unwrap().len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*KeyInfoType).KeyName.unwrap().len,
                                                &mut (*KeyInfoType).KeyName.unwrap().data,
                                                65,
                                            )?;
                                        } else {
                                            return Err(STRINGVALUES_NOT_SUPPORTED);
                                        }
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        1 => {
                            decode_iso2_KeyValueType(stream, &mut (*KeyInfoType).KeyValue.unwrap())?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            decode_iso2_RetrievalMethodType(
                                stream,
                                &mut (*KeyInfoType).RetrievalMethod.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        3 => {
                            decode_iso2_X509DataType(stream, &mut (*KeyInfoType).X509Data.unwrap())?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        4 => {
                            decode_iso2_PGPDataType(stream, &mut (*KeyInfoType).PGPData.unwrap())?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        5 => {
                            decode_iso2_SPKIDataType(stream, &mut (*KeyInfoType).SPKIData.unwrap())?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        6 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*KeyInfoType).MgmtData.unwrap().len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*KeyInfoType).MgmtData.unwrap().len as i32 >= 2 as i32
                                        {
                                            (*KeyInfoType).MgmtData.unwrap().len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*KeyInfoType).MgmtData.unwrap().len as usize,
                                                &mut (*KeyInfoType).MgmtData.unwrap().data,
                                                (64 as i32 + 1 as i32) as usize,
                                            )?;
                                        } else {
                                            return Err(STRINGVALUES_NOT_SUPPORTED);
                                        }
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        7 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*KeyInfoType).ANY.unwrap().len,
                                &mut (*KeyInfoType).ANY.unwrap().data,
                                4 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_ObjectType(
    stream: &mut ExiBitstream,
    mut ObjectType: &mut Iso2ObjectType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 102 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            102 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*ObjectType).Encoding.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*ObjectType).Encoding.unwrap().len as i32 >= 2 as i32 {
                                    (*ObjectType).Encoding.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*ObjectType).Encoding.unwrap().len,
                                        &mut (*ObjectType).Encoding.unwrap().data,
                                        65,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 103 as i32;
                        }
                        1 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*ObjectType).Id.as_mut().unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*ObjectType).Id.as_ref().unwrap().len as i32 >= 2 as i32 {
                                    (*ObjectType).Id.as_mut().unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*ObjectType).Id.as_ref().unwrap().len,
                                        &mut (*ObjectType).Id.as_mut().unwrap().data,
                                        65,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 104 as i32;
                        }
                        2 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*ObjectType).MimeType.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*ObjectType).MimeType.unwrap().len as i32 >= 2 as i32 {
                                    (*ObjectType).MimeType.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*ObjectType).MimeType.unwrap().len,
                                        &mut (*ObjectType).MimeType.unwrap().data,
                                        65,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 105 as i32;
                        }
                        3 => {
                            return Err(UNKNOWN_EVENT_FOR_DECODING);
                        }
                        4 => {
                            return Ok(NO_ERROR);
                        }
                        5 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*ObjectType).ANY.unwrap().len,
                                &mut (*ObjectType).ANY.unwrap().data,
                                4 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            103 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*ObjectType).Id.unwrap().len as u16)
                            )?;
                            if error == 0 as i32 {
                                if (*ObjectType).Id.unwrap().len as i32 >= 2 as i32 {
                                    (*ObjectType).Id.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*ObjectType).Id.unwrap().len,
                                        &mut (*ObjectType).Id.unwrap().data,
                                        65,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 104 as i32;
                        }
                        1 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*ObjectType).MimeType.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*ObjectType).MimeType.unwrap().len as i32 >= 2 as i32 {
                                    (*ObjectType).MimeType.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*ObjectType).MimeType.unwrap().len,
                                        &mut (*ObjectType).MimeType.unwrap().data,
                                        65,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 105 as i32;
                        }
                        2 => {
                            return Err(UNKNOWN_EVENT_FOR_DECODING);
                        }
                        3 => {
                            return Ok(NO_ERROR);
                        }
                        4 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*ObjectType).ANY.unwrap().len,
                                &mut (*ObjectType).ANY.unwrap().data,
                                4,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            104 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*ObjectType).MimeType.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*ObjectType).MimeType.unwrap().len as i32 >= 2 as i32 {
                                    (*ObjectType).MimeType.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*ObjectType).MimeType.unwrap().len,
                                        &mut (*ObjectType).MimeType.unwrap().data,
                                        65,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 105 as i32;
                        }
                        1 => {
                            return Err(UNKNOWN_EVENT_FOR_DECODING);
                        }
                        2 => {
                            return Ok(NO_ERROR);
                        }
                        3 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*ObjectType).ANY.unwrap().len,
                                &mut (*ObjectType).ANY.unwrap().data,
                                4,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            105 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Err(UNKNOWN_EVENT_FOR_DECODING);
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        2 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*ObjectType).ANY.unwrap().len,
                                &mut (*ObjectType).ANY.unwrap().data,
                                4,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_SupportedEnergyTransferModeType(
    stream: &mut ExiBitstream,
    mut SupportedEnergyTransferModeType: &mut Iso2SupportedEnergyTransferModeType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 106 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            106 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SupportedEnergyTransferModeType)
                                .EnergyTransferMode
                                .arrayLen as i32)
                                < 6 as i32
                            {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        let mut value: u32 = 0;
                                        exi_basetypes_decoder_nbit_uint(
                                            stream,
                                            3 as i32 as usize,
                                            &mut value,
                                        )?;
                                        if error == 0 as i32 {
                                            (*SupportedEnergyTransferModeType)
                                                .EnergyTransferMode
                                                .array
                                                [(*SupportedEnergyTransferModeType)
                                                    .EnergyTransferMode
                                                    .arrayLen
                                                    as usize] = value as Iso2EnergyTransferModeType;
                                            (*SupportedEnergyTransferModeType)
                                                .EnergyTransferMode
                                                .arrayLen = ((*SupportedEnergyTransferModeType)
                                                .EnergyTransferMode
                                                .arrayLen)
                                                .wrapping_add(1);
                                            (*SupportedEnergyTransferModeType)
                                                .EnergyTransferMode
                                                .arrayLen;
                                        }
                                    } else {
                                        return Err(UNSUPPORTED_SUB_EVENT);
                                    }
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 107 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            107 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SupportedEnergyTransferModeType)
                                .EnergyTransferMode
                                .arrayLen as i32)
                                < 6 as i32
                            {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        let mut value_0: u32 = 0;
                                        exi_basetypes_decoder_nbit_uint(
                                            stream,
                                            3 as i32 as usize,
                                            &mut value_0,
                                        )?;
                                        if error == 0 as i32 {
                                            (*SupportedEnergyTransferModeType)
                                                .EnergyTransferMode
                                                .array
                                                [(*SupportedEnergyTransferModeType)
                                                    .EnergyTransferMode
                                                    .arrayLen
                                                    as usize] =
                                                value_0 as Iso2EnergyTransferModeType;
                                            (*SupportedEnergyTransferModeType)
                                                .EnergyTransferMode
                                                .arrayLen = ((*SupportedEnergyTransferModeType)
                                                .EnergyTransferMode
                                                .arrayLen)
                                                .wrapping_add(1);
                                            (*SupportedEnergyTransferModeType)
                                                .EnergyTransferMode
                                                .arrayLen;
                                        }
                                    } else {
                                        return Err(UNSUPPORTED_SUB_EVENT);
                                    }
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 107 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_CertificateChainType(
    stream: &mut ExiBitstream,
    mut CertificateChainType: &mut Iso2CertificateChainType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 108 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            108 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*CertificateChainType).Id.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*CertificateChainType).Id.unwrap().len as i32 >= 2 as i32 {
                                    (*CertificateChainType).Id.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*CertificateChainType).Id.unwrap().len,
                                        &mut (*CertificateChainType).Id.unwrap().data,
                                        65,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 109 as i32;
                        }
                        1 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*CertificateChainType).Certificate.len,
                                &mut (*CertificateChainType).Certificate.data,
                                800 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 110 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            109 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*CertificateChainType).Certificate.len,
                                &mut (*CertificateChainType).Certificate.data,
                                800 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 110 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            110 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_SubCertificatesType(
                                stream,
                                &mut (*CertificateChainType).SubCertificates.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_BodyBaseType(
    stream: &mut ExiBitstream,
    mut _BodyBaseType: &mut Iso2BodyBaseType,
) -> Result<u8, i16> {
    let mut eventCode: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
    if eventCode != 0 as i32 as u32 {
        return Err(UNKNOWN_EVENT_CODE);
    }
    return Ok(NO_ERROR);
}
pub fn decode_iso2_NotificationType(
    stream: &mut ExiBitstream,
    mut NotificationType: &mut Iso2NotificationType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 111 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            111 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*NotificationType).FaultCode = value as Iso2FaultCodeType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 112 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            112 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*NotificationType).FaultMsg.unwrap().len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*NotificationType).FaultMsg.unwrap().len as i32
                                            >= 2 as i32
                                        {
                                            (*NotificationType).FaultMsg.unwrap().len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*NotificationType).FaultMsg.unwrap().len,
                                                &mut (*NotificationType).FaultMsg.unwrap().data,
                                                65,
                                            )?;
                                        } else {
                                            return Err(STRINGVALUES_NOT_SUPPORTED);
                                        }
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_DC_EVSEStatusType(
    stream: &mut ExiBitstream,
    mut DC_EVSEStatusType: &mut Iso2DCEVSEStatusType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 113 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            113 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_uint16(
                                stream,
                                &mut (*DC_EVSEStatusType).NotificationMaxDelay,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 114 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            114 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*DC_EVSEStatusType).EVSENotification =
                                            value as Iso2EvseNotificationType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 115 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            115 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        3 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*DC_EVSEStatusType).EVSEIsolationStatus =
                                            Some(value_0 as Iso2IsolationLevelType);
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 116 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_1: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        4 as i32 as usize,
                                        &mut value_1,
                                    )?;
                                    if error == 0 as i32 {
                                        (*DC_EVSEStatusType).EVSEStatusCode =
                                            value_1 as Iso2DcEvseStatusCodeType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            116 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_2: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        4 as i32 as usize,
                                        &mut value_2,
                                    )?;
                                    if error == 0 as i32 {
                                        (*DC_EVSEStatusType).EVSEStatusCode =
                                            value_2 as Iso2DcEvseStatusCodeType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_SelectedServiceListType(
    stream: &mut ExiBitstream,
    mut SelectedServiceListType: &mut Iso2SelectedServiceListType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 117 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            117 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SelectedServiceListType).SelectedService.arrayLen as i32) < 16 as i32 {
                                let fresh18 = (*SelectedServiceListType).SelectedService.arrayLen;
                                (*SelectedServiceListType).SelectedService.arrayLen =
                                    ((*SelectedServiceListType).SelectedService.arrayLen).wrapping_add(1);
                                if let Some(selected_service) = (*SelectedServiceListType)
                                    .SelectedService
                                    .array
                                    .get_mut(fresh18 as usize)
                                {
                                    decode_iso2_SelectedServiceType(stream, selected_service)?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            grammar_id = 118 as i32;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            118 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SelectedServiceListType).SelectedService.arrayLen as i32)
                                < 16 as i32
                            {
                                let fresh19 = (*SelectedServiceListType).SelectedService.arrayLen;
                                (*SelectedServiceListType).SelectedService.arrayLen =
                                    ((*SelectedServiceListType).SelectedService.arrayLen)
                                        .wrapping_add(1);
                                if let Some(selected_service) = (*SelectedServiceListType)
                                    .SelectedService
                                    .array
                                    .get_mut(fresh19 as usize)
                                {
                                    decode_iso2_SelectedServiceType(stream, selected_service)?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            if ((*SelectedServiceListType).SelectedService.arrayLen as i32)
                                < 16 as i32
                            {
                                grammar_id = 118 as i32;
                            } else {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_PaymentOptionListType(
    stream: &mut ExiBitstream,
    mut PaymentOptionListType: &mut Iso2PaymentOptionListType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 119 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            119 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*PaymentOptionListType).PaymentOption.arrayLen as i32) < 2 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        let mut value: u32 = 0;
                                        exi_basetypes_decoder_nbit_uint(
                                            stream,
                                            1 as i32 as usize,
                                            &mut value,
                                        )?;
                                        if error == 0 as i32 {
                                            (*PaymentOptionListType).PaymentOption.array
                                                [(*PaymentOptionListType).PaymentOption.arrayLen
                                                    as usize] = value as Iso2PaymentOptionType;
                                            (*PaymentOptionListType).PaymentOption.arrayLen =
                                                ((*PaymentOptionListType).PaymentOption.arrayLen)
                                                    .wrapping_add(1);
                                            (*PaymentOptionListType).PaymentOption.arrayLen;
                                        }
                                    } else {
                                        return Err(UNSUPPORTED_SUB_EVENT);
                                    }
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 120 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            120 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*PaymentOptionListType).PaymentOption.arrayLen as i32) < 2 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        let mut value_0: u32 = 0;
                                        exi_basetypes_decoder_nbit_uint(
                                            stream,
                                            1 as i32 as usize,
                                            &mut value_0,
                                        )?;
                                        if error == 0 as i32 {
                                            (*PaymentOptionListType).PaymentOption.array
                                                [(*PaymentOptionListType).PaymentOption.arrayLen
                                                    as usize] = value_0 as Iso2PaymentOptionType;
                                            (*PaymentOptionListType).PaymentOption.arrayLen =
                                                ((*PaymentOptionListType).PaymentOption.arrayLen)
                                                    .wrapping_add(1);
                                            (*PaymentOptionListType).PaymentOption.arrayLen;
                                        }
                                    } else {
                                        return Err(UNSUPPORTED_SUB_EVENT);
                                    }
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_SignatureType(
    stream: &mut ExiBitstream,
    mut SignatureType: &mut Iso2SignatureType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 121 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            121 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*SignatureType).Id.unwrap().len as u16)
                            )?;
                            if error == 0 as i32 {
                                if (*SignatureType).Id.unwrap().len as i32 >= 2 as i32 {
                                    (*SignatureType).Id.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*SignatureType).Id.unwrap().len,
                                        &mut (*SignatureType).Id.unwrap().data,
                                        65,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 122 as i32;
                        }
                        1 => {
                            decode_iso2_SignedInfoType(
                                stream,
                                &mut (*SignatureType).SignedInfo,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 123 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            122 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_SignedInfoType(
                                stream,
                                &mut (*SignatureType).SignedInfo,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 123 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            123 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_SignatureValueType(
                                stream,
                                &mut (*SignatureType).SignatureValue,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 124 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            124 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_KeyInfoType(stream, &mut (*SignatureType).KeyInfo.unwrap())?;
                            if error == 0 as i32 {
                                grammar_id = 126 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_ObjectType(stream, &mut (*SignatureType).Object.unwrap())?;
                            if error == 0 as i32 {
                                grammar_id = 125 as i32;
                            }
                        }
                        2 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            125 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Err(ARRAY_OUT_OF_BOUNDS);
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            126 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_ObjectType(stream, &mut (*SignatureType).Object.unwrap())?;
                            if error == 0 as i32 {
                                grammar_id = 127 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            127 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Err(ARRAY_OUT_OF_BOUNDS);
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_ChargingProfileType(
    stream: &mut ExiBitstream,
    mut ChargingProfileType: &mut Iso2ChargingProfileType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 128 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            128 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ChargingProfileType).ProfileEntry.arrayLen as i32) < 24 as i32 {
                                let fresh20 = (*ChargingProfileType).ProfileEntry.arrayLen;
                                (*ChargingProfileType).ProfileEntry.arrayLen =
                                    ((*ChargingProfileType).ProfileEntry.arrayLen).wrapping_add(1);
                                if let Some(entry) = (*ChargingProfileType)
                                    .ProfileEntry
                                    .array
                                    .get_mut(fresh20 as usize)
                                {
                                    decode_iso2_ProfileEntryType(stream, entry)?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            grammar_id = 129 as i32;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            129 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ChargingProfileType).ProfileEntry.arrayLen as i32) < 24 as i32 {
                                let fresh21 = (*ChargingProfileType).ProfileEntry.arrayLen;
                                (*ChargingProfileType).ProfileEntry.arrayLen =
                                    ((*ChargingProfileType).ProfileEntry.arrayLen).wrapping_add(1);
                                if let Some(entry) = (*ChargingProfileType)
                                    .ProfileEntry
                                    .array
                                    .get_mut(fresh21 as usize)
                                {
                                    decode_iso2_ProfileEntryType(stream, entry)?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            if ((*ChargingProfileType).ProfileEntry.arrayLen as i32) < 24 as i32 {
                                grammar_id = 129 as i32;
                            } else {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_ServiceParameterListType(
    stream: &mut ExiBitstream,
    mut ServiceParameterListType: &mut Iso2ServiceParameterListType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 130 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            130 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ServiceParameterListType).ParameterSet.arrayLen as i32) < 5 as i32
                            {
                                let fresh22 = (*ServiceParameterListType).ParameterSet.arrayLen;
                                (*ServiceParameterListType).ParameterSet.arrayLen =
                                    ((*ServiceParameterListType).ParameterSet.arrayLen)
                                        .wrapping_add(1);
                                if let Some(param_set) = (*ServiceParameterListType)
                                    .ParameterSet
                                    .array
                                    .get_mut(fresh22 as usize)
                                {
                                    decode_iso2_ParameterSetType(stream, param_set)?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            grammar_id = 131 as i32;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            131 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ServiceParameterListType).ParameterSet.arrayLen as i32) < 5 as i32
                            {
                                let fresh23 = (*ServiceParameterListType).ParameterSet.arrayLen;
                                (*ServiceParameterListType).ParameterSet.arrayLen =
                                    ((*ServiceParameterListType).ParameterSet.arrayLen)
                                        .wrapping_add(1);
                                if let Some(param_set) = (*ServiceParameterListType)
                                    .ParameterSet
                                    .array
                                    .get_mut(fresh23 as usize)
                                {
                                    decode_iso2_ParameterSetType(stream, param_set)?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            if ((*ServiceParameterListType).ParameterSet.arrayLen as i32)
                                < 255 as i32
                            {
                                grammar_id = 131 as i32;
                            } else {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_ListOfRootCertificateIDsType(
    stream: &mut ExiBitstream,
    mut ListOfRootCertificateIDsType: &mut Iso2ListOfRootCertificateIDsType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 132 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            132 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ListOfRootCertificateIDsType).RootCertificateID.arrayLen as i32)
                                < 5 as i32
                            {
                                let fresh24 =
                                    (*ListOfRootCertificateIDsType).RootCertificateID.arrayLen;
                                (*ListOfRootCertificateIDsType).RootCertificateID.arrayLen =
                                    ((*ListOfRootCertificateIDsType).RootCertificateID.arrayLen)
                                        .wrapping_add(1);
                                if let Some(root_cert) = (*ListOfRootCertificateIDsType)
                                    .RootCertificateID
                                    .array
                                    .get_mut(fresh24 as usize)
                                {
                                    decode_iso2_X509IssuerSerialType(
                                        stream,
                                        root_cert,
                                    )?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            grammar_id = 133 as i32;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            133 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ListOfRootCertificateIDsType).RootCertificateID.arrayLen as i32)
                                < 5 as i32
                            {
                                let fresh25 =
                                    (*ListOfRootCertificateIDsType).RootCertificateID.arrayLen;
                                (*ListOfRootCertificateIDsType).RootCertificateID.arrayLen =
                                    ((*ListOfRootCertificateIDsType).RootCertificateID.arrayLen)
                                        .wrapping_add(1);
                                if let Some(root_cert) = (*ListOfRootCertificateIDsType)
                                    .RootCertificateID
                                    .array
                                    .get_mut(fresh25 as usize)
                                {
                                    decode_iso2_X509IssuerSerialType(
                                        stream,
                                        root_cert,
                                    )?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            if ((*ListOfRootCertificateIDsType).RootCertificateID.arrayLen as i32)
                                < 20 as i32
                            {
                                grammar_id = 133 as i32;
                            } else {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_AC_EVChargeParameterType(
    stream: &mut ExiBitstream,
    mut AC_EVChargeParameterType: &mut Iso2ACEVChargeParameterType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 134 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            134 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_uint32(
                                stream,
                                &mut (*AC_EVChargeParameterType).DepartureTime.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 135 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*AC_EVChargeParameterType).EAmount,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 136 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            135 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*AC_EVChargeParameterType).EAmount,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 136 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            136 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*AC_EVChargeParameterType).EVMaxVoltage,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 137 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            137 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*AC_EVChargeParameterType).EVMaxCurrent,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 138 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            138 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*AC_EVChargeParameterType).EVMinCurrent,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_DC_EVChargeParameterType(
    stream: &mut ExiBitstream,
    mut DC_EVChargeParameterType: &mut Iso2DCEVChargeParameterType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 139 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            139 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_uint32(
                                stream,
                                &mut (*DC_EVChargeParameterType).DepartureTime.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 140 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_DC_EVStatusType(
                                stream,
                                &mut (*DC_EVChargeParameterType).DC_EVStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 141 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            140 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_DC_EVStatusType(
                                stream,
                                &mut (*DC_EVChargeParameterType).DC_EVStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 141 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            141 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*DC_EVChargeParameterType).EVMaximumCurrentLimit,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 142 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            142 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*DC_EVChargeParameterType).EVMaximumPowerLimit.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 143 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*DC_EVChargeParameterType).EVMaximumVoltageLimit,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 144 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            143 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*DC_EVChargeParameterType).EVMaximumVoltageLimit,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 144 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            144 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*DC_EVChargeParameterType).EVEnergyCapacity.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 145 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*DC_EVChargeParameterType).EVEnergyRequest.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 146 as i32;
                            }
                        }
                        2 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*DC_EVChargeParameterType).FullSOC = Some(value as i8);
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 147 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        3 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*DC_EVChargeParameterType).BulkSOC = Some(value_0 as i8);
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        4 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            145 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*DC_EVChargeParameterType).EVEnergyRequest.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 146 as i32;
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_1: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_1,
                                    )?;
                                    if error == 0 as i32 {
                                        (*DC_EVChargeParameterType).FullSOC = Some(value_1 as i8);
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 147 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        2 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_2: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_2,
                                    )?;
                                    if error == 0 as i32 {
                                        (*DC_EVChargeParameterType).BulkSOC = Some(value_2 as i8);
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        3 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            146 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_3: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_3,
                                    )?;
                                    if error == 0 as i32 {
                                        (*DC_EVChargeParameterType).FullSOC = Some(value_3 as i8);
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 147 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_4: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_4,
                                    )?;
                                    if error == 0 as i32 {
                                        (*DC_EVChargeParameterType).BulkSOC = Some(value_4 as i8);
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        2 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            147 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_5: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_5,
                                    )?;
                                    if error == 0 as i32 {
                                        (*DC_EVChargeParameterType).BulkSOC = Some(value_5 as i8);
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_EVChargeParameterType(
    stream: &mut ExiBitstream,
    mut EVChargeParameterType: &mut Iso2EVChargeParameterType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 148 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            148 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_uint32(
                                stream,
                                &mut (*EVChargeParameterType).DepartureTime.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 149 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_AC_EVChargeParameterType(
                                stream,
                                &mut (*EVChargeParameterType).AC_EVChargeParameter,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 150 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            149 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_AC_EVChargeParameterType(
                                stream,
                                &mut (*EVChargeParameterType).AC_EVChargeParameter,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 150 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            150 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_DC_EVChargeParameterType(
                                stream,
                                &mut (*EVChargeParameterType).DC_EVChargeParameter,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_SASchedulesType(
    stream: &mut ExiBitstream,
    mut _SASchedulesType: &mut Iso2SASchedulesType,
) -> Result<u8, i16> {
    let mut eventCode: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
    if eventCode != 0 as i32 as u32 {
        return Err(UNKNOWN_EVENT_CODE);
    }
    return Ok(NO_ERROR);
}
pub fn decode_iso2_SAScheduleListType(
    stream: &mut ExiBitstream,
    mut SAScheduleListType: &mut Iso2SAScheduleListType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 151 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            151 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SAScheduleListType).SAScheduleTuple.arrayLen as i32) < 3 as i32 {
                                let fresh26 = (*SAScheduleListType).SAScheduleTuple.arrayLen;
                                (*SAScheduleListType).SAScheduleTuple.arrayLen =
                                    ((*SAScheduleListType).SAScheduleTuple.arrayLen)
                                        .wrapping_add(1);
                                if let Some(tuple) = (*SAScheduleListType)
                                    .SAScheduleTuple
                                    .array
                                    .get_mut(fresh26 as usize)
                                {
                                    decode_iso2_SAScheduleTupleType(stream, tuple)?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            grammar_id = 152 as i32;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            152 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SAScheduleListType).SAScheduleTuple.arrayLen as i32) < 3 as i32 {
                                let fresh27 = (*SAScheduleListType).SAScheduleTuple.arrayLen;
                                (*SAScheduleListType).SAScheduleTuple.arrayLen =
                                    ((*SAScheduleListType).SAScheduleTuple.arrayLen)
                                        .wrapping_add(1);
                                if let Some(tuple) = (*SAScheduleListType)
                                    .SAScheduleTuple
                                    .array
                                    .get_mut(fresh27 as usize)
                                {
                                    decode_iso2_SAScheduleTupleType(stream, tuple)?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            if ((*SAScheduleListType).SAScheduleTuple.arrayLen as i32) < 3 as i32 {
                                grammar_id = 152 as i32;
                            } else {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_ChargeServiceType(
    stream: &mut ExiBitstream,
    mut ChargeServiceType: &mut Iso2ChargeServiceType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 153 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            153 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_uint16(stream, &mut (*ChargeServiceType).ServiceID)?;
                            grammar_id = 154 as i32;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            154 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*ChargeServiceType).ServiceName.unwrap().len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*ChargeServiceType).ServiceName.unwrap().len as i32
                                            >= 2 as i32
                                        {
                                            (*ChargeServiceType).ServiceName.unwrap().len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*ChargeServiceType).ServiceName.unwrap().len,
                                                &mut (*ChargeServiceType).ServiceName.unwrap().data,
                                                33,
                                            )?;
                                        } else {
                                            return Err(STRINGVALUES_NOT_SUPPORTED);
                                        }
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 155 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ChargeServiceType).ServiceCategory =
                                            value as Iso2ServiceCategoryType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 156 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            155 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ChargeServiceType).ServiceCategory =
                                            value_0 as Iso2ServiceCategoryType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 156 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            156 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*ChargeServiceType).ServiceScope.unwrap().len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*ChargeServiceType).ServiceScope.unwrap().len >= 2 {
                                            (*ChargeServiceType).ServiceScope.unwrap().len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*ChargeServiceType).ServiceScope.unwrap().len
                                                    as usize,
                                                &mut (*ChargeServiceType).ServiceScope.unwrap().data,
                                                65,
                                            )?;
                                        } else {
                                            return Err(STRINGVALUES_NOT_SUPPORTED);
                                        }
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 157 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_1: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_1,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ChargeServiceType).FreeService = value_1 as i32;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 158 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            157 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_2: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_2,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ChargeServiceType).FreeService = value_2 as i32;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 158 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            158 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_SupportedEnergyTransferModeType(
                                stream,
                                &mut (*ChargeServiceType).SupportedEnergyTransferMode,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_EVPowerDeliveryParameterType(
    stream: &mut ExiBitstream,
    mut _EVPowerDeliveryParameterType: &mut Iso2EVPowerDeliveryParameterType,
) -> Result<u8, i16> {
    let mut eventCode: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
    if eventCode != 0 as i32 as u32 {
        return Err(UNKNOWN_EVENT_CODE);
    }
    return Ok(NO_ERROR);
}
pub fn decode_iso2_DC_EVPowerDeliveryParameterType(
    stream: &mut ExiBitstream,
    mut DC_EVPowerDeliveryParameterType: &mut Iso2DCEVPowerDeliveryParameterType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 159 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            159 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_DC_EVStatusType(
                                stream,
                                &mut (*DC_EVPowerDeliveryParameterType).DC_EVStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 160 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            160 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*DC_EVPowerDeliveryParameterType).BulkChargingComplete = Some(
                                            value as i32,
                                        );
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 161 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*DC_EVPowerDeliveryParameterType).ChargingComplete =
                                            value_0 as i32;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            161 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_1: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_1,
                                    )?;
                                    if error == 0 as i32 {
                                        (*DC_EVPowerDeliveryParameterType).ChargingComplete =
                                            value_1 as i32;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_ContractSignatureEncryptedPrivateKeyType(
    stream: &mut ExiBitstream,
    mut ContractSignatureEncryptedPrivateKeyType: &mut Iso2ContractSignatureEncryptedPrivateKeyType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 162 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            162 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*ContractSignatureEncryptedPrivateKeyType).Id.len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*ContractSignatureEncryptedPrivateKeyType).Id.len >= 2 {
                                    (*ContractSignatureEncryptedPrivateKeyType).Id.len -= 2;

                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*ContractSignatureEncryptedPrivateKeyType).Id.len
                                            as usize,
                                        &mut (*ContractSignatureEncryptedPrivateKeyType).Id.data,
                                        65,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 163 as i32;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            163 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*ContractSignatureEncryptedPrivateKeyType).CONTENT.len as u16),
                            )?;
                            if error == 0 as i32 {
                                exi_basetypes_decoder_bytes(
                                    stream,
                                    (*ContractSignatureEncryptedPrivateKeyType).CONTENT.len as usize,
                                    &mut (*ContractSignatureEncryptedPrivateKeyType).CONTENT.data,
                                )?;
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_EVSEChargeParameterType(
    stream: &mut ExiBitstream,
    mut _EVSEChargeParameterType: &mut Iso2EVSEChargeParameterType,
) -> Result<u8, i16> {
    let mut eventCode: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
    if eventCode != 0 as i32 as u32 {
        return Err(UNKNOWN_EVENT_CODE);
    }
    return Ok(NO_ERROR);
}
pub fn decode_iso2_DC_EVSEChargeParameterType(
    stream: &mut ExiBitstream,
    mut DC_EVSEChargeParameterType: &mut Iso2DCEVSEChargeParameterType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 164 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            164 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_DC_EVSEStatusType(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).DC_EVSEStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 165 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            165 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).EVSEMaximumCurrentLimit,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 166 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            166 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).EVSEMaximumPowerLimit,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 167 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            167 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).EVSEMaximumVoltageLimit,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 168 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            168 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).EVSEMinimumCurrentLimit,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 169 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            169 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).EVSEMinimumVoltageLimit,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 170 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            170 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).EVSECurrentRegulationTolerance.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 171 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).EVSEPeakCurrentRipple,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 172 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            171 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).EVSEPeakCurrentRipple,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 172 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            172 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).EVSEEnergyToBeDelivered.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_ServiceListType(
    stream: &mut ExiBitstream,
    mut ServiceListType: &mut Iso2ServiceListType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 173 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            173 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ServiceListType).Service.arrayLen as i32) < 8 as i32 {
                                let fresh28 = (*ServiceListType).Service.arrayLen;
                                (*ServiceListType).Service.arrayLen =
                                    ((*ServiceListType).Service.arrayLen).wrapping_add(1);
                                // Use safe borrows instead of pointer arithmetic
                                if let Some(service) = (*ServiceListType)
                                    .Service
                                    .array
                                    .get_mut(fresh28 as usize)
                                {
                                    decode_iso2_ServiceType(stream, service)?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            grammar_id = 174 as i32;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            174 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ServiceListType).Service.arrayLen as i32) < 8 as i32 {
                                let fresh29 = (*ServiceListType).Service.arrayLen;
                                (*ServiceListType).Service.arrayLen =
                                    ((*ServiceListType).Service.arrayLen).wrapping_add(1);
                                // Use safe borrows instead of pointer arithmetic
                                if let Some(service) = (*ServiceListType)
                                    .Service
                                    .array
                                    .get_mut(fresh29 as usize)
                                {
                                    decode_iso2_ServiceType(stream, service)?;
                                } else {
                                    return Err(ARRAY_OUT_OF_BOUNDS);
                                }
                            } else {
                                return Err(ARRAY_OUT_OF_BOUNDS);
                            }
                            if ((*ServiceListType).Service.arrayLen as i32) < 8 as i32 {
                                grammar_id = 174 as i32;
                            } else {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_DiffieHellmanPublickeyType(
    stream: &mut ExiBitstream,
    mut DiffieHellmanPublickeyType: &mut Iso2DiffieHellmanPublickeyType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 175 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            175 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*DiffieHellmanPublickeyType).Id.len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*DiffieHellmanPublickeyType).Id.len >= 2 {
                                    (*DiffieHellmanPublickeyType).Id.len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*DiffieHellmanPublickeyType).Id.len as usize,
                                        &mut (*DiffieHellmanPublickeyType).Id.data,
                                        65,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 176 as i32;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            176 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*DiffieHellmanPublickeyType).CONTENT.len as u16),
                            )?;
                            exi_basetypes_decoder_bytes(
                                stream,
                                DiffieHellmanPublickeyType.CONTENT .len,
                                &mut DiffieHellmanPublickeyType.CONTENT .data,
                            )?;
                            grammar_id = 3 as i32;

                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_EMAIDType(
    stream: &mut ExiBitstream,
    mut EMAIDType: &mut Iso2EMAIDType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 177 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            177 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*EMAIDType).Id.len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*EMAIDType).Id.len >= 2 {
                                    (*EMAIDType).Id.len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*EMAIDType).Id.len as usize,
                                        &mut (*EMAIDType).Id.data,
                                        65,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 178 as i32;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            178 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*EMAIDType).CONTENT.len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*EMAIDType).CONTENT.len >= 2 {
                                    (*EMAIDType).CONTENT.len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*EMAIDType).CONTENT.len as usize,
                                        &mut (*EMAIDType).CONTENT.data,
                                        65,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_AC_EVSEStatusType(
    stream: &mut ExiBitstream,
    mut AC_EVSEStatusType: &mut Iso2ACEVSEStatusType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 179 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            179 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_uint16(
                                stream,
                                &mut (*AC_EVSEStatusType).NotificationMaxDelay,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 180 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            180 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*AC_EVSEStatusType).EVSENotification =
                                            value as Iso2EvseNotificationType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 181 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            181 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*AC_EVSEStatusType).RCD = value_0 as i32;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_EVSEStatusType(
    stream: &mut ExiBitstream,
    mut EVSEStatusType: &mut Iso2EVSEStatusType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 182 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            182 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_uint16(
                                stream,
                                &mut (*EVSEStatusType).NotificationMaxDelay,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 183 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            183 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*EVSEStatusType).EVSENotification =
                                            value as Iso2EvseNotificationType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 184 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            184 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_AC_EVSEStatusType(
                                stream,
                                &mut (*EVSEStatusType).AC_EVSEStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 185 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            185 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_DC_EVSEStatusType(
                                stream,
                                &mut (*EVSEStatusType).DC_EVSEStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_AC_EVSEChargeParameterType(
    stream: &mut ExiBitstream,
    mut AC_EVSEChargeParameterType: &mut Iso2ACEVSEChargeParameterType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 186 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            186 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_AC_EVSEStatusType(
                                stream,
                                &mut (*AC_EVSEChargeParameterType).AC_EVSEStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 187 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            187 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*AC_EVSEChargeParameterType).EVSENominalVoltage,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 188 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            188 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*AC_EVSEChargeParameterType).EVSEMaxCurrent,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_MeterInfoType(
    stream: &mut ExiBitstream,
    mut MeterInfoType: &mut Iso2MeterInfoType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 189 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            189 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*MeterInfoType).MeterID.len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*MeterInfoType).MeterID.len >= 2 {
                                            (*MeterInfoType).MeterID.len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*MeterInfoType).MeterID.len as usize,
                                                &mut (*MeterInfoType).MeterID.data,
                                                33,
                                            )?;
                                        } else {
                                            return Err(STRINGVALUES_NOT_SUPPORTED);
                                        }
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 190 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            190 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_uint64(stream, &mut (*MeterInfoType).MeterReading.unwrap())?;
                            grammar_id = 191 as i32;
                        }
                        1 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*MeterInfoType).SigMeterReading.unwrap().len,
                                &mut (*MeterInfoType).SigMeterReading.unwrap().data,
                                64 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 192 as i32;
                            }
                        }
                        2 => {
                            decode_exi_type_integer16(
                                stream,
                                &mut (*MeterInfoType).MeterStatus.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 193 as i32;
                            }
                        }
                        3 => {
                            decode_exi_type_integer64(stream, &mut (*MeterInfoType).TMeter.unwrap())?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        4 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            191 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*MeterInfoType).SigMeterReading.unwrap().len,
                                &mut (*MeterInfoType).SigMeterReading.unwrap().data,
                                64 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 192 as i32;
                            }
                        }
                        1 => {
                            decode_exi_type_integer16(
                                stream,
                                &mut (*MeterInfoType).MeterStatus.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 193 as i32;
                            }
                        }
                        2 => {
                            decode_exi_type_integer64(stream, &mut (*MeterInfoType).TMeter.unwrap())?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        3 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            192 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_integer16(
                                stream,
                                &mut (*MeterInfoType).MeterStatus.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 193 as i32;
                            }
                        }
                        1 => {
                            decode_exi_type_integer64(stream, &mut (*MeterInfoType).TMeter.unwrap())?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            193 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_integer64(stream, &mut (*MeterInfoType).TMeter.unwrap())?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_MessageHeaderType(
    stream: &mut ExiBitstream,
    mut MessageHeaderType: &mut Iso2MessageHeaderType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 194 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            194 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*MessageHeaderType).SessionID.len,
                                &mut (*MessageHeaderType).SessionID.data,
                                8 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 195 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            195 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            // If notification is None skip
                            if (*MessageHeaderType).Notification.is_some() {
                                decode_iso2_NotificationType(
                                    stream,
                                    &mut (*MessageHeaderType).Notification.unwrap(),
                                )?;
                            }
                            if error == 0 as i32 {
                                grammar_id = 196 as i32;
                            }
                        }
                        1 => {
                            if (*MessageHeaderType).Signature.is_some() {
                                decode_iso2_SignatureType(
                                    stream,
                                    &mut (*MessageHeaderType).Signature.unwrap(),
                                )?;
                            }
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            196 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if (*MessageHeaderType).Signature.is_some() {
                                decode_iso2_SignatureType(
                                    stream,
                                    &mut (*MessageHeaderType).Signature.unwrap(),
                                )?;
                            }
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_PowerDeliveryReqType(
    stream: &mut ExiBitstream,
    mut PowerDeliveryReqType: &mut Iso2PowerDeliveryReqType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 197 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            197 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*PowerDeliveryReqType).ChargeProgress =
                                            value as Iso2ChargeProgressType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 198 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            198 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        8 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*PowerDeliveryReqType).SAScheduleTupleID =
                                            value_0.wrapping_add(1) as u8;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 199 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            199 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_ChargingProfileType(
                                stream,
                                &mut (*PowerDeliveryReqType).ChargingProfile.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 200 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_DC_EVPowerDeliveryParameterType(
                                stream,
                                &mut (*PowerDeliveryReqType).DC_EVPowerDeliveryParameter.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            decode_iso2_EVPowerDeliveryParameterType(
                                stream,
                                &mut (*PowerDeliveryReqType).EVPowerDeliveryParameter.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        3 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            200 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_DC_EVPowerDeliveryParameterType(
                                stream,
                                &mut (*PowerDeliveryReqType).DC_EVPowerDeliveryParameter.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_EVPowerDeliveryParameterType(
                                stream,
                                &mut (*PowerDeliveryReqType).EVPowerDeliveryParameter.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_CurrentDemandResType(
    stream: &mut ExiBitstream,
    mut CurrentDemandResType: &mut Iso2CurrentDemandResType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 201 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            201 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*CurrentDemandResType).ResponseCode =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 202 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            202 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_DC_EVSEStatusType(
                                stream,
                                &mut (*CurrentDemandResType).DC_EVSEStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 203 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            203 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandResType).EVSEPresentVoltage,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 204 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            204 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandResType).EVSEPresentCurrent,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 205 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            205 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*CurrentDemandResType).EVSECurrentLimitAchieved =
                                            value_0 as i32;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 206 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            206 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_1: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_1,
                                    )?;
                                    if error == 0 as i32 {
                                        (*CurrentDemandResType).EVSEVoltageLimitAchieved =
                                            value_1 as i32;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 207 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            207 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_2: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_2,
                                    )?;
                                    if error == 0 as i32 {
                                        (*CurrentDemandResType).EVSEPowerLimitAchieved =
                                            value_2 as i32;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 208 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            208 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandResType).EVSEMaximumVoltageLimit.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 209 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandResType).EVSEMaximumCurrentLimit.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 210 as i32;
                            }
                        }
                        2 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandResType).EVSEMaximumPowerLimit.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 211 as i32;
                            }
                        }
                        3 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*CurrentDemandResType).EVSEID.len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*CurrentDemandResType).EVSEID.len as i32
                                            >= 2 as i32
                                        {
                                            (*CurrentDemandResType).EVSEID.len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*CurrentDemandResType).EVSEID.len
                                                    as usize,
                                                &mut (*CurrentDemandResType).EVSEID.data,
                                                38,
                                            )?;
                                        } else {
                                            return Err(STRINGVALUES_NOT_SUPPORTED);
                                        }
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 212 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            209 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandResType).EVSEMaximumCurrentLimit.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 210 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandResType).EVSEMaximumPowerLimit.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 211 as i32;
                            }
                        }
                        2 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*CurrentDemandResType).EVSEID.len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*CurrentDemandResType).EVSEID.len as i32
                                            >= 2 as i32
                                        {
                                            (*CurrentDemandResType).EVSEID.len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*CurrentDemandResType).EVSEID.len
                                                    as usize,
                                                &mut (*CurrentDemandResType).EVSEID.data,
                                                38,
                                            )?;
                                        } else {
                                            return Err(STRINGVALUES_NOT_SUPPORTED);
                                        }
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 212 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            210 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandResType).EVSEMaximumPowerLimit.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 211 as i32;
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*CurrentDemandResType).EVSEID.len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*CurrentDemandResType).EVSEID.len as i32
                                            >= 2 as i32
                                        {
                                            (*CurrentDemandResType).EVSEID.len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*CurrentDemandResType).EVSEID.len
                                                    as usize,
                                                &mut (*CurrentDemandResType).EVSEID.data,
                                                38,
                                            )?;
                                        } else {
                                            return Err(STRINGVALUES_NOT_SUPPORTED);
                                        }
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 212 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            211 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*CurrentDemandResType).EVSEID.len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*CurrentDemandResType).EVSEID.len as i32
                                            >= 2 as i32
                                        {
                                            (*CurrentDemandResType).EVSEID.len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*CurrentDemandResType).EVSEID.len,
                                                &mut (*CurrentDemandResType).EVSEID.data,
                                                38,
                                            )?;
                                        } else {
                                            return Err(STRINGVALUES_NOT_SUPPORTED);
                                        }
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 212 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            212 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_3: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        8 as i32 as usize,
                                        &mut value_3,
                                    )?;
                                    if error == 0 as i32 {
                                        (*CurrentDemandResType).SAScheduleTupleID =
                                            value_3.wrapping_add(1 as i32 as u32) as u8;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 213 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            213 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_MeterInfoType(
                                stream,
                                &mut (*CurrentDemandResType).MeterInfo.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 214 as i32;
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_4: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_4,
                                    )?;
                                    if error == 0 as i32 {
                                        (*CurrentDemandResType).ReceiptRequired = Some(value_4 as i32);
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        2 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            214 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_5: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_5,
                                    )?;
                                    if error == 0 as i32 {
                                        (*CurrentDemandResType).ReceiptRequired = Some(value_5 as i32);
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_ChargingStatusResType(
    stream: &mut ExiBitstream,
    mut ChargingStatusResType: &mut Iso2ChargingStatusResType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 215 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            215 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ChargingStatusResType).ResponseCode =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 216 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            216 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*ChargingStatusResType).EVSEID.len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*ChargingStatusResType).EVSEID.len as i32
                                            >= 2 as i32
                                        {
                                            (*ChargingStatusResType).EVSEID.len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*ChargingStatusResType).EVSEID.len,
                                                &mut (*ChargingStatusResType).EVSEID.data,
                                                38,
                                            )?;
                                        } else {
                                            return Err(STRINGVALUES_NOT_SUPPORTED);
                                        }
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 217 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            217 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        8 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ChargingStatusResType).SAScheduleTupleID =
                                            value_0.wrapping_add(1 as i32 as u32) as u8;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 218 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            218 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*ChargingStatusResType).EVSEMaxCurrent.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 219 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_MeterInfoType(
                                stream,
                                &mut (*ChargingStatusResType).MeterInfo.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 220 as i32;
                            }
                        }
                        2 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_1: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_1,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ChargingStatusResType).ReceiptRequired = Some(value_1 as i32);
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 221 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        3 => {
                            decode_iso2_AC_EVSEStatusType(
                                stream,
                                &mut (*ChargingStatusResType).AC_EVSEStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            219 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_MeterInfoType(
                                stream,
                                &mut (*ChargingStatusResType).MeterInfo.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 220 as i32;
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_2: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_2,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ChargingStatusResType).ReceiptRequired = Some(value_2 as i32);
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 221 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        2 => {
                            decode_iso2_AC_EVSEStatusType(
                                stream,
                                &mut (*ChargingStatusResType).AC_EVSEStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            220 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_3: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_3,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ChargingStatusResType).ReceiptRequired = Some(value_3 as i32);
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 221 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        1 => {
                            decode_iso2_AC_EVSEStatusType(
                                stream,
                                &mut (*ChargingStatusResType).AC_EVSEStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            221 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_AC_EVSEStatusType(
                                stream,
                                &mut (*ChargingStatusResType).AC_EVSEStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_AuthorizationReqType(
    stream: &mut ExiBitstream,
    mut AuthorizationReqType: &mut Iso2AuthorizationReqType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 222 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            222 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*AuthorizationReqType).Id.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*AuthorizationReqType).Id.unwrap().len as i32 >= 2 as i32 {
                                    (*AuthorizationReqType).Id.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*AuthorizationReqType).Id.unwrap().len,
                                        &mut (*AuthorizationReqType).Id.unwrap().data,
                                    65,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 223 as i32;
                        }
                        1 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*AuthorizationReqType).GenChallenge.unwrap().len,
                                &mut (*AuthorizationReqType).GenChallenge.unwrap().data,
                                16 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            223 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*AuthorizationReqType).GenChallenge.unwrap().len,
                                &mut (*AuthorizationReqType).GenChallenge.unwrap().data,
                                16 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_PreChargeReqType(
    stream: &mut ExiBitstream,
    mut PreChargeReqType: &mut Iso2PreChargeReqType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 224 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            224 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_DC_EVStatusType(
                                stream,
                                &mut (*PreChargeReqType).DC_EVStatus.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 225 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            225 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*PreChargeReqType).EVTargetVoltage.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 226 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            226 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*PreChargeReqType).EVTargetCurrent.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_ServiceDetailResType(
    stream: &mut ExiBitstream,
    mut ServiceDetailResType: &mut Iso2ServiceDetailResType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 227 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            227 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ServiceDetailResType).ResponseCode =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 228 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            228 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_uint16(
                                stream,
                                &mut (*ServiceDetailResType).ServiceID,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 229 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            229 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_ServiceParameterListType(
                                stream,
                                &mut (*ServiceDetailResType).ServiceParameterList.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_PaymentServiceSelectionResType(
    stream: &mut ExiBitstream,
    mut PaymentServiceSelectionResType: &mut Iso2PaymentServiceSelectionResType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 230 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            230 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*PaymentServiceSelectionResType).ResponseCode =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_CertificateUpdateReqType(
    stream: &mut ExiBitstream,
    mut CertificateUpdateReqType: &mut Iso2CertificateUpdateReqType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 231 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            231 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*CertificateUpdateReqType).Id.len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*CertificateUpdateReqType).Id.len as i32 >= 2 as i32 {
                                    (*CertificateUpdateReqType).Id.len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*CertificateUpdateReqType).Id.len as usize,
                                        &mut (*CertificateUpdateReqType).Id.data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 232 as i32;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            232 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_CertificateChainType(
                                stream,
                                &mut (*CertificateUpdateReqType).ContractSignatureCertChain,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 233 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            233 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*CertificateUpdateReqType).eMaid.len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*CertificateUpdateReqType).eMaid.len as i32
                                            >= 2 as i32
                                        {
                                            (*CertificateUpdateReqType).eMaid.len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*CertificateUpdateReqType).eMaid.len,
                                                &mut (*CertificateUpdateReqType).eMaid.data,
                                                (15 as i32 + 1 as i32) as usize,
                                            )?;
                                        } else {
                                            return Err(STRINGVALUES_NOT_SUPPORTED);
                                        }
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 234 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            234 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_ListOfRootCertificateIDsType(
                                stream,
                                &mut (*CertificateUpdateReqType).ListOfRootCertificateIDs,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_SessionSetupResType(
    stream: &mut ExiBitstream,
    mut SessionSetupResType: &mut Iso2SessionSetupResType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 235 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            235 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*SessionSetupResType).ResponseCode =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 236 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            236 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*SessionSetupResType).EVSEID.len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*SessionSetupResType).EVSEID.len as i32
                                            >= 2 as i32
                                        {
                                            (*SessionSetupResType).EVSEID.len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*SessionSetupResType).EVSEID.len,
                                                &mut (*SessionSetupResType).EVSEID.data,
                                                38,
                                            )?;
                                        } else {
                                            return Err(STRINGVALUES_NOT_SUPPORTED);
                                        }
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 237 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            237 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_integer64(
                                stream,
                                &mut (*SessionSetupResType).EVSETimeStamp.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_CertificateInstallationReqType(
    stream: &mut ExiBitstream,
    mut CertificateInstallationReqType: &mut Iso2CertificateInstallationReqType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 238 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            238 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*CertificateInstallationReqType).Id.len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*CertificateInstallationReqType).Id.len as i32
                                    >= 2 as i32
                                {
                                    (*CertificateInstallationReqType).Id.len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*CertificateInstallationReqType).Id.len as usize,
                                        &mut (*CertificateInstallationReqType).Id.data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 239 as i32;
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            239 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*CertificateInstallationReqType)
                                    .OEMProvisioningCert
                                    .len,
                                &mut (*CertificateInstallationReqType).OEMProvisioningCert.data,
                                800 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 240 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            240 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_ListOfRootCertificateIDsType(
                                stream,
                                &mut (*CertificateInstallationReqType).ListOfRootCertificateIDs,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_CertificateInstallationResType(
    stream: &mut ExiBitstream,
    mut CertificateInstallationResType: &mut Iso2CertificateInstallationResType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 241 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            241 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*CertificateInstallationResType).ResponseCode =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 242 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            242 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_CertificateChainType(
                                stream,
                                &mut (*CertificateInstallationResType)
                                    .SAProvisioningCertificateChain,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 243 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            243 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_CertificateChainType(
                                stream,
                                &mut (*CertificateInstallationResType).ContractSignatureCertChain,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 244 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            244 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_ContractSignatureEncryptedPrivateKeyType(
                                stream,
                                &mut (*CertificateInstallationResType)
                                    .ContractSignatureEncryptedPrivateKey,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 245 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            245 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_DiffieHellmanPublickeyType(
                                stream,
                                &mut (*CertificateInstallationResType).DHpublickey,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 246 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            246 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_EMAIDType(
                                stream,
                                &mut (*CertificateInstallationResType).eMaid,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_WeldingDetectionResType(
    stream: &mut ExiBitstream,
    mut WeldingDetectionResType: &mut Iso2WeldingDetectionResType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 247 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            247 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*WeldingDetectionResType).ResponseCode =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 248 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            248 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_DC_EVSEStatusType(
                                stream,
                                &mut (*WeldingDetectionResType).DC_EVSEStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 249 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            249 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*WeldingDetectionResType).EVSEPresentVoltage,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_CurrentDemandReqType(
    stream: &mut ExiBitstream,
    mut CurrentDemandReqType: &mut Iso2CurrentDemandReqType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 250 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            250 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_DC_EVStatusType(
                                stream,
                                &mut (*CurrentDemandReqType).DC_EVStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 251 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            251 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandReqType).EVTargetCurrent,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 252 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            252 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandReqType).EVMaximumVoltageLimit.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 253 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandReqType).EVMaximumCurrentLimit.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 254 as i32;
                            }
                        }
                        2 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandReqType).EVMaximumPowerLimit.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 255 as i32;
                            }
                        }
                        3 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*CurrentDemandReqType).BulkChargingComplete = Some(value as i32);
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 256 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        4 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*CurrentDemandReqType).ChargingComplete = value_0 as i32;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 257 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            253 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandReqType).EVMaximumCurrentLimit.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 254 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandReqType).EVMaximumPowerLimit.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 255 as i32;
                            }
                        }
                        2 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_1: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_1,
                                    )?;
                                    if error == 0 as i32 {
                                        (*CurrentDemandReqType).BulkChargingComplete =
                                            Some(value_1 as i32);
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 256 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        3 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_2: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_2,
                                    )?;
                                    if error == 0 as i32 {
                                        (*CurrentDemandReqType).ChargingComplete = value_2 as i32;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 257 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            254 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandReqType).EVMaximumPowerLimit.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 255 as i32;
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_3: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_3,
                                    )?;
                                    if error == 0 as i32 {
                                        (*CurrentDemandReqType).BulkChargingComplete =
                                            Some(value_3 as i32);
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 256 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        2 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_4: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_4,
                                    )?;
                                    if error == 0 as i32 {
                                        (*CurrentDemandReqType).ChargingComplete = value_4 as i32;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 257 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            255 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_5: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_5,
                                    )?;
                                    if error == 0 as i32 {
                                        (*CurrentDemandReqType).BulkChargingComplete = Some(value_5 as i32);
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 256 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_6: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_6,
                                    )?;
                                    if error == 0 as i32 {
                                        (*CurrentDemandReqType).ChargingComplete = value_6 as i32;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 257 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            256 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_7: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_7,
                                    )?;
                                    if error == 0 as i32 {
                                        (*CurrentDemandReqType).ChargingComplete = value_7 as i32;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 257 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            257 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandReqType).RemainingTimeToFullSoC.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 258 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandReqType).RemainingTimeToBulkSoC.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 259 as i32;
                            }
                        }
                        2 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandReqType).EVTargetVoltage,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            258 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandReqType).RemainingTimeToBulkSoC.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 259 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandReqType).EVTargetVoltage,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            259 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandReqType).EVTargetVoltage,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_PreChargeResType(
    stream: &mut ExiBitstream,
    mut PreChargeResType: &mut Iso2PreChargeResType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 260 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            260 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*PreChargeResType).ResponseCode =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 261 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            261 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_DC_EVSEStatusType(
                                stream,
                                &mut (*PreChargeResType).DC_EVSEStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 262 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            262 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PhysicalValueType(
                                stream,
                                &mut (*PreChargeResType).EVSEPresentVoltage,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_CertificateUpdateResType(
    stream: &mut ExiBitstream,
    mut CertificateUpdateResType: &mut Iso2CertificateUpdateResType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 263 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            263 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*CertificateUpdateResType).ResponseCode =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 264 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            264 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_CertificateChainType(
                                stream,
                                &mut (*CertificateUpdateResType).SAProvisioningCertificateChain,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 265 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            265 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_CertificateChainType(
                                stream,
                                &mut (*CertificateUpdateResType).ContractSignatureCertChain,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 266 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            266 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_ContractSignatureEncryptedPrivateKeyType(
                                stream,
                                &mut (*CertificateUpdateResType)
                                    .ContractSignatureEncryptedPrivateKey,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 267 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            267 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_DiffieHellmanPublickeyType(
                                stream,
                                &mut (*CertificateUpdateResType).DHpublickey,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 268 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            268 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_EMAIDType(
                                stream,
                                &mut (*CertificateUpdateResType).eMaid,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 269 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            269 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_integer16(
                                stream,
                                &mut (*CertificateUpdateResType).RetryCounter.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_MeteringReceiptReqType(
    stream: &mut ExiBitstream,
    mut MeteringReceiptReqType: &mut Iso2MeteringReceiptReqType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 270 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            270 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*MeteringReceiptReqType).Id.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*MeteringReceiptReqType).Id.unwrap().len as i32 >= 2 as i32 {
                                    (*MeteringReceiptReqType).Id.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*MeteringReceiptReqType).Id.unwrap().len,
                                        &mut (*MeteringReceiptReqType).Id.unwrap().data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(STRINGVALUES_NOT_SUPPORTED);
                                }
                            }
                            grammar_id = 271 as i32;
                        }
                        1 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*MeteringReceiptReqType).SessionID.len,
                                &mut (*MeteringReceiptReqType).SessionID.data,
                                8 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 272 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            271 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*MeteringReceiptReqType).SessionID.len,
                                &mut (*MeteringReceiptReqType).SessionID.data,
                                8 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 272 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            272 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        8 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*MeteringReceiptReqType).SAScheduleTupleID = Some(
                                            value.wrapping_add(1 as i32 as u32) as u8);
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 273 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        1 => {
                            decode_iso2_MeterInfoType(
                                stream,
                                &mut (*MeteringReceiptReqType).MeterInfo,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            273 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_MeterInfoType(
                                stream,
                                &mut (*MeteringReceiptReqType).MeterInfo,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_ChargingStatusReqType(
    stream: &mut ExiBitstream,
    mut _ChargingStatusReqType: &mut Iso2ChargingStatusReqType,
) -> Result<u8, i16> {
    let mut eventCode: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
    if eventCode != 0 {
        return Err(UNKNOWN_EVENT_CODE);
    }

    return Ok(NO_ERROR);
}
pub fn decode_iso2_SessionStopResType(
    stream: &mut ExiBitstream,
    mut SessionStopResType: &mut Iso2SessionStopResType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 274 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            274 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*SessionStopResType).ResponseCode =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_ChargeParameterDiscoveryReqType(
    stream: &mut ExiBitstream,
    mut ChargeParameterDiscoveryReqType: &mut Iso2ChargeParameterDiscoveryReqType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 275 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            275 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_uint16(
                                stream,
                                &mut (*ChargeParameterDiscoveryReqType).MaxEntriesSAScheduleTuple.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 276 as i32;
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        3 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ChargeParameterDiscoveryReqType)
                                            .RequestedEnergyTransferMode =
                                            value as Iso2EnergyTransferModeType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 277 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            276 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        3 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ChargeParameterDiscoveryReqType)
                                            .RequestedEnergyTransferMode =
                                            value_0 as Iso2EnergyTransferModeType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 277 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            277 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_AC_EVChargeParameterType(
                                stream,
                                &mut (*ChargeParameterDiscoveryReqType).AC_EVChargeParameter.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_DC_EVChargeParameterType(
                                stream,
                                &mut (*ChargeParameterDiscoveryReqType).DC_EVChargeParameter.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            decode_iso2_EVChargeParameterType(
                                stream,
                                &mut (*ChargeParameterDiscoveryReqType).EVChargeParameter.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_CableCheckReqType(
    stream: &mut ExiBitstream,
    mut CableCheckReqType: &mut Iso2CableCheckReqType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 278 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            278 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_DC_EVStatusType(
                                stream,
                                &mut (*CableCheckReqType).DC_EVStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_WeldingDetectionReqType(
    stream: &mut ExiBitstream,
    mut WeldingDetectionReqType: &mut Iso2WeldingDetectionReqType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 279 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            279 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_DC_EVStatusType(
                                stream,
                                &mut (*WeldingDetectionReqType).DC_EVStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_PowerDeliveryResType(
    stream: &mut ExiBitstream,
    mut PowerDeliveryResType: &mut Iso2PowerDeliveryResType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 280 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            280 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*PowerDeliveryResType).ResponseCode =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 281 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            281 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_AC_EVSEStatusType(
                                stream,
                                &mut (*PowerDeliveryResType).AC_EVSEStatus.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_DC_EVSEStatusType(
                                stream,
                                &mut (*PowerDeliveryResType).DC_EVSEStatus.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            decode_iso2_EVSEStatusType(
                                stream,
                                &mut (*PowerDeliveryResType).EVSEStatus.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_ChargeParameterDiscoveryResType(
    stream: &mut ExiBitstream,
    mut ChargeParameterDiscoveryResType: &mut Iso2ChargeParameterDiscoveryResType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 282 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            282 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ChargeParameterDiscoveryResType).ResponseCode =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 283 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            283 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ChargeParameterDiscoveryResType).EVSEProcessing =
                                            value_0 as Iso2EvseProcessingType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 284 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            284 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_SAScheduleListType(
                                stream,
                                &mut (*ChargeParameterDiscoveryResType).SAScheduleList.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 285 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_SASchedulesType(
                                stream,
                                &mut (*ChargeParameterDiscoveryResType).SASchedules.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 285 as i32;
                            }
                        }
                        2 => {
                            decode_iso2_AC_EVSEChargeParameterType(
                                stream,
                                &mut (*ChargeParameterDiscoveryResType).AC_EVSEChargeParameter.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        3 => {
                            decode_iso2_DC_EVSEChargeParameterType(
                                stream,
                                &mut (*ChargeParameterDiscoveryResType).DC_EVSEChargeParameter.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        4 => {
                            decode_iso2_EVSEChargeParameterType(
                                stream,
                                &mut (*ChargeParameterDiscoveryResType).EVSEChargeParameter.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            285 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_AC_EVSEChargeParameterType(
                                stream,
                                &mut (*ChargeParameterDiscoveryResType).AC_EVSEChargeParameter.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_DC_EVSEChargeParameterType(
                                stream,
                                &mut (*ChargeParameterDiscoveryResType).DC_EVSEChargeParameter.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            decode_iso2_EVSEChargeParameterType(
                                stream,
                                &mut (*ChargeParameterDiscoveryResType).EVSEChargeParameter.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_PaymentServiceSelectionReqType(
    stream: &mut ExiBitstream,
    mut PaymentServiceSelectionReqType: &mut Iso2PaymentServiceSelectionReqType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 286 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            286 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*PaymentServiceSelectionReqType).SelectedPaymentOption =
                                            value as Iso2PaymentOptionType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 287 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            287 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_SelectedServiceListType(
                                stream,
                                &mut (*PaymentServiceSelectionReqType).SelectedServiceList,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_MeteringReceiptResType(
    stream: &mut ExiBitstream,
    mut MeteringReceiptResType: &mut Iso2MeteringReceiptResType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 288 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            288 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*MeteringReceiptResType).ResponseCode =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 289 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            289 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_AC_EVSEStatusType(
                                stream,
                                &mut (*MeteringReceiptResType).AC_EVSEStatus.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_DC_EVSEStatusType(
                                stream,
                                &mut (*MeteringReceiptResType).DC_EVSEStatus.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            decode_iso2_EVSEStatusType(
                                stream,
                                &mut (*MeteringReceiptResType).EVSEStatus.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_CableCheckResType(
    stream: &mut ExiBitstream,
    mut CableCheckResType: &mut Iso2CableCheckResType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 290 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            290 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*CableCheckResType).ResponseCode =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 291 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            291 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_DC_EVSEStatusType(
                                stream,
                                &mut (*CableCheckResType).DC_EVSEStatus.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 292 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            292 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*CableCheckResType).EVSEProcessing =
                                            value_0 as Iso2EvseProcessingType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_ServiceDiscoveryResType(
    stream: &mut ExiBitstream,
    mut ServiceDiscoveryResType: &mut Iso2ServiceDiscoveryResType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 293 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            293 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ServiceDiscoveryResType).ResponseCode =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 294 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            294 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_PaymentOptionListType(
                                stream,
                                &mut (*ServiceDiscoveryResType).PaymentOptionList,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 295 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            295 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_ChargeServiceType(
                                stream,
                                &mut (*ServiceDiscoveryResType).ChargeService,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 296 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            296 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_ServiceListType(
                                stream,
                                &mut (*ServiceDiscoveryResType).ServiceList.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_ServiceDetailReqType(
    stream: &mut ExiBitstream,
    mut ServiceDetailReqType: &mut Iso2ServiceDetailReqType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 297 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            297 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_uint16(
                                stream,
                                &mut (*ServiceDetailReqType).ServiceID,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_SessionSetupReqType(
    stream: &mut ExiBitstream,
    mut SessionSetupReqType: &mut Iso2SessionSetupReqType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 298 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            298 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*SessionSetupReqType).EVCCID.len,
                                &mut (*SessionSetupReqType).EVCCID.data,
                                6 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_SessionStopReqType(
    stream: &mut ExiBitstream,
    mut SessionStopReqType: &mut Iso2SessionStopReqType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 299 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            299 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*SessionStopReqType).ChargingSession =
                                            value as Iso2ChargingSessionType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_ServiceDiscoveryReqType(
    stream: &mut ExiBitstream,
    mut ServiceDiscoveryReqType: &mut Iso2ServiceDiscoveryReqType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 300 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            300 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*ServiceDiscoveryReqType).ServiceScope.unwrap().len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*ServiceDiscoveryReqType).ServiceScope.unwrap().len
                                            as i32
                                            >= 2 as i32
                                        {
                                            (*ServiceDiscoveryReqType).ServiceScope.unwrap().len =
                                                ((*ServiceDiscoveryReqType)
                                                    .ServiceScope.unwrap().len
                                                    as i32
                                                    - 2 as i32)
                                                    as usize;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*ServiceDiscoveryReqType)
                                                    .ServiceScope.unwrap().len,
                                                &mut ((*ServiceDiscoveryReqType)
                                                    .ServiceScope.unwrap().data),
                                                (64 as i32 + 1 as i32) as usize,
                                            )?;
                                        } else {
                                            return Err(STRINGVALUES_NOT_SUPPORTED);
                                        }
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 301 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ServiceDiscoveryReqType).ServiceCategory = Some(
                                            value as Iso2ServiceCategoryType);
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        2 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            301 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ServiceDiscoveryReqType).ServiceCategory = Some(
                                            value_0 as Iso2ServiceCategoryType);
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        1 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_AuthorizationResType(
    stream: &mut ExiBitstream,
    mut AuthorizationResType: &mut Iso2AuthorizationResType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 302 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            302 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*AuthorizationResType).ResponseCode =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 303 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            303 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*AuthorizationResType).EVSEProcessing =
                                            value_0 as Iso2EvseProcessingType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_PaymentDetailsReqType(
    stream: &mut ExiBitstream,
    mut PaymentDetailsReqType: &mut Iso2PaymentDetailsReqType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 304 as i32;

    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            304 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*PaymentDetailsReqType).eMaid.len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*PaymentDetailsReqType).eMaid.len as i32
                                            >= 2 as i32
                                        {
                                            (*PaymentDetailsReqType).eMaid.len =
                                                ((*PaymentDetailsReqType).eMaid.len
                                                    as i32
                                                    - 2 as i32)
                                                    as usize;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*PaymentDetailsReqType).eMaid.len,
                                                &mut (*PaymentDetailsReqType).eMaid.data,
                                                (15 as i32 + 1 as i32) as usize,
                                            )?;
                                        } else {
                                            return Err(STRINGVALUES_NOT_SUPPORTED);
                                        }
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 305 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            305 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_CertificateChainType(
                                stream,
                                &mut (*PaymentDetailsReqType).ContractSignatureCertChain,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}
pub fn decode_iso2_PaymentDetailsResType(
    stream: &mut ExiBitstream,
    mut PaymentDetailsResType: &mut Iso2PaymentDetailsResType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 306 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            306 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            )?;
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*PaymentDetailsResType).ResponseCode =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(UNSUPPORTED_SUB_EVENT);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                )?;
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 307 as i32;
                                    } else {
                                        return Err(DEVIANTS_NOT_SUPPORTED);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            307 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*PaymentDetailsResType).GenChallenge.len,
                                &mut (*PaymentDetailsResType).GenChallenge.data,
                                16 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 308 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            308 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_exi_type_integer64(
                                stream,
                                &mut (*PaymentDetailsResType).EVSETimeStamp,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}


pub fn decode_iso2_BodyType(
    stream: &mut ExiBitstream,
    mut BodyType: &mut Iso2BodyType,
) -> Result<u8, i16> {
    let mut eventCode: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 6 as i32 as usize, &mut eventCode)?;
    match eventCode {
        0 => {
            let mut authorization_req: Iso2AuthorizationReqType = Default::default();
            decode_iso2_AuthorizationReqType(stream, &mut authorization_req)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::AuthorizationReq(authorization_req);
        }
        1 => {
            let mut authorization_res: Iso2AuthorizationResType = Default::default();
            decode_iso2_AuthorizationResType(stream, &mut authorization_res)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::AuthorizationRes(authorization_res);
        }
        2 => {
            let mut body_element: Iso2BodyBaseType = Default::default();
            decode_iso2_BodyBaseType(stream, &mut body_element)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::BodyElement(body_element);
        }
        3 => {
            let mut cable_check_req: Iso2CableCheckReqType = Default::default();
            decode_iso2_CableCheckReqType(stream, &mut cable_check_req)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::CableCheckReq(cable_check_req);
        }
        4 => {
            let mut cable_check_res: Iso2CableCheckResType = Default::default();
            decode_iso2_CableCheckResType(stream, &mut cable_check_res)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::CableCheckRes(cable_check_res);
        }
        5 => {
            let mut certificate_installation_req: Iso2CertificateInstallationReqType = Default::default();
            decode_iso2_CertificateInstallationReqType(stream, &mut certificate_installation_req)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::CertificateInstallationReq(certificate_installation_req);
        }
        6 => {
            let mut certificate_installation_res: Iso2CertificateInstallationResType = Default::default();
            decode_iso2_CertificateInstallationResType(stream, &mut certificate_installation_res)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::CertificateInstallationRes(certificate_installation_res);
        }
        7 => {
            let mut certificate_update_req: Iso2CertificateUpdateReqType = Default::default();
            decode_iso2_CertificateUpdateReqType(stream, &mut certificate_update_req)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::CertificateUpdateReq(certificate_update_req);
        }
        8 => {
            let mut certificate_update_res: Iso2CertificateUpdateResType = Default::default();
            decode_iso2_CertificateUpdateResType(stream, &mut certificate_update_res)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::CertificateUpdateRes(certificate_update_res);
        }
        9 => {
            let mut charge_parameter_discovery_req: Iso2ChargeParameterDiscoveryReqType = Default::default();
            decode_iso2_ChargeParameterDiscoveryReqType(stream, &mut charge_parameter_discovery_req)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::ChargeParameterDiscoveryReq(charge_parameter_discovery_req);
        }
        10 => {
            let mut charge_parameter_discovery_res: Iso2ChargeParameterDiscoveryResType = Default::default();
            decode_iso2_ChargeParameterDiscoveryResType(stream, &mut charge_parameter_discovery_res)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::ChargeParameterDiscoveryRes(charge_parameter_discovery_res);
        }
        11 => {
            let mut charging_status_req: Iso2ChargingStatusReqType = Default::default();
            decode_iso2_ChargingStatusReqType(stream, &mut charging_status_req)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::ChargingStatusReq(charging_status_req);
        }
        12 => {
            let mut charging_status_res: Iso2ChargingStatusResType = Default::default();
            decode_iso2_ChargingStatusResType(stream, &mut charging_status_res)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::ChargingStatusRes(charging_status_res);
        }
        13 => {
            let mut current_demand_req: Iso2CurrentDemandReqType = Default::default();
            decode_iso2_CurrentDemandReqType(stream, &mut current_demand_req)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::CurrentDemandReq(current_demand_req);
        }
        14 => {
            let mut current_demand_res: Iso2CurrentDemandResType = Default::default();
            decode_iso2_CurrentDemandResType(stream, &mut current_demand_res)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::CurrentDemandRes(current_demand_res);
        }
        15 => {
            let mut metering_receipt_req: Iso2MeteringReceiptReqType = Default::default();
            decode_iso2_MeteringReceiptReqType(stream, &mut metering_receipt_req)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::MeteringReceiptReq(metering_receipt_req);
        }
        16 => {
            let mut metering_receipt_res: Iso2MeteringReceiptResType = Default::default();
            decode_iso2_MeteringReceiptResType(stream, &mut metering_receipt_res)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::MeteringReceiptRes(metering_receipt_res);
        }
        17 => {
            let mut payment_details_req: Iso2PaymentDetailsReqType = Default::default();
            decode_iso2_PaymentDetailsReqType(stream, &mut payment_details_req)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::PaymentDetailsReq(payment_details_req);
        }
        18 => {
            let mut payment_details_res: Iso2PaymentDetailsResType = Default::default();
            decode_iso2_PaymentDetailsResType(stream, &mut payment_details_res)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::PaymentDetailsRes(payment_details_res);
        }
        19 => {
            let mut payment_service_selection_req: Iso2PaymentServiceSelectionReqType = Default::default();
            decode_iso2_PaymentServiceSelectionReqType(stream, &mut payment_service_selection_req)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::PaymentServiceSelectionReq(payment_service_selection_req);
        }
        20 => {
            let mut payment_service_selection_res: Iso2PaymentServiceSelectionResType = Default::default();
            decode_iso2_PaymentServiceSelectionResType(stream, &mut payment_service_selection_res)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::PaymentServiceSelectionRes(payment_service_selection_res);
        }
        21 => {
            let mut power_delivery_req: Iso2PowerDeliveryReqType = Default::default();
            decode_iso2_PowerDeliveryReqType(stream, &mut power_delivery_req)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::PowerDeliveryReq(power_delivery_req);
        }
        22 => {
            let mut power_delivery_res: Iso2PowerDeliveryResType = Default::default();
            decode_iso2_PowerDeliveryResType(stream, &mut power_delivery_res)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::PowerDeliveryRes(power_delivery_res);
        }
        23 => {
            let mut pre_charge_req: Iso2PreChargeReqType = Default::default();
            decode_iso2_PreChargeReqType(stream, &mut pre_charge_req)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::PreChargeReq(pre_charge_req);
        }
        24 => {
            let mut pre_charge_res: Iso2PreChargeResType = Default::default();
            decode_iso2_PreChargeResType(stream, &mut pre_charge_res)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::PreChargeRes(pre_charge_res);
        }
        25 => {
            let mut service_detail_req: Iso2ServiceDetailReqType = Default::default();
            decode_iso2_ServiceDetailReqType(stream, &mut service_detail_req)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::ServiceDetailReq(service_detail_req);
        }
        26 => {
            let mut service_detail_res: Iso2ServiceDetailResType = Default::default();
            decode_iso2_ServiceDetailResType(stream, &mut service_detail_res)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::ServiceDetailRes(service_detail_res);
        }
        27 => {
            let mut service_discovery_req: Iso2ServiceDiscoveryReqType = Default::default();
            decode_iso2_ServiceDiscoveryReqType(stream, &mut service_discovery_req)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::ServiceDiscoveryReq(service_discovery_req);
        }
        28 => {
            let mut service_discovery_res: Iso2ServiceDiscoveryResType = Default::default();
            decode_iso2_ServiceDiscoveryResType(stream, &mut service_discovery_res)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::ServiceDiscoveryRes(service_discovery_res);
        }
        29 => {
            let mut session_setup_req: Iso2SessionSetupReqType = Default::default();
            decode_iso2_SessionSetupReqType(stream, &mut session_setup_req)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::SessionSetupReq(session_setup_req);
        }
        30 => {
            let mut session_setup_res: Iso2SessionSetupResType = Default::default();
            decode_iso2_SessionSetupResType(stream, &mut session_setup_res)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::SessionSetupRes(session_setup_res);
        }
        31 => {
            let mut session_stop_req: Iso2SessionStopReqType = Default::default();
            decode_iso2_SessionStopReqType(stream, &mut session_stop_req)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::SessionStopReq(session_stop_req);
        }
        32 => {
            let mut session_stop_res: Iso2SessionStopResType = Default::default();
            decode_iso2_SessionStopResType(stream, &mut session_stop_res)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::SessionStopRes(session_stop_res);
        }
        33 => {
            let mut welding_detection_req: Iso2WeldingDetectionReqType = Default::default();
            decode_iso2_WeldingDetectionReqType(stream, &mut welding_detection_req)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::WeldingDetectionReq(welding_detection_req);
        }
        34 => {
            let mut welding_detection_res: Iso2WeldingDetectionResType = Default::default();
            decode_iso2_WeldingDetectionResType(stream, &mut welding_detection_res)?;
            BodyType.BodyTypeComponent = Iso2BodyTypeEnum::WeldingDetectionRes(welding_detection_res);
        }
        _ => {
            return Err(UNKNOWN_EVENT_CODE);
        }
    }
    let mut eventCode2: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode2)?;
    match eventCode2 {
        0 => Ok(NO_ERROR),
        _ => Err(UNKNOWN_EVENT_CODE),
    }
}
pub fn decode_iso2_V2G_Message(
    stream: &mut ExiBitstream,
    mut V2G_Message: &mut Iso2v2gMessage,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 310 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            310 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_MessageHeaderType(stream, &mut (*V2G_Message).Header)?;
                            if error == 0 as i32 {
                                grammar_id = 311 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            311 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            decode_iso2_BodyType(stream, &mut (*V2G_Message).Body)?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            return Ok(NO_ERROR);
                        }
                        _ => {
                            return Err(UNKNOWN_EVENT_CODE);
                        }
                    }
                }
            }
            _ => {
                return Err(UNKNOWN_GRAMMAR_ID);
            }
        }
    }
}

pub fn decode_iso2_exiDocument(
    stream: &mut ExiBitstream,
    mut exiDoc: &mut Iso2ExiDocument,
) -> Result<u8, i16> {
    let mut eventCode: u32 = 0;
    exi_header_read_and_check(stream)?;
    exi_basetypes_decoder_nbit_uint(stream, 7 as i32 as usize, &mut eventCode)?;
    match eventCode {
        0 | 76 => {
            decode_iso2_V2G_Message(stream, &mut (*exiDoc).V2G_Message)?;
            return Ok(NO_ERROR);
        }
        _ => {
            return Err(UNSUPPORTED_SUB_EVENT);
        }
    }
}

pub fn decode_iso2_exiFragment(
    stream: &mut ExiBitstream,
    mut _exiFrag: &mut Iso2ExiFragment,
) -> Result<u8, i16> {
    let mut eventCode: u32 = 0;
    exi_header_read_and_check(stream)?;
    exi_basetypes_decoder_nbit_uint(stream, 8 as i32 as usize, &mut eventCode)?;
    return Err(NOT_IMPLEMENTED_YET);
    // match eventCode {
    //     0 => {}
    //     1 => {}
    //     2 => {}
    //     3 => {}
    //     4 => {
    //         decode_iso2_AuthorizationReqType(
    //             stream,
    //             &mut (*exiFrag).c2rust_unnamed.AuthorizationReq,
    //         )?;
    //         (*exiFrag).set_AuthorizationReq_isUsed(1 as u32);
    //     }
    //     5 => {}
    //     6 => {}
    //     7 => {}
    //     8 => {}
    //     9 => {}
    //     10 => {}
    //     11 => {}
    //     12 => {}
    //     13 => {}
    //     14 => {}
    //     15 => {
    //         decode_iso2_CertificateInstallationReqType(
    //             stream,
    //             &mut (*exiFrag).c2rust_unnamed.CertificateInstallationReq,
    //         )?;
    //         (*exiFrag).set_CertificateInstallationReq_isUsed(1 as u32);
    //     }
    //     16 => {}
    //     17 => {
    //         decode_iso2_CertificateUpdateReqType(
    //             stream,
    //             &mut (*exiFrag).c2rust_unnamed.CertificateUpdateReq,
    //         )?;
    //         (*exiFrag).set_CertificateUpdateReq_isUsed(1 as u32);
    //     }
    //     18 => {}
    //     19 => {}
    //     20 => {}
    //     21 => {}
    //     22 => {}
    //     23 => {}
    //     24 => {}
    //     25 => {}
    //     26 => {}
    //     27 => {}
    //     28 => {}
    //     29 => {}
    //     30 => {}
    //     31 => {}
    //     32 => {}
    //     33 => {
    //         decode_iso2_CertificateChainType(
    //             stream,
    //             &mut (*exiFrag).c2rust_unnamed.ContractSignatureCertChain,
    //         )?;
    //         (*exiFrag).set_ContractSignatureCertChain_isUsed(1 as u32);
    //     }
    //     34 => {
    //         decode_iso2_ContractSignatureEncryptedPrivateKeyType(
    //             stream,
    //             &mut (*exiFrag)
    //                 .c2rust_unnamed
    //                 .ContractSignatureEncryptedPrivateKey,
    //         )?;
    //         (*exiFrag).set_ContractSignatureEncryptedPrivateKey_isUsed(1 as u32);
    //     }
    //     35 => {}
    //     36 => {}
    //     37 => {}
    //     38 => {}
    //     39 => {}
    //     40 => {}
    //     41 => {}
    //     42 => {}
    //     43 => {}
    //     44 => {}
    //     45 => {
    //         decode_iso2_DiffieHellmanPublickeyType(
    //             stream,
    //             &mut (*exiFrag).c2rust_unnamed.DHpublickey,
    //         )?;
    //         (*exiFrag).set_DHpublickey_isUsed(1 as u32);
    //     }
    //     46 => {}
    //     47 => {}
    //     48 => {}
    //     49 => {}
    //     50 => {}
    //     51 => {}
    //     52 => {}
    //     53 => {}
    //     54 => {}
    //     55 => {}
    //     56 => {}
    //     57 => {}
    //     58 => {}
    //     59 => {}
    //     60 => {}
    //     61 => {}
    //     62 => {}
    //     63 => {}
    //     64 => {}
    //     65 => {}
    //     66 => {}
    //     67 => {}
    //     68 => {}
    //     69 => {}
    //     70 => {}
    //     71 => {}
    //     72 => {}
    //     73 => {}
    //     74 => {}
    //     75 => {}
    //     76 => {}
    //     77 => {}
    //     78 => {}
    //     79 => {}
    //     80 => {}
    //     81 => {}
    //     82 => {}
    //     83 => {}
    //     84 => {}
    //     85 => {}
    //     86 => {}
    //     87 => {}
    //     88 => {}
    //     89 => {}
    //     90 => {}
    //     91 => {}
    //     92 => {}
    //     93 => {}
    //     94 => {}
    //     95 => {}
    //     96 => {}
    //     97 => {}
    //     98 => {}
    //     99 => {}
    //     100 => {}
    //     101 => {}
    //     102 => {}
    //     103 => {}
    //     104 => {}
    //     105 => {}
    //     106 => {}
    //     107 => {}
    //     108 => {}
    //     109 => {}
    //     110 => {}
    //     111 => {}
    //     112 => {}
    //     113 => {}
    //     114 => {}
    //     115 => {}
    //     116 => {}
    //     117 => {}
    //     118 => {}
    //     119 => {}
    //     120 => {}
    //     121 => {
    //         decode_iso2_MeteringReceiptReqType(
    //             stream,
    //             &mut (*exiFrag).c2rust_unnamed.MeteringReceiptReq,
    //         )?;
    //         (*exiFrag).set_MeteringReceiptReq_isUsed(1 as u32);
    //     }
    //     122 => {}
    //     123 => {}
    //     124 => {}
    //     125 => {}
    //     126 => {}
    //     127 => {}
    //     128 => {}
    //     129 => {}
    //     130 => {}
    //     131 => {}
    //     132 => {}
    //     133 => {}
    //     134 => {}
    //     135 => {}
    //     136 => {}
    //     137 => {}
    //     138 => {}
    //     139 => {}
    //     140 => {}
    //     141 => {}
    //     142 => {}
    //     143 => {}
    //     144 => {}
    //     145 => {}
    //     146 => {}
    //     147 => {}
    //     148 => {}
    //     149 => {}
    //     150 => {}
    //     151 => {}
    //     152 => {}
    //     153 => {}
    //     154 => {}
    //     155 => {}
    //     156 => {}
    //     157 => {}
    //     158 => {}
    //     159 => {}
    //     160 => {}
    //     161 => {}
    //     162 => {}
    //     163 => {}
    //     164 => {}
    //     165 => {}
    //     166 => {}
    //     167 => {}
    //     168 => {}
    //     169 => {}
    //     170 => {}
    //     171 => {}
    //     172 => {}
    //     173 => {}
    //     174 => {
    //         decode_iso2_SalesTariffType(
    //             stream,
    //             &mut (*exiFrag).c2rust_unnamed.SalesTariff,
    //         )?;
    //         (*exiFrag).set_SalesTariff_isUsed(1 as u32);
    //     }
    //     175 => {}
    //     176 => {}
    //     177 => {}
    //     178 => {}
    //     179 => {}
    //     180 => {}
    //     181 => {}
    //     182 => {}
    //     183 => {}
    //     184 => {}
    //     185 => {}
    //     186 => {}
    //     187 => {}
    //     188 => {}
    //     189 => {}
    //     190 => {}
    //     191 => {}
    //     192 => {}
    //     193 => {}
    //     194 => {}
    //     195 => {}
    //     196 => {}
    //     197 => {}
    //     198 => {}
    //     199 => {}
    //     200 => {}
    //     201 => {}
    //     202 => {}
    //     203 => {}
    //     204 => {}
    //     205 => {}
    //     206 => {}
    //     207 => {}
    //     208 => {
    //         decode_iso2_SignedInfoType(
    //             stream,
    //             &mut (*exiFrag).c2rust_unnamed.SignedInfo,
    //         )?;
    //         (*exiFrag).set_SignedInfo_isUsed(1 as u32);
    //     }
    //     209 => {}
    //     210 => {}
    //     211 => {}
    //     212 => {}
    //     213 => {}
    //     214 => {}
    //     215 => {}
    //     216 => {}
    //     217 => {}
    //     218 => {}
    //     219 => {}
    //     220 => {}
    //     221 => {}
    //     222 => {}
    //     223 => {}
    //     224 => {}
    //     225 => {}
    //     226 => {}
    //     227 => {}
    //     228 => {}
    //     229 => {}
    //     230 => {}
    //     231 => {}
    //     232 => {}
    //     233 => {}
    //     234 => {}
    //     235 => {}
    //     236 => {
    //         decode_iso2_EMAIDType(stream, &mut (*exiFrag).c2rust_unnamed.eMaid);
    //         (*exiFrag).set_eMAID_isUsed(1 as u32);
    //     }
    //     237 => {}
    //     238 => {}
    //     239 => {}
    //     240 => {}
    //     241 => {}
    //     242 => {}
    //     _ => {
    //         return Err(UNSUPPORTED_SUB_EVENT);
    //     }
    // }
    // if error == 0 as i32 {
    //     exi_basetypes_decoder_nbit_uint(stream, 8 as i32 as usize, &mut eventCode)?;
    //     if error == 0 as i32 {
    //         if eventCode != 244 as i32 as u32 {
    //             return Err(INCORRECT_END_FRAGMENT_VALUE);
    //         }
    //     }
    // }
}

pub fn decode_iso2_xmldsigFragment(
    stream: &mut ExiBitstream,
    mut _xmldsigFrag: &mut Iso2XmlDSigFragment,
) -> Result<u8, i16> {
    let mut eventCode: u32 = 0;
    exi_header_read_and_check(stream)?;
    exi_basetypes_decoder_nbit_uint(stream, 6 as i32 as usize, &mut eventCode)?;
    return Err(NOT_IMPLEMENTED_YET);
    // match eventCode {
    //     0 => {
    //         decode_iso2_CanonicalizationMethodType(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.CanonicalizationMethod,
    //         )?;
    //         (*xmldsigFrag).set_CanonicalizationMethod_isUsed(1 as u32);
    //     }
    //     1 => {
    //         decode_iso2_DSAKeyValueType(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.DSAKeyValue,
    //         )?;
    //         (*xmldsigFrag).set_DSAKeyValue_isUsed(1 as u32);
    //     }
    //     2 => {
    //         decode_iso2_DigestMethodType(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.DigestMethod,
    //         )?;
    //         (*xmldsigFrag).set_DigestMethod_isUsed(1 as u32);
    //     }
    //     3 => {}
    //     4 => {}
    //     5 => {}
    //     6 => {}
    //     7 => {}
    //     8 => {
    //         error =
    //             decode_iso2_KeyInfoType(stream, &mut (*xmldsigFrag).XmlDSigComponents.KeyInfo);
    //         (*xmldsigFrag).set_KeyInfo_isUsed(1 as u32);
    //     }
    //     9 => {}
    //     10 => {
    //         decode_iso2_KeyValueType(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.KeyValue,
    //         )?;
    //         (*xmldsigFrag).set_KeyValue_isUsed(1 as u32);
    //     }
    //     11 => {}
    //     12 => {}
    //     13 => {}
    //     14 => {
    //         error =
    //             decode_iso2_ObjectType(stream, &mut (*xmldsigFrag).XmlDSigComponents.Object);
    //         (*xmldsigFrag).set_Object_isUsed(1 as u32);
    //     }
    //     15 => {}
    //     16 => {
    //         error =
    //             decode_iso2_PGPDataType(stream, &mut (*xmldsigFrag).XmlDSigComponents.PGPData);
    //         (*xmldsigFrag).set_PGPData_isUsed(1 as u32);
    //     }
    //     17 => {}
    //     18 => {}
    //     19 => {}
    //     20 => {}
    //     21 => {
    //         decode_iso2_RSAKeyValueType(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.RSAKeyValue,
    //         )?;
    //         (*xmldsigFrag).set_RSAKeyValue_isUsed(1 as u32);
    //     }
    //     22 => {
    //         decode_iso2_ReferenceType(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.Reference,
    //         )?;
    //         (*xmldsigFrag).set_Reference_isUsed(1 as u32);
    //     }
    //     23 => {
    //         decode_iso2_RetrievalMethodType(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.RetrievalMethod,
    //         )?;
    //         (*xmldsigFrag).set_RetrievalMethod_isUsed(1 as u32);
    //     }
    //     24 => {
    //         decode_iso2_SPKIDataType(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.SPKIData,
    //         )?;
    //         (*xmldsigFrag).set_SPKIData_isUsed(1 as u32);
    //     }
    //     25 => {}
    //     26 => {}
    //     27 => {
    //         decode_iso2_SignatureType(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.Signature,
    //         )?;
    //         (*xmldsigFrag).set_Signature_isUsed(1 as u32);
    //     }
    //     28 => {
    //         decode_iso2_SignatureMethodType(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.SignatureMethod,
    //         )?;
    //         (*xmldsigFrag).set_SignatureMethod_isUsed(1 as u32);
    //     }
    //     29 => {}
    //     30 => {}
    //     31 => {
    //         decode_iso2_SignatureValueType(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.SignatureValue,
    //         )?;
    //         (*xmldsigFrag).set_SignatureValue_isUsed(1 as u32);
    //     }
    //     32 => {
    //         decode_iso2_SignedInfoType(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.SignedInfo,
    //         )?;
    //         (*xmldsigFrag).set_SignedInfo_isUsed(1 as u32);
    //     }
    //     33 => {
    //         decode_iso2_TransformType(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.Transform,
    //         )?;
    //         (*xmldsigFrag).set_Transform_isUsed(1 as u32);
    //     }
    //     34 => {
    //         decode_iso2_TransformsType(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.Transforms,
    //         )?;
    //         (*xmldsigFrag).set_Transforms_isUsed(1 as u32);
    //     }
    //     35 => {}
    //     36 => {}
    //     37 => {
    //         decode_iso2_X509DataType(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.X509Data,
    //         )?;
    //         (*xmldsigFrag).set_X509Data_isUsed(1 as u32);
    //     }
    //     38 => {}
    //     39 => {
    //         decode_iso2_X509IssuerSerialType(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.X509IssuerSerial,
    //         )?;
    //         (*xmldsigFrag).set_X509IssuerSerial_isUsed(1 as u32);
    //     }
    //     40 => {}
    //     41 => {}
    //     42 => {}
    //     43 => {}
    //     44 => {}
    //     _ => {
    //         return Err(UNSUPPORTED_SUB_EVENT);
    //     }
    // }
    // if error == 0 as i32 {
    //     exi_basetypes_decoder_nbit_uint(stream, 6 as i32 as usize, &mut eventCode)?;
    //     if error == 0 as i32 {
    //         if eventCode != 46 as i32 as u32 {
    //             return Err(INCORRECT_END_FRAGMENT_VALUE);
    //         }
    //     }
    // }
}
