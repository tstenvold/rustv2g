pub const NO_ERROR: u8 = 0;

// stream processing -1 to -19
pub const BITSTREAM_OVERFLOW: i16 = -1;

// stream header -20 to -29
pub const HEADER_COOKIE_NOT_SUPPORTED: i16 = -20;
pub const HEADER_OPTIONS_NOT_SUPPORTED: i16 = -21;
pub const HEADER_INCORRECT: i16 = -22;

// stream read -30 to -39
pub const SUPPORTED_MAX_OCTETS_OVERRUN: i16 = -30;
pub const OCTET_COUNT_LARGER_THAN_TYPE_SUPPORTS: i16 = -31;

// decoder -50 to -69
pub const UNKNOWN_EVENT_FOR_DECODING: i16 = -50;
pub const DECODER_NOT_IMPLEMENTED: i16 = -69;

// encoder -70 to -89
pub const UNKNOWN_EVENT_FOR_ENCODING: i16 = -70;
pub const ENCODER_NOT_IMPLEMENTED: i16 = -89;

// common errors -100 to -129
pub const BIT_COUNT_LARGER_THAN_TYPE_SIZE: i16 = -100;
pub const BYTE_COUNT_LARGER_THAN_TYPE_SIZE: i16 = -101;
pub const ARRAY_OUT_OF_BOUNDS: i16 = -110;
pub const CHARACTER_BUFFER_TOO_SMALL: i16 = -111;
pub const BYTE_BUFFER_TOO_SMALL: i16 = -112;
pub const ENCODED_INTEGER_SIZE_LARGER_THAN_DESTINATION: i16 = -113;

// grammar errors -130 to -149
pub const UNKNOWN_GRAMMAR_ID: i16 = -130;

// event errors -150 to -169
pub const UNKNOWN_EVENT_CODE: i16 = -150;
pub const UNSUPPORTED_SUB_EVENT: i16 = -151;

// document errors -170 to -199
pub const DEVIANTS_NOT_SUPPORTED: i16 = -170;

// datatype errors -200 to -229
pub const STRINGVALUES_NOT_SUPPORTED: i16 = -200;
pub const UNSUPPORTED_INTEGER_VALUE_TYPE: i16 = -210;
pub const UNSUPPORTED_DATETIME_TYPE: i16 = -211;
pub const UNSUPPORTED_CHARACTER_VALUE: i16 = -212;

// fragment errors -230 to -259
pub const INCORRECT_END_FRAGMENT_VALUE: i16 = -230;

// internal errors
pub const NOT_IMPLEMENTED_YET: i16 = -299;
