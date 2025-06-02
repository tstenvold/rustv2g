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
    pub octets_count: usize,
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
pub type iso20_acdp_cpStatusType = u32;
pub const iso20_acdp_cpStatusType_StateE: iso20_acdp_cpStatusType = 4;
pub const iso20_acdp_cpStatusType_StateD: iso20_acdp_cpStatusType = 3;
pub const iso20_acdp_cpStatusType_StateC: iso20_acdp_cpStatusType = 2;
pub const iso20_acdp_cpStatusType_StateB: iso20_acdp_cpStatusType = 1;
pub const iso20_acdp_cpStatusType_StateA: iso20_acdp_cpStatusType = 0;
pub type iso20_acdp_errorCodeType = u32;
pub const iso20_acdp_errorCodeType_FAILED_Reserved2: iso20_acdp_errorCodeType = 9;
pub const iso20_acdp_errorCodeType_FAILED_Reserved1: iso20_acdp_errorCodeType = 8;
pub const iso20_acdp_errorCodeType_FAILED_ChargingVoltageOutOfRange: iso20_acdp_errorCodeType = 7;
pub const iso20_acdp_errorCodeType_FAILED_ChargingCurrentDifferential: iso20_acdp_errorCodeType = 6;
pub const iso20_acdp_errorCodeType_FAILED_RESS: iso20_acdp_errorCodeType = 5;
pub const iso20_acdp_errorCodeType_FAILED_RESSTemperatureInhibit: iso20_acdp_errorCodeType = 4;
pub const iso20_acdp_errorCodeType_FAILED_Breaker: iso20_acdp_errorCodeType = 3;
pub const iso20_acdp_errorCodeType_FAILED_EmergencyEvent: iso20_acdp_errorCodeType = 2;
pub const iso20_acdp_errorCodeType_FAILED: iso20_acdp_errorCodeType = 1;
pub const iso20_acdp_errorCodeType_OK_NoEVError: iso20_acdp_errorCodeType = 0;
pub type iso20_acdp_responseCodeType = u32;
pub const iso20_acdp_responseCodeType_FAILED_WrongChargeParameter: iso20_acdp_responseCodeType = 39;
pub const iso20_acdp_responseCodeType_FAILED_UnknownSession: iso20_acdp_responseCodeType = 38;
pub const iso20_acdp_responseCodeType_FAILED_SignatureError: iso20_acdp_responseCodeType = 37;
pub const iso20_acdp_responseCodeType_FAILED_ServiceSelectionInvalid: iso20_acdp_responseCodeType =
    36;
pub const iso20_acdp_responseCodeType_FAILED_ServiceIDInvalid: iso20_acdp_responseCodeType = 35;
pub const iso20_acdp_responseCodeType_FAILED_SequenceError: iso20_acdp_responseCodeType = 34;
pub const iso20_acdp_responseCodeType_FAILED_ScheduleSelectionInvalid: iso20_acdp_responseCodeType =
    33;
pub const iso20_acdp_responseCodeType_FAILED_ScheduleRenegotiation: iso20_acdp_responseCodeType =
    32;
pub const iso20_acdp_responseCodeType_FAILED_PowerToleranceNotConfirmed:
    iso20_acdp_responseCodeType = 31;
pub const iso20_acdp_responseCodeType_FAILED_PowerDeliveryNotApplied: iso20_acdp_responseCodeType =
    30;
pub const iso20_acdp_responseCodeType_FAILED_PauseNotAllowed: iso20_acdp_responseCodeType = 29;
pub const iso20_acdp_responseCodeType_FAILED_NoServiceRenegotiationSupported:
    iso20_acdp_responseCodeType = 28;
pub const iso20_acdp_responseCodeType_FAILED_NoEnergyTransferServiceSelected:
    iso20_acdp_responseCodeType = 27;
pub const iso20_acdp_responseCodeType_FAILED_MeteringSignatureNotValid:
    iso20_acdp_responseCodeType = 26;
pub const iso20_acdp_responseCodeType_FAILED_EVPowerProfileViolation: iso20_acdp_responseCodeType =
    25;
pub const iso20_acdp_responseCodeType_FAILED_EVPowerProfileInvalid: iso20_acdp_responseCodeType =
    24;
pub const iso20_acdp_responseCodeType_FAILED_ContactorError: iso20_acdp_responseCodeType = 23;
pub const iso20_acdp_responseCodeType_FAILED_AssociationError: iso20_acdp_responseCodeType = 22;
pub const iso20_acdp_responseCodeType_FAILED: iso20_acdp_responseCodeType = 21;
pub const iso20_acdp_responseCodeType_WARNING_WPT: iso20_acdp_responseCodeType = 20;
pub const iso20_acdp_responseCodeType_WARNING_StandbyNotAllowed: iso20_acdp_responseCodeType = 19;
pub const iso20_acdp_responseCodeType_WARNING_ScheduleRenegotiationFailed:
    iso20_acdp_responseCodeType = 18;
pub const iso20_acdp_responseCodeType_WARNING_PowerToleranceNotConfirmed:
    iso20_acdp_responseCodeType = 17;
pub const iso20_acdp_responseCodeType_WARNING_NoContractMatchingPCIDFound:
    iso20_acdp_responseCodeType = 16;
pub const iso20_acdp_responseCodeType_WARNING_NoCertificateAvailable: iso20_acdp_responseCodeType =
    15;
pub const iso20_acdp_responseCodeType_WARNING_GeneralPnCAuthorizationError:
    iso20_acdp_responseCodeType = 14;
pub const iso20_acdp_responseCodeType_WARNING_EVPowerProfileViolation: iso20_acdp_responseCodeType =
    13;
pub const iso20_acdp_responseCodeType_WARNING_eMSPUnknown: iso20_acdp_responseCodeType = 12;
pub const iso20_acdp_responseCodeType_WARNING_EIMAuthorizationFailure: iso20_acdp_responseCodeType =
    11;
pub const iso20_acdp_responseCodeType_WARNING_ChallengeInvalid: iso20_acdp_responseCodeType = 10;
pub const iso20_acdp_responseCodeType_WARNING_CertificateValidationError:
    iso20_acdp_responseCodeType = 9;
pub const iso20_acdp_responseCodeType_WARNING_CertificateRevoked: iso20_acdp_responseCodeType = 8;
pub const iso20_acdp_responseCodeType_WARNING_CertificateNotYetValid: iso20_acdp_responseCodeType =
    7;
pub const iso20_acdp_responseCodeType_WARNING_CertificateExpired: iso20_acdp_responseCodeType = 6;
pub const iso20_acdp_responseCodeType_WARNING_AuthorizationSelectionInvalid:
    iso20_acdp_responseCodeType = 5;
pub const iso20_acdp_responseCodeType_OK_PowerToleranceConfirmed: iso20_acdp_responseCodeType = 4;
pub const iso20_acdp_responseCodeType_OK_OldSessionJoined: iso20_acdp_responseCodeType = 3;
pub const iso20_acdp_responseCodeType_OK_NewSessionEstablished: iso20_acdp_responseCodeType = 2;
pub const iso20_acdp_responseCodeType_OK_CertificateExpiresSoon: iso20_acdp_responseCodeType = 1;
pub const iso20_acdp_responseCodeType_OK: iso20_acdp_responseCodeType = 0;
pub type iso20_acdp_electricalChargingDeviceStatusType = u32;
pub const iso20_acdp_electricalChargingDeviceStatusType_State_D:
    iso20_acdp_electricalChargingDeviceStatusType = 3;
pub const iso20_acdp_electricalChargingDeviceStatusType_State_C:
    iso20_acdp_electricalChargingDeviceStatusType = 2;
pub const iso20_acdp_electricalChargingDeviceStatusType_State_B:
    iso20_acdp_electricalChargingDeviceStatusType = 1;
pub const iso20_acdp_electricalChargingDeviceStatusType_State_A:
    iso20_acdp_electricalChargingDeviceStatusType = 0;
pub type iso20_acdp_processingType = u32;
pub const iso20_acdp_processingType_Ongoing_WaitingForCustomerInteraction:
    iso20_acdp_processingType = 2;
pub const iso20_acdp_processingType_Ongoing: iso20_acdp_processingType = 1;
pub const iso20_acdp_processingType_Finished: iso20_acdp_processingType = 0;
pub type iso20_acdp_mechanicalChargingDeviceStatusType = u32;
pub const iso20_acdp_mechanicalChargingDeviceStatusType_EndPosition:
    iso20_acdp_mechanicalChargingDeviceStatusType = 2;
pub const iso20_acdp_mechanicalChargingDeviceStatusType_Moving:
    iso20_acdp_mechanicalChargingDeviceStatusType = 1;
pub const iso20_acdp_mechanicalChargingDeviceStatusType_Home:
    iso20_acdp_mechanicalChargingDeviceStatusType = 0;
pub type iso20_acdp_isolationStatusType = u32;
pub const iso20_acdp_isolationStatusType_Fault: iso20_acdp_isolationStatusType = 3;
pub const iso20_acdp_isolationStatusType_Warning: iso20_acdp_isolationStatusType = 2;
pub const iso20_acdp_isolationStatusType_Safe: iso20_acdp_isolationStatusType = 1;
pub const iso20_acdp_isolationStatusType_Invalid: iso20_acdp_isolationStatusType = 0;
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_acdp_TransformType {
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

pub struct iso20_acdp_TransformsType {
    pub Transform: iso20_acdp_TransformType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_acdp_DSAKeyValueType {
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

pub struct iso20_acdp_X509IssuerSerialType {
    pub X509IssuerName: C2RustUnnamed_9,
    pub X509SerialNumber: ExiSigned,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_9 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_acdp_DigestMethodType {
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

pub struct iso20_acdp_RSAKeyValueType {
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

pub struct iso20_acdp_CanonicalizationMethodType {
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

pub struct iso20_acdp_SignatureMethodType {
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

pub struct iso20_acdp_KeyValueType {
    pub DSAKeyValue: iso20_acdp_DSAKeyValueType,
    #[bitfield(name = "DSAKeyValue_isUsed", ty = "u32", bits = "0..=0")]
    pub DSAKeyValue_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub RSAKeyValue: iso20_acdp_RSAKeyValueType,
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

pub struct iso20_acdp_ReferenceType {
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
    pub Transforms: iso20_acdp_TransformsType,
    #[bitfield(name = "Transforms_isUsed", ty = "u32", bits = "0..=0")]
    pub Transforms_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 3],
    pub DigestMethod: iso20_acdp_DigestMethodType,
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

pub struct iso20_acdp_RetrievalMethodType {
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
    pub Transforms: iso20_acdp_TransformsType,
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

pub struct iso20_acdp_X509DataType {
    pub X509IssuerSerial: iso20_acdp_X509IssuerSerialType,
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

pub struct iso20_acdp_PGPDataType {
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

pub struct iso20_acdp_SPKIDataType {
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

pub struct iso20_acdp_SignedInfoType {
    pub Id: C2RustUnnamed_41,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub CanonicalizationMethod: iso20_acdp_CanonicalizationMethodType,
    pub SignatureMethod: iso20_acdp_SignatureMethodType,
    pub Reference: C2RustUnnamed_40,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_40 {
    pub array: [iso20_acdp_ReferenceType; 4],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_41 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_acdp_SignatureValueType {
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

pub struct iso20_acdp_KeyInfoType {
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
    pub KeyValue: iso20_acdp_KeyValueType,
    #[bitfield(name = "KeyValue_isUsed", ty = "u32", bits = "0..=0")]
    pub KeyValue_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 3],
    pub RetrievalMethod: iso20_acdp_RetrievalMethodType,
    #[bitfield(name = "RetrievalMethod_isUsed", ty = "u32", bits = "0..=0")]
    pub RetrievalMethod_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 3],
    pub X509Data: iso20_acdp_X509DataType,
    #[bitfield(name = "X509Data_isUsed", ty = "u32", bits = "0..=0")]
    pub X509Data_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 3],
    pub PGPData: iso20_acdp_PGPDataType,
    #[bitfield(name = "PGPData_isUsed", ty = "u32", bits = "0..=0")]
    pub PGPData_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 3],
    pub SPKIData: iso20_acdp_SPKIDataType,
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

pub struct iso20_acdp_ObjectType {
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
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_acdp_SignatureType {
    pub Id: C2RustUnnamed_52,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub SignedInfo: iso20_acdp_SignedInfoType,
    pub SignatureValue: iso20_acdp_SignatureValueType,
    pub KeyInfo: iso20_acdp_KeyInfoType,
    #[bitfield(name = "KeyInfo_isUsed", ty = "u32", bits = "0..=0")]
    pub KeyInfo_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
    pub Object: iso20_acdp_ObjectType,
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

pub struct iso20_acdp_RationalNumberType {
    pub Exponent: int8_t,
    pub Value: i16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_acdp_MessageHeaderType {
    pub SessionID: C2RustUnnamed_53,
    pub TimeStamp: u64,
    pub Signature: iso20_acdp_SignatureType,
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

pub struct iso20_acdp_SignaturePropertyType {
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

pub struct iso20_acdp_EVTechnicalStatusType {
    pub EVReadyToCharge: i32,
    pub EVImmobilizationRequest: i32,
    pub EVImmobilized: i32,
    #[bitfield(name = "EVImmobilized_isUsed", ty = "u32", bits = "0..=0")]
    pub EVImmobilized_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub EVWLANStrength: iso20_acdp_RationalNumberType,
    #[bitfield(name = "EVWLANStrength_isUsed", ty = "u32", bits = "0..=0")]
    pub EVWLANStrength_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub EVCPStatus: iso20_acdp_cpStatusType,
    #[bitfield(name = "EVCPStatus_isUsed", ty = "u32", bits = "0..=0")]
    pub EVCPStatus_isUsed: [u8; 1],
    pub EVSOC: int8_t,
    #[bitfield(name = "EVSOC_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSOC_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub EVErrorCode: iso20_acdp_errorCodeType,
    #[bitfield(name = "EVErrorCode_isUsed", ty = "u32", bits = "0..=0")]
    pub EVErrorCode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 3],
    pub EVTimeout: i32,
    #[bitfield(name = "EVTimeout_isUsed", ty = "u32", bits = "0..=0")]
    pub EVTimeout_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 3],
}
#[derive(Copy, Clone)]

pub struct iso20_acdp_ACDP_VehiclePositioningReqType {
    pub Header: iso20_acdp_MessageHeaderType,
    pub EVMobilityStatus: i32,
    pub EVPositioningSupport: i32,
}
#[derive(Copy, Clone)]

pub struct iso20_acdp_ACDP_VehiclePositioningResType {
    pub Header: iso20_acdp_MessageHeaderType,
    pub ResponseCode: iso20_acdp_responseCodeType,
    pub EVSEProcessing: iso20_acdp_processingType,
    pub EVSEPositioningSupport: i32,
    pub EVRelativeXDeviation: i16,
    pub EVRelativeYDeviation: i16,
    pub ContactWindowXc: i16,
    pub ContactWindowYc: i16,
    pub EVInChargePosition: i32,
}
#[derive(Copy, Clone)]

pub struct iso20_acdp_ACDP_ConnectReqType {
    pub Header: iso20_acdp_MessageHeaderType,
    pub EVElectricalChargingDeviceStatus: iso20_acdp_electricalChargingDeviceStatusType,
}
#[derive(Copy, Clone)]

pub struct iso20_acdp_ACDP_ConnectResType {
    pub Header: iso20_acdp_MessageHeaderType,
    pub ResponseCode: iso20_acdp_responseCodeType,
    pub EVSEProcessing: iso20_acdp_processingType,
    pub EVSEElectricalChargingDeviceStatus: iso20_acdp_electricalChargingDeviceStatusType,
    pub EVSEMechanicalChargingDeviceStatus: iso20_acdp_mechanicalChargingDeviceStatusType,
}
#[derive(Copy, Clone)]

pub struct iso20_acdp_ACDP_SystemStatusReqType {
    pub Header: iso20_acdp_MessageHeaderType,
    pub EVTechnicalStatus: iso20_acdp_EVTechnicalStatusType,
}
#[derive(Copy, Clone)]

pub struct iso20_acdp_ACDP_SystemStatusResType {
    pub Header: iso20_acdp_MessageHeaderType,
    pub ResponseCode: iso20_acdp_responseCodeType,
    pub EVSEMechanicalChargingDeviceStatus: iso20_acdp_mechanicalChargingDeviceStatusType,
    pub EVSEReadyToCharge: i32,
    pub EVSEIsolationStatus: iso20_acdp_isolationStatusType,
    pub EVSEDisabled: i32,
    pub EVSEUtilityInterruptEvent: i32,
    pub EVSEEmergencyShutdown: i32,
    pub EVSEMalfunction: i32,
    pub EVInChargePosition: i32,
    pub EVAssociationStatus: i32,
}
#[derive(Copy, Clone)]

pub struct iso20_acdp_CLReqControlModeType {
    pub _unused: i32,
}
#[derive(Copy, Clone)]

pub struct iso20_acdp_CLResControlModeType {
    pub _unused: i32,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_acdp_ManifestType {
    pub Id: C2RustUnnamed_58,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub Reference: C2RustUnnamed_57,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_57 {
    pub array: [iso20_acdp_ReferenceType; 4],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_58 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_acdp_SignaturePropertiesType {
    pub Id: C2RustUnnamed_59,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub SignatureProperty: iso20_acdp_SignaturePropertyType,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_59 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_acdp_exiDocument {
    pub c2rust_unnamed: C2RustUnnamed_60,
    #[bitfield(name = "ACDP_VehiclePositioningReq_isUsed", ty = "u32", bits = "0..=0")]
    #[bitfield(name = "ACDP_VehiclePositioningRes_isUsed", ty = "u32", bits = "1..=1")]
    #[bitfield(name = "ACDP_ConnectReq_isUsed", ty = "u32", bits = "2..=2")]
    #[bitfield(name = "ACDP_ConnectRes_isUsed", ty = "u32", bits = "3..=3")]
    #[bitfield(name = "ACDP_DisconnectReq_isUsed", ty = "u32", bits = "4..=4")]
    #[bitfield(name = "ACDP_DisconnectRes_isUsed", ty = "u32", bits = "5..=5")]
    #[bitfield(name = "ACDP_SystemStatusReq_isUsed", ty = "u32", bits = "6..=6")]
    #[bitfield(name = "ACDP_SystemStatusRes_isUsed", ty = "u32", bits = "7..=7")]
    #[bitfield(name = "CLReqControlMode_isUsed", ty = "u32", bits = "8..=8")]
    #[bitfield(name = "CLResControlMode_isUsed", ty = "u32", bits = "9..=9")]
    #[bitfield(name = "Signature_isUsed", ty = "u32", bits = "10..=10")]
    #[bitfield(name = "SignatureValue_isUsed", ty = "u32", bits = "11..=11")]
    #[bitfield(name = "SignedInfo_isUsed", ty = "u32", bits = "12..=12")]
    #[bitfield(name = "CanonicalizationMethod_isUsed", ty = "u32", bits = "13..=13")]
    #[bitfield(name = "SignatureMethod_isUsed", ty = "u32", bits = "14..=14")]
    #[bitfield(name = "Reference_isUsed", ty = "u32", bits = "15..=15")]
    #[bitfield(name = "Transforms_isUsed", ty = "u32", bits = "16..=16")]
    #[bitfield(name = "Transform_isUsed", ty = "u32", bits = "17..=17")]
    #[bitfield(name = "DigestMethod_isUsed", ty = "u32", bits = "18..=18")]
    #[bitfield(name = "KeyInfo_isUsed", ty = "u32", bits = "19..=19")]
    #[bitfield(name = "KeyValue_isUsed", ty = "u32", bits = "20..=20")]
    #[bitfield(name = "RetrievalMethod_isUsed", ty = "u32", bits = "21..=21")]
    #[bitfield(name = "X509Data_isUsed", ty = "u32", bits = "22..=22")]
    #[bitfield(name = "PGPData_isUsed", ty = "u32", bits = "23..=23")]
    #[bitfield(name = "SPKIData_isUsed", ty = "u32", bits = "24..=24")]
    #[bitfield(name = "Object_isUsed", ty = "u32", bits = "25..=25")]
    #[bitfield(name = "Manifest_isUsed", ty = "u32", bits = "26..=26")]
    #[bitfield(name = "SignatureProperties_isUsed", ty = "u32", bits = "27..=27")]
    #[bitfield(name = "SignatureProperty_isUsed", ty = "u32", bits = "28..=28")]
    #[bitfield(name = "DSAKeyValue_isUsed", ty = "u32", bits = "29..=29")]
    #[bitfield(name = "RSAKeyValue_isUsed", ty = "u32", bits = "30..=30")]
    pub ACDP_VehiclePositioningReq_isUsed_ACDP_VehiclePositioningRes_isUsed_ACDP_ConnectReq_isUsed_ACDP_ConnectRes_isUsed_ACDP_DisconnectReq_isUsed_ACDP_DisconnectRes_isUsed_ACDP_SystemStatusReq_isUsed_ACDP_SystemStatusRes_isUsed_CLReqControlMode_isUsed_CLResControlMode_isUsed_Signature_isUsed_SignatureValue_isUsed_SignedInfo_isUsed_CanonicalizationMethod_isUsed_SignatureMethod_isUsed_Reference_isUsed_Transforms_isUsed_Transform_isUsed_DigestMethod_isUsed_KeyInfo_isUsed_KeyValue_isUsed_RetrievalMethod_isUsed_X509Data_isUsed_PGPData_isUsed_SPKIData_isUsed_Object_isUsed_Manifest_isUsed_SignatureProperties_isUsed_SignatureProperty_isUsed_DSAKeyValue_isUsed_RSAKeyValue_isUsed:
        [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(Copy, Clone)]

pub union C2RustUnnamed_60 {
    pub ACDP_VehiclePositioningReq: iso20_acdp_ACDP_VehiclePositioningReqType,
    pub ACDP_VehiclePositioningRes: iso20_acdp_ACDP_VehiclePositioningResType,
    pub ACDP_ConnectReq: iso20_acdp_ACDP_ConnectReqType,
    pub ACDP_ConnectRes: iso20_acdp_ACDP_ConnectResType,
    pub ACDP_DisconnectReq: iso20_acdp_ACDP_ConnectReqType,
    pub ACDP_DisconnectRes: iso20_acdp_ACDP_ConnectResType,
    pub ACDP_SystemStatusReq: iso20_acdp_ACDP_SystemStatusReqType,
    pub ACDP_SystemStatusRes: iso20_acdp_ACDP_SystemStatusResType,
    pub CLReqControlMode: iso20_acdp_CLReqControlModeType,
    pub CLResControlMode: iso20_acdp_CLResControlModeType,
    pub Signature: iso20_acdp_SignatureType,
    pub SignatureValue: iso20_acdp_SignatureValueType,
    pub SignedInfo: iso20_acdp_SignedInfoType,
    pub CanonicalizationMethod: iso20_acdp_CanonicalizationMethodType,
    pub SignatureMethod: iso20_acdp_SignatureMethodType,
    pub Reference: iso20_acdp_ReferenceType,
    pub Transforms: iso20_acdp_TransformsType,
    pub Transform: iso20_acdp_TransformType,
    pub DigestMethod: iso20_acdp_DigestMethodType,
    pub KeyInfo: iso20_acdp_KeyInfoType,
    pub KeyValue: iso20_acdp_KeyValueType,
    pub RetrievalMethod: iso20_acdp_RetrievalMethodType,
    pub X509Data: iso20_acdp_X509DataType,
    pub PGPData: iso20_acdp_PGPDataType,
    pub SPKIData: iso20_acdp_SPKIDataType,
    pub Object: iso20_acdp_ObjectType,
    pub Manifest: iso20_acdp_ManifestType,
    pub SignatureProperties: iso20_acdp_SignaturePropertiesType,
    pub SignatureProperty: iso20_acdp_SignaturePropertyType,
    pub DSAKeyValue: iso20_acdp_DSAKeyValueType,
    pub RSAKeyValue: iso20_acdp_RSAKeyValueType,
}
unsafe extern "C" fn encode_iso20_acdp_TransformType(
    stream: &mut ExiBitstream,
    mut TransformType: *const iso20_acdp_TransformType,
) -> i32 {
    let mut grammar_id: i32 = 0 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            0 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_TransformsType(
    stream: &mut ExiBitstream,
    mut TransformsType: *const iso20_acdp_TransformsType,
) -> i32 {
    let mut grammar_id: i32 = 4 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            4 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_acdp_TransformType(stream, &(*TransformsType).Transform);
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
                        error =
                            encode_iso20_acdp_TransformType(stream, &(*TransformsType).Transform);
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_DSAKeyValueType(
    stream: &mut ExiBitstream,
    mut DSAKeyValueType: *const iso20_acdp_DSAKeyValueType,
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_X509IssuerSerialType(
    stream: &mut ExiBitstream,
    mut X509IssuerSerialType: *const iso20_acdp_X509IssuerSerialType,
) -> i32 {
    let mut grammar_id: i32 = 13 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            13 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_DigestMethodType(
    stream: &mut ExiBitstream,
    mut DigestMethodType: *const iso20_acdp_DigestMethodType,
) -> i32 {
    let mut grammar_id: i32 = 15 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            15 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_RSAKeyValueType(
    stream: &mut ExiBitstream,
    mut RSAKeyValueType: *const iso20_acdp_RSAKeyValueType,
) -> i32 {
    let mut grammar_id: i32 = 17 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            17 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_CanonicalizationMethodType(
    stream: &mut ExiBitstream,
    mut CanonicalizationMethodType: *const iso20_acdp_CanonicalizationMethodType,
) -> i32 {
    let mut grammar_id: i32 = 19 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            19 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_SignatureMethodType(
    stream: &mut ExiBitstream,
    mut SignatureMethodType: *const iso20_acdp_SignatureMethodType,
) -> i32 {
    let mut grammar_id: i32 = 21 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            21 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_KeyValueType(
    stream: &mut ExiBitstream,
    mut KeyValueType: *const iso20_acdp_KeyValueType,
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
                            encode_iso20_acdp_DSAKeyValueType(stream, &(*KeyValueType).DSAKeyValue);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyValueType).RSAKeyValue_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_acdp_RSAKeyValueType(stream, &(*KeyValueType).RSAKeyValue);
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_ReferenceType(
    stream: &mut ExiBitstream,
    mut ReferenceType: *const iso20_acdp_ReferenceType,
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
                            encode_iso20_acdp_TransformsType(stream, &(*ReferenceType).Transforms);
                        if error == 0 as i32 {
                            grammar_id = 29 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_acdp_DigestMethodType(
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
                            encode_iso20_acdp_TransformsType(stream, &(*ReferenceType).Transforms);
                        if error == 0 as i32 {
                            grammar_id = 29 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_acdp_DigestMethodType(
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
                            encode_iso20_acdp_TransformsType(stream, &(*ReferenceType).Transforms);
                        if error == 0 as i32 {
                            grammar_id = 29 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_acdp_DigestMethodType(
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
                            encode_iso20_acdp_TransformsType(stream, &(*ReferenceType).Transforms);
                        if error == 0 as i32 {
                            grammar_id = 29 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_acdp_DigestMethodType(
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso20_acdp_DigestMethodType(stream, &(*ReferenceType).DigestMethod);
                    if error == 0 as i32 {
                        grammar_id = 30 as i32;
                    }
                }
            }
            30 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_RetrievalMethodType(
    stream: &mut ExiBitstream,
    mut RetrievalMethodType: *const iso20_acdp_RetrievalMethodType,
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
                        error = encode_iso20_acdp_TransformsType(
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
                        error = encode_iso20_acdp_TransformsType(
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
                        error = encode_iso20_acdp_TransformsType(
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_X509DataType(
    stream: &mut ExiBitstream,
    mut X509DataType: *const iso20_acdp_X509DataType,
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
                        error = encode_iso20_acdp_X509IssuerSerialType(
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_PGPDataType(
    stream: &mut ExiBitstream,
    mut PGPDataType: *const iso20_acdp_PGPDataType,
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_SPKIDataType(
    stream: &mut ExiBitstream,
    mut SPKIDataType: *const iso20_acdp_SPKIDataType,
) -> i32 {
    let mut grammar_id: i32 = 40 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            40 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_SignedInfoType(
    stream: &mut ExiBitstream,
    mut SignedInfoType: *const iso20_acdp_SignedInfoType,
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
                        error = encode_iso20_acdp_CanonicalizationMethodType(
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_acdp_CanonicalizationMethodType(
                        stream,
                        &(*SignedInfoType).CanonicalizationMethod,
                    );
                    if error == 0 as i32 {
                        grammar_id = 44 as i32;
                    }
                }
            }
            44 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_acdp_SignatureMethodType(
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
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh0 = Reference_currentIndex;
                        Reference_currentIndex = Reference_currentIndex.wrapping_add(1);
                        error = encode_iso20_acdp_ReferenceType(
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
                        error = encode_iso20_acdp_ReferenceType(
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_SignatureValueType(
    stream: &mut ExiBitstream,
    mut SignatureValueType: *const iso20_acdp_SignatureValueType,
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_KeyInfoType(
    stream: &mut ExiBitstream,
    mut KeyInfoType: *const iso20_acdp_KeyInfoType,
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
                        error = encode_iso20_acdp_KeyValueType(stream, &(*KeyInfoType).KeyValue);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).RetrievalMethod_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_acdp_RetrievalMethodType(
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
                        error = encode_iso20_acdp_X509DataType(stream, &(*KeyInfoType).X509Data);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).PGPData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_acdp_PGPDataType(stream, &(*KeyInfoType).PGPData);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).SPKIData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_acdp_SPKIDataType(stream, &(*KeyInfoType).SPKIData);
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
                        error = encode_iso20_acdp_KeyValueType(stream, &(*KeyInfoType).KeyValue);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).RetrievalMethod_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_acdp_RetrievalMethodType(
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
                        error = encode_iso20_acdp_X509DataType(stream, &(*KeyInfoType).X509Data);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).PGPData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_acdp_PGPDataType(stream, &(*KeyInfoType).PGPData);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).SPKIData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_acdp_SPKIDataType(stream, &(*KeyInfoType).SPKIData);
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_ObjectType(
    stream: &mut ExiBitstream,
    mut ObjectType: *const iso20_acdp_ObjectType,
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_SignatureType(
    stream: &mut ExiBitstream,
    mut SignatureType: *const iso20_acdp_SignatureType,
) -> i32 {
    let mut grammar_id: i32 = 55 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            55 => {
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
                                grammar_id = 56 as i32;
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_acdp_SignedInfoType(stream, &(*SignatureType).SignedInfo);
                        if error == 0 as i32 {
                            grammar_id = 57 as i32;
                        }
                    }
                }
            }
            56 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_acdp_SignedInfoType(stream, &(*SignatureType).SignedInfo);
                    if error == 0 as i32 {
                        grammar_id = 57 as i32;
                    }
                }
            }
            57 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_acdp_SignatureValueType(
                        stream,
                        &(*SignatureType).SignatureValue,
                    );
                    if error == 0 as i32 {
                        grammar_id = 58 as i32;
                    }
                }
            }
            58 => {
                if (*SignatureType).KeyInfo_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_acdp_KeyInfoType(stream, &(*SignatureType).KeyInfo);
                        if error == 0 as i32 {
                            grammar_id = 60 as i32;
                        }
                    }
                } else if (*SignatureType).Object_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_acdp_ObjectType(stream, &(*SignatureType).Object);
                        if error == 0 as i32 {
                            grammar_id = 59 as i32;
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
            59 => {
                if 1 as i32 == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_acdp_ObjectType(stream, &(*SignatureType).Object);
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
            60 => {
                if (*SignatureType).Object_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_acdp_ObjectType(stream, &(*SignatureType).Object);
                        if error == 0 as i32 {
                            grammar_id = 61 as i32;
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
            61 => {
                if 1 as i32 == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_acdp_ObjectType(stream, &(*SignatureType).Object);
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_RationalNumberType(
    stream: &mut ExiBitstream,
    mut RationalNumberType: *const iso20_acdp_RationalNumberType,
) -> i32 {
    let mut grammar_id: i32 = 62 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            62 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
                                grammar_id = 63 as i32;
                            }
                        }
                    }
                }
            }
            63 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_MessageHeaderType(
    stream: &mut ExiBitstream,
    mut MessageHeaderType: *const iso20_acdp_MessageHeaderType,
) -> i32 {
    let mut grammar_id: i32 = 64 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            64 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
                                    grammar_id = 65 as i32;
                                }
                            }
                        }
                    }
                }
            }
            65 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
                                grammar_id = 66 as i32;
                            }
                        }
                    }
                }
            }
            66 => {
                if (*MessageHeaderType).Signature_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_acdp_SignatureType(
                            stream,
                            &(*MessageHeaderType).Signature,
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_SignaturePropertyType(
    stream: &mut ExiBitstream,
    mut SignaturePropertyType: *const iso20_acdp_SignaturePropertyType,
) -> i32 {
    let mut grammar_id: i32 = 67 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            67 => {
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
                                grammar_id = 68 as i32;
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
                                grammar_id = 69 as i32;
                            }
                        }
                    }
                }
            }
            68 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
                            grammar_id = 69 as i32;
                        }
                    }
                }
            }
            69 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_EVTechnicalStatusType(
    stream: &mut ExiBitstream,
    mut EVTechnicalStatusType: *const iso20_acdp_EVTechnicalStatusType,
) -> i32 {
    let mut grammar_id: i32 = 70 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            70 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(
                            stream,
                            (*EVTechnicalStatusType).EVReadyToCharge,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 71 as i32;
                            }
                        }
                    }
                }
            }
            71 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(
                            stream,
                            (*EVTechnicalStatusType).EVImmobilizationRequest,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 72 as i32;
                            }
                        }
                    }
                }
            }
            72 => {
                if (*EVTechnicalStatusType).EVImmobilized_isUsed() == 1 as u32 {
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
                                (*EVTechnicalStatusType).EVImmobilized,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 73 as i32;
                                }
                            }
                        }
                    }
                } else if (*EVTechnicalStatusType).EVWLANStrength_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_acdp_RationalNumberType(
                            stream,
                            &(*EVTechnicalStatusType).EVWLANStrength,
                        );
                        if error == 0 as i32 {
                            grammar_id = 74 as i32;
                        }
                    }
                } else if (*EVTechnicalStatusType).EVCPStatus_isUsed() == 1 as u32 {
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
                                3 as i32 as usize,
                                (*EVTechnicalStatusType).EVCPStatus as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 75 as i32;
                                }
                            }
                        }
                    }
                } else if (*EVTechnicalStatusType).EVSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
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
                                (*EVTechnicalStatusType).EVSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 76 as i32;
                                }
                            }
                        }
                    }
                } else if (*EVTechnicalStatusType).EVErrorCode_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                4 as i32 as usize,
                                (*EVTechnicalStatusType).EVErrorCode as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 77 as i32;
                                }
                            }
                        }
                    }
                } else if (*EVTechnicalStatusType).EVTimeout_isUsed() == 1 as u32 {
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
                                (*EVTechnicalStatusType).EVTimeout,
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
            73 => {
                if (*EVTechnicalStatusType).EVWLANStrength_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_acdp_RationalNumberType(
                            stream,
                            &(*EVTechnicalStatusType).EVWLANStrength,
                        );
                        if error == 0 as i32 {
                            grammar_id = 74 as i32;
                        }
                    }
                } else if (*EVTechnicalStatusType).EVCPStatus_isUsed() == 1 as u32 {
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
                                3 as i32 as usize,
                                (*EVTechnicalStatusType).EVCPStatus as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 75 as i32;
                                }
                            }
                        }
                    }
                } else if (*EVTechnicalStatusType).EVSOC_isUsed() == 1 as u32 {
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
                                (*EVTechnicalStatusType).EVSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 76 as i32;
                                }
                            }
                        }
                    }
                } else if (*EVTechnicalStatusType).EVErrorCode_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                4 as i32 as usize,
                                (*EVTechnicalStatusType).EVErrorCode as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 77 as i32;
                                }
                            }
                        }
                    }
                } else if (*EVTechnicalStatusType).EVTimeout_isUsed() == 1 as u32 {
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
                                (*EVTechnicalStatusType).EVTimeout,
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
            74 => {
                if (*EVTechnicalStatusType).EVCPStatus_isUsed() == 1 as u32 {
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
                                3 as i32 as usize,
                                (*EVTechnicalStatusType).EVCPStatus as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 75 as i32;
                                }
                            }
                        }
                    }
                } else if (*EVTechnicalStatusType).EVSOC_isUsed() == 1 as u32 {
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
                                (*EVTechnicalStatusType).EVSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 76 as i32;
                                }
                            }
                        }
                    }
                } else if (*EVTechnicalStatusType).EVErrorCode_isUsed() == 1 as u32 {
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
                                4 as i32 as usize,
                                (*EVTechnicalStatusType).EVErrorCode as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 77 as i32;
                                }
                            }
                        }
                    }
                } else if (*EVTechnicalStatusType).EVTimeout_isUsed() == 1 as u32 {
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
                                (*EVTechnicalStatusType).EVTimeout,
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
            75 => {
                if (*EVTechnicalStatusType).EVSOC_isUsed() == 1 as u32 {
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
                                (*EVTechnicalStatusType).EVSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 76 as i32;
                                }
                            }
                        }
                    }
                } else if (*EVTechnicalStatusType).EVErrorCode_isUsed() == 1 as u32 {
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
                                4 as i32 as usize,
                                (*EVTechnicalStatusType).EVErrorCode as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 77 as i32;
                                }
                            }
                        }
                    }
                } else if (*EVTechnicalStatusType).EVTimeout_isUsed() == 1 as u32 {
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
                                (*EVTechnicalStatusType).EVTimeout,
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
            76 => {
                if (*EVTechnicalStatusType).EVErrorCode_isUsed() == 1 as u32 {
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
                                4 as i32 as usize,
                                (*EVTechnicalStatusType).EVErrorCode as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 77 as i32;
                                }
                            }
                        }
                    }
                } else if (*EVTechnicalStatusType).EVTimeout_isUsed() == 1 as u32 {
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
                                (*EVTechnicalStatusType).EVTimeout,
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
            77 => {
                if (*EVTechnicalStatusType).EVTimeout_isUsed() == 1 as u32 {
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
                                (*EVTechnicalStatusType).EVTimeout,
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_ACDP_VehiclePositioningReqType(
    stream: &mut ExiBitstream,
    mut ACDP_VehiclePositioningReqType: *const iso20_acdp_ACDP_VehiclePositioningReqType,
) -> i32 {
    let mut grammar_id: i32 = 78 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            78 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_acdp_MessageHeaderType(
                        stream,
                        &(*ACDP_VehiclePositioningReqType).Header,
                    );
                    if error == 0 as i32 {
                        grammar_id = 79 as i32;
                    }
                }
            }
            79 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(
                            stream,
                            (*ACDP_VehiclePositioningReqType).EVMobilityStatus,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 80 as i32;
                            }
                        }
                    }
                }
            }
            80 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(
                            stream,
                            (*ACDP_VehiclePositioningReqType).EVPositioningSupport,
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_ACDP_VehiclePositioningResType(
    stream: &mut ExiBitstream,
    mut ACDP_VehiclePositioningResType: *const iso20_acdp_ACDP_VehiclePositioningResType,
) -> i32 {
    let mut grammar_id: i32 = 81 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            81 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_acdp_MessageHeaderType(
                        stream,
                        &(*ACDP_VehiclePositioningResType).Header,
                    );
                    if error == 0 as i32 {
                        grammar_id = 82 as i32;
                    }
                }
            }
            82 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            6 as i32 as usize,
                            (*ACDP_VehiclePositioningResType).ResponseCode as u32,
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
            }
            83 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*ACDP_VehiclePositioningResType).EVSEProcessing as u32,
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
            }
            84 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(
                            stream,
                            (*ACDP_VehiclePositioningResType).EVSEPositioningSupport,
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
            }
            85 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_integer_16(
                            stream,
                            (*ACDP_VehiclePositioningResType).EVRelativeXDeviation,
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
            }
            86 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_integer_16(
                            stream,
                            (*ACDP_VehiclePositioningResType).EVRelativeYDeviation,
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
            }
            87 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_integer_16(
                            stream,
                            (*ACDP_VehiclePositioningResType).ContactWindowXc,
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
            }
            88 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_integer_16(
                            stream,
                            (*ACDP_VehiclePositioningResType).ContactWindowYc,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 89 as i32;
                            }
                        }
                    }
                }
            }
            89 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(
                            stream,
                            (*ACDP_VehiclePositioningResType).EVInChargePosition,
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_ACDP_ConnectReqType(
    stream: &mut ExiBitstream,
    mut ACDP_ConnectReqType: *const iso20_acdp_ACDP_ConnectReqType,
) -> i32 {
    let mut grammar_id: i32 = 90 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            90 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso20_acdp_MessageHeaderType(stream, &(*ACDP_ConnectReqType).Header);
                    if error == 0 as i32 {
                        grammar_id = 91 as i32;
                    }
                }
            }
            91 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*ACDP_ConnectReqType).EVElectricalChargingDeviceStatus as u32,
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_ACDP_ConnectResType(
    stream: &mut ExiBitstream,
    mut ACDP_ConnectResType: *const iso20_acdp_ACDP_ConnectResType,
) -> i32 {
    let mut grammar_id: i32 = 92 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            92 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso20_acdp_MessageHeaderType(stream, &(*ACDP_ConnectResType).Header);
                    if error == 0 as i32 {
                        grammar_id = 93 as i32;
                    }
                }
            }
            93 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            6 as i32 as usize,
                            (*ACDP_ConnectResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 94 as i32;
                            }
                        }
                    }
                }
            }
            94 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*ACDP_ConnectResType).EVSEProcessing as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 95 as i32;
                            }
                        }
                    }
                }
            }
            95 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*ACDP_ConnectResType).EVSEElectricalChargingDeviceStatus as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 96 as i32;
                            }
                        }
                    }
                }
            }
            96 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*ACDP_ConnectResType).EVSEMechanicalChargingDeviceStatus as u32,
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_ACDP_SystemStatusReqType(
    stream: &mut ExiBitstream,
    mut ACDP_SystemStatusReqType: *const iso20_acdp_ACDP_SystemStatusReqType,
) -> i32 {
    let mut grammar_id: i32 = 97 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            97 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_acdp_MessageHeaderType(
                        stream,
                        &(*ACDP_SystemStatusReqType).Header,
                    );
                    if error == 0 as i32 {
                        grammar_id = 98 as i32;
                    }
                }
            }
            98 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_acdp_EVTechnicalStatusType(
                        stream,
                        &(*ACDP_SystemStatusReqType).EVTechnicalStatus,
                    );
                    if error == 0 as i32 {
                        grammar_id = 2 as i32;
                    }
                }
            }
            2 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_ACDP_SystemStatusResType(
    stream: &mut ExiBitstream,
    mut ACDP_SystemStatusResType: *const iso20_acdp_ACDP_SystemStatusResType,
) -> i32 {
    let mut grammar_id: i32 = 99 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            99 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_acdp_MessageHeaderType(
                        stream,
                        &(*ACDP_SystemStatusResType).Header,
                    );
                    if error == 0 as i32 {
                        grammar_id = 100 as i32;
                    }
                }
            }
            100 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            6 as i32 as usize,
                            (*ACDP_SystemStatusResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 101 as i32;
                            }
                        }
                    }
                }
            }
            101 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*ACDP_SystemStatusResType).EVSEMechanicalChargingDeviceStatus as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 102 as i32;
                            }
                        }
                    }
                }
            }
            102 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(
                            stream,
                            (*ACDP_SystemStatusResType).EVSEReadyToCharge,
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*ACDP_SystemStatusResType).EVSEIsolationStatus as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 104 as i32;
                            }
                        }
                    }
                }
            }
            104 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(
                            stream,
                            (*ACDP_SystemStatusResType).EVSEDisabled,
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
            }
            105 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(
                            stream,
                            (*ACDP_SystemStatusResType).EVSEUtilityInterruptEvent,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 106 as i32;
                            }
                        }
                    }
                }
            }
            106 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(
                            stream,
                            (*ACDP_SystemStatusResType).EVSEEmergencyShutdown,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 107 as i32;
                            }
                        }
                    }
                }
            }
            107 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(
                            stream,
                            (*ACDP_SystemStatusResType).EVSEMalfunction,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 108 as i32;
                            }
                        }
                    }
                }
            }
            108 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(
                            stream,
                            (*ACDP_SystemStatusResType).EVInChargePosition,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 109 as i32;
                            }
                        }
                    }
                }
            }
            109 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(
                            stream,
                            (*ACDP_SystemStatusResType).EVAssociationStatus,
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_CLReqControlModeType(
    stream: &mut ExiBitstream,
    mut CLReqControlModeType: *const iso20_acdp_CLReqControlModeType,
) -> i32 {
    let mut error: i32 =
        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
    return error;
}
unsafe extern "C" fn encode_iso20_acdp_CLResControlModeType(
    stream: &mut ExiBitstream,
    mut CLResControlModeType: *const iso20_acdp_CLResControlModeType,
) -> i32 {
    let mut error: i32 =
        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
    return error;
}
unsafe extern "C" fn encode_iso20_acdp_ManifestType(
    stream: &mut ExiBitstream,
    mut ManifestType: *const iso20_acdp_ManifestType,
) -> i32 {
    let mut grammar_id: i32 = 110 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut Reference_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            110 => {
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
                                grammar_id = 112 as i32;
                            }
                        }
                    }
                } else if (Reference_currentIndex as i32)
                    < (*ManifestType).Reference.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh2 = Reference_currentIndex;
                        Reference_currentIndex = Reference_currentIndex.wrapping_add(1);
                        error = encode_iso20_acdp_ReferenceType(
                            stream,
                            &*((*ManifestType).Reference.array)
                                .as_ptr()
                                .offset(fresh2 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 111 as i32;
                        }
                    }
                }
            }
            111 => {
                if (Reference_currentIndex as i32) < (*ManifestType).Reference.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh3 = Reference_currentIndex;
                        Reference_currentIndex = Reference_currentIndex.wrapping_add(1);
                        error = encode_iso20_acdp_ReferenceType(
                            stream,
                            &*((*ManifestType).Reference.array)
                                .as_ptr()
                                .offset(fresh3 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 111 as i32;
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
            112 => {
                if (Reference_currentIndex as i32) < (*ManifestType).Reference.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh4 = Reference_currentIndex;
                        Reference_currentIndex = Reference_currentIndex.wrapping_add(1);
                        error = encode_iso20_acdp_ReferenceType(
                            stream,
                            &*((*ManifestType).Reference.array)
                                .as_ptr()
                                .offset(fresh4 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 113 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            113 => {
                if (Reference_currentIndex as i32) < (*ManifestType).Reference.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh5 = Reference_currentIndex;
                        Reference_currentIndex = Reference_currentIndex.wrapping_add(1);
                        error = encode_iso20_acdp_ReferenceType(
                            stream,
                            &*((*ManifestType).Reference.array)
                                .as_ptr()
                                .offset(fresh5 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 113 as i32;
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_acdp_SignaturePropertiesType(
    stream: &mut ExiBitstream,
    mut SignaturePropertiesType: *const iso20_acdp_SignaturePropertiesType,
) -> i32 {
    let mut grammar_id: i32 = 114 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            114 => {
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
                                grammar_id = 116 as i32;
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_acdp_SignaturePropertyType(
                            stream,
                            &(*SignaturePropertiesType).SignatureProperty,
                        );
                        if error == 0 as i32 {
                            grammar_id = 115 as i32;
                        }
                    }
                }
            }
            115 => {
                if 1 as i32 == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_acdp_SignaturePropertyType(
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
            116 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_acdp_SignaturePropertyType(
                        stream,
                        &(*SignaturePropertiesType).SignatureProperty,
                    );
                    if error == 0 as i32 {
                        grammar_id = 117 as i32;
                    }
                }
            }
            117 => {
                if 1 as i32 == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_acdp_SignaturePropertyType(
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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

pub unsafe extern "C" fn encode_iso20_acdp_exiDocument(
    stream: &mut ExiBitstream,
    mut exiDoc: *mut iso20_acdp_exiDocument,
) -> i32 {
    let mut error: i32 = exi_header_write(stream);
    if error == 0 as i32 {
        if (*exiDoc).ACDP_ConnectReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 0 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_acdp_ACDP_ConnectReqType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.ACDP_ConnectReq,
                );
            }
        } else if (*exiDoc).ACDP_DisconnectReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 1 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_acdp_ACDP_ConnectReqType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.ACDP_DisconnectReq,
                );
            }
        } else if (*exiDoc).ACDP_ConnectRes_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 2 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_acdp_ACDP_ConnectResType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.ACDP_ConnectRes,
                );
            }
        } else if (*exiDoc).ACDP_DisconnectRes_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 3 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_acdp_ACDP_ConnectResType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.ACDP_DisconnectRes,
                );
            }
        } else if (*exiDoc).ACDP_SystemStatusReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 4 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_acdp_ACDP_SystemStatusReqType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.ACDP_SystemStatusReq,
                );
            }
        } else if (*exiDoc).ACDP_SystemStatusRes_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 5 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_acdp_ACDP_SystemStatusResType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.ACDP_SystemStatusRes,
                );
            }
        } else if (*exiDoc).ACDP_VehiclePositioningReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 6 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_acdp_ACDP_VehiclePositioningReqType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.ACDP_VehiclePositioningReq,
                );
            }
        } else if (*exiDoc).ACDP_VehiclePositioningRes_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 7 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_acdp_ACDP_VehiclePositioningResType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.ACDP_VehiclePositioningRes,
                );
            }
        } else if (*exiDoc).CLReqControlMode_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 8 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_acdp_CLReqControlModeType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.CLReqControlMode,
                );
            }
        } else if (*exiDoc).CLResControlMode_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 9 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_acdp_CLResControlModeType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.CLResControlMode,
                );
            }
        } else if (*exiDoc).CanonicalizationMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 10 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_acdp_CanonicalizationMethodType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.CanonicalizationMethod,
                );
            }
        } else if (*exiDoc).DSAKeyValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 11 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_acdp_DSAKeyValueType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.DSAKeyValue,
                );
            }
        } else if (*exiDoc).DigestMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 12 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_acdp_DigestMethodType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.DigestMethod,
                );
            }
        } else if (*exiDoc).KeyInfo_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 14 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_acdp_KeyInfoType(stream, &mut (*exiDoc).c2rust_unnamed.KeyInfo);
            }
        } else if (*exiDoc).KeyValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 16 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_acdp_KeyValueType(stream, &mut (*exiDoc).c2rust_unnamed.KeyValue);
            }
        } else if (*exiDoc).Manifest_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 17 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_acdp_ManifestType(stream, &mut (*exiDoc).c2rust_unnamed.Manifest);
            }
        } else if (*exiDoc).Object_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 19 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_acdp_ObjectType(stream, &mut (*exiDoc).c2rust_unnamed.Object);
            }
        } else if (*exiDoc).PGPData_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 20 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_acdp_PGPDataType(stream, &mut (*exiDoc).c2rust_unnamed.PGPData);
            }
        } else if (*exiDoc).RSAKeyValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 21 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_acdp_RSAKeyValueType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.RSAKeyValue,
                );
            }
        } else if (*exiDoc).Reference_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 22 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_acdp_ReferenceType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.Reference,
                );
            }
        } else if (*exiDoc).RetrievalMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 23 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_acdp_RetrievalMethodType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.RetrievalMethod,
                );
            }
        } else if (*exiDoc).SPKIData_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 24 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_acdp_SPKIDataType(stream, &mut (*exiDoc).c2rust_unnamed.SPKIData);
            }
        } else if (*exiDoc).SignatureMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 25 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_acdp_SignatureMethodType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.SignatureMethod,
                );
            }
        } else if (*exiDoc).SignatureProperties_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 26 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_acdp_SignaturePropertiesType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.SignatureProperties,
                );
            }
        } else if (*exiDoc).SignatureProperty_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 27 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_acdp_SignaturePropertyType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.SignatureProperty,
                );
            }
        } else if (*exiDoc).Signature_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 28 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_acdp_SignatureType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.Signature,
                );
            }
        } else if (*exiDoc).SignatureValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 29 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_acdp_SignatureValueType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.SignatureValue,
                );
            }
        } else if (*exiDoc).SignedInfo_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 30 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_acdp_SignedInfoType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.SignedInfo,
                );
            }
        } else if (*exiDoc).Transform_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 31 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_acdp_TransformType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.Transform,
                );
            }
        } else if (*exiDoc).Transforms_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 32 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_acdp_TransformsType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.Transforms,
                );
            }
        } else if (*exiDoc).X509Data_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 33 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_acdp_X509DataType(stream, &mut (*exiDoc).c2rust_unnamed.X509Data);
            }
        } else {
            error = -(70 as i32);
        }
    }
    return error;
}
