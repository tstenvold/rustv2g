#![allow(non_snake_case)]

#[cfg(test)]
mod tests {
    use rustv2g::app_handshake::appHand_Datatypes::*;
    use rustv2g::app_handshake::appHand_Encoder::*;
    use rustv2g::common::exi_error_codes::*;
    use rustv2g::common::exi_bitstream::*;
    use rustv2g::app_handshake::appHand_Decoder::*;


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
                arrayLen: 1,
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
        let mut stream = ExiBitstream::default();
        exi_bitstream_init(&mut stream, &mut buffer, 0, None);

        let protocol = make_protocol_type();
        let result = encode_appHand_AppProtocolType(&mut stream, protocol);
        assert_eq!(result, Ok(NO_ERROR));
    }

    #[test]
    fn test_encode_appHand_supportedAppProtocolReq_ok() {
        let mut buffer = [0u8; 1024];
        let mut stream = ExiBitstream::default();
        exi_bitstream_init(&mut stream, &mut buffer, 0, None);
        let req = make_supported_app_protocol_req();
        let result = encode_appHand_supportedAppProtocolReq(&mut stream, req);
        assert_eq!(result, Ok(NO_ERROR));
    }

    #[test]
    fn test_encode_appHand_supportedAppProtocolRes_with_schema_id() {
        let mut buffer = [0u8; 1024];
        let mut stream = ExiBitstream::default();
        exi_bitstream_init(&mut stream, &mut buffer, 0, None);
        let res = make_supported_app_protocol_res(Some(1));
        let result = encode_appHand_supportedAppProtocolRes(&mut stream, res);
        assert_eq!(result, Ok(0));
    }

    #[test]
    fn test_encode_appHand_supportedAppProtocolRes_without_schema_id() {
        let mut buffer = [0u8; 1024];
        let mut stream = ExiBitstream::default();
        exi_bitstream_init(&mut stream, &mut buffer, 0, None);
        let res = make_supported_app_protocol_res(None);
        let result = encode_appHand_supportedAppProtocolRes(&mut stream, res);
        assert_eq!(result, Ok(0));
    }

    #[test]
    fn test_encode_appHand_exiDocument_with_req() {
        let mut buffer = [0u8; 1024];
        let mut stream = ExiBitstream::default();
        exi_bitstream_init(&mut stream, &mut buffer, 0, None);
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
        let mut stream = ExiBitstream::default();
        exi_bitstream_init(&mut stream, &mut buffer, 0, None);
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
        let mut stream = ExiBitstream::default();
        exi_bitstream_init(&mut stream, &mut buffer, 0, None);
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
        let mut stream = ExiBitstream::default();
        exi_bitstream_init(&mut stream, &mut buffer, 0, None);
        let req = AppHandSupportedAppProtocolReq {
            AppProtocol: AppHandSupportedAppProtocolReqArray {
                array: [make_protocol_type(); 5],
                arrayLen: 0,
            },
        };
        let result = encode_appHand_supportedAppProtocolReq(&mut stream, req);
        assert_eq!(result, Err(UNKNOWN_EVENT_CODE));
    }

    // Decoder Tests

    #[test]
    fn test_decode_appHand_supportedAppProtocolReq_too_many() {
        let mut req = AppHandSupportedAppProtocolReq::default();
        req.AppProtocol.arrayLen = 5; // Already at max
        let mut data = [0b00000000];
        let mut stream = ExiBitstream::default();
        exi_bitstream_init(&mut stream, &mut data, 0, None);
        let result = decode_appHand_supportedAppProtocolReq(&mut stream, &mut req);
        assert_eq!(result, Err(-110));
    }

    #[test]
    fn test_decode_appHand_supportedAppProtocolRes_response_code() {
        let mut data = [
            0b00000000, // eventCode = 0
            0b00000000, // eventCode = 0
            0b00000010, // value = 2
            0b00000000, // eventCode = 0
            0b00000001, // eventCode = 1 (end)
        ];
        let mut stream = ExiBitstream::default();
        exi_bitstream_init(&mut stream, &mut data, 0, None);
        let mut res = AppHandSupportedAppProtocolRes::default();
        let result = decode_appHand_supportedAppProtocolRes(&mut stream, &mut res);
        assert_eq!(result, Ok(NO_ERROR));
        assert_eq!(res.ResponseCode,
            AppHandResponseCodeType::from(0));
    }

    #[test]
    fn test_decode_appHand_exiDocument_unsupported_event() {
        // header ok, eventCode = 2 (unsupported)
        let mut data = [0] as [u8; 1];

        let mut stream = ExiBitstream::default();
        exi_bitstream_init(&mut stream, &mut data, 0, None);

        let mut doc = AppHandExiDocument::default();
        let result = decode_appHand_exiDocument(&mut stream, &mut doc);
        assert_eq!(result, Err(-22));
    }

    #[test]
    fn test_decode_appHand_AppProtocolType_invalid_namespace_len() {
        // eventCode = 0, eventCode = 0, charactersLen = 1 (invalid, < 2)
        let mut data = [
            0b00000000, // eventCode = 0
            0b00000000, // eventCode = 0
            0x01, 0x00, // charactersLen = 1
        ];
        let mut stream = ExiBitstream::default();
        exi_bitstream_init(&mut stream, &mut data, 0, None);
        let mut proto: AppHandAppProtocolType = AppHandAppProtocolType::default();
        let result = decode_appHand_AppProtocolType(&mut stream, &mut proto);
        assert_eq!(result, Err(-200));
    }

    // Integration tests for the entire handshake process
    #[test]
    fn test_app_handshake_process_res() {
        let app_res = AppHandSupportedAppProtocolRes {
            ResponseCode: AppHandResponseCodeType::AppHandResponseCodeTypeOkSuccessfulNegotiation,
            SchemaID: Some(1),
        };
        let exi_response = AppHandExiDocument {
            SupportedAppProtocolRes: Some(app_res),
            ..Default::default()
        };

        let mut resp_buf = [0; 1024];

        let mut response_stream = ExiBitstream {
            data: &mut resp_buf,
            bit_count: 0,
            byte_pos: 0,
            init_called: true,
            flag_byte_pos: 0,
            status_callback: None,
        };

        // Encode the response
        let res = encode_appHand_exiDocument(&mut response_stream, exi_response);
        assert_eq!(res, Ok(NO_ERROR));

        // Now decode the response
        let mut decoded_stream = ExiBitstream {
            data: &mut resp_buf,
            bit_count: 0,
            byte_pos: 0,
            init_called: true,
            flag_byte_pos: 0,
            status_callback: None,
        };
        let mut decoded_doc = AppHandExiDocument::default();
        let decode_result = decode_appHand_exiDocument(&mut decoded_stream, &mut decoded_doc);
        assert_eq!(decode_result, Ok(NO_ERROR));
        assert!(decoded_doc.SupportedAppProtocolRes.is_some());
        let res = decoded_doc.SupportedAppProtocolRes.unwrap();
        assert_eq!(res.ResponseCode, AppHandResponseCodeType::AppHandResponseCodeTypeOkSuccessfulNegotiation);
        assert_eq!(res.SchemaID, Some(1));
    }

    #[test]
    fn test_app_handshake_process_req() {

        let app_protocol_name= AppHandProtocolNamespaceType {
            characters: {
                let mut arr = [0u8; 100];
                let bytes = b"urn:iso:15118:3:2015:MsgDef";
                arr[..bytes.len()].copy_from_slice(bytes);
                arr
            },
            charactersLen: 27,
        };

        let proto = AppHandAppProtocolType {
            ProtocolNamespace: app_protocol_name,
            VersionNumberMajor: 3,
            VersionNumberMinor: 5,
            SchemaID: 2,
            Priority: 1,
        };

        let app_req = AppHandSupportedAppProtocolReq {
            AppProtocol: AppHandSupportedAppProtocolReqArray {
                array: [proto; 5],
                arrayLen: 1,
            },
        };
        let exi_request = AppHandExiDocument {
            SupportedAppProtocolReq: Some(app_req),
            ..Default::default()
        };

        let mut req_buf = [0; 1024];

        let mut request_stream = ExiBitstream {
            data: &mut req_buf,
            bit_count: 0,
            byte_pos: 0,
            init_called: true,
            flag_byte_pos: 0,
            status_callback: None,
        };

        // Encode the request
        let res = encode_appHand_exiDocument(&mut request_stream, exi_request);
        assert_eq!(res, Ok(NO_ERROR));

        // Now decode the request
        let mut decoded_stream = ExiBitstream {
            data: &mut req_buf,
            bit_count: 0,
            byte_pos: 0,
            init_called: true,
            flag_byte_pos: 0,
            status_callback: None,
        };
        let mut decoded_doc = AppHandExiDocument::default();
        let decode_result = decode_appHand_exiDocument(&mut decoded_stream, &mut decoded_doc);
        assert_eq!(decode_result, Ok(NO_ERROR));
        assert!(decoded_doc.SupportedAppProtocolReq.is_some());
    }
}
