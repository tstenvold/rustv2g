use crate::common::exi_error_codes::ExiError;
use core::cmp::min;
use heapless::Vec;

pub const EXI_UNSIGNED_MAX_OCTETS: usize = 29;

pub type ExiChar = u8;

#[derive(Default)]
pub struct ExiUnsigned {
    pub octets: Vec<u8, EXI_UNSIGNED_MAX_OCTETS>,
}

impl ExiUnsigned {
    #[must_use]
    pub const fn new() -> Self {
        Self { octets: Vec::new() }
    }

    pub fn convert_to_unsigned(
        &mut self,
        mut value: u32,
        max_octets: usize,
    ) -> Result<(), ExiError> {
        for _ in 0..max_octets {
            let mut octet = (value & 0x7f) as u8;
            value >>= 7;
            if value != 0 {
                octet |= 0x80;
            }
            self.octets
                .push(octet)
                .map_err(|_| ExiError::OctetCountLargerThanTypeSupports)?;
            if value == 0 {
                break;
            }
        }
        Ok(())
    }

    pub fn convert_64_to_unsigned(&mut self, mut value: u64) -> Result<(), ExiError> {
        for _ in 0..10 {
            let mut octet = (value & 0x7f) as u8;
            value >>= 7;
            if value != 0 {
                octet |= 0x80;
            }
            self.octets
                .push(octet)
                .map_err(|_| ExiError::OctetCountLargerThanTypeSupports)?;
            if value == 0 {
                break;
            }
        }
        Ok(())
    }

    pub fn convert_from_unsigned(
        &mut self,
        value: &mut u32,
        max_octets: usize,
    ) -> Result<(), ExiError> {
        *value = 0;
        for (n, &octet) in self
            .octets
            .iter()
            .take(min(max_octets, self.octets.len()))
            .enumerate()
        {
            *value = value.wrapping_add(u32::from(octet & 0x7f) << (n * 7));
        }
        Ok(())
    }

    pub fn convert_64_from_unsigned(&mut self, value: &mut u64) -> Result<(), ExiError> {
        if self.octets.len() > 10 {
            return Err(ExiError::OctetCountLargerThanTypeSupports);
        }
        *value = 0;
        for (n, &octet) in self.octets.iter().enumerate() {
            *value = value.wrapping_add(u64::from(octet & 0x7f) << (n * 7));
        }
        Ok(())
    }

    pub fn convert_bytes_from_unsigned(
        &mut self,
        data: &mut [u8],
        data_len: &mut usize,
    ) -> Result<(), ExiError> {
        let mut temp: u16 = 0;
        *data_len = 0;
        let mut total_offset: u16 = 0;
        let mut n: usize = 0;
        while n < self.octets.len() {
            temp += u16::from(self.octets[n] & 0x7f) << total_offset;
            total_offset += 7;
            if total_offset >= 8 {
                if *data_len >= data.len() {
                    return Err(ExiError::EncodedIntegerSizeLargerThanDestination);
                }
                total_offset -= 8;
                data[*data_len] = (temp & 0xff) as u8;
                *data_len += 1;
                temp >>= 8;
            }
            n += 1;
        }
        if total_offset != 0 && (temp & 0xff) != 0 {
            if *data_len >= data.len() {
                return Err(ExiError::EncodedIntegerSizeLargerThanDestination);
            }
            data[*data_len] = (temp & 0xff) as u8;
            *data_len += 1;
        }
        reverse_array(&mut data[..*data_len]);
        Ok(())
    }

    pub fn convert_bytes_to_unsigned(&mut self, data: &[u8]) -> Result<(), ExiError> {
        let mut bytenum = 0;
        while bytenum < data.len() {
            if data[bytenum] != 0 {
                break;
            }
            bytenum += 1;
        }
        if bytenum == data.len() {
            self.octets
                .push(0)
                .map_err(|_| ExiError::OctetCountLargerThanTypeSupports)?;
            return Ok(());
        }
        let byte = data[bytenum];
        let mut bits_in_byte = 0;
        let mut tmp = byte;
        while tmp != 0 {
            bits_in_byte += 1;
            tmp >>= 1;
        }
        let total_relevant_input_bits = ((data.len() - bytenum - 1) * 8) + bits_in_byte;
        let exi_expected_octets_count = (total_relevant_input_bits + 6) / 7;
        let mut dummy: u16 = 0;
        let mut dummy_count: u8 = 0;
        let mut incount: usize = 0;
        let mut outcount: usize = 0;
        while outcount < exi_expected_octets_count {
            if dummy_count < 7 && incount < data.len() - bytenum {
                dummy |= u16::from(data[data.len() - incount - 1]) << dummy_count;
                dummy_count += 8;
                incount += 1;
            }
            self.octets
                .push(
                    ((dummy & 0x7f) | 0x80)
                        .try_into()
                        .map_err(|_| ExiError::InvalidValue)?,
                )
                .map_err(|_| ExiError::OctetCountLargerThanTypeSupports)?;
            dummy >>= 7;
            dummy_count -= 7;
            outcount += 1;
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct ExiSigned {
    pub data: ExiUnsigned,
    pub is_negative: u8,
}

impl ExiSigned {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn convert_to_signed(&mut self, value: i32, max_octets: usize) -> Result<(), ExiError> {
        if value < 0 {
            self.is_negative = 1;
            self.data.convert_to_unsigned(
                (-value).try_into().map_err(|_| ExiError::InvalidValue)?,
                max_octets,
            )
        } else {
            self.is_negative = 0;
            self.data.convert_to_unsigned(
                value.try_into().map_err(|_| ExiError::InvalidValue)?,
                max_octets,
            )
        }
    }

    pub fn convert_64_to_signed(&mut self, value: i64) -> Result<(), ExiError> {
        if value < 0 {
            self.is_negative = 1;
            self.data
                .convert_64_to_unsigned((-value).try_into().map_err(|_| ExiError::InvalidValue)?)
        } else {
            self.is_negative = 0;
            self.data
                .convert_64_to_unsigned(value.try_into().map_err(|_| ExiError::InvalidValue)?)
        }
    }

    pub fn convert_from_signed(
        &mut self,
        value: &mut i32,
        max_octets: usize,
    ) -> Result<(), ExiError> {
        let mut u_value: u32 = 0;
        let res = self.data.convert_from_unsigned(&mut u_value, max_octets);
        *value = if self.is_negative == 0 {
            u_value.try_into().map_err(|_| ExiError::InvalidValue)?
        } else {
            (i32::try_from(u_value).map_err(|_| ExiError::InvalidValue)?).wrapping_neg()
        };
        res
    }

    pub fn convert_64_from_signed(&mut self, value: &mut i64) -> Result<(), ExiError> {
        let mut u_value: u64 = 0;
        let res = self.data.convert_64_from_unsigned(&mut u_value);
        *value = if self.is_negative == 0 {
            i64::try_from(u_value).map_err(|_| ExiError::InvalidValue)?
        } else {
            i64::try_from(u_value)
                .map_err(|_| ExiError::InvalidValue)?
                .wrapping_neg()
        };
        res
    }
}

fn reverse_array(data: &mut [u8]) {
    data.reverse();
}
