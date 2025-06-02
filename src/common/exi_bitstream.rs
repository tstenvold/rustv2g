use crate::common::exi_error_codes::*;

pub type StatusCallback = Option<fn(i32, i32, i32, i32)>;

#[derive(Debug)]
pub struct ExiBitstream<'a> {
    pub data: &'a mut [u8],
    pub bit_count: u8,
    pub byte_pos: usize,
    pub init_called: bool,
    pub flag_byte_pos: usize,
    pub status_callback: StatusCallback,
}

fn exi_bitstream_has_overflow(stream: &mut ExiBitstream) -> Result<u8, i16> {
    if stream.bit_count == 8 {
        if stream.byte_pos < stream.data.len() {
            stream.byte_pos = stream.byte_pos.wrapping_add(1);
            stream.bit_count = 0;
        } else {
            return Err(BITSTREAM_OVERFLOW);
        }
    }
    Ok(NO_ERROR)
}

fn exi_bitstream_write_bit(stream: &mut ExiBitstream, bit: u8) -> Result<u8, i16> {
    exi_bitstream_has_overflow(stream)?;

    if stream.byte_pos >= stream.data.len() {
        return Err(BITSTREAM_OVERFLOW);
    }

    let current_byte = &mut stream.data[stream.byte_pos];
    if stream.bit_count == 0 {
        *current_byte = 0;
    }
    if bit != 0 {
        *current_byte |= 1 << (7 - stream.bit_count);
    }
    stream.bit_count = stream.bit_count.wrapping_add(1);
    Ok(NO_ERROR)
}

fn exi_bitstream_read_bit(stream: &mut ExiBitstream, bit: &mut u8) -> Result<u8, i16> {
    exi_bitstream_has_overflow(stream)?;

    if stream.byte_pos >= stream.data.len() {
        return Err(BITSTREAM_OVERFLOW);
    }

    let current_bit = (stream.data[stream.byte_pos] >> (7 - stream.bit_count)) & 1;
    *bit = current_bit;
    stream.bit_count = stream.bit_count.wrapping_add(1);
    Ok(NO_ERROR)
}

pub fn exi_bitstream_init<'a>(
    stream: &mut ExiBitstream<'a>,
    data: &'a mut [u8],
    data_offset: usize,
    status_callback: StatusCallback,
) {
    stream.byte_pos = data_offset;
    stream.bit_count = 0;
    stream.data = data;
    stream.init_called = true;
    stream.flag_byte_pos = data_offset;
    stream.status_callback = status_callback;
}

pub fn exi_bitstream_reset(stream: &mut ExiBitstream) {
    if stream.init_called {
        stream.byte_pos = stream.flag_byte_pos;
    } else {
        stream.byte_pos = 0;
    }
    stream.bit_count = 0;
}

pub fn exi_bitstream_get_length(stream: &ExiBitstream) -> usize {
    let mut length = stream.byte_pos;
    if stream.init_called && stream.flag_byte_pos > 0 {
        length = length.wrapping_sub(stream.flag_byte_pos);
    }
    length = length.wrapping_add(if stream.bit_count > 0 { 1 } else { 0 });
    length
}

pub fn exi_bitstream_write_bits(
    stream: &mut ExiBitstream,
    bit_count: usize,
    value: u32,
) -> Result<u8, i16> {
    if bit_count > 32 {
        return Err(-100);
    }

    for idx in 0..bit_count {
        let bit = if value & (1 << (bit_count - idx - 1)) != 0 {
            1
        } else {
            0
        };
        exi_bitstream_write_bit(stream, bit)?;
    }
    Ok(NO_ERROR)
}

pub fn exi_bitstream_write_octet(stream: &mut ExiBitstream, value: u8) -> Result<u8, i16> {
    exi_bitstream_write_bits(stream, 8, value as u32)
}

pub fn exi_bitstream_read_bits(
    stream: &mut ExiBitstream,
    bit_count: usize,
    value: &mut u32,
) -> Result<u8, i16> {
    if bit_count > 32 {
        return Err(-100);
    }
    *value = 0;
    for _ in 0..bit_count {
        let mut bit = 0;
        exi_bitstream_read_bit(stream, &mut bit)?;
        *value = (*value << 1) | (bit as u32);
    }
    Ok(NO_ERROR)
}

pub fn exi_bitstream_read_octet(stream: &mut ExiBitstream, value: &mut u8) -> Result<u8, i16> {
    *value = 0;
    for _ in 0..8 {
        let mut bit = 0;
        exi_bitstream_read_bit(stream, &mut bit)?;
        *value = (*value << 1) | bit;
    }
    Ok(NO_ERROR)
}
