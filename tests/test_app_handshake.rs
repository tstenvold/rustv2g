#[cfg(test)]
mod tests {
    use heapless::String;
    use heapless::Vec;
    use rustv2g::app_handshake::app_handshake_datatypes::*;

    fn hexstr_to_bytes(s: &str) -> Vec<u8, 2048> {
        s.as_bytes()
            .chunks(2)
            .map(|pair| {
                let hex = core::str::from_utf8(pair).unwrap();
                u8::from_str_radix(hex, 16).unwrap()
            })
            .collect()
    }

    fn make_protocol_type() -> AppHandAppProtocolType {
        let namespace: String<100> = String::try_from("urn:iso:15118:2:2016:MsgDef").unwrap();
        let ns = AppHandProtocolNamespaceType::new(namespace);

        AppHandAppProtocolType {
            protocol_namespace: ns,
            version_number_major: 2,
            version_number_minor: 4,
            schema_id: 42,
            priority: 1,
        }
    }

    // Integration tests for the entire handshake process
    #[test]
    fn test_app_handshake_process_req_to_exi() {
        let mut app_req = AppHandSupportedAppProtocolReq::default();
        let _ = app_req.app_protocols.push(make_protocol_type());

        let result = match app_req.to_bytes() {
            Ok((data, len)) => data[..len].to_vec(),
            Err(e) => panic!("Failed to encode AppHandSupportedAppProtocolReq: {:?}", e),
        };

        assert!(!result.is_empty(), "Encoded data should not be empty");
        // try to decode the result and see if it matches the original request
        match AppHandSupportedAppProtocolReq::try_from_bytes(&result, result.len()) {
            Ok(decoded_req) => {
                assert_eq!(
                    decoded_req.app_protocols.len(),
                    app_req.app_protocols.len(),
                    "Decoded app protocol array length should match original"
                );
                assert_eq!(
                    decoded_req.app_protocols[0].protocol_namespace.characters,
                    app_req.app_protocols[0].protocol_namespace.characters,
                    "Decoded protocol namespace should match original"
                );
                assert_eq!(
                    decoded_req.app_protocols[0].version_number_major,
                    app_req.app_protocols[0].version_number_major,
                    "Decoded major version should match original"
                );
                assert_eq!(
                    decoded_req.app_protocols[0].version_number_minor,
                    app_req.app_protocols[0].version_number_minor,
                    "Decoded minor version should match original"
                );
                assert_eq!(
                    decoded_req.app_protocols[0].schema_id, app_req.app_protocols[0].schema_id,
                    "Decoded schema ID should match original"
                );
            }
            Err(e) => panic!("Failed to decode AppHandSupportedAppProtocolReq: {:?}", e),
        }
    }

    #[test]
    fn test_app_handshake_process_exi_to_req() {
        let hex: &str = "8000dbab9371d3234b71d1b981899189d191818991d26b9b3a232b300200200c0001d75726e3a69736f3a31353131383a323a323031333a4d73674465660020000080880";
        let buf = hexstr_to_bytes(hex);

        let app_req = match AppHandSupportedAppProtocolReq::try_from_bytes(&buf, buf.len()) {
            Ok(app_req) => {
                assert!(
                    !app_req.app_protocols.is_empty(),
                    "App protocol array should not be empty"
                );
                assert_eq!(
                    app_req.app_protocols[0].protocol_namespace.characters,
                    "urn:din:70121:2012:MsgDef",
                    "Protocol namespace should match"
                );
                assert_eq!(
                    app_req.app_protocols[0].version_number_major, 2,
                    "Major version should match"
                );
                assert_eq!(
                    app_req.app_protocols[0].version_number_minor, 1,
                    "Minor version should match"
                );
                assert_eq!(
                    app_req.app_protocols[0].schema_id, 3,
                    "Schema ID should match"
                );
                assert_eq!(
                    app_req.app_protocols[0].priority, 1,
                    "Priority should match"
                );

                assert_eq!(
                    app_req.app_protocols[1].protocol_namespace.characters,
                    "urn:iso:15118:2:2013:MsgDef",
                    "Protocol namespace should match"
                );
                assert_eq!(
                    app_req.app_protocols[1].version_number_major, 1,
                    "Major version should match"
                );
                assert_eq!(
                    app_req.app_protocols[1].version_number_minor, 0,
                    "Minor version should match"
                );
                assert_eq!(
                    app_req.app_protocols[1].schema_id, 1,
                    "Schema ID should match"
                );
                assert_eq!(
                    app_req.app_protocols[1].priority, 2,
                    "Priority should match"
                );
                app_req
            }
            Err(e) => panic!("Failed to decode AppHandSupportedAppProtocolReq: {:?}", e),
        };

        // Encode the request back to EXI and check if it matches the original
        let encoded_result = app_req.to_bytes();
        assert!(
            encoded_result.is_ok(),
            "Encoding AppHandSupportedAppProtocolReq failed: {:?}",
            encoded_result.err()
        );

        //make sure encoded result and buf are the same
        let (encoded_data, len) = encoded_result.unwrap();
        assert_eq!(
            encoded_data[..len],
            buf,
            "Encoded data should match original EXI bytes"
        );
    }
}
