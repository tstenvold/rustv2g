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
        let mut bytes = hexstr_to_bytes(hex_data);
        let len = bytes.len();

        let session_setup_req = Iso2SessionSetupReqType::try_from_bytes(&mut bytes, len)
            .expect("Failed to decode Iso2SessionSetupReq");

        println!("{:?}", session_setup_req.evcc_id);
    }
}
