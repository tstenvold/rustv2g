use crate::lib::cbv2g::common::exi_error_codes::*;

pub fn v2gtp_write_header(
    stream_data: &mut [u8],
    stream_payload_length: u32,
) -> Result<usize, i16> {
    v2gtp20_write_header(stream_data, stream_payload_length, 0x8001)
}

pub fn v2gtp20_write_header(
    stream_data: &mut [u8],
    stream_payload_length: u32,
    v2gtp20_payload_id: u16,
) -> Result<usize, i16> {
    if stream_data.len() < 8 {
        return Err(-3); // Not enough space
    }
    stream_data[0] = 0x1;
    stream_data[1] = 0xfe;
    stream_data[2] = (v2gtp20_payload_id >> 8) as u8;
    stream_data[3] = (v2gtp20_payload_id & 0xff) as u8;
    stream_data[4] = (stream_payload_length >> 24) as u8;
    stream_data[5] = (stream_payload_length >> 16) as u8;
    stream_data[6] = (stream_payload_length >> 8) as u8;
    stream_data[7] = (stream_payload_length & 0xff) as u8;
    Ok(NO_ERROR as usize)
}

pub fn v2gtp_read_header(stream_data: &[u8]) -> Result<usize, i16> {
    v2gtp20_read_header(stream_data, 0x8001)
}

pub fn v2gtp20_read_header(stream_data: &[u8], v2gtp20_payload_id: u16) -> Result<usize, i16> {
    if stream_data.len() < 8 {
        return Err(-3); // Not enough data
    }
    if stream_data[0] != 0x1 || stream_data[1] != 0xfe {
        return Err(BITSTREAM_OVERFLOW);
    }
    let payload_id = ((stream_data[2] as u16) << 8) | (stream_data[3] as u16);
    if payload_id != v2gtp20_payload_id {
        return Err(-2);
    }
    let stream_payload_length = ((stream_data[4] as u32) << 24)
        | ((stream_data[5] as u32) << 16)
        | ((stream_data[6] as u32) << 8)
        | (stream_data[7] as u32);
    Ok(stream_payload_length as usize)
}
