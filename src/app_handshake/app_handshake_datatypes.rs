use heapless::String;
use heapless::Vec;

use crate::app_handshake::app_handshake_decoder::decode_app_hand_supported_app_protocol_req;
use crate::app_handshake::app_handshake_encoder::encode_app_hand_supported_app_protocol_req;
use crate::common::exi_basetypes_decoder::exi_basetypes_decoder_nbit_uint;
use crate::common::exi_basetypes_encoder::exi_basetypes_encoder_nbit_uint;
use crate::common::exi_bitstream::{ExiBitstream, exi_bitstream_get_length};
use crate::common::exi_error_codes::ExiError;
use crate::common::exi_header::{exi_header_read_and_check, exi_header_write};

pub enum AppHandResponseCodeType {
    AppHandResponseCodeTypeFailedNoNegotiation = 2,
    AppHandResponseCodeTypeOkSuccessfulNegotiationWithMinorDeviation = 1,
    AppHandResponseCodeTypeOkSuccessfulNegotiation = 0,
}

#[derive(Default)]
pub struct AppHandAppProtocolType {
    pub protocol_namespace: AppHandProtocolNamespaceType,
    pub version_number_major: u32,
    pub version_number_minor: u32,
    pub schema_id: u8,
    pub priority: u8,
}

#[derive(Default)]
pub struct AppHandProtocolNamespaceType {
    pub characters: String<100>,
}

impl AppHandProtocolNamespaceType {
    #[must_use] pub fn new(characters: String<100>) -> Self {
        Self { characters }
    }
}

#[derive(Default)]
pub struct AppHandSupportedAppProtocolReq {
    pub app_protocol: AppHandSupportedAppProtocolReqArray,
}

impl AppHandSupportedAppProtocolReq {
    #[must_use] pub fn new(app_protocol: AppHandSupportedAppProtocolReqArray) -> Self {
        Self { app_protocol }
    }

    pub fn to_bytes(&self) -> Result<([u8; 1024], usize), ExiError> {
        let mut buf = [0u8; 1024];

        let mut stream = ExiBitstream {
            data: &mut buf,
            ..Default::default()
        };

        exi_header_write(&mut stream)?;
        exi_basetypes_encoder_nbit_uint(&mut stream, 2, 0)?;
        encode_app_hand_supported_app_protocol_req(&mut stream, self)?;
        let len = exi_bitstream_get_length(&stream);
        Ok((buf, len))
    }

    pub fn try_from_bytes(bytes: &[u8], len: usize) -> Result<Self, ExiError> {
        let mut buf = [0u8; 1024];
        if len > buf.len() {
            return Err(ExiError::ByteBufferTooSmall);
        }
        buf[..len].copy_from_slice(&bytes[..len]);

        let mut stream = ExiBitstream {
            data: buf.as_mut(),
            ..Default::default()
        };

        let mut req = Self::default();
        let mut event_code: u32 = 0;
        exi_header_read_and_check(&mut stream)?;
        exi_basetypes_decoder_nbit_uint(&mut stream, 2, &mut event_code)?;
        match event_code {
            0 => {
                decode_app_hand_supported_app_protocol_req(&mut stream, &mut req)?;
            }
            _ => {
                return Err(ExiError::UnknownEventForDecoding);
            }
        }
        Ok(req)
    }
}

pub struct AppHandSupportedAppProtocolReqArray {
    pub array: Vec<AppHandAppProtocolType, 5>,
}

impl Default for AppHandSupportedAppProtocolReqArray {
    fn default() -> Self {
        Self { array: Vec::new() }
    }
}

pub struct AppHandSupportedAppProtocolRes {
    pub response_code: AppHandResponseCodeType,
    pub schema_id: Option<u8>,
}

impl Default for AppHandSupportedAppProtocolRes {
    fn default() -> Self {
        Self {
            response_code: AppHandResponseCodeType::AppHandResponseCodeTypeFailedNoNegotiation,
            schema_id: None,
        }
    }
}

impl AppHandSupportedAppProtocolRes {
    #[must_use] pub fn new(response_code: AppHandResponseCodeType, schema_id: Option<u8>) -> Self {
        Self {
            response_code,
            schema_id,
        }
    }

    #[must_use] pub fn to_bytes(&self) -> [u8; 4] {
        let schema_id = self.schema_id.unwrap_or(0);
        let packed: [u8; 4] = match self.response_code {
            AppHandResponseCodeType::AppHandResponseCodeTypeOkSuccessfulNegotiation => [0x80, 0x40, 0x00, schema_id * 0x40],
            AppHandResponseCodeType::AppHandResponseCodeTypeOkSuccessfulNegotiationWithMinorDeviation => [0x80, 0x44, 0x00, schema_id * 0x40],
            AppHandResponseCodeType::AppHandResponseCodeTypeFailedNoNegotiation => [0x80, 0x48, 0x00, 0x00],
        };
        packed
    }
}

#[derive(Default)]
pub struct AppHandExiDocument {
    pub supported_app_protocol_req: Option<AppHandSupportedAppProtocolReq>,
    pub supported_app_protocol_res: Option<AppHandSupportedAppProtocolRes>,
}
