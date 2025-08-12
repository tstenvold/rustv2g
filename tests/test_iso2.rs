#[cfg(test)]
mod tests {
    use heapless::Vec;
    use rustv2g::iso_2::iso2_datatypes::*;

    fn hexstr_to_bytes(s: &str) -> Vec<u8, 2048> {
        s.as_bytes()
            .chunks(2)
            .map(|pair| {
                let hex = core::str::from_utf8(pair).unwrap();
                u8::from_str_radix(hex, 16).unwrap()
            })
            .collect()
    }

    // Integration tests for the ISO 2 data types
    #[test]
    fn test_iso2_session_setup_req() {
        let hex_data = "8098004011d018048d159e26ac00";
        let bytes: Vec<u8, 2048> = hexstr_to_bytes(hex_data);
        let mut bytes_array = [0u8; 2048];
        bytes_array[..bytes.len()].copy_from_slice(&bytes);
        let v2g_msg = Iso2v2gMessage::try_from_bytes::<2048>(&mut bytes_array)
            .expect("Failed to decode Iso2SessionSetupReq");

        let evcc_id_bytes = hexstr_to_bytes("0123456789ab");
        let evcc_id: Vec<u8, 6> = Vec::from_slice(&evcc_id_bytes).unwrap();
        let session_id_bytes = hexstr_to_bytes("00");
        let session_id: Vec<u8, 8> = Vec::from_slice(&session_id_bytes).unwrap();

        match v2g_msg.body {
            Iso2BodyTypeEnum::SessionSetupReq(ref req) => {
                assert_eq!(req.evcc_id, evcc_id);
            }
            _ => panic!("Expected SessionSetupReq body type"),
        }

        // Build a new request to test encoding
        let req: Iso2v2gMessage = Iso2v2gMessage {
            header: Iso2MessageHeaderType {
                session_id,
                notification: None,
                signature: None,
            },
            body: Iso2BodyTypeEnum::SessionSetupReq(Iso2SessionSetupReqType { evcc_id }),
        };

        let (encoded, len) = req
            .to_exi_bytes::<2048>()
            .expect("Failed to encode request");
        assert_eq!(encoded[..len], bytes);
    }

    #[test]
    fn test_iso2_session_setup_res() {
        let hex_data = "8098020088d55a226af37bd1e0202905090d111518c4c880";
        let bytes: Vec<u8, 2048> = hexstr_to_bytes(hex_data);
        let mut bytes_array = [0u8; 2048];
        bytes_array[..bytes.len()].copy_from_slice(&bytes);
        let v2g_msg = Iso2v2gMessage::try_from_bytes::<2048>(&mut bytes_array)
            .expect("Failed to decode Iso2SessionSetupRes");

        match v2g_msg.body {
            Iso2BodyTypeEnum::SessionSetupRes(ref res) => {
                assert_eq!(res.evse_id, b"ABCDEF12");
            }
            _ => panic!("Expected SessionSetupRes body type"),
        }

        let session_id_bytes = hexstr_to_bytes("0223556889ABCDEF");
        let session_id: Vec<u8, 8> = Vec::from_slice(&session_id_bytes).unwrap();

        // Build a new response to test encoding
        let res: Iso2v2gMessage = Iso2v2gMessage {
            header: Iso2MessageHeaderType {
                session_id,
                notification: None,
                signature: None,
            },
            body: Iso2BodyTypeEnum::SessionSetupRes(Iso2SessionSetupResType {
                response_code: Iso2ResponseCodeType::OkNewSessionEstablished,
                evse_id: Vec::from_slice(b"ABCDEF12").unwrap(),
                evse_time_stamp: None,
            }),
        };

        let (encoded, len) = res
            .to_exi_bytes::<2048>()
            .expect("Failed to encode response");
        assert_eq!(encoded[..len], bytes);
    }

    #[test]
    fn test_iso2_service_discovery_req() {
        let hex_data = "8098004011b01c8af0c2dae0d8caa6c6dee0ca0000";
        let bytes: Vec<u8, 2048> = hexstr_to_bytes(hex_data);
        let mut bytes_array = [0u8; 2048];
        bytes_array[..bytes.len()].copy_from_slice(&bytes);
        let v2g_msg = Iso2v2gMessage::try_from_bytes::<2048>(&mut bytes_array)
            .expect("Failed to decode Iso2ServiceDiscoveryReq");

        let service_list = b"ExampleScope";
        let session_id_bytes = hexstr_to_bytes("00");
        let session_id: Vec<u8, 8> = Vec::from_slice(&session_id_bytes).unwrap();

        // TODO: Add assertions for the decoded request fields
        match v2g_msg.body {
            Iso2BodyTypeEnum::ServiceDiscoveryReq(ref req) => {
                // Example assertion (replace with actual expected values):
                assert_eq!(
                    req.service_scope,
                    Some(Vec::from_slice(service_list).unwrap())
                );
                assert_eq!(
                    req.service_category,
                    Some(Iso2ServiceCategoryType::EvCharging)
                );
            }
            _ => panic!("Expected ServiceDiscoveryReq body type"),
        }

        // // TODO: Implement Encoding
        // let req: Iso2v2gMessage = Iso2v2gMessage {
        //     header: Iso2MessageHeaderType {
        //         session_id: session_id,
        //         notification: None,
        //         signature: None,
        //     },
        //     body: Iso2BodyTypeEnum::ServiceDiscoveryReq(Iso2ServiceDiscoveryReqType {
        //         service_scope: Some(Vec::from_slice(service_list).unwrap()),
        //         service_category: Some(Iso2ServiceCategoryType::EvCharging),
        //     }),
        // };

        // let (encoded, len) = req
        //     .to_exi_bytes::<2048>()
        //     .expect("Failed to encode request");
        // assert_eq!(encoded[..len], bytes);
    }

    #[test]
    fn test_iso2_service_discovery_res() {
        // TODO: Replace with the correct EXI hex string for ServiceDiscoveryRes
        let hex_data = "8098004011c00008004041050d7d110d7d0da185c99da5b99c0502400300f4661737420496e7465726e657409000200d4365727469666963617465110400";
        let bytes: Vec<u8, 2048> = hexstr_to_bytes(hex_data);
        let mut bytes_array = [0u8; 2048];
        bytes_array[..bytes.len()].copy_from_slice(&bytes);
        let v2g_msg = Iso2v2gMessage::try_from_bytes::<2048>(&mut bytes_array)
            .expect("Failed to decode Iso2ServiceDiscoveryRes");

        // TODO: Add assertions for the decoded response fields
        match v2g_msg.body {
            Iso2BodyTypeEnum::ServiceDiscoveryRes(ref res) => {
                assert_eq!(res.response_code, Iso2ResponseCodeType::Ok);
                assert_eq!(res.charge_service.free_service, 1);
                assert_eq!(
                    res.charge_service.service_category,
                    Iso2ServiceCategoryType::EvCharging
                );
                assert_eq!(res.charge_service.service_id, 1);
            }
            _ => panic!("Expected ServiceDiscoveryRes body type"),
        }

        //     // TODO: Build a new response to test encoding (replace with actual fields)
        //     let res: Iso2v2gMessage = Iso2v2gMessage {
        //         header: Iso2MessageHeaderType {
        //             session_id: Vec::new(), // Replace with actual session_id
        //             notification: None,
        //             signature: None,
        //         },
        //         body: Iso2BodyTypeEnum::ServiceDiscoveryRes{

        //         },
        //     };

        //     let (encoded, len) = res
        //         .to_exi_bytes::<2048>()
        //         .expect("Failed to encode response");
        //     assert_eq!(encoded[..len], bytes);
    }
}
