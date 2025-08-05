use crate::common::exi_basetypes_decoder::*;
use crate::common::exi_basetypes_encoder::*;
use crate::common::exi_bitstream::*;
use crate::common::exi_error_codes::ExiError;
use crate::iso_2::iso2_datatypes::*;

pub fn decode_iso2_cost(
    stream: &mut ExiBitstream,
    mut cost_type: &mut Iso2CostType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 0 as i32;
    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            0 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*cost_type).cost_kind = value as Iso2CostKindType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent as i16);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 1 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode as i16);
                        }
                    }
                }
            }
            1 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_uint32(stream, &mut (*cost_type).amount)?;
                            if error == 0 as i32 {
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode as i16);
                        }
                    }
                }
            }
            2 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        3 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*cost_type).amount_multiplier =
                                            Some(value_0.wrapping_sub(3) as i8);
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent as i16);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported as i16);
                                    }
                                }
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}

pub fn decode_iso2_transform(
    stream: &mut ExiBitstream,
    mut transform_type: &mut Iso2TransformType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 5 as i32;
    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            5 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*transform_type).algorithm.len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*transform_type).algorithm.len as i32 >= 2 as i32 {
                                    (*transform_type).algorithm.len =
                                        ((*transform_type).algorithm.len as i32 - 2 as i32)
                                            as usize;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*transform_type).algorithm.len as usize,
                                        &mut ((*transform_type).algorithm.data),
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(ExiError::StringvaluesNotSupported as i16);
                                }
                            }
                            grammar_id = 6 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode as i16);
                        }
                    }
                }
            }
            6 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*transform_type).xpath.unwrap().len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*transform_type).xpath.unwrap().len as i32 >= 2 as i32 {
                                            (*transform_type).xpath.unwrap().len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*transform_type).xpath.unwrap().len,
                                                &mut (*transform_type).xpath.unwrap().data,
                                                (64 as i32 + 1 as i32) as usize,
                                            )?;
                                        } else {
                                            return Err(ExiError::StringvaluesNotSupported as i16);
                                        }
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent as i16);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported as i16);
                                    }
                                }
                            }
                        }
                        1 => {
                            return Err(ExiError::UnknownEventForDecoding as i16);
                        }
                        2 => {
                            return Ok(());
                        }
                        3 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*transform_type).any.unwrap().len,
                                &mut (*transform_type).any.unwrap().data,
                                4 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode as i16);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode as i16);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId as i16);
            }
        }
    }
}
pub fn decode_iso2_interval(
    stream: &mut ExiBitstream,
    mut _interval_type: &mut Iso2IntervalType,
) -> Result<(), ExiError> {
    let mut event_code: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code != 0 as i32 as u32 {
        return Err(ExiError::UnknownEventCode as i16);
    }

    return Ok(());
}
pub fn decode_iso2_transforms(
    stream: &mut ExiBitstream,
    mut transforms_type: &mut Iso2TransformsType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 7 as i32;
    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            7 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_transform(stream, &mut (*transforms_type).transform)?;
                            grammar_id = 8;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode as i16);
                        }
                    }
                }
            }
            8 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Err(ExiError::ArrayOutOfBounds);
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_dsa_key_value(
    stream: &mut ExiBitstream,
    mut dsa_key_value: &mut Iso2DSAKeyValueType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 9 as i32;
    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            9 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*dsa_key_value).p.unwrap().len,
                                &mut (*dsa_key_value).p.unwrap().data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 10 as i32;
                            }
                        }
                        1 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*dsa_key_value).g.unwrap().len,
                                &mut (*dsa_key_value).g.unwrap().data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 12 as i32;
                            }
                        }
                        2 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*dsa_key_value).y.unwrap().len,
                                &mut (*dsa_key_value).y.unwrap().data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 13 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            10 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*dsa_key_value).q.unwrap().len,
                                &mut (*dsa_key_value).q.unwrap().data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 11 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            11 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*dsa_key_value).g.unwrap().len,
                                &mut (*dsa_key_value).g.unwrap().data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 12 as i32;
                            }
                        }
                        1 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*dsa_key_value).y.unwrap().len,
                                &mut (*dsa_key_value).y.unwrap().data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 13 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            12 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*dsa_key_value).y.unwrap().len,
                                &mut (*dsa_key_value).y.unwrap().data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 13 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            13 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*dsa_key_value).j.unwrap().len,
                                &mut (*dsa_key_value).j.unwrap().data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 14 as i32;
                            }
                        }
                        1 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*dsa_key_value).seed.unwrap().len,
                                &mut (*dsa_key_value).seed.unwrap().data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 15 as i32;
                            }
                        }
                        2 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            14 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*dsa_key_value).seed.unwrap().len,
                                &mut (*dsa_key_value).seed.unwrap().data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 15 as i32;
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            15 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*dsa_key_value).pgen_counter.unwrap().len,
                                &mut (*dsa_key_value).pgen_counter.unwrap().data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_x509_issuer_serial(
    stream: &mut ExiBitstream,
    mut x509_issuer_serial: &mut Iso2X509IssuerSerialType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 16 as i32;
    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            16 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*x509_issuer_serial).x509_issuer_name.len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*x509_issuer_serial).x509_issuer_name.len as i32
                                            >= 2 as i32
                                        {
                                            (*x509_issuer_serial).x509_issuer_name.len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*x509_issuer_serial).x509_issuer_name.len as usize,
                                                &mut (*x509_issuer_serial).x509_issuer_name.data,
                                                (64 as i32 + 1 as i32) as usize,
                                            )?;
                                        } else {
                                            return Err(ExiError::StringValuesNotSupported);
                                        }
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 17 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            17 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                exi_basetypes_decoder_signed(
                                    stream,
                                    &mut (*x509_issuer_serial).x509_serial_number,
                                )?;
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_relative_time_interval(
    stream: &mut ExiBitstream,
    mut relative_time_interval: &mut Iso2RelativeTimeIntervalType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 18 as i32;
    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            18 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_uint32(stream, &mut (*relative_time_interval).start)?;
                            if error == 0 as i32 {
                                grammar_id = 19 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            19 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
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
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_digest_method(
    stream: &mut ExiBitstream,
    mut digest_method: &mut Iso2DigestMethodType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 20 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            20 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*digest_method).algorithm.len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*digest_method).algorithm.len as i32 >= 2 as i32 {
                                    (*digest_method).algorithm.len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*digest_method).algorithm.len as usize,
                                        &mut (*digest_method).algorithm.data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 21 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            21 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Err(ExiError::UnknownEventForDecoding);
                        }
                        1 => {
                            return Ok(());
                        }
                        2 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*digest_method).any.unwrap().len,
                                &mut (*digest_method).any.unwrap().data,
                                4 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_rsakey_value(
    stream: &mut ExiBitstream,
    mut rsa_key_value: &mut Iso2RSAKeyValueType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 22 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            22 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut rsa_key_value.modulus.len,
                                &mut rsa_key_value.modulus.data,
                                350 as i32 as usize,
                            )?;
                            grammar_id = 23 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            23 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*rsa_key_value).exponent.len,
                                &mut (*rsa_key_value).exponent.data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_canonicalization_method(
    stream: &mut ExiBitstream,
    mut canonicalization_method: &mut Iso2CanonicalizationMethodType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 24 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            24 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*canonicalization_method).algorithm.len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*canonicalization_method).algorithm.len as i32 >= 2 as i32 {
                                    (*canonicalization_method).algorithm.len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*canonicalization_method).algorithm.len as usize,
                                        &mut (*canonicalization_method).algorithm.data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 25 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            25 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Err(ExiError::UnknownEventForDecoding);
                        }
                        1 => {
                            return Ok(());
                        }
                        2 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*canonicalization_method).any.unwrap().len,
                                &mut (*canonicalization_method).any.unwrap().data,
                                4 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_signature_method(
    stream: &mut ExiBitstream,
    mut signature_method: &mut Iso2SignatureMethodType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 26 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            26 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*signature_method).algorithm.len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*signature_method).algorithm.len as i32 >= 2 as i32 {
                                    (*signature_method).algorithm.len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*signature_method).algorithm.len as usize,
                                        &mut (*signature_method).algorithm.data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 27 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            27 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                exi_basetypes_decoder_signed(
                                    stream,
                                    &mut (*signature_method).hmac_output_length.unwrap(),
                                )?;
                                if error == 0 as i32 {
                                    grammar_id = 28 as i32;
                                }
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                            }
                        }
                        1 => {
                            return Err(ExiError::UnknownEventForDecoding);
                        }
                        2 => {
                            return Ok(());
                        }
                        3 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*signature_method).any.unwrap().len,
                                &mut (*signature_method).any.unwrap().data,
                                4 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            28 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Err(ExiError::UnknownEventForDecoding);
                        }
                        1 => {
                            return Ok(());
                        }
                        2 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*signature_method).any.unwrap().len,
                                &mut (*signature_method).any.unwrap().data,
                                4 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_key_value(
    stream: &mut ExiBitstream,
    mut key_value: &mut Iso2KeyValueType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 29 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            29 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_dsa_key_value(
                                stream,
                                &mut (*key_value).dsa_key_value.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_rsakey_value(
                                stream,
                                &mut (*key_value).rsa_key_value.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*key_value).any.unwrap().len,
                                &mut (*key_value).any.unwrap().data,
                                4 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_physical_value(
    stream: &mut ExiBitstream,
    mut physical_value: &mut Iso2PhysicalValueType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 30 as i32;
    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            30 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        3 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*physical_value).multiplier =
                                            value.wrapping_add(-(3 as i32) as u32) as i8;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 31 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            31 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        3 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*physical_value).unit = value_0 as Iso2UnitSymbolType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 32 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            32 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_integer16(stream, &mut (*physical_value).value)?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_consumption_cost(
    stream: &mut ExiBitstream,
    mut consumption_cost_type: &mut Iso2ConsumptionCostType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 33 as i32;
    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            33 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_physical_value(
                                stream,
                                &mut (*consumption_cost_type).start_value,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 34 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            34 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*consumption_cost_type).cost.len() as i32) < 3 as i32 {
                                let fresh0 = (*consumption_cost_type).cost.len();
                                (*consumption_cost_type).cost.len() =
                                    ((*consumption_cost_type).cost.len()).wrapping_add(1);
                                if let Some(cost) =
                                    (*consumption_cost_type).cost.get_mut(fresh0 as usize)
                                {
                                    decode_iso2_cost(stream, cost)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            grammar_id = 35 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            35 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*ConsumptionCostType).cost.len() as i32) < 3 as i32 {
                                let fresh1 = (*ConsumptionCostType).cost.len();
                                (*ConsumptionCostType).cost.len() =
                                    ((*ConsumptionCostType).cost.len()).wrapping_add(1);
                                if let Some(cost) =
                                    (*ConsumptionCostType).cost.get_mut(fresh1 as usize)
                                {
                                    decode_iso2_Cost(stream, cost)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            if ((*ConsumptionCostType).cost.len() as i32) < 3 as i32 {
                                grammar_id = 35 as i32;
                            } else {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_pmax_schedule_entry(
    stream: &mut ExiBitstream,
    mut pmax_schedule_entry: &mut Iso2PMaxScheduleEntryType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 36 as i32;
    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            36 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_relative_time_interval(
                                stream,
                                &mut (*pmax_schedule_entry_type).RelativeTimeInterval.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 37 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_interval(
                                stream,
                                &mut (*pmax_schedule_entry_type).TimeInterval.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 37 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            37 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*pmax_schedule_entry_type).PMax,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_sales_tariff_entry(
    stream: &mut ExiBitstream,
    mut sales_tariff_entry: &mut Iso2SalesTariffEntryType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 38 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            38 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_relative_time_interval(
                                stream,
                                &mut (*SalesTariffEntryType).RelativeTimeInterval.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 39 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_interval(
                                stream,
                                &mut (*SalesTariffEntryType).TimeInterval.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 39 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            39 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 41 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        1 => {
                            if ((*SalesTariffEntryType).ConsumptionCost.len() as i32) < 3 as i32
                            {
                                let fresh2 = (*SalesTariffEntryType).ConsumptionCost.len();
                                (*SalesTariffEntryType).ConsumptionCost.len() =
                                    ((*SalesTariffEntryType).ConsumptionCost.len())
                                        .wrapping_add(1);
                                if let Some(consumption_cost) = (*SalesTariffEntryType)
                                    .ConsumptionCost
                                    .array
                                    .get_mut(fresh2 as usize)
                                {
                                    decode_iso2_consumption_cost(stream, consumption_cost)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            grammar_id = 40 as i32;
                        }
                        2 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            40 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*SalesTariffEntryType).ConsumptionCost.len() as i32) < 3 as i32
                            {
                                let fresh3 = (*SalesTariffEntryType).ConsumptionCost.len();
                                (*SalesTariffEntryType).ConsumptionCost.len() =
                                    ((*SalesTariffEntryType).ConsumptionCost.len())
                                        .wrapping_add(1);
                                if let Some(consumption_cost) = (*SalesTariffEntryType)
                                    .ConsumptionCost
                                    .array
                                    .get_mut(fresh3 as usize)
                                {
                                    decode_iso2_consumption_cost(stream, consumption_cost)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            if ((*SalesTariffEntryType).ConsumptionCost.len() as i32) < 3 as i32
                            {
                                grammar_id = 40 as i32;
                            } else {
                                grammar_id = 41 as i32;
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            41 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*SalesTariffEntryType).ConsumptionCost.len() as i32) < 3 as i32
                            {
                                let fresh4 = (*SalesTariffEntryType).ConsumptionCost.len();
                                (*SalesTariffEntryType).ConsumptionCost.len() =
                                    ((*SalesTariffEntryType).ConsumptionCost.len())
                                        .wrapping_add(1);
                                if let Some(consumption_cost) = (*SalesTariffEntryType)
                                    .ConsumptionCost
                                    .array
                                    .get_mut(fresh4 as usize)
                                {
                                    decode_iso2_consumption_cost(stream, consumption_cost)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            grammar_id = 42 as i32;
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            42 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*SalesTariffEntryType).ConsumptionCost.len() as i32) < 3 as i32
                            {
                                let fresh5 = (*SalesTariffEntryType).ConsumptionCost.len();
                                (*SalesTariffEntryType).ConsumptionCost.len() =
                                    ((*SalesTariffEntryType).ConsumptionCost.len())
                                        .wrapping_add(1);
                                if let Some(consumption_cost) = (*SalesTariffEntryType)
                                    .ConsumptionCost
                                    .array
                                    .get_mut(fresh5 as usize)
                                {
                                    decode_iso2_consumption_cost(stream, consumption_cost)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            if ((*SalesTariffEntryType).ConsumptionCost.len() as i32) < 3 as i32
                            {
                                grammar_id = 42 as i32;
                            } else {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_parameter(
    stream: &mut ExiBitstream,
    mut ParameterType: &mut Iso2ParameterType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 43 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            43 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
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
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 44 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            44 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        2 => {
                            decode_exi_type_integer16(
                                stream,
                                &mut (*ParameterType).shortValue.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        3 => {
                            decode_exi_type_integer32(
                                stream,
                                &mut (*ParameterType).intValue.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        4 => {
                            decode_iso2_PhysicalValue(
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
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                            return Err(ExiError::StringValuesNotSupported);
                                        }
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_pmax_schedule(
    stream: &mut ExiBitstream,
    mut PMaxScheduleType: &mut Iso2PMaxScheduleType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 45 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            45 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*PMaxScheduleType).PMaxScheduleEntry.len() as i32) < 12 as i32 {
                                let fresh6 = (*PMaxScheduleType).PMaxScheduleEntry.len();
                                (*PMaxScheduleType).PMaxScheduleEntry.len() =
                                    ((*PMaxScheduleType).PMaxScheduleEntry.len())
                                        .wrapping_add(1);
                                if let Some(entry) = (*PMaxScheduleType)
                                    .PMaxScheduleEntry
                                    .array
                                    .get_mut(fresh6 as usize)
                                {
                                    decode_iso2_pmax_schedule_entry(stream, entry)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            grammar_id = 46 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            46 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*PMaxScheduleType).PMaxScheduleEntry.len() as i32) < 12 as i32 {
                                let fresh7 = (*PMaxScheduleType).PMaxScheduleEntry.len();
                                (*PMaxScheduleType).PMaxScheduleEntry.len() =
                                    ((*PMaxScheduleType).PMaxScheduleEntry.len())
                                        .wrapping_add(1);
                                if let Some(entry) = (*PMaxScheduleType)
                                    .PMaxScheduleEntry
                                    .array
                                    .get_mut(fresh7 as usize)
                                {
                                    decode_iso2_pmax_schedule_entry(stream, entry)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            if ((*PMaxScheduleType).PMaxScheduleEntry.len() as i32) < 1024 as i32
                            {
                                grammar_id = 46 as i32;
                            } else {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_reference(
    stream: &mut ExiBitstream,
    mut reference: &mut Iso2ReferenceType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 47 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            47 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*reference).id.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*reference).id.unwrap().len as i32 >= 2 as i32 {
                                    (*reference).id.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*reference).id.unwrap().len as usize,
                                        &mut (*reference).id.unwrap().data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 48 as i32;
                        }
                        1 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*reference).ref_type.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*reference).ref_type.unwrap().len as i32 >= 2 as i32 {
                                    (*reference).ref_type.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*reference).ref_type.unwrap().len as usize,
                                        &mut (*reference).ref_type.unwrap().data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 49 as i32;
                        }
                        2 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*reference).uri.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*reference).uri.unwrap().len as i32 >= 2 as i32 {
                                    (*reference).uri.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*reference).uri.unwrap().len as usize,
                                        &mut (*reference).uri.unwrap().data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 50 as i32;
                        }
                        3 => {
                            decode_iso2_transforms(
                                stream,
                                &mut (*reference).transforms.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 51 as i32;
                            }
                        }
                        4 => {
                            decode_iso2_digest_method(
                                stream,
                                &mut (*reference).digest_method,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 52 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            48 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*reference).ref_type.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*reference).ref_type.unwrap().len as i32 >= 2 as i32 {
                                    (*reference).ref_type.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*reference).ref_type.unwrap().len as usize,
                                        &mut (*reference).ref_type.unwrap().data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 49 as i32;
                        }
                        1 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*reference).uri.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*reference).uri.unwrap().len as i32 >= 2 as i32 {
                                    (*reference).uri.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*reference).uri.unwrap().len as usize,
                                        &mut (*reference).uri.unwrap().data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 50 as i32;
                        }
                        2 => {
                            decode_iso2_transforms(
                                stream,
                                &mut (*reference).transforms.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 51 as i32;
                            }
                        }
                        3 => {
                            decode_iso2_digest_method(
                                stream,
                                &mut (*reference).digest_method,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 52 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            49 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*reference).uri.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*reference).uri.unwrap().len as i32 >= 2 as i32 {
                                    (*reference).uri.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*reference).uri.unwrap().len as usize,
                                        &mut (*reference).uri.unwrap().data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 50 as i32;
                        }
                        1 => {
                            decode_iso2_transforms(
                                stream,
                                &mut (*reference).transforms.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 51 as i32;
                            }
                        }
                        2 => {
                            decode_iso2_digest_method(
                                stream,
                                &mut (*reference).digest_method,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 52 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            50 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_transforms(
                                stream,
                                &mut (*reference).transforms.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 51 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_digest_method(
                                stream,
                                &mut (*reference).digest_method,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 52 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            51 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_digest_method(
                                stream,
                                &mut (*reference).digest_method,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 52 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            52 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*reference).digest_value.unwrap().len,
                                &mut (*reference).digest_value.unwrap().data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_retrieval_method(
    stream: &mut ExiBitstream,
    mut retrieval_method: &mut Iso2RetrievalMethodType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 53 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            53 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*retrieval_method).ret_type.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*retrieval_method).ret_type.unwrap().len as i32 >= 2 as i32 {
                                    (*retrieval_method).ret_type.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*retrieval_method).ret_type.unwrap().len as usize,
                                        &mut (*retrieval_method).ret_type.unwrap().data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 54 as i32;
                        }
                        1 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*retrieval_method).uri.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*retrieval_method).uri.unwrap().len as i32 >= 2 as i32 {
                                    (*retrieval_method).uri.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*retrieval_method).uri.unwrap().len as usize,
                                        &mut (*retrieval_method).uri.unwrap().data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 55 as i32;
                        }
                        2 => {
                            decode_iso2_transforms(
                                stream,
                                &mut (*retrieval_method).transforms.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        3 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            54 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*retrieval_method).uri.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*retrieval_method).uri.unwrap().len as i32 >= 2 as i32 {
                                    (*retrieval_method).uri.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*retrieval_method).uri.unwrap().len as usize,
                                        &mut (*retrieval_method).uri.unwrap().data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 55 as i32;
                        }
                        1 => {
                            decode_iso2_transforms(
                                stream,
                                &mut (*retrieval_method).transforms.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            55 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_transforms(
                                stream,
                                &mut (*retrieval_method).transforms.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_sales_tariff(
    stream: &mut ExiBitstream,
    mut sales_tariff: &mut Iso2SalesTariffType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 56 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            56 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*sales_tariff).id.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*sales_tariff).id.unwrap().len as i32 >= 2 as i32 {
                                    (*sales_tariff).id.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*sales_tariff).id.unwrap().len as usize,
                                        &mut (*sales_tariff).id.unwrap().data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 57 as i32;
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        8 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*sales_tariff).SalesTariffID =
                                            value.wrapping_add(1 as i32 as u32) as u8;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 58 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            57 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        8 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*sales_tariff).SalesTariffID =
                                            value_0.wrapping_add(1 as i32 as u32) as u8;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 58 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            58 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*sales_tariff).sales_tariff_description.unwrap().len
                                            as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*sales_tariff).sales_tariff_description.unwrap().len
                                            as i32
                                            >= 2 as i32
                                        {
                                            (*sales_tariff)
                                                .sales_tariff_description
                                                .unwrap()
                                                .len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*sales_tariff)
                                                    .sales_tariff_description
                                                    .unwrap()
                                                    .len,
                                                &mut (*sales_tariff)
                                                    .sales_tariff_description
                                                    .unwrap()
                                                    .data,
                                                (32 as i32 + 1 as i32) as usize,
                                            )?;
                                        } else {
                                            return Err(ExiError::StringValuesNotSupported);
                                        }
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 60 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value_1: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        8 as i32 as usize,
                                        &mut value_1,
                                    )?;
                                    if error == 0 as i32 {
                                        (*sales_tariff).num_e_price_levels = Some(value_1 as u8);
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 62 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        2 => {
                            if ((*sales_tariff).sales_tariff_entry.len() as i32) < 12 as i32 {
                                let fresh8 = (*sales_tariff).sales_tariff_entry.len();
                                (*sales_tariff).sales_tariff_entry.len() =
                                    ((*sales_tariff).sales_tariff_entry.len()).wrapping_add(1);
                                if let Some(entry) = (*sales_tariff)
                                    .sales_tariff_entry
                                    .array
                                    .get_mut(fresh8 as usize)
                                {
                                    decode_iso2_sales_tariff_entry(stream, entry)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            grammar_id = 59 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            59 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*sales_tariff).sales_tariff_entry.len() as i32) < 12 as i32 {
                                let fresh9 = (*sales_tariff).sales_tariff_entry.len();
                                (*sales_tariff).sales_tariff_entry.len() =
                                    ((*sales_tariff).sales_tariff_entry.len()).wrapping_add(1);
                                if let Some(entry) = (*sales_tariff)
                                    .sales_tariff_entry
                                    .array
                                    .get_mut(fresh9 as usize)
                                {
                                    decode_iso2_sales_tariff_entry(stream, entry)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            if ((*sales_tariff).sales_tariff_entry.len() as i32) < 1024 as i32 {
                                grammar_id = 59 as i32;
                            } else {
                                grammar_id = 60 as i32;
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            60 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value_2: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        8 as i32 as usize,
                                        &mut value_2,
                                    )?;
                                    if error == 0 as i32 {
                                        (*sales_tariff).num_e_price_levels = Some(value_2 as u8);
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 62 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        1 => {
                            if ((*sales_tariff).sales_tariff_entry.len() as i32) < 12 as i32 {
                                let fresh10 = (*sales_tariff).sales_tariff_entry.len();
                                (*sales_tariff).sales_tariff_entry.len() =
                                    ((*sales_tariff).sales_tariff_entry.len()).wrapping_add(1);
                                if let Some(entry) = (*sales_tariff)
                                    .sales_tariff_entry
                                    .array
                                    .get_mut(fresh10 as usize)
                                {
                                    decode_iso2_sales_tariff_entry(stream, entry)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            grammar_id = 61 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            61 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*sales_tariff).sales_tariff_entry.len() as i32) < 12 as i32 {
                                let fresh11 = (*sales_tariff).sales_tariff_entry.len();
                                (*sales_tariff).sales_tariff_entry.len() =
                                    ((*sales_tariff).sales_tariff_entry.len()).wrapping_add(1);
                                if let Some(entry) = (*sales_tariff)
                                    .sales_tariff_entry
                                    .array
                                    .get_mut(fresh11 as usize)
                                {
                                    decode_iso2_sales_tariff_entry(stream, entry)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            if ((*sales_tariff).SalesTariffEntry.len() as i32) < 1024 as i32 {
                                grammar_id = 61 as i32;
                            } else {
                                grammar_id = 62 as i32;
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            62 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*sales_tariff).sales_tariff_entry.len() as i32) < 12 as i32 {
                                let fresh12 = (*sales_tariff).sales_tariff_entry.len();
                                (*sales_tariff).sales_tariff_entry.len() =
                                    ((*sales_tariff).sales_tariff_entry.len()).wrapping_add(1);
                                if let Some(entry) = (*sales_tariff)
                                    .sales_tariff_entry
                                    .array
                                    .get_mut(fresh12 as usize)
                                {
                                    decode_iso2_sales_tariff_entry(stream, entry)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            grammar_id = 63 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            63 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*sales_tariff).sales_tariff_entry.len() as i32) < 12 as i32 {
                                let fresh13 = (*sales_tariff).sales_tariff_entry.len();
                                (*sales_tariff).sales_tariff_entry.len() =
                                    ((*sales_tariff).sales_tariff_entry.len()).wrapping_add(1);
                                if let Some(entry) = (*sales_tariff)
                                    .sales_tariff_entry
                                    .array
                                    .get_mut(fresh13 as usize)
                                {
                                    decode_iso2_sales_tariff_entry(stream, entry)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            if ((*sales_tariff).sales_tariff_entry.len() as i32) < 1024 as i32 {
                                grammar_id = 63 as i32;
                            } else {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_x509_data(
    stream: &mut ExiBitstream,
    mut X509DataType: &mut Iso2X509DataType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 64 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            64 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_X509IssuerSerial(
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
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                            return Err(ExiError::StringValuesNotSupported);
                                        }
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
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
                                &mut (*X509DataType).any.unwrap().len,
                                &mut (*X509DataType).any.unwrap().data,
                                4 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_pgp_data(
    stream: &mut ExiBitstream,
    mut PGPDataType: &mut Iso2PGPDataType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 65 as i32;
    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            65 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            let pgp_data = match &mut (*PGPDataType).PGPComponent {
                                Iso2PGPComponentType::Choice1(ref mut c1) => c1,
                                _ => return Err(ExiError::UnknownEventCode),
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
                                _ => return Err(ExiError::UnknownEventCode),
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
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            66 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            let pgp_data = match &mut (*PGPDataType).PGPComponent {
                                Iso2PGPComponentType::Choice1(ref mut c1) => c1,
                                _ => return Err(ExiError::UnknownEventCode),
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
                            return Err(ExiError::UnknownEventForDecoding);
                        }
                        2 => {
                            return Ok(());
                        }
                        3 => {
                            let pgp_data = match &mut (*PGPDataType).PGPComponent {
                                Iso2PGPComponentType::Choice1(ref mut c1) => c1,
                                _ => return Err(ExiError::UnknownEventCode),
                            };
                            decode_exi_type_hex_binary(
                                stream,
                                &mut pgp_data.any.unwrap().len,
                                &mut pgp_data.any.unwrap().data,
                                4 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 68 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            67 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Err(ExiError::UnknownEventForDecoding);
                        }
                        1 => {
                            return Ok(());
                        }
                        2 => {
                            return Ok(());
                        }
                        3 => {
                            let pgp_data = match &mut (*PGPDataType).PGPComponent {
                                Iso2PGPComponentType::Choice1(ref mut c1) => c1,
                                _ => return Err(ExiError::UnknownEventCode),
                            };
                            decode_exi_type_hex_binary(
                                stream,
                                &mut pgp_data.any.unwrap().len,
                                &mut pgp_data.any.unwrap().data,
                                4 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 68 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            68 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            // Ensure we are using PGPChoice2Type
                            let pgp_data = match &mut (*PGPDataType).PGPComponent {
                                Iso2PGPComponentType::Choice2(ref mut c2) => c2,
                                _ => return Err(ExiError::UnknownEventCode),
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
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            69 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Err(ExiError::UnknownEventForDecoding);
                        }
                        1 => {
                            return Ok(());
                        }
                        2 => {
                            let pgp_data = match &mut (*PGPDataType).PGPComponent {
                                Iso2PGPComponentType::Choice2(ref mut c2) => c2,
                                _ => return Err(ExiError::UnknownEventCode),
                            };
                            decode_exi_type_hex_binary(
                                stream,
                                &mut pgp_data.any.unwrap().len,
                                &mut pgp_data.any.unwrap().data,
                                4,
                            )?;
                            grammar_id = 68 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_spki_data(
    stream: &mut ExiBitstream,
    mut spkidata_type: &mut Iso2SPKIDataType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 70 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            70 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*spkidata_type).SPKISexp.len,
                                &mut (*spkidata_type).SPKISexp.data,
                                350 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 71 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            71 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Err(ExiError::UnknownEventForDecoding);
                        }
                        1 => {
                            return Ok(());
                        }
                        2 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*spkidata_type).any.unwrap().len,
                                &mut (*spkidata_type).any.unwrap().data,
                                4 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_signed_info(
    stream: &mut ExiBitstream,
    mut signed_info: &mut Iso2SignedInfoType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 72 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            72 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*SignedInfoType).id.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*SignedInfoType).id.unwrap().len as i32 >= 2 as i32 {
                                    (*SignedInfoType).id.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*SignedInfoType).id.unwrap().len,
                                        &mut (*SignedInfoType).id.unwrap().data,
                                        65,
                                    )?;
                                } else {
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 73 as i32;
                        }
                        1 => {
                            decode_iso2_canonicalization_method(
                                stream,
                                &mut (*SignedInfoType).CanonicalizationMethod,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 74 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            73 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_canonicalization_method(
                                stream,
                                &mut (*SignedInfoType).CanonicalizationMethod,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 74 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            74 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_signature_method(
                                stream,
                                &mut (*SignedInfoType).SignatureMethod,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 75 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            75 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if (*SignedInfoType).ReferenceLen < 4 {
                                let fresh14 = (*SignedInfoType).ReferenceLen;
                                if let Some(reference) =
                                    (*SignedInfoType).Reference.get_mut(fresh14 as usize)
                                {
                                    decode_iso2_Reference(stream, reference)?;
                                    (*SignedInfoType).ReferenceLen += 1;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            grammar_id = 76 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            76 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*SignedInfoType).ReferenceLen as i32) < 4 as i32 {
                                let fresh15 = (*SignedInfoType).ReferenceLen;
                                (*SignedInfoType).ReferenceLen =
                                    ((*SignedInfoType).ReferenceLen).wrapping_add(1);
                                if let Some(reference) =
                                    (*SignedInfoType).Reference.get_mut(fresh15 as usize)
                                {
                                    decode_iso2_Reference(stream, reference)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            grammar_id = 76 as i32;
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_profile_entry(
    stream: &mut ExiBitstream,
    mut profile_entry: &mut Iso2ProfileEntryType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 77 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            77 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
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
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            78 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*ProfileEntryType).ChargingProfileEntryMaxPower,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 79 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            79 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_dc_ev_status(
    stream: &mut ExiBitstream,
    mut dc_ev_status_type: &mut Iso2DCEVStatusType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 80 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            80 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*dc_evstatus_type).EVReady = value as i32;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 81 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            81 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        4 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*dc_evstatus_type).EVErrorCode =
                                            value_0 as Iso2DcEvErrorCodeType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 82 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            82 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value_1: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_1,
                                    )?;
                                    if error == 0 as i32 {
                                        (*dc_evstatus_type).EVRESSSOC = value_1 as i8;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_parameter_set(
    stream: &mut ExiBitstream,
    mut parameter_set: &mut Iso2ParameterSetType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 83 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            83 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
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
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            84 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*ParameterSetType).Parameter.len() as i32) < 16 as i32 {
                                let fresh16 = (*ParameterSetType).Parameter.len();
                                (*ParameterSetType).Parameter.len() =
                                    ((*ParameterSetType).Parameter.len()).wrapping_add(1);
                                if let Some(param) = (*ParameterSetType)
                                    .Parameter
                                    .array
                                    .get_mut(fresh16 as usize)
                                {
                                    decode_iso2_parameter(stream, param)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            grammar_id = 85 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            85 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*ParameterSetType).Parameter.len() as i32) < 16 as i32 {
                                let fresh17 = (*ParameterSetType).Parameter.len();
                                (*ParameterSetType).Parameter.len() =
                                    ((*ParameterSetType).Parameter.len()).wrapping_add(1);
                                if let Some(param) = (*ParameterSetType)
                                    .Parameter
                                    .array
                                    .get_mut(fresh17 as usize)
                                {
                                    decode_iso2_parameter(stream, param)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            if ((*ParameterSetType).Parameter.len() as i32) < 16 as i32 {
                                grammar_id = 85 as i32;
                            } else {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_sa_schedule_tuple(
    stream: &mut ExiBitstream,
    mut sa_schedule_tuple: &mut Iso2SAScheduleTupleType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 86 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            86 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 87 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            87 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_pmax_schedule(
                                stream,
                                &mut (*SAScheduleTupleType).PMaxSchedule,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 88 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            88 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_sales_tariff(
                                stream,
                                &mut (*SAScheduleTupleType).SalesTariff.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_selected_service(
    stream: &mut ExiBitstream,
    mut selected_service: &mut Iso2SelectedServiceType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 89 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            89 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_uint16(stream, &mut (*SelectedServiceType).ServiceID)?;
                            if error == 0 as i32 {
                                grammar_id = 90 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            90 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
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
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_service(
    stream: &mut ExiBitstream,
    mut service: &mut Iso2ServiceType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 91 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            91 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_uint16(stream, &mut (*ServiceType).ServiceID)?;
                            if error == 0 as i32 {
                                grammar_id = 92 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            92 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                            return Err(ExiError::StringValuesNotSupported);
                                        }
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 93 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ServiceType).service_category =
                                            value as Iso2ServiceCategoryType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 94 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            93 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ServiceType).service_category =
                                            value_0 as Iso2ServiceCategoryType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 94 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            94 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                            return Err(ExiError::StringValuesNotSupported);
                                        }
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 95 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            95 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_signature_value(
    stream: &mut ExiBitstream,
    mut signature_value_type: &mut Iso2SignatureValueType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 96 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            96 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*SignatureValueType).id.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*SignatureValueType).id.unwrap().len as i32 >= 2 as i32 {
                                    (*SignatureValueType).id.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*SignatureValueType).id.unwrap().len as usize,
                                        &mut (*SignatureValueType).id.unwrap().data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(ExiError::StringValuesNotSupported);
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
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            97 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
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
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_sub_certificates(
    stream: &mut ExiBitstream,
    mut sub_certificates: &mut Iso2SubCertificatesType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 98 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            98 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if (sub_certificates_type.Certificate.len() as i32) < 4 as i32 {
                                let idx = sub_certificates.Certificate.len() as usize;
                                if let Some(cert) = sub_certificates.Certificate.get_mut(idx) {
                                    decode_exi_type_hex_binary(
                                        stream,
                                        &mut cert.len,
                                        &mut cert.data,
                                        800 as usize,
                                    )?;
                                    if error == 0 as i32 {
                                        (*SubCertificatesType).Certificate.len() =
                                            ((*SubCertificatesType).Certificate.len())
                                                .wrapping_add(1);
                                        grammar_id = 99 as i32;
                                    }
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            99 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if (sub_certificates.Certificate.len() as i32) < 4 as i32 {
                                let idx = sub_certificates.Certificate.len() as usize;
                                if let Some(cert) = sub_certificates.Certificate.get_mut(idx) {
                                    decode_exi_type_hex_binary(
                                        stream,
                                        &mut cert.len,
                                        &mut cert.data,
                                        800 as usize,
                                    )?;
                                    if error == 0 as i32 {
                                        sub_certificates.Certificate.len() =
                                            (sub_certificates.Certificate.len())
                                                .wrapping_add(1);
                                        grammar_id = 99 as i32;
                                    }
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_key_info(
    stream: &mut ExiBitstream,
    mut key_info_type: &mut Iso2KeyInfoType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 100 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            100 => {
                exi_basetypes_decoder_nbit_uint(stream, 4 as i32 as usize, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*KeyInfoType).id.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*KeyInfoType).id.unwrap().len as i32 >= 2 as i32 {
                                    (*KeyInfoType).id.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*KeyInfoType).id.unwrap().len,
                                        &mut (*KeyInfoType).id.unwrap().data,
                                        65,
                                    )?;
                                } else {
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 101 as i32;
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                            return Err(ExiError::StringValuesNotSupported);
                                        }
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        2 => {
                            decode_iso2_KeyValue(
                                stream,
                                &mut (*KeyInfoType).KeyValue.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        3 => {
                            decode_iso2_RetrievalMethod(
                                stream,
                                &mut (*KeyInfoType).RetrievalMethod.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        4 => {
                            decode_iso2_x509_data(
                                stream,
                                &mut (*KeyInfoType).X509Data.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        5 => {
                            decode_iso2_pgpdata(stream, &mut (*KeyInfoType).PGPData.unwrap())?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        6 => {
                            decode_iso2_spkidata(
                                stream,
                                &mut (*KeyInfoType).SPKIData.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        7 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                            return Err(ExiError::StringValuesNotSupported);
                                        }
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        8 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*KeyInfoType).any.unwrap().len,
                                &mut (*KeyInfoType).any.unwrap().data,
                                4,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            101 => {
                exi_basetypes_decoder_nbit_uint(stream, 4 as i32 as usize, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                            return Err(ExiError::StringValuesNotSupported);
                                        }
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        1 => {
                            decode_iso2_KeyValue(
                                stream,
                                &mut (*KeyInfoType).KeyValue.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            decode_iso2_RetrievalMethod(
                                stream,
                                &mut (*KeyInfoType).RetrievalMethod.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        3 => {
                            decode_iso2_x509_data(
                                stream,
                                &mut (*KeyInfoType).X509Data.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        4 => {
                            decode_iso2_pgpdata(stream, &mut (*KeyInfoType).PGPData.unwrap())?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        5 => {
                            decode_iso2_spkidata(
                                stream,
                                &mut (*KeyInfoType).SPKIData.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        6 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*KeyInfoType).MgmtData.unwrap().len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*KeyInfoType).MgmtData.unwrap().len as i32 >= 2 as i32 {
                                            (*KeyInfoType).MgmtData.unwrap().len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*KeyInfoType).MgmtData.unwrap().len as usize,
                                                &mut (*KeyInfoType).MgmtData.unwrap().data,
                                                (64 as i32 + 1 as i32) as usize,
                                            )?;
                                        } else {
                                            return Err(ExiError::StringValuesNotSupported);
                                        }
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        7 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*KeyInfoType).any.unwrap().len,
                                &mut (*KeyInfoType).any.unwrap().data,
                                4 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_object(
    stream: &mut ExiBitstream,
    mut ObjectType: &mut Iso2ObjectType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 102 as i32;
    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            102 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
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
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 103 as i32;
                        }
                        1 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*ObjectType).id.as_mut().unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*ObjectType).id.as_ref().unwrap().len as i32 >= 2 as i32 {
                                    (*ObjectType).id.as_mut().unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*ObjectType).id.as_ref().unwrap().len,
                                        &mut (*ObjectType).id.as_mut().unwrap().data,
                                        65,
                                    )?;
                                } else {
                                    return Err(ExiError::StringValuesNotSupported);
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
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 105 as i32;
                        }
                        3 => {
                            return Err(ExiError::UnknownEventForDecoding);
                        }
                        4 => {
                            return Ok(());
                        }
                        5 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*ObjectType).any.unwrap().len,
                                &mut (*ObjectType).any.unwrap().data,
                                4 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            103 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*ObjectType).id.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*ObjectType).id.unwrap().len as i32 >= 2 as i32 {
                                    (*ObjectType).id.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*ObjectType).id.unwrap().len,
                                        &mut (*ObjectType).id.unwrap().data,
                                        65,
                                    )?;
                                } else {
                                    return Err(ExiError::StringValuesNotSupported);
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
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 105 as i32;
                        }
                        2 => {
                            return Err(ExiError::UnknownEventForDecoding);
                        }
                        3 => {
                            return Ok(());
                        }
                        4 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*ObjectType).any.unwrap().len,
                                &mut (*ObjectType).any.unwrap().data,
                                4,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            104 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
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
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 105 as i32;
                        }
                        1 => {
                            return Err(ExiError::UnknownEventForDecoding);
                        }
                        2 => {
                            return Ok(());
                        }
                        3 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*ObjectType).any.unwrap().len,
                                &mut (*ObjectType).any.unwrap().data,
                                4,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            105 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Err(ExiError::UnknownEventForDecoding);
                        }
                        1 => {
                            return Ok(());
                        }
                        2 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*ObjectType).any.unwrap().len,
                                &mut (*ObjectType).any.unwrap().data,
                                4,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_supported_energy_transfer_mode(
    stream: &mut ExiBitstream,
    mut supported_energy_transfer_mode_type: &mut Iso2SupportedEnergyTransferModeType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 106 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            106 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*SupportedEnergyTransferModeType)
                                .EnergyTransferMode
                                .len() as i32)
                                < 6 as i32
                            {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
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
                                                    .len()
                                                    as usize] = value as Iso2EnergyTransferModeType;
                                            (*SupportedEnergyTransferModeType)
                                                .EnergyTransferMode
                                                .len() = ((*SupportedEnergyTransferModeType)
                                                .EnergyTransferMode
                                                .len())
                                                .wrapping_add(1);
                                            (*SupportedEnergyTransferModeType)
                                                .EnergyTransferMode
                                                .len();
                                        }
                                    } else {
                                        return Err(ExiError::UnsupportedSubEvent);
                                    }
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 107 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            107 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*SupportedEnergyTransferModeType)
                                .EnergyTransferMode
                                .len() as i32)
                                < 6 as i32
                            {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
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
                                                    .len()
                                                    as usize] =
                                                value_0 as Iso2EnergyTransferModeType;
                                            (*SupportedEnergyTransferModeType)
                                                .EnergyTransferMode
                                                .len() = ((*SupportedEnergyTransferModeType)
                                                .EnergyTransferMode
                                                .len())
                                                .wrapping_add(1);
                                            (*SupportedEnergyTransferModeType)
                                                .EnergyTransferMode
                                                .len();
                                        }
                                    } else {
                                        return Err(ExiError::UnsupportedSubEvent);
                                    }
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 107 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_certificate_chain(
    stream: &mut ExiBitstream,
    mut CertificateChainType: &mut Iso2CertificateChainType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 108 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            108 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*CertificateChainType).id.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*CertificateChainType).id.unwrap().len as i32 >= 2 as i32 {
                                    (*CertificateChainType).id.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*CertificateChainType).id.unwrap().len,
                                        &mut (*CertificateChainType).id.unwrap().data,
                                        65,
                                    )?;
                                } else {
                                    return Err(ExiError::StringValuesNotSupported);
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
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            109 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
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
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            110 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_SubCertificates(
                                stream,
                                &mut (*CertificateChainType).SubCertificates.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_body_base(
    stream: &mut ExiBitstream,
    mut _BodyBaseType: &mut Iso2BodyBaseType,
) -> Result<(), ExiError> {
    let mut event_code: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code != 0 as i32 as u32 {
        return Err(ExiError::UnknownEventCode);
    }
    return Ok(());
}
pub fn decode_iso2_notification(
    stream: &mut ExiBitstream,
    mut NotificationType: &mut Iso2NotificationType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 111 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            111 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 112 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            112 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                            return Err(ExiError::StringValuesNotSupported);
                                        }
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_dc_evse_status(
    stream: &mut ExiBitstream,
    mut DC_EVSEStatusType: &mut Iso2DCEVSEStatusType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 113 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            113 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
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
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            114 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 115 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            115 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 116 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            116 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_selected_service_list(
    stream: &mut ExiBitstream,
    mut SelectedServiceListType: &mut Iso2SelectedServiceListType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 117 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            117 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*SelectedServiceListType).SelectedService.len() as i32)
                                < 16 as i32
                            {
                                let fresh18 = (*SelectedServiceListType).SelectedService.len();
                                (*SelectedServiceListType).SelectedService.len() =
                                    ((*SelectedServiceListType).SelectedService.len())
                                        .wrapping_add(1);
                                if let Some(selected_service) = (*SelectedServiceListType)
                                    .SelectedService
                                    .array
                                    .get_mut(fresh18 as usize)
                                {
                                    decode_iso2_SelectedService(stream, selected_service)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            grammar_id = 118 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            118 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*SelectedServiceListType).SelectedService.len() as i32)
                                < 16 as i32
                            {
                                let fresh19 = (*SelectedServiceListType).SelectedService.len();
                                (*SelectedServiceListType).SelectedService.len() =
                                    ((*SelectedServiceListType).SelectedService.len())
                                        .wrapping_add(1);
                                if let Some(selected_service) = (*SelectedServiceListType)
                                    .SelectedService
                                    .array
                                    .get_mut(fresh19 as usize)
                                {
                                    decode_iso2_SelectedService(stream, selected_service)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            if ((*SelectedServiceListType).SelectedService.len() as i32)
                                < 16 as i32
                            {
                                grammar_id = 118 as i32;
                            } else {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_payment_option_list(
    stream: &mut ExiBitstream,
    mut PaymentOptionListType: &mut Iso2PaymentOptionListType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 119 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            119 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*PaymentOptionListType).PaymentOption.len() as i32) < 2 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        let mut value: u32 = 0;
                                        exi_basetypes_decoder_nbit_uint(
                                            stream,
                                            1 as i32 as usize,
                                            &mut value,
                                        )?;
                                        if error == 0 as i32 {
                                            (*PaymentOptionListType).PaymentOption.array
                                                [(*PaymentOptionListType).PaymentOption.len()
                                                    as usize] = value as Iso2PaymentOptionType;
                                            (*PaymentOptionListType).PaymentOption.len() =
                                                ((*PaymentOptionListType).PaymentOption.len())
                                                    .wrapping_add(1);
                                            (*PaymentOptionListType).PaymentOption.len();
                                        }
                                    } else {
                                        return Err(ExiError::UnsupportedSubEvent);
                                    }
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 120 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            120 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*PaymentOptionListType).PaymentOption.len() as i32) < 2 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        let mut value_0: u32 = 0;
                                        exi_basetypes_decoder_nbit_uint(
                                            stream,
                                            1 as i32 as usize,
                                            &mut value_0,
                                        )?;
                                        if error == 0 as i32 {
                                            (*PaymentOptionListType).PaymentOption.array
                                                [(*PaymentOptionListType).PaymentOption.len()
                                                    as usize] = value_0 as Iso2PaymentOptionType;
                                            (*PaymentOptionListType).PaymentOption.len() =
                                                ((*PaymentOptionListType).PaymentOption.len())
                                                    .wrapping_add(1);
                                            (*PaymentOptionListType).PaymentOption.len();
                                        }
                                    } else {
                                        return Err(ExiError::UnsupportedSubEvent);
                                    }
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_signature(
    stream: &mut ExiBitstream,
    mut SignatureType: &mut Iso2SignatureType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 121 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            121 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*SignatureType).id.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*SignatureType).id.unwrap().len as i32 >= 2 as i32 {
                                    (*SignatureType).id.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*SignatureType).id.unwrap().len,
                                        &mut (*SignatureType).id.unwrap().data,
                                        65,
                                    )?;
                                } else {
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 122 as i32;
                        }
                        1 => {
                            decode_iso2_SignedInfo(stream, &mut (*SignatureType).SignedInfo)?;
                            if error == 0 as i32 {
                                grammar_id = 123 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            122 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_SignedInfo(stream, &mut (*SignatureType).SignedInfo)?;
                            if error == 0 as i32 {
                                grammar_id = 123 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            123 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_signature_value(
                                stream,
                                &mut (*SignatureType).SignatureValue,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 124 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            124 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_KeyInfo(
                                stream,
                                &mut (*SignatureType).KeyInfo.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 126 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_object(stream, &mut (*SignatureType).Object.unwrap())?;
                            if error == 0 as i32 {
                                grammar_id = 125 as i32;
                            }
                        }
                        2 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            125 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Err(ExiError::ArrayOutOfBounds);
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            126 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_object(stream, &mut (*SignatureType).Object.unwrap())?;
                            if error == 0 as i32 {
                                grammar_id = 127 as i32;
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            127 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Err(ExiError::ArrayOutOfBounds);
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_charging_profile(
    stream: &mut ExiBitstream,
    mut ChargingProfileType: &mut Iso2ChargingProfileType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 128 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            128 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*ChargingProfileType).ProfileEntry.len() as i32) < 24 as i32 {
                                let fresh20 = (*ChargingProfileType).ProfileEntry.len();
                                (*ChargingProfileType).ProfileEntry.len() =
                                    ((*ChargingProfileType).ProfileEntry.len()).wrapping_add(1);
                                if let Some(entry) = (*ChargingProfileType)
                                    .ProfileEntry
                                    .array
                                    .get_mut(fresh20 as usize)
                                {
                                    decode_iso2_ProfileEntry(stream, entry)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            grammar_id = 129 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            129 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*ChargingProfileType).ProfileEntry.len() as i32) < 24 as i32 {
                                let fresh21 = (*ChargingProfileType).ProfileEntry.len();
                                (*ChargingProfileType).ProfileEntry.len() =
                                    ((*ChargingProfileType).ProfileEntry.len()).wrapping_add(1);
                                if let Some(entry) = (*ChargingProfileType)
                                    .ProfileEntry
                                    .array
                                    .get_mut(fresh21 as usize)
                                {
                                    decode_iso2_ProfileEntry(stream, entry)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            if ((*ChargingProfileType).ProfileEntry.len() as i32) < 24 as i32 {
                                grammar_id = 129 as i32;
                            } else {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_service_parameter_list(
    stream: &mut ExiBitstream,
    mut ServiceParameterListType: &mut Iso2ServiceParameterListType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 130 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            130 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*ServiceParameterListType).ParameterSet.len() as i32) < 5 as i32
                            {
                                let fresh22 = (*ServiceParameterListType).ParameterSet.len();
                                (*ServiceParameterListType).ParameterSet.len() =
                                    ((*ServiceParameterListType).ParameterSet.len())
                                        .wrapping_add(1);
                                if let Some(param_set) = (*ServiceParameterListType)
                                    .ParameterSet
                                    .array
                                    .get_mut(fresh22 as usize)
                                {
                                    decode_iso2_parameter_set(stream, param_set)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            grammar_id = 131 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            131 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*ServiceParameterListType).ParameterSet.len() as i32) < 5 as i32
                            {
                                let fresh23 = (*ServiceParameterListType).ParameterSet.len();
                                (*ServiceParameterListType).ParameterSet.len() =
                                    ((*ServiceParameterListType).ParameterSet.len())
                                        .wrapping_add(1);
                                if let Some(param_set) = (*ServiceParameterListType)
                                    .ParameterSet
                                    .array
                                    .get_mut(fresh23 as usize)
                                {
                                    decode_iso2_parameter_set(stream, param_set)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            if ((*ServiceParameterListType).ParameterSet.len() as i32)
                                < 255 as i32
                            {
                                grammar_id = 131 as i32;
                            } else {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_list_of_root_certificate_ids(
    stream: &mut ExiBitstream,
    mut ListOfRootCertificateIDsType: &mut Iso2ListOfRootCertificateIDsType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 132 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            132 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*ListOfRootCertificateIDsType).RootCertificateID.len() as i32)
                                < 5 as i32
                            {
                                let fresh24 =
                                    (*ListOfRootCertificateIDsType).RootCertificateID.len();
                                (*ListOfRootCertificateIDsType).RootCertificateID.len() =
                                    ((*ListOfRootCertificateIDsType).RootCertificateID.len())
                                        .wrapping_add(1);
                                if let Some(root_cert) = (*ListOfRootCertificateIDsType)
                                    .RootCertificateID
                                    .array
                                    .get_mut(fresh24 as usize)
                                {
                                    decode_iso2_X509IssuerSerial(stream, root_cert)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            grammar_id = 133 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            133 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*ListOfRootCertificateIDsType).RootCertificateID.len() as i32)
                                < 5 as i32
                            {
                                let fresh25 =
                                    (*ListOfRootCertificateIDsType).RootCertificateID.len();
                                (*ListOfRootCertificateIDsType).RootCertificateID.len() =
                                    ((*ListOfRootCertificateIDsType).RootCertificateID.len())
                                        .wrapping_add(1);
                                if let Some(root_cert) = (*ListOfRootCertificateIDsType)
                                    .RootCertificateID
                                    .array
                                    .get_mut(fresh25 as usize)
                                {
                                    decode_iso2_X509IssuerSerial(stream, root_cert)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            if ((*ListOfRootCertificateIDsType).RootCertificateID.len() as i32)
                                < 20 as i32
                            {
                                grammar_id = 133 as i32;
                            } else {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_ac_ev_charge_parameter(
    stream: &mut ExiBitstream,
    mut AC_EVChargeParameterType: &mut Iso2ACEVChargeParameterType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 134 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            134 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
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
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*AC_EVChargeParameterType).EAmount,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 136 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            135 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*AC_EVChargeParameterType).EAmount,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 136 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            136 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*AC_EVChargeParameterType).EVMaxVoltage,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 137 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            137 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*AC_EVChargeParameterType).EVMaxCurrent,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 138 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            138 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*AC_EVChargeParameterType).EVMinCurrent,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_dc_ev_charge_parameter(
    stream: &mut ExiBitstream,
    mut DC_EVChargeParameterType: &mut Iso2DCEVChargeParameterType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 139 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            139 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
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
                            decode_iso2_DC_EVStatus(
                                stream,
                                &mut (*DC_EVChargeParameterType).dc_ev_status,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 141 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            140 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_DC_EVStatus(
                                stream,
                                &mut (*DC_EVChargeParameterType).dc_ev_status,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 141 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            141 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*DC_EVChargeParameterType).EVMaximumCurrentLimit,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 142 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            142 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*DC_EVChargeParameterType).EVMaximumPowerLimit.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 143 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*DC_EVChargeParameterType).EVMaximumVoltageLimit,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 144 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            143 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*DC_EVChargeParameterType).EVMaximumVoltageLimit,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 144 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            144 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*DC_EVChargeParameterType).EVEnergyCapacity.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 145 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_PhysicalValue(
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
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 147 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        3 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        4 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            145 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
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
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 147 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        2 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        3 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            146 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 147 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        2 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            147 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_ev_charge_parameter(
    stream: &mut ExiBitstream,
    mut EVChargeParameterType: &mut Iso2EVChargeParameterType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 148 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            148 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
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
                            decode_iso2_ac_evcharge_parameter(
                                stream,
                                &mut (*EVChargeParameterType).AC_EVChargeParameter,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 150 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            149 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_ac_evcharge_parameter(
                                stream,
                                &mut (*EVChargeParameterType).AC_EVChargeParameter,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 150 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            150 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_dc_evcharge_parameter(
                                stream,
                                &mut (*EVChargeParameterType).DC_EVChargeParameter,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_sa_schedules(
    stream: &mut ExiBitstream,
    mut _SASchedulesType: &mut Iso2SASchedulesType,
) -> Result<(), ExiError> {
    let mut event_code: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code != 0 as i32 as u32 {
        return Err(ExiError::UnknownEventCode);
    }
    return Ok(());
}
pub fn decode_iso2_sa_schedule_list(
    stream: &mut ExiBitstream,
    mut SAScheduleListType: &mut Iso2SAScheduleListType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 151 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            151 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*SAScheduleListType).SAScheduleTuple.len() as i32) < 3 as i32 {
                                let fresh26 = (*SAScheduleListType).SAScheduleTuple.len();
                                (*SAScheduleListType).SAScheduleTuple.len() =
                                    ((*SAScheduleListType).SAScheduleTuple.len())
                                        .wrapping_add(1);
                                if let Some(tuple) = (*SAScheduleListType)
                                    .SAScheduleTuple
                                    .array
                                    .get_mut(fresh26 as usize)
                                {
                                    decode_iso2_saschedule_tuple(stream, tuple)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            grammar_id = 152 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            152 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*SAScheduleListType).SAScheduleTuple.len() as i32) < 3 as i32 {
                                let fresh27 = (*SAScheduleListType).SAScheduleTuple.len();
                                (*SAScheduleListType).SAScheduleTuple.len() =
                                    ((*SAScheduleListType).SAScheduleTuple.len())
                                        .wrapping_add(1);
                                if let Some(tuple) = (*SAScheduleListType)
                                    .SAScheduleTuple
                                    .array
                                    .get_mut(fresh27 as usize)
                                {
                                    decode_iso2_saschedule_tuple(stream, tuple)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            if ((*SAScheduleListType).SAScheduleTuple.len() as i32) < 3 as i32 {
                                grammar_id = 152 as i32;
                            } else {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_charge_service(
    stream: &mut ExiBitstream,
    mut ChargeServiceType: &mut Iso2ChargeServiceType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 153 as i32;
    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            153 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_uint16(stream, &mut (*ChargeServiceType).ServiceID)?;
                            grammar_id = 154 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            154 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                            return Err(ExiError::StringValuesNotSupported);
                                        }
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 155 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ChargeServiceType).service_category =
                                            value as Iso2ServiceCategoryType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 156 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            155 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ChargeServiceType).service_category =
                                            value_0 as Iso2ServiceCategoryType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 156 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            156 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*ChargeServiceType).ServiceScope.unwrap().len
                                            as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*ChargeServiceType).ServiceScope.unwrap().len >= 2 {
                                            (*ChargeServiceType).ServiceScope.unwrap().len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*ChargeServiceType).ServiceScope.unwrap().len
                                                    as usize,
                                                &mut (*ChargeServiceType)
                                                    .ServiceScope
                                                    .unwrap()
                                                    .data,
                                                65,
                                            )?;
                                        } else {
                                            return Err(ExiError::StringValuesNotSupported);
                                        }
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 157 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 158 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            157 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 158 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            158 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_supported_energy_transfer_mode(
                                stream,
                                &mut (*ChargeServiceType).SupportedEnergyTransferMode,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_ev_power_delivery_parameter(
    stream: &mut ExiBitstream,
    mut _EVPowerDeliveryParameterType: &mut Iso2EVPowerDeliveryParameterType,
) -> Result<(), ExiError> {
    let mut event_code: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code != 0 as i32 as u32 {
        return Err(ExiError::UnknownEventCode);
    }
    return Ok(());
}
pub fn decode_iso2_dc_ev_power_delivery_parameter(
    stream: &mut ExiBitstream,
    mut DC_EVPowerDeliveryParameterType: &mut Iso2DCEVPowerDeliveryParameterType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 159 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            159 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_DC_EVStatus(
                                stream,
                                &mut (*DC_EVPowerDeliveryParameterType).dc_ev_status,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 160 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            160 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*DC_EVPowerDeliveryParameterType).BulkChargingComplete =
                                            Some(value as i32);
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 161 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            161 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_contract_signature_encrypted_private_key(
    stream: &mut ExiBitstream,
    mut ContractSignatureEncryptedPrivateKeyType: &mut Iso2ContractSignatureEncryptedPrivateKeyType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 162 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            162 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*ContractSignatureEncryptedPrivateKeyType).id.len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*ContractSignatureEncryptedPrivateKeyType).id.len >= 2 {
                                    (*ContractSignatureEncryptedPrivateKeyType).id.len -= 2;

                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*ContractSignatureEncryptedPrivateKeyType).id.len as usize,
                                        &mut (*ContractSignatureEncryptedPrivateKeyType).id.data,
                                        65,
                                    )?;
                                } else {
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 163 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            163 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*ContractSignatureEncryptedPrivateKeyType).CONTENT.len
                                    as u16),
                            )?;
                            if error == 0 as i32 {
                                exi_basetypes_decoder_bytes(
                                    stream,
                                    (*ContractSignatureEncryptedPrivateKeyType).CONTENT.len
                                        as usize,
                                    &mut (*ContractSignatureEncryptedPrivateKeyType).CONTENT.data,
                                )?;
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_evse_charge_parameter(
    stream: &mut ExiBitstream,
    mut _EVSEChargeParameterType: &mut Iso2EVSEChargeParameterType,
) -> Result<(), ExiError> {
    let mut event_code: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code != 0 as i32 as u32 {
        return Err(ExiError::UnknownEventCode);
    }
    return Ok(());
}
pub fn decode_iso2_dc_evse_charge_parameter(
    stream: &mut ExiBitstream,
    mut DC_EVSEChargeParameterType: &mut Iso2DCEVSEChargeParameterType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 164 as i32;
    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            164 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_DC_EVSEStatus(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).DC_EVSEStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 165 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            165 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).EVSEMaximumCurrentLimit,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 166 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            166 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).EVSEMaximumPowerLimit,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 167 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            167 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).EVSEMaximumVoltageLimit,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 168 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            168 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).EVSEMinimumCurrentLimit,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 169 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            169 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).EVSEMinimumVoltageLimit,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 170 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            170 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*DC_EVSEChargeParameterType)
                                    .EVSECurrentRegulationTolerance
                                    .unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 171 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).EVSEPeakCurrentRipple,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 172 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            171 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).EVSEPeakCurrentRipple,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 172 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            172 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*DC_EVSEChargeParameterType)
                                    .EVSEEnergyToBeDelivered
                                    .unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_service_list(
    stream: &mut ExiBitstream,
    mut ServiceListType: &mut Iso2ServiceListType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 173 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            173 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*ServiceListType).Service.len() as i32) < 8 as i32 {
                                let fresh28 = (*ServiceListType).Service.len();
                                (*ServiceListType).Service.len() =
                                    ((*ServiceListType).Service.len()).wrapping_add(1);
                                // Use safe borrows instead of pointer arithmetic
                                if let Some(service) =
                                    (*ServiceListType).Service.get_mut(fresh28 as usize)
                                {
                                    decode_iso2_Service(stream, service)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            grammar_id = 174 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            174 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if ((*ServiceListType).Service.len() as i32) < 8 as i32 {
                                let fresh29 = (*ServiceListType).Service.len();
                                (*ServiceListType).Service.len() =
                                    ((*ServiceListType).Service.len()).wrapping_add(1);
                                // Use safe borrows instead of pointer arithmetic
                                if let Some(service) =
                                    (*ServiceListType).Service.get_mut(fresh29 as usize)
                                {
                                    decode_iso2_Service(stream, service)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            if ((*ServiceListType).Service.len() as i32) < 8 as i32 {
                                grammar_id = 174 as i32;
                            } else {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_diffie_hellman_publickey(
    stream: &mut ExiBitstream,
    mut DiffieHellmanPublickeyType: &mut Iso2DiffieHellmanPublickeyType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 175 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            175 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*DiffieHellmanPublickeyType).id.len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*DiffieHellmanPublickeyType).id.len >= 2 {
                                    (*DiffieHellmanPublickeyType).id.len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*DiffieHellmanPublickeyType).id.len as usize,
                                        &mut (*DiffieHellmanPublickeyType).id.data,
                                        65,
                                    )?;
                                } else {
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 176 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            176 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*DiffieHellmanPublickeyType).CONTENT.len as u16),
                            )?;
                            exi_basetypes_decoder_bytes(
                                stream,
                                DiffieHellmanPublickeyType.CONTENT.len,
                                &mut DiffieHellmanPublickeyType.CONTENT.data,
                            )?;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_emaid(
    stream: &mut ExiBitstream,
    mut EMAIDType: &mut Iso2EMAIDType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 177 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            177 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*EMAIDType).id.len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*EMAIDType).id.len >= 2 {
                                    (*EMAIDType).id.len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*EMAIDType).id.len as usize,
                                        &mut (*EMAIDType).id.data,
                                        65,
                                    )?;
                                } else {
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 178 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            178 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
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
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_ac_evse_status(
    stream: &mut ExiBitstream,
    mut AC_EVSEStatusType: &mut Iso2ACEVSEStatusType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 179 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            179 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
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
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            180 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 181 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            181 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_evse_status(
    stream: &mut ExiBitstream,
    mut EVSEStatusType: &mut Iso2EVSEStatusType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 182 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            182 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
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
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            183 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 184 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            184 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_ac_evsestatus(
                                stream,
                                &mut (*EVSEStatusType).AC_EVSEStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 185 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            185 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_DC_EVSEStatus(
                                stream,
                                &mut (*EVSEStatusType).DC_EVSEStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_ac_evse_charge_parameter(
    stream: &mut ExiBitstream,
    mut AC_EVSEChargeParameterType: &mut Iso2ACEVSEChargeParameterType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 186 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            186 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_ac_evsestatus(
                                stream,
                                &mut (*AC_EVSEChargeParameterType).AC_EVSEStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 187 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            187 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*AC_EVSEChargeParameterType).EVSENominalVoltage,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 188 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            188 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*AC_EVSEChargeParameterType).EVSEMaxCurrent,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_meter_info(
    stream: &mut ExiBitstream,
    mut MeterInfoType: &mut Iso2MeterInfoType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 189 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            189 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                            return Err(ExiError::StringValuesNotSupported);
                                        }
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 190 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            190 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_uint64(
                                stream,
                                &mut (*MeterInfoType).MeterReading.unwrap(),
                            )?;
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
                            decode_exi_type_integer64(
                                stream,
                                &mut (*MeterInfoType).TMeter.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        4 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            191 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
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
                            decode_exi_type_integer64(
                                stream,
                                &mut (*MeterInfoType).TMeter.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        3 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            192 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
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
                            decode_exi_type_integer64(
                                stream,
                                &mut (*MeterInfoType).TMeter.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            193 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_integer64(
                                stream,
                                &mut (*MeterInfoType).TMeter.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_message_header(
    stream: &mut ExiBitstream,
    mut MessageHeaderType: &mut Iso2MessageHeaderType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 194 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            194 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*MessageHeaderType).session_id.len,
                                &mut (*MessageHeaderType).session_id.data,
                                8 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 195 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            195 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            // If notification is None skip
                            if (*MessageHeaderType).Notification.is_some() {
                                decode_iso2_notification(
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
                                decode_iso2_signature(
                                    stream,
                                    &mut (*MessageHeaderType).Signature.unwrap(),
                                )?;
                            }
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            196 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            if (*MessageHeaderType).Signature.is_some() {
                                decode_iso2_signature(
                                    stream,
                                    &mut (*MessageHeaderType).Signature.unwrap(),
                                )?;
                            }
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_power_delivery_req(
    stream: &mut ExiBitstream,
    mut PowerDeliveryReqType: &mut Iso2PowerDeliveryReqType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 197 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            197 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 198 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            198 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 199 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            199 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_charging_profile(
                                stream,
                                &mut (*PowerDeliveryReqType).ChargingProfile.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 200 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_DC_EVPowerDeliveryParameter(
                                stream,
                                &mut (*PowerDeliveryReqType).DC_EVPowerDeliveryParameter.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            decode_iso2_evpower_delivery_parameter(
                                stream,
                                &mut (*PowerDeliveryReqType).EVPowerDeliveryParameter.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        3 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            200 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_DC_EVPowerDeliveryParameter(
                                stream,
                                &mut (*PowerDeliveryReqType).DC_EVPowerDeliveryParameter.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_evpower_delivery_parameter(
                                stream,
                                &mut (*PowerDeliveryReqType).EVPowerDeliveryParameter.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_current_demand_res(
    stream: &mut ExiBitstream,
    mut CurrentDemandResType: &mut Iso2CurrentDemandResType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 201 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            201 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*CurrentDemandResType).response_code =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 202 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            202 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_DC_EVSEStatus(
                                stream,
                                &mut (*CurrentDemandResType).DC_EVSEStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 203 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            203 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*CurrentDemandResType).EVSEPresentVoltage,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 204 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            204 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*CurrentDemandResType).EVSEPresentCurrent,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 205 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            205 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 206 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            206 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 207 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            207 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 208 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            208 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*CurrentDemandResType).EVSEMaximumVoltageLimit.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 209 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*CurrentDemandResType).EVSEMaximumCurrentLimit.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 210 as i32;
                            }
                        }
                        2 => {
                            decode_iso2_PhysicalValue(
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
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*CurrentDemandResType).EVSEID.len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*CurrentDemandResType).EVSEID.len as i32 >= 2 as i32 {
                                            (*CurrentDemandResType).EVSEID.len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*CurrentDemandResType).EVSEID.len as usize,
                                                &mut (*CurrentDemandResType).EVSEID.data,
                                                38,
                                            )?;
                                        } else {
                                            return Err(ExiError::StringValuesNotSupported);
                                        }
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 212 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            209 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*CurrentDemandResType).EVSEMaximumCurrentLimit.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 210 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_PhysicalValue(
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
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*CurrentDemandResType).EVSEID.len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*CurrentDemandResType).EVSEID.len as i32 >= 2 as i32 {
                                            (*CurrentDemandResType).EVSEID.len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*CurrentDemandResType).EVSEID.len as usize,
                                                &mut (*CurrentDemandResType).EVSEID.data,
                                                38,
                                            )?;
                                        } else {
                                            return Err(ExiError::StringValuesNotSupported);
                                        }
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 212 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            210 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
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
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*CurrentDemandResType).EVSEID.len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*CurrentDemandResType).EVSEID.len as i32 >= 2 as i32 {
                                            (*CurrentDemandResType).EVSEID.len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*CurrentDemandResType).EVSEID.len as usize,
                                                &mut (*CurrentDemandResType).EVSEID.data,
                                                38,
                                            )?;
                                        } else {
                                            return Err(ExiError::StringValuesNotSupported);
                                        }
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 212 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            211 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*CurrentDemandResType).EVSEID.len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*CurrentDemandResType).EVSEID.len as i32 >= 2 as i32 {
                                            (*CurrentDemandResType).EVSEID.len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*CurrentDemandResType).EVSEID.len,
                                                &mut (*CurrentDemandResType).EVSEID.data,
                                                38,
                                            )?;
                                        } else {
                                            return Err(ExiError::StringValuesNotSupported);
                                        }
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 212 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            212 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 213 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            213 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_meter_info(
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
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value_4: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_4,
                                    )?;
                                    if error == 0 as i32 {
                                        (*CurrentDemandResType).ReceiptRequired =
                                            Some(value_4 as i32);
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        2 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            214 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value_5: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_5,
                                    )?;
                                    if error == 0 as i32 {
                                        (*CurrentDemandResType).ReceiptRequired =
                                            Some(value_5 as i32);
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_charging_status_res(
    stream: &mut ExiBitstream,
    mut ChargingStatusResType: &mut Iso2ChargingStatusResType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 215 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            215 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ChargingStatusResType).response_code =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 216 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            216 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*ChargingStatusResType).EVSEID.len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*ChargingStatusResType).EVSEID.len as i32 >= 2 as i32 {
                                            (*ChargingStatusResType).EVSEID.len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*ChargingStatusResType).EVSEID.len,
                                                &mut (*ChargingStatusResType).EVSEID.data,
                                                38,
                                            )?;
                                        } else {
                                            return Err(ExiError::StringValuesNotSupported);
                                        }
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 217 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            217 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 218 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            218 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*ChargingStatusResType).EVSEMaxCurrent.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 219 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_meter_info(
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
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value_1: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_1,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ChargingStatusResType).ReceiptRequired =
                                            Some(value_1 as i32);
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 221 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        3 => {
                            decode_iso2_ac_evsestatus(
                                stream,
                                &mut (*ChargingStatusResType).AC_EVSEStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            219 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_meter_info(
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
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value_2: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_2,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ChargingStatusResType).ReceiptRequired =
                                            Some(value_2 as i32);
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 221 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        2 => {
                            decode_iso2_ac_evsestatus(
                                stream,
                                &mut (*ChargingStatusResType).AC_EVSEStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            220 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value_3: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_3,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ChargingStatusResType).ReceiptRequired =
                                            Some(value_3 as i32);
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 221 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        1 => {
                            decode_iso2_ac_evsestatus(
                                stream,
                                &mut (*ChargingStatusResType).AC_EVSEStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            221 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_ac_evsestatus(
                                stream,
                                &mut (*ChargingStatusResType).AC_EVSEStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_authorization_req(
    stream: &mut ExiBitstream,
    mut AuthorizationReqType: &mut Iso2AuthorizationReqType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 222 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            222 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*AuthorizationReqType).id.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*AuthorizationReqType).id.unwrap().len as i32 >= 2 as i32 {
                                    (*AuthorizationReqType).id.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*AuthorizationReqType).id.unwrap().len,
                                        &mut (*AuthorizationReqType).id.unwrap().data,
                                        65,
                                    )?;
                                } else {
                                    return Err(ExiError::StringValuesNotSupported);
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
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            223 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
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
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_pre_charge_req(
    stream: &mut ExiBitstream,
    mut PreChargeReqType: &mut Iso2PreChargeReqType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 224 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            224 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_DC_EVStatus(
                                stream,
                                &mut (*PreChargeReqType).dc_ev_status.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 225 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            225 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*PreChargeReqType).EVTargetVoltage.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 226 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            226 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*PreChargeReqType).EVTargetCurrent.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_service_detail_res(
    stream: &mut ExiBitstream,
    mut ServiceDetailResType: &mut Iso2ServiceDetailResType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 227 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            227 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ServiceDetailResType).response_code =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 228 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            228 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_uint16(stream, &mut (*ServiceDetailResType).ServiceID)?;
                            if error == 0 as i32 {
                                grammar_id = 229 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            229 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_service_parameter_list(
                                stream,
                                &mut (*ServiceDetailResType).ServiceParameterList.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_payment_service_selection_res(
    stream: &mut ExiBitstream,
    mut PaymentServiceSelectionResType: &mut Iso2PaymentServiceSelectionResType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 230 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            230 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*PaymentServiceSelectionResType).response_code =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_certificate_update_req(
    stream: &mut ExiBitstream,
    mut CertificateUpdateReqType: &mut Iso2CertificateUpdateReqType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 231 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            231 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*CertificateUpdateReqType).id.len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*CertificateUpdateReqType).id.len as i32 >= 2 as i32 {
                                    (*CertificateUpdateReqType).id.len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*CertificateUpdateReqType).id.len as usize,
                                        &mut (*CertificateUpdateReqType).id.data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 232 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            232 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_certificate_chain(
                                stream,
                                &mut (*CertificateUpdateReqType).contract_signature_cert_chain,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 233 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            233 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*CertificateUpdateReqType).eMaid.len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*CertificateUpdateReqType).eMaid.len as i32 >= 2 as i32
                                        {
                                            (*CertificateUpdateReqType).eMaid.len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*CertificateUpdateReqType).eMaid.len,
                                                &mut (*CertificateUpdateReqType).eMaid.data,
                                                (15 as i32 + 1 as i32) as usize,
                                            )?;
                                        } else {
                                            return Err(ExiError::StringValuesNotSupported);
                                        }
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 234 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            234 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_list_of_root_certificate_ids(
                                stream,
                                &mut (*CertificateUpdateReqType).ListOfRootCertificateIDs,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_session_setup_res(
    stream: &mut ExiBitstream,
    mut SessionSetupResType: &mut Iso2SessionSetupResType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 235 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            235 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*SessionSetupResType).response_code =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 236 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            236 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*SessionSetupResType).EVSEID.len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*SessionSetupResType).EVSEID.len as i32 >= 2 as i32 {
                                            (*SessionSetupResType).EVSEID.len -= 2;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*SessionSetupResType).EVSEID.len,
                                                &mut (*SessionSetupResType).EVSEID.data,
                                                38,
                                            )?;
                                        } else {
                                            return Err(ExiError::StringValuesNotSupported);
                                        }
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 237 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            237 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
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
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_certificate_installation_req(
    stream: &mut ExiBitstream,
    mut CertificateInstallationReqType: &mut Iso2CertificateInstallationReqType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 238 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            238 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*CertificateInstallationReqType).id.len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*CertificateInstallationReqType).id.len as i32 >= 2 as i32 {
                                    (*CertificateInstallationReqType).id.len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*CertificateInstallationReqType).id.len as usize,
                                        &mut (*CertificateInstallationReqType).id.data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 239 as i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            239 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*CertificateInstallationReqType).OEMProvisioningCert.len,
                                &mut (*CertificateInstallationReqType).OEMProvisioningCert.data,
                                800 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 240 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            240 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_list_of_root_certificate_ids(
                                stream,
                                &mut (*CertificateInstallationReqType).ListOfRootCertificateIDs,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_certificate_installation_res(
    stream: &mut ExiBitstream,
    mut CertificateInstallationResType: &mut Iso2CertificateInstallationResType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 241 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            241 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*CertificateInstallationResType).response_code =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 242 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            242 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_certificate_chain(
                                stream,
                                &mut (*CertificateInstallationResType)
                                    .SAProvisioningCertificateChain,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 243 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            243 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_certificate_chain(
                                stream,
                                &mut (*CertificateInstallationResType).contract_signature_cert_chain,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 244 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            244 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_contract_signature_encrypted_private_key(
                                stream,
                                &mut (*CertificateInstallationResType)
                                    .ContractSignatureEncryptedPrivateKey,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 245 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            245 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_diffie_hellman_publickey(
                                stream,
                                &mut (*CertificateInstallationResType).DHpublickey,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 246 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            246 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_emaid(
                                stream,
                                &mut (*CertificateInstallationResType).eMaid,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_welding_detection_res(
    stream: &mut ExiBitstream,
    mut WeldingDetectionResType: &mut Iso2WeldingDetectionResType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 247 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            247 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*WeldingDetectionResType).response_code =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 248 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            248 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_DC_EVSEStatus(
                                stream,
                                &mut (*WeldingDetectionResType).DC_EVSEStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 249 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            249 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*WeldingDetectionResType).EVSEPresentVoltage,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_current_demand_req(
    stream: &mut ExiBitstream,
    mut CurrentDemandReqType: &mut Iso2CurrentDemandReqType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 250 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            250 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_DC_EVStatus(
                                stream,
                                &mut (*CurrentDemandReqType).dc_ev_status,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 251 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            251 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*CurrentDemandReqType).EVTargetCurrent,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 252 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            252 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*CurrentDemandReqType).EVMaximumVoltageLimit.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 253 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*CurrentDemandReqType).EVMaximumCurrentLimit.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 254 as i32;
                            }
                        }
                        2 => {
                            decode_iso2_PhysicalValue(
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
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*CurrentDemandReqType).BulkChargingComplete =
                                            Some(value as i32);
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 256 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        4 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 257 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            253 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*CurrentDemandReqType).EVMaximumCurrentLimit.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 254 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_PhysicalValue(
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
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 256 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        3 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 257 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            254 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
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
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 256 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        2 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 257 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            255 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value_5: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_5,
                                    )?;
                                    if error == 0 as i32 {
                                        (*CurrentDemandReqType).BulkChargingComplete =
                                            Some(value_5 as i32);
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 256 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 257 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            256 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
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
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 257 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            257 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*CurrentDemandReqType).RemainingTimeToFullSoC.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 258 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*CurrentDemandReqType).RemainingTimeToBulkSoC.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 259 as i32;
                            }
                        }
                        2 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*CurrentDemandReqType).EVTargetVoltage,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            258 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*CurrentDemandReqType).RemainingTimeToBulkSoC.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 259 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*CurrentDemandReqType).EVTargetVoltage,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            259 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*CurrentDemandReqType).EVTargetVoltage,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_pre_charge_res(
    stream: &mut ExiBitstream,
    mut PreChargeResType: &mut Iso2PreChargeResType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 260 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            260 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*PreChargeResType).response_code =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 261 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            261 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_DC_EVSEStatus(
                                stream,
                                &mut (*PreChargeResType).DC_EVSEStatus,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 262 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            262 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_PhysicalValue(
                                stream,
                                &mut (*PreChargeResType).EVSEPresentVoltage,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_certificate_update_res(
    stream: &mut ExiBitstream,
    mut CertificateUpdateResType: &mut Iso2CertificateUpdateResType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 263 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            263 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*CertificateUpdateResType).response_code =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 264 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            264 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_certificate_chain(
                                stream,
                                &mut (*CertificateUpdateResType).sa_provisioning_certificate_chain,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 265 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            265 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_certificate_chain(
                                stream,
                                &mut (*CertificateUpdateResType).contract_signature_cert_chain,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 266 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            266 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_contract_signature_encrypted_private_key(
                                stream,
                                &mut (*CertificateUpdateResType)
                                    .contract_signature_encrypted_private_key,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 267 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            267 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_diffie_hellman_publickey(
                                stream,
                                &mut (*CertificateUpdateResType).dh_public_key,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 268 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            268 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_emaid(stream, &mut (*CertificateUpdateResType).e_maid)?;
                            if error == 0 as i32 {
                                grammar_id = 269 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            269 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_integer16(
                                stream,
                                &mut (*CertificateUpdateResType).retry_counter.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_metering_receipt_req(
    stream: &mut ExiBitstream,
    mut MeteringReceiptReqType: &mut Iso2MeteringReceiptReqType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 270 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            270 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut ((*MeteringReceiptReqType).id.unwrap().len as u16),
                            )?;
                            if error == 0 as i32 {
                                if (*MeteringReceiptReqType).id.unwrap().len as i32 >= 2 as i32 {
                                    (*MeteringReceiptReqType).id.unwrap().len -= 2;
                                    exi_basetypes_decoder_characters(
                                        stream,
                                        (*MeteringReceiptReqType).id.unwrap().len,
                                        &mut (*MeteringReceiptReqType).id.unwrap().data,
                                        (64 as i32 + 1 as i32) as usize,
                                    )?;
                                } else {
                                    return Err(ExiError::StringValuesNotSupported);
                                }
                            }
                            grammar_id = 271 as i32;
                        }
                        1 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*MeteringReceiptReqType).session_id.len,
                                &mut (*MeteringReceiptReqType).session_id.data,
                                8 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 272 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            271 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*MeteringReceiptReqType).session_id.len,
                                &mut (*MeteringReceiptReqType).session_id.data,
                                8 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 272 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            272 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        8 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*MeteringReceiptReqType).sa_schedule_tuple_id =
                                            Some(value.wrapping_add(1 as i32 as u32) as u8);
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 273 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        1 => {
                            decode_iso2_meter_info(
                                stream,
                                &mut (*MeteringReceiptReqType).meter_info,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            273 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_meter_info(
                                stream,
                                &mut (*MeteringReceiptReqType).meter_info,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_charging_status_req(
    stream: &mut ExiBitstream,
    mut _ChargingStatusReqType: &mut Iso2ChargingStatusReqType,
) -> Result<(), ExiError> {
    let mut event_code: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code != 0 {
        return Err(ExiError::UnknownEventCode);
    }

    return Ok(());
}
pub fn decode_iso2_session_stop_res(
    stream: &mut ExiBitstream,
    mut SessionStopResType: &mut Iso2SessionStopResType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 274 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            274 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*SessionStopResType).response_code =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_charge_parameter_discovery_req(
    stream: &mut ExiBitstream,
    mut ChargeParameterDiscoveryReqType: &mut Iso2ChargeParameterDiscoveryReqType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 275 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            275 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_uint16(
                                stream,
                                &mut (*ChargeParameterDiscoveryReqType)
                                    .max_entries_sa_schedule_tuple
                                    .unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 276 as i32;
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        3 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ChargeParameterDiscoveryReqType)
                                            .requested_energy_transfer_mode =
                                            value as Iso2EnergyTransferModeType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 277 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            276 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        3 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ChargeParameterDiscoveryReqType)
                                            .requested_energy_transfer_mode =
                                            value_0 as Iso2EnergyTransferModeType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 277 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            277 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_ac_evcharge_parameter(
                                stream,
                                &mut (*ChargeParameterDiscoveryReqType)
                                    .ac_ev_charge_parameter
                                    .unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_dc_evcharge_parameter(
                                stream,
                                &mut (*ChargeParameterDiscoveryReqType)
                                    .dc_ev_charge_parameter
                                    .unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            decode_iso2_evcharge_parameter(
                                stream,
                                &mut (*ChargeParameterDiscoveryReqType)
                                    .ev_charge_parameter
                                    .unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_cable_check_req(
    stream: &mut ExiBitstream,
    mut CableCheckReqType: &mut Iso2CableCheckReqType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 278 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            278 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_DC_EVStatus(
                                stream,
                                &mut (*CableCheckReqType).dc_ev_status,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_welding_detection_req(
    stream: &mut ExiBitstream,
    mut WeldingDetectionReqType: &mut Iso2WeldingDetectionReqType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 279 as i32;
    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            279 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_dc_ev_status(
                                stream,
                                &mut (*WeldingDetectionReqType).dc_ev_status,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_power_delivery_res(
    stream: &mut ExiBitstream,
    mut power_delivery_res_type: &mut Iso2PowerDeliveryResType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 280 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            280 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*power_delivery_res_type).response_code =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 281 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            281 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_ac_evse_status(
                                stream,
                                &mut (*power_delivery_res_type).ac_evse_status.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_dc_evse_status(
                                stream,
                                &mut (*power_delivery_res_type).dc_evse_status.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            decode_iso2_evse_status(
                                stream,
                                &mut (*power_delivery_res_type).evse_status.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_charge_parameter_discovery_res(
    stream: &mut ExiBitstream,
    mut charge_parameter_discovery_res_type: &mut Iso2ChargeParameterDiscoveryResType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 282 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            282 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*charge_parameter_discovery_res_type).response_code =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 283 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            283 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*charge_parameter_discovery_res_type).evse_processing =
                                            value_0 as Iso2EvseProcessingType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 284 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            284 => {
                exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_sa_schedule_list(
                                stream,
                                &mut (*charge_parameter_discovery_res_type).sa_schedule_list.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 285 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_sa_schedules(
                                stream,
                                &mut (*charge_parameter_discovery_res_type).sa_schedules.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 285 as i32;
                            }
                        }
                        2 => {
                            decode_iso2_ac_evse_charge_parameter(
                                stream,
                                &mut (*charge_parameter_discovery_res_type)
                                    .ac_evse_charge_parameter
                                    .unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        3 => {
                            decode_iso2_dc_evse_charge_parameter(
                                stream,
                                &mut (*charge_parameter_discovery_res_type)
                                    .dc_evse_charge_parameter
                                    .unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        4 => {
                            decode_iso2_evse_charge_parameter(
                                stream,
                                &mut (*charge_parameter_discovery_res_type)
                                    .evse_charge_parameter
                                    .unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            285 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_ac_evse_charge_parameter(
                                stream,
                                &mut (*charge_parameter_discovery_res_type)
                                    .ac_evse_charge_parameter
                                    .unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_dc_evse_charge_parameter(
                                stream,
                                &mut (*charge_parameter_discovery_res_type)
                                    .dc_evse_charge_parameter
                                    .unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            decode_iso2_evse_charge_parameter(
                                stream,
                                &mut (*charge_parameter_discovery_res_type)
                                    .evse_charge_parameter
                                    .unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_payment_service_selection_req(
    stream: &mut ExiBitstream,
    mut payment_service_selection_req_type: &mut Iso2PaymentServiceSelectionReqType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 286 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            286 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*payment_service_selection_req_type).selected_payment_option =
                                            value as Iso2PaymentOptionType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 287 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            287 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_selected_service_list(
                                stream,
                                &mut (*payment_service_selection_req_type).selected_service_list,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_metering_receipt_res(
    stream: &mut ExiBitstream,
    mut metering_receipt_res_type: &mut Iso2MeteringReceiptResType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 288 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            288 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*metering_receipt_res_type).response_code =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 289 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            289 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_ac_evse_status(
                                stream,
                                &mut (*metering_receipt_res_type).ac_evse_status.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            decode_iso2_dc_evse_status(
                                stream,
                                &mut (*metering_receipt_res_type).dc_evse_status.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            decode_iso2_evse_status(
                                stream,
                                &mut (*metering_receipt_res_type).evse_status.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_cable_check_res(
    stream: &mut ExiBitstream,
    mut cable_check_res_type: &mut Iso2CableCheckResType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 290 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            290 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*cable_check_res_type).response_code =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 291 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            291 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_dc_evse_status(
                                stream,
                                &mut (*cable_check_res_type).dc_evse_status.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 292 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            292 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*cable_check_res_type).evse_processing =
                                            value_0 as Iso2EvseProcessingType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_service_discovery_res(
    stream: &mut ExiBitstream,
    mut service_discovery_res_type: &mut Iso2ServiceDiscoveryResType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 293 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            293 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*service_discovery_res_type).response_code =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 294 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            294 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_payment_option_list(
                                stream,
                                &mut (*service_discovery_res_type).payment_option_list,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 295 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            295 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_charge_service(
                                stream,
                                &mut (*service_discovery_res_type).charge_service,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 296 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            296 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_service_list(
                                stream,
                                &mut (*service_discovery_res_type).service_list.unwrap(),
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_service_detail_req(
    stream: &mut ExiBitstream,
    mut ServiceDetailReqType: &mut Iso2ServiceDetailReqType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 297 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            297 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_uint16(stream, &mut (*ServiceDetailReqType).ServiceID)?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_session_setup_req(
    stream: &mut ExiBitstream,
    mut session_setup_req_type: &mut Iso2SessionSetupReqType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 298 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            298 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*SessionSetupReqType).evcc_id.len,
                                &mut (*SessionSetupReqType).evcc_id.data,
                                6 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_session_stop_req(
    stream: &mut ExiBitstream,
    mut SessionStopReqType: &mut Iso2SessionStopReqType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 299 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            299 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*SessionStopReqType).charging_session =
                                            value as Iso2ChargingSessionType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_service_discovery_req(
    stream: &mut ExiBitstream,
    mut ServiceDiscoveryReqType: &mut Iso2ServiceDiscoveryReqType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 300 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            300 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*ServiceDiscoveryReqType).service_scope.unwrap().len
                                            as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*ServiceDiscoveryReqType).service_scope.unwrap().len
                                            as i32
                                            >= 2 as i32
                                        {
                                            (*ServiceDiscoveryReqType).service_scope.unwrap().len =
                                                ((*ServiceDiscoveryReqType)
                                                    .service_scope
                                                    .unwrap()
                                                    .len
                                                    as i32
                                                    - 2 as i32)
                                                    as usize;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*ServiceDiscoveryReqType)
                                                    .service_scope
                                                    .unwrap()
                                                    .len,
                                                &mut ((*ServiceDiscoveryReqType)
                                                    .service_scope
                                                    .unwrap()
                                                    .data),
                                                (64 as i32 + 1 as i32) as usize,
                                            )?;
                                        } else {
                                            return Err(ExiError::StringvaluesNotSupported);
                                        }
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 301 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ServiceDiscoveryReqType).service_category =
                                            Some(value as Iso2ServiceCategoryType);
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        2 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            301 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*ServiceDiscoveryReqType).service_category =
                                            Some(value_0 as Iso2ServiceCategoryType);
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        1 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_authorization_res(
    stream: &mut ExiBitstream,
    mut AuthorizationResType: &mut Iso2AuthorizationResType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 302 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            302 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*AuthorizationResType).response_code =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 303 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            303 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value_0,
                                    )?;
                                    if error == 0 as i32 {
                                        (*AuthorizationResType).evse_processing =
                                            value_0 as Iso2EvseProcessingType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 3 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_payment_details_req(
    stream: &mut ExiBitstream,
    mut PaymentDetailsReqType: &mut Iso2PaymentDetailsReqType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 304 as i32;

    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            304 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut ((*PaymentDetailsReqType).e_maid.len as u16),
                                    )?;
                                    if error == 0 as i32 {
                                        if (*PaymentDetailsReqType).e_maid.len as i32 >= 2 as i32 {
                                            (*PaymentDetailsReqType).e_maid.len =
                                                ((*PaymentDetailsReqType).e_maid.len as i32
                                                    - 2 as i32)
                                                    as usize;
                                            exi_basetypes_decoder_characters(
                                                stream,
                                                (*PaymentDetailsReqType).e_maid.len,
                                                &mut (*PaymentDetailsReqType).e_maid.data,
                                                (15 as i32 + 1 as i32) as usize,
                                            )?;
                                        } else {
                                            return Err(ExiError::StringValuesNotSupported);
                                        }
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 305 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            305 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_certificate_chain(
                                stream,
                                &mut (*PaymentDetailsReqType).contract_signature_cert_chain,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}
pub fn decode_iso2_payment_details_res(
    stream: &mut ExiBitstream,
    mut payment_details_res_type: &mut Iso2PaymentDetailsResType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 306 as i32;
    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            306 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0 as i32 {
                                if event_code == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    )?;
                                    if error == 0 as i32 {
                                        (*payment_details_res_type).response_code =
                                            value as Iso2ResponseCodeType;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0 as i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0 as i32 {
                                    if event_code == 0 as i32 as u32 {
                                        grammar_id = 307 as i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            307 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_hex_binary(
                                stream,
                                &mut (*payment_details_res_type).gen_challenge.len,
                                &mut (*payment_details_res_type).gen_challenge.data,
                                16 as i32 as usize,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 308 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            308 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_exi_type_integer64(
                                stream,
                                &mut (*payment_details_res_type).evse_time_stamp,
                            )?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}

pub fn decode_iso2_body(
    stream: &mut ExiBitstream,
    mut body: &mut Iso2BodyType,
) -> Result<(), ExiError> {
    let mut event_code: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 6 as i32 as usize, &mut event_code)?;
    match event_code {
        0 => {
            let mut authorization_req: Iso2AuthorizationReqType = Default::default();
            decode_iso2_authorization_req(stream, &mut authorization_req)?;
            body.body_type_component = Iso2BodyTypeEnum::AuthorizationReq(authorization_req);
        }
        1 => {
            let mut authorization_res: Iso2AuthorizationResType = Default::default();
            decode_iso2_authorization_res(stream, &mut authorization_res)?;
            body.body_type_component = Iso2BodyTypeEnum::AuthorizationRes(authorization_res);
        }
        2 => {
            let mut body_element: Iso2BodyBaseType = Default::default();
            decode_iso2_body_base(stream, &mut body_element)?;
            body.body_type_component = Iso2BodyTypeEnum::BodyElement(body_element);
        }
        3 => {
            let mut cable_check_req: Iso2CableCheckReqType = Default::default();
            decode_iso2_cable_check_req(stream, &mut cable_check_req)?;
            body.body_type_component = Iso2BodyTypeEnum::CableCheckReq(cable_check_req);
        }
        4 => {
            let mut cable_check_res: Iso2CableCheckResType = Default::default();
            decode_iso2_cable_check_res(stream, &mut cable_check_res)?;
            body.body_type_component = Iso2BodyTypeEnum::CableCheckRes(cable_check_res);
        }
        5 => {
            let mut certificate_installation_req: Iso2CertificateInstallationReqType =
                Default::default();
            decode_iso2_certificate_installation_req(stream, &mut certificate_installation_req)?;
            body.body_type_component =
                Iso2BodyTypeEnum::CertificateInstallationReq(certificate_installation_req);
        }
        6 => {
            let mut certificate_installation_res: Iso2CertificateInstallationResType =
                Default::default();
            decode_iso2_certificate_installation_res(stream, &mut certificate_installation_res)?;
            body.body_type_component =
                Iso2BodyTypeEnum::CertificateInstallationRes(certificate_installation_res);
        }
        7 => {
            let mut certificate_update_req: Iso2CertificateUpdateReqType = Default::default();
            decode_iso2_certificate_update_req(stream, &mut certificate_update_req)?;
            body.body_type_component =
                Iso2BodyTypeEnum::CertificateUpdateReq(certificate_update_req);
        }
        8 => {
            let mut certificate_update_res: Iso2CertificateUpdateResType = Default::default();
            decode_iso2_certificate_update_res(stream, &mut certificate_update_res)?;
            body.body_type_component =
                Iso2BodyTypeEnum::CertificateUpdateRes(certificate_update_res);
        }
        9 => {
            let mut charge_parameter_discovery_req: Iso2ChargeParameterDiscoveryReqType =
                Default::default();
            decode_iso2_charge_parameter_discovery_req(
                stream,
                &mut charge_parameter_discovery_req,
            )?;
            body.body_type_component =
                Iso2BodyTypeEnum::ChargeParameterDiscoveryReq(charge_parameter_discovery_req);
        }
        10 => {
            let mut charge_parameter_discovery_res: Iso2ChargeParameterDiscoveryResType =
                Default::default();
            decode_iso2_charge_parameter_discovery_res(
                stream,
                &mut charge_parameter_discovery_res,
            )?;
            body.body_type_component =
                Iso2BodyTypeEnum::ChargeParameterDiscoveryRes(charge_parameter_discovery_res);
        }
        11 => {
            let mut charging_status_req: Iso2ChargingStatusReqType = Default::default();
            decode_iso2_charging_status_req(stream, &mut charging_status_req)?;
            body.body_type_component = Iso2BodyTypeEnum::ChargingStatusReq(charging_status_req);
        }
        12 => {
            let mut charging_status_res: Iso2ChargingStatusResType = Default::default();
            decode_iso2_charging_status_res(stream, &mut charging_status_res)?;
            body.body_type_component = Iso2BodyTypeEnum::ChargingStatusRes(charging_status_res);
        }
        13 => {
            let mut current_demand_req: Iso2CurrentDemandReqType = Default::default();
            decode_iso2_current_demand_req(stream, &mut current_demand_req)?;
            body.body_type_component = Iso2BodyTypeEnum::CurrentDemandReq(current_demand_req);
        }
        14 => {
            let mut current_demand_res: Iso2CurrentDemandResType = Default::default();
            decode_iso2_current_demand_res(stream, &mut current_demand_res)?;
            body.body_type_component = Iso2BodyTypeEnum::CurrentDemandRes(current_demand_res);
        }
        15 => {
            let mut metering_receipt_req: Iso2MeteringReceiptReqType = Default::default();
            decode_iso2_metering_receipt_req(stream, &mut metering_receipt_req)?;
            body.body_type_component = Iso2BodyTypeEnum::MeteringReceiptReq(metering_receipt_req);
        }
        16 => {
            let mut metering_receipt_res: Iso2MeteringReceiptResType = Default::default();
            decode_iso2_metering_receipt_res(stream, &mut metering_receipt_res)?;
            body.body_type_component = Iso2BodyTypeEnum::MeteringReceiptRes(metering_receipt_res);
        }
        17 => {
            let mut payment_details_req: Iso2PaymentDetailsReqType = Default::default();
            decode_iso2_payment_details_req(stream, &mut payment_details_req)?;
            body.body_type_component = Iso2BodyTypeEnum::PaymentDetailsReq(payment_details_req);
        }
        18 => {
            let mut payment_details_res: Iso2PaymentDetailsResType = Default::default();
            decode_iso2_payment_details_res(stream, &mut payment_details_res)?;
            body.body_type_component = Iso2BodyTypeEnum::PaymentDetailsRes(payment_details_res);
        }
        19 => {
            let mut payment_service_selection_req: Iso2PaymentServiceSelectionReqType =
                Default::default();
            decode_iso2_payment_service_selection_req(stream, &mut payment_service_selection_req)?;
            body.body_type_component =
                Iso2BodyTypeEnum::PaymentServiceSelectionReq(payment_service_selection_req);
        }
        20 => {
            let mut payment_service_selection_res: Iso2PaymentServiceSelectionResType =
                Default::default();
            decode_iso2_payment_service_selection_res(stream, &mut payment_service_selection_res)?;
            body.body_type_component =
                Iso2BodyTypeEnum::PaymentServiceSelectionRes(payment_service_selection_res);
        }
        21 => {
            let mut power_delivery_req: Iso2PowerDeliveryReqType = Default::default();
            decode_iso2_power_delivery_req(stream, &mut power_delivery_req)?;
            body.body_type_component = Iso2BodyTypeEnum::PowerDeliveryReq(power_delivery_req);
        }
        22 => {
            let mut power_delivery_res: Iso2PowerDeliveryResType = Default::default();
            decode_iso2_power_delivery_res(stream, &mut power_delivery_res)?;
            body.body_type_component = Iso2BodyTypeEnum::PowerDeliveryRes(power_delivery_res);
        }
        23 => {
            let mut pre_charge_req: Iso2PreChargeReqType = Default::default();
            decode_iso2_pre_charge_req(stream, &mut pre_charge_req)?;
            body.body_type_component = Iso2BodyTypeEnum::PreChargeReq(pre_charge_req);
        }
        24 => {
            let mut pre_charge_res: Iso2PreChargeResType = Default::default();
            decode_iso2_pre_charge_res(stream, &mut pre_charge_res)?;
            body.body_type_component = Iso2BodyTypeEnum::PreChargeRes(pre_charge_res);
        }
        25 => {
            let mut service_detail_req: Iso2ServiceDetailReqType = Default::default();
            decode_iso2_service_detail_req(stream, &mut service_detail_req)?;
            body.body_type_component = Iso2BodyTypeEnum::ServiceDetailReq(service_detail_req);
        }
        26 => {
            let mut service_detail_res: Iso2ServiceDetailResType = Default::default();
            decode_iso2_service_detail_res(stream, &mut service_detail_res)?;
            body.body_type_component = Iso2BodyTypeEnum::ServiceDetailRes(service_detail_res);
        }
        27 => {
            let mut service_discovery_req: Iso2ServiceDiscoveryReqType = Default::default();
            decode_iso2_service_discovery_req(stream, &mut service_discovery_req)?;
            body.body_type_component =
                Iso2BodyTypeEnum::ServiceDiscoveryReq(service_discovery_req);
        }
        28 => {
            let mut service_discovery_res: Iso2ServiceDiscoveryResType = Default::default();
            decode_iso2_service_discovery_res(stream, &mut service_discovery_res)?;
            body.body_type_component =
                Iso2BodyTypeEnum::ServiceDiscoveryRes(service_discovery_res);
        }
        29 => {
            let mut session_setup_req: Iso2SessionSetupReqType = Default::default();
            decode_iso2_session_setup_req(stream, &mut session_setup_req)?;
            body.body_type_component = Iso2BodyTypeEnum::SessionSetupReq(session_setup_req);
        }
        30 => {
            let mut session_setup_res: Iso2SessionSetupResType = Default::default();
            decode_iso2_session_setup_res(stream, &mut session_setup_res)?;
            body.body_type_component = Iso2BodyTypeEnum::SessionSetupRes(session_setup_res);
        }
        31 => {
            let mut session_stop_req: Iso2SessionStopReqType = Default::default();
            decode_iso2_session_stop_req(stream, &mut session_stop_req)?;
            body.body_type_component = Iso2BodyTypeEnum::SessionStopReq(session_stop_req);
        }
        32 => {
            let mut session_stop_res: Iso2SessionStopResType = Default::default();
            decode_iso2_session_stop_res(stream, &mut session_stop_res)?;
            body.body_type_component = Iso2BodyTypeEnum::SessionStopRes(session_stop_res);
        }
        33 => {
            let mut welding_detection_req: Iso2WeldingDetectionReqType = Default::default();
            decode_iso2_welding_detection_req(stream, &mut welding_detection_req)?;
            body.body_type_component =
                Iso2BodyTypeEnum::WeldingDetectionReq(welding_detection_req);
        }
        34 => {
            let mut welding_detection_res: Iso2WeldingDetectionResType = Default::default();
            decode_iso2_welding_detection_res(stream, &mut welding_detection_res)?;
            body.body_type_component =
                Iso2BodyTypeEnum::WeldingDetectionRes(welding_detection_res);
        }
        _ => {
            return Err(ExiError::UnknownEventCode);
        }
    }
    let mut event_code: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    match event_code {
        0 => Ok(()),
        _ => Err(ExiError::UnknownEventCode),
    }
}

pub fn decode_iso2_v2g_message(
    stream: &mut ExiBitstream,
    mut v2g_msg: &mut Iso2v2gMessage,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 310 as i32;
    let mut event_code: u32 = 0;
    let mut error: i32 = 0;
    loop {
        match grammar_id {
            310 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_message_header(stream, &mut (*v2g_msg).header)?;
                            if error == 0 as i32 {
                                grammar_id = 311 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            311 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            decode_iso2_body(stream, &mut (*v2g_msg).body)?;
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0 as i32 {
                    match event_code {
                        0 => {
                            return Ok(());
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}

pub fn decode_iso2_exi_document(
    stream: &mut ExiBitstream,
    mut exi_doc: &mut Iso2ExiDocument,
) -> Result<(), ExiError> {
    let mut event_code: u32 = 0;
    stream.read_and_check_header()?;
    exi_basetypes_decoder_nbit_uint(stream, 7 as i32 as usize, &mut event_code)?;
    match event_code {
        0 | 76 => {
            decode_iso2_v2g_message(stream, &mut (*exi_doc).v2g_message)?;
            return Ok(());
        }
        _ => {
            return Err(ExiError::UnsupportedSubEvent);
        }
    }
}

pub fn decode_iso2_exi_fragment(
    stream: &mut ExiBitstream,
    mut _exi_frag: &mut Iso2ExiFragment,
) -> Result<(), ExiError> {
    let mut event_code: u32 = 0;
    stream.read_and_check_header()?;
    exi_basetypes_decoder_nbit_uint(stream, 8 as i32 as usize, &mut event_code)?;
    return Err(ExiError::NotImplementedYet);
    // match event_code {
    //     0 => {}
    //     1 => {}
    //     2 => {}
    //     3 => {}
    //     4 => {
    //         decode_iso2_AuthorizationReq(
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
    //         decode_iso2_CertificateInstallationReq(
    //             stream,
    //             &mut (*exiFrag).c2rust_unnamed.CertificateInstallationReq,
    //         )?;
    //         (*exiFrag).set_CertificateInstallationReq_isUsed(1 as u32);
    //     }
    //     16 => {}
    //     17 => {
    //         decode_iso2_CertificateUpdateReq(
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
    //         decode_iso2_CertificateChain(
    //             stream,
    //             &mut (*exiFrag).c2rust_unnamed.contract_signature_cert_chain,
    //         )?;
    //         (*exiFrag).set_ContractSignatureCertChain_isUsed(1 as u32);
    //     }
    //     34 => {
    //         decode_iso2_ContractSignatureEncryptedPrivateKey(
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
    //         decode_iso2_DiffieHellmanPublickey(
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
    //         decode_iso2_MeteringReceiptReq(
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
    //         decode_iso2_SalesTariff(
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
    //         decode_iso2_SignedInfo(
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
    //         decode_iso2_EMAID(stream, &mut (*exiFrag).c2rust_unnamed.eMaid);
    //         (*exiFrag).set_eMAID_isUsed(1 as u32);
    //     }
    //     237 => {}
    //     238 => {}
    //     239 => {}
    //     240 => {}
    //     241 => {}
    //     242 => {}
    //     _ => {
    //         return Err(ExiError::UnsupportedSubEvent);
    //     }
    // }
    // if error == 0 as i32 {
    //     exi_basetypes_decoder_nbit_uint(stream, 8 as i32 as usize, &mut event_code)?;
    //     if error == 0 as i32 {
    //         if event_code != 244 as i32 as u32 {
    //             return Err(ExiError::INCORRECT_END_FRAGMENT_VALUE);
    //         }
    //     }
    // }
}

pub fn decode_iso2_xmldsig_fragment(
    stream: &mut ExiBitstream,
    mut _xmldsigFrag: &mut Iso2XmlDSigFragment,
) -> Result<(), ExiError> {
    let mut event_code: u32 = 0;
    stream.read_and_check_header()?;
    exi_basetypes_decoder_nbit_uint(stream, 6 as i32 as usize, &mut event_code)?;
    return Err(ExiError::NotImplementedYet);
    // match event_code {
    //     0 => {
    //         decode_iso2_CanonicalizationMethod(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.CanonicalizationMethod,
    //         )?;
    //         (*xmldsigFrag).set_CanonicalizationMethod_isUsed(1 as u32);
    //     }
    //     1 => {
    //         decode_iso2_dsa_key_value(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.dsa_key_value,
    //         )?;
    //         (*xmldsigFrag).set_dsa_key_value_isUsed(1 as u32);
    //     }
    //     2 => {
    //         decode_iso2_DigestMethod(
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
    //             decode_iso2_KeyInfo(stream, &mut (*xmldsigFrag).XmlDSigComponents.KeyInfo);
    //         (*xmldsigFrag).set_KeyInfo_isUsed(1 as u32);
    //     }
    //     9 => {}
    //     10 => {
    //         decode_iso2_KeyValue(
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
    //             decode_iso2_Object(stream, &mut (*xmldsigFrag).XmlDSigComponents.Object);
    //         (*xmldsigFrag).set_Object_isUsed(1 as u32);
    //     }
    //     15 => {}
    //     16 => {
    //         error =
    //             decode_iso2_PGPData(stream, &mut (*xmldsigFrag).XmlDSigComponents.PGPData);
    //         (*xmldsigFrag).set_PGPData_isUsed(1 as u32);
    //     }
    //     17 => {}
    //     18 => {}
    //     19 => {}
    //     20 => {}
    //     21 => {
    //         decode_iso2_RSAKeyValue(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.RSAKeyValue,
    //         )?;
    //         (*xmldsigFrag).set_RSAKeyValue_isUsed(1 as u32);
    //     }
    //     22 => {
    //         decode_iso2_Reference(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.Reference,
    //         )?;
    //         (*xmldsigFrag).set_Reference_isUsed(1 as u32);
    //     }
    //     23 => {
    //         decode_iso2_RetrievalMethod(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.RetrievalMethod,
    //         )?;
    //         (*xmldsigFrag).set_RetrievalMethod_isUsed(1 as u32);
    //     }
    //     24 => {
    //         decode_iso2_SPKIData(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.SPKIData,
    //         )?;
    //         (*xmldsigFrag).set_SPKIData_isUsed(1 as u32);
    //     }
    //     25 => {}
    //     26 => {}
    //     27 => {
    //         decode_iso2_Signature(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.Signature,
    //         )?;
    //         (*xmldsigFrag).set_Signature_isUsed(1 as u32);
    //     }
    //     28 => {
    //         decode_iso2_SignatureMethod(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.SignatureMethod,
    //         )?;
    //         (*xmldsigFrag).set_SignatureMethod_isUsed(1 as u32);
    //     }
    //     29 => {}
    //     30 => {}
    //     31 => {
    //         decode_iso2_SignatureValue(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.SignatureValue,
    //         )?;
    //         (*xmldsigFrag).set_SignatureValue_isUsed(1 as u32);
    //     }
    //     32 => {
    //         decode_iso2_SignedInfo(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.SignedInfo,
    //         )?;
    //         (*xmldsigFrag).set_SignedInfo_isUsed(1 as u32);
    //     }
    //     33 => {
    //         decode_iso2_Transform(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.Transform,
    //         )?;
    //         (*xmldsigFrag).set_Transform_isUsed(1 as u32);
    //     }
    //     34 => {
    //         decode_iso2_Transforms(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.Transforms,
    //         )?;
    //         (*xmldsigFrag).set_Transforms_isUsed(1 as u32);
    //     }
    //     35 => {}
    //     36 => {}
    //     37 => {
    //         decode_iso2_X509Data(
    //             stream,
    //             &mut (*xmldsigFrag).XmlDSigComponents.X509Data,
    //         )?;
    //         (*xmldsigFrag).set_X509Data_isUsed(1 as u32);
    //     }
    //     38 => {}
    //     39 => {
    //         decode_iso2_X509IssuerSerial(
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
    //         return Err(ExiError::UnsupportedSubEvent);
    //     }
    // }
    // if error == 0 as i32 {
    //     exi_basetypes_decoder_nbit_uint(stream, 6 as i32 as usize, &mut event_code)?;
    //     if error == 0 as i32 {
    //         if event_code != 46 as i32 as u32 {
    //             return Err(ExiError::INCORRECT_END_FRAGMENT_VALUE);
    //         }
    //     }
    // }
}
