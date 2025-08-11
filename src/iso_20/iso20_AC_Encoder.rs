use ::c2rust_bitfields;
use c2rust_bitfields::*;
extern "C" {
    fn exi_basetypes_encoder_bool(stream: *mut exi_bitstream_t, value: i32) -> i32;
    fn exi_basetypes_encoder_bytes(
        stream: *mut exi_bitstream_t,
        bytes_len: usize,
        bytes: *const u8,
        bytes_size: usize,
    ) -> i32;
    fn exi_basetypes_encoder_nbit_uint(
        stream: *mut exi_bitstream_t,
        bit_count: usize,
        value: u32,
    ) -> i32;
    fn exi_basetypes_encoder_uint_16(stream: *mut exi_bitstream_t, value: u16) -> i32;
    fn exi_basetypes_encoder_uint_32(stream: *mut exi_bitstream_t, value: u32) -> i32;
    fn exi_basetypes_encoder_uint_64(stream: *mut exi_bitstream_t, value: u64) -> i32;
    fn exi_basetypes_encoder_integer_16(stream: *mut exi_bitstream_t, value: i16) -> i32;
    fn exi_basetypes_encoder_signed(
        stream: *mut exi_bitstream_t,
        value: *const ExiSigned,
    ) -> i32;
    fn exi_basetypes_encoder_characters(
        stream: *mut exi_bitstream_t,
        characters_len: usize,
        characters: *const ExiChar,
        characters_size: usize,
    ) -> i32;
    fn exi_header_write(stream: *mut exi_bitstream_t) -> i32;
}
pub type i8 = i8;
pub type __u8 = u8;
pub type __int16_t = i16;
pub type __u16 = u16;
pub type __ui32 = u32;
pub type __u64 = u64;
pub type int8_t = i8;
pub type i16 = __int16_t;
pub type u8 = __u8;
pub type u16 = __u16;
pub type u32 = __ui32;
pub type u64 = __u64;
pub type usize = u64;
#[derive(Copy, Clone)]

pub struct ExiUnsigned {
    pub octets: [u8; 29],
    pub octets.len(): usize,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct ExiSigned {
    pub data: ExiUnsigned,
    #[bitfield(name = "is_negative", ty = "u8", bits = "0..=0")]
    pub is_negative: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type ExiChar = i8;
pub type exi_status_callback = Option<unsafe extern "C" fn(i32, i32, i32, i32) -> ()>;
#[derive(Copy, Clone)]

pub struct exi_bitstream {
    pub data: *mut u8,
    pub data_size: usize,
    pub bit_count: u8,
    pub byte_pos: usize,
    pub _init_called: u8,
    pub _flag_byte_pos: usize,
    pub status_callback: exi_status_callback,
}
pub type exi_bitstream_t = exi_bitstream;
pub type iso20_ac_evseNotificationType = u32;
pub const iso20_ac_evseNotificationType_MeteringConfirmation: iso20_ac_evseNotificationType = 5;
pub const iso20_ac_evseNotificationType_ServiceRenegotiation: iso20_ac_evseNotificationType = 4;
pub const iso20_ac_evseNotificationType_ScheduleRenegotiation: iso20_ac_evseNotificationType = 3;
pub const iso20_ac_evseNotificationType_Terminate: iso20_ac_evseNotificationType = 2;
pub const iso20_ac_evseNotificationType_ExitStandby: iso20_ac_evseNotificationType = 1;
pub const iso20_ac_evseNotificationType_Pause: iso20_ac_evseNotificationType = 0;
pub type iso20_ac_responseCodeType = u32;
pub const iso20_ac_responseCodeType_FAILED_WrongChargeParameter: iso20_ac_responseCodeType = 39;
pub const iso20_ac_responseCodeType_FAILED_UnknownSession: iso20_ac_responseCodeType = 38;
pub const iso20_ac_responseCodeType_FAILED_SignatureError: iso20_ac_responseCodeType = 37;
pub const iso20_ac_responseCodeType_FAILED_ServiceSelectionInvalid: iso20_ac_responseCodeType = 36;
pub const iso20_ac_responseCodeType_FAILED_ServiceIDInvalid: iso20_ac_responseCodeType = 35;
pub const iso20_ac_responseCodeType_FAILED_SequenceError: iso20_ac_responseCodeType = 34;
pub const iso20_ac_responseCodeType_FAILED_ScheduleSelectionInvalid: iso20_ac_responseCodeType = 33;
pub const iso20_ac_responseCodeType_FAILED_ScheduleRenegotiation: iso20_ac_responseCodeType = 32;
pub const iso20_ac_responseCodeType_FAILED_PowerToleranceNotConfirmed: iso20_ac_responseCodeType =
    31;
pub const iso20_ac_responseCodeType_FAILED_PowerDeliveryNotApplied: iso20_ac_responseCodeType = 30;
pub const iso20_ac_responseCodeType_FAILED_PauseNotAllowed: iso20_ac_responseCodeType = 29;
pub const iso20_ac_responseCodeType_FAILED_NoServiceRenegotiationSupported:
    iso20_ac_responseCodeType = 28;
pub const iso20_ac_responseCodeType_FAILED_NoEnergyTransferServiceSelected:
    iso20_ac_responseCodeType = 27;
pub const iso20_ac_responseCodeType_FAILED_MeteringSignatureNotValid: iso20_ac_responseCodeType =
    26;
pub const iso20_ac_responseCodeType_FAILED_EVPowerProfileViolation: iso20_ac_responseCodeType = 25;
pub const iso20_ac_responseCodeType_FAILED_EVPowerProfileInvalid: iso20_ac_responseCodeType = 24;
pub const iso20_ac_responseCodeType_FAILED_ContactorError: iso20_ac_responseCodeType = 23;
pub const iso20_ac_responseCodeType_FAILED_AssociationError: iso20_ac_responseCodeType = 22;
pub const iso20_ac_responseCodeType_FAILED: iso20_ac_responseCodeType = 21;
pub const iso20_ac_responseCodeType_WARNING_WPT: iso20_ac_responseCodeType = 20;
pub const iso20_ac_responseCodeType_WARNING_StandbyNotAllowed: iso20_ac_responseCodeType = 19;
pub const iso20_ac_responseCodeType_WARNING_ScheduleRenegotiationFailed: iso20_ac_responseCodeType =
    18;
pub const iso20_ac_responseCodeType_WARNING_PowerToleranceNotConfirmed: iso20_ac_responseCodeType =
    17;
pub const iso20_ac_responseCodeType_WARNING_NoContractMatchingPCIDFound: iso20_ac_responseCodeType =
    16;
pub const iso20_ac_responseCodeType_WARNING_NoCertificateAvailable: iso20_ac_responseCodeType = 15;
pub const iso20_ac_responseCodeType_WARNING_GeneralPnCAuthorizationError:
    iso20_ac_responseCodeType = 14;
pub const iso20_ac_responseCodeType_WARNING_EVPowerProfileViolation: iso20_ac_responseCodeType = 13;
pub const iso20_ac_responseCodeType_WARNING_eMSPUnknown: iso20_ac_responseCodeType = 12;
pub const iso20_ac_responseCodeType_WARNING_EIMAuthorizationFailure: iso20_ac_responseCodeType = 11;
pub const iso20_ac_responseCodeType_WARNING_ChallengeInvalid: iso20_ac_responseCodeType = 10;
pub const iso20_ac_responseCodeType_WARNING_CertificateValidationError: iso20_ac_responseCodeType =
    9;
pub const iso20_ac_responseCodeType_WARNING_CertificateRevoked: iso20_ac_responseCodeType = 8;
pub const iso20_ac_responseCodeType_WARNING_CertificateNotYetValid: iso20_ac_responseCodeType = 7;
pub const iso20_ac_responseCodeType_WARNING_CertificateExpired: iso20_ac_responseCodeType = 6;
pub const iso20_ac_responseCodeType_WARNING_AuthorizationSelectionInvalid:
    iso20_ac_responseCodeType = 5;
pub const iso20_ac_responseCodeType_OK_PowerToleranceConfirmed: iso20_ac_responseCodeType = 4;
pub const iso20_ac_responseCodeType_OK_OldSessionJoined: iso20_ac_responseCodeType = 3;
pub const iso20_ac_responseCodeType_OK_NewSessionEstablished: iso20_ac_responseCodeType = 2;
pub const iso20_ac_responseCodeType_OK_CertificateExpiresSoon: iso20_ac_responseCodeType = 1;
pub const iso20_ac_responseCodeType_OK: iso20_ac_responseCodeType = 0;
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_TransformType {
    pub Algorithm: C2RustUnnamed_1,
    pub ANY: C2RustUnnamed_0,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub XPath: C2RustUnnamed,
    #[bitfield(name = "XPath_isUsed", ty = "u32", bits = "0..=0")]
    pub XPath_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_0 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_1 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_ac_TransformsType {
    pub Transform: iso20_ac_TransformType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_DSAKeyValueType {
    pub P: C2RustUnnamed_8,
    #[bitfield(name = "P_isUsed", ty = "u32", bits = "0..=0")]
    pub P_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub Q: C2RustUnnamed_7,
    #[bitfield(name = "Q_isUsed", ty = "u32", bits = "0..=0")]
    pub Q_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub G: C2RustUnnamed_6,
    #[bitfield(name = "G_isUsed", ty = "u32", bits = "0..=0")]
    pub G_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub Y: C2RustUnnamed_5,
    pub J: C2RustUnnamed_4,
    #[bitfield(name = "J_isUsed", ty = "u32", bits = "0..=0")]
    pub J_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub Seed: C2RustUnnamed_3,
    #[bitfield(name = "Seed_isUsed", ty = "u32", bits = "0..=0")]
    pub Seed_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 1],
    pub PgenCounter: C2RustUnnamed_2,
    #[bitfield(name = "PgenCounter_isUsed", ty = "u32", bits = "0..=0")]
    pub PgenCounter_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 1],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_2 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_3 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_4 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_5 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_6 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_7 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_8 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_ac_X509IssuerSerialType {
    pub X509IssuerName: C2RustUnnamed_9,
    pub X509SerialNumber: ExiSigned,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_9 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_DigestMethodType {
    pub Algorithm: C2RustUnnamed_11,
    pub ANY: C2RustUnnamed_10,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_10 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_11 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_ac_RSAKeyValueType {
    pub Modulus: C2RustUnnamed_13,
    pub Exponent: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_12 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_13 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_CanonicalizationMethodType {
    pub Algorithm: C2RustUnnamed_15,
    pub ANY: C2RustUnnamed_14,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_14 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_15 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_SignatureMethodType {
    pub Algorithm: C2RustUnnamed_17,
    pub HMACOutputLength: ExiSigned,
    #[bitfield(name = "HMACOutputLength_isUsed", ty = "u32", bits = "0..=0")]
    pub HMACOutputLength_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub ANY: C2RustUnnamed_16,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 7],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_16 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_17 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_KeyValueType {
    pub DSAKeyValue: iso20_ac_DSAKeyValueType,
    #[bitfield(name = "DSAKeyValue_isUsed", ty = "u32", bits = "0..=0")]
    pub DSAKeyValue_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub RSAKeyValue: iso20_ac_RSAKeyValueType,
    #[bitfield(name = "RSAKeyValue_isUsed", ty = "u32", bits = "0..=0")]
    pub RSAKeyValue_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub ANY: C2RustUnnamed_18,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_18 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_ReferenceType {
    pub Id: C2RustUnnamed_22,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub Type: C2RustUnnamed_21,
    #[bitfield(name = "Type_isUsed", ty = "u32", bits = "0..=0")]
    pub Type_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub URI: C2RustUnnamed_20,
    #[bitfield(name = "URI_isUsed", ty = "u32", bits = "0..=0")]
    pub URI_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 3],
    pub Transforms: iso20_ac_TransformsType,
    #[bitfield(name = "Transforms_isUsed", ty = "u32", bits = "0..=0")]
    pub Transforms_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 3],
    pub DigestMethod: iso20_ac_DigestMethodType,
    pub DigestValue: C2RustUnnamed_19,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_19 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_20 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_21 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_22 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_RetrievalMethodType {
    pub Type: C2RustUnnamed_24,
    #[bitfield(name = "Type_isUsed", ty = "u32", bits = "0..=0")]
    pub Type_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub URI: C2RustUnnamed_23,
    #[bitfield(name = "URI_isUsed", ty = "u32", bits = "0..=0")]
    pub URI_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub Transforms: iso20_ac_TransformsType,
    #[bitfield(name = "Transforms_isUsed", ty = "u32", bits = "0..=0")]
    pub Transforms_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 3],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_23 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_24 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_X509DataType {
    pub X509IssuerSerial: iso20_ac_X509IssuerSerialType,
    #[bitfield(name = "X509IssuerSerial_isUsed", ty = "u32", bits = "0..=0")]
    pub X509IssuerSerial_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub X509SKI: C2RustUnnamed_29,
    #[bitfield(name = "X509SKI_isUsed", ty = "u32", bits = "0..=0")]
    pub X509SKI_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub X509SubjectName: C2RustUnnamed_28,
    #[bitfield(name = "X509SubjectName_isUsed", ty = "u32", bits = "0..=0")]
    pub X509SubjectName_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub X509Certificate: C2RustUnnamed_27,
    #[bitfield(name = "X509Certificate_isUsed", ty = "u32", bits = "0..=0")]
    pub X509Certificate_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub X509CRL: C2RustUnnamed_26,
    #[bitfield(name = "X509CRL_isUsed", ty = "u32", bits = "0..=0")]
    pub X509CRL_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 1],
    pub ANY: C2RustUnnamed_25,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 3],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_25 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_26 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_27 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_28 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_29 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_ac_PGPDataType {
    pub c2rust_unnamed: C2RustUnnamed_30,
}
#[derive(Copy, Clone)]

pub union C2RustUnnamed_30 {
    pub choice_1: C2RustUnnamed_34,
    pub choice_1_isUsed: u32,
    pub choice_2: C2RustUnnamed_31,
    pub choice_2_isUsed: u32,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct C2RustUnnamed_31 {
    pub PGPKeyPacket: C2RustUnnamed_33,
    pub ANY: C2RustUnnamed_32,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_32 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_33 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct C2RustUnnamed_34 {
    pub PGPKeyID: C2RustUnnamed_37,
    pub PGPKeyPacket: C2RustUnnamed_36,
    #[bitfield(name = "PGPKeyPacket_isUsed", ty = "u32", bits = "0..=0")]
    pub PGPKeyPacket_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub ANY: C2RustUnnamed_35,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_35 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_36 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_37 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_SPKIDataType {
    pub SPKISexp: C2RustUnnamed_39,
    pub ANY: C2RustUnnamed_38,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_38 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_39 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_SignedInfoType {
    pub Id: C2RustUnnamed_41,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub CanonicalizationMethod: iso20_ac_CanonicalizationMethodType,
    pub SignatureMethod: iso20_ac_SignatureMethodType,
    pub Reference: C2RustUnnamed_40,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_40 {
    pub array: [iso20_ac_ReferenceType; 4],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_41 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_SignatureValueType {
    pub Id: C2RustUnnamed_43,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub CONTENT: C2RustUnnamed_42,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_42 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_43 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_KeyInfoType {
    pub Id: C2RustUnnamed_47,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub KeyName: C2RustUnnamed_46,
    #[bitfield(name = "KeyName_isUsed", ty = "u32", bits = "0..=0")]
    pub KeyName_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub KeyValue: iso20_ac_KeyValueType,
    #[bitfield(name = "KeyValue_isUsed", ty = "u32", bits = "0..=0")]
    pub KeyValue_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 3],
    pub RetrievalMethod: iso20_ac_RetrievalMethodType,
    #[bitfield(name = "RetrievalMethod_isUsed", ty = "u32", bits = "0..=0")]
    pub RetrievalMethod_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 3],
    pub X509Data: iso20_ac_X509DataType,
    #[bitfield(name = "X509Data_isUsed", ty = "u32", bits = "0..=0")]
    pub X509Data_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 3],
    pub PGPData: iso20_ac_PGPDataType,
    #[bitfield(name = "PGPData_isUsed", ty = "u32", bits = "0..=0")]
    pub PGPData_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 3],
    pub SPKIData: iso20_ac_SPKIDataType,
    #[bitfield(name = "SPKIData_isUsed", ty = "u32", bits = "0..=0")]
    pub SPKIData_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_5: [u8; 1],
    pub MgmtData: C2RustUnnamed_45,
    #[bitfield(name = "MgmtData_isUsed", ty = "u32", bits = "0..=0")]
    pub MgmtData_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_6: [u8; 1],
    pub ANY: C2RustUnnamed_44,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_7: [u8; 5],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_44 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_45 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_46 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_47 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_ObjectType {
    pub Encoding: C2RustUnnamed_51,
    #[bitfield(name = "Encoding_isUsed", ty = "u32", bits = "0..=0")]
    pub Encoding_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub Id: C2RustUnnamed_50,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub MimeType: C2RustUnnamed_49,
    #[bitfield(name = "MimeType_isUsed", ty = "u32", bits = "0..=0")]
    pub MimeType_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub ANY: C2RustUnnamed_48,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 3],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_48 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_49 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_50 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_51 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_ac_RationalNumberType {
    pub Exponent: int8_t,
    pub Value: i16,
}
#[derive(Copy, Clone)]

pub struct iso20_ac_DetailedCostType {
    pub Amount: iso20_ac_RationalNumberType,
    pub CostPerUnit: iso20_ac_RationalNumberType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_SignatureType {
    pub Id: C2RustUnnamed_52,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub SignedInfo: iso20_ac_SignedInfoType,
    pub SignatureValue: iso20_ac_SignatureValueType,
    pub KeyInfo: iso20_ac_KeyInfoType,
    #[bitfield(name = "KeyInfo_isUsed", ty = "u32", bits = "0..=0")]
    pub KeyInfo_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
    pub Object: iso20_ac_ObjectType,
    #[bitfield(name = "Object_isUsed", ty = "u32", bits = "0..=0")]
    pub Object_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 7],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_52 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_ac_DetailedTaxType {
    pub TaxRuleID: u32,
    pub Amount: iso20_ac_RationalNumberType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_MessageHeaderType {
    pub SessionID: C2RustUnnamed_53,
    pub TimeStamp: u64,
    pub Signature: iso20_ac_SignatureType,
    #[bitfield(name = "Signature_isUsed", ty = "u32", bits = "0..=0")]
    pub Signature_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_53 {
    pub bytes: [u8; 8],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_SignaturePropertyType {
    pub Id: C2RustUnnamed_56,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub Target: C2RustUnnamed_55,
    pub ANY: C2RustUnnamed_54,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_54 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_55 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_56 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_AC_CPDReqEnergyTransferModeType {
    pub EVMaximumChargePower: iso20_ac_RationalNumberType,
    pub EVMaximumChargePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMaximumChargePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumChargePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub EVMaximumChargePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMaximumChargePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumChargePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub EVMinimumChargePower: iso20_ac_RationalNumberType,
    pub EVMinimumChargePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMinimumChargePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumChargePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub EVMinimumChargePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMinimumChargePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumChargePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_DisplayParametersType {
    pub PresentSOC: int8_t,
    #[bitfield(name = "PresentSOC_isUsed", ty = "u32", bits = "0..=0")]
    pub PresentSOC_isUsed: [u8; 1],
    pub MinimumSOC: int8_t,
    #[bitfield(name = "MinimumSOC_isUsed", ty = "u32", bits = "0..=0")]
    pub MinimumSOC_isUsed: [u8; 1],
    pub TargetSOC: int8_t,
    #[bitfield(name = "TargetSOC_isUsed", ty = "u32", bits = "0..=0")]
    pub TargetSOC_isUsed: [u8; 1],
    pub MaximumSOC: int8_t,
    #[bitfield(name = "MaximumSOC_isUsed", ty = "u32", bits = "0..=0")]
    pub MaximumSOC_isUsed: [u8; 1],
    pub RemainingTimeToMinimumSOC: u32,
    #[bitfield(name = "RemainingTimeToMinimumSOC_isUsed", ty = "u32", bits = "0..=0")]
    pub RemainingTimeToMinimumSOC_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub RemainingTimeToTargetSOC: u32,
    #[bitfield(name = "RemainingTimeToTargetSOC_isUsed", ty = "u32", bits = "0..=0")]
    pub RemainingTimeToTargetSOC_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
    pub RemainingTimeToMaximumSOC: u32,
    #[bitfield(name = "RemainingTimeToMaximumSOC_isUsed", ty = "u32", bits = "0..=0")]
    pub RemainingTimeToMaximumSOC_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 3],
    pub ChargingComplete: i32,
    #[bitfield(name = "ChargingComplete_isUsed", ty = "u32", bits = "0..=0")]
    pub ChargingComplete_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub BatteryEnergyCapacity: iso20_ac_RationalNumberType,
    #[bitfield(name = "BatteryEnergyCapacity_isUsed", ty = "u32", bits = "0..=0")]
    pub BatteryEnergyCapacity_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 1],
    pub InletHot: i32,
    #[bitfield(name = "InletHot_isUsed", ty = "u32", bits = "0..=0")]
    pub InletHot_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 3],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_AC_CPDResEnergyTransferModeType {
    pub EVSEMaximumChargePower: iso20_ac_RationalNumberType,
    pub EVSEMaximumChargePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEMaximumChargePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEMaximumChargePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub EVSEMaximumChargePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEMaximumChargePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEMaximumChargePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub EVSEMinimumChargePower: iso20_ac_RationalNumberType,
    pub EVSEMinimumChargePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEMinimumChargePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEMinimumChargePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub EVSEMinimumChargePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEMinimumChargePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEMinimumChargePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub EVSENominalFrequency: iso20_ac_RationalNumberType,
    pub MaximumPowerAsymmetry: iso20_ac_RationalNumberType,
    #[bitfield(name = "MaximumPowerAsymmetry_isUsed", ty = "u32", bits = "0..=0")]
    pub MaximumPowerAsymmetry_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 1],
    pub EVSEPowerRampLimitation: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEPowerRampLimitation_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEPowerRampLimitation_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 1],
    pub EVSEPresentActivePower: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEPresentActivePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEPresentActivePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_5: [u8; 1],
    pub EVSEPresentActivePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEPresentActivePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEPresentActivePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_6: [u8; 1],
    pub EVSEPresentActivePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEPresentActivePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEPresentActivePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_7: [u8; 3],
}
#[derive(Copy, Clone)]

pub struct iso20_ac_EVSEStatusType {
    pub NotificationMaxDelay: u16,
    pub EVSENotification: iso20_ac_evseNotificationType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_Dynamic_AC_CLReqControlModeType {
    pub DepartureTime: u32,
    #[bitfield(name = "DepartureTime_isUsed", ty = "u32", bits = "0..=0")]
    pub DepartureTime_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub EVTargetEnergyRequest: iso20_ac_RationalNumberType,
    pub EVMaximumEnergyRequest: iso20_ac_RationalNumberType,
    pub EVMinimumEnergyRequest: iso20_ac_RationalNumberType,
    pub EVMaximumChargePower: iso20_ac_RationalNumberType,
    pub EVMaximumChargePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMaximumChargePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumChargePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub EVMaximumChargePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMaximumChargePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumChargePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub EVMinimumChargePower: iso20_ac_RationalNumberType,
    pub EVMinimumChargePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMinimumChargePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumChargePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub EVMinimumChargePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMinimumChargePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumChargePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 1],
    pub EVPresentActivePower: iso20_ac_RationalNumberType,
    pub EVPresentActivePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVPresentActivePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVPresentActivePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 1],
    pub EVPresentActivePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVPresentActivePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVPresentActivePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_5: [u8; 1],
    pub EVPresentReactivePower: iso20_ac_RationalNumberType,
    pub EVPresentReactivePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVPresentReactivePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVPresentReactivePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_6: [u8; 1],
    pub EVPresentReactivePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVPresentReactivePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVPresentReactivePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_7: [u8; 3],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_Scheduled_AC_CLReqControlModeType {
    pub EVTargetEnergyRequest: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVTargetEnergyRequest_isUsed", ty = "u32", bits = "0..=0")]
    pub EVTargetEnergyRequest_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub EVMaximumEnergyRequest: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMaximumEnergyRequest_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumEnergyRequest_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub EVMinimumEnergyRequest: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMinimumEnergyRequest_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumEnergyRequest_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub EVMaximumChargePower: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMaximumChargePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumChargePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub EVMaximumChargePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMaximumChargePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumChargePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 1],
    pub EVMaximumChargePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMaximumChargePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumChargePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 1],
    pub EVMinimumChargePower: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMinimumChargePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumChargePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_5: [u8; 1],
    pub EVMinimumChargePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMinimumChargePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumChargePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_6: [u8; 1],
    pub EVMinimumChargePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMinimumChargePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumChargePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_7: [u8; 1],
    pub EVPresentActivePower: iso20_ac_RationalNumberType,
    pub EVPresentActivePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVPresentActivePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVPresentActivePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_8: [u8; 1],
    pub EVPresentActivePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVPresentActivePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVPresentActivePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_9: [u8; 1],
    pub EVPresentReactivePower: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVPresentReactivePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVPresentReactivePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_10: [u8; 1],
    pub EVPresentReactivePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVPresentReactivePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVPresentReactivePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_11: [u8; 1],
    pub EVPresentReactivePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVPresentReactivePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVPresentReactivePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_12: [u8; 1],
}
#[derive(Copy, Clone)]

pub struct iso20_ac_CLReqControlModeType {
    _unused: i32,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_MeterInfoType {
    pub MeterID: C2RustUnnamed_58,
    pub ChargedEnergyReadingWh: u64,
    pub BPT_DischargedEnergyReadingWh: u64,
    #[bitfield(
        name = "BPT_DischargedEnergyReadingWh_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub BPT_DischargedEnergyReadingWh_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub CapacitiveEnergyReadingVARh: u64,
    #[bitfield(
        name = "CapacitiveEnergyReadingVARh_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub CapacitiveEnergyReadingVARh_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 7],
    pub BPT_InductiveEnergyReadingVARh: u64,
    #[bitfield(
        name = "BPT_InductiveEnergyReadingVARh_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub BPT_InductiveEnergyReadingVARh_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub MeterSignature: C2RustUnnamed_57,
    #[bitfield(name = "MeterSignature_isUsed", ty = "u32", bits = "0..=0")]
    pub MeterSignature_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub MeterStatus: i16,
    #[bitfield(name = "MeterStatus_isUsed", ty = "u32", bits = "0..=0")]
    pub MeterStatus_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 7],
    pub MeterTimestamp: u64,
    #[bitfield(name = "MeterTimestamp_isUsed", ty = "u32", bits = "0..=0")]
    pub MeterTimestamp_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 7],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_57 {
    pub bytes: [u8; 64],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_58 {
    pub characters: [i8; 33],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_ReceiptType {
    pub TimeAnchor: u64,
    pub EnergyCosts: iso20_ac_DetailedCostType,
    #[bitfield(name = "EnergyCosts_isUsed", ty = "u32", bits = "0..=0")]
    pub EnergyCosts_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub OccupancyCosts: iso20_ac_DetailedCostType,
    #[bitfield(name = "OccupancyCosts_isUsed", ty = "u32", bits = "0..=0")]
    pub OccupancyCosts_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub AdditionalServicesCosts: iso20_ac_DetailedCostType,
    #[bitfield(name = "AdditionalServicesCosts_isUsed", ty = "u32", bits = "0..=0")]
    pub AdditionalServicesCosts_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub OverstayCosts: iso20_ac_DetailedCostType,
    #[bitfield(name = "OverstayCosts_isUsed", ty = "u32", bits = "0..=0")]
    pub OverstayCosts_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub TaxCosts: C2RustUnnamed_59,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_59 {
    pub array: [iso20_ac_DetailedTaxType; 10],
    pub arrayLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_Scheduled_AC_CLResControlModeType {
    pub EVSETargetActivePower: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSETargetActivePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSETargetActivePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub EVSETargetActivePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSETargetActivePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSETargetActivePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub EVSETargetActivePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSETargetActivePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSETargetActivePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub EVSETargetReactivePower: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSETargetReactivePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSETargetReactivePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub EVSETargetReactivePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSETargetReactivePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSETargetReactivePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 1],
    pub EVSETargetReactivePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSETargetReactivePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSETargetReactivePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 1],
    pub EVSEPresentActivePower: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEPresentActivePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEPresentActivePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_5: [u8; 1],
    pub EVSEPresentActivePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEPresentActivePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEPresentActivePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_6: [u8; 1],
    pub EVSEPresentActivePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEPresentActivePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEPresentActivePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_7: [u8; 3],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_Dynamic_AC_CLResControlModeType {
    pub DepartureTime: u32,
    #[bitfield(name = "DepartureTime_isUsed", ty = "u32", bits = "0..=0")]
    pub DepartureTime_isUsed: [u8; 1],
    pub MinimumSOC: int8_t,
    #[bitfield(name = "MinimumSOC_isUsed", ty = "u32", bits = "0..=0")]
    pub MinimumSOC_isUsed: [u8; 1],
    pub TargetSOC: int8_t,
    #[bitfield(name = "TargetSOC_isUsed", ty = "u32", bits = "0..=0")]
    pub TargetSOC_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub AckMaxDelay: u16,
    #[bitfield(name = "AckMaxDelay_isUsed", ty = "u32", bits = "0..=0")]
    pub AckMaxDelay_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub EVSETargetActivePower: iso20_ac_RationalNumberType,
    pub EVSETargetActivePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSETargetActivePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSETargetActivePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub EVSETargetActivePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSETargetActivePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSETargetActivePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub EVSETargetReactivePower: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSETargetReactivePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSETargetReactivePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 1],
    pub EVSETargetReactivePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSETargetReactivePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSETargetReactivePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 1],
    pub EVSETargetReactivePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSETargetReactivePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSETargetReactivePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_5: [u8; 1],
    pub EVSEPresentActivePower: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEPresentActivePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEPresentActivePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_6: [u8; 1],
    pub EVSEPresentActivePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEPresentActivePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEPresentActivePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_7: [u8; 1],
    pub EVSEPresentActivePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEPresentActivePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEPresentActivePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_8: [u8; 3],
}
#[derive(Copy, Clone)]

pub struct iso20_ac_CLResControlModeType {
    _unused: i32,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_BPT_AC_CPDReqEnergyTransferModeType {
    pub EVMaximumChargePower: iso20_ac_RationalNumberType,
    pub EVMaximumChargePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMaximumChargePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumChargePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub EVMaximumChargePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMaximumChargePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumChargePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub EVMinimumChargePower: iso20_ac_RationalNumberType,
    pub EVMinimumChargePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMinimumChargePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumChargePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub EVMinimumChargePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMinimumChargePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumChargePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub EVMaximumDischargePower: iso20_ac_RationalNumberType,
    pub EVMaximumDischargePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMaximumDischargePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumDischargePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 1],
    pub EVMaximumDischargePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMaximumDischargePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumDischargePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 1],
    pub EVMinimumDischargePower: iso20_ac_RationalNumberType,
    pub EVMinimumDischargePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMinimumDischargePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumDischargePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_5: [u8; 1],
    pub EVMinimumDischargePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMinimumDischargePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumDischargePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_6: [u8; 1],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_AC_ChargeParameterDiscoveryReqType {
    pub Header: iso20_ac_MessageHeaderType,
    pub AC_CPDReqEnergyTransferMode: iso20_ac_AC_CPDReqEnergyTransferModeType,
    #[bitfield(
        name = "AC_CPDReqEnergyTransferMode_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub AC_CPDReqEnergyTransferMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub BPT_AC_CPDReqEnergyTransferMode: iso20_ac_BPT_AC_CPDReqEnergyTransferModeType,
    #[bitfield(
        name = "BPT_AC_CPDReqEnergyTransferMode_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub BPT_AC_CPDReqEnergyTransferMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_BPT_AC_CPDResEnergyTransferModeType {
    pub EVSEMaximumChargePower: iso20_ac_RationalNumberType,
    pub EVSEMaximumChargePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEMaximumChargePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEMaximumChargePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub EVSEMaximumChargePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEMaximumChargePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEMaximumChargePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub EVSEMinimumChargePower: iso20_ac_RationalNumberType,
    pub EVSEMinimumChargePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEMinimumChargePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEMinimumChargePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub EVSEMinimumChargePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEMinimumChargePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEMinimumChargePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub EVSENominalFrequency: iso20_ac_RationalNumberType,
    pub MaximumPowerAsymmetry: iso20_ac_RationalNumberType,
    #[bitfield(name = "MaximumPowerAsymmetry_isUsed", ty = "u32", bits = "0..=0")]
    pub MaximumPowerAsymmetry_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 1],
    pub EVSEPowerRampLimitation: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEPowerRampLimitation_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEPowerRampLimitation_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 1],
    pub EVSEPresentActivePower: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEPresentActivePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEPresentActivePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_5: [u8; 1],
    pub EVSEPresentActivePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEPresentActivePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEPresentActivePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_6: [u8; 1],
    pub EVSEPresentActivePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEPresentActivePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEPresentActivePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_7: [u8; 1],
    pub EVSEMaximumDischargePower: iso20_ac_RationalNumberType,
    pub EVSEMaximumDischargePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(
        name = "EVSEMaximumDischargePower_L2_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub EVSEMaximumDischargePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_8: [u8; 1],
    pub EVSEMaximumDischargePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(
        name = "EVSEMaximumDischargePower_L3_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub EVSEMaximumDischargePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_9: [u8; 1],
    pub EVSEMinimumDischargePower: iso20_ac_RationalNumberType,
    pub EVSEMinimumDischargePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(
        name = "EVSEMinimumDischargePower_L2_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub EVSEMinimumDischargePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_10: [u8; 1],
    pub EVSEMinimumDischargePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(
        name = "EVSEMinimumDischargePower_L3_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub EVSEMinimumDischargePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_11: [u8; 3],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_AC_ChargeParameterDiscoveryResType {
    pub Header: iso20_ac_MessageHeaderType,
    pub ResponseCode: iso20_ac_responseCodeType,
    pub AC_CPDResEnergyTransferMode: iso20_ac_AC_CPDResEnergyTransferModeType,
    #[bitfield(
        name = "AC_CPDResEnergyTransferMode_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub AC_CPDResEnergyTransferMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub BPT_AC_CPDResEnergyTransferMode: iso20_ac_BPT_AC_CPDResEnergyTransferModeType,
    #[bitfield(
        name = "BPT_AC_CPDResEnergyTransferMode_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub BPT_AC_CPDResEnergyTransferMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 7],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_BPT_Scheduled_AC_CLReqControlModeType {
    pub EVTargetEnergyRequest: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVTargetEnergyRequest_isUsed", ty = "u32", bits = "0..=0")]
    pub EVTargetEnergyRequest_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub EVMaximumEnergyRequest: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMaximumEnergyRequest_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumEnergyRequest_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub EVMinimumEnergyRequest: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMinimumEnergyRequest_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumEnergyRequest_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub EVMaximumChargePower: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMaximumChargePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumChargePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub EVMaximumChargePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMaximumChargePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumChargePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 1],
    pub EVMaximumChargePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMaximumChargePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumChargePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 1],
    pub EVMinimumChargePower: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMinimumChargePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumChargePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_5: [u8; 1],
    pub EVMinimumChargePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMinimumChargePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumChargePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_6: [u8; 1],
    pub EVMinimumChargePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMinimumChargePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumChargePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_7: [u8; 1],
    pub EVPresentActivePower: iso20_ac_RationalNumberType,
    pub EVPresentActivePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVPresentActivePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVPresentActivePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_8: [u8; 1],
    pub EVPresentActivePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVPresentActivePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVPresentActivePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_9: [u8; 1],
    pub EVPresentReactivePower: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVPresentReactivePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVPresentReactivePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_10: [u8; 1],
    pub EVPresentReactivePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVPresentReactivePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVPresentReactivePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_11: [u8; 1],
    pub EVPresentReactivePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVPresentReactivePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVPresentReactivePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_12: [u8; 1],
    pub EVMaximumDischargePower: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMaximumDischargePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumDischargePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_13: [u8; 1],
    pub EVMaximumDischargePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMaximumDischargePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumDischargePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_14: [u8; 1],
    pub EVMaximumDischargePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMaximumDischargePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumDischargePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_15: [u8; 1],
    pub EVMinimumDischargePower: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMinimumDischargePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumDischargePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_16: [u8; 1],
    pub EVMinimumDischargePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMinimumDischargePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumDischargePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_17: [u8; 1],
    pub EVMinimumDischargePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMinimumDischargePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumDischargePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_18: [u8; 1],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_BPT_Scheduled_AC_CLResControlModeType {
    pub EVSETargetActivePower: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSETargetActivePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSETargetActivePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub EVSETargetActivePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSETargetActivePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSETargetActivePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub EVSETargetActivePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSETargetActivePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSETargetActivePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub EVSETargetReactivePower: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSETargetReactivePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSETargetReactivePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub EVSETargetReactivePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSETargetReactivePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSETargetReactivePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 1],
    pub EVSETargetReactivePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSETargetReactivePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSETargetReactivePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 1],
    pub EVSEPresentActivePower: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEPresentActivePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEPresentActivePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_5: [u8; 1],
    pub EVSEPresentActivePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEPresentActivePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEPresentActivePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_6: [u8; 1],
    pub EVSEPresentActivePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEPresentActivePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEPresentActivePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_7: [u8; 3],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_BPT_Dynamic_AC_CLReqControlModeType {
    pub DepartureTime: u32,
    #[bitfield(name = "DepartureTime_isUsed", ty = "u32", bits = "0..=0")]
    pub DepartureTime_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub EVTargetEnergyRequest: iso20_ac_RationalNumberType,
    pub EVMaximumEnergyRequest: iso20_ac_RationalNumberType,
    pub EVMinimumEnergyRequest: iso20_ac_RationalNumberType,
    pub EVMaximumChargePower: iso20_ac_RationalNumberType,
    pub EVMaximumChargePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMaximumChargePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumChargePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub EVMaximumChargePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMaximumChargePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumChargePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub EVMinimumChargePower: iso20_ac_RationalNumberType,
    pub EVMinimumChargePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMinimumChargePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumChargePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub EVMinimumChargePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMinimumChargePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumChargePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 1],
    pub EVPresentActivePower: iso20_ac_RationalNumberType,
    pub EVPresentActivePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVPresentActivePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVPresentActivePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 1],
    pub EVPresentActivePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVPresentActivePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVPresentActivePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_5: [u8; 1],
    pub EVPresentReactivePower: iso20_ac_RationalNumberType,
    pub EVPresentReactivePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVPresentReactivePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVPresentReactivePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_6: [u8; 1],
    pub EVPresentReactivePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVPresentReactivePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVPresentReactivePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_7: [u8; 1],
    pub EVMaximumDischargePower: iso20_ac_RationalNumberType,
    pub EVMaximumDischargePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMaximumDischargePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumDischargePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_8: [u8; 1],
    pub EVMaximumDischargePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMaximumDischargePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumDischargePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_9: [u8; 1],
    pub EVMinimumDischargePower: iso20_ac_RationalNumberType,
    pub EVMinimumDischargePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMinimumDischargePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumDischargePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_10: [u8; 1],
    pub EVMinimumDischargePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMinimumDischargePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumDischargePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_11: [u8; 1],
    pub EVMaximumV2XEnergyRequest: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMaximumV2XEnergyRequest_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumV2XEnergyRequest_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_12: [u8; 1],
    pub EVMinimumV2XEnergyRequest: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVMinimumV2XEnergyRequest_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumV2XEnergyRequest_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_13: [u8; 3],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_AC_ChargeLoopReqType {
    pub Header: iso20_ac_MessageHeaderType,
    pub DisplayParameters: iso20_ac_DisplayParametersType,
    #[bitfield(name = "DisplayParameters_isUsed", ty = "u32", bits = "0..=0")]
    pub DisplayParameters_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub MeterInfoRequested: i32,
    pub BPT_Dynamic_AC_CLReqControlMode: iso20_ac_BPT_Dynamic_AC_CLReqControlModeType,
    #[bitfield(
        name = "BPT_Dynamic_AC_CLReqControlMode_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub BPT_Dynamic_AC_CLReqControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
    pub BPT_Scheduled_AC_CLReqControlMode: iso20_ac_BPT_Scheduled_AC_CLReqControlModeType,
    #[bitfield(
        name = "BPT_Scheduled_AC_CLReqControlMode_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub BPT_Scheduled_AC_CLReqControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 3],
    pub CLReqControlMode: iso20_ac_CLReqControlModeType,
    #[bitfield(name = "CLReqControlMode_isUsed", ty = "u32", bits = "0..=0")]
    pub CLReqControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 3],
    pub Dynamic_AC_CLReqControlMode: iso20_ac_Dynamic_AC_CLReqControlModeType,
    #[bitfield(
        name = "Dynamic_AC_CLReqControlMode_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub Dynamic_AC_CLReqControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 3],
    pub Scheduled_AC_CLReqControlMode: iso20_ac_Scheduled_AC_CLReqControlModeType,
    #[bitfield(
        name = "Scheduled_AC_CLReqControlMode_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub Scheduled_AC_CLReqControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 7],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_BPT_Dynamic_AC_CLResControlModeType {
    pub DepartureTime: u32,
    #[bitfield(name = "DepartureTime_isUsed", ty = "u32", bits = "0..=0")]
    pub DepartureTime_isUsed: [u8; 1],
    pub MinimumSOC: int8_t,
    #[bitfield(name = "MinimumSOC_isUsed", ty = "u32", bits = "0..=0")]
    pub MinimumSOC_isUsed: [u8; 1],
    pub TargetSOC: int8_t,
    #[bitfield(name = "TargetSOC_isUsed", ty = "u32", bits = "0..=0")]
    pub TargetSOC_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub AckMaxDelay: u16,
    #[bitfield(name = "AckMaxDelay_isUsed", ty = "u32", bits = "0..=0")]
    pub AckMaxDelay_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub EVSETargetActivePower: iso20_ac_RationalNumberType,
    pub EVSETargetActivePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSETargetActivePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSETargetActivePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub EVSETargetActivePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSETargetActivePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSETargetActivePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub EVSETargetReactivePower: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSETargetReactivePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSETargetReactivePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 1],
    pub EVSETargetReactivePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSETargetReactivePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSETargetReactivePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 1],
    pub EVSETargetReactivePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSETargetReactivePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSETargetReactivePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_5: [u8; 1],
    pub EVSEPresentActivePower: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEPresentActivePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEPresentActivePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_6: [u8; 1],
    pub EVSEPresentActivePower_L2: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEPresentActivePower_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEPresentActivePower_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_7: [u8; 1],
    pub EVSEPresentActivePower_L3: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSEPresentActivePower_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEPresentActivePower_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_8: [u8; 3],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_AC_ChargeLoopResType {
    pub Header: iso20_ac_MessageHeaderType,
    pub ResponseCode: iso20_ac_responseCodeType,
    pub EVSEStatus: iso20_ac_EVSEStatusType,
    #[bitfield(name = "EVSEStatus_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEStatus_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub MeterInfo: iso20_ac_MeterInfoType,
    #[bitfield(name = "MeterInfo_isUsed", ty = "u32", bits = "0..=0")]
    pub MeterInfo_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 7],
    pub Receipt: iso20_ac_ReceiptType,
    #[bitfield(name = "Receipt_isUsed", ty = "u32", bits = "0..=0")]
    pub Receipt_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub EVSETargetFrequency: iso20_ac_RationalNumberType,
    #[bitfield(name = "EVSETargetFrequency_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSETargetFrequency_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub BPT_Dynamic_AC_CLResControlMode: iso20_ac_BPT_Dynamic_AC_CLResControlModeType,
    #[bitfield(
        name = "BPT_Dynamic_AC_CLResControlMode_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub BPT_Dynamic_AC_CLResControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 3],
    pub BPT_Scheduled_AC_CLResControlMode: iso20_ac_BPT_Scheduled_AC_CLResControlModeType,
    #[bitfield(
        name = "BPT_Scheduled_AC_CLResControlMode_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub BPT_Scheduled_AC_CLResControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 3],
    pub CLResControlMode: iso20_ac_CLResControlModeType,
    #[bitfield(name = "CLResControlMode_isUsed", ty = "u32", bits = "0..=0")]
    pub CLResControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_5: [u8; 3],
    pub Dynamic_AC_CLResControlMode: iso20_ac_Dynamic_AC_CLResControlModeType,
    #[bitfield(
        name = "Dynamic_AC_CLResControlMode_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub Dynamic_AC_CLResControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_6: [u8; 3],
    pub Scheduled_AC_CLResControlMode: iso20_ac_Scheduled_AC_CLResControlModeType,
    #[bitfield(
        name = "Scheduled_AC_CLResControlMode_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub Scheduled_AC_CLResControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_7: [u8; 3],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_ManifestType {
    pub Id: C2RustUnnamed_61,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub Reference: C2RustUnnamed_60,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_60 {
    pub array: [iso20_ac_ReferenceType; 4],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_61 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_SignaturePropertiesType {
    pub Id: C2RustUnnamed_62,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub SignatureProperty: iso20_ac_SignaturePropertyType,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_62 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_exiDocument {
    pub c2rust_unnamed: C2RustUnnamed_63,
    #[bitfield(
        name = "AC_ChargeParameterDiscoveryReq_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    #[bitfield(
        name = "AC_ChargeParameterDiscoveryRes_isUsed",
        ty = "u32",
        bits = "1..=1"
    )]
    #[bitfield(name = "AC_ChargeLoopReq_isUsed", ty = "u32", bits = "2..=2")]
    #[bitfield(name = "AC_ChargeLoopRes_isUsed", ty = "u32", bits = "3..=3")]
    #[bitfield(
        name = "AC_CPDReqEnergyTransferMode_isUsed",
        ty = "u32",
        bits = "4..=4"
    )]
    #[bitfield(
        name = "AC_CPDResEnergyTransferMode_isUsed",
        ty = "u32",
        bits = "5..=5"
    )]
    #[bitfield(
        name = "BPT_AC_CPDReqEnergyTransferMode_isUsed",
        ty = "u32",
        bits = "6..=6"
    )]
    #[bitfield(
        name = "BPT_AC_CPDResEnergyTransferMode_isUsed",
        ty = "u32",
        bits = "7..=7"
    )]
    #[bitfield(
        name = "Scheduled_AC_CLReqControlMode_isUsed",
        ty = "u32",
        bits = "8..=8"
    )]
    #[bitfield(
        name = "Scheduled_AC_CLResControlMode_isUsed",
        ty = "u32",
        bits = "9..=9"
    )]
    #[bitfield(
        name = "BPT_Scheduled_AC_CLReqControlMode_isUsed",
        ty = "u32",
        bits = "10..=10"
    )]
    #[bitfield(
        name = "BPT_Scheduled_AC_CLResControlMode_isUsed",
        ty = "u32",
        bits = "11..=11"
    )]
    #[bitfield(
        name = "Dynamic_AC_CLReqControlMode_isUsed",
        ty = "u32",
        bits = "12..=12"
    )]
    #[bitfield(
        name = "Dynamic_AC_CLResControlMode_isUsed",
        ty = "u32",
        bits = "13..=13"
    )]
    #[bitfield(
        name = "BPT_Dynamic_AC_CLReqControlMode_isUsed",
        ty = "u32",
        bits = "14..=14"
    )]
    #[bitfield(
        name = "BPT_Dynamic_AC_CLResControlMode_isUsed",
        ty = "u32",
        bits = "15..=15"
    )]
    #[bitfield(name = "CLReqControlMode_isUsed", ty = "u32", bits = "16..=16")]
    #[bitfield(name = "CLResControlMode_isUsed", ty = "u32", bits = "17..=17")]
    #[bitfield(name = "Signature_isUsed", ty = "u32", bits = "18..=18")]
    #[bitfield(name = "SignatureValue_isUsed", ty = "u32", bits = "19..=19")]
    #[bitfield(name = "SignedInfo_isUsed", ty = "u32", bits = "20..=20")]
    #[bitfield(name = "CanonicalizationMethod_isUsed", ty = "u32", bits = "21..=21")]
    #[bitfield(name = "SignatureMethod_isUsed", ty = "u32", bits = "22..=22")]
    #[bitfield(name = "Reference_isUsed", ty = "u32", bits = "23..=23")]
    #[bitfield(name = "Transforms_isUsed", ty = "u32", bits = "24..=24")]
    #[bitfield(name = "Transform_isUsed", ty = "u32", bits = "25..=25")]
    #[bitfield(name = "DigestMethod_isUsed", ty = "u32", bits = "26..=26")]
    #[bitfield(name = "KeyInfo_isUsed", ty = "u32", bits = "27..=27")]
    #[bitfield(name = "KeyValue_isUsed", ty = "u32", bits = "28..=28")]
    #[bitfield(name = "RetrievalMethod_isUsed", ty = "u32", bits = "29..=29")]
    #[bitfield(name = "X509Data_isUsed", ty = "u32", bits = "30..=30")]
    #[bitfield(name = "PGPData_isUsed", ty = "u32", bits = "31..=31")]
    #[bitfield(name = "SPKIData_isUsed", ty = "u32", bits = "32..=32")]
    #[bitfield(name = "Object_isUsed", ty = "u32", bits = "33..=33")]
    #[bitfield(name = "Manifest_isUsed", ty = "u32", bits = "34..=34")]
    #[bitfield(name = "SignatureProperties_isUsed", ty = "u32", bits = "35..=35")]
    #[bitfield(name = "SignatureProperty_isUsed", ty = "u32", bits = "36..=36")]
    #[bitfield(name = "DSAKeyValue_isUsed", ty = "u32", bits = "37..=37")]
    #[bitfield(name = "RSAKeyValue_isUsed", ty = "u32", bits = "38..=38")]
    pub AC_ChargeParameterDiscoveryReq_isUsed_AC_ChargeParameterDiscoveryRes_isUsed_AC_ChargeLoopReq_isUsed_AC_ChargeLoopRes_isUsed_AC_CPDReqEnergyTransferMode_isUsed_AC_CPDResEnergyTransferMode_isUsed_BPT_AC_CPDReqEnergyTransferMode_isUsed_BPT_AC_CPDResEnergyTransferMode_isUsed_Scheduled_AC_CLReqControlMode_isUsed_Scheduled_AC_CLResControlMode_isUsed_BPT_Scheduled_AC_CLReqControlMode_isUsed_BPT_Scheduled_AC_CLResControlMode_isUsed_Dynamic_AC_CLReqControlMode_isUsed_Dynamic_AC_CLResControlMode_isUsed_BPT_Dynamic_AC_CLReqControlMode_isUsed_BPT_Dynamic_AC_CLResControlMode_isUsed_CLReqControlMode_isUsed_CLResControlMode_isUsed_Signature_isUsed_SignatureValue_isUsed_SignedInfo_isUsed_CanonicalizationMethod_isUsed_SignatureMethod_isUsed_Reference_isUsed_Transforms_isUsed_Transform_isUsed_DigestMethod_isUsed_KeyInfo_isUsed_KeyValue_isUsed_RetrievalMethod_isUsed_X509Data_isUsed_PGPData_isUsed_SPKIData_isUsed_Object_isUsed_Manifest_isUsed_SignatureProperties_isUsed_SignatureProperty_isUsed_DSAKeyValue_isUsed_RSAKeyValue_isUsed:
        [u8; 5],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]

pub union C2RustUnnamed_63 {
    pub AC_ChargeParameterDiscoveryReq: iso20_ac_AC_ChargeParameterDiscoveryReqType,
    pub AC_ChargeParameterDiscoveryRes: iso20_ac_AC_ChargeParameterDiscoveryResType,
    pub AC_ChargeLoopReq: iso20_ac_AC_ChargeLoopReqType,
    pub AC_ChargeLoopRes: iso20_ac_AC_ChargeLoopResType,
    pub AC_CPDReqEnergyTransferMode: iso20_ac_AC_CPDReqEnergyTransferModeType,
    pub AC_CPDResEnergyTransferMode: iso20_ac_AC_CPDResEnergyTransferModeType,
    pub BPT_AC_CPDReqEnergyTransferMode: iso20_ac_BPT_AC_CPDReqEnergyTransferModeType,
    pub BPT_AC_CPDResEnergyTransferMode: iso20_ac_BPT_AC_CPDResEnergyTransferModeType,
    pub Scheduled_AC_CLReqControlMode: iso20_ac_Scheduled_AC_CLReqControlModeType,
    pub Scheduled_AC_CLResControlMode: iso20_ac_Scheduled_AC_CLResControlModeType,
    pub BPT_Scheduled_AC_CLReqControlMode: iso20_ac_BPT_Scheduled_AC_CLReqControlModeType,
    pub BPT_Scheduled_AC_CLResControlMode: iso20_ac_BPT_Scheduled_AC_CLResControlModeType,
    pub Dynamic_AC_CLReqControlMode: iso20_ac_Dynamic_AC_CLReqControlModeType,
    pub Dynamic_AC_CLResControlMode: iso20_ac_Dynamic_AC_CLResControlModeType,
    pub BPT_Dynamic_AC_CLReqControlMode: iso20_ac_BPT_Dynamic_AC_CLReqControlModeType,
    pub BPT_Dynamic_AC_CLResControlMode: iso20_ac_BPT_Dynamic_AC_CLResControlModeType,
    pub CLReqControlMode: iso20_ac_CLReqControlModeType,
    pub CLResControlMode: iso20_ac_CLResControlModeType,
    pub Signature: iso20_ac_SignatureType,
    pub SignatureValue: iso20_ac_SignatureValueType,
    pub SignedInfo: iso20_ac_SignedInfoType,
    pub CanonicalizationMethod: iso20_ac_CanonicalizationMethodType,
    pub SignatureMethod: iso20_ac_SignatureMethodType,
    pub Reference: iso20_ac_ReferenceType,
    pub Transforms: iso20_ac_TransformsType,
    pub Transform: iso20_ac_TransformType,
    pub DigestMethod: iso20_ac_DigestMethodType,
    pub KeyInfo: iso20_ac_KeyInfoType,
    pub KeyValue: iso20_ac_KeyValueType,
    pub RetrievalMethod: iso20_ac_RetrievalMethodType,
    pub X509Data: iso20_ac_X509DataType,
    pub PGPData: iso20_ac_PGPDataType,
    pub SPKIData: iso20_ac_SPKIDataType,
    pub Object: iso20_ac_ObjectType,
    pub Manifest: iso20_ac_ManifestType,
    pub SignatureProperties: iso20_ac_SignaturePropertiesType,
    pub SignatureProperty: iso20_ac_SignaturePropertyType,
    pub DSAKeyValue: iso20_ac_DSAKeyValueType,
    pub RSAKeyValue: iso20_ac_RSAKeyValueType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_exiFragment {
    pub c2rust_unnamed: C2RustUnnamed_64,
    #[bitfield(
        name = "AC_ChargeParameterDiscoveryRes_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    #[bitfield(name = "SignedInfo_isUsed", ty = "u32", bits = "1..=1")]
    pub AC_ChargeParameterDiscoveryRes_isUsed_SignedInfo_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]

pub union C2RustUnnamed_64 {
    pub AC_ChargeParameterDiscoveryRes: iso20_ac_AC_ChargeParameterDiscoveryResType,
    pub SignedInfo: iso20_ac_SignedInfoType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ac_xmldsigFragment {
    pub c2rust_unnamed: C2RustUnnamed_65,
    #[bitfield(name = "CanonicalizationMethod_isUsed", ty = "u32", bits = "0..=0")]
    #[bitfield(name = "DSAKeyValue_isUsed", ty = "u32", bits = "1..=1")]
    #[bitfield(name = "DigestMethod_isUsed", ty = "u32", bits = "2..=2")]
    #[bitfield(name = "KeyInfo_isUsed", ty = "u32", bits = "3..=3")]
    #[bitfield(name = "KeyValue_isUsed", ty = "u32", bits = "4..=4")]
    #[bitfield(name = "Manifest_isUsed", ty = "u32", bits = "5..=5")]
    #[bitfield(name = "Object_isUsed", ty = "u32", bits = "6..=6")]
    #[bitfield(name = "PGPData_isUsed", ty = "u32", bits = "7..=7")]
    #[bitfield(name = "RSAKeyValue_isUsed", ty = "u32", bits = "8..=8")]
    #[bitfield(name = "Reference_isUsed", ty = "u32", bits = "9..=9")]
    #[bitfield(name = "RetrievalMethod_isUsed", ty = "u32", bits = "10..=10")]
    #[bitfield(name = "SPKIData_isUsed", ty = "u32", bits = "11..=11")]
    #[bitfield(name = "Signature_isUsed", ty = "u32", bits = "12..=12")]
    #[bitfield(name = "SignatureMethod_isUsed", ty = "u32", bits = "13..=13")]
    #[bitfield(name = "SignatureProperties_isUsed", ty = "u32", bits = "14..=14")]
    #[bitfield(name = "SignatureProperty_isUsed", ty = "u32", bits = "15..=15")]
    #[bitfield(name = "SignatureValue_isUsed", ty = "u32", bits = "16..=16")]
    #[bitfield(name = "SignedInfo_isUsed", ty = "u32", bits = "17..=17")]
    #[bitfield(name = "Transform_isUsed", ty = "u32", bits = "18..=18")]
    #[bitfield(name = "Transforms_isUsed", ty = "u32", bits = "19..=19")]
    #[bitfield(name = "X509Data_isUsed", ty = "u32", bits = "20..=20")]
    #[bitfield(name = "X509IssuerSerial_isUsed", ty = "u32", bits = "21..=21")]
    pub CanonicalizationMethod_isUsed_DSAKeyValue_isUsed_DigestMethod_isUsed_KeyInfo_isUsed_KeyValue_isUsed_Manifest_isUsed_Object_isUsed_PGPData_isUsed_RSAKeyValue_isUsed_Reference_isUsed_RetrievalMethod_isUsed_SPKIData_isUsed_Signature_isUsed_SignatureMethod_isUsed_SignatureProperties_isUsed_SignatureProperty_isUsed_SignatureValue_isUsed_SignedInfo_isUsed_Transform_isUsed_Transforms_isUsed_X509Data_isUsed_X509IssuerSerial_isUsed:
        [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 5],
}
#[derive(Copy, Clone)]

pub union C2RustUnnamed_65 {
    pub CanonicalizationMethod: iso20_ac_CanonicalizationMethodType,
    pub DSAKeyValue: iso20_ac_DSAKeyValueType,
    pub DigestMethod: iso20_ac_DigestMethodType,
    pub KeyInfo: iso20_ac_KeyInfoType,
    pub KeyValue: iso20_ac_KeyValueType,
    pub Manifest: iso20_ac_ManifestType,
    pub Object: iso20_ac_ObjectType,
    pub PGPData: iso20_ac_PGPDataType,
    pub RSAKeyValue: iso20_ac_RSAKeyValueType,
    pub Reference: iso20_ac_ReferenceType,
    pub RetrievalMethod: iso20_ac_RetrievalMethodType,
    pub SPKIData: iso20_ac_SPKIDataType,
    pub Signature: iso20_ac_SignatureType,
    pub SignatureMethod: iso20_ac_SignatureMethodType,
    pub SignatureProperties: iso20_ac_SignaturePropertiesType,
    pub SignatureProperty: iso20_ac_SignaturePropertyType,
    pub SignatureValue: iso20_ac_SignatureValueType,
    pub SignedInfo: iso20_ac_SignedInfoType,
    pub Transform: iso20_ac_TransformType,
    pub Transforms: iso20_ac_TransformsType,
    pub X509Data: iso20_ac_X509DataType,
    pub X509IssuerSerial: iso20_ac_X509IssuerSerialType,
}
unsafe extern "C" fn encode_iso20_ac_TransformType(
    stream: &mut ExiBitstream,
    mut TransformType: *const iso20_ac_TransformType,
) -> i32 {
    let mut grammar_id: i32 = 0 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            0 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = exi_basetypes_encoder_uint_16(
                        stream,
                        ((*TransformType).Algorithm.charactersLen as i32 + 2 as i32) as u16,
                    );
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_characters(
                            stream,
                            (*TransformType).Algorithm.charactersLen as usize,
                            ((*TransformType).Algorithm.characters).as_ptr(),
                            (64 as i32 + 1 as i32) as usize,
                        );
                        if error == 0 as i32 {
                            grammar_id = 1 as i32;
                        }
                    }
                }
            }
            1 => {
                if (*TransformType).XPath_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                ((*TransformType).XPath.charactersLen as i32 + 2 as i32) as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*TransformType).XPath.charactersLen as usize,
                                    ((*TransformType).XPath.characters).as_ptr(),
                                    (64 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 2 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*TransformType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*TransformType).ANY.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*TransformType).ANY.bytesLen as usize,
                                    ((*TransformType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 2 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_TransformsType(
    stream: &mut ExiBitstream,
    mut TransformsType: *const iso20_ac_TransformsType,
) -> i32 {
    let mut grammar_id: i32 = 4 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            4 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_TransformType(stream, &(*TransformsType).Transform);
                    if error == 0 as i32 {
                        grammar_id = 5 as i32;
                    }
                }
            }
            5 => {
                if 1 as i32 == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_TransformType(stream, &(*TransformsType).Transform);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_DSAKeyValueType(
    stream: &mut ExiBitstream,
    mut DSAKeyValueType: *const iso20_ac_DSAKeyValueType,
) -> i32 {
    let mut grammar_id: i32 = 6 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            6 => {
                if (*DSAKeyValueType).P_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*DSAKeyValueType).P.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*DSAKeyValueType).P.bytesLen as usize,
                                    ((*DSAKeyValueType).P.bytes).as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 7 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*DSAKeyValueType).G_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*DSAKeyValueType).G.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*DSAKeyValueType).G.bytesLen as usize,
                                    ((*DSAKeyValueType).G.bytes).as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 9 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*DSAKeyValueType).Y.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*DSAKeyValueType).Y.bytesLen as usize,
                                    ((*DSAKeyValueType).Y.bytes).as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 10 as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            7 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                    if error == 0 as i32 {
                        error =
                            exi_basetypes_encoder_uint_16(stream, (*DSAKeyValueType).Q.bytesLen);
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*DSAKeyValueType).Q.bytesLen as usize,
                                ((*DSAKeyValueType).Q.bytes).as_ptr(),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 8 as i32;
                                }
                            }
                        }
                    }
                }
            }
            8 => {
                if (*DSAKeyValueType).G_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*DSAKeyValueType).G.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*DSAKeyValueType).G.bytesLen as usize,
                                    ((*DSAKeyValueType).G.bytes).as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 9 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*DSAKeyValueType).Y.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*DSAKeyValueType).Y.bytesLen as usize,
                                    ((*DSAKeyValueType).Y.bytes).as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 10 as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            9 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                    if error == 0 as i32 {
                        error =
                            exi_basetypes_encoder_uint_16(stream, (*DSAKeyValueType).Y.bytesLen);
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*DSAKeyValueType).Y.bytesLen as usize,
                                ((*DSAKeyValueType).Y.bytes).as_ptr(),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 10 as i32;
                                }
                            }
                        }
                    }
                }
            }
            10 => {
                if (*DSAKeyValueType).J_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*DSAKeyValueType).J.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*DSAKeyValueType).J.bytesLen as usize,
                                    ((*DSAKeyValueType).J.bytes).as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 11 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*DSAKeyValueType).Seed_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*DSAKeyValueType).Seed.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*DSAKeyValueType).Seed.bytesLen as usize,
                                    ((*DSAKeyValueType).Seed.bytes).as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 12 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            11 => {
                if (*DSAKeyValueType).Seed_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*DSAKeyValueType).Seed.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*DSAKeyValueType).Seed.bytesLen as usize,
                                    ((*DSAKeyValueType).Seed.bytes).as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 12 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            12 => {
                if (*DSAKeyValueType).PgenCounter_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*DSAKeyValueType).PgenCounter.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*DSAKeyValueType).PgenCounter.bytesLen as usize,
                                    ((*DSAKeyValueType).PgenCounter.bytes).as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 2 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_X509IssuerSerialType(
    stream: &mut ExiBitstream,
    mut X509IssuerSerialType: *const iso20_ac_X509IssuerSerialType,
) -> i32 {
    let mut grammar_id: i32 = 13 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            13 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*X509IssuerSerialType).X509IssuerName.charactersLen as i32 + 2 as i32)
                                as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*X509IssuerSerialType).X509IssuerName.charactersLen as usize,
                                ((*X509IssuerSerialType).X509IssuerName.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 14 as i32;
                                }
                            }
                        }
                    }
                }
            }
            14 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_signed(
                            stream,
                            &(*X509IssuerSerialType).X509SerialNumber,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 2 as i32;
                            }
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_DigestMethodType(
    stream: &mut ExiBitstream,
    mut DigestMethodType: *const iso20_ac_DigestMethodType,
) -> i32 {
    let mut grammar_id: i32 = 15 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            15 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = exi_basetypes_encoder_uint_16(
                        stream,
                        ((*DigestMethodType).Algorithm.charactersLen as i32 + 2 as i32) as u16,
                    );
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_characters(
                            stream,
                            (*DigestMethodType).Algorithm.charactersLen as usize,
                            ((*DigestMethodType).Algorithm.characters).as_ptr(),
                            (64 as i32 + 1 as i32) as usize,
                        );
                        if error == 0 as i32 {
                            grammar_id = 16 as i32;
                        }
                    }
                }
            }
            16 => {
                if (*DigestMethodType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*DigestMethodType).ANY.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*DigestMethodType).ANY.bytesLen as usize,
                                    ((*DigestMethodType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 2 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_RSAKeyValueType(
    stream: &mut ExiBitstream,
    mut RSAKeyValueType: *const iso20_ac_RSAKeyValueType,
) -> i32 {
    let mut grammar_id: i32 = 17 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            17 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*RSAKeyValueType).Modulus.bytesLen,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*RSAKeyValueType).Modulus.bytesLen as usize,
                                ((*RSAKeyValueType).Modulus.bytes).as_ptr(),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 18 as i32;
                                }
                            }
                        }
                    }
                }
            }
            18 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*RSAKeyValueType).Exponent.bytesLen,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*RSAKeyValueType).Exponent.bytesLen as usize,
                                ((*RSAKeyValueType).Exponent.bytes).as_ptr(),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 2 as i32;
                                }
                            }
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_CanonicalizationMethodType(
    stream: &mut ExiBitstream,
    mut CanonicalizationMethodType: *const iso20_ac_CanonicalizationMethodType,
) -> i32 {
    let mut grammar_id: i32 = 19 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            19 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = exi_basetypes_encoder_uint_16(
                        stream,
                        ((*CanonicalizationMethodType).Algorithm.charactersLen as i32 + 2 as i32)
                            as u16,
                    );
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_characters(
                            stream,
                            (*CanonicalizationMethodType).Algorithm.charactersLen as usize,
                            ((*CanonicalizationMethodType).Algorithm.characters).as_ptr(),
                            (64 as i32 + 1 as i32) as usize,
                        );
                        if error == 0 as i32 {
                            grammar_id = 20 as i32;
                        }
                    }
                }
            }
            20 => {
                if (*CanonicalizationMethodType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*CanonicalizationMethodType).ANY.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*CanonicalizationMethodType).ANY.bytesLen as usize,
                                    ((*CanonicalizationMethodType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 2 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_SignatureMethodType(
    stream: &mut ExiBitstream,
    mut SignatureMethodType: *const iso20_ac_SignatureMethodType,
) -> i32 {
    let mut grammar_id: i32 = 21 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            21 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = exi_basetypes_encoder_uint_16(
                        stream,
                        ((*SignatureMethodType).Algorithm.charactersLen as i32 + 2 as i32) as u16,
                    );
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_characters(
                            stream,
                            (*SignatureMethodType).Algorithm.charactersLen as usize,
                            ((*SignatureMethodType).Algorithm.characters).as_ptr(),
                            (64 as i32 + 1 as i32) as usize,
                        );
                        if error == 0 as i32 {
                            grammar_id = 22 as i32;
                        }
                    }
                }
            }
            22 => {
                if (*SignatureMethodType).HMACOutputLength_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_signed(
                                stream,
                                &(*SignatureMethodType).HMACOutputLength,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 23 as i32;
                                }
                            }
                        }
                    }
                } else if (*SignatureMethodType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*SignatureMethodType).ANY.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*SignatureMethodType).ANY.bytesLen as usize,
                                    ((*SignatureMethodType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 2 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            23 => {
                if (*SignatureMethodType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*SignatureMethodType).ANY.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*SignatureMethodType).ANY.bytesLen as usize,
                                    ((*SignatureMethodType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 2 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_KeyValueType(
    stream: &mut ExiBitstream,
    mut KeyValueType: *const iso20_ac_KeyValueType,
) -> i32 {
    let mut grammar_id: i32 = 24 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            24 => {
                if (*KeyValueType).DSAKeyValue_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_ac_DSAKeyValueType(stream, &(*KeyValueType).DSAKeyValue);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyValueType).RSAKeyValue_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_ac_RSAKeyValueType(stream, &(*KeyValueType).RSAKeyValue);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyValueType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error =
                                exi_basetypes_encoder_uint_16(stream, (*KeyValueType).ANY.bytesLen);
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*KeyValueType).ANY.bytesLen as usize,
                                    ((*KeyValueType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 2 as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_ReferenceType(
    stream: &mut ExiBitstream,
    mut ReferenceType: *const iso20_ac_ReferenceType,
) -> i32 {
    let mut grammar_id: i32 = 25 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            25 => {
                if (*ReferenceType).Id_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*ReferenceType).Id.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*ReferenceType).Id.charactersLen as usize,
                                ((*ReferenceType).Id.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 26 as i32;
                            }
                        }
                    }
                } else if (*ReferenceType).Type_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*ReferenceType).Type.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*ReferenceType).Type.charactersLen as usize,
                                ((*ReferenceType).Type.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 27 as i32;
                            }
                        }
                    }
                } else if (*ReferenceType).URI_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*ReferenceType).URI.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*ReferenceType).URI.charactersLen as usize,
                                ((*ReferenceType).URI.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 28 as i32;
                            }
                        }
                    }
                } else if (*ReferenceType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_ac_TransformsType(stream, &(*ReferenceType).Transforms);
                        if error == 0 as i32 {
                            grammar_id = 29 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_DigestMethodType(
                            stream,
                            &(*ReferenceType).DigestMethod,
                        );
                        if error == 0 as i32 {
                            grammar_id = 30 as i32;
                        }
                    }
                }
            }
            26 => {
                if (*ReferenceType).Type_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*ReferenceType).Type.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*ReferenceType).Type.charactersLen as usize,
                                ((*ReferenceType).Type.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 27 as i32;
                            }
                        }
                    }
                } else if (*ReferenceType).URI_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*ReferenceType).URI.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*ReferenceType).URI.charactersLen as usize,
                                ((*ReferenceType).URI.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 28 as i32;
                            }
                        }
                    }
                } else if (*ReferenceType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_ac_TransformsType(stream, &(*ReferenceType).Transforms);
                        if error == 0 as i32 {
                            grammar_id = 29 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_DigestMethodType(
                            stream,
                            &(*ReferenceType).DigestMethod,
                        );
                        if error == 0 as i32 {
                            grammar_id = 30 as i32;
                        }
                    }
                }
            }
            27 => {
                if (*ReferenceType).URI_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*ReferenceType).URI.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*ReferenceType).URI.charactersLen as usize,
                                ((*ReferenceType).URI.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 28 as i32;
                            }
                        }
                    }
                } else if (*ReferenceType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_ac_TransformsType(stream, &(*ReferenceType).Transforms);
                        if error == 0 as i32 {
                            grammar_id = 29 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_DigestMethodType(
                            stream,
                            &(*ReferenceType).DigestMethod,
                        );
                        if error == 0 as i32 {
                            grammar_id = 30 as i32;
                        }
                    }
                }
            }
            28 => {
                if (*ReferenceType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_ac_TransformsType(stream, &(*ReferenceType).Transforms);
                        if error == 0 as i32 {
                            grammar_id = 29 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_DigestMethodType(
                            stream,
                            &(*ReferenceType).DigestMethod,
                        );
                        if error == 0 as i32 {
                            grammar_id = 30 as i32;
                        }
                    }
                }
            }
            29 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error =
                        encode_iso20_ac_DigestMethodType(stream, &(*ReferenceType).DigestMethod);
                    if error == 0 as i32 {
                        grammar_id = 30 as i32;
                    }
                }
            }
            30 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*ReferenceType).DigestValue.bytesLen,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*ReferenceType).DigestValue.bytesLen as usize,
                                ((*ReferenceType).DigestValue.bytes).as_ptr(),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 2 as i32;
                                }
                            }
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_RetrievalMethodType(
    stream: &mut ExiBitstream,
    mut RetrievalMethodType: *const iso20_ac_RetrievalMethodType,
) -> i32 {
    let mut grammar_id: i32 = 31 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            31 => {
                if (*RetrievalMethodType).Type_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*RetrievalMethodType).Type.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*RetrievalMethodType).Type.charactersLen as usize,
                                ((*RetrievalMethodType).Type.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 32 as i32;
                            }
                        }
                    }
                } else if (*RetrievalMethodType).URI_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*RetrievalMethodType).URI.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*RetrievalMethodType).URI.charactersLen as usize,
                                ((*RetrievalMethodType).URI.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 33 as i32;
                            }
                        }
                    }
                } else if (*RetrievalMethodType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_TransformsType(
                            stream,
                            &(*RetrievalMethodType).Transforms,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            32 => {
                if (*RetrievalMethodType).URI_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*RetrievalMethodType).URI.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*RetrievalMethodType).URI.charactersLen as usize,
                                ((*RetrievalMethodType).URI.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 33 as i32;
                            }
                        }
                    }
                } else if (*RetrievalMethodType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_TransformsType(
                            stream,
                            &(*RetrievalMethodType).Transforms,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            33 => {
                if (*RetrievalMethodType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_TransformsType(
                            stream,
                            &(*RetrievalMethodType).Transforms,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_X509DataType(
    stream: &mut ExiBitstream,
    mut X509DataType: *const iso20_ac_X509DataType,
) -> i32 {
    let mut grammar_id: i32 = 34 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            34 => {
                if (*X509DataType).X509IssuerSerial_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_X509IssuerSerialType(
                            stream,
                            &(*X509DataType).X509IssuerSerial,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*X509DataType).X509SKI_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*X509DataType).X509SKI.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*X509DataType).X509SKI.bytesLen as usize,
                                    ((*X509DataType).X509SKI.bytes).as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 2 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*X509DataType).X509SubjectName_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                ((*X509DataType).X509SubjectName.charactersLen as i32 + 2 as i32)
                                    as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*X509DataType).X509SubjectName.charactersLen as usize,
                                    ((*X509DataType).X509SubjectName.characters).as_ptr(),
                                    (64 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 2 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*X509DataType).X509Certificate_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*X509DataType).X509Certificate.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*X509DataType).X509Certificate.bytesLen as usize,
                                    ((*X509DataType).X509Certificate.bytes).as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 2 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*X509DataType).X509CRL_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*X509DataType).X509CRL.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*X509DataType).X509CRL.bytesLen as usize,
                                    ((*X509DataType).X509CRL.bytes).as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 2 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*X509DataType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error =
                                exi_basetypes_encoder_uint_16(stream, (*X509DataType).ANY.bytesLen);
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*X509DataType).ANY.bytesLen as usize,
                                    ((*X509DataType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 2 as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_PGPDataType(
    stream: &mut ExiBitstream,
    mut PGPDataType: *const iso20_ac_PGPDataType,
) -> i32 {
    let mut grammar_id: i32 = 35 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            35 => {
                if ((*PGPDataType).c2rust_unnamed).choice_1_isUsed == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*PGPDataType).c2rust_unnamed.choice_1.PGPKeyID.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*PGPDataType).c2rust_unnamed.choice_1.PGPKeyID.bytesLen
                                        as usize,
                                    ((*PGPDataType).c2rust_unnamed.choice_1.PGPKeyID.bytes)
                                        .as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 36 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*PGPDataType).c2rust_unnamed.choice_1.PGPKeyPacket.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*PGPDataType).c2rust_unnamed.choice_1.PGPKeyPacket.bytesLen
                                        as usize,
                                    ((*PGPDataType).c2rust_unnamed.choice_1.PGPKeyPacket.bytes)
                                        .as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 37 as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            36 => {
                if ((*PGPDataType).c2rust_unnamed).choice_1_isUsed == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*PGPDataType).c2rust_unnamed.choice_1.PGPKeyPacket.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*PGPDataType).c2rust_unnamed.choice_1.PGPKeyPacket.bytesLen
                                        as usize,
                                    ((*PGPDataType).c2rust_unnamed.choice_1.PGPKeyPacket.bytes)
                                        .as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 37 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if ((*PGPDataType).c2rust_unnamed).choice_1_isUsed == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*PGPDataType).c2rust_unnamed.choice_1.ANY.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*PGPDataType).c2rust_unnamed.choice_1.ANY.bytesLen as usize,
                                    ((*PGPDataType).c2rust_unnamed.choice_1.ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 38 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            37 => {
                if ((*PGPDataType).c2rust_unnamed).choice_1_isUsed == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*PGPDataType).c2rust_unnamed.choice_1.ANY.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*PGPDataType).c2rust_unnamed.choice_1.ANY.bytesLen as usize,
                                    ((*PGPDataType).c2rust_unnamed.choice_1.ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 38 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            38 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*PGPDataType).c2rust_unnamed.choice_2.PGPKeyPacket.bytesLen,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*PGPDataType).c2rust_unnamed.choice_2.PGPKeyPacket.bytesLen
                                    as usize,
                                ((*PGPDataType).c2rust_unnamed.choice_2.PGPKeyPacket.bytes)
                                    .as_ptr(),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 39 as i32;
                                }
                            }
                        }
                    }
                }
            }
            39 => {
                if ((*PGPDataType).c2rust_unnamed).choice_2_isUsed == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*PGPDataType).c2rust_unnamed.choice_2.ANY.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*PGPDataType).c2rust_unnamed.choice_2.ANY.bytesLen as usize,
                                    ((*PGPDataType).c2rust_unnamed.choice_2.ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 38 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_SPKIDataType(
    stream: &mut ExiBitstream,
    mut SPKIDataType: *const iso20_ac_SPKIDataType,
) -> i32 {
    let mut grammar_id: i32 = 40 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            40 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*SPKIDataType).SPKISexp.bytesLen,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*SPKIDataType).SPKISexp.bytesLen as usize,
                                ((*SPKIDataType).SPKISexp.bytes).as_ptr(),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 41 as i32;
                                }
                            }
                        }
                    }
                }
            }
            41 => {
                if (*SPKIDataType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error =
                                exi_basetypes_encoder_uint_16(stream, (*SPKIDataType).ANY.bytesLen);
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*SPKIDataType).ANY.bytesLen as usize,
                                    ((*SPKIDataType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 2 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_SignedInfoType(
    stream: &mut ExiBitstream,
    mut SignedInfoType: *const iso20_ac_SignedInfoType,
) -> i32 {
    let mut grammar_id: i32 = 42 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut Reference_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            42 => {
                if (*SignedInfoType).Id_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*SignedInfoType).Id.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*SignedInfoType).Id.charactersLen as usize,
                                ((*SignedInfoType).Id.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 43 as i32;
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_CanonicalizationMethodType(
                            stream,
                            &(*SignedInfoType).CanonicalizationMethod,
                        );
                        if error == 0 as i32 {
                            grammar_id = 44 as i32;
                        }
                    }
                }
            }
            43 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_CanonicalizationMethodType(
                        stream,
                        &(*SignedInfoType).CanonicalizationMethod,
                    );
                    if error == 0 as i32 {
                        grammar_id = 44 as i32;
                    }
                }
            }
            44 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_SignatureMethodType(
                        stream,
                        &(*SignedInfoType).SignatureMethod,
                    );
                    if error == 0 as i32 {
                        grammar_id = 45 as i32;
                    }
                }
            }
            45 => {
                if (Reference_currentIndex as i32) < (*SignedInfoType).Reference.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                    if error == 0 as i32 {
                        let fresh0 = Reference_currentIndex;
                        Reference_currentIndex = Reference_currentIndex.wrapping_add(1);
                        error = encode_iso20_ac_ReferenceType(
                            stream,
                            &*((*SignedInfoType).Reference.array)
                                .as_ptr()
                                .offset(fresh0 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 46 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            46 => {
                if (Reference_currentIndex as i32) < (*SignedInfoType).Reference.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh1 = Reference_currentIndex;
                        Reference_currentIndex = Reference_currentIndex.wrapping_add(1);
                        error = encode_iso20_ac_ReferenceType(
                            stream,
                            &*((*SignedInfoType).Reference.array)
                                .as_ptr()
                                .offset(fresh1 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 46 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_SignatureValueType(
    stream: &mut ExiBitstream,
    mut SignatureValueType: *const iso20_ac_SignatureValueType,
) -> i32 {
    let mut grammar_id: i32 = 47 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            47 => {
                if (*SignatureValueType).Id_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*SignatureValueType).Id.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*SignatureValueType).Id.charactersLen as usize,
                                ((*SignatureValueType).Id.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 48 as i32;
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*SignatureValueType).CONTENT.bytesLen,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*SignatureValueType).CONTENT.bytesLen as usize,
                                ((*SignatureValueType).CONTENT.bytes).as_ptr(),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 2 as i32;
                            }
                        }
                    }
                }
            }
            48 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = exi_basetypes_encoder_uint_16(
                        stream,
                        (*SignatureValueType).CONTENT.bytesLen,
                    );
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bytes(
                            stream,
                            (*SignatureValueType).CONTENT.bytesLen as usize,
                            ((*SignatureValueType).CONTENT.bytes).as_ptr(),
                            350 as i32 as usize,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_KeyInfoType(
    stream: &mut ExiBitstream,
    mut KeyInfoType: *const iso20_ac_KeyInfoType,
) -> i32 {
    let mut grammar_id: i32 = 49 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            49 => {
                if (*KeyInfoType).Id_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*KeyInfoType).Id.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*KeyInfoType).Id.charactersLen as usize,
                                ((*KeyInfoType).Id.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 50 as i32;
                            }
                        }
                    }
                } else if (*KeyInfoType).KeyName_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                ((*KeyInfoType).KeyName.charactersLen as i32 + 2 as i32) as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*KeyInfoType).KeyName.charactersLen as usize,
                                    ((*KeyInfoType).KeyName.characters).as_ptr(),
                                    (64 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 2 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*KeyInfoType).KeyValue_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_KeyValueType(stream, &(*KeyInfoType).KeyValue);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).RetrievalMethod_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RetrievalMethodType(
                            stream,
                            &(*KeyInfoType).RetrievalMethod,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).X509Data_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_X509DataType(stream, &(*KeyInfoType).X509Data);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).PGPData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_PGPDataType(stream, &(*KeyInfoType).PGPData);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).SPKIData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_SPKIDataType(stream, &(*KeyInfoType).SPKIData);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).MgmtData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                ((*KeyInfoType).MgmtData.charactersLen as i32 + 2 as i32) as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*KeyInfoType).MgmtData.charactersLen as usize,
                                    ((*KeyInfoType).MgmtData.characters).as_ptr(),
                                    (64 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 2 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*KeyInfoType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 8 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error =
                                exi_basetypes_encoder_uint_16(stream, (*KeyInfoType).ANY.bytesLen);
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*KeyInfoType).ANY.bytesLen as usize,
                                    ((*KeyInfoType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 2 as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            50 => {
                if (*KeyInfoType).KeyName_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                ((*KeyInfoType).KeyName.charactersLen as i32 + 2 as i32) as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*KeyInfoType).KeyName.charactersLen as usize,
                                    ((*KeyInfoType).KeyName.characters).as_ptr(),
                                    (64 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 2 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*KeyInfoType).KeyValue_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_KeyValueType(stream, &(*KeyInfoType).KeyValue);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).RetrievalMethod_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RetrievalMethodType(
                            stream,
                            &(*KeyInfoType).RetrievalMethod,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).X509Data_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_X509DataType(stream, &(*KeyInfoType).X509Data);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).PGPData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_PGPDataType(stream, &(*KeyInfoType).PGPData);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).SPKIData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_SPKIDataType(stream, &(*KeyInfoType).SPKIData);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).MgmtData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                ((*KeyInfoType).MgmtData.charactersLen as i32 + 2 as i32) as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*KeyInfoType).MgmtData.charactersLen as usize,
                                    ((*KeyInfoType).MgmtData.characters).as_ptr(),
                                    (64 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 2 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*KeyInfoType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error =
                                exi_basetypes_encoder_uint_16(stream, (*KeyInfoType).ANY.bytesLen);
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*KeyInfoType).ANY.bytesLen as usize,
                                    ((*KeyInfoType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 2 as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_ObjectType(
    stream: &mut ExiBitstream,
    mut ObjectType: *const iso20_ac_ObjectType,
) -> i32 {
    let mut grammar_id: i32 = 51 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            51 => {
                if (*ObjectType).Encoding_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*ObjectType).Encoding.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*ObjectType).Encoding.charactersLen as usize,
                                ((*ObjectType).Encoding.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 52 as i32;
                            }
                        }
                    }
                } else if (*ObjectType).Id_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*ObjectType).Id.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*ObjectType).Id.charactersLen as usize,
                                ((*ObjectType).Id.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 53 as i32;
                            }
                        }
                    }
                } else if (*ObjectType).MimeType_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*ObjectType).MimeType.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*ObjectType).MimeType.charactersLen as usize,
                                ((*ObjectType).MimeType.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 54 as i32;
                            }
                        }
                    }
                } else if (*ObjectType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error =
                                exi_basetypes_encoder_uint_16(stream, (*ObjectType).ANY.bytesLen);
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*ObjectType).ANY.bytesLen as usize,
                                    ((*ObjectType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 2 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            52 => {
                if (*ObjectType).Id_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*ObjectType).Id.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*ObjectType).Id.charactersLen as usize,
                                ((*ObjectType).Id.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 53 as i32;
                            }
                        }
                    }
                } else if (*ObjectType).MimeType_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*ObjectType).MimeType.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*ObjectType).MimeType.charactersLen as usize,
                                ((*ObjectType).MimeType.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 54 as i32;
                            }
                        }
                    }
                } else if (*ObjectType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error =
                                exi_basetypes_encoder_uint_16(stream, (*ObjectType).ANY.bytesLen);
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*ObjectType).ANY.bytesLen as usize,
                                    ((*ObjectType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 2 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            53 => {
                if (*ObjectType).MimeType_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*ObjectType).MimeType.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*ObjectType).MimeType.charactersLen as usize,
                                ((*ObjectType).MimeType.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 54 as i32;
                            }
                        }
                    }
                } else if (*ObjectType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error =
                                exi_basetypes_encoder_uint_16(stream, (*ObjectType).ANY.bytesLen);
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*ObjectType).ANY.bytesLen as usize,
                                    ((*ObjectType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 2 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            54 => {
                if (*ObjectType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error =
                                exi_basetypes_encoder_uint_16(stream, (*ObjectType).ANY.bytesLen);
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*ObjectType).ANY.bytesLen as usize,
                                    ((*ObjectType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 2 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_RationalNumberType(
    stream: &mut ExiBitstream,
    mut RationalNumberType: *const iso20_ac_RationalNumberType,
) -> i32 {
    let mut grammar_id: i32 = 55 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            55 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            8 as i32 as usize,
                            ((*RationalNumberType).Exponent as i32 + -(128 as i32)) as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 56 as i32;
                            }
                        }
                    }
                }
            }
            56 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                    if error == 0 as i32 {
                        error =
                            exi_basetypes_encoder_integer_16(stream, (*RationalNumberType).Value);
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 2 as i32;
                            }
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_DetailedCostType(
    stream: &mut ExiBitstream,
    mut DetailedCostType: *const iso20_ac_DetailedCostType,
) -> i32 {
    let mut grammar_id: i32 = 57 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            57 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(stream, &(*DetailedCostType).Amount);
                    if error == 0 as i32 {
                        grammar_id = 58 as i32;
                    }
                }
            }
            58 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*DetailedCostType).CostPerUnit,
                    );
                    if error == 0 as i32 {
                        grammar_id = 2 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_SignatureType(
    stream: &mut ExiBitstream,
    mut SignatureType: *const iso20_ac_SignatureType,
) -> i32 {
    let mut grammar_id: i32 = 59 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            59 => {
                if (*SignatureType).Id_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*SignatureType).Id.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*SignatureType).Id.charactersLen as usize,
                                ((*SignatureType).Id.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 60 as i32;
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_ac_SignedInfoType(stream, &(*SignatureType).SignedInfo);
                        if error == 0 as i32 {
                            grammar_id = 61 as i32;
                        }
                    }
                }
            }
            60 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_SignedInfoType(stream, &(*SignatureType).SignedInfo);
                    if error == 0 as i32 {
                        grammar_id = 61 as i32;
                    }
                }
            }
            61 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_SignatureValueType(
                        stream,
                        &(*SignatureType).SignatureValue,
                    );
                    if error == 0 as i32 {
                        grammar_id = 62 as i32;
                    }
                }
            }
            62 => {
                if (*SignatureType).KeyInfo_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_KeyInfoType(stream, &(*SignatureType).KeyInfo);
                        if error == 0 as i32 {
                            grammar_id = 64 as i32;
                        }
                    }
                } else if (*SignatureType).Object_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_ObjectType(stream, &(*SignatureType).Object);
                        if error == 0 as i32 {
                            grammar_id = 63 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            63 => {
                if 1 as i32 == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_ObjectType(stream, &(*SignatureType).Object);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            64 => {
                if (*SignatureType).Object_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_ObjectType(stream, &(*SignatureType).Object);
                        if error == 0 as i32 {
                            grammar_id = 65 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            65 => {
                if 1 as i32 == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_ObjectType(stream, &(*SignatureType).Object);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_DetailedTaxType(
    stream: &mut ExiBitstream,
    mut DetailedTaxType: *const iso20_ac_DetailedTaxType,
) -> i32 {
    let mut grammar_id: i32 = 66 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            66 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_32(stream, (*DetailedTaxType).TaxRuleID);
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 67 as i32;
                            }
                        }
                    }
                }
            }
            67 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(stream, &(*DetailedTaxType).Amount);
                    if error == 0 as i32 {
                        grammar_id = 2 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_MessageHeaderType(
    stream: &mut ExiBitstream,
    mut MessageHeaderType: *const iso20_ac_MessageHeaderType,
) -> i32 {
    let mut grammar_id: i32 = 68 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            68 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*MessageHeaderType).SessionID.bytesLen,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*MessageHeaderType).SessionID.bytesLen as usize,
                                ((*MessageHeaderType).SessionID.bytes).as_ptr(),
                                8 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 69 as i32;
                                }
                            }
                        }
                    }
                }
            }
            69 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                    if error == 0 as i32 {
                        error =
                            exi_basetypes_encoder_uint_64(stream, (*MessageHeaderType).TimeStamp);
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 70 as i32;
                            }
                        }
                    }
                }
            }
            70 => {
                if (*MessageHeaderType).Signature_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_ac_SignatureType(stream, &(*MessageHeaderType).Signature);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_SignaturePropertyType(
    stream: &mut ExiBitstream,
    mut SignaturePropertyType: *const iso20_ac_SignaturePropertyType,
) -> i32 {
    let mut grammar_id: i32 = 71 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            71 => {
                if (*SignaturePropertyType).Id_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*SignaturePropertyType).Id.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*SignaturePropertyType).Id.charactersLen as usize,
                                ((*SignaturePropertyType).Id.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 72 as i32;
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*SignaturePropertyType).Target.charactersLen as i32 + 2 as i32)
                                as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*SignaturePropertyType).Target.charactersLen as usize,
                                ((*SignaturePropertyType).Target.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 73 as i32;
                            }
                        }
                    }
                }
            }
            72 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = exi_basetypes_encoder_uint_16(
                        stream,
                        ((*SignaturePropertyType).Target.charactersLen as i32 + 2 as i32) as u16,
                    );
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_characters(
                            stream,
                            (*SignaturePropertyType).Target.charactersLen as usize,
                            ((*SignaturePropertyType).Target.characters).as_ptr(),
                            (64 as i32 + 1 as i32) as usize,
                        );
                        if error == 0 as i32 {
                            grammar_id = 73 as i32;
                        }
                    }
                }
            }
            73 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*SignaturePropertyType).ANY.bytesLen,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*SignaturePropertyType).ANY.bytesLen as usize,
                                ((*SignaturePropertyType).ANY.bytes).as_ptr(),
                                4 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 2 as i32;
                                }
                            }
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_AC_CPDReqEnergyTransferModeType(
    stream: &mut ExiBitstream,
    mut AC_CPDReqEnergyTransferModeType: *const iso20_ac_AC_CPDReqEnergyTransferModeType,
) -> i32 {
    let mut grammar_id: i32 = 74 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            74 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*AC_CPDReqEnergyTransferModeType).EVMaximumChargePower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 75 as i32;
                    }
                }
            }
            75 => {
                if (*AC_CPDReqEnergyTransferModeType).EVMaximumChargePower_L2_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDReqEnergyTransferModeType).EVMaximumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 76 as i32;
                        }
                    }
                } else if (*AC_CPDReqEnergyTransferModeType).EVMaximumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDReqEnergyTransferModeType).EVMaximumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 77 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDReqEnergyTransferModeType).EVMinimumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 78 as i32;
                        }
                    }
                }
            }
            76 => {
                if (*AC_CPDReqEnergyTransferModeType).EVMaximumChargePower_L3_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDReqEnergyTransferModeType).EVMaximumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 77 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDReqEnergyTransferModeType).EVMinimumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 78 as i32;
                        }
                    }
                }
            }
            77 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*AC_CPDReqEnergyTransferModeType).EVMinimumChargePower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 78 as i32;
                    }
                }
            }
            78 => {
                if (*AC_CPDReqEnergyTransferModeType).EVMinimumChargePower_L2_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDReqEnergyTransferModeType).EVMinimumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 79 as i32;
                        }
                    }
                } else if (*AC_CPDReqEnergyTransferModeType).EVMinimumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDReqEnergyTransferModeType).EVMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            79 => {
                if (*AC_CPDReqEnergyTransferModeType).EVMinimumChargePower_L3_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDReqEnergyTransferModeType).EVMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_DisplayParametersType(
    stream: &mut ExiBitstream,
    mut DisplayParametersType: *const iso20_ac_DisplayParametersType,
) -> i32 {
    let mut grammar_id: i32 = 80 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            80 => {
                if (*DisplayParametersType).PresentSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                7 as i32 as usize,
                                (*DisplayParametersType).PresentSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 81 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).MinimumSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                7 as i32 as usize,
                                (*DisplayParametersType).MinimumSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 82 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).TargetSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                7 as i32 as usize,
                                (*DisplayParametersType).TargetSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 83 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).MaximumSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                7 as i32 as usize,
                                (*DisplayParametersType).MaximumSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 84 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).RemainingTimeToMinimumSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_32(
                                stream,
                                (*DisplayParametersType).RemainingTimeToMinimumSOC,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 85 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).RemainingTimeToTargetSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_32(
                                stream,
                                (*DisplayParametersType).RemainingTimeToTargetSOC,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 86 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).RemainingTimeToMaximumSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_32(
                                stream,
                                (*DisplayParametersType).RemainingTimeToMaximumSOC,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 87 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).ChargingComplete_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*DisplayParametersType).ChargingComplete,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 88 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).BatteryEnergyCapacity_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 8 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*DisplayParametersType).BatteryEnergyCapacity,
                        );
                        if error == 0 as i32 {
                            grammar_id = 89 as i32;
                        }
                    }
                } else if (*DisplayParametersType).InletHot_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 9 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*DisplayParametersType).InletHot,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 2 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        4 as i32 as usize,
                        10 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            81 => {
                if (*DisplayParametersType).MinimumSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                7 as i32 as usize,
                                (*DisplayParametersType).MinimumSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 82 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).TargetSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                7 as i32 as usize,
                                (*DisplayParametersType).TargetSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 83 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).MaximumSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                7 as i32 as usize,
                                (*DisplayParametersType).MaximumSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 84 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).RemainingTimeToMinimumSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_32(
                                stream,
                                (*DisplayParametersType).RemainingTimeToMinimumSOC,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 85 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).RemainingTimeToTargetSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_32(
                                stream,
                                (*DisplayParametersType).RemainingTimeToTargetSOC,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 86 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).RemainingTimeToMaximumSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_32(
                                stream,
                                (*DisplayParametersType).RemainingTimeToMaximumSOC,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 87 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).ChargingComplete_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*DisplayParametersType).ChargingComplete,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 88 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).BatteryEnergyCapacity_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*DisplayParametersType).BatteryEnergyCapacity,
                        );
                        if error == 0 as i32 {
                            grammar_id = 89 as i32;
                        }
                    }
                } else if (*DisplayParametersType).InletHot_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 8 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*DisplayParametersType).InletHot,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 2 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 9 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            82 => {
                if (*DisplayParametersType).TargetSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                7 as i32 as usize,
                                (*DisplayParametersType).TargetSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 83 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).MaximumSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                7 as i32 as usize,
                                (*DisplayParametersType).MaximumSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 84 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).RemainingTimeToMinimumSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_32(
                                stream,
                                (*DisplayParametersType).RemainingTimeToMinimumSOC,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 85 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).RemainingTimeToTargetSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_32(
                                stream,
                                (*DisplayParametersType).RemainingTimeToTargetSOC,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 86 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).RemainingTimeToMaximumSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_32(
                                stream,
                                (*DisplayParametersType).RemainingTimeToMaximumSOC,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 87 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).ChargingComplete_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*DisplayParametersType).ChargingComplete,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 88 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).BatteryEnergyCapacity_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*DisplayParametersType).BatteryEnergyCapacity,
                        );
                        if error == 0 as i32 {
                            grammar_id = 89 as i32;
                        }
                    }
                } else if (*DisplayParametersType).InletHot_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*DisplayParametersType).InletHot,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 2 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 8 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            83 => {
                if (*DisplayParametersType).MaximumSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                7 as i32 as usize,
                                (*DisplayParametersType).MaximumSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 84 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).RemainingTimeToMinimumSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_32(
                                stream,
                                (*DisplayParametersType).RemainingTimeToMinimumSOC,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 85 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).RemainingTimeToTargetSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_32(
                                stream,
                                (*DisplayParametersType).RemainingTimeToTargetSOC,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 86 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).RemainingTimeToMaximumSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_32(
                                stream,
                                (*DisplayParametersType).RemainingTimeToMaximumSOC,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 87 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).ChargingComplete_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*DisplayParametersType).ChargingComplete,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 88 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).BatteryEnergyCapacity_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*DisplayParametersType).BatteryEnergyCapacity,
                        );
                        if error == 0 as i32 {
                            grammar_id = 89 as i32;
                        }
                    }
                } else if (*DisplayParametersType).InletHot_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*DisplayParametersType).InletHot,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 2 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            84 => {
                if (*DisplayParametersType).RemainingTimeToMinimumSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_32(
                                stream,
                                (*DisplayParametersType).RemainingTimeToMinimumSOC,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 85 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).RemainingTimeToTargetSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_32(
                                stream,
                                (*DisplayParametersType).RemainingTimeToTargetSOC,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 86 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).RemainingTimeToMaximumSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_32(
                                stream,
                                (*DisplayParametersType).RemainingTimeToMaximumSOC,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 87 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).ChargingComplete_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*DisplayParametersType).ChargingComplete,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 88 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).BatteryEnergyCapacity_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*DisplayParametersType).BatteryEnergyCapacity,
                        );
                        if error == 0 as i32 {
                            grammar_id = 89 as i32;
                        }
                    }
                } else if (*DisplayParametersType).InletHot_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*DisplayParametersType).InletHot,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 2 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            85 => {
                if (*DisplayParametersType).RemainingTimeToTargetSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_32(
                                stream,
                                (*DisplayParametersType).RemainingTimeToTargetSOC,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 86 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).RemainingTimeToMaximumSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_32(
                                stream,
                                (*DisplayParametersType).RemainingTimeToMaximumSOC,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 87 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).ChargingComplete_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*DisplayParametersType).ChargingComplete,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 88 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).BatteryEnergyCapacity_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*DisplayParametersType).BatteryEnergyCapacity,
                        );
                        if error == 0 as i32 {
                            grammar_id = 89 as i32;
                        }
                    }
                } else if (*DisplayParametersType).InletHot_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*DisplayParametersType).InletHot,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 2 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            86 => {
                if (*DisplayParametersType).RemainingTimeToMaximumSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_32(
                                stream,
                                (*DisplayParametersType).RemainingTimeToMaximumSOC,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 87 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).ChargingComplete_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*DisplayParametersType).ChargingComplete,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 88 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).BatteryEnergyCapacity_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*DisplayParametersType).BatteryEnergyCapacity,
                        );
                        if error == 0 as i32 {
                            grammar_id = 89 as i32;
                        }
                    }
                } else if (*DisplayParametersType).InletHot_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*DisplayParametersType).InletHot,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 2 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            87 => {
                if (*DisplayParametersType).ChargingComplete_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*DisplayParametersType).ChargingComplete,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 88 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).BatteryEnergyCapacity_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*DisplayParametersType).BatteryEnergyCapacity,
                        );
                        if error == 0 as i32 {
                            grammar_id = 89 as i32;
                        }
                    }
                } else if (*DisplayParametersType).InletHot_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*DisplayParametersType).InletHot,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 2 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            88 => {
                if (*DisplayParametersType).BatteryEnergyCapacity_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*DisplayParametersType).BatteryEnergyCapacity,
                        );
                        if error == 0 as i32 {
                            grammar_id = 89 as i32;
                        }
                    }
                } else if (*DisplayParametersType).InletHot_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*DisplayParametersType).InletHot,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 2 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            89 => {
                if (*DisplayParametersType).InletHot_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*DisplayParametersType).InletHot,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 2 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_AC_CPDResEnergyTransferModeType(
    stream: &mut ExiBitstream,
    mut AC_CPDResEnergyTransferModeType: *const iso20_ac_AC_CPDResEnergyTransferModeType,
) -> i32 {
    let mut grammar_id: i32 = 90 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            90 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*AC_CPDResEnergyTransferModeType).EVSEMaximumChargePower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 91 as i32;
                    }
                }
            }
            91 => {
                if (*AC_CPDResEnergyTransferModeType).EVSEMaximumChargePower_L2_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDResEnergyTransferModeType).EVSEMaximumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 92 as i32;
                        }
                    }
                } else if (*AC_CPDResEnergyTransferModeType).EVSEMaximumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDResEnergyTransferModeType).EVSEMaximumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 93 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDResEnergyTransferModeType).EVSEMinimumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 94 as i32;
                        }
                    }
                }
            }
            92 => {
                if (*AC_CPDResEnergyTransferModeType).EVSEMaximumChargePower_L3_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDResEnergyTransferModeType).EVSEMaximumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 93 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDResEnergyTransferModeType).EVSEMinimumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 94 as i32;
                        }
                    }
                }
            }
            93 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*AC_CPDResEnergyTransferModeType).EVSEMinimumChargePower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 94 as i32;
                    }
                }
            }
            94 => {
                if (*AC_CPDResEnergyTransferModeType).EVSEMinimumChargePower_L2_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDResEnergyTransferModeType).EVSEMinimumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 95 as i32;
                        }
                    }
                } else if (*AC_CPDResEnergyTransferModeType).EVSEMinimumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDResEnergyTransferModeType).EVSEMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 96 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDResEnergyTransferModeType).EVSENominalFrequency,
                        );
                        if error == 0 as i32 {
                            grammar_id = 97 as i32;
                        }
                    }
                }
            }
            95 => {
                if (*AC_CPDResEnergyTransferModeType).EVSEMinimumChargePower_L3_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDResEnergyTransferModeType).EVSEMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 96 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDResEnergyTransferModeType).EVSENominalFrequency,
                        );
                        if error == 0 as i32 {
                            grammar_id = 97 as i32;
                        }
                    }
                }
            }
            96 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*AC_CPDResEnergyTransferModeType).EVSENominalFrequency,
                    );
                    if error == 0 as i32 {
                        grammar_id = 97 as i32;
                    }
                }
            }
            97 => {
                if (*AC_CPDResEnergyTransferModeType).MaximumPowerAsymmetry_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDResEnergyTransferModeType).MaximumPowerAsymmetry,
                        );
                        if error == 0 as i32 {
                            grammar_id = 98 as i32;
                        }
                    }
                } else if (*AC_CPDResEnergyTransferModeType).EVSEPowerRampLimitation_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDResEnergyTransferModeType).EVSEPowerRampLimitation,
                        );
                        if error == 0 as i32 {
                            grammar_id = 99 as i32;
                        }
                    }
                } else if (*AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDResEnergyTransferModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 100 as i32;
                        }
                    }
                } else if (*AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 101 as i32;
                        }
                    }
                } else if (*AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            98 => {
                if (*AC_CPDResEnergyTransferModeType).EVSEPowerRampLimitation_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDResEnergyTransferModeType).EVSEPowerRampLimitation,
                        );
                        if error == 0 as i32 {
                            grammar_id = 99 as i32;
                        }
                    }
                } else if (*AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDResEnergyTransferModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 100 as i32;
                        }
                    }
                } else if (*AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 101 as i32;
                        }
                    }
                } else if (*AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            99 => {
                if (*AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDResEnergyTransferModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 100 as i32;
                        }
                    }
                } else if (*AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 101 as i32;
                        }
                    }
                } else if (*AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            100 => {
                if (*AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L2_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 101 as i32;
                        }
                    }
                } else if (*AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            101 => {
                if (*AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L3_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_EVSEStatusType(
    stream: &mut ExiBitstream,
    mut EVSEStatusType: *const iso20_ac_EVSEStatusType,
) -> i32 {
    let mut grammar_id: i32 = 102 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            102 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*EVSEStatusType).NotificationMaxDelay,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 103 as i32;
                            }
                        }
                    }
                }
            }
            103 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            3 as i32 as usize,
                            (*EVSEStatusType).EVSENotification as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 2 as i32;
                            }
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_Dynamic_AC_CLReqControlModeType(
    stream: &mut ExiBitstream,
    mut Dynamic_AC_CLReqControlModeType: *const iso20_ac_Dynamic_AC_CLReqControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 104 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            104 => {
                if (*Dynamic_AC_CLReqControlModeType).DepartureTime_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_32(
                                stream,
                                (*Dynamic_AC_CLReqControlModeType).DepartureTime,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 105 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLReqControlModeType).EVTargetEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 106 as i32;
                        }
                    }
                }
            }
            105 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*Dynamic_AC_CLReqControlModeType).EVTargetEnergyRequest,
                    );
                    if error == 0 as i32 {
                        grammar_id = 106 as i32;
                    }
                }
            }
            106 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*Dynamic_AC_CLReqControlModeType).EVMaximumEnergyRequest,
                    );
                    if error == 0 as i32 {
                        grammar_id = 107 as i32;
                    }
                }
            }
            107 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*Dynamic_AC_CLReqControlModeType).EVMinimumEnergyRequest,
                    );
                    if error == 0 as i32 {
                        grammar_id = 108 as i32;
                    }
                }
            }
            108 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*Dynamic_AC_CLReqControlModeType).EVMaximumChargePower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 109 as i32;
                    }
                }
            }
            109 => {
                if (*Dynamic_AC_CLReqControlModeType).EVMaximumChargePower_L2_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLReqControlModeType).EVMaximumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 110 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLReqControlModeType).EVMaximumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLReqControlModeType).EVMaximumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 111 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLReqControlModeType).EVMinimumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 112 as i32;
                        }
                    }
                }
            }
            110 => {
                if (*Dynamic_AC_CLReqControlModeType).EVMaximumChargePower_L3_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLReqControlModeType).EVMaximumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 111 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLReqControlModeType).EVMinimumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 112 as i32;
                        }
                    }
                }
            }
            111 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*Dynamic_AC_CLReqControlModeType).EVMinimumChargePower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 112 as i32;
                    }
                }
            }
            112 => {
                if (*Dynamic_AC_CLReqControlModeType).EVMinimumChargePower_L2_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLReqControlModeType).EVMinimumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 113 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLReqControlModeType).EVMinimumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLReqControlModeType).EVMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 114 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLReqControlModeType).EVPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 115 as i32;
                        }
                    }
                }
            }
            113 => {
                if (*Dynamic_AC_CLReqControlModeType).EVMinimumChargePower_L3_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLReqControlModeType).EVMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 114 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLReqControlModeType).EVPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 115 as i32;
                        }
                    }
                }
            }
            114 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*Dynamic_AC_CLReqControlModeType).EVPresentActivePower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 115 as i32;
                    }
                }
            }
            115 => {
                if (*Dynamic_AC_CLReqControlModeType).EVPresentActivePower_L2_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLReqControlModeType).EVPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 116 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLReqControlModeType).EVPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLReqControlModeType).EVPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 117 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLReqControlModeType).EVPresentReactivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 118 as i32;
                        }
                    }
                }
            }
            116 => {
                if (*Dynamic_AC_CLReqControlModeType).EVPresentActivePower_L3_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLReqControlModeType).EVPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 117 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLReqControlModeType).EVPresentReactivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 118 as i32;
                        }
                    }
                }
            }
            117 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*Dynamic_AC_CLReqControlModeType).EVPresentReactivePower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 118 as i32;
                    }
                }
            }
            118 => {
                if (*Dynamic_AC_CLReqControlModeType).EVPresentReactivePower_L2_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLReqControlModeType).EVPresentReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 119 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLReqControlModeType).EVPresentReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLReqControlModeType).EVPresentReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            119 => {
                if (*Dynamic_AC_CLReqControlModeType).EVPresentReactivePower_L3_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLReqControlModeType).EVPresentReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_Scheduled_AC_CLReqControlModeType(
    stream: &mut ExiBitstream,
    mut Scheduled_AC_CLReqControlModeType: *const iso20_ac_Scheduled_AC_CLReqControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 120 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            120 => {
                if (*Scheduled_AC_CLReqControlModeType).EVTargetEnergyRequest_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVTargetEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 121 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMaximumEnergyRequest_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMaximumEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 122 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMinimumEnergyRequest_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMinimumEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 123 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 124 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 125 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 126 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 127 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 128 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 8 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 129 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 9 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 130 as i32;
                        }
                    }
                }
            }
            121 => {
                if (*Scheduled_AC_CLReqControlModeType).EVMaximumEnergyRequest_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMaximumEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 122 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMinimumEnergyRequest_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMinimumEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 123 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 124 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 125 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 126 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 127 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 128 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 129 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 8 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 130 as i32;
                        }
                    }
                }
            }
            122 => {
                if (*Scheduled_AC_CLReqControlModeType).EVMinimumEnergyRequest_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMinimumEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 123 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 124 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 125 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 126 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 127 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 128 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 129 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 130 as i32;
                        }
                    }
                }
            }
            123 => {
                if (*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 124 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 125 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 126 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 127 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 128 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 129 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 130 as i32;
                        }
                    }
                }
            }
            124 => {
                if (*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L2_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 125 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 126 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 127 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 128 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 129 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 130 as i32;
                        }
                    }
                }
            }
            125 => {
                if (*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L3_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 126 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 127 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 128 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 129 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 130 as i32;
                        }
                    }
                }
            }
            126 => {
                if (*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 127 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 128 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 129 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 130 as i32;
                        }
                    }
                }
            }
            127 => {
                if (*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 128 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 129 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 130 as i32;
                        }
                    }
                }
            }
            128 => {
                if (*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 129 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 130 as i32;
                        }
                    }
                }
            }
            129 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*Scheduled_AC_CLReqControlModeType).EVPresentActivePower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 130 as i32;
                    }
                }
            }
            130 => {
                if (*Scheduled_AC_CLReqControlModeType).EVPresentActivePower_L2_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 131 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 132 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVPresentReactivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 133 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 134 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            131 => {
                if (*Scheduled_AC_CLReqControlModeType).EVPresentActivePower_L3_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 132 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVPresentReactivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 133 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 134 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            132 => {
                if (*Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVPresentReactivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 133 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 134 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            133 => {
                if (*Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 134 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            134 => {
                if (*Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_CLReqControlModeType(
    stream: &mut ExiBitstream,
    mut CLReqControlModeType: *const iso20_ac_CLReqControlModeType,
) -> i32 {
    let mut error: i32 =
        exi_basetypes_encoder_nbit_uint(stream, 1, 0);
    return error;
}
unsafe extern "C" fn encode_iso20_ac_MeterInfoType(
    stream: &mut ExiBitstream,
    mut MeterInfoType: *const iso20_ac_MeterInfoType,
) -> i32 {
    let mut grammar_id: i32 = 135 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            135 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*MeterInfoType).MeterID.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*MeterInfoType).MeterID.charactersLen as usize,
                                ((*MeterInfoType).MeterID.characters).as_ptr(),
                                (32 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 136 as i32;
                                }
                            }
                        }
                    }
                }
            }
            136 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_64(
                            stream,
                            (*MeterInfoType).ChargedEnergyReadingWh,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 137 as i32;
                            }
                        }
                    }
                }
            }
            137 => {
                if (*MeterInfoType).BPT_DischargedEnergyReadingWh_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_64(
                                stream,
                                (*MeterInfoType).BPT_DischargedEnergyReadingWh,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 138 as i32;
                                }
                            }
                        }
                    }
                } else if (*MeterInfoType).CapacitiveEnergyReadingVARh_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_64(
                                stream,
                                (*MeterInfoType).CapacitiveEnergyReadingVARh,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 139 as i32;
                                }
                            }
                        }
                    }
                } else if (*MeterInfoType).BPT_InductiveEnergyReadingVARh_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_64(
                                stream,
                                (*MeterInfoType).BPT_InductiveEnergyReadingVARh,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 140 as i32;
                                }
                            }
                        }
                    }
                } else if (*MeterInfoType).MeterSignature_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*MeterInfoType).MeterSignature.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*MeterInfoType).MeterSignature.bytesLen as usize,
                                    ((*MeterInfoType).MeterSignature.bytes).as_ptr(),
                                    64 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 141 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*MeterInfoType).MeterStatus_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_integer_16(
                                stream,
                                (*MeterInfoType).MeterStatus,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 142 as i32;
                                }
                            }
                        }
                    }
                } else if (*MeterInfoType).MeterTimestamp_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_64(
                                stream,
                                (*MeterInfoType).MeterTimestamp,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 2 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            138 => {
                if (*MeterInfoType).CapacitiveEnergyReadingVARh_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_64(
                                stream,
                                (*MeterInfoType).CapacitiveEnergyReadingVARh,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 139 as i32;
                                }
                            }
                        }
                    }
                } else if (*MeterInfoType).BPT_InductiveEnergyReadingVARh_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_64(
                                stream,
                                (*MeterInfoType).BPT_InductiveEnergyReadingVARh,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 140 as i32;
                                }
                            }
                        }
                    }
                } else if (*MeterInfoType).MeterSignature_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*MeterInfoType).MeterSignature.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*MeterInfoType).MeterSignature.bytesLen as usize,
                                    ((*MeterInfoType).MeterSignature.bytes).as_ptr(),
                                    64 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 141 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*MeterInfoType).MeterStatus_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_integer_16(
                                stream,
                                (*MeterInfoType).MeterStatus,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 142 as i32;
                                }
                            }
                        }
                    }
                } else if (*MeterInfoType).MeterTimestamp_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_64(
                                stream,
                                (*MeterInfoType).MeterTimestamp,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 2 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            139 => {
                if (*MeterInfoType).BPT_InductiveEnergyReadingVARh_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_64(
                                stream,
                                (*MeterInfoType).BPT_InductiveEnergyReadingVARh,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 140 as i32;
                                }
                            }
                        }
                    }
                } else if (*MeterInfoType).MeterSignature_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*MeterInfoType).MeterSignature.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*MeterInfoType).MeterSignature.bytesLen as usize,
                                    ((*MeterInfoType).MeterSignature.bytes).as_ptr(),
                                    64 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 141 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*MeterInfoType).MeterStatus_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_integer_16(
                                stream,
                                (*MeterInfoType).MeterStatus,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 142 as i32;
                                }
                            }
                        }
                    }
                } else if (*MeterInfoType).MeterTimestamp_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_64(
                                stream,
                                (*MeterInfoType).MeterTimestamp,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 2 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            140 => {
                if (*MeterInfoType).MeterSignature_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*MeterInfoType).MeterSignature.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*MeterInfoType).MeterSignature.bytesLen as usize,
                                    ((*MeterInfoType).MeterSignature.bytes).as_ptr(),
                                    64 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 141 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*MeterInfoType).MeterStatus_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_integer_16(
                                stream,
                                (*MeterInfoType).MeterStatus,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 142 as i32;
                                }
                            }
                        }
                    }
                } else if (*MeterInfoType).MeterTimestamp_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_64(
                                stream,
                                (*MeterInfoType).MeterTimestamp,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 2 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            141 => {
                if (*MeterInfoType).MeterStatus_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_integer_16(
                                stream,
                                (*MeterInfoType).MeterStatus,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 142 as i32;
                                }
                            }
                        }
                    }
                } else if (*MeterInfoType).MeterTimestamp_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_64(
                                stream,
                                (*MeterInfoType).MeterTimestamp,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 2 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            142 => {
                if (*MeterInfoType).MeterTimestamp_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_64(
                                stream,
                                (*MeterInfoType).MeterTimestamp,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 2 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_ReceiptType(
    stream: &mut ExiBitstream,
    mut ReceiptType: *const iso20_ac_ReceiptType,
) -> i32 {
    let mut grammar_id: i32 = 143 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut TaxCosts_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            143 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_64(stream, (*ReceiptType).TimeAnchor);
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 144 as i32;
                            }
                        }
                    }
                }
            }
            144 => {
                if (*ReceiptType).EnergyCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_ac_DetailedCostType(stream, &(*ReceiptType).EnergyCosts);
                        if error == 0 as i32 {
                            grammar_id = 146 as i32;
                        }
                    }
                } else if (*ReceiptType).OccupancyCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_DetailedCostType(
                            stream,
                            &(*ReceiptType).OccupancyCosts,
                        );
                        if error == 0 as i32 {
                            grammar_id = 148 as i32;
                        }
                    }
                } else if (*ReceiptType).AdditionalServicesCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_DetailedCostType(
                            stream,
                            &(*ReceiptType).AdditionalServicesCosts,
                        );
                        if error == 0 as i32 {
                            grammar_id = 150 as i32;
                        }
                    }
                } else if (*ReceiptType).OverstayCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_ac_DetailedCostType(stream, &(*ReceiptType).OverstayCosts);
                        if error == 0 as i32 {
                            grammar_id = 152 as i32;
                        }
                    }
                } else if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh2 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_ac_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh2 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 145 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            145 => {
                if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh3 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_ac_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh3 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 145 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            146 => {
                if (*ReceiptType).OccupancyCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_DetailedCostType(
                            stream,
                            &(*ReceiptType).OccupancyCosts,
                        );
                        if error == 0 as i32 {
                            grammar_id = 148 as i32;
                        }
                    }
                } else if (*ReceiptType).AdditionalServicesCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_DetailedCostType(
                            stream,
                            &(*ReceiptType).AdditionalServicesCosts,
                        );
                        if error == 0 as i32 {
                            grammar_id = 150 as i32;
                        }
                    }
                } else if (*ReceiptType).OverstayCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_ac_DetailedCostType(stream, &(*ReceiptType).OverstayCosts);
                        if error == 0 as i32 {
                            grammar_id = 152 as i32;
                        }
                    }
                } else if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh4 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_ac_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh4 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 147 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            147 => {
                if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh5 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_ac_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh5 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 147 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            148 => {
                if (*ReceiptType).AdditionalServicesCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_DetailedCostType(
                            stream,
                            &(*ReceiptType).AdditionalServicesCosts,
                        );
                        if error == 0 as i32 {
                            grammar_id = 150 as i32;
                        }
                    }
                } else if (*ReceiptType).OverstayCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_ac_DetailedCostType(stream, &(*ReceiptType).OverstayCosts);
                        if error == 0 as i32 {
                            grammar_id = 152 as i32;
                        }
                    }
                } else if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh6 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_ac_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh6 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 149 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            149 => {
                if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh7 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_ac_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh7 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 149 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            150 => {
                if (*ReceiptType).OverstayCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_ac_DetailedCostType(stream, &(*ReceiptType).OverstayCosts);
                        if error == 0 as i32 {
                            grammar_id = 152 as i32;
                        }
                    }
                } else if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh8 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_ac_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh8 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 151 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            151 => {
                if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh9 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_ac_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh9 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 151 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            152 => {
                if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh10 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_ac_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh10 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 153 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            153 => {
                if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh11 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_ac_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh11 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 153 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_Scheduled_AC_CLResControlModeType(
    stream: &mut ExiBitstream,
    mut Scheduled_AC_CLResControlModeType: *const iso20_ac_Scheduled_AC_CLResControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 154 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            154 => {
                if (*Scheduled_AC_CLResControlModeType).EVSETargetActivePower_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSETargetActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 155 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSETargetActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSETargetActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 156 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSETargetActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSETargetActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 157 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 158 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 159 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 160 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 161 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 162 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 8 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 9 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            155 => {
                if (*Scheduled_AC_CLResControlModeType).EVSETargetActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSETargetActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 156 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSETargetActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSETargetActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 157 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 158 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 159 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 160 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 161 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 162 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 8 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            156 => {
                if (*Scheduled_AC_CLResControlModeType).EVSETargetActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSETargetActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 157 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 158 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 159 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 160 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 161 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 162 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            157 => {
                if (*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 158 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 159 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 160 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 161 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 162 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            158 => {
                if (*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 159 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 160 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 161 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 162 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            159 => {
                if (*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 160 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 161 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 162 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            160 => {
                if (*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 161 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 162 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            161 => {
                if (*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 162 as i32;
                        }
                    }
                } else if (*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            162 => {
                if (*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_Dynamic_AC_CLResControlModeType(
    stream: &mut ExiBitstream,
    mut Dynamic_AC_CLResControlModeType: *const iso20_ac_Dynamic_AC_CLResControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 163 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            163 => {
                if (*Dynamic_AC_CLResControlModeType).DepartureTime_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_32(
                                stream,
                                (*Dynamic_AC_CLResControlModeType).DepartureTime,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 164 as i32;
                                }
                            }
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).MinimumSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                7 as i32 as usize,
                                (*Dynamic_AC_CLResControlModeType).MinimumSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 165 as i32;
                                }
                            }
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).TargetSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                7 as i32 as usize,
                                (*Dynamic_AC_CLResControlModeType).TargetSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 166 as i32;
                                }
                            }
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).AckMaxDelay_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*Dynamic_AC_CLResControlModeType).AckMaxDelay,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 167 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSETargetActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 168 as i32;
                        }
                    }
                }
            }
            164 => {
                if (*Dynamic_AC_CLResControlModeType).MinimumSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                7 as i32 as usize,
                                (*Dynamic_AC_CLResControlModeType).MinimumSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 165 as i32;
                                }
                            }
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).TargetSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                7 as i32 as usize,
                                (*Dynamic_AC_CLResControlModeType).TargetSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 166 as i32;
                                }
                            }
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).AckMaxDelay_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*Dynamic_AC_CLResControlModeType).AckMaxDelay,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 167 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSETargetActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 168 as i32;
                        }
                    }
                }
            }
            165 => {
                if (*Dynamic_AC_CLResControlModeType).TargetSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                7 as i32 as usize,
                                (*Dynamic_AC_CLResControlModeType).TargetSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 166 as i32;
                                }
                            }
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).AckMaxDelay_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*Dynamic_AC_CLResControlModeType).AckMaxDelay,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 167 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSETargetActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 168 as i32;
                        }
                    }
                }
            }
            166 => {
                if (*Dynamic_AC_CLResControlModeType).AckMaxDelay_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*Dynamic_AC_CLResControlModeType).AckMaxDelay,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 167 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSETargetActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 168 as i32;
                        }
                    }
                }
            }
            167 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*Dynamic_AC_CLResControlModeType).EVSETargetActivePower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 168 as i32;
                    }
                }
            }
            168 => {
                if (*Dynamic_AC_CLResControlModeType).EVSETargetActivePower_L2_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSETargetActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 169 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSETargetActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSETargetActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 170 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSETargetReactivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 171 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 172 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 173 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 174 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 175 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 8 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            169 => {
                if (*Dynamic_AC_CLResControlModeType).EVSETargetActivePower_L3_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSETargetActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 170 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSETargetReactivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 171 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 172 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 173 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 174 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 175 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            170 => {
                if (*Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSETargetReactivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 171 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 172 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 173 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 174 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 175 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            171 => {
                if (*Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 172 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 173 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 174 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 175 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            172 => {
                if (*Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 173 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 174 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 175 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            173 => {
                if (*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 174 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 175 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            174 => {
                if (*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 175 as i32;
                        }
                    }
                } else if (*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            175 => {
                if (*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_CLResControlModeType(
    stream: &mut ExiBitstream,
    mut CLResControlModeType: *const iso20_ac_CLResControlModeType,
) -> i32 {
    let mut error: i32 =
        exi_basetypes_encoder_nbit_uint(stream, 1, 0);
    return error;
}
unsafe extern "C" fn encode_iso20_ac_BPT_AC_CPDReqEnergyTransferModeType(
    stream: &mut ExiBitstream,
    mut BPT_AC_CPDReqEnergyTransferModeType: *const iso20_ac_BPT_AC_CPDReqEnergyTransferModeType,
) -> i32 {
    let mut grammar_id: i32 = 176 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            176 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*BPT_AC_CPDReqEnergyTransferModeType).EVMaximumChargePower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 177 as i32;
                    }
                }
            }
            177 => {
                if (*BPT_AC_CPDReqEnergyTransferModeType).EVMaximumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDReqEnergyTransferModeType).EVMaximumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 178 as i32;
                        }
                    }
                } else if (*BPT_AC_CPDReqEnergyTransferModeType).EVMaximumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDReqEnergyTransferModeType).EVMaximumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 179 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDReqEnergyTransferModeType).EVMinimumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 180 as i32;
                        }
                    }
                }
            }
            178 => {
                if (*BPT_AC_CPDReqEnergyTransferModeType).EVMaximumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDReqEnergyTransferModeType).EVMaximumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 179 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDReqEnergyTransferModeType).EVMinimumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 180 as i32;
                        }
                    }
                }
            }
            179 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*BPT_AC_CPDReqEnergyTransferModeType).EVMinimumChargePower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 180 as i32;
                    }
                }
            }
            180 => {
                if (*BPT_AC_CPDReqEnergyTransferModeType).EVMinimumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDReqEnergyTransferModeType).EVMinimumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 181 as i32;
                        }
                    }
                } else if (*BPT_AC_CPDReqEnergyTransferModeType).EVMinimumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDReqEnergyTransferModeType).EVMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 182 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDReqEnergyTransferModeType).EVMaximumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 183 as i32;
                        }
                    }
                }
            }
            181 => {
                if (*BPT_AC_CPDReqEnergyTransferModeType).EVMinimumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDReqEnergyTransferModeType).EVMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 182 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDReqEnergyTransferModeType).EVMaximumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 183 as i32;
                        }
                    }
                }
            }
            182 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*BPT_AC_CPDReqEnergyTransferModeType).EVMaximumDischargePower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 183 as i32;
                    }
                }
            }
            183 => {
                if (*BPT_AC_CPDReqEnergyTransferModeType).EVMaximumDischargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDReqEnergyTransferModeType).EVMaximumDischargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 184 as i32;
                        }
                    }
                } else if (*BPT_AC_CPDReqEnergyTransferModeType).EVMaximumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDReqEnergyTransferModeType).EVMaximumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 185 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDReqEnergyTransferModeType).EVMinimumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 186 as i32;
                        }
                    }
                }
            }
            184 => {
                if (*BPT_AC_CPDReqEnergyTransferModeType).EVMaximumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDReqEnergyTransferModeType).EVMaximumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 185 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDReqEnergyTransferModeType).EVMinimumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 186 as i32;
                        }
                    }
                }
            }
            185 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*BPT_AC_CPDReqEnergyTransferModeType).EVMinimumDischargePower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 186 as i32;
                    }
                }
            }
            186 => {
                if (*BPT_AC_CPDReqEnergyTransferModeType).EVMinimumDischargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDReqEnergyTransferModeType).EVMinimumDischargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 187 as i32;
                        }
                    }
                } else if (*BPT_AC_CPDReqEnergyTransferModeType).EVMinimumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDReqEnergyTransferModeType).EVMinimumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            187 => {
                if (*BPT_AC_CPDReqEnergyTransferModeType).EVMinimumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDReqEnergyTransferModeType).EVMinimumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_AC_ChargeParameterDiscoveryReqType(
    stream: &mut ExiBitstream,
    mut AC_ChargeParameterDiscoveryReqType: *const iso20_ac_AC_ChargeParameterDiscoveryReqType,
) -> i32 {
    let mut grammar_id: i32 = 188 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            188 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_MessageHeaderType(
                        stream,
                        &(*AC_ChargeParameterDiscoveryReqType).Header,
                    );
                    if error == 0 as i32 {
                        grammar_id = 189 as i32;
                    }
                }
            }
            189 => {
                if (*AC_ChargeParameterDiscoveryReqType).AC_CPDReqEnergyTransferMode_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_AC_CPDReqEnergyTransferModeType(
                            stream,
                            &(*AC_ChargeParameterDiscoveryReqType).AC_CPDReqEnergyTransferMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_BPT_AC_CPDReqEnergyTransferModeType(
                            stream,
                            &(*AC_ChargeParameterDiscoveryReqType).BPT_AC_CPDReqEnergyTransferMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_BPT_AC_CPDResEnergyTransferModeType(
    stream: &mut ExiBitstream,
    mut BPT_AC_CPDResEnergyTransferModeType: *const iso20_ac_BPT_AC_CPDResEnergyTransferModeType,
) -> i32 {
    let mut grammar_id: i32 = 190 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            190 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*BPT_AC_CPDResEnergyTransferModeType).EVSEMaximumChargePower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 191 as i32;
                    }
                }
            }
            191 => {
                if (*BPT_AC_CPDResEnergyTransferModeType).EVSEMaximumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEMaximumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 192 as i32;
                        }
                    }
                } else if (*BPT_AC_CPDResEnergyTransferModeType).EVSEMaximumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEMaximumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 193 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEMinimumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 194 as i32;
                        }
                    }
                }
            }
            192 => {
                if (*BPT_AC_CPDResEnergyTransferModeType).EVSEMaximumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEMaximumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 193 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEMinimumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 194 as i32;
                        }
                    }
                }
            }
            193 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*BPT_AC_CPDResEnergyTransferModeType).EVSEMinimumChargePower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 194 as i32;
                    }
                }
            }
            194 => {
                if (*BPT_AC_CPDResEnergyTransferModeType).EVSEMinimumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEMinimumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 195 as i32;
                        }
                    }
                } else if (*BPT_AC_CPDResEnergyTransferModeType).EVSEMinimumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 196 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSENominalFrequency,
                        );
                        if error == 0 as i32 {
                            grammar_id = 197 as i32;
                        }
                    }
                }
            }
            195 => {
                if (*BPT_AC_CPDResEnergyTransferModeType).EVSEMinimumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 196 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSENominalFrequency,
                        );
                        if error == 0 as i32 {
                            grammar_id = 197 as i32;
                        }
                    }
                }
            }
            196 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*BPT_AC_CPDResEnergyTransferModeType).EVSENominalFrequency,
                    );
                    if error == 0 as i32 {
                        grammar_id = 197 as i32;
                    }
                }
            }
            197 => {
                if (*BPT_AC_CPDResEnergyTransferModeType).MaximumPowerAsymmetry_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).MaximumPowerAsymmetry,
                        );
                        if error == 0 as i32 {
                            grammar_id = 198 as i32;
                        }
                    }
                } else if (*BPT_AC_CPDResEnergyTransferModeType).EVSEPowerRampLimitation_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEPowerRampLimitation,
                        );
                        if error == 0 as i32 {
                            grammar_id = 199 as i32;
                        }
                    }
                } else if (*BPT_AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 200 as i32;
                        }
                    }
                } else if (*BPT_AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 201 as i32;
                        }
                    }
                } else if (*BPT_AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 202 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEMaximumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 203 as i32;
                        }
                    }
                }
            }
            198 => {
                if (*BPT_AC_CPDResEnergyTransferModeType).EVSEPowerRampLimitation_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEPowerRampLimitation,
                        );
                        if error == 0 as i32 {
                            grammar_id = 199 as i32;
                        }
                    }
                } else if (*BPT_AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 200 as i32;
                        }
                    }
                } else if (*BPT_AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 201 as i32;
                        }
                    }
                } else if (*BPT_AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 202 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEMaximumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 203 as i32;
                        }
                    }
                }
            }
            199 => {
                if (*BPT_AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 200 as i32;
                        }
                    }
                } else if (*BPT_AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 201 as i32;
                        }
                    }
                } else if (*BPT_AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 202 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEMaximumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 203 as i32;
                        }
                    }
                }
            }
            200 => {
                if (*BPT_AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 201 as i32;
                        }
                    }
                } else if (*BPT_AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 202 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEMaximumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 203 as i32;
                        }
                    }
                }
            }
            201 => {
                if (*BPT_AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 202 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEMaximumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 203 as i32;
                        }
                    }
                }
            }
            202 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*BPT_AC_CPDResEnergyTransferModeType).EVSEMaximumDischargePower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 203 as i32;
                    }
                }
            }
            203 => {
                if (*BPT_AC_CPDResEnergyTransferModeType).EVSEMaximumDischargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEMaximumDischargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 204 as i32;
                        }
                    }
                } else if (*BPT_AC_CPDResEnergyTransferModeType)
                    .EVSEMaximumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEMaximumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 205 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEMinimumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 206 as i32;
                        }
                    }
                }
            }
            204 => {
                if (*BPT_AC_CPDResEnergyTransferModeType).EVSEMaximumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEMaximumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 205 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEMinimumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 206 as i32;
                        }
                    }
                }
            }
            205 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*BPT_AC_CPDResEnergyTransferModeType).EVSEMinimumDischargePower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 206 as i32;
                    }
                }
            }
            206 => {
                if (*BPT_AC_CPDResEnergyTransferModeType).EVSEMinimumDischargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEMinimumDischargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 207 as i32;
                        }
                    }
                } else if (*BPT_AC_CPDResEnergyTransferModeType)
                    .EVSEMinimumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEMinimumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            207 => {
                if (*BPT_AC_CPDResEnergyTransferModeType).EVSEMinimumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_AC_CPDResEnergyTransferModeType).EVSEMinimumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_AC_ChargeParameterDiscoveryResType(
    stream: &mut ExiBitstream,
    mut AC_ChargeParameterDiscoveryResType: *const iso20_ac_AC_ChargeParameterDiscoveryResType,
) -> i32 {
    let mut grammar_id: i32 = 208 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            208 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_MessageHeaderType(
                        stream,
                        &(*AC_ChargeParameterDiscoveryResType).Header,
                    );
                    if error == 0 as i32 {
                        grammar_id = 209 as i32;
                    }
                }
            }
            209 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            6 as i32 as usize,
                            (*AC_ChargeParameterDiscoveryResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 210 as i32;
                            }
                        }
                    }
                }
            }
            210 => {
                if (*AC_ChargeParameterDiscoveryResType).AC_CPDResEnergyTransferMode_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_AC_CPDResEnergyTransferModeType(
                            stream,
                            &(*AC_ChargeParameterDiscoveryResType).AC_CPDResEnergyTransferMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_BPT_AC_CPDResEnergyTransferModeType(
                            stream,
                            &(*AC_ChargeParameterDiscoveryResType).BPT_AC_CPDResEnergyTransferMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_BPT_Scheduled_AC_CLReqControlModeType(
    stream: &mut ExiBitstream,
    mut BPT_Scheduled_AC_CLReqControlModeType: *const iso20_ac_BPT_Scheduled_AC_CLReqControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 211 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            211 => {
                if (*BPT_Scheduled_AC_CLReqControlModeType).EVTargetEnergyRequest_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVTargetEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 212 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumEnergyRequest_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 213 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumEnergyRequest_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 214 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 215 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 216 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 217 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 218 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 219 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 8 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 220 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 9 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 221 as i32;
                        }
                    }
                }
            }
            212 => {
                if (*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumEnergyRequest_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 213 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumEnergyRequest_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 214 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 215 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 216 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 217 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 218 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 219 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 220 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 8 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 221 as i32;
                        }
                    }
                }
            }
            213 => {
                if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumEnergyRequest_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 214 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 215 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 216 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 217 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 218 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 219 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 220 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 221 as i32;
                        }
                    }
                }
            }
            214 => {
                if (*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 215 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 216 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 217 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 218 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 219 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 220 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 221 as i32;
                        }
                    }
                }
            }
            215 => {
                if (*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 216 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 217 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 218 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 219 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 220 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 221 as i32;
                        }
                    }
                }
            }
            216 => {
                if (*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 217 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 218 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 219 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 220 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 221 as i32;
                        }
                    }
                }
            }
            217 => {
                if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 218 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 219 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 220 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 221 as i32;
                        }
                    }
                }
            }
            218 => {
                if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 219 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 220 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 221 as i32;
                        }
                    }
                }
            }
            219 => {
                if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 220 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 221 as i32;
                        }
                    }
                }
            }
            220 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*BPT_Scheduled_AC_CLReqControlModeType).EVPresentActivePower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 221 as i32;
                    }
                }
            }
            221 => {
                if (*BPT_Scheduled_AC_CLReqControlModeType).EVPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 222 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 223 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVPresentReactivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 224 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVPresentReactivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 225 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVPresentReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 226 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 227 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMaximumDischargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 228 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMaximumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 229 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 8 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 230 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMinimumDischargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 9 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 231 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMinimumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        4 as i32 as usize,
                        10 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        4 as i32 as usize,
                        11 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            222 => {
                if (*BPT_Scheduled_AC_CLReqControlModeType).EVPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 223 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVPresentReactivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 224 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVPresentReactivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 225 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVPresentReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 226 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 227 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMaximumDischargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 228 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMaximumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 229 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 230 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMinimumDischargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 8 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 231 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMinimumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 9 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        4 as i32 as usize,
                        10 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            223 => {
                if (*BPT_Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVPresentReactivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 224 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVPresentReactivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 225 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVPresentReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 226 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 227 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMaximumDischargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 228 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMaximumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 229 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 230 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMinimumDischargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 231 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMinimumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 8 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 9 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            224 => {
                if (*BPT_Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 225 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVPresentReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 226 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 227 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMaximumDischargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 228 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMaximumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 229 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 230 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMinimumDischargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 231 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMinimumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 8 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            225 => {
                if (*BPT_Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVPresentReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 226 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 227 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMaximumDischargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 228 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMaximumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 229 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 230 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMinimumDischargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 231 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMinimumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            226 => {
                if (*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 227 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMaximumDischargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 228 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMaximumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 229 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 230 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMinimumDischargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 231 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMinimumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            227 => {
                if (*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 228 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMaximumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 229 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 230 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMinimumDischargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 231 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMinimumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            228 => {
                if (*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMaximumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 229 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 230 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMinimumDischargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 231 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMinimumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            229 => {
                if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 230 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMinimumDischargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 231 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMinimumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            230 => {
                if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 231 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLReqControlModeType)
                    .EVMinimumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            231 => {
                if (*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLReqControlModeType).EVMinimumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_BPT_Scheduled_AC_CLResControlModeType(
    stream: &mut ExiBitstream,
    mut BPT_Scheduled_AC_CLResControlModeType: *const iso20_ac_BPT_Scheduled_AC_CLResControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 232 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            232 => {
                if (*BPT_Scheduled_AC_CLResControlModeType).EVSETargetActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSETargetActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 233 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType).EVSETargetActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSETargetActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 234 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType).EVSETargetActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSETargetActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 235 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSETargetReactivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 236 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType)
                    .EVSETargetReactivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 237 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType)
                    .EVSETargetReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 238 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 239 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType)
                    .EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 240 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType)
                    .EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 8 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 9 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            233 => {
                if (*BPT_Scheduled_AC_CLResControlModeType).EVSETargetActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSETargetActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 234 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType).EVSETargetActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSETargetActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 235 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSETargetReactivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 236 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType)
                    .EVSETargetReactivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 237 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType)
                    .EVSETargetReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 238 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 239 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType)
                    .EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 240 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType)
                    .EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 8 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            234 => {
                if (*BPT_Scheduled_AC_CLResControlModeType).EVSETargetActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSETargetActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 235 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSETargetReactivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 236 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType)
                    .EVSETargetReactivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 237 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType)
                    .EVSETargetReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 238 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 239 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType)
                    .EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 240 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType)
                    .EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            235 => {
                if (*BPT_Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSETargetReactivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 236 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType)
                    .EVSETargetReactivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 237 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType)
                    .EVSETargetReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 238 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 239 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType)
                    .EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 240 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType)
                    .EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            236 => {
                if (*BPT_Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 237 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType)
                    .EVSETargetReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 238 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 239 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType)
                    .EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 240 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType)
                    .EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            237 => {
                if (*BPT_Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSETargetReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 238 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 239 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType)
                    .EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 240 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType)
                    .EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            238 => {
                if (*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 239 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType)
                    .EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 240 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType)
                    .EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            239 => {
                if (*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 240 as i32;
                        }
                    }
                } else if (*BPT_Scheduled_AC_CLResControlModeType)
                    .EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            240 => {
                if (*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Scheduled_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_BPT_Dynamic_AC_CLReqControlModeType(
    stream: &mut ExiBitstream,
    mut BPT_Dynamic_AC_CLReqControlModeType: *const iso20_ac_BPT_Dynamic_AC_CLReqControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 241 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            241 => {
                if (*BPT_Dynamic_AC_CLReqControlModeType).DepartureTime_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_32(
                                stream,
                                (*BPT_Dynamic_AC_CLReqControlModeType).DepartureTime,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 242 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVTargetEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 243 as i32;
                        }
                    }
                }
            }
            242 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*BPT_Dynamic_AC_CLReqControlModeType).EVTargetEnergyRequest,
                    );
                    if error == 0 as i32 {
                        grammar_id = 243 as i32;
                    }
                }
            }
            243 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*BPT_Dynamic_AC_CLReqControlModeType).EVMaximumEnergyRequest,
                    );
                    if error == 0 as i32 {
                        grammar_id = 244 as i32;
                    }
                }
            }
            244 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*BPT_Dynamic_AC_CLReqControlModeType).EVMinimumEnergyRequest,
                    );
                    if error == 0 as i32 {
                        grammar_id = 245 as i32;
                    }
                }
            }
            245 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*BPT_Dynamic_AC_CLReqControlModeType).EVMaximumChargePower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 246 as i32;
                    }
                }
            }
            246 => {
                if (*BPT_Dynamic_AC_CLReqControlModeType).EVMaximumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVMaximumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 247 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLReqControlModeType).EVMaximumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVMaximumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 248 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVMinimumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 249 as i32;
                        }
                    }
                }
            }
            247 => {
                if (*BPT_Dynamic_AC_CLReqControlModeType).EVMaximumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVMaximumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 248 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVMinimumChargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 249 as i32;
                        }
                    }
                }
            }
            248 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*BPT_Dynamic_AC_CLReqControlModeType).EVMinimumChargePower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 249 as i32;
                    }
                }
            }
            249 => {
                if (*BPT_Dynamic_AC_CLReqControlModeType).EVMinimumChargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVMinimumChargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 250 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLReqControlModeType).EVMinimumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 251 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 252 as i32;
                        }
                    }
                }
            }
            250 => {
                if (*BPT_Dynamic_AC_CLReqControlModeType).EVMinimumChargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVMinimumChargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 251 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 252 as i32;
                        }
                    }
                }
            }
            251 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*BPT_Dynamic_AC_CLReqControlModeType).EVPresentActivePower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 252 as i32;
                    }
                }
            }
            252 => {
                if (*BPT_Dynamic_AC_CLReqControlModeType).EVPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 253 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLReqControlModeType).EVPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 254 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVPresentReactivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 255 as i32;
                        }
                    }
                }
            }
            253 => {
                if (*BPT_Dynamic_AC_CLReqControlModeType).EVPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 254 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVPresentReactivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 255 as i32;
                        }
                    }
                }
            }
            254 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*BPT_Dynamic_AC_CLReqControlModeType).EVPresentReactivePower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 255 as i32;
                    }
                }
            }
            255 => {
                if (*BPT_Dynamic_AC_CLReqControlModeType).EVPresentReactivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVPresentReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 256 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLReqControlModeType).EVPresentReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVPresentReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 257 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVMaximumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 258 as i32;
                        }
                    }
                }
            }
            256 => {
                if (*BPT_Dynamic_AC_CLReqControlModeType).EVPresentReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVPresentReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 257 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVMaximumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 258 as i32;
                        }
                    }
                }
            }
            257 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*BPT_Dynamic_AC_CLReqControlModeType).EVMaximumDischargePower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 258 as i32;
                    }
                }
            }
            258 => {
                if (*BPT_Dynamic_AC_CLReqControlModeType).EVMaximumDischargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVMaximumDischargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 259 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLReqControlModeType).EVMaximumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVMaximumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 260 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVMinimumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 261 as i32;
                        }
                    }
                }
            }
            259 => {
                if (*BPT_Dynamic_AC_CLReqControlModeType).EVMaximumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVMaximumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 260 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVMinimumDischargePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 261 as i32;
                        }
                    }
                }
            }
            260 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*BPT_Dynamic_AC_CLReqControlModeType).EVMinimumDischargePower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 261 as i32;
                    }
                }
            }
            261 => {
                if (*BPT_Dynamic_AC_CLReqControlModeType).EVMinimumDischargePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVMinimumDischargePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 262 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLReqControlModeType).EVMinimumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVMinimumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 263 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLReqControlModeType).EVMaximumV2XEnergyRequest_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVMaximumV2XEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 264 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLReqControlModeType).EVMinimumV2XEnergyRequest_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVMinimumV2XEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            262 => {
                if (*BPT_Dynamic_AC_CLReqControlModeType).EVMinimumDischargePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVMinimumDischargePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 263 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLReqControlModeType).EVMaximumV2XEnergyRequest_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVMaximumV2XEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 264 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLReqControlModeType).EVMinimumV2XEnergyRequest_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVMinimumV2XEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            263 => {
                if (*BPT_Dynamic_AC_CLReqControlModeType).EVMaximumV2XEnergyRequest_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVMaximumV2XEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 264 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLReqControlModeType).EVMinimumV2XEnergyRequest_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVMinimumV2XEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            264 => {
                if (*BPT_Dynamic_AC_CLReqControlModeType).EVMinimumV2XEnergyRequest_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLReqControlModeType).EVMinimumV2XEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_AC_ChargeLoopReqType(
    stream: &mut ExiBitstream,
    mut AC_ChargeLoopReqType: *const iso20_ac_AC_ChargeLoopReqType,
) -> i32 {
    let mut grammar_id: i32 = 265 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            265 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error =
                        encode_iso20_ac_MessageHeaderType(stream, &(*AC_ChargeLoopReqType).Header);
                    if error == 0 as i32 {
                        grammar_id = 266 as i32;
                    }
                }
            }
            266 => {
                if (*AC_ChargeLoopReqType).DisplayParameters_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_DisplayParametersType(
                            stream,
                            &(*AC_ChargeLoopReqType).DisplayParameters,
                        );
                        if error == 0 as i32 {
                            grammar_id = 267 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*AC_ChargeLoopReqType).MeterInfoRequested,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 268 as i32;
                                }
                            }
                        }
                    }
                }
            }
            267 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(
                            stream,
                            (*AC_ChargeLoopReqType).MeterInfoRequested,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 268 as i32;
                            }
                        }
                    }
                }
            }
            268 => {
                if (*AC_ChargeLoopReqType).BPT_Dynamic_AC_CLReqControlMode_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_BPT_Dynamic_AC_CLReqControlModeType(
                            stream,
                            &(*AC_ChargeLoopReqType).BPT_Dynamic_AC_CLReqControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopReqType).BPT_Scheduled_AC_CLReqControlMode_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_BPT_Scheduled_AC_CLReqControlModeType(
                            stream,
                            &(*AC_ChargeLoopReqType).BPT_Scheduled_AC_CLReqControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopReqType).CLReqControlMode_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_CLReqControlModeType(
                            stream,
                            &(*AC_ChargeLoopReqType).CLReqControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopReqType).Dynamic_AC_CLReqControlMode_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_Dynamic_AC_CLReqControlModeType(
                            stream,
                            &(*AC_ChargeLoopReqType).Dynamic_AC_CLReqControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_Scheduled_AC_CLReqControlModeType(
                            stream,
                            &(*AC_ChargeLoopReqType).Scheduled_AC_CLReqControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_BPT_Dynamic_AC_CLResControlModeType(
    stream: &mut ExiBitstream,
    mut BPT_Dynamic_AC_CLResControlModeType: *const iso20_ac_BPT_Dynamic_AC_CLResControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 269 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            269 => {
                if (*BPT_Dynamic_AC_CLResControlModeType).DepartureTime_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_32(
                                stream,
                                (*BPT_Dynamic_AC_CLResControlModeType).DepartureTime,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 270 as i32;
                                }
                            }
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).MinimumSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                7 as i32 as usize,
                                (*BPT_Dynamic_AC_CLResControlModeType).MinimumSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 271 as i32;
                                }
                            }
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).TargetSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                7 as i32 as usize,
                                (*BPT_Dynamic_AC_CLResControlModeType).TargetSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 272 as i32;
                                }
                            }
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).AckMaxDelay_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*BPT_Dynamic_AC_CLResControlModeType).AckMaxDelay,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 273 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSETargetActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 274 as i32;
                        }
                    }
                }
            }
            270 => {
                if (*BPT_Dynamic_AC_CLResControlModeType).MinimumSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                7 as i32 as usize,
                                (*BPT_Dynamic_AC_CLResControlModeType).MinimumSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 271 as i32;
                                }
                            }
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).TargetSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                7 as i32 as usize,
                                (*BPT_Dynamic_AC_CLResControlModeType).TargetSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 272 as i32;
                                }
                            }
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).AckMaxDelay_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*BPT_Dynamic_AC_CLResControlModeType).AckMaxDelay,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 273 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSETargetActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 274 as i32;
                        }
                    }
                }
            }
            271 => {
                if (*BPT_Dynamic_AC_CLResControlModeType).TargetSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                7 as i32 as usize,
                                (*BPT_Dynamic_AC_CLResControlModeType).TargetSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 272 as i32;
                                }
                            }
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).AckMaxDelay_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*BPT_Dynamic_AC_CLResControlModeType).AckMaxDelay,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 273 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSETargetActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 274 as i32;
                        }
                    }
                }
            }
            272 => {
                if (*BPT_Dynamic_AC_CLResControlModeType).AckMaxDelay_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*BPT_Dynamic_AC_CLResControlModeType).AckMaxDelay,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 273 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSETargetActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 274 as i32;
                        }
                    }
                }
            }
            273 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_RationalNumberType(
                        stream,
                        &(*BPT_Dynamic_AC_CLResControlModeType).EVSETargetActivePower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 274 as i32;
                    }
                }
            }
            274 => {
                if (*BPT_Dynamic_AC_CLResControlModeType).EVSETargetActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSETargetActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 275 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSETargetActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSETargetActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 276 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSETargetReactivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 277 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 278 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 279 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 280 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 281 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 8 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            275 => {
                if (*BPT_Dynamic_AC_CLResControlModeType).EVSETargetActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSETargetActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 276 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSETargetReactivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 277 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 278 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 279 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 280 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 281 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            276 => {
                if (*BPT_Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSETargetReactivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 277 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 278 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 279 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 280 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 281 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            277 => {
                if (*BPT_Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 278 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 279 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 280 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 281 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            278 => {
                if (*BPT_Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSETargetReactivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 279 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 280 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 281 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            279 => {
                if (*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower,
                        );
                        if error == 0 as i32 {
                            grammar_id = 280 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 281 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            280 => {
                if (*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 281 as i32;
                        }
                    }
                } else if (*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            281 => {
                if (*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*BPT_Dynamic_AC_CLResControlModeType).EVSEPresentActivePower_L3,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_AC_ChargeLoopResType(
    stream: &mut ExiBitstream,
    mut AC_ChargeLoopResType: *const iso20_ac_AC_ChargeLoopResType,
) -> i32 {
    let mut grammar_id: i32 = 282 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            282 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error =
                        encode_iso20_ac_MessageHeaderType(stream, &(*AC_ChargeLoopResType).Header);
                    if error == 0 as i32 {
                        grammar_id = 283 as i32;
                    }
                }
            }
            283 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            6 as i32 as usize,
                            (*AC_ChargeLoopResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 284 as i32;
                            }
                        }
                    }
                }
            }
            284 => {
                if (*AC_ChargeLoopResType).EVSEStatus_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_EVSEStatusType(
                            stream,
                            &(*AC_ChargeLoopResType).EVSEStatus,
                        );
                        if error == 0 as i32 {
                            grammar_id = 285 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopResType).MeterInfo_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_MeterInfoType(
                            stream,
                            &(*AC_ChargeLoopResType).MeterInfo,
                        );
                        if error == 0 as i32 {
                            grammar_id = 286 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopResType).Receipt_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_ac_ReceiptType(stream, &(*AC_ChargeLoopResType).Receipt);
                        if error == 0 as i32 {
                            grammar_id = 287 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopResType).EVSETargetFrequency_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_ChargeLoopResType).EVSETargetFrequency,
                        );
                        if error == 0 as i32 {
                            grammar_id = 288 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopResType).BPT_Dynamic_AC_CLResControlMode_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_BPT_Dynamic_AC_CLResControlModeType(
                            stream,
                            &(*AC_ChargeLoopResType).BPT_Dynamic_AC_CLResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopResType).BPT_Scheduled_AC_CLResControlMode_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_BPT_Scheduled_AC_CLResControlModeType(
                            stream,
                            &(*AC_ChargeLoopResType).BPT_Scheduled_AC_CLResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopResType).CLResControlMode_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_CLResControlModeType(
                            stream,
                            &(*AC_ChargeLoopResType).CLResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopResType).Dynamic_AC_CLResControlMode_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_Dynamic_AC_CLResControlModeType(
                            stream,
                            &(*AC_ChargeLoopResType).Dynamic_AC_CLResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 8 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_Scheduled_AC_CLResControlModeType(
                            stream,
                            &(*AC_ChargeLoopResType).Scheduled_AC_CLResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                }
            }
            285 => {
                if (*AC_ChargeLoopResType).MeterInfo_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_MeterInfoType(
                            stream,
                            &(*AC_ChargeLoopResType).MeterInfo,
                        );
                        if error == 0 as i32 {
                            grammar_id = 286 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopResType).Receipt_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_ac_ReceiptType(stream, &(*AC_ChargeLoopResType).Receipt);
                        if error == 0 as i32 {
                            grammar_id = 287 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopResType).EVSETargetFrequency_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_ChargeLoopResType).EVSETargetFrequency,
                        );
                        if error == 0 as i32 {
                            grammar_id = 288 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopResType).BPT_Dynamic_AC_CLResControlMode_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_BPT_Dynamic_AC_CLResControlModeType(
                            stream,
                            &(*AC_ChargeLoopResType).BPT_Dynamic_AC_CLResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopResType).BPT_Scheduled_AC_CLResControlMode_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_BPT_Scheduled_AC_CLResControlModeType(
                            stream,
                            &(*AC_ChargeLoopResType).BPT_Scheduled_AC_CLResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopResType).CLResControlMode_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_CLResControlModeType(
                            stream,
                            &(*AC_ChargeLoopResType).CLResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopResType).Dynamic_AC_CLResControlMode_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_Dynamic_AC_CLResControlModeType(
                            stream,
                            &(*AC_ChargeLoopResType).Dynamic_AC_CLResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_Scheduled_AC_CLResControlModeType(
                            stream,
                            &(*AC_ChargeLoopResType).Scheduled_AC_CLResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                }
            }
            286 => {
                if (*AC_ChargeLoopResType).Receipt_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_ac_ReceiptType(stream, &(*AC_ChargeLoopResType).Receipt);
                        if error == 0 as i32 {
                            grammar_id = 287 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopResType).EVSETargetFrequency_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_ChargeLoopResType).EVSETargetFrequency,
                        );
                        if error == 0 as i32 {
                            grammar_id = 288 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopResType).BPT_Dynamic_AC_CLResControlMode_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_BPT_Dynamic_AC_CLResControlModeType(
                            stream,
                            &(*AC_ChargeLoopResType).BPT_Dynamic_AC_CLResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopResType).BPT_Scheduled_AC_CLResControlMode_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_BPT_Scheduled_AC_CLResControlModeType(
                            stream,
                            &(*AC_ChargeLoopResType).BPT_Scheduled_AC_CLResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopResType).CLResControlMode_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_CLResControlModeType(
                            stream,
                            &(*AC_ChargeLoopResType).CLResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopResType).Dynamic_AC_CLResControlMode_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_Dynamic_AC_CLResControlModeType(
                            stream,
                            &(*AC_ChargeLoopResType).Dynamic_AC_CLResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_Scheduled_AC_CLResControlModeType(
                            stream,
                            &(*AC_ChargeLoopResType).Scheduled_AC_CLResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                }
            }
            287 => {
                if (*AC_ChargeLoopResType).EVSETargetFrequency_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_RationalNumberType(
                            stream,
                            &(*AC_ChargeLoopResType).EVSETargetFrequency,
                        );
                        if error == 0 as i32 {
                            grammar_id = 288 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopResType).BPT_Dynamic_AC_CLResControlMode_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_BPT_Dynamic_AC_CLResControlModeType(
                            stream,
                            &(*AC_ChargeLoopResType).BPT_Dynamic_AC_CLResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopResType).BPT_Scheduled_AC_CLResControlMode_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_BPT_Scheduled_AC_CLResControlModeType(
                            stream,
                            &(*AC_ChargeLoopResType).BPT_Scheduled_AC_CLResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopResType).CLResControlMode_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_CLResControlModeType(
                            stream,
                            &(*AC_ChargeLoopResType).CLResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopResType).Dynamic_AC_CLResControlMode_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_Dynamic_AC_CLResControlModeType(
                            stream,
                            &(*AC_ChargeLoopResType).Dynamic_AC_CLResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_Scheduled_AC_CLResControlModeType(
                            stream,
                            &(*AC_ChargeLoopResType).Scheduled_AC_CLResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                }
            }
            288 => {
                if (*AC_ChargeLoopResType).BPT_Dynamic_AC_CLResControlMode_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_BPT_Dynamic_AC_CLResControlModeType(
                            stream,
                            &(*AC_ChargeLoopResType).BPT_Dynamic_AC_CLResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopResType).BPT_Scheduled_AC_CLResControlMode_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_BPT_Scheduled_AC_CLResControlModeType(
                            stream,
                            &(*AC_ChargeLoopResType).BPT_Scheduled_AC_CLResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopResType).CLResControlMode_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_CLResControlModeType(
                            stream,
                            &(*AC_ChargeLoopResType).CLResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*AC_ChargeLoopResType).Dynamic_AC_CLResControlMode_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_Dynamic_AC_CLResControlModeType(
                            stream,
                            &(*AC_ChargeLoopResType).Dynamic_AC_CLResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_Scheduled_AC_CLResControlModeType(
                            stream,
                            &(*AC_ChargeLoopResType).Scheduled_AC_CLResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_ManifestType(
    stream: &mut ExiBitstream,
    mut ManifestType: *const iso20_ac_ManifestType,
) -> i32 {
    let mut grammar_id: i32 = 289 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut Reference_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            289 => {
                if (*ManifestType).Id_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*ManifestType).Id.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*ManifestType).Id.charactersLen as usize,
                                ((*ManifestType).Id.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 291 as i32;
                            }
                        }
                    }
                } else if (Reference_currentIndex as i32)
                    < (*ManifestType).Reference.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh12 = Reference_currentIndex;
                        Reference_currentIndex = Reference_currentIndex.wrapping_add(1);
                        error = encode_iso20_ac_ReferenceType(
                            stream,
                            &*((*ManifestType).Reference.array)
                                .as_ptr()
                                .offset(fresh12 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 290 as i32;
                        }
                    }
                }
            }
            290 => {
                if (Reference_currentIndex as i32) < (*ManifestType).Reference.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh13 = Reference_currentIndex;
                        Reference_currentIndex = Reference_currentIndex.wrapping_add(1);
                        error = encode_iso20_ac_ReferenceType(
                            stream,
                            &*((*ManifestType).Reference.array)
                                .as_ptr()
                                .offset(fresh13 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 290 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            291 => {
                if (Reference_currentIndex as i32) < (*ManifestType).Reference.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                    if error == 0 as i32 {
                        let fresh14 = Reference_currentIndex;
                        Reference_currentIndex = Reference_currentIndex.wrapping_add(1);
                        error = encode_iso20_ac_ReferenceType(
                            stream,
                            &*((*ManifestType).Reference.array)
                                .as_ptr()
                                .offset(fresh14 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 292 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            292 => {
                if (Reference_currentIndex as i32) < (*ManifestType).Reference.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh15 = Reference_currentIndex;
                        Reference_currentIndex = Reference_currentIndex.wrapping_add(1);
                        error = encode_iso20_ac_ReferenceType(
                            stream,
                            &*((*ManifestType).Reference.array)
                                .as_ptr()
                                .offset(fresh15 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 292 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
unsafe extern "C" fn encode_iso20_ac_SignaturePropertiesType(
    stream: &mut ExiBitstream,
    mut SignaturePropertiesType: *const iso20_ac_SignaturePropertiesType,
) -> i32 {
    let mut grammar_id: i32 = 293 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            293 => {
                if (*SignaturePropertiesType).Id_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*SignaturePropertiesType).Id.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*SignaturePropertiesType).Id.charactersLen as usize,
                                ((*SignaturePropertiesType).Id.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 295 as i32;
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_SignaturePropertyType(
                            stream,
                            &(*SignaturePropertiesType).SignatureProperty,
                        );
                        if error == 0 as i32 {
                            grammar_id = 294 as i32;
                        }
                    }
                }
            }
            294 => {
                if 1 as i32 == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_SignaturePropertyType(
                            stream,
                            &(*SignaturePropertiesType).SignatureProperty,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            295 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    error = encode_iso20_ac_SignaturePropertyType(
                        stream,
                        &(*SignaturePropertiesType).SignatureProperty,
                    );
                    if error == 0 as i32 {
                        grammar_id = 296 as i32;
                    }
                }
            }
            296 => {
                if 1 as i32 == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ac_SignaturePropertyType(
                            stream,
                            &(*SignaturePropertiesType).SignatureProperty,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1, 0);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 3 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}

pub unsafe extern "C" fn encode_iso20_ac_exiDocument(
    stream: &mut ExiBitstream,
    mut exiDoc: *mut iso20_ac_exiDocument,
) -> i32 {
    let mut error: i32 = exi_header_write(stream);
    if error == 0 as i32 {
        if (*exiDoc).AC_CPDReqEnergyTransferMode_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 0 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_AC_CPDReqEnergyTransferModeType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.AC_CPDReqEnergyTransferMode,
                );
            }
        } else if (*exiDoc).AC_CPDResEnergyTransferMode_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 1 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_AC_CPDResEnergyTransferModeType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.AC_CPDResEnergyTransferMode,
                );
            }
        } else if (*exiDoc).AC_ChargeLoopReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 2 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_AC_ChargeLoopReqType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.AC_ChargeLoopReq,
                );
            }
        } else if (*exiDoc).AC_ChargeLoopRes_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 3 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_AC_ChargeLoopResType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.AC_ChargeLoopRes,
                );
            }
        } else if (*exiDoc).AC_ChargeParameterDiscoveryReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 4 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_AC_ChargeParameterDiscoveryReqType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.AC_ChargeParameterDiscoveryReq,
                );
            }
        } else if (*exiDoc).AC_ChargeParameterDiscoveryRes_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 5 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_AC_ChargeParameterDiscoveryResType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.AC_ChargeParameterDiscoveryRes,
                );
            }
        } else if (*exiDoc).BPT_AC_CPDReqEnergyTransferMode_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 6 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_BPT_AC_CPDReqEnergyTransferModeType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.BPT_AC_CPDReqEnergyTransferMode,
                );
            }
        } else if (*exiDoc).BPT_AC_CPDResEnergyTransferMode_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 7 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_BPT_AC_CPDResEnergyTransferModeType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.BPT_AC_CPDResEnergyTransferMode,
                );
            }
        } else if (*exiDoc).BPT_Dynamic_AC_CLReqControlMode_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 8 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_BPT_Dynamic_AC_CLReqControlModeType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.BPT_Dynamic_AC_CLReqControlMode,
                );
            }
        } else if (*exiDoc).BPT_Dynamic_AC_CLResControlMode_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 9 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_BPT_Dynamic_AC_CLResControlModeType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.BPT_Dynamic_AC_CLResControlMode,
                );
            }
        } else if (*exiDoc).BPT_Scheduled_AC_CLReqControlMode_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 10 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_BPT_Scheduled_AC_CLReqControlModeType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.BPT_Scheduled_AC_CLReqControlMode,
                );
            }
        } else if (*exiDoc).BPT_Scheduled_AC_CLResControlMode_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 11 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_BPT_Scheduled_AC_CLResControlModeType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.BPT_Scheduled_AC_CLResControlMode,
                );
            }
        } else if (*exiDoc).CLReqControlMode_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 12 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_CLReqControlModeType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.CLReqControlMode,
                );
            }
        } else if (*exiDoc).CLResControlMode_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 13 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_CLResControlModeType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.CLResControlMode,
                );
            }
        } else if (*exiDoc).CanonicalizationMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 14 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_CanonicalizationMethodType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.CanonicalizationMethod,
                );
            }
        } else if (*exiDoc).DSAKeyValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 15 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_DSAKeyValueType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.DSAKeyValue,
                );
            }
        } else if (*exiDoc).DigestMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 16 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_DigestMethodType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.DigestMethod,
                );
            }
        } else if (*exiDoc).Dynamic_AC_CLReqControlMode_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 18 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_Dynamic_AC_CLReqControlModeType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.Dynamic_AC_CLReqControlMode,
                );
            }
        } else if (*exiDoc).Dynamic_AC_CLResControlMode_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 19 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_Dynamic_AC_CLResControlModeType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.Dynamic_AC_CLResControlMode,
                );
            }
        } else if (*exiDoc).KeyInfo_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 20 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_KeyInfoType(stream, &mut (*exiDoc).c2rust_unnamed.KeyInfo);
            }
        } else if (*exiDoc).KeyValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 22 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_ac_KeyValueType(stream, &mut (*exiDoc).c2rust_unnamed.KeyValue);
            }
        } else if (*exiDoc).Manifest_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 23 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_ac_ManifestType(stream, &mut (*exiDoc).c2rust_unnamed.Manifest);
            }
        } else if (*exiDoc).Object_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 25 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_ObjectType(stream, &mut (*exiDoc).c2rust_unnamed.Object);
            }
        } else if (*exiDoc).PGPData_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 26 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_PGPDataType(stream, &mut (*exiDoc).c2rust_unnamed.PGPData);
            }
        } else if (*exiDoc).RSAKeyValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 27 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_RSAKeyValueType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.RSAKeyValue,
                );
            }
        } else if (*exiDoc).Reference_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 28 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_ac_ReferenceType(stream, &mut (*exiDoc).c2rust_unnamed.Reference);
            }
        } else if (*exiDoc).RetrievalMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 29 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_RetrievalMethodType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.RetrievalMethod,
                );
            }
        } else if (*exiDoc).SPKIData_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 30 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_ac_SPKIDataType(stream, &mut (*exiDoc).c2rust_unnamed.SPKIData);
            }
        } else if (*exiDoc).Scheduled_AC_CLReqControlMode_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 31 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_Scheduled_AC_CLReqControlModeType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.Scheduled_AC_CLReqControlMode,
                );
            }
        } else if (*exiDoc).Scheduled_AC_CLResControlMode_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 32 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_Scheduled_AC_CLResControlModeType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.Scheduled_AC_CLResControlMode,
                );
            }
        } else if (*exiDoc).SignatureMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 33 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_SignatureMethodType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.SignatureMethod,
                );
            }
        } else if (*exiDoc).SignatureProperties_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 34 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_SignaturePropertiesType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.SignatureProperties,
                );
            }
        } else if (*exiDoc).SignatureProperty_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 35 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_SignaturePropertyType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.SignatureProperty,
                );
            }
        } else if (*exiDoc).Signature_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 36 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_ac_SignatureType(stream, &mut (*exiDoc).c2rust_unnamed.Signature);
            }
        } else if (*exiDoc).SignatureValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 37 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_SignatureValueType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.SignatureValue,
                );
            }
        } else if (*exiDoc).SignedInfo_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 38 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_SignedInfoType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.SignedInfo,
                );
            }
        } else if (*exiDoc).Transform_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 39 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_ac_TransformType(stream, &mut (*exiDoc).c2rust_unnamed.Transform);
            }
        } else if (*exiDoc).Transforms_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 40 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_TransformsType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.Transforms,
                );
            }
        } else if (*exiDoc).X509Data_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 41 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_ac_X509DataType(stream, &mut (*exiDoc).c2rust_unnamed.X509Data);
            }
        } else {
            error = -(70 as i32);
        }
    }
    return error;
}

pub unsafe extern "C" fn encode_iso20_ac_exiFragment(
    stream: &mut ExiBitstream,
    mut exiFrag: *mut iso20_ac_exiFragment,
) -> i32 {
    let mut error: i32 = exi_header_write(stream);
    if error == 0 as i32 {
        if 0 as i32 == 1 as i32 {
            error = -(299 as i32);
        } else if (*exiFrag).AC_ChargeParameterDiscoveryRes_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 8 as i32 as usize, 5 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_AC_ChargeParameterDiscoveryResType(
                    stream,
                    &mut (*exiFrag).c2rust_unnamed.AC_ChargeParameterDiscoveryRes,
                );
            }
        } else if (*exiFrag).SignedInfo_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 8 as i32 as usize, 135 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_SignedInfoType(
                    stream,
                    &mut (*exiFrag).c2rust_unnamed.SignedInfo,
                );
            }
        } else {
            error = -(70 as i32);
        }
        if error == 0 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 8 as i32 as usize, 155 as i32 as u32);
        }
    }
    return error;
}

pub unsafe extern "C" fn encode_iso20_ac_xmldsigFragment(
    stream: &mut ExiBitstream,
    mut xmldsigFrag: *mut iso20_ac_xmldsigFragment,
) -> i32 {
    let mut error: i32 = exi_header_write(stream);
    if error == 0 as i32 {
        if (*xmldsigFrag).CanonicalizationMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 0 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_CanonicalizationMethodType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.CanonicalizationMethod,
                );
            }
        } else if (*xmldsigFrag).DSAKeyValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 1 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_DSAKeyValueType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.DSAKeyValue,
                );
            }
        } else if (*xmldsigFrag).DigestMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 2 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_DigestMethodType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.DigestMethod,
                );
            }
        } else if (*xmldsigFrag).KeyInfo_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 8 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_ac_KeyInfoType(stream, &mut (*xmldsigFrag).c2rust_unnamed.KeyInfo);
            }
        } else if (*xmldsigFrag).KeyValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 10 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_KeyValueType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.KeyValue,
                );
            }
        } else if (*xmldsigFrag).Manifest_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 11 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_ManifestType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.Manifest,
                );
            }
        } else if (*xmldsigFrag).Object_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 14 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_ac_ObjectType(stream, &mut (*xmldsigFrag).c2rust_unnamed.Object);
            }
        } else if (*xmldsigFrag).PGPData_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 16 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_ac_PGPDataType(stream, &mut (*xmldsigFrag).c2rust_unnamed.PGPData);
            }
        } else if (*xmldsigFrag).RSAKeyValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 21 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_RSAKeyValueType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.RSAKeyValue,
                );
            }
        } else if (*xmldsigFrag).Reference_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 22 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_ReferenceType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.Reference,
                );
            }
        } else if (*xmldsigFrag).RetrievalMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 23 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_RetrievalMethodType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.RetrievalMethod,
                );
            }
        } else if (*xmldsigFrag).SPKIData_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 24 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_SPKIDataType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.SPKIData,
                );
            }
        } else if (*xmldsigFrag).Signature_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 27 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_SignatureType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.Signature,
                );
            }
        } else if (*xmldsigFrag).SignatureMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 28 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_SignatureMethodType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.SignatureMethod,
                );
            }
        } else if (*xmldsigFrag).SignatureProperties_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 29 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_SignaturePropertiesType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.SignatureProperties,
                );
            }
        } else if (*xmldsigFrag).SignatureProperty_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 30 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_SignaturePropertyType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.SignatureProperty,
                );
            }
        } else if (*xmldsigFrag).SignatureValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 31 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_SignatureValueType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.SignatureValue,
                );
            }
        } else if (*xmldsigFrag).SignedInfo_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 32 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_SignedInfoType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.SignedInfo,
                );
            }
        } else if (*xmldsigFrag).Transform_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 33 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_TransformType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.Transform,
                );
            }
        } else if (*xmldsigFrag).Transforms_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 34 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_TransformsType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.Transforms,
                );
            }
        } else if (*xmldsigFrag).X509Data_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 37 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_X509DataType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.X509Data,
                );
            }
        } else if (*xmldsigFrag).X509IssuerSerial_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 39 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ac_X509IssuerSerialType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.X509IssuerSerial,
                );
            }
        } else {
            error = -(70 as i32);
        }
        if error == 0 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 46 as i32 as u32);
        }
    }
    return error;
}
