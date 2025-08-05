use crate::common::exi_error_codes::ExiError;

pub type StatusCallback = Option<fn(i32, i32, i32, i32)>;

#[derive(Default, Debug)]
pub struct ExiBitstream<'a> {
    pub data: &'a mut [u8],
    pub bit_count: u8,
    pub byte_pos: usize,
    pub flag_byte_pos: usize,
    pub status_callback: StatusCallback,
}

impl ExiBitstream<'_> {
    pub fn reset(&mut self) {
        if self.flag_byte_pos == 0 {
            self.byte_pos = 0;
        } else {
            self.byte_pos = self.flag_byte_pos;
        }
        self.bit_count = 0;
    }

    pub fn has_overflow(&mut self) -> Result<(), ExiError> {
        if self.bit_count == 8 {
            if self.byte_pos < self.data.len() {
                self.byte_pos = self.byte_pos.wrapping_add(1);
                self.bit_count = 0;
            } else {
                return Err(ExiError::BitstreamOverflow);
            }
        }
        Ok(())
    }

    pub fn write_bit(&mut self, bit: u8) -> Result<(), ExiError> {
        self.has_overflow()?;
        if self.byte_pos >= self.data.len() {
            return Err(ExiError::BitstreamOverflow);
        }
        let current_byte = &mut self.data[self.byte_pos];
        if self.bit_count == 0 {
            *current_byte = 0;
        }
        if bit != 0 {
            *current_byte |= 1 << (7 - self.bit_count);
        }
        self.bit_count = self.bit_count.wrapping_add(1);
        Ok(())
    }

    pub fn read_bit(&mut self) -> Result<u8, ExiError> {
        self.has_overflow()?;

        if self.byte_pos >= self.data.len() {
            return Err(ExiError::BitstreamOverflow);
        }

        let current_bit = (self.data[self.byte_pos] >> (7 - self.bit_count)) & 1;
        let bit = current_bit;
        self.bit_count = self.bit_count.wrapping_add(1);
        Ok(bit)
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.byte_pos == 0 && self.bit_count == 0 && self.flag_byte_pos == 0
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        let mut length = self.byte_pos;
        if self.flag_byte_pos != 0 {
            length = length.wrapping_sub(self.flag_byte_pos);
        }
        length = length.wrapping_add(if self.bit_count > 0 { 1 } else { 0 });
        length
    }

    pub fn write_bits(&mut self, bit_count: usize, value: u32) -> Result<(), ExiError> {
        if bit_count > 32 {
            return Err(ExiError::BitCountLargerThanTypeSize);
        }

        for idx in 0..bit_count {
            let bit = if value & (1 << (bit_count - idx - 1)) == 0 {
                0
            } else {
                1
            };
            self.write_bit(bit)?;
        }
        Ok(())
    }

    pub fn write_octet(&mut self, value: u8) -> Result<(), ExiError> {
        self.write_bits(8, u32::from(value))
    }

    pub fn read_bits(&mut self, bit_count: usize) -> Result<u32, ExiError> {
        if bit_count > 32 {
            return Err(ExiError::BitCountLargerThanTypeSize);
        }
        let mut value: u32 = 0;
        for _ in 0..bit_count {
            let bit = self.read_bit()?;
            value = (value << 1) | u32::from(bit);
        }
        Ok(value)
    }

    pub fn read_octet(&mut self) -> Result<u8, ExiError> {
        let mut value: u8 = 0;
        for _ in 0..8 {
            let bit = self.read_bit()?;
            value = (value << 1) | bit;
        }
        Ok(value)
    }

    pub fn write_header(&mut self) -> Result<(), ExiError> {
        self.write_bits(8, 0x80)
    }

    pub fn read_header(&mut self) -> Result<u32, ExiError> {
        self.read_bits(8)
    }

    pub fn read_and_check_header(&mut self) -> Result<(), ExiError> {
        match self.read_header() {
            Ok(header) => {
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
}
