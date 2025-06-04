use crate::common::exi_error_codes::*;

const EXI_UNSIGNED_MAX_OCTETS: usize = 29;

pub type ExiChar = u8;

#[derive(Copy, Clone)]
pub struct ExiUnsigned {
    pub octets: [u8; EXI_UNSIGNED_MAX_OCTETS],
    pub octets_count: usize,
}

impl Default for ExiUnsigned {
    fn default() -> Self {
        ExiUnsigned {
            octets: [0; EXI_UNSIGNED_MAX_OCTETS],
            octets_count: 0,
        }
    }
}
impl ExiUnsigned {
    pub fn new() -> Self {
        ExiUnsigned::default()
    }
}

#[derive(Copy, Clone)]
pub struct ExiSigned {
    pub data: ExiUnsigned,
    pub is_negative: u8,
}

impl Default for ExiSigned {
    fn default() -> Self {
        ExiSigned {
            data: ExiUnsigned::default(),
            is_negative: 0,
        }
    }
}
impl ExiSigned {
    pub fn new() -> Self {
        ExiSigned::default()
    }
}

pub fn exi_basetypes_convert_to_unsigned(
    exi_unsigned: &mut ExiUnsigned,
    mut value: u32,
    max_octets: usize,
) -> Result<u8, i16> {
    exi_unsigned.octets_count = 0;
    let mut n = 0;
    while n < 5 {
        if exi_unsigned.octets_count >= exi_unsigned.octets.len() {
            // Prevent overflow
            return Err(OCTET_COUNT_LARGER_THAN_TYPE_SUPPORTS);
        }
        let mut octet = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            octet |= 0x80;
        }
        exi_unsigned.octets[exi_unsigned.octets_count] = octet;
        exi_unsigned.octets_count += 1;
        if value == 0 {
            break;
        }
        n += 1;
    }
    if exi_unsigned.octets_count <= max_octets {
        Ok(NO_ERROR)
    } else {
        Err(OCTET_COUNT_LARGER_THAN_TYPE_SUPPORTS)
    }
}

pub fn exi_basetypes_convert_64_to_unsigned(
    exi_unsigned: &mut ExiUnsigned,
    mut value: u64,
) -> Result<u8, i16> {
    exi_unsigned.octets_count = 0;
    let mut n = 0;
    while n < 10 {
        if exi_unsigned.octets_count >= exi_unsigned.octets.len() {
            // Prevent overflow
            return Err(OCTET_COUNT_LARGER_THAN_TYPE_SUPPORTS);
        }
        let mut octet = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            octet |= 0x80;
        }
        exi_unsigned.octets[exi_unsigned.octets_count] = octet;
        exi_unsigned.octets_count += 1;
        if value == 0 {
            break;
        }
        n += 1;
    }
    if exi_unsigned.octets_count <= 10 {
        Ok(NO_ERROR)
    } else {
        Err(OCTET_COUNT_LARGER_THAN_TYPE_SUPPORTS)
    }
}

pub fn exi_basetypes_convert_to_signed(
    exi_signed: &mut ExiSigned,
    value: i32,
    max_octets: usize,
) -> Result<u8, i16> {
    if value < 0 {
        exi_signed.is_negative = 1;
        exi_basetypes_convert_to_unsigned(&mut exi_signed.data, (-value) as u32, max_octets)
    } else {
        exi_signed.is_negative = 0;
        exi_basetypes_convert_to_unsigned(&mut exi_signed.data, value as u32, max_octets)
    }
}

pub fn exi_basetypes_convert_64_to_signed(
    exi_signed: &mut ExiSigned,
    value: i64,
) -> Result<u8, i16> {
    if value < 0 {
        exi_signed.is_negative = 1;
        exi_basetypes_convert_64_to_unsigned(&mut exi_signed.data, (-value) as u64)
    } else {
        exi_signed.is_negative = 0;
        exi_basetypes_convert_64_to_unsigned(&mut exi_signed.data, value as u64)
    }
}

pub fn exi_basetypes_convert_from_unsigned(
    exi_unsigned: &ExiUnsigned,
    value: &mut u32,
    max_octets: usize,
) -> Result<u8, i16> {
    if exi_unsigned.octets_count > max_octets {
        return Err(OCTET_COUNT_LARGER_THAN_TYPE_SUPPORTS);
    }
    *value = 0;
    for (n, &octet) in exi_unsigned
        .octets
        .iter()
        .take(exi_unsigned.octets_count)
        .enumerate()
    {
        *value = value.wrapping_add(((octet & 0x7f) as u32) << (n * 7));
    }
    Ok(NO_ERROR)
}

pub fn exi_basetypes_convert_64_from_unsigned(
    exi_unsigned: &ExiUnsigned,
    value: &mut u64,
) -> Result<u8, i16> {
    if exi_unsigned.octets_count > 10 {
        return Err(OCTET_COUNT_LARGER_THAN_TYPE_SUPPORTS);
    }
    *value = 0;
    for (n, &octet) in exi_unsigned
        .octets
        .iter()
        .take(exi_unsigned.octets_count)
        .enumerate()
    {
        *value = value.wrapping_add(((octet & 0x7f) as u64) << (n * 7));
    }
    Ok(NO_ERROR)
}

pub fn exi_basetypes_convert_from_signed(
    exi_signed: &ExiSigned,
    value: &mut i32,
    max_octets: usize,
) -> Result<u8, i16> {
    let mut u_value: u32 = 0;
    let res = exi_basetypes_convert_from_unsigned(&exi_signed.data, &mut u_value, max_octets);
    *value = if exi_signed.is_negative == 0 {
        u_value as i32
    } else {
        (u_value as i32).wrapping_neg()
    };
    res
}

pub fn exi_basetypes_convert_64_from_signed(
    exi_signed: &ExiSigned,
    value: &mut i64,
) -> Result<u8, i16> {
    let mut u_value: u64 = 0;
    let res = exi_basetypes_convert_64_from_unsigned(&exi_signed.data, &mut u_value);
    *value = if exi_signed.is_negative == 0 {
        u_value as i64
    } else {
        (u_value as i64).wrapping_neg()
    };
    res
}

fn reverse_array(data: &mut [u8]) {
    data.reverse();
}

pub fn exi_basetypes_convert_bytes_from_unsigned(
    exi_unsigned: &ExiUnsigned,
    data: &mut [u8],
    data_len: &mut usize,
) -> Result<u8, i16> {
    let mut temp: u16 = 0;
    *data_len = 0;
    let mut total_offset: usize = 0;
    let mut n: usize = 0;
    while n < exi_unsigned.octets_count {
        temp = (temp as u32
            + (((exi_unsigned.octets[n] & 0x7f) as u16 as u32) << total_offset) as u32)
            as u16;
        total_offset += 7;
        if total_offset >= 8 {
            if *data_len >= data.len() {
                return Err(ENCODED_INTEGER_SIZE_LARGER_THAN_DESTINATION);
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
            return Err(ENCODED_INTEGER_SIZE_LARGER_THAN_DESTINATION);
        }
        data[*data_len] = (temp & 0xff) as u8;
        *data_len += 1;
    }
    reverse_array(&mut data[..*data_len]);
    Ok(NO_ERROR)
}

pub fn exi_basetypes_convert_bytes_to_unsigned(
    exi_unsigned: &mut ExiUnsigned,
    data: &[u8],
) -> Result<u8, i16> {
    exi_unsigned.octets_count = 0;
    let mut bytenum = 0;
    while bytenum < data.len() {
        if data[bytenum] != 0 {
            break;
        }
        bytenum += 1;
    }
    if bytenum == data.len() {
        exi_unsigned.octets[0] = 0;
        exi_unsigned.octets_count = 1;
        return Ok(NO_ERROR);
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
        if dummy_count < 7 {
            if incount < data.len() - bytenum {
                dummy |= (data[data.len() - incount - 1] as u16) << dummy_count;
                dummy_count += 8;
                incount += 1;
            }
        }
        exi_unsigned.octets[outcount] = (dummy & 0x7f) as u8;
        exi_unsigned.octets_count += 1;
        if exi_unsigned.octets_count < exi_expected_octets_count {
            exi_unsigned.octets[outcount] |= 0x80;
        }
        dummy >>= 7;
        dummy_count -= 7;
        outcount += 1;
    }
    Ok(NO_ERROR)
}
