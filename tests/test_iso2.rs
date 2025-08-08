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

        match v2g_msg.body {
            Iso2BodyTypeEnum::SessionSetupReq(ref req) => {
                assert_eq!(req.evcc_id, hexstr_to_bytes("0123456789ab"));
            }
            _ => panic!("Expected SessionSetupReq body type"),
        }
    }
}
