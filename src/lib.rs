// #![allow(dead_code)]
// #![allow(mutable_transmutes)]
// #![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// #![allow(non_upper_case_globals)]
// #![allow(unused_assignments)]
#![allow(unused_mut)]
#![no_std]

pub mod exi_v2gtp;
pub mod app_handshake {
    pub mod appHand_Datatypes;
    pub mod appHand_Decoder;
    pub mod appHand_Encoder;
} // mod app_handshake
pub mod common {
    pub mod exi_basetypes;
    pub mod exi_basetypes_decoder;
    pub mod exi_basetypes_encoder;
    pub mod exi_bitstream;
    pub mod exi_error_codes;
    pub mod exi_header;
    pub mod exi_types_decoder;
} // mod common
pub mod iso_2 {
    pub mod iso2_msgDefDatatypes;
    pub mod iso2_msgDefDecoder;
}
