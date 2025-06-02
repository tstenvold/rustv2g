// #![allow(dead_code)]
// #![allow(mutable_transmutes)]
// #![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// #![allow(non_upper_case_globals)]
// #![allow(unused_assignments)]
#![allow(unused_mut)]
#![no_main]
#![no_std]
use panic_probe as _;

pub mod lib {
    pub mod cbv2g {
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
          //     pub mod din {
          //         pub mod din_msgDefDatatypes;
          //         pub mod din_msgDefDecoder;
          //         pub mod din_msgDefEncoder;
          //     } // mod din
        pub mod iso_2 {
            pub mod iso2_msgDefDatatypes;
            pub mod iso2_msgDefDecoder;
            //         pub mod iso2_msgDefEncoder;
        } // mod iso_2
          //     pub mod iso_20 {
          //         pub mod iso20_ACDP_Datatypes;
          //         pub mod iso20_ACDP_Decoder;
          //         pub mod iso20_ACDP_Encoder;
          //         pub mod iso20_AC_Datatypes;
          //         pub mod iso20_AC_Decoder;
          //         pub mod iso20_AC_Encoder;
          //         pub mod iso20_CommonMessages_Datatypes;
          //         pub mod iso20_CommonMessages_Decoder;
          //         pub mod iso20_CommonMessages_Encoder;
          //         pub mod iso20_DC_Datatypes;
          //         pub mod iso20_DC_Decoder;
          //         pub mod iso20_DC_Encoder;
          //         pub mod iso20_WPT_Datatypes;
          //         pub mod iso20_WPT_Decoder;
          //         pub mod iso20_WPT_Encoder;
          //     } // mod iso_20
    } // mod cbv2g
} // mod lib
