use crate::common::exi_error_codes::ExiError;

pub fn v2gtp_write_header(
    stream_data: &mut [u8],
    stream_payload_length: u32,
) -> Result<(), ExiError> {
    v2gtp20_write_header(stream_data, stream_payload_length, 0x8001)
}

pub fn v2gtp20_write_header(
    stream_data: &mut [u8],
    stream_payload_length: u32,
    v2gtp20_payload_id: u16,
) -> Result<(), ExiError> {
    if stream_data.len() < 8 {
        return Err(ExiError::NotEnoughSpace);
    }
    stream_data[0] = 0x1;
    stream_data[1] = 0xfe;
    stream_data[2] = (v2gtp20_payload_id >> 8) as u8;
    stream_data[3] = (v2gtp20_payload_id & 0xff) as u8;
    stream_data[4] = (stream_payload_length >> 24) as u8;
    stream_data[5] = (stream_payload_length >> 16) as u8;
    stream_data[6] = (stream_payload_length >> 8) as u8;
    stream_data[7] = (stream_payload_length & 0xff) as u8;
    Ok(())
}

pub fn v2gtp_read_header(stream_data: &[u8]) -> Result<usize, ExiError> {
    v2gtp20_read_header(stream_data, 0x8001)
}

pub fn v2gtp20_read_header(stream_data: &[u8], v2gtp20_payload_id: u16) -> Result<usize, ExiError> {
    if stream_data.len() < 8 {
        return Err(ExiError::NotEnoughSpace);
    }
    if stream_data[0] != 0x1 || stream_data[1] != 0xfe {
        return Err(ExiError::BitstreamOverflow);
    }
    let payload_id = (u16::from(stream_data[2]) << 8) | u16::from(stream_data[3]);
    if payload_id != v2gtp20_payload_id {
        return Err(ExiError::UnknownEventCode);
    }
    let stream_payload_length = (u32::from(stream_data[4]) << 24)
        | (u32::from(stream_data[5]) << 16)
        | (u32::from(stream_data[6]) << 8)
        | u32::from(stream_data[7]);
    Ok(stream_payload_length as usize)
}
