#![no_std]
// #![warn(clippy::pedantic)]
#![deny(clippy::cargo)]
// #![warn(clippy::nursery)]
// #![allow(clippy::missing_errors_doc)] // TODO: Document errors and remove this allowance

pub mod exi_v2gtp;

pub mod app_handshake {
    pub mod app_handshake_datatypes;
    pub mod app_handshake_decoder;
    pub mod app_handshake_encoder;
}

pub mod common {
    pub mod exi_basetypes;
    pub mod exi_basetypes_decoder;
    pub mod exi_basetypes_encoder;
    pub mod exi_bitstream;
    pub mod exi_error_codes;
    pub mod exi_types_decoder;
}

pub mod iso_2 {
    pub mod iso2_datatypes;
    pub mod iso2_decoder;
}
