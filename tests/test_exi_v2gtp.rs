#[cfg(test)]
mod tests {
    use rustv2g::exi_v2gtp::{
        v2gtp_write_header,
        v2gtp20_write_header,
        v2gtp_read_header,
        v2gtp20_read_header,
    };
    use rustv2g::common::exi_error_codes::BITSTREAM_OVERFLOW;
    #[test]
    fn test_v2gtp_write_header_success() {
        let mut buf = [0u8; 8];
        let payload_length = 0x12345678;
        let res = v2gtp_write_header(&mut buf, payload_length);
        assert_eq!(res, Ok(0));
        assert_eq!(buf, [0x1, 0xfe, 0x80, 0x01, 0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_v2gtp_write_header_buffer_too_small() {
        let mut buf = [0u8; 7];
        let res = v2gtp_write_header(&mut buf, 0x1234);
        assert_eq!(res, Err(-3));
    }

    #[test]
    fn test_v2gtp20_write_header_custom_payload_id() {
        let mut buf = [0u8; 8];
        let res = v2gtp20_write_header(&mut buf, 0x01020304, 0x1234);
        assert_eq!(res, Ok(0));
        assert_eq!(buf, [0x1, 0xfe, 0x12, 0x34, 0x01, 0x02, 0x03, 0x04]);
    }

    #[test]
    fn test_v2gtp_read_header_success() {
        let buf = [0x1, 0xfe, 0x80, 0x01, 0x00, 0x00, 0x00, 0x10];
        let res = v2gtp_read_header(&buf);
        assert_eq!(res, Ok(16));
    }

    #[test]
    fn test_v2gtp_read_header_too_short() {
        let buf = [0x1, 0xfe, 0x80, 0x01, 0x00, 0x00, 0x00];
        let res = v2gtp_read_header(&buf);
        assert_eq!(res, Err(-3));
    }

    #[test]
    fn test_v2gtp_read_header_wrong_magic() {
        let buf = [0x2, 0xfe, 0x80, 0x01, 0, 0, 0, 0];
        let res = v2gtp_read_header(&buf);
        assert_eq!(res, Err(BITSTREAM_OVERFLOW));
    }

    #[test]
    fn test_v2gtp20_read_header_wrong_payload_id() {
        let buf = [0x1, 0xfe, 0x12, 0x34, 0, 0, 0, 0];
        let res = v2gtp20_read_header(&buf, 0x8001);
        assert_eq!(res, Err(-2));
    }

    #[test]
    fn test_v2gtp20_read_header_custom_payload_id_success() {
        let buf = [0x1, 0xfe, 0x12, 0x34, 0x00, 0x00, 0x00, 0x05];
        let res = v2gtp20_read_header(&buf, 0x1234);
        assert_eq!(res, Ok(5));
    }
}
