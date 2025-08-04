use core::result::Result;

use crate::app_handshake::app_handshake_datatypes::*;
use crate::common::exi_basetypes_encoder::*;
use crate::common::exi_bitstream::*;
use crate::common::exi_error_codes::*;
use crate::common::exi_header::*;

pub fn encode_app_hand_app_protocol_type(
    stream: &mut ExiBitstream,
    app_protocol_type: &AppHandAppProtocolType,
) -> Result<(), ExiError> {
    let mut grammar_id: u8 = 0;
    loop {
        match grammar_id {
            0 => {
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                exi_basetypes_encoder_uint_16(
                    stream,
                    (app_protocol_type.protocol_namespace.characters.len() + 2) as u16,
                )?;

                exi_basetypes_encoder_characters(
                    stream,
                    &app_protocol_type.protocol_namespace.characters,
                )?;

                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

                grammar_id = 1;
                continue;
            }
            1 => {
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                exi_basetypes_encoder_uint_32(stream, app_protocol_type.version_number_major)?;
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                grammar_id = 2;
                continue;
            }
            2 => {
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                exi_basetypes_encoder_uint_32(stream, app_protocol_type.version_number_minor)?;
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

                grammar_id = 3;
                continue;
            }
            3 => {
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                exi_basetypes_encoder_nbit_uint(stream, 8, app_protocol_type.schema_id as u32)?;
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                grammar_id = 4;
                continue;
            }
            4 => {
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                exi_basetypes_encoder_nbit_uint(
                    stream,
                    5,
                    (app_protocol_type.priority as u32).wrapping_sub(1),
                )?;

                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                grammar_id = 5;
                continue;
            }
            5 => {
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                //grammar_id = 6 as i32;
                return Ok(());
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}

pub fn encode_app_hand_supported_app_protocol_req(
    stream: &mut ExiBitstream,
    supported_app_protocol_req: &AppHandSupportedAppProtocolReq,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 7;
    let mut app_protocol_current_index: usize = 0;
    loop {
        match grammar_id {
            7 => {
                if app_protocol_current_index < supported_app_protocol_req.app_protocol.array.len()
                {
                    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

                    let fresh0 = app_protocol_current_index;
                    app_protocol_current_index = app_protocol_current_index.wrapping_add(1);
                    encode_app_hand_app_protocol_type(
                        stream,
                        &supported_app_protocol_req.app_protocol.array[fresh0],
                    )?;
                    grammar_id = 8;
                    continue;
                } else {
                    return Err(ExiError::UnknownEventCode);
                }
            }
            8 => {
                if app_protocol_current_index < supported_app_protocol_req.app_protocol.array.len()
                {
                    exi_basetypes_encoder_nbit_uint(stream, 2, 0)?;

                    let fresh1 = app_protocol_current_index;
                    app_protocol_current_index = app_protocol_current_index.wrapping_add(1);
                    encode_app_hand_app_protocol_type(
                        stream,
                        &supported_app_protocol_req.app_protocol.array[fresh1],
                    )?;

                    grammar_id = 8;
                    continue;
                } else {
                    exi_basetypes_encoder_nbit_uint(stream, 2, 1)?;
                    return Ok(());
                }
            }
            5 => {
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                return Ok(());
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}

pub fn encode_app_hand_supported_app_protocol_res(
    stream: &mut ExiBitstream,
    supported_app_protocol_res: &AppHandSupportedAppProtocolRes,
) -> Result<(), ExiError> {
    let mut grammar_id: i32 = 9;
    loop {
        match grammar_id {
            9 => {
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                exi_basetypes_encoder_nbit_uint(
                    stream,
                    2,
                    supported_app_protocol_res.response_code as u32,
                )?;
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

                grammar_id = 10;
                continue;
            }
            10 => {
                if supported_app_protocol_res.schema_id == Some(1) {
                    exi_basetypes_encoder_nbit_uint(stream, 2, 0)?;
                    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

                    exi_basetypes_encoder_nbit_uint(
                        stream,
                        8,
                        supported_app_protocol_res.schema_id.unwrap_or(0) as u32,
                    )?;

                    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                    grammar_id = 5;
                    continue;
                } else {
                    exi_basetypes_encoder_nbit_uint(stream, 2, 1)?;
                    return Ok(());
                }
            }
            5 => {
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                return Ok(());
            }
            _ => {
                return Err(ExiError::UnknownGrammarId);
            }
        }
    }
}

pub fn encode_app_handshake_exi_document(
    stream: &mut ExiBitstream,
    exi_doc: AppHandExiDocument,
) -> Result<(), ExiError> {
    exi_header_write(stream)?;
    if let Some(req) = exi_doc.supported_app_protocol_req {
        exi_basetypes_encoder_nbit_uint(stream, 2, 0)?;
        encode_app_hand_supported_app_protocol_req(stream, &req)?;
    } else if let Some(res) = exi_doc.supported_app_protocol_res {
        exi_basetypes_encoder_nbit_uint(stream, 2, 1)?;
        encode_app_hand_supported_app_protocol_res(stream, &res)?;
    } else {
        return Err(ExiError::UnknownEventForEncoding);
    }

    Ok(())
}
