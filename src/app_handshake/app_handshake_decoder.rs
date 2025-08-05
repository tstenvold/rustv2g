use core::result::Result;

use crate::app_handshake::app_handshake_datatypes::{
    AppHandAppProtocolType, AppHandExiDocument, AppHandResponseCodeType,
    AppHandSupportedAppProtocolReq, AppHandSupportedAppProtocolRes,
};
use crate::common::exi_basetypes_decoder::{
    exi_basetypes_decoder_characters, exi_basetypes_decoder_nbit_uint,
    exi_basetypes_decoder_uint_16,
};
use crate::common::exi_bitstream::ExiBitstream;
use crate::common::exi_error_codes::ExiError;
use crate::common::exi_types_decoder::decode_exi_type_uint32;

#[allow(clippy::too_many_lines)]
pub fn decode_app_hand_app_protocol_type(
    stream: &mut ExiBitstream,
    mut app_protocol_type: &mut AppHandAppProtocolType,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 0;
    let mut event_code: u32 = 0;
    loop {
        match grammar_id {
            0 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                match event_code {
                    0 => {
                        exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;

                        if event_code == 0 {
                            let mut char_len: u16 = 0;
                            exi_basetypes_decoder_uint_16(stream, &mut char_len)?;

                            if char_len >= 2 {
                                char_len -= 2;
                                exi_basetypes_decoder_characters(
                                    stream,
                                    char_len as usize,
                                    &mut app_protocol_type.protocol_namespace.characters,
                                    100,
                                )?;
                            } else {
                                return Err(ExiError::InvalidCharactersLength);
                            }
                        } else {
                            return Err(ExiError::UnsupportedSubEvent);
                        }

                        exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                        if event_code == 0 {
                            grammar_id = 1;
                            continue;
                        }
                        return Err(ExiError::DeviantsNotSupported);
                    }
                    _ => {
                        return Err(ExiError::UnknownEventCode);
                    }
                }
            }
            1 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                match event_code {
                    0 => {
                        decode_exi_type_uint32(
                            stream,
                            &mut app_protocol_type.version_number_major,
                        )?;
                        grammar_id = 2;
                        continue;
                    }
                    _ => {
                        return Err(ExiError::UnknownEventCode);
                    }
                }
            }
            2 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                match event_code {
                    0 => {
                        decode_exi_type_uint32(
                            stream,
                            &mut app_protocol_type.version_number_minor,
                        )?;
                        grammar_id = 3;
                        continue;
                    }
                    _ => {
                        return Err(ExiError::UnknownEventCode);
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                match event_code {
                    0 => {
                        exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;

                        match event_code {
                            0 => {
                                let mut value: u32 = 0;
                                exi_basetypes_decoder_nbit_uint(stream, 8, &mut value)?;
                                app_protocol_type.schema_id =
                                    value.try_into().map_err(|_| ExiError::InvalidValue)?;
                            }
                            _ => {
                                return Err(ExiError::UnsupportedSubEvent);
                            }
                        }
                        exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                        if event_code == 0 {
                            grammar_id = 4;
                            continue;
                        }
                        return Err(ExiError::DeviantsNotSupported);
                    }
                    _ => {
                        return Err(ExiError::UnknownEventCode);
                    }
                }
            }
            4 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                match event_code {
                    0 => {
                        exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                        if event_code == 0 {
                            let mut value_0: u32 = 0;
                            exi_basetypes_decoder_nbit_uint(stream, 5, &mut value_0)?;
                            app_protocol_type.priority = value_0
                                .wrapping_add(1)
                                .try_into()
                                .map_err(|_| ExiError::InvalidValue)?;
                        } else {
                            return Err(ExiError::UnsupportedSubEvent);
                        }

                        exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                        if event_code == 0 {
                            grammar_id = 5;
                            continue;
                        }
                        return Err(ExiError::DeviantsNotSupported);
                    }
                    _ => {
                        return Err(ExiError::UnknownEventCode);
                    }
                }
            }
            5 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                match event_code {
                    0 => {
                        //grammar_id = 6 as i32;
                        return Ok(());
                    }
                    _ => {
                        return Err(ExiError::UnknownEventCode);
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}

pub fn decode_app_hand_supported_app_protocol_req(
    stream: &mut ExiBitstream,
    supported_app_protocol_req: &mut AppHandSupportedAppProtocolReq,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 7;
    let mut event_code: u32 = 0;
    loop {
        match grammar_id {
            7 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                match event_code {
                    0 => {
                        if supported_app_protocol_req.app_protocols.len() < 5 {
                            let idx = supported_app_protocol_req.app_protocols.len() as usize;

                            match supported_app_protocol_req
                                .app_protocols
                                .push(AppHandAppProtocolType::default())
                            {
                                Ok(()) => {
                                    decode_app_hand_app_protocol_type(
                                        stream,
                                        &mut supported_app_protocol_req.app_protocols[idx],
                                    )?;
                                }
                                Err(_) => {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            }
                        } else {
                            return Err(ExiError::ArrayOutOfBounds);
                        }
                        grammar_id = 8;
                        continue;
                    }
                    _ => {
                        return Err(ExiError::UnknownEventCode);
                    }
                }
            }
            8 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                match event_code {
                    0 => {
                        if supported_app_protocol_req.app_protocols.len() < 5 {
                            let idx = supported_app_protocol_req.app_protocols.len();
                            match supported_app_protocol_req
                                .app_protocols
                                .push(AppHandAppProtocolType::default())
                            {
                                Ok(()) => {
                                    decode_app_hand_app_protocol_type(
                                        stream,
                                        &mut supported_app_protocol_req.app_protocols[idx],
                                    )?;
                                }
                                Err(_) => {
                                    return Err(ExiError::ArrayOutOfBounds);
                                }
                            }
                        } else {
                            return Err(ExiError::ArrayOutOfBounds);
                        }
                        if supported_app_protocol_req.app_protocols.len() < 5 {
                            grammar_id = 8;
                            continue;
                        }
                        grammar_id = 5;
                        continue;
                    }
                    1 => {
                        //grammar_id = 6 as i32;
                        return Ok(());
                    }
                    _ => {
                        return Err(ExiError::UnknownEventCode);
                    }
                }
            }
            5 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                match event_code {
                    0 => {
                        //grammar_id = 6 as i32;
                        return Ok(());
                    }
                    _ => {
                        return Err(ExiError::UnknownEventCode);
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}

pub fn decode_app_hand_supported_app_protocol_res(
    stream: &mut ExiBitstream,
    mut supported_app_protocol_res: &mut AppHandSupportedAppProtocolRes,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 9;
    let mut event_code: u32 = 0;
    loop {
        match grammar_id {
            9 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                match event_code {
                    0 => {
                        exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                        if event_code == 0 {
                            let mut value: u32 = 0;
                            exi_basetypes_decoder_nbit_uint(stream, 2, &mut value)?;
                            match value {
                                0 => supported_app_protocol_res.response_code = AppHandResponseCodeType::AppHandResponseCodeTypeOkSuccessfulNegotiation,
                                1 => supported_app_protocol_res.response_code = AppHandResponseCodeType::AppHandResponseCodeTypeOkSuccessfulNegotiationWithMinorDeviation,
                                2 => supported_app_protocol_res.response_code = AppHandResponseCodeType::AppHandResponseCodeTypeFailedNoNegotiation,
                                _ => return Err(ExiError::UnknownEventCode),
                            }
                        } else {
                            return Err(ExiError::UnsupportedSubEvent);
                        }

                        exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;

                        if event_code == 0 {
                            grammar_id = 10;
                            continue;
                        }
                        return Err(ExiError::DeviantsNotSupported);
                    }
                    _ => {
                        return Err(ExiError::UnknownEventCode);
                    }
                }
            }
            10 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
                match event_code {
                    0 => {
                        exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                        if event_code == 0 {
                            let mut value_0: u32 = 0;
                            exi_basetypes_decoder_nbit_uint(stream, 8, &mut value_0)?;

                            supported_app_protocol_res.schema_id =
                                Some(value_0.try_into().map_err(|_| ExiError::InvalidValue)?);
                        } else {
                            return Err(ExiError::UnsupportedSubEvent);
                        }
                        exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                        if event_code == 0 {
                            grammar_id = 5;
                            continue;
                        }
                        return Err(ExiError::DeviantsNotSupported);
                    }
                    1 => {
                        //grammar_id = 6 as i32;
                        return Ok(());
                    }
                    _ => {
                        return Err(ExiError::UnknownEventCode);
                    }
                }
            }
            5 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut event_code)?;
                match event_code {
                    0 => {
                        //grammar_id = 6 as i32;
                        return Ok(());
                    }
                    _ => {
                        return Err(ExiError::UnknownEventCode);
                    }
                }
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}

pub fn decode_app_hand_exi_document(
    stream: &mut ExiBitstream,
    exi_doc: &mut AppHandExiDocument,
) -> Result<(), ExiError> {
    let mut event_code: u32 = 0;
    stream.read_and_check_header()?;
    exi_basetypes_decoder_nbit_uint(stream, 2, &mut event_code)?;
    match event_code {
        0 => {
            // Create a new struct, decode into it, then set the enum
            let mut req = AppHandSupportedAppProtocolReq::default();
            decode_app_hand_supported_app_protocol_req(stream, &mut req)?;
            exi_doc.supported_app_protocol_req = Some(req);
        }
        1 => {
            let mut res = AppHandSupportedAppProtocolRes::default();
            decode_app_hand_supported_app_protocol_res(stream, &mut res)?;
            exi_doc.supported_app_protocol_res = Some(res);
        }
        _ => return Err(ExiError::UnsupportedSubEvent),
    }
    Ok(())
}
