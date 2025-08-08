use crate::common::exi_basetypes::ExiSigned;
use crate::common::exi_basetypes_decoder::{
    exi_basetypes_decoder_bytes, exi_basetypes_decoder_characters, exi_basetypes_decoder_nbit_uint,
    exi_basetypes_decoder_signed, exi_basetypes_decoder_uint_16,
};
use crate::common::exi_bitstream::ExiBitstream;
use crate::common::exi_error_codes::ExiError;
use crate::common::exi_types_decoder::{decode_exi_type_hex_binary, decode_exi_type_integer64};
use crate::iso_2::iso2_datatypes::{
    Iso2BodyType, Iso2BodyTypeEnum, Iso2CanonicalizationMethodType, Iso2DSAKeyValueType,
    Iso2DigestMethodType, Iso2ExiDocument, Iso2FaultCodeType, Iso2KeyInfoType, Iso2KeyValueType,
    Iso2MessageHeaderType, Iso2NotificationType, Iso2ObjectType, Iso2PGPComponentType,
    Iso2PGPDataType, Iso2RSAKeyValueType, Iso2ReferenceType, Iso2ResponseCodeType,
    Iso2RetrievalMethodType, Iso2SPKIDataType, Iso2SessionSetupReqType, Iso2SessionSetupResType,
    Iso2SignatureMethodType, Iso2SignatureType, Iso2SignatureValueType, Iso2SignedInfoType,
    Iso2TransformType, Iso2TransformsType, Iso2X509DataType, Iso2X509IssuerSerialType,
    Iso2v2gMessage,
};

// pub fn decode_iso2_cost(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2CostType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 0 as i32;
//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             0 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         2 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).cost_kind = value as Iso2CostKindType;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 1 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             1 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_uint_32(stream, &mut (*message).amount)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 2 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             2 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_0: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         3 as i32 as usize,
//                                         &mut value_0,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).amount_multiplier =
//                                             Some(value_0.wrapping_sub(3) as i8);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 3 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }

pub fn decode_iso2_transform(
    stream: &mut ExiBitstream,
    message: &mut Iso2TransformType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 5_i32;
    let mut event_code: u32 = 0;
    let error: i32 = 0;
    loop {
        match grammar_id {
            5 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                            message.algorithm =
                                exi_basetypes_decoder_characters::<65>(stream, len)?;
                            grammar_id = 6_i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            6 => {
                exi_basetypes_decoder_nbit_uint(stream, 3_i32 as usize, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1_i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0_i32 {
                                if event_code == 0_i32 as u32 {
                                    let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                                    message.xpath =
                                        Some(exi_basetypes_decoder_characters::<65>(stream, len)?);
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0_i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1_i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0_i32 {
                                    if event_code == 0_i32 as u32 {
                                        grammar_id = 3_i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        1 => {
                            return Err(ExiError::UnknownEventForDecoding);
                        }
                        2 => {
                            return Ok(());
                        }
                        3 => {
                            message.any = Some(decode_exi_type_hex_binary::<4>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 3_i32;
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
                if error == 0_i32 {
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
// pub fn decode_iso2_interval(
//     stream: &mut ExiBitstream,
//     _message: &mut Iso2IntervalType,
// ) -> Result<(), ExiError> {
//     let mut event_code: u32 = 0;
//     exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//     if event_code != 0 as i32 as u32 {
//         return Err(ExiError::UnknownEventCode);
//     }

//     return Ok(());
// }
pub fn decode_iso2_transforms(
    stream: &mut ExiBitstream,
    message: &mut Iso2TransformsType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 7_i32;
    let mut event_code: u32 = 0;
    let error: i32 = 0;
    loop {
        match grammar_id {
            7 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            decode_iso2_transform(stream, &mut message.transform)?;
                            grammar_id = 8;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            8 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0_i32 {
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
                if error == 0_i32 {
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
    message: &mut Iso2DSAKeyValueType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 9_i32;
    let mut event_code: u32 = 0;
    let error: i32 = 0;
    loop {
        match grammar_id {
            9 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            //TODO: These clones fix the compile time issues but will cause runtime issues
                            message.p = Some(decode_exi_type_hex_binary::<350>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 10_i32;
                            }
                        }
                        1 => {
                            message.g = Some(decode_exi_type_hex_binary::<350>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 12_i32;
                            }
                        }
                        2 => {
                            message.y = decode_exi_type_hex_binary::<350>(stream)?;
                            if error == 0_i32 {
                                grammar_id = 13_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            message.q = Some(decode_exi_type_hex_binary::<350>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 11_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            message.g = Some(decode_exi_type_hex_binary::<350>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 12_i32;
                            }
                        }
                        1 => {
                            message.y = decode_exi_type_hex_binary::<350>(stream)?;
                            if error == 0_i32 {
                                grammar_id = 13_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            message.y = decode_exi_type_hex_binary::<350>(stream)?;
                            if error == 0_i32 {
                                grammar_id = 13_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            message.j = Some(decode_exi_type_hex_binary::<350>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 14_i32;
                            }
                        }
                        1 => {
                            message.seed = Some(decode_exi_type_hex_binary::<350>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 15_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            message.seed = Some(decode_exi_type_hex_binary::<350>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 15_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            message.pgen_counter = Some(decode_exi_type_hex_binary::<350>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 3_i32;
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
                if error == 0_i32 {
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
    message: &mut Iso2X509IssuerSerialType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 16_i32;
    let mut event_code: u32 = 0;
    let error: i32 = 0;
    loop {
        match grammar_id {
            16 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1_i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0_i32 {
                                if event_code == 0_i32 as u32 {
                                    let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                                    message.x509_issuer_name =
                                        exi_basetypes_decoder_characters::<65>(stream, len)?;
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0_i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1_i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0_i32 {
                                    if event_code == 0_i32 as u32 {
                                        grammar_id = 17_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1_i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0_i32 {
                                exi_basetypes_decoder_signed(
                                    stream,
                                    &mut message.x509_serial_number,
                                )?;
                                if error == 0_i32 {
                                    grammar_id = 3_i32;
                                }
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1_i32 as usize,
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
                if error == 0_i32 {
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
// pub fn decode_iso2_relative_time_interval(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2RelativeTimeIntervalType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 18 as i32;
//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             18 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_uint32(stream, &mut (*message).start)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 19 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             19 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_uint32(
//                                 stream,
//                                 &mut (*message).duration.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
pub fn decode_iso2_digest_method(
    stream: &mut ExiBitstream,
    message: &mut Iso2DigestMethodType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 20_i32;

    let mut event_code: u32 = 0;
    let error: i32 = 0;
    loop {
        match grammar_id {
            20 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                            message.algorithm =
                                exi_basetypes_decoder_characters::<65>(stream, len)?;
                            grammar_id = 21_i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            21 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            return Err(ExiError::UnknownEventForDecoding);
                        }
                        1 => {
                            return Ok(());
                        }
                        2 => {
                            message.any = Some(decode_exi_type_hex_binary::<4>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 3_i32;
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
                if error == 0_i32 {
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
pub fn decode_iso2_rsa_key_value(
    stream: &mut ExiBitstream,
    message: &mut Iso2RSAKeyValueType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 22_i32;

    let mut event_code: u32 = 0;
    let error: i32 = 0;
    loop {
        match grammar_id {
            22 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            message.modulus = decode_exi_type_hex_binary::<350>(stream)?;
                            grammar_id = 23_i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            23 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            message.exponent = decode_exi_type_hex_binary::<350>(stream)?;
                            if error == 0_i32 {
                                grammar_id = 3_i32;
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
                if error == 0_i32 {
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
    message: &mut Iso2CanonicalizationMethodType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 24_i32;

    let mut event_code: u32 = 0;
    let error: i32 = 0;
    loop {
        match grammar_id {
            24 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                            message.algorithm =
                                exi_basetypes_decoder_characters::<65>(stream, len)?;
                            grammar_id = 25_i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            25 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            return Err(ExiError::UnknownEventForDecoding);
                        }
                        1 => {
                            return Ok(());
                        }
                        2 => {
                            message.any = Some(decode_exi_type_hex_binary::<4>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 3_i32;
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
                if error == 0_i32 {
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
    message: &mut Iso2SignatureMethodType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 26_i32;

    let mut event_code: u32 = 0;
    let error: i32 = 0;
    loop {
        match grammar_id {
            26 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                            message.algorithm =
                                exi_basetypes_decoder_characters::<65>(stream, len)?;
                            grammar_id = 27_i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            27 => {
                exi_basetypes_decoder_nbit_uint(stream, 3_i32 as usize, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1_i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0_i32 {
                                //TODO: likely broke this
                                let mut hmac: ExiSigned = ExiSigned::default();
                                exi_basetypes_decoder_signed(stream, &mut hmac)?;
                                message.hmac_output_length = Some(hmac);
                                if error == 0_i32 {
                                    grammar_id = 28_i32;
                                }
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1_i32 as usize,
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
                            message.any = Some(decode_exi_type_hex_binary::<4>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 3_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            return Err(ExiError::UnknownEventForDecoding);
                        }
                        1 => {
                            return Ok(());
                        }
                        2 => {
                            message.any = Some(decode_exi_type_hex_binary::<4>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 3_i32;
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
                if error == 0_i32 {
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
    message: &mut Iso2KeyValueType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 29_i32;

    let mut event_code: u32 = 0;
    let error: i32 = 0;
    loop {
        match grammar_id {
            29 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            decode_iso2_dsa_key_value(
                                stream,
                                &mut message.dsa_key_value.clone().unwrap(),
                            )?;
                            if error == 0_i32 {
                                grammar_id = 3_i32;
                            }
                        }
                        1 => {
                            decode_iso2_rsa_key_value(
                                stream,
                                &mut message.rsa_key_value.clone().unwrap(),
                            )?;
                            if error == 0_i32 {
                                grammar_id = 3_i32;
                            }
                        }
                        2 => {
                            message.any = Some(decode_exi_type_hex_binary::<4>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 3_i32;
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
                if error == 0_i32 {
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

// pub fn decode_iso2_physical_value(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2PhysicalValueType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 30 as i32;
//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             30 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         3 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).multiplier =
//                                             value.wrapping_add(-(3 as i32) as u32) as i8;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 31 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             31 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_0: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         3 as i32 as usize,
//                                         &mut value_0,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).unit = value_0 as Iso2UnitSymbolType;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 32 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             32 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_integer16(stream, &mut (*message).value)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_consumption_cost(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2ConsumptionCostType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 33 as i32;
//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             33 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(stream, &mut (*message).start_value)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 34 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             34 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*message).cost.len() as i32) < 3 as i32 {
//                                 let fresh0 = (*message).cost.len();
//                                 if let Some(cost) = (*message).cost.get_mut(fresh0 as usize) {
//                                     decode_iso2_cost(stream, cost)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             grammar_id = 35 as i32;
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             35 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*message).cost.len() as i32) < 3 as i32 {
//                                 let fresh1 = (*message).cost.len();
//                                 if let Some(cost) = (*message).cost.get_mut(fresh1 as usize) {
//                                     decode_iso2_cost(stream, cost)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             if ((*message).cost.len() as i32) < 3 as i32 {
//                                 grammar_id = 35 as i32;
//                             } else {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_pmax_schedule_entry(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2PMaxScheduleEntryType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 36 as i32;
//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             36 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_relative_time_interval(
//                                 stream,
//                                 &mut (*message)
//                                     .relative_time_interval
//                                     .as_mut()
//                                     .unwrap()
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 37 as i32;
//                             }
//                         }
//                         1 => {
//                             decode_iso2_interval(
//                                 stream,
//                                 &mut (*message).time_interval.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 37 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             37 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(stream, &mut (*message).p_max)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_sales_tariff_entry(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2SalesTariffEntryType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 38 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             38 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_relative_time_interval(
//                                 stream,
//                                 &mut (*message).relative_time_interval.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 39 as i32;
//                             }
//                         }
//                         1 => {
//                             decode_iso2_interval(
//                                 stream,
//                                 &mut (*message).time_interval.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 39 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             39 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         8 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).e_price_level = Some(value as u8);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 41 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         1 => {
//                             if ((*message).consumption_cost.len() as i32) < 3 as i32 {
//                                 let fresh2 = (*message).consumption_cost.len();
//                                 if let Some(consumption_cost) =
//                                     (*message).consumption_cost.get_mut(fresh2 as usize)
//                                 {
//                                     decode_iso2_consumption_cost(stream, consumption_cost)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             grammar_id = 40 as i32;
//                         }
//                         2 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             40 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*message).consumption_cost.len() as i32) < 3 as i32 {
//                                 let fresh3 = (*message).consumption_cost.len();
//                                 if let Some(consumption_cost) =
//                                     (*message).consumption_cost.get_mut(fresh3 as usize)
//                                 {
//                                     decode_iso2_consumption_cost(stream, consumption_cost)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             if ((*message).consumption_cost.len() as i32) < 3 as i32 {
//                                 grammar_id = 40 as i32;
//                             } else {
//                                 grammar_id = 41 as i32;
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             41 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*message).consumption_cost.len() as i32) < 3 as i32 {
//                                 let fresh4 = (*message).consumption_cost.len();
//                                 if let Some(consumption_cost) =
//                                     (*message).consumption_cost.get_mut(fresh4 as usize)
//                                 {
//                                     decode_iso2_consumption_cost(stream, consumption_cost)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             grammar_id = 42 as i32;
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             42 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*message).consumption_cost.len() as i32) < 3 as i32 {
//                                 let fresh5 = (*message).consumption_cost.len();
//                                 if let Some(consumption_cost) =
//                                     (*message).consumption_cost.get_mut(fresh5 as usize)
//                                 {
//                                     decode_iso2_consumption_cost(stream, consumption_cost)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             if ((*message).consumption_cost.len() as i32) < 3 as i32 {
//                                 grammar_id = 42 as i32;
//                             } else {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_parameter(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2ParameterType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 43 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             43 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_uint_16(
//                                 stream,
//                                 &mut ((*message).name.len() as u16),
//                             )?;
//                             if error == 0 as i32 {
//                                 if (*message).name.len() >= 2 {
//                                     exi_basetypes_decoder_characters(
//                                         stream,
//                                         (*message).name.len() as usize,
//                                         &mut (*message).name,
//                                         (64 as i32 + 1 as i32) as usize,
//                                     )?;
//                                 } else {
//                                     return Err(ExiError::StringValuesNotSupported);
//                                 }
//                             }
//                             grammar_id = 44 as i32;
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             44 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).bool_value = Some(value as i32);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 3 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         1 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_0: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         8 as i32 as usize,
//                                         &mut value_0,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).byte_value =
//                                             Some(value_0.wrapping_sub(128) as i8);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 3 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         2 => {
//                             decode_exi_type_integer16(
//                                 stream,
//                                 &mut (*message).short_value.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         3 => {
//                             decode_exi_type_integer32(
//                                 stream,
//                                 &mut (*message).int_value.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         4 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).physical_value.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         5 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     exi_basetypes_decoder_uint_16(
//                                         stream,
//                                         &mut ((*message).string_value.clone().unwrap().len() as u16),
//                                     )?;
//                                     if error == 0 as i32 {
//                                         if (*message).string_value.clone().unwrap().len() >= 2 {
//                                             exi_basetypes_decoder_characters(
//                                                 stream,
//                                                 (*message).string_value.clone().unwrap().len() as usize,
//                                                 &mut (*message).string_value.unwrap(),
//                                             )?;
//                                         } else {
//                                             return Err(ExiError::StringValuesNotSupported);
//                                         }
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 3 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_pmax_schedule(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2PMaxScheduleType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 45 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             45 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*message).p_max_schedule_entry.len() as i32) < 12 as i32 {
//                                 let fresh6 = (*message).p_max_schedule_entry.len();
//                                 if let Some(entry) = (*message)
//                                     .p_max_schedule_entry
//                                     .get_mut(fresh6 as usize)
//                                 {
//                                     decode_iso2_pmax_schedule_entry(stream, entry)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             grammar_id = 46 as i32;
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             46 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*message).p_max_schedule_entry.len() as i32) < 12 as i32 {
//                                 let fresh7 = (*message).p_max_schedule_entry.len();
//                                 if let Some(entry) = (*message)
//                                     .p_max_schedule_entry
//                                     .get_mut(fresh7 as usize)
//                                 {
//                                     decode_iso2_pmax_schedule_entry(stream, entry)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             if ((*message).p_max_schedule_entry.len() as i32) < 1024 as i32 {
//                                 grammar_id = 46 as i32;
//                             } else {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
pub fn decode_iso2_reference(
    stream: &mut ExiBitstream,
    message: &mut Iso2ReferenceType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 47_i32;

    let mut event_code: u32 = 0;
    let error: i32 = 0;
    loop {
        match grammar_id {
            47 => {
                exi_basetypes_decoder_nbit_uint(stream, 3_i32 as usize, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                            message.id = Some(exi_basetypes_decoder_characters::<65>(stream, len)?);
                            grammar_id = 48_i32;
                        }
                        1 => {
                            let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                            message.ref_type =
                                Some(exi_basetypes_decoder_characters::<65>(stream, len)?);
                            grammar_id = 49_i32;
                        }
                        2 => {
                            let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                            message.uri =
                                Some(exi_basetypes_decoder_characters::<65>(stream, len)?);
                            grammar_id = 50_i32;
                        }
                        3 => {
                            decode_iso2_transforms(
                                stream,
                                &mut message.transforms.clone().unwrap(),
                            )?;
                            if error == 0_i32 {
                                grammar_id = 51_i32;
                            }
                        }
                        4 => {
                            decode_iso2_digest_method(stream, &mut message.digest_method)?;
                            if error == 0_i32 {
                                grammar_id = 52_i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            48 => {
                exi_basetypes_decoder_nbit_uint(stream, 3_i32 as usize, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                            message.ref_type =
                                Some(exi_basetypes_decoder_characters::<65>(stream, len)?);
                            grammar_id = 49_i32;
                        }
                        1 => {
                            let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                            message.uri =
                                Some(exi_basetypes_decoder_characters::<65>(stream, len)?);
                            grammar_id = 50_i32;
                        }
                        2 => {
                            decode_iso2_transforms(
                                stream,
                                &mut message.transforms.clone().unwrap(),
                            )?;
                            if error == 0_i32 {
                                grammar_id = 51_i32;
                            }
                        }
                        3 => {
                            decode_iso2_digest_method(stream, &mut message.digest_method)?;
                            if error == 0_i32 {
                                grammar_id = 52_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                            message.uri =
                                Some(exi_basetypes_decoder_characters::<65>(stream, len)?);
                            grammar_id = 50_i32;
                        }
                        1 => {
                            decode_iso2_transforms(
                                stream,
                                &mut message.transforms.clone().unwrap(),
                            )?;
                            if error == 0_i32 {
                                grammar_id = 51_i32;
                            }
                        }
                        2 => {
                            decode_iso2_digest_method(stream, &mut message.digest_method)?;
                            if error == 0_i32 {
                                grammar_id = 52_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            decode_iso2_transforms(
                                stream,
                                &mut message.transforms.clone().unwrap(),
                            )?;
                            if error == 0_i32 {
                                grammar_id = 51_i32;
                            }
                        }
                        1 => {
                            decode_iso2_digest_method(stream, &mut message.digest_method)?;
                            if error == 0_i32 {
                                grammar_id = 52_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            decode_iso2_digest_method(stream, &mut message.digest_method)?;
                            if error == 0_i32 {
                                grammar_id = 52_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            message.digest_value = Some(decode_exi_type_hex_binary::<350>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 3_i32;
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
                if error == 0_i32 {
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
    message: &mut Iso2RetrievalMethodType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 53_i32;

    let mut event_code: u32 = 0;
    let error: i32 = 0;
    loop {
        match grammar_id {
            53 => {
                exi_basetypes_decoder_nbit_uint(stream, 3_i32 as usize, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                            message.ref_type =
                                Some(exi_basetypes_decoder_characters::<65>(stream, len)?);
                            grammar_id = 54_i32;
                        }
                        1 => {
                            let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                            message.uri =
                                Some(exi_basetypes_decoder_characters::<65>(stream, len)?);
                            grammar_id = 55_i32;
                        }
                        2 => {
                            decode_iso2_transforms(
                                stream,
                                &mut message.transforms.clone().unwrap(),
                            )?;
                            if error == 0_i32 {
                                grammar_id = 3_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                            message.uri =
                                Some(exi_basetypes_decoder_characters::<65>(stream, len)?);
                            grammar_id = 55_i32;
                        }
                        1 => {
                            decode_iso2_transforms(
                                stream,
                                &mut message.transforms.clone().unwrap(),
                            )?;
                            if error == 0_i32 {
                                grammar_id = 3_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            decode_iso2_transforms(
                                stream,
                                &mut message.transforms.clone().unwrap(),
                            )?;
                            if error == 0_i32 {
                                grammar_id = 3_i32;
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
                if error == 0_i32 {
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
// pub fn decode_iso2_sales_tariff(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2SalesTariffType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 56 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             56 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_uint_16(
//                                 stream,
//                                 &mut ((*message).id.clone().unwrap().len() as u16),
//                             )?;
//                             if error == 0 as i32 {
//                                 if (*message).id.clone().unwrap().len() >= 2 {
//                                     (*message).id.unwrap().len -= 2;
//                                     exi_basetypes_decoder_characters(
//                                         stream,
//                                         (*message).id.clone().unwrap().len() as usize,
//                                         &mut (*message).id.as_mut().unwrap()
//                                         (64 as i32 + 1 as i32) as usize,
//                                     )?;
//                                 } else {
//                                     return Err(ExiError::StringValuesNotSupported);
//                                 }
//                             }
//                             grammar_id = 57 as i32;
//                         }
//                         1 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         8 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).SalesTariffID =
//                                             value.wrapping_add(1 as i32 as u32) as u8;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 58 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             57 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_0: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         8 as i32 as usize,
//                                         &mut value_0,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).SalesTariffID =
//                                             value_0.wrapping_add(1 as i32 as u32) as u8;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 58 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             58 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     exi_basetypes_decoder_uint_16(
//                                         stream,
//                                         &mut ((*message).sales_tariff_description.clone().unwrap().len()
//                                             as u16),
//                                     )?;
//                                     if error == 0 as i32 {
//                                         if (*message).sales_tariff_description.clone().unwrap().len() as i32
//                                             >= 2 as i32
//                                         {
//                                             exi_basetypes_decoder_characters(
//                                                 stream,
//                                                 (*message)
//                                                     .sales_tariff_description
//                                                     .unwrap().clone()
//                                                     .len(),
//                                                 &mut (*message)
//                                                     .sales_tariff_description.as_mut()
//                                                     .unwrap(),
//                                             )?;
//                                         } else {
//                                             return Err(ExiError::StringValuesNotSupported);
//                                         }
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 60 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         1 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_1: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         8 as i32 as usize,
//                                         &mut value_1,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).num_e_price_levels = Some(value_1 as u8);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 62 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         2 => {
//                             if ((*message).sales_tariff_entry.len() as i32) < 12 as i32 {
//                                 let fresh8 = (*message).sales_tariff_entry.len();
//                                 (*message).sales_tariff_entry.len() =
//                                     ((*message).sales_tariff_entry.len()).wrapping_add(1);
//                                 if let Some(entry) =
//                                     (*sales_tariff).sales_tariff_entry.get_mut(fresh8 as usize)
//                                 {
//                                     decode_iso2_sales_tariff_entry(stream, entry)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             grammar_id = 59 as i32;
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             59 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*message).sales_tariff_entry.len() as i32) < 12 as i32 {
//                                 let fresh9 = (*message).sales_tariff_entry.len();
//                                 (*message).sales_tariff_entry.len() =
//                                     ((*message).sales_tariff_entry.len()).wrapping_add(1);
//                                 if let Some(entry) =
//                                     (*sales_tariff).sales_tariff_entry.get_mut(fresh9 as usize)
//                                 {
//                                     decode_iso2_sales_tariff_entry(stream, entry)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             if ((*message).sales_tariff_entry.len() as i32) < 1024 as i32 {
//                                 grammar_id = 59 as i32;
//                             } else {
//                                 grammar_id = 60 as i32;
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             60 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_2: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         8 as i32 as usize,
//                                         &mut value_2,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).num_e_price_levels = Some(value_2 as u8);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 62 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         1 => {
//                             if ((*message).sales_tariff_entry.len() as i32) < 12 as i32 {
//                                 let fresh10 = (*message).sales_tariff_entry.len();
//                                 (*message).sales_tariff_entry.len() =
//                                     ((*message).sales_tariff_entry.len()).wrapping_add(1);
//                                 if let Some(entry) =
//                                     (*sales_tariff).sales_tariff_entry.get_mut(fresh10 as usize)
//                                 {
//                                     decode_iso2_sales_tariff_entry(stream, entry)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             grammar_id = 61 as i32;
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             61 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*message).sales_tariff_entry.len() as i32) < 12 as i32 {
//                                 let fresh11 = (*message).sales_tariff_entry.len();
//                                 (*message).sales_tariff_entry.len() =
//                                     ((*message).sales_tariff_entry.len()).wrapping_add(1);
//                                 if let Some(entry) =
//                                     (*sales_tariff).sales_tariff_entry.get_mut(fresh11 as usize)
//                                 {
//                                     decode_iso2_sales_tariff_entry(stream, entry)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             if ((*message).SalesTariffEntry.len() as i32) < 1024 as i32 {
//                                 grammar_id = 61 as i32;
//                             } else {
//                                 grammar_id = 62 as i32;
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             62 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*message).sales_tariff_entry.len() as i32) < 12 as i32 {
//                                 let fresh12 = (*message).sales_tariff_entry.len();
//                                 (*message).sales_tariff_entry.len() =
//                                     ((*message).sales_tariff_entry.len()).wrapping_add(1);
//                                 if let Some(entry) =
//                                     (*sales_tariff).sales_tariff_entry.get_mut(fresh12 as usize)
//                                 {
//                                     decode_iso2_sales_tariff_entry(stream, entry)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             grammar_id = 63 as i32;
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             63 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*message).sales_tariff_entry.len() as i32) < 12 as i32 {
//                                 let fresh13 = (*message).sales_tariff_entry.len();
//                                 (*message).sales_tariff_entry.len() =
//                                     ((*message).sales_tariff_entry.len()).wrapping_add(1);
//                                 if let Some(entry) =
//                                     (*sales_tariff).sales_tariff_entry.get_mut(fresh13 as usize)
//                                 {
//                                     decode_iso2_sales_tariff_entry(stream, entry)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             if ((*message).sales_tariff_entry.len() as i32) < 1024 as i32 {
//                                 grammar_id = 63 as i32;
//                             } else {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
pub fn decode_iso2_x509_data(
    stream: &mut ExiBitstream,
    message: &mut Iso2X509DataType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 64_i32;

    let mut event_code: u32 = 0;
    let error: i32 = 0;
    loop {
        match grammar_id {
            64 => {
                exi_basetypes_decoder_nbit_uint(stream, 3_i32 as usize, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            let mut issuer: Iso2X509IssuerSerialType = Default::default();
                            decode_iso2_x509_issuer_serial(stream, &mut issuer)?;
                            message.x509_issuer_serial = Some(issuer);
                            if error == 0_i32 {
                                grammar_id = 3_i32;
                            }
                        }
                        1 => {
                            message.x509_ski = Some(decode_exi_type_hex_binary::<350>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 3_i32;
                            }
                        }
                        2 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1_i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0_i32 {
                                if event_code == 0_i32 as u32 {
                                    let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                                    message.x509_subject_name =
                                        Some(exi_basetypes_decoder_characters::<65>(stream, len)?);
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0_i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1_i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0_i32 {
                                    if event_code == 0_i32 as u32 {
                                        grammar_id = 3_i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        3 => {
                            message.x509_certificate = Some(decode_exi_type_hex_binary::<350>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 3_i32;
                            }
                        }
                        4 => {
                            message.x509_crl = Some(decode_exi_type_hex_binary::<350>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 3_i32;
                            }
                        }
                        5 => {
                            message.any = Some(decode_exi_type_hex_binary::<4>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 3_i32;
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
                if error == 0_i32 {
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
    message: &mut Iso2PGPDataType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 65_i32;
    let mut event_code: u32 = 0;
    let error: i32 = 0;
    loop {
        match grammar_id {
            65 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            let pgp_data = match &mut message.pgp_component {
                                Iso2PGPComponentType::Choice1(ref mut c1) => c1,
                                _ => return Err(ExiError::UnknownEventCode),
                            };
                            pgp_data.pgp_key_id = decode_exi_type_hex_binary::<350>(stream)?;
                            if error == 0_i32 {
                                grammar_id = 66_i32;
                            }
                        }
                        1 => {
                            let pgp_data = match &mut message.pgp_component {
                                Iso2PGPComponentType::Choice1(ref mut c1) => c1,
                                _ => return Err(ExiError::UnknownEventCode),
                            };
                            pgp_data.pgp_key_packet =
                                Some(decode_exi_type_hex_binary::<350>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 67_i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            66 => {
                exi_basetypes_decoder_nbit_uint(stream, 3_i32 as usize, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            let pgp_data = match &mut message.pgp_component {
                                Iso2PGPComponentType::Choice1(ref mut c1) => c1,
                                _ => return Err(ExiError::UnknownEventCode),
                            };
                            pgp_data.pgp_key_packet =
                                Some(decode_exi_type_hex_binary::<350>(stream)?);
                            grammar_id = 67_i32;
                        }
                        1 => {
                            return Err(ExiError::UnknownEventForDecoding);
                        }
                        2 => {
                            return Ok(());
                        }
                        3 => {
                            let pgp_data = match &mut message.pgp_component {
                                Iso2PGPComponentType::Choice1(ref mut c1) => c1,
                                _ => return Err(ExiError::UnknownEventCode),
                            };
                            pgp_data.any = Some(decode_exi_type_hex_binary::<4>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 68_i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            67 => {
                exi_basetypes_decoder_nbit_uint(stream, 3_i32 as usize, &mut event_code)?;
                if error == 0_i32 {
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
                            let pgp_data = match &mut message.pgp_component {
                                Iso2PGPComponentType::Choice1(ref mut c1) => c1,
                                _ => return Err(ExiError::UnknownEventCode),
                            };
                            pgp_data.any = Some(decode_exi_type_hex_binary::<4>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 68_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            // Ensure we are using PGPChoice2Type
                            let pgp_data = match &mut message.pgp_component {
                                Iso2PGPComponentType::Choice2(ref mut c2) => c2,
                                _ => return Err(ExiError::UnknownEventCode),
                            };
                            pgp_data.pgp_key_packet = decode_exi_type_hex_binary::<350>(stream)?;
                            if error == 0_i32 {
                                grammar_id = 69_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            return Err(ExiError::UnknownEventForDecoding);
                        }
                        1 => {
                            return Ok(());
                        }
                        2 => {
                            let pgp_data = match &mut message.pgp_component {
                                Iso2PGPComponentType::Choice2(ref mut c2) => c2,
                                _ => return Err(ExiError::UnknownEventCode),
                            };
                            pgp_data.any = Some(decode_exi_type_hex_binary::<4>(stream)?);
                            grammar_id = 68_i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0_i32 {
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
    message: &mut Iso2SPKIDataType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 70_i32;

    let mut event_code: u32 = 0;
    let error: i32 = 0;
    loop {
        match grammar_id {
            70 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            message.spkisexp = decode_exi_type_hex_binary::<350>(stream)?;
                            grammar_id = 71_i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            71 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            return Err(ExiError::UnknownEventForDecoding);
                        }
                        1 => {
                            return Ok(());
                        }
                        2 => {
                            message.any = Some(decode_exi_type_hex_binary::<4>(stream)?);
                            grammar_id = 3_i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0_i32 {
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
    message: &mut Iso2SignedInfoType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 72_i32;

    let mut event_code: u32 = 0;
    let error: i32 = 0;
    loop {
        match grammar_id {
            72 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                            message.id = Some(exi_basetypes_decoder_characters::<65>(stream, len)?);
                            grammar_id = 73_i32;
                        }
                        1 => {
                            decode_iso2_canonicalization_method(
                                stream,
                                &mut message.canonicalization_method,
                            )?;
                            if error == 0_i32 {
                                grammar_id = 74_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            decode_iso2_canonicalization_method(
                                stream,
                                &mut message.canonicalization_method,
                            )?;
                            if error == 0_i32 {
                                grammar_id = 74_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            decode_iso2_signature_method(stream, &mut message.signature_method)?;
                            if error == 0_i32 {
                                grammar_id = 75_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            if message.reference_len < 4 {
                                let fresh14 = message.reference_len;
                                if let Some(reference) = message.reference.get_mut(fresh14) {
                                    decode_iso2_reference(stream, reference)?;
                                    message.reference_len += 1;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            grammar_id = 76_i32;
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            76 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            if (message.reference_len) < 4_usize {
                                let fresh15 = message.reference_len;
                                if let Some(reference) = message.reference.get_mut(fresh15) {
                                    decode_iso2_reference(stream, reference)?;
                                } else {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            } else {
                                return Err(ExiError::ArrayOutOfBounds);
                            }
                            grammar_id = 76_i32;
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
                if error == 0_i32 {
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
// pub fn decode_iso2_profile_entry(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2ProfileEntryType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 77 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             77 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_uint32(
//                                 stream,
//                                 &mut (*message).ChargingProfileEntryStart,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 78 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             78 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).ChargingProfileEntryMaxPower,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 79 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             79 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         2 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*ProfileEntryType)
//                                             .ChargingProfileEntryMaxNumberOfPhasesInUse =
//                                             Some(value.wrapping_add(1) as i8);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 3 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_dc_ev_status(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2DCEVStatusType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 80 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             80 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).EVReady = value as i32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 81 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             81 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_0: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         4 as i32 as usize,
//                                         &mut value_0,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).EVErrorCode = value_0 as Iso2DcEvErrorCodeType;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 82 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             82 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_1: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         7 as i32 as usize,
//                                         &mut value_1,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).EVRESSSOC = value_1 as i8;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 3 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_parameter_set(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2ParameterSetType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 83 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             83 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_integer16(stream, &mut (*message).ParameterSetID)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 84 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             84 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*message).Parameter.len() as i32) < 16 as i32 {
//                                 let fresh16 = (*message).Parameter.len();
//                                 (*message).Parameter.len() =
//                                     ((*message).Parameter.len()).wrapping_add(1);
//                                 if let Some(param) =
//                                     (*ParameterSetType).Parameter.get_mut(fresh16 as usize)
//                                 {
//                                     decode_iso2_parameter(stream, param)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             grammar_id = 85 as i32;
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             85 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*message).Parameter.len() as i32) < 16 as i32 {
//                                 let fresh17 = (*message).Parameter.len();
//                                 (*message).Parameter.len() =
//                                     ((*message).Parameter.len()).wrapping_add(1);
//                                 if let Some(param) =
//                                     (*ParameterSetType).Parameter.get_mut(fresh17 as usize)
//                                 {
//                                     decode_iso2_parameter(stream, param)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             if ((*message).Parameter.len() as i32) < 16 as i32 {
//                                 grammar_id = 85 as i32;
//                             } else {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_sa_schedule_tuple(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2SAScheduleTupleType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 86 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             86 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         8 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).SAScheduleTupleID =
//                                             value.wrapping_add(1 as i32 as u32) as u8;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 87 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             87 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_pmax_schedule(stream, &mut (*message).PMaxSchedule)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 88 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             88 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_sales_tariff(
//                                 stream,
//                                 &mut (*message).SalesTariff.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_selected_service(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2SelectedServiceType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 89 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             89 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_uint16(stream, &mut (*message).service_id)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 90 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             90 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_integer16(
//                                 stream,
//                                 &mut (*message).ParameterSetID.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_service(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2ServiceType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 91 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             91 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_uint16(stream, &mut (*message).service_id)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 92 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             92 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     exi_basetypes_decoder_uint_16(
//                                         stream,
//                                         &mut ((*message).service_name.clone().unwrap().len() as u16),
//                                     )?;
//                                     if error == 0 as i32 {
//                                         if (*message).service_name.clone().unwrap().len() >= 2 {
//                                             (*message).service_name.unwrap().len -= 2;
//                                             exi_basetypes_decoder_characters(
//                                                 stream,
//                                                 (*message).service_name.clone().unwrap().len(),
//                                                 &mut (*message).service_name.unwrap(),
//                                                 33,
//                                             )?;
//                                         } else {
//                                             return Err(ExiError::StringValuesNotSupported);
//                                         }
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 93 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         1 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         2 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).service_category =
//                                             value as Iso2ServiceCategoryType;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 94 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             93 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_0: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         2 as i32 as usize,
//                                         &mut value_0,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).service_category =
//                                             value_0 as Iso2ServiceCategoryType;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 94 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             94 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     exi_basetypes_decoder_uint_16(
//                                         stream,
//                                         &mut ((*message).ServiceScope.clone().unwrap().len() as u16),
//                                     )?;
//                                     if error == 0 as i32 {
//                                         if (*message).ServiceScope.clone().unwrap().len() >= 2 {
//                                             (*message).ServiceScope.unwrap().len -= 2;
//                                             exi_basetypes_decoder_characters(
//                                                 stream,
//                                                 (*message).ServiceScope.clone().unwrap().len(),
//                                                 &mut (*message).ServiceScope.unwrap(),
//                                                 65,
//                                             )?;
//                                         } else {
//                                             return Err(ExiError::StringValuesNotSupported);
//                                         }
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 95 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         1 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_1: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value_1,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).FreeService = value_1 as i32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 3 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             95 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_2: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value_2,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).FreeService = value_2 as i32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 3 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
pub fn decode_iso2_signature_value(
    stream: &mut ExiBitstream,
    message: &mut Iso2SignatureValueType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 96_i32;

    let mut event_code: u32 = 0;
    let error: i32 = 0;
    loop {
        match grammar_id {
            96 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                            message.id = Some(exi_basetypes_decoder_characters::<65>(stream, len)?);
                            grammar_id = 97_i32;
                        }
                        1 => {
                            let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                            if error == 0_i32 {
                                message.content = exi_basetypes_decoder_bytes::<65>(stream, len)?;
                                grammar_id = 3_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                            if error == 0_i32 {
                                message.content = exi_basetypes_decoder_bytes::<65>(stream, len)?;
                                if error == 0_i32 {
                                    grammar_id = 3_i32;
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
                if error == 0_i32 {
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
// pub fn decode_iso2_sub_certificates(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2SubCertificatesType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 98 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             98 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if (sub_certificates_type.Certificate.len() as i32) < 4 as i32 {
//                                 let idx = sub_certificates.Certificate.len() as usize;
//                                 if let Some(cert) = sub_certificates.Certificate.get_mut(idx) {
//                                     decode_exi_type_hex_binary(stream, &mut cert.len(), &mut cert)?;
//                                     if error == 0 as i32 {
//                                         (*message).Certificate.len() =
//                                             ((*message).Certificate.len()).wrapping_add(1);
//                                         grammar_id = 99 as i32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             99 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if (sub_certificates.Certificate.len() as i32) < 4 as i32 {
//                                 let idx = sub_certificates.Certificate.len() as usize;
//                                 if let Some(cert) = sub_certificates.Certificate.get_mut(idx) {
//                                     decode_exi_type_hex_binary(stream, &mut cert.len(), &mut cert)?;
//                                     if error == 0 as i32 {
//                                         sub_certificates.Certificate.len() =
//                                             (sub_certificates.Certificate.len()).wrapping_add(1);
//                                         grammar_id = 99 as i32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
pub fn decode_iso2_key_info(
    stream: &mut ExiBitstream,
    message: &mut Iso2KeyInfoType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 100_i32;

    let mut event_code: u32 = 0;
    let error: i32 = 0;
    loop {
        match grammar_id {
            100 => {
                exi_basetypes_decoder_nbit_uint(stream, 4_i32 as usize, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                            message.id = Some(exi_basetypes_decoder_characters::<65>(stream, len)?);
                            grammar_id = 101_i32;
                        }
                        1 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1_i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0_i32 {
                                if event_code == 0_i32 as u32 {
                                    let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                                    message.key_name =
                                        Some(exi_basetypes_decoder_characters::<65>(stream, len)?);
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0_i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1_i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0_i32 {
                                    if event_code == 0_i32 as u32 {
                                        grammar_id = 3_i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        2 => {
                            decode_iso2_key_value(stream, &mut message.key_value.clone().unwrap())?;
                            if error == 0_i32 {
                                grammar_id = 3_i32;
                            }
                        }
                        3 => {
                            decode_iso2_retrieval_method(
                                stream,
                                &mut message.retrieval_method.clone().unwrap(),
                            )?;
                            if error == 0_i32 {
                                grammar_id = 3_i32;
                            }
                        }
                        4 => {
                            decode_iso2_x509_data(stream, message.x509_data.as_mut().unwrap())?;
                            if error == 0_i32 {
                                grammar_id = 3_i32;
                            }
                        }
                        5 => {
                            decode_iso2_pgp_data(stream, message.pgp_data.as_mut().unwrap())?;
                            if error == 0_i32 {
                                grammar_id = 3_i32;
                            }
                        }
                        6 => {
                            decode_iso2_spki_data(stream, &mut message.spki_data.clone().unwrap())?;
                            if error == 0_i32 {
                                grammar_id = 3_i32;
                            }
                        }
                        7 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1_i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0_i32 {
                                if event_code == 0_i32 as u32 {
                                    let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                                    message.mgmt_data =
                                        Some(exi_basetypes_decoder_characters::<65>(stream, len)?);
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0_i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1_i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0_i32 {
                                    if event_code == 0_i32 as u32 {
                                        grammar_id = 3_i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        8 => {
                            message.any = Some(decode_exi_type_hex_binary::<4>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 3_i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            101 => {
                exi_basetypes_decoder_nbit_uint(stream, 4_i32 as usize, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1_i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0_i32 {
                                if event_code == 0_i32 as u32 {
                                    let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                                    message.key_name =
                                        Some(exi_basetypes_decoder_characters::<65>(stream, len)?);
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0_i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1_i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0_i32 {
                                    if event_code == 0_i32 as u32 {
                                        grammar_id = 3_i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        1 => {
                            decode_iso2_key_value(stream, &mut message.key_value.clone().unwrap())?;
                            if error == 0_i32 {
                                grammar_id = 3_i32;
                            }
                        }
                        2 => {
                            decode_iso2_retrieval_method(
                                stream,
                                &mut message.retrieval_method.clone().unwrap(),
                            )?;
                            if error == 0_i32 {
                                grammar_id = 3_i32;
                            }
                        }
                        3 => {
                            decode_iso2_x509_data(stream, message.x509_data.as_mut().unwrap())?;
                            if error == 0_i32 {
                                grammar_id = 3_i32;
                            }
                        }
                        4 => {
                            decode_iso2_pgp_data(stream, message.pgp_data.as_mut().unwrap())?;
                            if error == 0_i32 {
                                grammar_id = 3_i32;
                            }
                        }
                        5 => {
                            decode_iso2_spki_data(stream, &mut message.spki_data.clone().unwrap())?;
                            if error == 0_i32 {
                                grammar_id = 3_i32;
                            }
                        }
                        6 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1_i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0_i32 {
                                if event_code == 0_i32 as u32 {
                                    let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                                    message.mgmt_data =
                                        Some(exi_basetypes_decoder_characters::<65>(stream, len)?);
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0_i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1_i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0_i32 {
                                    if event_code == 0_i32 as u32 {
                                        grammar_id = 3_i32;
                                    } else {
                                        return Err(ExiError::DeviantsNotSupported);
                                    }
                                }
                            }
                        }
                        7 => {
                            message.any = Some(decode_exi_type_hex_binary::<4>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 3_i32;
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
                if error == 0_i32 {
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
    message: &mut Iso2ObjectType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 102_i32;
    let mut event_code: u32 = 0;
    let error: i32 = 0;
    loop {
        match grammar_id {
            102 => {
                exi_basetypes_decoder_nbit_uint(stream, 3_i32 as usize, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                            message.encoding =
                                Some(exi_basetypes_decoder_characters::<65>(stream, len)?);
                            grammar_id = 103_i32;
                        }
                        1 => {
                            let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                            message.id = Some(exi_basetypes_decoder_characters::<65>(stream, len)?);
                            grammar_id = 104_i32;
                        }
                        2 => {
                            let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                            message.mime_type =
                                Some(exi_basetypes_decoder_characters::<65>(stream, len)?);
                            grammar_id = 105_i32;
                        }
                        3 => {
                            return Err(ExiError::UnknownEventForDecoding);
                        }
                        4 => {
                            return Ok(());
                        }
                        5 => {
                            message.any = Some(decode_exi_type_hex_binary::<4>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 3_i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            103 => {
                exi_basetypes_decoder_nbit_uint(stream, 3_i32 as usize, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                            message.id = Some(exi_basetypes_decoder_characters::<65>(stream, len)?);
                            grammar_id = 104_i32;
                        }
                        1 => {
                            let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                            message.mime_type =
                                Some(exi_basetypes_decoder_characters::<65>(stream, len)?);
                            grammar_id = 105_i32;
                        }
                        2 => {
                            return Err(ExiError::UnknownEventForDecoding);
                        }
                        3 => {
                            return Ok(());
                        }
                        4 => {
                            message.any = Some(decode_exi_type_hex_binary::<4>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 3_i32;
                            }
                        }
                        _ => {
                            return Err(ExiError::UnknownEventCode);
                        }
                    }
                }
            }
            104 => {
                exi_basetypes_decoder_nbit_uint(stream, 3_i32 as usize, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                            message.mime_type =
                                Some(exi_basetypes_decoder_characters::<65>(stream, len)?);
                            grammar_id = 105_i32;
                        }
                        1 => {
                            return Err(ExiError::UnknownEventForDecoding);
                        }
                        2 => {
                            return Ok(());
                        }
                        3 => {
                            message.any = Some(decode_exi_type_hex_binary::<4>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 3_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            return Err(ExiError::UnknownEventForDecoding);
                        }
                        1 => {
                            return Ok(());
                        }
                        2 => {
                            message.any = Some(decode_exi_type_hex_binary::<4>(stream)?);
                            if error == 0_i32 {
                                grammar_id = 3_i32;
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
                if error == 0_i32 {
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
// pub fn decode_iso2_supported_energy_transfer_mode(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2SupportedEnergyTransferModeType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 106 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             106 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*SupportedEnergyTransferModeType).EnergyTransferMode.len() as i32)
//                                 < 6 as i32
//                             {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         let mut value: u32 = 0;
//                                         exi_basetypes_decoder_nbit_uint(
//                                             stream,
//                                             3 as i32 as usize,
//                                             &mut value,
//                                         )?;
//                                         if error == 0 as i32 {
//                                             (*SupportedEnergyTransferModeType).EnergyTransferMode
//                                                 [(*SupportedEnergyTransferModeType)
//                                                     .EnergyTransferMode
//                                                     .len()
//                                                     as usize] = value as Iso2EnergyTransferModeType;
//                                             (*SupportedEnergyTransferModeType)
//                                                 .EnergyTransferMode
//                                                 .len() = ((*SupportedEnergyTransferModeType)
//                                                 .EnergyTransferMode
//                                                 .len())
//                                             .wrapping_add(1);
//                                             (*SupportedEnergyTransferModeType)
//                                                 .EnergyTransferMode
//                                                 .len();
//                                         }
//                                     } else {
//                                         return Err(ExiError::UnsupportedSubEvent);
//                                     }
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 107 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             107 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*SupportedEnergyTransferModeType).EnergyTransferMode.len() as i32)
//                                 < 6 as i32
//                             {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         let mut value_0: u32 = 0;
//                                         exi_basetypes_decoder_nbit_uint(
//                                             stream,
//                                             3 as i32 as usize,
//                                             &mut value_0,
//                                         )?;
//                                         if error == 0 as i32 {
//                                             (*SupportedEnergyTransferModeType).EnergyTransferMode
//                                                 [(*SupportedEnergyTransferModeType)
//                                                     .EnergyTransferMode
//                                                     .len()
//                                                     as usize] =
//                                                 value_0 as Iso2EnergyTransferModeType;
//                                             (*SupportedEnergyTransferModeType)
//                                                 .EnergyTransferMode
//                                                 .len() = ((*SupportedEnergyTransferModeType)
//                                                 .EnergyTransferMode
//                                                 .len())
//                                             .wrapping_add(1);
//                                             (*SupportedEnergyTransferModeType)
//                                                 .EnergyTransferMode
//                                                 .len();
//                                         }
//                                     } else {
//                                         return Err(ExiError::UnsupportedSubEvent);
//                                     }
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 107 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_certificate_chain(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2CertificateChainType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 108 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             108 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_uint_16(
//                                 stream,
//                                 &mut ((*message).id.clone().unwrap().len() as u16),
//                             )?;
//                             if error == 0 as i32 {
//                                 if (*message).id.clone().unwrap().len() >= 2 {
//                                     (*message).id.unwrap().len -= 2;
//                                     exi_basetypes_decoder_characters(
//                                         stream,
//                                         (*message).id.clone().unwrap().len(),
//                                         &mut (*message).id.unwrap(),
//                                         65,
//                                     )?;
//                                 } else {
//                                     return Err(ExiError::StringValuesNotSupported);
//                                 }
//                             }
//                             grammar_id = 109 as i32;
//                         }
//                         1 => {
//                             decode_exi_type_hex_binary(
//                                 stream,
//                                 &mut (*message).Certificate.len(),
//                                 &mut (*message).Certificate,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 110 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             109 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_hex_binary(
//                                 stream,
//                                 &mut (*message).Certificate.len(),
//                                 &mut (*message).Certificate,
//                                 800 as i32 as usize,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 110 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             110 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_SubCertificates(
//                                 stream,
//                                 &mut (*message).SubCertificates.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_body_base(
//     stream: &mut ExiBitstream,
//     _message: &mut Iso2BodyBaseType,
// ) -> Result<(), ExiError> {
//     let mut event_code: u32 = 0;
//     exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//     if event_code != 0 as i32 as u32 {
//         return Err(ExiError::UnknownEventCode);
//     }
//     return Ok(());
// }
pub fn decode_iso2_notification(
    stream: &mut ExiBitstream,
    message: &mut Iso2NotificationType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 111_i32;

    let mut event_code: u32 = 0;
    let error: i32 = 0;
    loop {
        match grammar_id {
            111 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1_i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0_i32 {
                                if event_code == 0_i32 as u32 {
                                    let value: Iso2FaultCodeType = Iso2FaultCodeType::Unknown;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2_i32 as usize,
                                        &mut (value as u32),
                                    )?;
                                    if error == 0_i32 {
                                        message.fault_code = value;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0_i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1_i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0_i32 {
                                    if event_code == 0_i32 as u32 {
                                        grammar_id = 112_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1_i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0_i32 {
                                if event_code == 0_i32 as u32 {
                                    let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                                    message.fault_msg =
                                        Some(exi_basetypes_decoder_characters::<65>(stream, len)?);
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0_i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1_i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0_i32 {
                                    if event_code == 0_i32 as u32 {
                                        grammar_id = 3_i32;
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
                if error == 0_i32 {
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
// pub fn decode_iso2_dc_evse_status(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2DCEvseStatusType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 113 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             113 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_uint16(
//                                 stream,
//                                 &mut (*DC_EVSEStatusType).NotificationMaxDelay,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 114 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             114 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         2 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*DC_EVSEStatusType).EVSENotification =
//                                             value as Iso2EvseNotificationType;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 115 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             115 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_0: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         3 as i32 as usize,
//                                         &mut value_0,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*DC_EVSEStatusType).EVSEIsolationStatus =
//                                             Some(value_0 as Iso2IsolationLevelType);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 116 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         1 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_1: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         4 as i32 as usize,
//                                         &mut value_1,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*DC_EVSEStatusType).EVSEStatusCode =
//                                             value_1 as Iso2DcEvseStatusCodeType;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 3 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             116 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_2: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         4 as i32 as usize,
//                                         &mut value_2,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*DC_EVSEStatusType).EVSEStatusCode =
//                                             value_2 as Iso2DcEvseStatusCodeType;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 3 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_selected_service_list(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2SelectedServiceListType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 117 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             117 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*message).SelectedService.len() as i32) < 16 as i32 {
//                                 let fresh18 = (*message).SelectedService.len();
//                                 (*message).SelectedService.len() =
//                                     ((*message).SelectedService.len()).wrapping_add(1);
//                                 if let Some(selected_service) = (*SelectedServiceListType)
//                                     .SelectedService
//                                     .get_mut(fresh18 as usize)
//                                 {
//                                     decode_iso2_SelectedService(stream, selected_service)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             grammar_id = 118 as i32;
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             118 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*message).SelectedService.len() as i32) < 16 as i32 {
//                                 let fresh19 = (*message).SelectedService.len();
//                                 (*message).SelectedService.len() =
//                                     ((*message).SelectedService.len()).wrapping_add(1);
//                                 if let Some(selected_service) = (*SelectedServiceListType)
//                                     .SelectedService
//                                     .get_mut(fresh19 as usize)
//                                 {
//                                     decode_iso2_SelectedService(stream, selected_service)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             if ((*message).SelectedService.len() as i32) < 16 as i32 {
//                                 grammar_id = 118 as i32;
//                             } else {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_payment_option_list(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2PaymentOptionListType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 119 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             119 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*message).PaymentOption.len() as i32) < 2 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         let mut value: u32 = 0;
//                                         exi_basetypes_decoder_nbit_uint(
//                                             stream,
//                                             1 as i32 as usize,
//                                             &mut value,
//                                         )?;
//                                         if error == 0 as i32 {
//                                             (*message).PaymentOption
//                                                 [(*message).PaymentOption.len() as usize] =
//                                                 value as Iso2PaymentOptionType;
//                                             (*message).PaymentOption.len() =
//                                                 ((*message).PaymentOption.len()).wrapping_add(1);
//                                             (*message).PaymentOption.len();
//                                         }
//                                     } else {
//                                         return Err(ExiError::UnsupportedSubEvent);
//                                     }
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 120 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             120 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*message).payment_option.len() as i32) < 2 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         let mut value_0: u32 = 0;
//                                         exi_basetypes_decoder_nbit_uint(
//                                             stream,
//                                             1 as i32 as usize,
//                                             &mut value_0,
//                                         )?;
//                                         if error == 0 as i32 {
//                                             (*message).payment_option
//                                                 [(*message).payment_option.len() as usize] =
//                                                 value_0 as Iso2PaymentOptionType;
//                                             (*message).payment_option.len() =
//                                                 ((*message).payment_option.len()).wrapping_add(1);
//                                             (*message).payment_option.len();
//                                         }
//                                     } else {
//                                         return Err(ExiError::UnsupportedSubEvent);
//                                     }
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 3 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
pub fn decode_iso2_signature(
    stream: &mut ExiBitstream,
    message: &mut Iso2SignatureType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 121_i32;

    let mut event_code: u32 = 0;
    let error: i32 = 0;
    loop {
        match grammar_id {
            121 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                            message.id = Some(exi_basetypes_decoder_characters::<65>(stream, len)?);
                            grammar_id = 122_i32;
                        }
                        1 => {
                            decode_iso2_signed_info(stream, &mut message.signed_info)?;
                            if error == 0_i32 {
                                grammar_id = 123_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            decode_iso2_signed_info(stream, &mut message.signed_info)?;
                            if error == 0_i32 {
                                grammar_id = 123_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            decode_iso2_signature_value(stream, &mut message.signature_value)?;
                            if error == 0_i32 {
                                grammar_id = 124_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            decode_iso2_key_info(stream, message.key_info.as_mut().unwrap())?;
                            if error == 0_i32 {
                                grammar_id = 126_i32;
                            }
                        }
                        1 => {
                            decode_iso2_object(stream, message.object.as_mut().unwrap())?;
                            if error == 0_i32 {
                                grammar_id = 125_i32;
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
                if error == 0_i32 {
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            decode_iso2_object(stream, message.object.as_mut().unwrap())?;
                            if error == 0_i32 {
                                grammar_id = 127_i32;
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
                if error == 0_i32 {
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
                if error == 0_i32 {
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

// pub fn decode_iso2_charging_profile(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2ChargingProfileType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 128 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             128 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*message).profile_entry.len() as i32) < 24 as i32 {
//                                 let fresh20 = (*message).profile_entry.len();
//                                 if let Some(entry) = (*message)
//                                     .profile_entry
//                                     .get_mut(fresh20 as usize)
//                                 {
//                                     decode_iso2_profile_entry(stream, entry)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             grammar_id = 129 as i32;
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             129 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*message).profile_entry.len() as i32) < 24 as i32 {
//                                 let fresh21 = (*message).profile_entry.len();
//                                 if let Some(entry) = (*message)
//                                     .profile_entry
//                                     .get_mut(fresh21 as usize)
//                                 {
//                                     decode_iso2_profile_entry(stream, entry)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             if ((*message).profile_entry.len() as i32) < 24 as i32 {
//                                 grammar_id = 129 as i32;
//                             } else {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_service_parameter_list(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2ServiceParameterListType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 130 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             130 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*message).parameter_set.len() as i32) < 5 as i32 {
//                                 let fresh22 = (*message).parameter_set.len();
//                                 if let Some(param_set) = (*message)
//                                     .parameter_set
//                                     .get_mut(fresh22 as usize)
//                                 {
//                                     decode_iso2_parameter_set(stream, param_set)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             grammar_id = 131 as i32;
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             131 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*message).parameter_set.len() as i32) < 5 as i32 {
//                                 let fresh23 = (*message).parameter_set.len();
//                                 if let Some(param_set) = (*message)
//                                     .parameter_set
//                                     .get_mut(fresh23 as usize)
//                                 {
//                                     decode_iso2_parameter_set(stream, param_set)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             if ((*message).parameter_set.len() as i32) < 255 as i32 {
//                                 grammar_id = 131 as i32;
//                             } else {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_list_of_root_certificate_ids(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2ListOfRootCertificateIDsType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 132 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             132 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*message).root_certificate_id.len() as i32) < 5 as i32 {
//                                 let fresh24 = (*message).root_certificate_id.len();
//                                 if let Some(root_cert) = (*message)
//                                     .root_certificate_id
//                                     .get_mut(fresh24 as usize)
//                                 {
//                                     decode_iso2_x509_issuer_serial(stream, root_cert)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             grammar_id = 133 as i32;
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             133 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*message).root_certificate_id.len() as i32) < 5 as i32 {
//                                 let fresh25 = (*message).root_certificate_id.len();
//                                 if let Some(root_cert) = (*message)
//                                     .root_certificate_id
//                                     .get_mut(fresh25 as usize)
//                                 {
//                                     decode_iso2_x509_issuer_serial(stream, root_cert)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             if ((*message).root_certificate_id.len() as i32) < 20 as i32 {
//                                 grammar_id = 133 as i32;
//                             } else {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_ac_ev_charge_parameter(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2ACEVChargeParameterType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 134 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             134 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_uint32(
//                                 stream,
//                                 &mut (*message).departure_time.unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 135 as i32;
//                             }
//                         }
//                         1 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).e_amount,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 136 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             135 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).e_amount,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 136 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             136 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).ev_max_voltage,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 137 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             137 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).ev_max_current,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 138 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             138 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).ev_min_current,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_dc_ev_charge_parameter(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2DCEVChargeParameterType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 139 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             139 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_uint32(
//                                 stream,
//                                 &mut (*message).departure_time.as_mut().clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 140 as i32;
//                             }
//                         }
//                         1 => {
//                             decode_iso2_dc_ev_status(
//                                 stream,
//                                 &mut (*message).dc_ev_status,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 141 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             140 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_dc_ev_status(
//                                 stream,
//                                 &mut (*message).dc_ev_status,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 141 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             141 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).ev_maximum_current_limit,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 142 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             142 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message)
//                                     .ev_maximum_power_limit
//                                     .as_mut()
//                                     .unwrap()
//                                     .clone(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 143 as i32;
//                             }
//                         }
//                         1 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).ev_maximum_voltage_limit,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 144 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             143 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).ev_maximum_voltage_limit,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 144 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             144 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message)
//                                     .ev_energy_capacity
//                                     .as_mut()
//                                     .unwrap()
//                                     .clone(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 145 as i32;
//                             }
//                         }
//                         1 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).ev_energy_request.as_mut().clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 146 as i32;
//                             }
//                         }
//                         2 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         7 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).full_soc = Some(value as i8);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 147 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         3 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_0: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         7 as i32 as usize,
//                                         &mut value_0,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).bulk_soc = Some(value_0 as i8);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 3 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         4 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             145 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).ev_energy_request.as_mut().clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 146 as i32;
//                             }
//                         }
//                         1 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_1: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         7 as i32 as usize,
//                                         &mut value_1,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).full_soc = Some(value_1 as i8);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 147 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         2 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_2: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         7 as i32 as usize,
//                                         &mut value_2,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).bulk_soc = Some(value_2 as i8);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 3 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         3 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             146 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_3: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         7 as i32 as usize,
//                                         &mut value_3,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).full_soc = Some(value_3 as i8);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 147 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         1 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_4: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         7 as i32 as usize,
//                                         &mut value_4,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).bulk_soc = Some(value_4 as i8);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 3 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         2 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             147 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_5: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         7 as i32 as usize,
//                                         &mut value_5,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).bulk_soc = Some(value_5 as i8);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 3 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_ev_charge_parameter(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2EVChargeParameterType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 148 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             148 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_uint32(
//                                 stream,
//                                 &mut (*message).departure_time.as_mut().clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 149 as i32;
//                             }
//                         }
//                         1 => {
//                             decode_iso2_ac_ev_charge_parameter(
//                                 stream,
//                                 &mut (*message).ac_ev_charge_parameter,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 150 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             149 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_ac_ev_charge_parameter(
//                                 stream,
//                                 &mut (*message).ac_ev_charge_parameter,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 150 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             150 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_dc_ev_charge_parameter(
//                                 stream,
//                                 &mut (*message).dc_ev_charge_parameter,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_sa_schedules(
//     stream: &mut ExiBitstream,
//     _message: &mut Iso2SASchedulesType,
// ) -> Result<(), ExiError> {
//     let mut event_code: u32 = 0;
//     exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//     if event_code != 0 as i32 as u32 {
//         return Err(ExiError::UnknownEventCode);
//     }
//     return Ok(());
// }
// pub fn decode_iso2_sa_schedule_list(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2SAScheduleListType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 151 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             151 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*message).sa_schedule_tuple.len() as i32) < 3 as i32 {
//                                 let fresh26 = (*message).sa_schedule_tuple.len();
//                                 if let Some(tuple) = (*message)
//                                     .sa_schedule_tuple
//                                     .get_mut(fresh26 as usize)
//                                 {
//                                     decode_iso2_sa_schedule_tuple(stream, tuple)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             grammar_id = 152 as i32;
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             152 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*message).sa_schedule_tuple.len() as i32) < 3 as i32 {
//                                 let fresh27 = (*message).sa_schedule_tuple.len();
//                                 if let Some(tuple) = (*message)
//                                     .sa_schedule_tuple
//                                     .get_mut(fresh27 as usize)
//                                 {
//                                     decode_iso2_sa_schedule_tuple(stream, tuple)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             if ((*message).sa_schedule_tuple.len() as i32) < 3 as i32 {
//                                 grammar_id = 152 as i32;
//                             } else {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_charge_service(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2ChargeServiceType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 153 as i32;
//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             153 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_uint16(stream, &mut (*message).service_id)?;
//                             grammar_id = 154 as i32;
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             154 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     exi_basetypes_decoder_uint_16(
//                                         stream,
//                                         &mut ((*message).service_name.clone().clone().unwrap().len() as u16),
//                                     )?;
//                                     if error == 0 as i32 {
//                                         if (*message).service_name.clone().clone().unwrap().len() >= 2 {
//                                             exi_basetypes_decoder_characters(
//                                                 stream,
//                                                 (*message).service_name.clone().clone().unwrap().len(),
//                                                 &mut (*message).service_name.clone().unwrap(),
//                                             )?;
//                                         } else {
//                                             return Err(ExiError::StringValuesNotSupported);
//                                         }
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 155 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         1 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         2 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).service_category =
//                                             value as Iso2ServiceCategoryType;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 156 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             155 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_0: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         2 as i32 as usize,
//                                         &mut value_0,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).service_category =
//                                             value_0 as Iso2ServiceCategoryType;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 156 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             156 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     exi_basetypes_decoder_uint_16(
//                                         stream,
//                                         &mut ((*message).service_scope.clone().unwrap().len() as u16),
//                                     )?;
//                                     if error == 0 as i32 {
//                                         if (*message).service_scope.clone().unwrap().len() >= 2 {
//                                             exi_basetypes_decoder_characters(
//                                                 stream,
//                                                 (*message).service_scope.clone().unwrap().len() as usize,
//                                                 &mut (*message).service_scope.unwrap(),
//                                             )?;
//                                         } else {
//                                             return Err(ExiError::StringValuesNotSupported);
//                                         }
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 157 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         1 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_1: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value_1,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).free_service = value_1 as i32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 158 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             157 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_2: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value_2,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).free_service = value_2 as i32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 158 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             158 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_supported_energy_transfer_mode(
//                                 stream,
//                                 &mut (*message).supported_energy_transfer_mode,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_ev_power_delivery_parameter(
//     stream: &mut ExiBitstream,
//     _message: &mut Iso2EVPowerDeliveryParameterType,
// ) -> Result<(), ExiError> {
//     let mut event_code: u32 = 0;
//     exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//     if event_code != 0 as i32 as u32 {
//         return Err(ExiError::UnknownEventCode);
//     }
//     return Ok(());
// }
// pub fn decode_iso2_dc_ev_power_delivery_parameter(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2DCEVPowerDeliveryParameterType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 159 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             159 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_dc_ev_status(
//                                 stream,
//                                 &mut (*message).dc_ev_status,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 160 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             160 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).bulk_charging_complete =
//                                             Some(value as i32);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 161 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         1 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_0: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value_0,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).charging_complete =
//                                             value_0 as i32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 3 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             161 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_1: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value_1,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).charging_complete =
//                                             value_1 as i32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 3 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_contract_signature_encrypted_private_key(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2ContractSignatureEncryptedPrivateKeyType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 162 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             162 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_uint_16(
//                                 stream,
//                                 &mut ((*message).id.len() as u16),
//                             )?;
//                             if error == 0 as i32 {
//                                 if (*message).id.len() >= 2 {
//                                     exi_basetypes_decoder_characters(
//                                         stream,
//                                         (*message).id.len() as usize,
//                                         &mut (*message).id,
//                                     )?;
//                                 } else {
//                                     return Err(ExiError::StringValuesNotSupported);
//                                 }
//                             }
//                             grammar_id = 163 as i32;
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             163 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_uint_16(
//                                 stream,
//                                 &mut ((*message).content.len() as u16),
//                             )?;
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_bytes(
//                                     stream,
//                                     (*message).content.len() as usize,
//                                     &mut (*message).content,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     grammar_id = 3 as i32;
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_evse_charge_parameter(
//     stream: &mut ExiBitstream,
//     _message: &mut Iso2EVSEChargeParameterType,
// ) -> Result<(), ExiError> {
//     let mut event_code: u32 = 0;
//     exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//     if event_code != 0 as i32 as u32 {
//         return Err(ExiError::UnknownEventCode);
//     }
//     return Ok(());
// }
// pub fn decode_iso2_dc_evse_charge_parameter(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2DCEVSEChargeParameterType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 164 as i32;
//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             164 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_dc_evse_status(
//                                 stream,
//                                 &mut (*message).dc_evse_status,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 165 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             165 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).evse_maximum_current_limit,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 166 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             166 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).evse_maximum_power_limit,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 167 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             167 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).evse_maximum_voltage_limit,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 168 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             168 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).evse_minimum_current_limit,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 169 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             169 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).evse_minimum_voltage_limit,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 170 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             170 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message)
//                                     .evse_current_regulation_tolerance
//                                     .as_mut()
//                                     .unwrap()
//                                     .clone(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 171 as i32;
//                             }
//                         }
//                         1 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).evse_peak_current_ripple,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 172 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             171 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).evse_peak_current_ripple,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 172 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             172 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message)
//                                     .evse_energy_to_be_delivered
//                                     .as_mut()
//                                     .unwrap()
//                                     .clone(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_service_list(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2ServiceListType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 173 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             173 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*message).service.len() as i32) < 8 as i32 {
//                                 let fresh28 = (*message).service.len();
//                                 if let Some(service) = (*message).service.get_mut(fresh28 as usize)
//                                 {
//                                     decode_iso2_service(stream, service)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             grammar_id = 174 as i32;
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             174 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             if ((*message).service.len() as i32) < 8 as i32 {
//                                 let fresh29 = (*message).service.len();
//                                 if let Some(service) = (*message).service.get_mut(fresh29 as usize)
//                                 {
//                                     decode_iso2_service(stream, service)?;
//                                 } else {
//                                     return Err(ExiError::ArrayOutOfBounds);
//                                 }
//                             } else {
//                                 return Err(ExiError::ArrayOutOfBounds);
//                             }
//                             if ((*message).service.len() as i32) < 8 as i32 {
//                                 grammar_id = 174 as i32;
//                             } else {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_diffie_hellman_publickey(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2DiffieHellmanPublickeyType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 175 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             175 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_uint_16(
//                                 stream,
//                                 &mut ((*message).id.len() as u16),
//                             )?;
//                             if error == 0 as i32 {
//                                 if (*message).id.len() >= 2 {
//                                     exi_basetypes_decoder_characters(
//                                         stream,
//                                         (*message).id.len() as usize,
//                                         &mut (*message).id,
//                                     )?;
//                                 } else {
//                                     return Err(ExiError::StringValuesNotSupported);
//                                 }
//                             }
//                             grammar_id = 176 as i32;
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             176 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_uint_16(
//                                 stream,
//                                 &mut ((*message).content.len() as u16),
//                             )?;
//                             exi_basetypes_decoder_bytes(
//                                 stream,
//                                 message.content.len(),
//                                 &mut message.content,
//                             )?;
//                             grammar_id = 3 as i32;
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_emaid(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2EMAIDType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 177 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             177 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_uint_16(
//                                 stream,
//                                 &mut ((*message).id.len() as u16),
//                             )?;
//                             if error == 0 as i32 {
//                                 if (*message).id.len() >= 2 {
//                                     exi_basetypes_decoder_characters(
//                                         stream,
//                                         (*message).id.len() as usize,
//                                         &mut (*message).id,
//                                     )?;
//                                 } else {
//                                     return Err(ExiError::StringValuesNotSupported);
//                                 }
//                             }
//                             grammar_id = 178 as i32;
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             178 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_uint_16(
//                                 stream,
//                                 &mut ((*message).content.len() as u16),
//                             )?;
//                             if error == 0 as i32 {
//                                 if (*message).content.len() >= 2 {
//                                     exi_basetypes_decoder_characters(
//                                         stream,
//                                         (*message).content.len(),
//                                         &mut (*message).content,
//                                     )?;
//                                 } else {
//                                     return Err(ExiError::StringValuesNotSupported);
//                                 }
//                             }
//                             grammar_id = 3 as i32;
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_ac_evse_status(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2ACEVSEStatusType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 179 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             179 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_uint16(
//                                 stream,
//                                 &mut (*message).notification_max_delay,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 180 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             180 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         2 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).evse_notification =
//                                             value as Iso2EvseNotificationType;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 181 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             181 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_0: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value_0,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).rcd = value_0 as i32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 3 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_evse_status(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2EVSEStatusType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 182 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             182 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_uint16(stream, &mut (*message).notification_max_delay)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 183 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             183 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         2 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).evse_notification =
//                                             value as Iso2EvseNotificationType;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 184 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             184 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_ac_evse_status(stream, &mut (*message).ac_evse_status)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 185 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             185 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_dc_evse_status(stream, &mut (*message).dc_evse_status)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_ac_evse_charge_parameter(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2ACEVSEChargeParameterType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 186 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             186 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_ac_evse_status(
//                                 stream,
//                                 &mut (*message).ac_evse_status,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 187 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             187 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).evse_nominal_voltage,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 188 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             188 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).evse_max_current,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_meter_info(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2MeterInfoType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 189 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             189 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     exi_basetypes_decoder_uint_16(
//                                         stream,
//                                         &mut ((*message).meter_id.len() as u16),
//                                     )?;
//                                     if error == 0 as i32 {
//                                         if (*message).meter_id.len() >= 2 {
//                                             exi_basetypes_decoder_characters(
//                                                 stream,
//                                                 (*message).meter_id.len() as usize,
//                                                 &mut (*message).meter_id,
//                                             )?;
//                                         } else {
//                                             return Err(ExiError::StringValuesNotSupported);
//                                         }
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 190 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             190 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_uint64(
//                                 stream,
//                                 &mut (*message).meter_reading.as_mut().clone().unwrap(),
//                             )?;
//                             grammar_id = 191 as i32;
//                         }
//                         1 => {
//                             decode_exi_type_hex_binary(
//                                 stream,
//                                 &mut (*message).sig_meter_reading.as_mut().clone().unwrap().len(),
//                                 &mut (*message).sig_meter_reading.as_mut().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 192 as i32;
//                             }
//                         }
//                         2 => {
//                             decode_exi_type_integer16(
//                                 stream,
//                                 &mut (*message).meter_status.as_mut().clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 193 as i32;
//                             }
//                         }
//                         3 => {
//                             decode_exi_type_integer64(
//                                 stream,
//                                 &mut (*message).t_meter.as_mut().clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         4 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             191 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_hex_binary(
//                                 stream,
//                                 &mut (*message).sig_meter_reading.as_mut().clone().unwrap().len(),
//                                 &mut (*message).sig_meter_reading.as_mut().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 192 as i32;
//                             }
//                         }
//                         1 => {
//                             decode_exi_type_integer16(
//                                 stream,
//                                 &mut (*message).meter_status.as_mut().clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 193 as i32;
//                             }
//                         }
//                         2 => {
//                             decode_exi_type_integer64(
//                                 stream,
//                                 &mut (*message).t_meter.as_mut().clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         3 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             192 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_integer16(
//                                 stream,
//                                 &mut (*message).meter_status.as_mut().clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 193 as i32;
//                             }
//                         }
//                         1 => {
//                             decode_exi_type_integer64(
//                                 stream,
//                                 &mut (*message).t_meter.as_mut().clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         2 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             193 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_integer64(
//                                 stream,
//                                 &mut (*message).t_meter.unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
pub fn decode_iso2_message_header(
    stream: &mut ExiBitstream,
    message: &mut Iso2MessageHeaderType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 194_i32;

    let mut event_code: u32 = 0;
    let error: i32 = 0;
    loop {
        match grammar_id {
            194 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            message.session_id = decode_exi_type_hex_binary::<8>(stream)?;
                            if error == 0_i32 {
                                grammar_id = 195_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            // If notification is None skip
                            if let Some(ref mut notification) = message.notification {
                                decode_iso2_notification(stream, notification)?;
                            }
                            if error == 0_i32 {
                                grammar_id = 196_i32;
                            }
                        }
                        1 => {
                            if let Some(message_signature) = &mut message.signature {
                                decode_iso2_signature(stream, message_signature)?;
                            }
                            if error == 0_i32 {
                                grammar_id = 3_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            if let Some(ref mut signature) = message.signature {
                                decode_iso2_signature(stream, signature)?;
                            }
                            if error == 0_i32 {
                                grammar_id = 3_i32;
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
                if error == 0_i32 {
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

// pub fn decode_iso2_power_delivery_req(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2PowerDeliveryReqType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 197 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             197 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         2 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).charge_progress = value as Iso2ChargeProgressType;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 198 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             198 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_0: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         8 as i32 as usize,
//                                         &mut value_0,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).sa_schedule_tuple_id =
//                                             value_0.wrapping_add(1) as u8;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 199 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             199 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_charging_profile(
//                                 stream,
//                                 &mut (*message).charging_profile.as_mut().clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 200 as i32;
//                             }
//                         }
//                         1 => {
//                             decode_iso2_dc_ev_power_delivery_parameter(
//                                 stream,
//                                 &mut (*message).dc_ev_power_delivery_parameter.as_mut().clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         2 => {
//                             decode_iso2_ev_power_delivery_parameter(
//                                 stream,
//                                 &mut (*message).ev_power_delivery_parameter.as_mut().clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         3 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             200 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_dc_ev_power_delivery_parameter(
//                                 stream,
//                                 &mut (*message).dc_ev_power_delivery_parameter.as_mut().clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         1 => {
//                             decode_iso2_ev_power_delivery_parameter(
//                                 stream,
//                                 &mut (*message).ev_power_delivery_parameter.as_mut().clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         2 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_current_demand_res(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2CurrentDemandResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 201 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             201 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         5 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).response_code = value as u32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 202 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             202 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_dc_evse_status(stream, &mut (*message).dc_evse_status)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 203 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             203 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(stream, &mut (*message).evse_present_voltage)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 204 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             204 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(stream, &mut (*message).evse_present_current)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 205 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             205 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_0: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value_0,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).evse_current_limit_achieved = value_0 as i32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 206 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             206 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_1: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value_1,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).evse_voltage_limit_achieved = value_1 as i32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 207 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             207 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_2: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value_2,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).evse_power_limit_achieved = value_2 as i32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 208 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             208 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).evse_maximum_voltage_limit.as_mut().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 209 as i32;
//                             }
//                         }
//                         1 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).evse_maximum_current_limit.as_mut().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 210 as i32;
//                             }
//                         }
//                         2 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).evse_maximum_power_limit.as_mut().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 211 as i32;
//                             }
//                         }
//                         3 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     exi_basetypes_decoder_uint_16(
//                                         stream,
//                                         &mut ((*message).evse_id.len() as u16),
//                                     )?;
//                                     if error == 0 as i32 {
//                                         if (*message).evse_id.len() >= 2 {
//                                             exi_basetypes_decoder_characters(
//                                                 stream,
//                                                 (*message).evse_id.len() as usize,
//                                                 &mut (*message).evse_id,
//                                             )?;
//                                         } else {
//                                             return Err(ExiError::StringValuesNotSupported);
//                                         }
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 212 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             209 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).evse_maximum_current_limit.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 210 as i32;
//                             }
//                         }
//                         1 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).evse_maximum_power_limit.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 211 as i32;
//                             }
//                         }
//                         2 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     exi_basetypes_decoder_uint_16(
//                                         stream,
//                                         &mut ((*message).evse_id.len() as u16),
//                                     )?;
//                                     if error == 0 as i32 {
//                                         if (*message).evse_id.len() >= 2 {
//                                             exi_basetypes_decoder_characters(
//                                                 stream,
//                                                 (*message).evse_id.len() as usize,
//                                                 &mut (*message).evse_id,
//                                             )?;
//                                         } else {
//                                             return Err(ExiError::StringValuesNotSupported);
//                                         }
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 212 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             210 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).evse_maximum_power_limit.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 211 as i32;
//                             }
//                         }
//                         1 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     exi_basetypes_decoder_uint_16(
//                                         stream,
//                                         &mut ((*message).evse_id.len() as u16),
//                                     )?;
//                                     if error == 0 as i32 {
//                                         if (*message).evse_id.len() >= 2 {
//                                             exi_basetypes_decoder_characters(
//                                                 stream,
//                                                 (*message).evse_id.len() as usize,
//                                                 &mut (*message).evse_id,
//                                             )?;
//                                         } else {
//                                             return Err(ExiError::StringValuesNotSupported);
//                                         }
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 212 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             211 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     exi_basetypes_decoder_uint_16(
//                                         stream,
//                                         &mut ((*message).evse_id.len() as u16),
//                                     )?;
//                                     if error == 0 as i32 {
//                                         if (*message).evse_id.len() >= 2 {
//                                             exi_basetypes_decoder_characters(
//                                                 stream,
//                                                 (*message).evse_id.len(),
//                                                 &mut (*message).evse_id,
//                                             )?;
//                                         } else {
//                                             return Err(ExiError::StringValuesNotSupported);
//                                         }
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 212 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             212 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_3: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         8 as i32 as usize,
//                                         &mut value_3,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).sa_schedule_tuple_id =
//                                             value_3.wrapping_add(1 as i32 as u32) as u8;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 213 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             213 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_meter_info(
//                                 stream,
//                                 &mut (*message).meter_info.as_mut().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 214 as i32;
//                             }
//                         }
//                         1 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_4: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value_4,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).receipt_required = Some(value_4 as i32);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 3 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         2 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             214 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_5: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value_5,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).receipt_required = Some(value_5 as i32);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 3 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_charging_status_res(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2ChargingStatusResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 215 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             215 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         5 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).response_code = value as u32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 216 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             216 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     exi_basetypes_decoder_uint_16(
//                                         stream,
//                                         &mut ((*message).evse_id.len() as u16),
//                                     )?;
//                                     if error == 0 as i32 {
//                                         if (*message).evse_id.len() >= 2 {
//                                             exi_basetypes_decoder_characters(
//                                                 stream,
//                                                 (*message).evse_id.len(),
//                                                 &mut (*message).evse_id,
//                                             )?;
//                                         } else {
//                                             return Err(ExiError::StringValuesNotSupported);
//                                         }
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 217 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             217 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_0: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         8 as i32 as usize,
//                                         &mut value_0,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).sa_schedule_tuple_id =
//                                             value_0.wrapping_add(1 as i32 as u32) as u8;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 218 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             218 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).evse_max_current.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 219 as i32;
//                             }
//                         }
//                         1 => {
//                             decode_iso2_meter_info(
//                                 stream,
//                                 &mut (*message).meter_info.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 220 as i32;
//                             }
//                         }
//                         2 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_1: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value_1,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).receipt_required = Some(value_1 as i32);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 221 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         3 => {
//                             decode_iso2_ac_evse_status(stream, &mut (*message).ac_evse_status)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             219 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_meter_info(
//                                 stream,
//                                 &mut (*message).meter_info.as_mut().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 220 as i32;
//                             }
//                         }
//                         1 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_2: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value_2,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).receipt_required = Some(value_2 as i32);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 221 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         2 => {
//                             decode_iso2_ac_evse_status(stream, &mut (*message).ac_evse_status)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             220 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_3: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value_3,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).receipt_required = Some(value_3 as i32);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 221 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         1 => {
//                             decode_iso2_ac_evse_status(stream, &mut (*message).ac_evse_status)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             221 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_ac_evse_status(stream, &mut (*message).ac_evse_status)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_authorization_req(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2AuthorizationReqType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 222 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             222 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_uint_16(
//                                 stream,
//                                 &mut ((*message).id.clone().unwrap().len() as u16),
//                             )?;
//                             if error == 0 as i32 {
//                                 if (*message).id.clone().unwrap().len() >= 2 {
//                                     exi_basetypes_decoder_characters(
//                                         stream,
//                                         (*message).id.clone().unwrap().len(),
//                                         &mut (*message).id.as_mut().unwrap()
//                                     )?;
//                                 } else {
//                                     return Err(ExiError::StringValuesNotSupported);
//                                 }
//                             }
//                             grammar_id = 223 as i32;
//                         }
//                         1 => {
//                             decode_exi_type_hex_binary(
//                                 stream,
//                                 &mut (*message).gen_challenge.clone().unwrap().len(),
//                                 &mut (*message).gen_challenge.as_mut().unwrap()
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         2 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             223 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_hex_binary(
//                                 stream,
//                                 &mut (*message).gen_challenge.clone().unwrap().len(),
//                                 &mut (*message).gen_challenge.as_mut().as_mut().unwrap()
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_pre_charge_req(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2PreChargeReqType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 224 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             224 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_dc_ev_status(
//                                 stream,
//                                 &mut (*message).dc_ev_status.as_mut().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 225 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             225 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).ev_target_voltage.as_mut().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 226 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             226 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).ev_target_current.as_mut().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_service_detail_res(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2ServiceDetailResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 227 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             227 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         5 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).response_code = value as u32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 228 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             228 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_uint16(stream, &mut (*message).service_id)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 229 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             229 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_service_parameter_list(
//                                 stream,
//                                 &mut (*message).service_parameter_list.as_mut().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_payment_service_selection_res(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2PaymentServiceSelectionResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 230 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             230 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         5 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).response_code = value as u32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 3 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_certificate_update_req(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2CertificateUpdateReqType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 231 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             231 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_uint_16(
//                                 stream,
//                                 &mut ((*message).id.len() as u16),
//                             )?;
//                             if error == 0 as i32 {
//                                 if (*message).id.len() >= 2 {
//                                     exi_basetypes_decoder_characters(
//                                         stream,
//                                         (*message).id.len() as usize,
//                                         &mut (*message).id,
//                                     )?;
//                                 } else {
//                                     return Err(ExiError::StringValuesNotSupported);
//                                 }
//                             }
//                             grammar_id = 232 as i32;
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             232 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_certificate_chain(
//                                 stream,
//                                 &mut (*message).contract_signature_cert_chain,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 233 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             233 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     exi_basetypes_decoder_uint_16(
//                                         stream,
//                                         &mut ((*message).e_maid.len() as u16),
//                                     )?;
//                                     if error == 0 as i32 {
//                                         if (*message).e_maid.len() >= 2 {
//                                             exi_basetypes_decoder_characters(
//                                                 stream,
//                                                 (*message).e_maid.len(),
//                                                 &mut (*message).e_maid,
//                                             )?;
//                                         } else {
//                                             return Err(ExiError::StringValuesNotSupported);
//                                         }
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 234 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             234 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_list_of_root_certificate_ids(
//                                 stream,
//                                 &mut (*message).list_of_root_certificate_ids,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
pub fn decode_iso2_session_setup_res(
    stream: &mut ExiBitstream,
    message: &mut Iso2SessionSetupResType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 235_i32;

    let mut event_code: u32 = 0;
    let error: i32 = 0;
    loop {
        match grammar_id {
            235 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1_i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0_i32 {
                                if event_code == 0_i32 as u32 {
                                    let value: Iso2ResponseCodeType = Iso2ResponseCodeType::Unknown;
                                    exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5_i32 as usize,
                                        &mut (value as u32),
                                    )?;
                                    if error == 0_i32 {
                                        message.response_code = value;
                                    }
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0_i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1_i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0_i32 {
                                    if event_code == 0_i32 as u32 {
                                        grammar_id = 236_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            exi_basetypes_decoder_nbit_uint(
                                stream,
                                1_i32 as usize,
                                &mut event_code,
                            )?;
                            if error == 0_i32 {
                                if event_code == 0_i32 as u32 {
                                    let len = exi_basetypes_decoder_uint_16(stream)? as usize;
                                    message.evse_id =
                                        exi_basetypes_decoder_characters::<38>(stream, len)?;
                                } else {
                                    return Err(ExiError::UnsupportedSubEvent);
                                }
                            }
                            if error == 0_i32 {
                                exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1_i32 as usize,
                                    &mut event_code,
                                )?;
                                if error == 0_i32 {
                                    if event_code == 0_i32 as u32 {
                                        grammar_id = 237_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            if let Some(ref mut time_stamp) = message.evse_time_stamp {
                                decode_exi_type_integer64(stream, time_stamp)?;
                            }
                            grammar_id = 3_i32;
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
                if error == 0_i32 {
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

// pub fn decode_iso2_certificate_installation_req(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2CertificateInstallationReqType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 238 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             238 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_uint_16(
//                                 stream,
//                                 &mut ((*message).id.len() as u16),
//                             )?;
//                             if error == 0 as i32 {
//                                 if (*message).id.len() >= 2 {
//                                     exi_basetypes_decoder_characters(
//                                         stream,
//                                         (*message).id.len(),
//                                         &mut (*message).id,
//                                     )?;
//                                 } else {
//                                     return Err(ExiError::StringValuesNotSupported);
//                                 }
//                             }
//                             grammar_id = 239 as i32;
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             239 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_hex_binary(
//                                 stream,
//                                 &mut (*message).oem_provisioning_cert.len(),
//                                 &mut (*message).oem_provisioning_cert,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 240 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             240 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_list_of_root_certificate_ids(
//                                 stream,
//                                 &mut (*message).list_of_root_certificate_ids,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_certificate_installation_res(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2CertificateInstallationResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 241 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             241 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         5 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).response_code = value as u32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 242 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             242 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_certificate_chain(
//                                 stream,
//                                 &mut (*message).sa_provisioning_certificate_chain,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 243 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             243 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_certificate_chain(
//                                 stream,
//                                 &mut (*message).contract_signature_cert_chain,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 244 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             244 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_contract_signature_encrypted_private_key(
//                                 stream,
//                                 &mut (*message).contract_signature_encrypted_private_key,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 245 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             245 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_diffie_hellman_publickey(
//                                 stream,
//                                 &mut (*message).dh_public_key,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 246 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             246 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_emaid(stream, &mut (*message).e_maid)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_welding_detection_res(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2WeldingDetectionResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 247 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             247 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         5 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).response_code = value as u32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 248 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             248 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_dc_evse_status(stream, &mut (*message).dc_evse_status)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 249 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             249 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(stream, &mut (*message).evse_present_voltage)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_current_demand_req(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2CurrentDemandReqType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 250 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             250 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_dc_ev_status(stream, &mut (*message).dc_ev_status)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 251 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             251 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(stream, &mut (*message).ev_target_current)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 252 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             252 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).ev_maximum_voltage_limit.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 253 as i32;
//                             }
//                         }
//                         1 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).ev_maximum_current_limit.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 254 as i32;
//                             }
//                         }
//                         2 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).ev_maximum_power_limit.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 255 as i32;
//                             }
//                         }
//                         3 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).bulk_charging_complete = Some(value as i32);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 256 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         4 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_0: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value_0,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).charging_complete = value_0 as i32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 257 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             253 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).ev_maximum_current_limit.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 254 as i32;
//                             }
//                         }
//                         1 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).ev_maximum_power_limit.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 255 as i32;
//                             }
//                         }
//                         2 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_1: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value_1,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).bulk_charging_complete = Some(value_1 as i32);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 256 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         3 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_2: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value_2,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).charging_complete = value_2 as i32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 257 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             254 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).ev_maximum_power_limit.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 255 as i32;
//                             }
//                         }
//                         1 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_3: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value_3,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).bulk_charging_complete = Some(value_3 as i32);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 256 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         2 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_4: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value_4,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).charging_complete = value_4 as i32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 257 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             255 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_5: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value_5,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).bulk_charging_complete = Some(value_5 as i32);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 256 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         1 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_6: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value_6,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).charging_complete = value_6 as i32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 257 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             256 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_7: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value_7,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).charging_complete = value_7 as i32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 257 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             257 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).remaining_time_to_full_soc.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 258 as i32;
//                             }
//                         }
//                         1 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).remaining_time_to_bulk_soc.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 259 as i32;
//                             }
//                         }
//                         2 => {
//                             decode_iso2_physical_value(stream, &mut (*message).ev_target_voltage)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             258 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).remaining_time_to_bulk_soc.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 259 as i32;
//                             }
//                         }
//                         1 => {
//                             decode_iso2_physical_value(stream, &mut (*message).ev_target_voltage)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             259 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(stream, &mut (*message).ev_target_voltage)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_pre_charge_res(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2PreChargeResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 260 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             260 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         5 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).response_code = value as u32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 261 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             261 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_dc_evse_status(stream, &mut (*message).dc_evse_status)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 262 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             262 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_physical_value(
//                                 stream,
//                                 &mut (*message).evse_present_voltage,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_certificate_update_res(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2CertificateUpdateResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 263 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             263 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         5 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).response_code = value as u32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 264 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             264 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_certificate_chain(
//                                 stream,
//                                 &mut (*message).sa_provisioning_certificate_chain,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 265 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             265 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_certificate_chain(
//                                 stream,
//                                 &mut (*message).contract_signature_cert_chain,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 266 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             266 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_contract_signature_encrypted_private_key(
//                                 stream,
//                                 &mut (*message).contract_signature_encrypted_private_key,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 267 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             267 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_diffie_hellman_publickey(
//                                 stream,
//                                 &mut (*message).dh_public_key,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 268 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             268 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_emaid(stream, &mut (*message).e_maid)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 269 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             269 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_integer16(
//                                 stream,
//                                 &mut (*message).retry_counter.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_metering_receipt_req(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2MeteringReceiptReqType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 270 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             270 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_uint_16(
//                                 stream,
//                                 &mut ((*message).id.clone().clone().unwrap().len() as u16),
//                             )?;
//                             if error == 0 as i32 {
//                                 if (*message).id.clone().clone().unwrap().len() as i32 >= 2 as i32 {
//                                     exi_basetypes_decoder_characters(
//                                         stream,
//                                         (*message).id.clone().clone().unwrap().len(),
//                                         &mut (*message).id.clone().unwrap(),
//                                     )?;
//                                 } else {
//                                     return Err(ExiError::StringValuesNotSupported);
//                                 }
//                             }
//                             grammar_id = 271 as i32;
//                         }
//                         1 => {
//                             decode_exi_type_hex_binary(
//                                 stream,
//                                 &mut (*message).session_id.len(),
//                                 &mut (*message).session_id,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 272 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             271 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_hex_binary(
//                                 stream,
//                                 &mut (*message).session_id.len(),
//                                 &mut (*message).session_id,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 272 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             272 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         8 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).sa_schedule_tuple_id =
//                                             Some(value.wrapping_add(1 as i32 as u32) as u8);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 273 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         1 => {
//                             decode_iso2_meter_info(stream, &mut (*message).meter_info)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             273 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_meter_info(stream, &mut (*message).meter_info)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_charging_status_req(
//     stream: &mut ExiBitstream,
//     _message: &mut Iso2ChargingStatusReqType,
// ) -> Result<(), ExiError> {
//     let mut event_code: u32 = 0;
//     exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//     if event_code != 0 {
//         return Err(ExiError::UnknownEventCode);
//     }

//     return Ok(());
// }
// pub fn decode_iso2_session_stop_res(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2SessionStopResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 274 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             274 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         5 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).response_code = value as u32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 3 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_charge_parameter_discovery_req(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2ChargeParameterDiscoveryReqType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 275 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             275 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_uint16(
//                                 stream,
//                                 &mut (*message).max_entries_sa_schedule_tuple.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 276 as i32;
//                             }
//                         }
//                         1 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         3 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).requested_energy_transfer_mode =
//                                             value as Iso2EnergyTransferModeType;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 277 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             276 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_0: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         3 as i32 as usize,
//                                         &mut value_0,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).requested_energy_transfer_mode =
//                                             value_0 as Iso2EnergyTransferModeType;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 277 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             277 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_ac_ev_charge_parameter(
//                                 stream,
//                                 &mut (*message).ac_ev_charge_parameter.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         1 => {
//                             decode_iso2_dc_ev_charge_parameter(
//                                 stream,
//                                 &mut (*message).dc_ev_charge_parameter.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         2 => {
//                             decode_iso2_ev_charge_parameter(
//                                 stream,
//                                 &mut (*message).ev_charge_parameter.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_cable_check_req(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2CableCheckReqType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 278 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             278 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_dc_ev_status(stream, &mut (*message).dc_ev_status)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_welding_detection_req(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2WeldingDetectionReqType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 279 as i32;
//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             279 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_dc_ev_status(stream, &mut (*message).dc_ev_status)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_power_delivery_res(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2PowerDeliveryResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 280 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             280 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         5 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).response_code = value as u32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 281 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             281 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_ac_evse_status(
//                                 stream,
//                                 &mut (*message).ac_evse_status.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         1 => {
//                             decode_iso2_dc_evse_status(
//                                 stream,
//                                 &mut (*message).dc_evse_status.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         2 => {
//                             decode_iso2_evse_status(
//                                 stream,
//                                 &mut (*message).evse_status.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_charge_parameter_discovery_res(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2ChargeParameterDiscoveryResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 282 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             282 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         5 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).response_code = value as u32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 283 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             283 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_0: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         2 as i32 as usize,
//                                         &mut value_0,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).evse_processing =
//                                             value_0 as Iso2EvseProcessingType;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 284 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             284 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_sa_schedule_list(
//                                 stream,
//                                 &mut (*message).sa_schedule_list.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 285 as i32;
//                             }
//                         }
//                         1 => {
//                             decode_iso2_sa_schedules(
//                                 stream,
//                                 &mut (*message).sa_schedules.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 285 as i32;
//                             }
//                         }
//                         2 => {
//                             decode_iso2_ac_evse_charge_parameter(
//                                 stream,
//                                 &mut (*message).ac_evse_charge_parameter.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         3 => {
//                             decode_iso2_dc_evse_charge_parameter(
//                                 stream,
//                                 &mut (*message).dc_evse_charge_parameter.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         4 => {
//                             decode_iso2_evse_charge_parameter(
//                                 stream,
//                                 &mut (*message).evse_charge_parameter.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             285 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_ac_evse_charge_parameter(
//                                 stream,
//                                 &mut (*message).ac_evse_charge_parameter.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         1 => {
//                             decode_iso2_dc_evse_charge_parameter(
//                                 stream,
//                                 &mut (*message).dc_evse_charge_parameter.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         2 => {
//                             decode_iso2_evse_charge_parameter(
//                                 stream,
//                                 &mut (*message).evse_charge_parameter.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_payment_service_selection_req(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2PaymentServiceSelectionReqType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 286 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             286 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).selected_payment_option =
//                                             value as Iso2PaymentOptionType;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 287 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             287 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_selected_service_list(
//                                 stream,
//                                 &mut (*message).selected_service_list,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_metering_receipt_res(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2MeteringReceiptResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 288 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             288 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         5 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).response_code = value as u32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 289 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             289 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_ac_evse_status(
//                                 stream,
//                                 &mut (*message).ac_evse_status.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         1 => {
//                             decode_iso2_dc_evse_status(
//                                 stream,
//                                 &mut (*message).dc_evse_status.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         2 => {
//                             decode_iso2_evse_status(
//                                 stream,
//                                 &mut (*message).evse_status.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_cable_check_res(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2CableCheckResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 290 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             290 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         5 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).response_code = value as u32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 291 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             291 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_dc_evse_status(
//                                 stream,
//                                 &mut (*message).dc_evse_status.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 292 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             292 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_0: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         2 as i32 as usize,
//                                         &mut value_0,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).evse_processing =
//                                             value_0 as Iso2EvseProcessingType;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 3 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_service_discovery_res(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2ServiceDiscoveryResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 293 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             293 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         5 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).response_code = value as u32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 294 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             294 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_payment_option_list(
//                                 stream,
//                                 &mut (*message).payment_option_list,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 295 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             295 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_charge_service(stream, &mut (*message).charge_service)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 296 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             296 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_service_list(
//                                 stream,
//                                 &mut (*message).service_list.clone().unwrap(),
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_service_detail_req(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2ServiceDetailReqType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 297 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             297 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_uint16(stream, &mut (*message).service_id)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }

pub fn decode_iso2_session_setup_req(
    stream: &mut ExiBitstream,
    message: &mut Iso2SessionSetupReqType,
) -> Result<(), ExiError> {
    let mut event_code: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
    if event_code == 0 {
        message.evcc_id = decode_exi_type_hex_binary::<6>(stream)?;
        exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
        if event_code == 0 {
            return Ok(());
        }
    }
    Err(ExiError::UnknownEventCode)
}

// pub fn decode_iso2_session_stop_req(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2SessionStopReqType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 299 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             299 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).charging_session =
//                                             value as Iso2ChargingSessionType;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 3 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_service_discovery_req(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2ServiceDiscoveryReqType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 300 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             300 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     exi_basetypes_decoder_uint_16(
//                                         stream,
//                                         &mut ((*message).service_scope.clone().clone().unwrap().len()
//                                             as u16),
//                                     )?;
//                                     if error == 0 as i32 {
//                                         if (*message).service_scope.clone().clone().unwrap().len() as i32
//                                             >= 2 as i32
//                                         {
//                                             exi_basetypes_decoder_characters(
//                                                 stream,
//                                                 (*message).service_scope.clone().clone().unwrap().len(),
//                                                 &mut (*message).service_scope.clone().unwrap(),
//                                             )?;
//                                         } else {
//                                             return Err(ExiError::StringValuesNotSupported);
//                                         }
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 301 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         1 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         2 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).service_category =
//                                             Some(value as Iso2ServiceCategoryType);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 3 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         2 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             301 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_0: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         2 as i32 as usize,
//                                         &mut value_0,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).service_category =
//                                             Some(value_0 as Iso2ServiceCategoryType);
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 3 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         1 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_authorization_res(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2AuthorizationResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 302 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             302 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         5 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).response_code = value as u32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 303 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             303 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value_0: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         2 as i32 as usize,
//                                         &mut value_0,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).evse_processing =
//                                             value_0 as Iso2EvseProcessingType;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 3 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_payment_details_req(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2PaymentDetailsReqType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 304 as i32;

//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             304 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     exi_basetypes_decoder_uint_16(
//                                         stream,
//                                         &mut ((*message).e_maid.len() as u16),
//                                     )?;
//                                     if error == 0 as i32 {
//                                         if (*message).e_maid.len() as i32 >= 2 as i32 {
//                                             exi_basetypes_decoder_characters(
//                                                 stream,
//                                                 (*message).e_maid.len(),
//                                                 &mut (*message).e_maid,
//                                             )?;
//                                         } else {
//                                             return Err(ExiError::StringValuesNotSupported);
//                                         }
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 305 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             305 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_iso2_certificate_chain(
//                                 stream,
//                                 &mut (*message).contract_signature_cert_chain,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }
// pub fn decode_iso2_payment_details_res(
//     stream: &mut ExiBitstream,
//     message: &mut Iso2PaymentDetailsResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 306 as i32;
//     let mut event_code: u32 = 0;
//     let error: i32 = 0;
//     loop {
//         match grammar_id {
//             306 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             exi_basetypes_decoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 &mut event_code,
//                             )?;
//                             if error == 0 as i32 {
//                                 if event_code == 0 as i32 as u32 {
//                                     let mut value: u32 = 0;
//                                     exi_basetypes_decoder_nbit_uint(
//                                         stream,
//                                         5 as i32 as usize,
//                                         &mut value,
//                                     )?;
//                                     if error == 0 as i32 {
//                                         (*message).response_code = value as u32;
//                                     }
//                                 } else {
//                                     return Err(ExiError::UnsupportedSubEvent);
//                                 }
//                             }
//                             if error == 0 as i32 {
//                                 exi_basetypes_decoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     &mut event_code,
//                                 )?;
//                                 if error == 0 as i32 {
//                                     if event_code == 0 as i32 as u32 {
//                                         grammar_id = 307 as i32;
//                                     } else {
//                                         return Err(ExiError::DeviantsNotSupported);
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             307 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_hex_binary(
//                                 stream,
//                                 &mut (*message).gen_challenge.len(),
//                                 &mut (*message).gen_challenge,
//                             )?;
//                             if error == 0 as i32 {
//                                 grammar_id = 308 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             308 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             decode_exi_type_integer64(stream, &mut (*message).evse_time_stamp)?;
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
//                 if error == 0 as i32 {
//                     match event_code {
//                         0 => {
//                             return Ok(());
//                         }
//                         _ => {
//                             return Err(ExiError::UnknownEventCode);
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 return Err(ExiError::UnknownGrammarId);
//             }
//         }
//     }
// }

pub fn decode_iso2_body(
    stream: &mut ExiBitstream,
    message: &mut Iso2BodyType,
) -> Result<(), ExiError> {
    let mut event_code: u32 = 0;
    exi_basetypes_decoder_nbit_uint(stream, 6_i32 as usize, &mut event_code)?;
    match event_code {
        // 0 => {
        //     let mut authorization_req: Iso2AuthorizationReqType = Default::default();
        //     decode_iso2_authorization_req(stream, &mut authorization_req)?;
        //     message.body_type_component = Iso2BodyTypeEnum::AuthorizationReq(authorization_req);
        // }
        // 1 => {
        //     let mut authorization_res: Iso2AuthorizationResType = Default::default();
        //     decode_iso2_authorization_res(stream, &mut authorization_res)?;
        //     message.body_type_component = Iso2BodyTypeEnum::AuthorizationRes(authorization_res);
        // }
        // 2 => {
        //     let mut body_element: Iso2BodyBaseType = Default::default();
        //     decode_iso2_body_base(stream, &mut body_element)?;
        //     message.body_type_component = Iso2BodyTypeEnum::BodyElement(body_element);
        // }
        // 3 => {
        //     let mut cable_check_req: Iso2CableCheckReqType = Default::default();
        //     decode_iso2_cable_check_req(stream, &mut cable_check_req)?;
        //     message.body_type_component = Iso2BodyTypeEnum::CableCheckReq(cable_check_req);
        // }
        // 4 => {
        //     let mut cable_check_res: Iso2CableCheckResType = Default::default();
        //     decode_iso2_cable_check_res(stream, &mut cable_check_res)?;
        //     message.body_type_component = Iso2BodyTypeEnum::CableCheckRes(cable_check_res);
        // }
        // 5 => {
        //     let mut certificate_installation_req: Iso2CertificateInstallationReqType =
        //         Default::default();
        //     decode_iso2_certificate_installation_req(stream, &mut certificate_installation_req)?;
        //     message.body_type_component =
        //         Iso2BodyTypeEnum::CertificateInstallationReq(certificate_installation_req);
        // }
        // 6 => {
        //     let mut certificate_installation_res: Iso2CertificateInstallationResType =
        //         Default::default();
        //     decode_iso2_certificate_installation_res(stream, &mut certificate_installation_res)?;
        //     message.body_type_component =
        //         Iso2BodyTypeEnum::CertificateInstallationRes(certificate_installation_res);
        // }
        // 7 => {
        //     let mut certificate_update_req: Iso2CertificateUpdateReqType = Default::default();
        //     decode_iso2_certificate_update_req(stream, &mut certificate_update_req)?;
        //     message.body_type_component =
        //         Iso2BodyTypeEnum::CertificateUpdateReq(certificate_update_req);
        // }
        // 8 => {
        //     let mut certificate_update_res: Iso2CertificateUpdateResType = Default::default();
        //     decode_iso2_certificate_update_res(stream, &mut certificate_update_res)?;
        //     message.body_type_component =
        //         Iso2BodyTypeEnum::CertificateUpdateRes(certificate_update_res);
        // }
        // 9 => {
        //     let mut charge_parameter_discovery_req: Iso2ChargeParameterDiscoveryReqType =
        //         Default::default();
        //     decode_iso2_charge_parameter_discovery_req(
        //         stream,
        //         &mut charge_parameter_discovery_req,
        //     )?;
        //     message.body_type_component =
        //         Iso2BodyTypeEnum::ChargeParameterDiscoveryReq(charge_parameter_discovery_req);
        // }
        // 10 => {
        //     let mut charge_parameter_discovery_res: Iso2ChargeParameterDiscoveryResType =
        //         Default::default();
        //     decode_iso2_charge_parameter_discovery_res(
        //         stream,
        //         &mut charge_parameter_discovery_res,
        //     )?;
        //     message.body_type_component =
        //         Iso2BodyTypeEnum::ChargeParameterDiscoveryRes(charge_parameter_discovery_res);
        // }
        // 11 => {
        //     let mut charging_status_req: Iso2ChargingStatusReqType = Default::default();
        //     decode_iso2_charging_status_req(stream, &mut charging_status_req)?;
        //     message.body_type_component = Iso2BodyTypeEnum::ChargingStatusReq(charging_status_req);
        // }
        // 12 => {
        //     let mut charging_status_res: Iso2ChargingStatusResType = Default::default();
        //     decode_iso2_charging_status_res(stream, &mut charging_status_res)?;
        //     message.body_type_component = Iso2BodyTypeEnum::ChargingStatusRes(charging_status_res);
        // }
        // 13 => {
        //     let mut current_demand_req: Iso2CurrentDemandReqType = Default::default();
        //     decode_iso2_current_demand_req(stream, &mut current_demand_req)?;
        //     message.body_type_component = Iso2BodyTypeEnum::CurrentDemandReq(current_demand_req);
        // }
        // 14 => {
        //     let mut current_demand_res: Iso2CurrentDemandResType = Default::default();
        //     decode_iso2_current_demand_res(stream, &mut current_demand_res)?;
        //     message.body_type_component = Iso2BodyTypeEnum::CurrentDemandRes(current_demand_res);
        // }
        // 15 => {
        //     let mut metering_receipt_req: Iso2MeteringReceiptReqType = Default::default();
        //     decode_iso2_metering_receipt_req(stream, &mut metering_receipt_req)?;
        //     message.body_type_component =
        //         Iso2BodyTypeEnum::MeteringReceiptReq(metering_receipt_req);
        // }
        // 16 => {
        //     let mut metering_receipt_res: Iso2MeteringReceiptResType = Default::default();
        //     decode_iso2_metering_receipt_res(stream, &mut metering_receipt_res)?;
        //     message.body_type_component =
        //         Iso2BodyTypeEnum::MeteringReceiptRes(metering_receipt_res);
        // }
        // 17 => {
        //     let mut payment_details_req: Iso2PaymentDetailsReqType = Default::default();
        //     decode_iso2_payment_details_req(stream, &mut payment_details_req)?;
        //     message.body_type_component = Iso2BodyTypeEnum::PaymentDetailsReq(payment_details_req);
        // }
        // 18 => {
        //     let mut payment_details_res: Iso2PaymentDetailsResType = Default::default();
        //     decode_iso2_payment_details_res(stream, &mut payment_details_res)?;
        //     message.body_type_component = Iso2BodyTypeEnum::PaymentDetailsRes(payment_details_res);
        // }
        // 19 => {
        //     let mut payment_service_selection_req: Iso2PaymentServiceSelectionReqType =
        //         Default::default();
        //     decode_iso2_payment_service_selection_req(stream, &mut payment_service_selection_req)?;
        //     message.body_type_component =
        //         Iso2BodyTypeEnum::PaymentServiceSelectionReq(payment_service_selection_req);
        // }
        // 20 => {
        //     let mut payment_service_selection_res: Iso2PaymentServiceSelectionResType =
        //         Default::default();
        //     decode_iso2_payment_service_selection_res(stream, &mut payment_service_selection_res)?;
        //     message.body_type_component =
        //         Iso2BodyTypeEnum::PaymentServiceSelectionRes(payment_service_selection_res);
        // }
        // 21 => {
        //     let mut power_delivery_req: Iso2PowerDeliveryReqType = Default::default();
        //     decode_iso2_power_delivery_req(stream, &mut power_delivery_req)?;
        //     message.body_type_component = Iso2BodyTypeEnum::PowerDeliveryReq(power_delivery_req);
        // }
        // 22 => {
        //     let mut power_delivery_res: Iso2PowerDeliveryResType = Default::default();
        //     decode_iso2_power_delivery_res(stream, &mut power_delivery_res)?;
        //     message.body_type_component = Iso2BodyTypeEnum::PowerDeliveryRes(power_delivery_res);
        // }
        // 23 => {
        //     let mut pre_charge_req: Iso2PreChargeReqType = Default::default();
        //     decode_iso2_pre_charge_req(stream, &mut pre_charge_req)?;
        //     message.body_type_component = Iso2BodyTypeEnum::PreChargeReq(pre_charge_req);
        // }
        // 24 => {
        //     let mut pre_charge_res: Iso2PreChargeResType = Default::default();
        //     decode_iso2_pre_charge_res(stream, &mut pre_charge_res)?;
        //     message.body_type_component = Iso2BodyTypeEnum::PreChargeRes(pre_charge_res);
        // }
        // 25 => {
        //     let mut service_detail_req: Iso2ServiceDetailReqType = Default::default();
        //     decode_iso2_service_detail_req(stream, &mut service_detail_req)?;
        //     message.body_type_component = Iso2BodyTypeEnum::ServiceDetailReq(service_detail_req);
        // }
        // 26 => {
        //     let mut service_detail_res: Iso2ServiceDetailResType = Default::default();
        //     decode_iso2_service_detail_res(stream, &mut service_detail_res)?;
        //     message.body_type_component = Iso2BodyTypeEnum::ServiceDetailRes(service_detail_res);
        // }
        // 27 => {
        //     let mut service_discovery_req: Iso2ServiceDiscoveryReqType = Default::default();
        //     decode_iso2_service_discovery_req(stream, &mut service_discovery_req)?;
        //     message.body_type_component =
        //         Iso2BodyTypeEnum::ServiceDiscoveryReq(service_discovery_req);
        // }
        // 28 => {
        //     let mut service_discovery_res: Iso2ServiceDiscoveryResType = Default::default();
        //     decode_iso2_service_discovery_res(stream, &mut service_discovery_res)?;
        //     message.body_type_component =
        //         Iso2BodyTypeEnum::ServiceDiscoveryRes(service_discovery_res);
        // }
        29 => {
            let mut session_setup_req: Iso2SessionSetupReqType = Iso2SessionSetupReqType::default();
            decode_iso2_session_setup_req(stream, &mut session_setup_req)?;
            message.body_type_component = Iso2BodyTypeEnum::SessionSetupReq(session_setup_req);
        }
        30 => {
            let mut session_setup_res: Iso2SessionSetupResType = Iso2SessionSetupResType::default();
            decode_iso2_session_setup_res(stream, &mut session_setup_res)?;
            message.body_type_component = Iso2BodyTypeEnum::SessionSetupRes(session_setup_res);
        }
        // 31 => {
        //     let mut session_stop_req: Iso2SessionStopReqType = Default::default();
        //     decode_iso2_session_stop_req(stream, &mut session_stop_req)?;
        //     message.body_type_component = Iso2BodyTypeEnum::SessionStopReq(session_stop_req);
        // }
        // 32 => {
        //     let mut session_stop_res: Iso2SessionStopResType = Default::default();
        //     decode_iso2_session_stop_res(stream, &mut session_stop_res)?;
        //     message.body_type_component = Iso2BodyTypeEnum::SessionStopRes(session_stop_res);
        // }
        // 33 => {
        //     let mut welding_detection_req: Iso2WeldingDetectionReqType = Default::default();
        //     decode_iso2_welding_detection_req(stream, &mut welding_detection_req)?;
        //     message.body_type_component =
        //         Iso2BodyTypeEnum::WeldingDetectionReq(welding_detection_req);
        // }
        // 34 => {
        //     let mut welding_detection_res: Iso2WeldingDetectionResType = Default::default();
        //     decode_iso2_welding_detection_res(stream, &mut welding_detection_res)?;
        //     message.body_type_component =
        //         Iso2BodyTypeEnum::WeldingDetectionRes(welding_detection_res);
        // }
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
    message: &mut Iso2v2gMessage,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 310_i32;
    let mut event_code: u32 = 0;
    let error: i32 = 0;
    loop {
        match grammar_id {
            310 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            decode_iso2_message_header(stream, &mut message.header)?;
                            if error == 0_i32 {
                                grammar_id = 311_i32;
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
                if error == 0_i32 {
                    match event_code {
                        0 => {
                            decode_iso2_body(stream, &mut message.body)?;
                            if error == 0_i32 {
                                grammar_id = 3_i32;
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
                if error == 0_i32 {
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
    message: &mut Iso2ExiDocument,
) -> Result<(), ExiError> {
    let mut event_code: u32 = 0;
    stream.read_and_check_header()?;
    exi_basetypes_decoder_nbit_uint(stream, 7_i32 as usize, &mut event_code)?;
    match event_code {
        0 | 76 => {
            decode_iso2_v2g_message(stream, &mut message.v2g_message)?;
            Ok(())
        }
        _ => Err(ExiError::UnsupportedSubEvent),
    }
}

// pub fn decode_iso2_exi_fragment(
//     stream: &mut ExiBitstream,
//     _message: &mut Iso2ExiFragment,
// ) -> Result<(), ExiError> {
//     let mut event_code: u32 = 0;
//     stream.read_and_check_header()?;
//     exi_basetypes_decoder_nbit_uint(stream, 8 as i32 as usize, &mut event_code)?;
//     return Err(ExiError::NotImplementedYet);
//     // match event_code {
//     //     0 => {}
//     //     1 => {}
//     //     2 => {}
//     //     3 => {}
//     //     4 => {
//     //         decode_iso2_AuthorizationReq(
//     //             stream,
//     //             &mut (*message).c2rust_unnamed.AuthorizationReq,
//     //         )?;
//     //         (*message).set_AuthorizationReq_isUsed(1 as u32);
//     //     }
//     //     5 => {}
//     //     6 => {}
//     //     7 => {}
//     //     8 => {}
//     //     9 => {}
//     //     10 => {}
//     //     11 => {}
//     //     12 => {}
//     //     13 => {}
//     //     14 => {}
//     //     15 => {
//     //         decode_iso2_CertificateInstallationReq(
//     //             stream,
//     //             &mut (*message).c2rust_unnamed.CertificateInstallationReq,
//     //         )?;
//     //         (*message).set_CertificateInstallationReq_isUsed(1 as u32);
//     //     }
//     //     16 => {}
//     //     17 => {
//     //         decode_iso2_CertificateUpdateReq(
//     //             stream,
//     //             &mut (*message).c2rust_unnamed.CertificateUpdateReq,
//     //         )?;
//     //         (*message).set_CertificateUpdateReq_isUsed(1 as u32);
//     //     }
//     //     18 => {}
//     //     19 => {}
//     //     20 => {}
//     //     21 => {}
//     //     22 => {}
//     //     23 => {}
//     //     24 => {}
//     //     25 => {}
//     //     26 => {}
//     //     27 => {}
//     //     28 => {}
//     //     29 => {}
//     //     30 => {}
//     //     31 => {}
//     //     32 => {}
//     //     33 => {
//     //         decode_iso2_CertificateChain(
//     //             stream,
//     //             &mut (*message).c2rust_unnamed.contract_signature_cert_chain,
//     //         )?;
//     //         (*message).set_ContractSignatureCertChain_isUsed(1 as u32);
//     //     }
//     //     34 => {
//     //         decode_iso2_ContractSignatureEncryptedPrivateKey(
//     //             stream,
//     //             &mut (*exiFrag)
//     //                 .c2rust_unnamed
//     //                 .ContractSignatureEncryptedPrivateKey,
//     //         )?;
//     //         (*message).set_ContractSignatureEncryptedPrivateKey_isUsed(1 as u32);
//     //     }
//     //     35 => {}
//     //     36 => {}
//     //     37 => {}
//     //     38 => {}
//     //     39 => {}
//     //     40 => {}
//     //     41 => {}
//     //     42 => {}
//     //     43 => {}
//     //     44 => {}
//     //     45 => {
//     //         decode_iso2_DiffieHellmanPublickey(
//     //             stream,
//     //             &mut (*message).c2rust_unnamed.DHpublickey,
//     //         )?;
//     //         (*message).set_DHpublickey_isUsed(1 as u32);
//     //     }
//     //     46 => {}
//     //     47 => {}
//     //     48 => {}
//     //     49 => {}
//     //     50 => {}
//     //     51 => {}
//     //     52 => {}
//     //     53 => {}
//     //     54 => {}
//     //     55 => {}
//     //     56 => {}
//     //     57 => {}
//     //     58 => {}
//     //     59 => {}
//     //     60 => {}
//     //     61 => {}
//     //     62 => {}
//     //     63 => {}
//     //     64 => {}
//     //     65 => {}
//     //     66 => {}
//     //     67 => {}
//     //     68 => {}
//     //     69 => {}
//     //     70 => {}
//     //     71 => {}
//     //     72 => {}
//     //     73 => {}
//     //     74 => {}
//     //     75 => {}
//     //     76 => {}
//     //     77 => {}
//     //     78 => {}
//     //     79 => {}
//     //     80 => {}
//     //     81 => {}
//     //     82 => {}
//     //     83 => {}
//     //     84 => {}
//     //     85 => {}
//     //     86 => {}
//     //     87 => {}
//     //     88 => {}
//     //     89 => {}
//     //     90 => {}
//     //     91 => {}
//     //     92 => {}
//     //     93 => {}
//     //     94 => {}
//     //     95 => {}
//     //     96 => {}
//     //     97 => {}
//     //     98 => {}
//     //     99 => {}
//     //     100 => {}
//     //     101 => {}
//     //     102 => {}
//     //     103 => {}
//     //     104 => {}
//     //     105 => {}
//     //     106 => {}
//     //     107 => {}
//     //     108 => {}
//     //     109 => {}
//     //     110 => {}
//     //     111 => {}
//     //     112 => {}
//     //     113 => {}
//     //     114 => {}
//     //     115 => {}
//     //     116 => {}
//     //     117 => {}
//     //     118 => {}
//     //     119 => {}
//     //     120 => {}
//     //     121 => {
//     //         decode_iso2_MeteringReceiptReq(
//     //             stream,
//     //             &mut (*message).c2rust_unnamed.MeteringReceiptReq,
//     //         )?;
//     //         (*message).set_MeteringReceiptReq_isUsed(1 as u32);
//     //     }
//     //     122 => {}
//     //     123 => {}
//     //     124 => {}
//     //     125 => {}
//     //     126 => {}
//     //     127 => {}
//     //     128 => {}
//     //     129 => {}
//     //     130 => {}
//     //     131 => {}
//     //     132 => {}
//     //     133 => {}
//     //     134 => {}
//     //     135 => {}
//     //     136 => {}
//     //     137 => {}
//     //     138 => {}
//     //     139 => {}
//     //     140 => {}
//     //     141 => {}
//     //     142 => {}
//     //     143 => {}
//     //     144 => {}
//     //     145 => {}
//     //     146 => {}
//     //     147 => {}
//     //     148 => {}
//     //     149 => {}
//     //     150 => {}
//     //     151 => {}
//     //     152 => {}
//     //     153 => {}
//     //     154 => {}
//     //     155 => {}
//     //     156 => {}
//     //     157 => {}
//     //     158 => {}
//     //     159 => {}
//     //     160 => {}
//     //     161 => {}
//     //     162 => {}
//     //     163 => {}
//     //     164 => {}
//     //     165 => {}
//     //     166 => {}
//     //     167 => {}
//     //     168 => {}
//     //     169 => {}
//     //     170 => {}
//     //     171 => {}
//     //     172 => {}
//     //     173 => {}
//     //     174 => {
//     //         decode_iso2_SalesTariff(
//     //             stream,
//     //             &mut (*message).c2rust_unnamed.SalesTariff,
//     //         )?;
//     //         (*message).set_SalesTariff_isUsed(1 as u32);
//     //     }
//     //     175 => {}
//     //     176 => {}
//     //     177 => {}
//     //     178 => {}
//     //     179 => {}
//     //     180 => {}
//     //     181 => {}
//     //     182 => {}
//     //     183 => {}
//     //     184 => {}
//     //     185 => {}
//     //     186 => {}
//     //     187 => {}
//     //     188 => {}
//     //     189 => {}
//     //     190 => {}
//     //     191 => {}
//     //     192 => {}
//     //     193 => {}
//     //     194 => {}
//     //     195 => {}
//     //     196 => {}
//     //     197 => {}
//     //     198 => {}
//     //     199 => {}
//     //     200 => {}
//     //     201 => {}
//     //     202 => {}
//     //     203 => {}
//     //     204 => {}
//     //     205 => {}
//     //     206 => {}
//     //     207 => {}
//     //     208 => {
//     //         decode_iso2_SignedInfo(
//     //             stream,
//     //             &mut (*message).c2rust_unnamed.SignedInfo,
//     //         )?;
//     //         (*message).set_SignedInfo_isUsed(1 as u32);
//     //     }
//     //     209 => {}
//     //     210 => {}
//     //     211 => {}
//     //     212 => {}
//     //     213 => {}
//     //     214 => {}
//     //     215 => {}
//     //     216 => {}
//     //     217 => {}
//     //     218 => {}
//     //     219 => {}
//     //     220 => {}
//     //     221 => {}
//     //     222 => {}
//     //     223 => {}
//     //     224 => {}
//     //     225 => {}
//     //     226 => {}
//     //     227 => {}
//     //     228 => {}
//     //     229 => {}
//     //     230 => {}
//     //     231 => {}
//     //     232 => {}
//     //     233 => {}
//     //     234 => {}
//     //     235 => {}
//     //     236 => {
//     //         decode_iso2_EMAID(stream, &mut (*message).c2rust_unnamed.eMaid);
//     //         (*message).set_eMAID_isUsed(1 as u32);
//     //     }
//     //     237 => {}
//     //     238 => {}
//     //     239 => {}
//     //     240 => {}
//     //     241 => {}
//     //     242 => {}
//     //     _ => {
//     //         return Err(ExiError::UnsupportedSubEvent);
//     //     }
//     // }
//     // if error == 0 as i32 {
//     //     exi_basetypes_decoder_nbit_uint(stream, 8 as i32 as usize, &mut event_code)?;
//     //     if error == 0 as i32 {
//     //         if event_code != 244 as i32 as u32 {
//     //             return Err(ExiError::INCORRECT_END_FRAGMENT_VALUE);
//     //         }
//     //     }
//     // }
// }

// pub fn decode_iso2_xmldsig_fragment(
//     stream: &mut ExiBitstream,
//     _message: &mut Iso2XmlDSigFragment,
// ) -> Result<(), ExiError> {
//     let mut event_code: u32 = 0;
//     stream.read_and_check_header()?;
//     exi_basetypes_decoder_nbit_uint(stream, 6 as i32 as usize, &mut event_code)?;
//     return Err(ExiError::NotImplementedYet);
//     // match event_code {
//     //     0 => {
//     //         decode_iso2_CanonicalizationMethod(
//     //             stream,
//     //             &mut (*message).XmlDSigComponents.CanonicalizationMethod,
//     //         )?;
//     //         (*message).set_CanonicalizationMethod_isUsed(1 as u32);
//     //     }
//     //     1 => {
//     //         decode_iso2_dsa_key_value(
//     //             stream,
//     //             &mut (*message).XmlDSigComponents.dsa_key_value,
//     //         )?;
//     //         (*message).set_dsa_key_value_isUsed(1 as u32);
//     //     }
//     //     2 => {
//     //         decode_iso2_DigestMethod(
//     //             stream,
//     //             &mut (*message).XmlDSigComponents.DigestMethod,
//     //         )?;
//     //         (*message).set_DigestMethod_isUsed(1 as u32);
//     //     }
//     //     3 => {}
//     //     4 => {}
//     //     5 => {}
//     //     6 => {}
//     //     7 => {}
//     //     8 => {
//     //         error =
//     //             decode_iso2_KeyInfo(stream, &mut (*message).XmlDSigComponents.KeyInfo);
//     //         (*message).set_KeyInfo_isUsed(1 as u32);
//     //     }
//     //     9 => {}
//     //     10 => {
//     //         decode_iso2_key_value(
//     //             stream,
//     //             &mut (*message).XmlDSigComponents.key_value,
//     //         )?;
//     //         (*message).set_key_value_isUsed(1 as u32);
//     //     }
//     //     11 => {}
//     //     12 => {}
//     //     13 => {}
//     //     14 => {
//     //         error =
//     //             decode_iso2_Object(stream, &mut (*message).XmlDSigComponents.Object);
//     //         (*message).set_Object_isUsed(1 as u32);
//     //     }
//     //     15 => {}
//     //     16 => {
//     //         error =
//     //             decode_iso2_pgp_data(stream, &mut (*message).XmlDSigComponents.pgp_data);
//     //         (*message).set_pgp_data_isUsed(1 as u32);
//     //     }
//     //     17 => {}
//     //     18 => {}
//     //     19 => {}
//     //     20 => {}
//     //     21 => {
//     //         decode_iso2_RSAkey_value(
//     //             stream,
//     //             &mut (*message).XmlDSigComponents.RSAkey_value,
//     //         )?;
//     //         (*message).set_RSAkey_value_isUsed(1 as u32);
//     //     }
//     //     22 => {
//     //         decode_iso2_reference(
//     //             stream,
//     //             &mut (*message).XmlDSigComponents.reference,
//     //         )?;
//     //         (*message).set_reference_isUsed(1 as u32);
//     //     }
//     //     23 => {
//     //         decode_iso2_retrieval_method(
//     //             stream,
//     //             &mut (*message).XmlDSigComponents.retrieval_method,
//     //         )?;
//     //         (*message).set_retrieval_method_isUsed(1 as u32);
//     //     }
//     //     24 => {
//     //         decode_iso2_spki_data(
//     //             stream,
//     //             &mut (*message).XmlDSigComponents.spki_data,
//     //         )?;
//     //         (*message).set_spki_data_isUsed(1 as u32);
//     //     }
//     //     25 => {}
//     //     26 => {}
//     //     27 => {
//     //         decode_iso2_Signature(
//     //             stream,
//     //             &mut (*message).XmlDSigComponents.Signature,
//     //         )?;
//     //         (*message).set_Signature_isUsed(1 as u32);
//     //     }
//     //     28 => {
//     //         decode_iso2_SignatureMethod(
//     //             stream,
//     //             &mut (*message).XmlDSigComponents.SignatureMethod,
//     //         )?;
//     //         (*message).set_SignatureMethod_isUsed(1 as u32);
//     //     }
//     //     29 => {}
//     //     30 => {}
//     //     31 => {
//     //         decode_iso2_SignatureValue(
//     //             stream,
//     //             &mut (*message).XmlDSigComponents.SignatureValue,
//     //         )?;
//     //         (*message).set_SignatureValue_isUsed(1 as u32);
//     //     }
//     //     32 => {
//     //         decode_iso2_SignedInfo(
//     //             stream,
//     //             &mut (*message).XmlDSigComponents.SignedInfo,
//     //         )?;
//     //         (*message).set_SignedInfo_isUsed(1 as u32);
//     //     }
//     //     33 => {
//     //         decode_iso2_Transform(
//     //             stream,
//     //             &mut (*message).XmlDSigComponents.Transform,
//     //         )?;
//     //         (*message).set_Transform_isUsed(1 as u32);
//     //     }
//     //     34 => {
//     //         decode_iso2_Transforms(
//     //             stream,
//     //             &mut (*message).XmlDSigComponents.Transforms,
//     //         )?;
//     //         (*message).set_Transforms_isUsed(1 as u32);
//     //     }
//     //     35 => {}
//     //     36 => {}
//     //     37 => {
//     //         decode_iso2_x509_data(
//     //             stream,
//     //             &mut (*message).XmlDSigComponents.x509_data,
//     //         )?;
//     //         (*message).set_x509_data_isUsed(1 as u32);
//     //     }
//     //     38 => {}
//     //     39 => {
//     //         decode_iso2_X509IssuerSerial(
//     //             stream,
//     //             &mut (*message).XmlDSigComponents.X509IssuerSerial,
//     //         )?;
//     //         (*message).set_X509IssuerSerial_isUsed(1 as u32);
//     //     }
//     //     40 => {}
//     //     41 => {}
//     //     42 => {}
//     //     43 => {}
//     //     44 => {}
//     //     _ => {
//     //         return Err(ExiError::UnsupportedSubEvent);
//     //     }
//     // }
//     // if error == 0 as i32 {
//     //     exi_basetypes_decoder_nbit_uint(stream, 6 as i32 as usize, &mut event_code)?;
//     //     if error == 0 as i32 {
//     //         if event_code != 46 as i32 as u32 {
//     //             return Err(ExiError::INCORRECT_END_FRAGMENT_VALUE);
//     //         }
//     //     }
//     // }
// }
