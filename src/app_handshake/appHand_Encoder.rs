use core::result::Result;

use crate::app_handshake::appHand_Datatypes::*;
use crate::common::exi_basetypes_encoder::*;
use crate::common::exi_bitstream::*;
use crate::common::exi_error_codes::*;
use crate::common::exi_header::*;

fn encode_appHand_AppProtocolType(
    stream: &mut ExiBitstream,
    AppProtocolType: AppHandAppProtocolType,
) -> Result<u8, i16> {
    let mut grammar_id: u8 = 0;
    loop {
        match grammar_id {
            0 => {
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                exi_basetypes_encoder_uint_16(
                    stream,
                    (AppProtocolType.ProtocolNamespace.charactersLen + 2) as u16,
                )?;

                exi_basetypes_encoder_characters(
                    stream,
                    &AppProtocolType.ProtocolNamespace.characters,
                )?;

                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

                grammar_id = 1;
                continue;
            }
            1 => {
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                exi_basetypes_encoder_uint_32(stream, AppProtocolType.VersionNumberMajor)?;
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                grammar_id = 2;
                continue;
            }
            2 => {
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                exi_basetypes_encoder_uint_32(stream, AppProtocolType.VersionNumberMinor)?;
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

                grammar_id = 3;
                continue;
            }
            3 => {
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                exi_basetypes_encoder_nbit_uint(stream, 8, AppProtocolType.SchemaID as u32)?;
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
                    (AppProtocolType.Priority as u32).wrapping_sub(1),
                )?;

                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                grammar_id = 5;
                continue;
            }
            5 => {
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                //grammar_id = 6 as i32;
                return Ok(NO_ERROR);
            }
            _ => {
                return Err(-130);
            }
        }
    }
}

fn encode_appHand_supportedAppProtocolReq(
    stream: &mut ExiBitstream,
    supportedAppProtocolReq: AppHandSupportedAppProtocolReq,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 7;
    let mut AppProtocol_currentIndex: u16 = 0;
    loop {
        match grammar_id {
            7 => {
                if (AppProtocol_currentIndex as i32)
                    < supportedAppProtocolReq.AppProtocol.arrayLen as i32
                {
                    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

                    let fresh0 = AppProtocol_currentIndex as usize;
                    AppProtocol_currentIndex = AppProtocol_currentIndex.wrapping_add(1);
                    encode_appHand_AppProtocolType(
                        stream,
                        supportedAppProtocolReq.AppProtocol.array[fresh0],
                    )?;
                    grammar_id = 8;
                    continue;
                } else {
                    return Err(UNKNOWN_EVENT_CODE);
                }
            }
            8 => {
                if (AppProtocol_currentIndex as i32)
                    < supportedAppProtocolReq.AppProtocol.arrayLen as i32
                {
                    exi_basetypes_encoder_nbit_uint(stream, 2, 0)?;

                    let fresh1 = AppProtocol_currentIndex as usize;
                    AppProtocol_currentIndex = AppProtocol_currentIndex.wrapping_add(1);
                    encode_appHand_AppProtocolType(
                        stream,
                        supportedAppProtocolReq.AppProtocol.array[fresh1],
                    )?;

                    grammar_id = 8;
                    continue;
                } else {
                    exi_basetypes_encoder_nbit_uint(stream, 2, 1)?;
                    //grammar_id = 6 as i32;
                    return Ok(NO_ERROR);
                }
            }
            5 => {
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                //grammar_id = 6 as i32;
                return Ok(NO_ERROR);
            }
            _ => {
                return Err(-130);
            }
        }
    }
}

fn encode_appHand_supportedAppProtocolRes(
    stream: &mut ExiBitstream,
    supportedAppProtocolRes: AppHandSupportedAppProtocolRes,
) -> Result<i8, i16> {
    let mut grammar_id: i32 = 9;
    loop {
        match grammar_id {
            9 => {
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                exi_basetypes_encoder_nbit_uint(
                    stream,
                    2,
                    supportedAppProtocolRes.ResponseCode as u32,
                )?;
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

                grammar_id = 10;
                continue;
            }
            10 => {
                if supportedAppProtocolRes.SchemaID == Some(1) {
                    exi_basetypes_encoder_nbit_uint(stream, 2, 0)?;
                    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

                    exi_basetypes_encoder_nbit_uint(
                        stream,
                        8,
                        supportedAppProtocolRes.SchemaID.unwrap_or(0) as u32,
                    )?;

                    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                    grammar_id = 5;
                    continue;
                } else {
                    exi_basetypes_encoder_nbit_uint(stream, 2, 1)?;
                    //grammar_id = 6 as i32;
                    return Ok(0);
                }
            }
            5 => {
                exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
                //grammar_id = 6 as i32;
                return Ok(0);
            }
            _ => {
                return Err(-130);
            }
        }
    }
}

pub fn encode_appHand_exiDocument(
    stream: &mut ExiBitstream,
    exiDoc: AppHandExiDocument,
) -> Result<u8, i16> {
    exi_header_write(stream)?;
    if let Some(req) = exiDoc.SupportedAppProtocolReq {
        exi_basetypes_encoder_nbit_uint(stream, 2, 0)?;
        encode_appHand_supportedAppProtocolReq(stream, req)?;
    } else if let Some(res) = exiDoc.SupportedAppProtocolRes {
        exi_basetypes_encoder_nbit_uint(stream, 2, 1)?;
        encode_appHand_supportedAppProtocolRes(stream, res)?;
    } else {
        return Err(UNKNOWN_EVENT_FOR_ENCODING);
    }
    return Ok(NO_ERROR);
}
