use core::result::Result;

use crate::common::exi_bitstream::*;
use crate::common::exi_error_codes::*;

pub fn exi_header_write(stream: &mut ExiBitstream) -> Result<(), ExiError> {
    exi_bitstream_write_bits(stream, 8, 0x80)
}

pub fn exi_header_read(stream: &mut ExiBitstream, header: &mut u32) -> Result<(), ExiError> {
    exi_bitstream_read_bits(stream, 8, header)
}

pub fn exi_header_read_and_check(stream: &mut ExiBitstream) -> Result<(), ExiError> {
    let mut header: u32 = 0;
    match exi_header_read(stream, &mut header) {
        Ok(_) => {
            if header != 0x80 {
                return Err(ExiError::HeaderIncorrect);
            }
        }
        Err(e) => {
            return Err(e);
        }
    }
    Ok(())
}
