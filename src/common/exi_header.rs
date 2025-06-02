use core::result::Result;

use crate::common::exi_bitstream::*;
use crate::common::exi_error_codes::*;

pub fn exi_header_write(stream: &mut ExiBitstream) -> Result<u8, i16> {
    return exi_bitstream_write_bits(stream, 8, 0x80);
}

pub fn exi_header_read(stream: &mut ExiBitstream, mut header: &mut u32) -> Result<u8, i16> {
    return exi_bitstream_read_bits(stream, 8, &mut header);
}

pub fn exi_header_read_and_check(mut stream: &mut ExiBitstream) -> Result<u8, i16> {
    let mut header: u32 = 0;
    match exi_header_read(&mut stream, &mut header) {
        Ok(_) => {
            if header != 0x80 {
                return Err(-22);
            }
        }
        Err(e) => {
            return Err(e);
        }
    }
    Ok(NO_ERROR)
}
