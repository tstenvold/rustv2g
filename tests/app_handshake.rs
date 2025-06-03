#[cfg(test)]
mod tests {
    use rustv2g::app_handshake::appHand_Datatypes::*;
    use rustv2g::app_handshake::appHand_Encoder::*;
    use rustv2g::common::exi_error_codes::*;
    use rustv2g::common::exi_basetypes::*;
    use rustv2g::common::exi_bitstream::*;


    fn make_protocol_type() -> AppHandAppProtocolType {
        AppHandAppProtocolType {
            ProtocolNamespace: AppHandProtocolNamespaceType {
                characters: {
                    let mut arr = [0u8; 100];
                    let bytes = b"urn:iso:15118:2:2013:MsgDef";
                    arr[..bytes.len()].copy_from_slice(bytes);
                    arr
                },
                charactersLen: 27,
            },
            VersionNumberMajor: 2,
            VersionNumberMinor: 1,
            SchemaID: 42,
            Priority: 3,
        }
    }

    fn make_supported_app_protocol_req() -> AppHandSupportedAppProtocolReq {
        AppHandSupportedAppProtocolReq {
            AppProtocol: AppHandSupportedAppProtocolReqArray {
                array: [make_protocol_type(); 5],
                arrayLen: 2,
            },
        }
    }

    fn make_supported_app_protocol_res(schema_id: Option<u8>) -> AppHandSupportedAppProtocolRes {
        AppHandSupportedAppProtocolRes {
            ResponseCode: AppHandResponseCodeType::AppHandResponseCodeTypeOkSuccessfulNegotiationWithMinorDeviation,
            SchemaID: schema_id,
        }
    }

    #[test]
    fn test_encode_appHand_AppProtocolType_ok() {
        let mut buffer = [0u8; 1024];
        let mut stream = ExiBitstream::new(&mut buffer);

        let protocol = make_protocol_type();
        let result = encode_appHand_AppProtocolType(&mut stream, protocol);
        assert_eq!(result, Ok(NO_ERROR));
    }

    #[test]
    fn test_encode_appHand_supportedAppProtocolReq_ok() {
        let mut buffer = [0u8; 1024];
        let mut stream = ExiBitstream::new(&mut buffer);
        let req = make_supported_app_protocol_req();
        let result = encode_appHand_supportedAppProtocolReq(&mut stream, req);
        assert_eq!(result, Ok(NO_ERROR));
    }

    #[test]
    fn test_encode_appHand_supportedAppProtocolRes_with_schema_id() {
        let mut buffer = [0u8; 1024];
        let mut stream = ExiBitstream::new(&mut buffer);
        let res = make_supported_app_protocol_res(Some(1));
        let result = encode_appHand_supportedAppProtocolRes(&mut stream, res);
        assert_eq!(result, Ok(0));
    }

    #[test]
    fn test_encode_appHand_supportedAppProtocolRes_without_schema_id() {
        let mut buffer = [0u8; 1024];
        let mut stream = ExiBitstream::new(&mut buffer);
        let res = make_supported_app_protocol_res(None);
        let result = encode_appHand_supportedAppProtocolRes(&mut stream, res);
        assert_eq!(result, Ok(0));
    }

    #[test]
    fn test_encode_appHand_exiDocument_with_req() {
        let mut buffer = [0u8; 1024];
        let mut stream = ExiBitstream::new(&mut buffer);
        let doc = AppHandExiDocument {
            SupportedAppProtocolReq: Some(make_supported_app_protocol_req()),
            SupportedAppProtocolRes: None,
        };
        let result = encode_appHand_exiDocument(&mut stream, doc);
        assert_eq!(result, Ok(NO_ERROR));
    }

    #[test]
    fn test_encode_appHand_exiDocument_with_res() {
        let mut buffer = [0u8; 1024];
        let mut stream = ExiBitstream::new(&mut buffer);
        let doc = AppHandExiDocument {
            SupportedAppProtocolReq: None,
            SupportedAppProtocolRes: Some(make_supported_app_protocol_res(Some(1))),
        };
        let result = encode_appHand_exiDocument(&mut stream, doc);
        assert_eq!(result, Ok(NO_ERROR));
    }

    #[test]
    fn test_encode_appHand_exiDocument_with_none() {
        let mut buffer = [0u8; 1024];
        let mut stream = ExiBitstream::new(&mut buffer);
        let doc = AppHandExiDocument {
            SupportedAppProtocolReq: None,
            SupportedAppProtocolRes: None,
        };
        let result = encode_appHand_exiDocument(&mut stream, doc);
        assert_eq!(result, Err(UNKNOWN_EVENT_FOR_ENCODING));
    }

    #[test]
    fn test_encode_appHand_supportedAppProtocolReq_empty_array() {
        let mut buffer = [0u8; 1024];
        let mut stream = ExiBitstream::new(&mut buffer);
        let req = AppHandSupportedAppProtocolReq {
            AppProtocol: AppHandSupportedAppProtocolReqArray {
                array: [make_protocol_type(); 5],
                arrayLen: 0,
            },
        };
        let result = encode_appHand_supportedAppProtocolReq(&mut stream, req);
        assert_eq!(result, Err(UNKNOWN_EVENT_CODE));
    }
}
