use core::result::Result;

use crate::app_handshake::appHand_Datatypes::*;
use crate::common::exi_basetypes_decoder::*;
use crate::common::exi_bitstream::*;
use crate::common::exi_error_codes::*;
use crate::common::exi_header::*;
use crate::common::exi_types_decoder::*;

pub fn decode_appHand_AppProtocolType(
    stream: &mut ExiBitstream,
    mut AppProtocolType: &mut AppHandAppProtocolType,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 0;
    let mut eventCode: u32 = 0;
    loop {
        match grammar_id {
            0 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                match eventCode {
                    0 => {
                        exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;

                        if eventCode == 0 {
                            exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (AppProtocolType.ProtocolNamespace.charactersLen as u16),
                            )?;
                            if AppProtocolType.ProtocolNamespace.charactersLen as i32 >= 2 as i32 {
                                AppProtocolType.ProtocolNamespace.charactersLen =
                                    (AppProtocolType.ProtocolNamespace.charactersLen - 2) as usize;
                                exi_basetypes_decoder_characters(
                                    stream,
                                    AppProtocolType.ProtocolNamespace.charactersLen as usize,
                                    &mut AppProtocolType.ProtocolNamespace.characters,
                                    100,
                                )?;
                            } else {
                                return Err(-200);
                            }
                        } else {
                            return Err(UNSUPPORTED_SUB_EVENT);
                        }

                        exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                        if eventCode == 0 {
                            grammar_id = 1;
                            continue;
                        } else {
                            return Err(DEVIANTS_NOT_SUPPORTED);
                        }
                    }
                    _ => {
                        return Err(UNKNOWN_EVENT_CODE);
                    }
                }
            }
            1 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                match eventCode {
                    0 => {
                        decode_exi_type_uint32(stream, &mut AppProtocolType.VersionNumberMajor)?;
                        grammar_id = 2;
                        continue;
                    }
                    _ => {
                        return Err(UNKNOWN_EVENT_CODE);
                    }
                }
            }
            2 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                match eventCode {
                    0 => {
                        decode_exi_type_uint32(stream, &mut AppProtocolType.VersionNumberMinor)?;
                        grammar_id = 3;
                        continue;
                    }
                    _ => {
                        return Err(UNKNOWN_EVENT_CODE);
                    }
                }
            }
            3 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                match eventCode {
                    0 => {
                        exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;

                        match eventCode {
                            0 => {
                                let mut value: u32 = 0;
                                exi_basetypes_decoder_nbit_uint(stream, 8, &mut value)?;
                                AppProtocolType.SchemaID = value as u8;
                            }
                            _ => {
                                return Err(UNSUPPORTED_SUB_EVENT);
                            }
                        }
                        exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                        if eventCode == 0 {
                            grammar_id = 4;
                            continue;
                        } else {
                            return Err(DEVIANTS_NOT_SUPPORTED);
                        }
                    }
                    _ => {
                        return Err(UNKNOWN_EVENT_CODE);
                    }
                }
            }
            4 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                match eventCode {
                    0 => {
                        exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                        if eventCode == 0 {
                            let mut value_0: u32 = 0;
                            exi_basetypes_decoder_nbit_uint(stream, 5, &mut value_0)?;
                            AppProtocolType.Priority = value_0.wrapping_add(1) as u8;
                        } else {
                            return Err(UNSUPPORTED_SUB_EVENT);
                        }

                        exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                        if eventCode == 0 {
                            grammar_id = 5;
                            continue;
                        } else {
                            return Err(DEVIANTS_NOT_SUPPORTED);
                        }
                    }
                    _ => {
                        return Err(UNKNOWN_EVENT_CODE);
                    }
                }
            }
            5 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                match eventCode {
                    0 => {
                        //grammar_id = 6 as i32;
                        return Ok(NO_ERROR);
                    }
                    _ => {
                        return Err(UNKNOWN_EVENT_CODE);
                    }
                }
            }
            _ => {
                return Err(-130);
            }
        }
    }
}

pub fn decode_appHand_supportedAppProtocolReq(
    stream: &mut ExiBitstream,
    mut supportedAppProtocolReq: &mut AppHandSupportedAppProtocolReq,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 7;
    let mut eventCode: u32 = 0;
    loop {
        match grammar_id {
            7 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                match eventCode {
                    0 => {
                        if supportedAppProtocolReq.AppProtocol.arrayLen < 5 {
                            let idx = supportedAppProtocolReq.AppProtocol.arrayLen as usize;
                            supportedAppProtocolReq.AppProtocol.arrayLen += 1;
                            decode_appHand_AppProtocolType(
                                stream,
                                &mut supportedAppProtocolReq.AppProtocol.array[idx],
                            )?;
                        } else {
                            return Err(-110);
                        }
                        grammar_id = 8;
                        continue;
                    }
                    _ => {
                        return Err(-150);
                    }
                }
            }
            8 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                match eventCode {
                    0 => {
                        if supportedAppProtocolReq.AppProtocol.arrayLen < 5 {
                            let idx = supportedAppProtocolReq.AppProtocol.arrayLen as usize;
                            supportedAppProtocolReq.AppProtocol.arrayLen += 1;
                            decode_appHand_AppProtocolType(
                                stream,
                                &mut supportedAppProtocolReq.AppProtocol.array[idx],
                            )?;
                        } else {
                            return Err(-110);
                        }
                        if (supportedAppProtocolReq.AppProtocol.arrayLen as i32) < 20 as i32 {
                            grammar_id = 8;
                            continue;
                        } else {
                            grammar_id = 5;
                            continue;
                        }
                    }
                    1 => {
                        //grammar_id = 6 as i32;
                        return Ok(NO_ERROR);
                    }
                    _ => {
                        return Err(-150);
                    }
                }
            }
            5 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                match eventCode {
                    0 => {
                        //grammar_id = 6 as i32;
                        return Ok(NO_ERROR);
                    }
                    _ => {
                        return Err(-150);
                    }
                }
            }
            _ => {
                return Err(-130);
            }
        }
    }
}

pub fn decode_appHand_supportedAppProtocolRes(
    stream: &mut ExiBitstream,
    mut supportedAppProtocolRes: &mut AppHandSupportedAppProtocolRes,
) -> Result<u8, i16> {
    let mut grammar_id: i32 = 9;
    let mut eventCode: u32 = 0;
    loop {
        match grammar_id {
            9 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                match eventCode {
                    0 => {
                        exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                        if eventCode == 0 {
                            let mut value: u32 = 0;
                            exi_basetypes_decoder_nbit_uint(stream, 2, &mut value)?;
                            supportedAppProtocolRes.ResponseCode =
                                AppHandResponseCodeType::from(value);
                        } else {
                            return Err(-151);
                        }

                        exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;

                        if eventCode == 0 {
                            grammar_id = 10;
                            continue;
                        } else {
                            return Err(-170);
                        }
                    }
                    _ => {
                        return Err(-150);
                    }
                }
            }
            10 => {
                exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
                match eventCode {
                    0 => {
                        exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                        if eventCode == 0 {
                            let mut value_0: u32 = 0;
                            exi_basetypes_decoder_nbit_uint(stream, 8, &mut value_0)?;

                            supportedAppProtocolRes.SchemaID = Some(value_0 as u8);
                        } else {
                            return Err(-151);
                        }
                        exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                        if eventCode == 0 {
                            grammar_id = 5;
                            continue;
                        } else {
                            return Err(-170);
                        }
                    }
                    1 => {
                        //grammar_id = 6 as i32;
                        return Ok(NO_ERROR);
                    }
                    _ => {
                        return Err(-150);
                    }
                }
            }
            5 => {
                exi_basetypes_decoder_nbit_uint(stream, 1, &mut eventCode)?;
                match eventCode {
                    0 => {
                        //grammar_id = 6 as i32;
                        return Ok(NO_ERROR);
                    }
                    _ => {
                        return Err(-150);
                    }
                }
            }
            _ => {
                return Err(-130);
            }
        }
    }
}

pub fn decode_appHand_exiDocument(
    stream: &mut ExiBitstream,
    exiDoc: &mut AppHandExiDocument,
) -> Result<u8, i16> {
    let mut eventCode: u32 = 0;
    exi_header_read_and_check(stream)?;
    exi_basetypes_decoder_nbit_uint(stream, 2, &mut eventCode)?;
    match eventCode {
        0 => {
            // Create a new struct, decode into it, then set the enum
            let mut req = AppHandSupportedAppProtocolReq::default();
            decode_appHand_supportedAppProtocolReq(stream, &mut req)?;
            exiDoc.SupportedAppProtocolReq = Some(req)
        }
        1 => {
            let mut res = AppHandSupportedAppProtocolRes::default();
            decode_appHand_supportedAppProtocolRes(stream, &mut res)?;
            exiDoc.SupportedAppProtocolRes = Some(res)
        }
        _ => return Err(UNSUPPORTED_SUB_EVENT),
    }
    Ok(NO_ERROR)
}

