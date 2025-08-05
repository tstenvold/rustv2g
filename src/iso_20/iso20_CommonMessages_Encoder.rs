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
    fn exi_basetypes_encoder_integer_32(stream: *mut exi_bitstream_t, value: i32) -> i32;
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
pub type __i32 = i32;
pub type __ui32 = u32;
pub type __u64 = u64;
pub type int8_t = i8;
pub type i16 = __int16_t;
pub type i32 = __i32;
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
pub type iso20_powerToleranceAcceptanceType = u32;
pub const iso20_powerToleranceAcceptanceType_PowerToleranceConfirmed:
    iso20_powerToleranceAcceptanceType = 1;
pub const iso20_powerToleranceAcceptanceType_PowerToleranceNotConfirmed:
    iso20_powerToleranceAcceptanceType = 0;
pub type iso20_evseNotificationType = u32;
pub const iso20_evseNotificationType_MeteringConfirmation: iso20_evseNotificationType = 5;
pub const iso20_evseNotificationType_ServiceRenegotiation: iso20_evseNotificationType = 4;
pub const iso20_evseNotificationType_ScheduleRenegotiation: iso20_evseNotificationType = 3;
pub const iso20_evseNotificationType_Terminate: iso20_evseNotificationType = 2;
pub const iso20_evseNotificationType_ExitStandby: iso20_evseNotificationType = 1;
pub const iso20_evseNotificationType_Pause: iso20_evseNotificationType = 0;
pub type iso20_ecdhCurveType = u32;
pub const iso20_ecdhCurveType_X448: iso20_ecdhCurveType = 1;
pub const iso20_ecdhCurveType_SECP521: iso20_ecdhCurveType = 0;
pub type iso20_responseCodeType = u32;
pub const iso20_responseCodeType_FAILED_WrongChargeParameter: iso20_responseCodeType = 39;
pub const iso20_responseCodeType_FAILED_UnknownSession: iso20_responseCodeType = 38;
pub const iso20_responseCodeType_FAILED_SignatureError: iso20_responseCodeType = 37;
pub const iso20_responseCodeType_FAILED_ServiceSelectionInvalid: iso20_responseCodeType = 36;
pub const iso20_responseCodeType_FAILED_ServiceIDInvalid: iso20_responseCodeType = 35;
pub const iso20_responseCodeType_FAILED_SequenceError: iso20_responseCodeType = 34;
pub const iso20_responseCodeType_FAILED_ScheduleSelectionInvalid: iso20_responseCodeType = 33;
pub const iso20_responseCodeType_FAILED_ScheduleRenegotiation: iso20_responseCodeType = 32;
pub const iso20_responseCodeType_FAILED_PowerToleranceNotConfirmed: iso20_responseCodeType = 31;
pub const iso20_responseCodeType_FAILED_PowerDeliveryNotApplied: iso20_responseCodeType = 30;
pub const iso20_responseCodeType_FAILED_PauseNotAllowed: iso20_responseCodeType = 29;
pub const iso20_responseCodeType_FAILED_NoServiceRenegotiationSupported: iso20_responseCodeType =
    28;
pub const iso20_responseCodeType_FAILED_NoEnergyTransferServiceSelected: iso20_responseCodeType =
    27;
pub const iso20_responseCodeType_FAILED_MeteringSignatureNotValid: iso20_responseCodeType = 26;
pub const iso20_responseCodeType_FAILED_EVPowerProfileViolation: iso20_responseCodeType = 25;
pub const iso20_responseCodeType_FAILED_EVPowerProfileInvalid: iso20_responseCodeType = 24;
pub const iso20_responseCodeType_FAILED_ContactorError: iso20_responseCodeType = 23;
pub const iso20_responseCodeType_FAILED_AssociationError: iso20_responseCodeType = 22;
pub const iso20_responseCodeType_FAILED: iso20_responseCodeType = 21;
pub const iso20_responseCodeType_WARNING_WPT: iso20_responseCodeType = 20;
pub const iso20_responseCodeType_WARNING_StandbyNotAllowed: iso20_responseCodeType = 19;
pub const iso20_responseCodeType_WARNING_ScheduleRenegotiationFailed: iso20_responseCodeType = 18;
pub const iso20_responseCodeType_WARNING_PowerToleranceNotConfirmed: iso20_responseCodeType = 17;
pub const iso20_responseCodeType_WARNING_NoContractMatchingPCIDFound: iso20_responseCodeType = 16;
pub const iso20_responseCodeType_WARNING_NoCertificateAvailable: iso20_responseCodeType = 15;
pub const iso20_responseCodeType_WARNING_GeneralPnCAuthorizationError: iso20_responseCodeType = 14;
pub const iso20_responseCodeType_WARNING_EVPowerProfileViolation: iso20_responseCodeType = 13;
pub const iso20_responseCodeType_WARNING_eMSPUnknown: iso20_responseCodeType = 12;
pub const iso20_responseCodeType_WARNING_EIMAuthorizationFailure: iso20_responseCodeType = 11;
pub const iso20_responseCodeType_WARNING_ChallengeInvalid: iso20_responseCodeType = 10;
pub const iso20_responseCodeType_WARNING_CertificateValidationError: iso20_responseCodeType = 9;
pub const iso20_responseCodeType_WARNING_CertificateRevoked: iso20_responseCodeType = 8;
pub const iso20_responseCodeType_WARNING_CertificateNotYetValid: iso20_responseCodeType = 7;
pub const iso20_responseCodeType_WARNING_CertificateExpired: iso20_responseCodeType = 6;
pub const iso20_responseCodeType_WARNING_AuthorizationSelectionInvalid: iso20_responseCodeType = 5;
pub const iso20_responseCodeType_OK_PowerToleranceConfirmed: iso20_responseCodeType = 4;
pub const iso20_responseCodeType_OK_OldSessionJoined: iso20_responseCodeType = 3;
pub const iso20_responseCodeType_OK_NewSessionEstablished: iso20_responseCodeType = 2;
pub const iso20_responseCodeType_OK_CertificateExpiresSoon: iso20_responseCodeType = 1;
pub const iso20_responseCodeType_OK: iso20_responseCodeType = 0;
pub type iso20_chargingSessionType = u32;
pub const iso20_chargingSessionType_ServiceRenegotiation: iso20_chargingSessionType = 2;
pub const iso20_chargingSessionType_Terminate: iso20_chargingSessionType = 1;
pub const iso20_chargingSessionType_Pause: iso20_chargingSessionType = 0;
pub type iso20_evCheckInStatusType = u32;
pub const iso20_evCheckInStatusType_Completed: iso20_evCheckInStatusType = 2;
pub const iso20_evCheckInStatusType_Processing: iso20_evCheckInStatusType = 1;
pub const iso20_evCheckInStatusType_CheckIn: iso20_evCheckInStatusType = 0;
pub type iso20_evCheckOutStatusType = u32;
pub const iso20_evCheckOutStatusType_Completed: iso20_evCheckOutStatusType = 2;
pub const iso20_evCheckOutStatusType_Processing: iso20_evCheckOutStatusType = 1;
pub const iso20_evCheckOutStatusType_CheckOut: iso20_evCheckOutStatusType = 0;
pub type iso20_authorizationType = u32;
pub const iso20_authorizationType_PnC: iso20_authorizationType = 1;
pub const iso20_authorizationType_EIM: iso20_authorizationType = 0;
pub type iso20_processingType = u32;
pub const iso20_processingType_Ongoing_WaitingForCustomerInteraction: iso20_processingType = 2;
pub const iso20_processingType_Ongoing: iso20_processingType = 1;
pub const iso20_processingType_Finished: iso20_processingType = 0;
pub type iso20_chargeProgressType = u32;
pub const iso20_chargeProgressType_ScheduleRenegotiation: iso20_chargeProgressType = 3;
pub const iso20_chargeProgressType_Standby: iso20_chargeProgressType = 2;
pub const iso20_chargeProgressType_Stop: iso20_chargeProgressType = 1;
pub const iso20_chargeProgressType_Start: iso20_chargeProgressType = 0;
pub type iso20_parkingMethodType = u32;
pub const iso20_parkingMethodType_Manual: iso20_parkingMethodType = 2;
pub const iso20_parkingMethodType_MVGuideManual: iso20_parkingMethodType = 1;
pub const iso20_parkingMethodType_AutoParking: iso20_parkingMethodType = 0;
pub type iso20_evseCheckOutStatusType = u32;
pub const iso20_evseCheckOutStatusType_Completed: iso20_evseCheckOutStatusType = 1;
pub const iso20_evseCheckOutStatusType_Scheduled: iso20_evseCheckOutStatusType = 0;
pub type iso20_channelSelectionType = u32;
pub const iso20_channelSelectionType_Discharge: iso20_channelSelectionType = 1;
pub const iso20_channelSelectionType_Charge: iso20_channelSelectionType = 0;
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_TransformType {
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

pub struct iso20_TransformsType {
    pub Transform: iso20_TransformType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_DSAKeyValueType {
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

pub struct iso20_X509IssuerSerialType {
    pub X509IssuerName: C2RustUnnamed_9,
    pub X509SerialNumber: ExiSigned,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_9 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_DigestMethodType {
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

pub struct iso20_RSAKeyValueType {
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

pub struct iso20_CanonicalizationMethodType {
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
#[derive(Copy, Clone)]

pub struct iso20_PriceLevelScheduleEntryType {
    pub Duration: u32,
    pub PriceLevel: u8,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_SignatureMethodType {
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

pub struct iso20_KeyValueType {
    pub DSAKeyValue: iso20_DSAKeyValueType,
    #[bitfield(name = "DSAKeyValue_isUsed", ty = "u32", bits = "0..=0")]
    pub DSAKeyValue_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub RSAKeyValue: iso20_RSAKeyValueType,
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

pub struct iso20_ReferenceType {
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
    pub Transforms: iso20_TransformsType,
    #[bitfield(name = "Transforms_isUsed", ty = "u32", bits = "0..=0")]
    pub Transforms_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 3],
    pub DigestMethod: iso20_DigestMethodType,
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

pub struct iso20_RetrievalMethodType {
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
    pub Transforms: iso20_TransformsType,
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

pub struct iso20_X509DataType {
    pub X509IssuerSerial: iso20_X509IssuerSerialType,
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

pub struct iso20_PGPDataType {
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
#[derive(Copy, Clone)]

pub struct iso20_RationalNumberType {
    pub Exponent: int8_t,
    pub Value: i16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_PowerScheduleEntryType {
    pub Duration: u32,
    pub Power: iso20_RationalNumberType,
    pub Power_L2: iso20_RationalNumberType,
    #[bitfield(name = "Power_L2_isUsed", ty = "u32", bits = "0..=0")]
    pub Power_L2_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub Power_L3: iso20_RationalNumberType,
    #[bitfield(name = "Power_L3_isUsed", ty = "u32", bits = "0..=0")]
    pub Power_L3_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
}
#[derive(Copy, Clone)]

pub struct iso20_EVPriceRuleType {
    pub EnergyFee: iso20_RationalNumberType,
    pub PowerRangeStart: iso20_RationalNumberType,
}
#[derive(Copy, Clone)]

pub struct iso20_EVPowerScheduleEntryType {
    pub Duration: u32,
    pub Power: iso20_RationalNumberType,
}
#[derive(Copy, Clone)]

pub struct iso20_EVPriceRuleStackType {
    pub Duration: u32,
    pub EVPriceRule: C2RustUnnamed_38,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_38 {
    pub array: [iso20_EVPriceRuleType; 8],
    pub arrayLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_PriceRuleType {
    pub EnergyFee: iso20_RationalNumberType,
    pub ParkingFee: iso20_RationalNumberType,
    #[bitfield(name = "ParkingFee_isUsed", ty = "u32", bits = "0..=0")]
    pub ParkingFee_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub ParkingFeePeriod: u32,
    #[bitfield(name = "ParkingFeePeriod_isUsed", ty = "u32", bits = "0..=0")]
    pub ParkingFeePeriod_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub CarbonDioxideEmission: u16,
    #[bitfield(name = "CarbonDioxideEmission_isUsed", ty = "u32", bits = "0..=0")]
    pub CarbonDioxideEmission_isUsed: [u8; 1],
    pub RenewableGenerationPercentage: u8,
    #[bitfield(
        name = "RenewableGenerationPercentage_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub RenewableGenerationPercentage_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub PowerRangeStart: iso20_RationalNumberType,
}
#[derive(Copy, Clone)]

pub struct iso20_PowerScheduleEntryListType {
    pub PowerScheduleEntry: C2RustUnnamed_39,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_39 {
    pub array: [iso20_PowerScheduleEntryType; 1024],
    pub arrayLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_TaxRuleType {
    pub TaxRuleID: u32,
    pub TaxRuleName: C2RustUnnamed_40,
    #[bitfield(name = "TaxRuleName_isUsed", ty = "u32", bits = "0..=0")]
    pub TaxRuleName_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub TaxRate: iso20_RationalNumberType,
    pub TaxIncludedInPrice: i32,
    #[bitfield(name = "TaxIncludedInPrice_isUsed", ty = "u32", bits = "0..=0")]
    pub TaxIncludedInPrice_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
    pub AppliesToEnergyFee: i32,
    pub AppliesToParkingFee: i32,
    pub AppliesToOverstayFee: i32,
    pub AppliesMinimumMaximumCost: i32,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_40 {
    pub characters: [i8; 81],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_PriceRuleStackType {
    pub Duration: u32,
    pub PriceRule: C2RustUnnamed_41,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_41 {
    pub array: [iso20_PriceRuleType; 8],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_AdditionalServiceType {
    pub ServiceName: C2RustUnnamed_42,
    pub ServiceFee: iso20_RationalNumberType,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_42 {
    pub characters: [i8; 81],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_PowerScheduleType {
    pub TimeAnchor: u64,
    pub AvailableEnergy: iso20_RationalNumberType,
    #[bitfield(name = "AvailableEnergy_isUsed", ty = "u32", bits = "0..=0")]
    pub AvailableEnergy_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub PowerTolerance: iso20_RationalNumberType,
    #[bitfield(name = "PowerTolerance_isUsed", ty = "u32", bits = "0..=0")]
    pub PowerTolerance_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub PowerScheduleEntries: iso20_PowerScheduleEntryListType,
}
#[derive(Copy, Clone)]

pub struct iso20_EVPowerScheduleEntryListType {
    pub EVPowerScheduleEntry: C2RustUnnamed_43,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_43 {
    pub array: [iso20_EVPowerScheduleEntryType; 1024],
    pub arrayLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_OverstayRuleType {
    pub OverstayRuleDescription: C2RustUnnamed_44,
    #[bitfield(name = "OverstayRuleDescription_isUsed", ty = "u32", bits = "0..=0")]
    pub OverstayRuleDescription_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub StartTime: u32,
    pub OverstayFee: iso20_RationalNumberType,
    pub OverstayFeePeriod: u32,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_44 {
    pub characters: [i8; 161],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_EVPriceRuleStackListType {
    pub EVPriceRuleStack: C2RustUnnamed_45,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_45 {
    pub array: [iso20_EVPriceRuleStackType; 1024],
    pub arrayLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_SPKIDataType {
    pub SPKISexp: C2RustUnnamed_47,
    pub ANY: C2RustUnnamed_46,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_46 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_47 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_SignedInfoType {
    pub Id: C2RustUnnamed_49,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub CanonicalizationMethod: iso20_CanonicalizationMethodType,
    pub SignatureMethod: iso20_SignatureMethodType,
    pub Reference: C2RustUnnamed_48,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_48 {
    pub array: [iso20_ReferenceType; 4],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_49 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_EVPowerScheduleType {
    pub TimeAnchor: u64,
    pub EVPowerScheduleEntries: iso20_EVPowerScheduleEntryListType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_SignatureValueType {
    pub Id: C2RustUnnamed_51,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub CONTENT: C2RustUnnamed_50,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_50 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_51 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_SubCertificatesType {
    pub Certificate: C2RustUnnamed_52,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_52 {
    pub array: [C2RustUnnamed_53; 3],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_53 {
    pub bytes: [u8; 1600],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ParameterType {
    pub Name: C2RustUnnamed_55,
    pub boolValue: i32,
    #[bitfield(name = "boolValue_isUsed", ty = "u32", bits = "0..=0")]
    pub boolValue_isUsed: [u8; 1],
    pub byteValue: int8_t,
    #[bitfield(name = "byteValue_isUsed", ty = "u32", bits = "0..=0")]
    pub byteValue_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub shortValue: i16,
    #[bitfield(name = "shortValue_isUsed", ty = "u32", bits = "0..=0")]
    pub shortValue_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub intValue: i32,
    #[bitfield(name = "intValue_isUsed", ty = "u32", bits = "0..=0")]
    pub intValue_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub rationalNumber: iso20_RationalNumberType,
    #[bitfield(name = "rationalNumber_isUsed", ty = "u32", bits = "0..=0")]
    pub rationalNumber_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub finiteString: C2RustUnnamed_54,
    #[bitfield(name = "finiteString_isUsed", ty = "u32", bits = "0..=0")]
    pub finiteString_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 3],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_54 {
    pub characters: [i8; 81],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_55 {
    pub characters: [i8; 81],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_EVAbsolutePriceScheduleType {
    pub TimeAnchor: u64,
    pub Currency: C2RustUnnamed_57,
    pub PriceAlgorithm: C2RustUnnamed_56,
    pub EVPriceRuleStacks: iso20_EVPriceRuleStackListType,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_56 {
    pub characters: [i8; 256],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_57 {
    pub characters: [i8; 4],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_DetailedCostType {
    pub Amount: iso20_RationalNumberType,
    pub CostPerUnit: iso20_RationalNumberType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_KeyInfoType {
    pub Id: C2RustUnnamed_61,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub KeyName: C2RustUnnamed_60,
    #[bitfield(name = "KeyName_isUsed", ty = "u32", bits = "0..=0")]
    pub KeyName_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub KeyValue: iso20_KeyValueType,
    #[bitfield(name = "KeyValue_isUsed", ty = "u32", bits = "0..=0")]
    pub KeyValue_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 3],
    pub RetrievalMethod: iso20_RetrievalMethodType,
    #[bitfield(name = "RetrievalMethod_isUsed", ty = "u32", bits = "0..=0")]
    pub RetrievalMethod_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 3],
    pub X509Data: iso20_X509DataType,
    #[bitfield(name = "X509Data_isUsed", ty = "u32", bits = "0..=0")]
    pub X509Data_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 3],
    pub PGPData: iso20_PGPDataType,
    #[bitfield(name = "PGPData_isUsed", ty = "u32", bits = "0..=0")]
    pub PGPData_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 3],
    pub SPKIData: iso20_SPKIDataType,
    #[bitfield(name = "SPKIData_isUsed", ty = "u32", bits = "0..=0")]
    pub SPKIData_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_5: [u8; 1],
    pub MgmtData: C2RustUnnamed_59,
    #[bitfield(name = "MgmtData_isUsed", ty = "u32", bits = "0..=0")]
    pub MgmtData_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_6: [u8; 1],
    pub ANY: C2RustUnnamed_58,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_7: [u8; 5],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_58 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_59 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_60 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_61 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ObjectType {
    pub Encoding: C2RustUnnamed_65,
    #[bitfield(name = "Encoding_isUsed", ty = "u32", bits = "0..=0")]
    pub Encoding_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub Id: C2RustUnnamed_64,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub MimeType: C2RustUnnamed_63,
    #[bitfield(name = "MimeType_isUsed", ty = "u32", bits = "0..=0")]
    pub MimeType_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub ANY: C2RustUnnamed_62,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 3],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_62 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_63 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_64 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_65 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_PriceLevelScheduleEntryListType {
    pub PriceLevelScheduleEntry: C2RustUnnamed_66,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_66 {
    pub array: [iso20_PriceLevelScheduleEntryType; 1024],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_DetailedTaxType {
    pub TaxRuleID: u32,
    pub Amount: iso20_RationalNumberType,
}
#[derive(Copy, Clone)]

pub struct iso20_TaxRuleListType {
    pub TaxRule: C2RustUnnamed_67,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_67 {
    pub array: [iso20_TaxRuleType; 10],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_PriceRuleStackListType {
    pub PriceRuleStack: C2RustUnnamed_68,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_68 {
    pub array: [iso20_PriceRuleStackType; 64],
    pub arrayLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_OverstayRuleListType {
    pub OverstayTimeThreshold: u32,
    #[bitfield(name = "OverstayTimeThreshold_isUsed", ty = "u32", bits = "0..=0")]
    pub OverstayTimeThreshold_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub OverstayPowerThreshold: iso20_RationalNumberType,
    #[bitfield(name = "OverstayPowerThreshold_isUsed", ty = "u32", bits = "0..=0")]
    pub OverstayPowerThreshold_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub OverstayRule: C2RustUnnamed_69,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_69 {
    pub array: [iso20_OverstayRuleType; 5],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_AdditionalServiceListType {
    pub AdditionalService: C2RustUnnamed_70,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_70 {
    pub array: [iso20_AdditionalServiceType; 5],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_ServiceType {
    pub ServiceID: u16,
    pub FreeService: i32,
}
#[derive(Copy, Clone)]

pub struct iso20_ParameterSetType {
    pub ParameterSetID: u16,
    pub Parameter: C2RustUnnamed_71,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_71 {
    pub array: [iso20_ParameterType; 8],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_SupportedProvidersListType {
    pub ProviderID: C2RustUnnamed_72,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_72 {
    pub array: [C2RustUnnamed_73; 128],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_73 {
    pub characters: [i8; 81],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_ContractCertificateChainType {
    pub Certificate: C2RustUnnamed_74,
    pub SubCertificates: iso20_SubCertificatesType,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_74 {
    pub bytes: [u8; 1600],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_Dynamic_EVPPTControlModeType {
    pub _unused: i32,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_MeterInfoType {
    pub MeterID: C2RustUnnamed_76,
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
    pub MeterSignature: C2RustUnnamed_75,
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

pub struct C2RustUnnamed_75 {
    pub bytes: [u8; 64],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_76 {
    pub characters: [i8; 33],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_SignatureType {
    pub Id: C2RustUnnamed_77,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub SignedInfo: iso20_SignedInfoType,
    pub SignatureValue: iso20_SignatureValueType,
    pub KeyInfo: iso20_KeyInfoType,
    #[bitfield(name = "KeyInfo_isUsed", ty = "u32", bits = "0..=0")]
    pub KeyInfo_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
    pub Object: iso20_ObjectType,
    #[bitfield(name = "Object_isUsed", ty = "u32", bits = "0..=0")]
    pub Object_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 7],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_77 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_Scheduled_EVPPTControlModeType {
    pub SelectedScheduleTupleID: u32,
    pub PowerToleranceAcceptance: iso20_powerToleranceAcceptanceType,
    #[bitfield(name = "PowerToleranceAcceptance_isUsed", ty = "u32", bits = "0..=0")]
    pub PowerToleranceAcceptance_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ReceiptType {
    pub TimeAnchor: u64,
    pub EnergyCosts: iso20_DetailedCostType,
    #[bitfield(name = "EnergyCosts_isUsed", ty = "u32", bits = "0..=0")]
    pub EnergyCosts_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub OccupancyCosts: iso20_DetailedCostType,
    #[bitfield(name = "OccupancyCosts_isUsed", ty = "u32", bits = "0..=0")]
    pub OccupancyCosts_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub AdditionalServicesCosts: iso20_DetailedCostType,
    #[bitfield(name = "AdditionalServicesCosts_isUsed", ty = "u32", bits = "0..=0")]
    pub AdditionalServicesCosts_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub OverstayCosts: iso20_DetailedCostType,
    #[bitfield(name = "OverstayCosts_isUsed", ty = "u32", bits = "0..=0")]
    pub OverstayCosts_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub TaxCosts: C2RustUnnamed_78,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_78 {
    pub array: [iso20_DetailedTaxType; 10],
    pub arrayLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_AbsolutePriceScheduleType {
    pub Id: C2RustUnnamed_83,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub TimeAnchor: u64,
    pub PriceScheduleID: u32,
    pub PriceScheduleDescription: C2RustUnnamed_82,
    #[bitfield(name = "PriceScheduleDescription_isUsed", ty = "u32", bits = "0..=0")]
    pub PriceScheduleDescription_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub Currency: C2RustUnnamed_81,
    pub Language: C2RustUnnamed_80,
    pub PriceAlgorithm: C2RustUnnamed_79,
    pub MinimumCost: iso20_RationalNumberType,
    #[bitfield(name = "MinimumCost_isUsed", ty = "u32", bits = "0..=0")]
    pub MinimumCost_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub MaximumCost: iso20_RationalNumberType,
    #[bitfield(name = "MaximumCost_isUsed", ty = "u32", bits = "0..=0")]
    pub MaximumCost_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub TaxRules: iso20_TaxRuleListType,
    #[bitfield(name = "TaxRules_isUsed", ty = "u32", bits = "0..=0")]
    pub TaxRules_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 3],
    pub PriceRuleStacks: iso20_PriceRuleStackListType,
    pub OverstayRules: iso20_OverstayRuleListType,
    #[bitfield(name = "OverstayRules_isUsed", ty = "u32", bits = "0..=0")]
    pub OverstayRules_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 1],
    pub AdditionalSelectedServices: iso20_AdditionalServiceListType,
    #[bitfield(name = "AdditionalSelectedServices_isUsed", ty = "u32", bits = "0..=0")]
    pub AdditionalSelectedServices_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_5: [u8; 7],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_79 {
    pub characters: [i8; 256],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_80 {
    pub characters: [i8; 4],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_81 {
    pub characters: [i8; 4],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_82 {
    pub characters: [i8; 161],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_83 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_EVPowerProfileEntryListType {
    pub EVPowerProfileEntry: C2RustUnnamed_84,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_84 {
    pub array: [iso20_PowerScheduleEntryType; 2048],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_Dynamic_SMDTControlModeType {
    pub _unused: i32,
}
#[derive(Copy, Clone)]

pub struct iso20_EVEnergyOfferType {
    pub EVPowerSchedule: iso20_EVPowerScheduleType,
    pub EVAbsolutePriceSchedule: iso20_EVAbsolutePriceScheduleType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_PriceLevelScheduleType {
    pub Id: C2RustUnnamed_86,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub TimeAnchor: u64,
    pub PriceScheduleID: u32,
    pub PriceScheduleDescription: C2RustUnnamed_85,
    #[bitfield(name = "PriceScheduleDescription_isUsed", ty = "u32", bits = "0..=0")]
    pub PriceScheduleDescription_isUsed: [u8; 1],
    pub NumberOfPriceLevels: u8,
    pub PriceLevelScheduleEntries: iso20_PriceLevelScheduleEntryListType,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_85 {
    pub characters: [i8; 161],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_86 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ChargingScheduleType {
    pub PowerSchedule: iso20_PowerScheduleType,
    pub AbsolutePriceSchedule: iso20_AbsolutePriceScheduleType,
    #[bitfield(name = "AbsolutePriceSchedule_isUsed", ty = "u32", bits = "0..=0")]
    pub AbsolutePriceSchedule_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub PriceLevelSchedule: iso20_PriceLevelScheduleType,
    #[bitfield(name = "PriceLevelSchedule_isUsed", ty = "u32", bits = "0..=0")]
    pub PriceLevelSchedule_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 7],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ScheduleTupleType {
    pub ScheduleTupleID: u32,
    pub ChargingSchedule: iso20_ChargingScheduleType,
    pub DischargingSchedule: iso20_ChargingScheduleType,
    #[bitfield(name = "DischargingSchedule_isUsed", ty = "u32", bits = "0..=0")]
    pub DischargingSchedule_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]

pub struct iso20_Scheduled_SMDTControlModeType {
    pub SelectedScheduleTupleID: u32,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_MessageHeaderType {
    pub SessionID: C2RustUnnamed_87,
    pub TimeStamp: u64,
    pub Signature: iso20_SignatureType,
    #[bitfield(name = "Signature_isUsed", ty = "u32", bits = "0..=0")]
    pub Signature_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_87 {
    pub bytes: [u8; 8],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_SignaturePropertyType {
    pub Id: C2RustUnnamed_90,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub Target: C2RustUnnamed_89,
    pub ANY: C2RustUnnamed_88,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_88 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_89 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_90 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_ServiceIDListType {
    pub ServiceID: C2RustUnnamed_91,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_91 {
    pub array: [u16; 16],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_SelectedServiceType {
    pub ServiceID: u16,
    pub ParameterSetID: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_SignedMeteringDataType {
    pub Id: C2RustUnnamed_93,
    pub SessionID: C2RustUnnamed_92,
    pub MeterInfo: iso20_MeterInfoType,
    pub Receipt: iso20_ReceiptType,
    #[bitfield(name = "Receipt_isUsed", ty = "u32", bits = "0..=0")]
    pub Receipt_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub Dynamic_SMDTControlMode: iso20_Dynamic_SMDTControlModeType,
    #[bitfield(name = "Dynamic_SMDTControlMode_isUsed", ty = "u32", bits = "0..=0")]
    pub Dynamic_SMDTControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
    pub Scheduled_SMDTControlMode: iso20_Scheduled_SMDTControlModeType,
    #[bitfield(name = "Scheduled_SMDTControlMode_isUsed", ty = "u32", bits = "0..=0")]
    pub Scheduled_SMDTControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 7],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_92 {
    pub bytes: [u8; 8],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_93 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_SignedCertificateChainType {
    pub Id: C2RustUnnamed_95,
    pub Certificate: C2RustUnnamed_94,
    pub SubCertificates: iso20_SubCertificatesType,
    #[bitfield(name = "SubCertificates_isUsed", ty = "u32", bits = "0..=0")]
    pub SubCertificates_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_94 {
    pub bytes: [u8; 1600],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_95 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_EIM_AReqAuthorizationModeType {
    pub _unused: i32,
}
#[derive(Copy, Clone)]

pub struct iso20_SelectedServiceListType {
    pub SelectedService: C2RustUnnamed_96,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_96 {
    pub array: [iso20_SelectedServiceType; 16],
    pub arrayLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_Dynamic_SEReqControlModeType {
    pub DepartureTime: u32,
    pub MinimumSOC: int8_t,
    #[bitfield(name = "MinimumSOC_isUsed", ty = "u32", bits = "0..=0")]
    pub MinimumSOC_isUsed: [u8; 1],
    pub TargetSOC: int8_t,
    #[bitfield(name = "TargetSOC_isUsed", ty = "u32", bits = "0..=0")]
    pub TargetSOC_isUsed: [u8; 1],
    pub EVTargetEnergyRequest: iso20_RationalNumberType,
    pub EVMaximumEnergyRequest: iso20_RationalNumberType,
    pub EVMinimumEnergyRequest: iso20_RationalNumberType,
    pub EVMaximumV2XEnergyRequest: iso20_RationalNumberType,
    #[bitfield(name = "EVMaximumV2XEnergyRequest_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumV2XEnergyRequest_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub EVMinimumV2XEnergyRequest: iso20_RationalNumberType,
    #[bitfield(name = "EVMinimumV2XEnergyRequest_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumV2XEnergyRequest_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
}
#[derive(Copy, Clone)]

pub struct iso20_EVSEStatusType {
    pub NotificationMaxDelay: u16,
    pub EVSENotification: iso20_evseNotificationType,
}
#[derive(Copy, Clone)]

pub struct iso20_ListOfRootCertificateIDsType {
    pub RootCertificateID: C2RustUnnamed_97,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_97 {
    pub array: [iso20_X509IssuerSerialType; 20],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_PnC_AReqAuthorizationModeType {
    pub Id: C2RustUnnamed_99,
    pub GenChallenge: C2RustUnnamed_98,
    pub ContractCertificateChain: iso20_ContractCertificateChainType,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_98 {
    pub bytes: [u8; 16],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_99 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_ServiceListType {
    pub Service: C2RustUnnamed_100,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_100 {
    pub array: [iso20_ServiceType; 8],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_ServiceParameterListType {
    pub ParameterSet: C2RustUnnamed_101,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_101 {
    pub array: [iso20_ParameterSetType; 4],
    pub arrayLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_Scheduled_SEReqControlModeType {
    pub DepartureTime: u32,
    #[bitfield(name = "DepartureTime_isUsed", ty = "u32", bits = "0..=0")]
    pub DepartureTime_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub EVTargetEnergyRequest: iso20_RationalNumberType,
    #[bitfield(name = "EVTargetEnergyRequest_isUsed", ty = "u32", bits = "0..=0")]
    pub EVTargetEnergyRequest_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub EVMaximumEnergyRequest: iso20_RationalNumberType,
    #[bitfield(name = "EVMaximumEnergyRequest_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumEnergyRequest_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub EVMinimumEnergyRequest: iso20_RationalNumberType,
    #[bitfield(name = "EVMinimumEnergyRequest_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumEnergyRequest_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub EVEnergyOffer: iso20_EVEnergyOfferType,
    #[bitfield(name = "EVEnergyOffer_isUsed", ty = "u32", bits = "0..=0")]
    pub EVEnergyOffer_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 7],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_EVPowerProfileType {
    pub TimeAnchor: u64,
    pub Dynamic_EVPPTControlMode: iso20_Dynamic_EVPPTControlModeType,
    #[bitfield(name = "Dynamic_EVPPTControlMode_isUsed", ty = "u32", bits = "0..=0")]
    pub Dynamic_EVPPTControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub Scheduled_EVPPTControlMode: iso20_Scheduled_EVPPTControlModeType,
    #[bitfield(name = "Scheduled_EVPPTControlMode_isUsed", ty = "u32", bits = "0..=0")]
    pub Scheduled_EVPPTControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
    pub EVPowerProfileEntries: iso20_EVPowerProfileEntryListType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_CertificateChainType {
    pub Certificate: C2RustUnnamed_102,
    pub SubCertificates: iso20_SubCertificatesType,
    #[bitfield(name = "SubCertificates_isUsed", ty = "u32", bits = "0..=0")]
    pub SubCertificates_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_102 {
    pub bytes: [u8; 1600],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_EIM_ASResAuthorizationModeType {
    pub _unused: i32,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_Dynamic_SEResControlModeType {
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
    pub c2rust_padding: [u8; 7],
    pub AbsolutePriceSchedule: iso20_AbsolutePriceScheduleType,
    #[bitfield(name = "AbsolutePriceSchedule_isUsed", ty = "u32", bits = "0..=0")]
    pub AbsolutePriceSchedule_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 7],
    pub PriceLevelSchedule: iso20_PriceLevelScheduleType,
    #[bitfield(name = "PriceLevelSchedule_isUsed", ty = "u32", bits = "0..=0")]
    pub PriceLevelSchedule_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 7],
}
#[derive(Copy, Clone)]

pub struct iso20_EMAIDListType {
    pub EMAID: C2RustUnnamed_103,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_103 {
    pub array: [C2RustUnnamed_104; 8],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_104 {
    pub characters: [i8; 256],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_SignedInstallationDataType {
    pub Id: C2RustUnnamed_109,
    pub ContractCertificateChain: iso20_ContractCertificateChainType,
    pub ECDHCurve: iso20_ecdhCurveType,
    pub DHPublicKey: C2RustUnnamed_108,
    pub SECP521_EncryptedPrivateKey: C2RustUnnamed_107,
    #[bitfield(
        name = "SECP521_EncryptedPrivateKey_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub SECP521_EncryptedPrivateKey_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub X448_EncryptedPrivateKey: C2RustUnnamed_106,
    #[bitfield(name = "X448_EncryptedPrivateKey_isUsed", ty = "u32", bits = "0..=0")]
    pub X448_EncryptedPrivateKey_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub TPM_EncryptedPrivateKey: C2RustUnnamed_105,
    #[bitfield(name = "TPM_EncryptedPrivateKey_isUsed", ty = "u32", bits = "0..=0")]
    pub TPM_EncryptedPrivateKey_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_105 {
    pub bytes: [u8; 206],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_106 {
    pub bytes: [u8; 84],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_107 {
    pub bytes: [u8; 94],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_108 {
    pub bytes: [u8; 133],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_109 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_PnC_ASResAuthorizationModeType {
    pub GenChallenge: C2RustUnnamed_110,
    pub SupportedProviders: iso20_SupportedProvidersListType,
    #[bitfield(name = "SupportedProviders_isUsed", ty = "u32", bits = "0..=0")]
    pub SupportedProviders_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_110 {
    pub bytes: [u8; 16],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_Scheduled_SEResControlModeType {
    pub ScheduleTuple: C2RustUnnamed_111,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_111 {
    pub array: [iso20_ScheduleTupleType; 3],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_SessionSetupReqType {
    pub Header: iso20_MessageHeaderType,
    pub EVCCID: C2RustUnnamed_112,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_112 {
    pub characters: [i8; 256],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_SessionSetupResType {
    pub Header: iso20_MessageHeaderType,
    pub ResponseCode: iso20_responseCodeType,
    pub EVSEID: C2RustUnnamed_113,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_113 {
    pub characters: [i8; 256],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_AuthorizationSetupReqType {
    pub Header: iso20_MessageHeaderType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_AuthorizationSetupResType {
    pub Header: iso20_MessageHeaderType,
    pub ResponseCode: iso20_responseCodeType,
    pub AuthorizationServices: C2RustUnnamed_114,
    pub CertificateInstallationService: i32,
    pub EIM_ASResAuthorizationMode: iso20_EIM_ASResAuthorizationModeType,
    #[bitfield(name = "EIM_ASResAuthorizationMode_isUsed", ty = "u32", bits = "0..=0")]
    pub EIM_ASResAuthorizationMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub PnC_ASResAuthorizationMode: iso20_PnC_ASResAuthorizationModeType,
    #[bitfield(name = "PnC_ASResAuthorizationMode_isUsed", ty = "u32", bits = "0..=0")]
    pub PnC_ASResAuthorizationMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_114 {
    pub array: [iso20_authorizationType; 2],
    pub arrayLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_AuthorizationReqType {
    pub Header: iso20_MessageHeaderType,
    pub SelectedAuthorizationService: iso20_authorizationType,
    pub EIM_AReqAuthorizationMode: iso20_EIM_AReqAuthorizationModeType,
    #[bitfield(name = "EIM_AReqAuthorizationMode_isUsed", ty = "u32", bits = "0..=0")]
    pub EIM_AReqAuthorizationMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub PnC_AReqAuthorizationMode: iso20_PnC_AReqAuthorizationModeType,
    #[bitfield(name = "PnC_AReqAuthorizationMode_isUsed", ty = "u32", bits = "0..=0")]
    pub PnC_AReqAuthorizationMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 5],
}
#[derive(Copy, Clone)]

pub struct iso20_AuthorizationResType {
    pub Header: iso20_MessageHeaderType,
    pub ResponseCode: iso20_responseCodeType,
    pub EVSEProcessing: iso20_processingType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ServiceDiscoveryReqType {
    pub Header: iso20_MessageHeaderType,
    pub SupportedServiceIDs: iso20_ServiceIDListType,
    #[bitfield(name = "SupportedServiceIDs_isUsed", ty = "u32", bits = "0..=0")]
    pub SupportedServiceIDs_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 5],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ServiceDiscoveryResType {
    pub Header: iso20_MessageHeaderType,
    pub ResponseCode: iso20_responseCodeType,
    pub ServiceRenegotiationSupported: i32,
    pub EnergyTransferServiceList: iso20_ServiceListType,
    pub VASList: iso20_ServiceListType,
    #[bitfield(name = "VASList_isUsed", ty = "u32", bits = "0..=0")]
    pub VASList_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]

pub struct iso20_ServiceDetailReqType {
    pub Header: iso20_MessageHeaderType,
    pub ServiceID: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_ServiceDetailResType {
    pub Header: iso20_MessageHeaderType,
    pub ResponseCode: iso20_responseCodeType,
    pub ServiceID: u16,
    pub ServiceParameterList: iso20_ServiceParameterListType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ServiceSelectionReqType {
    pub Header: iso20_MessageHeaderType,
    pub SelectedEnergyTransferService: iso20_SelectedServiceType,
    pub SelectedVASList: iso20_SelectedServiceListType,
    #[bitfield(name = "SelectedVASList_isUsed", ty = "u32", bits = "0..=0")]
    pub SelectedVASList_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]

pub struct iso20_ServiceSelectionResType {
    pub Header: iso20_MessageHeaderType,
    pub ResponseCode: iso20_responseCodeType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ScheduleExchangeReqType {
    pub Header: iso20_MessageHeaderType,
    pub MaximumSupportingPoints: u16,
    pub Dynamic_SEReqControlMode: iso20_Dynamic_SEReqControlModeType,
    #[bitfield(name = "Dynamic_SEReqControlMode_isUsed", ty = "u32", bits = "0..=0")]
    pub Dynamic_SEReqControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub Scheduled_SEReqControlMode: iso20_Scheduled_SEReqControlModeType,
    #[bitfield(name = "Scheduled_SEReqControlMode_isUsed", ty = "u32", bits = "0..=0")]
    pub Scheduled_SEReqControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 7],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ScheduleExchangeResType {
    pub Header: iso20_MessageHeaderType,
    pub ResponseCode: iso20_responseCodeType,
    pub EVSEProcessing: iso20_processingType,
    pub GoToPause: i32,
    #[bitfield(name = "GoToPause_isUsed", ty = "u32", bits = "0..=0")]
    pub GoToPause_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub Dynamic_SEResControlMode: iso20_Dynamic_SEResControlModeType,
    #[bitfield(name = "Dynamic_SEResControlMode_isUsed", ty = "u32", bits = "0..=0")]
    pub Dynamic_SEResControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 7],
    pub Scheduled_SEResControlMode: iso20_Scheduled_SEResControlModeType,
    #[bitfield(name = "Scheduled_SEResControlMode_isUsed", ty = "u32", bits = "0..=0")]
    pub Scheduled_SEResControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 7],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_PowerDeliveryReqType {
    pub Header: iso20_MessageHeaderType,
    pub EVProcessing: iso20_processingType,
    pub ChargeProgress: iso20_chargeProgressType,
    pub EVPowerProfile: iso20_EVPowerProfileType,
    #[bitfield(name = "EVPowerProfile_isUsed", ty = "u32", bits = "0..=0")]
    pub EVPowerProfile_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub BPT_ChannelSelection: iso20_channelSelectionType,
    #[bitfield(name = "BPT_ChannelSelection_isUsed", ty = "u32", bits = "0..=0")]
    pub BPT_ChannelSelection_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 7],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_PowerDeliveryResType {
    pub Header: iso20_MessageHeaderType,
    pub ResponseCode: iso20_responseCodeType,
    pub EVSEStatus: iso20_EVSEStatusType,
    #[bitfield(name = "EVSEStatus_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEStatus_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]

pub struct iso20_MeteringConfirmationReqType {
    pub Header: iso20_MessageHeaderType,
    pub SignedMeteringData: iso20_SignedMeteringDataType,
}
#[derive(Copy, Clone)]

pub struct iso20_MeteringConfirmationResType {
    pub Header: iso20_MessageHeaderType,
    pub ResponseCode: iso20_responseCodeType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_SessionStopReqType {
    pub Header: iso20_MessageHeaderType,
    pub ChargingSession: iso20_chargingSessionType,
    pub EVTerminationCode: C2RustUnnamed_116,
    #[bitfield(name = "EVTerminationCode_isUsed", ty = "u32", bits = "0..=0")]
    pub EVTerminationCode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub EVTerminationExplanation: C2RustUnnamed_115,
    #[bitfield(name = "EVTerminationExplanation_isUsed", ty = "u32", bits = "0..=0")]
    pub EVTerminationExplanation_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_115 {
    pub characters: [i8; 161],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_116 {
    pub characters: [i8; 81],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_SessionStopResType {
    pub Header: iso20_MessageHeaderType,
    pub ResponseCode: iso20_responseCodeType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_CertificateInstallationReqType {
    pub Header: iso20_MessageHeaderType,
    pub OEMProvisioningCertificateChain: iso20_SignedCertificateChainType,
    pub ListOfRootCertificateIDs: iso20_ListOfRootCertificateIDsType,
    pub MaximumContractCertificateChains: u8,
    pub PrioritizedEMAIDs: iso20_EMAIDListType,
    #[bitfield(name = "PrioritizedEMAIDs_isUsed", ty = "u32", bits = "0..=0")]
    pub PrioritizedEMAIDs_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]

pub struct iso20_CertificateInstallationResType {
    pub Header: iso20_MessageHeaderType,
    pub ResponseCode: iso20_responseCodeType,
    pub EVSEProcessing: iso20_processingType,
    pub CPSCertificateChain: iso20_CertificateChainType,
    pub SignedInstallationData: iso20_SignedInstallationDataType,
    pub RemainingContractCertificateChains: u8,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_VehicleCheckInReqType {
    pub Header: iso20_MessageHeaderType,
    pub EVCheckInStatus: iso20_evCheckInStatusType,
    pub ParkingMethod: iso20_parkingMethodType,
    pub VehicleFrame: i16,
    #[bitfield(name = "VehicleFrame_isUsed", ty = "u32", bits = "0..=0")]
    pub VehicleFrame_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub DeviceOffset: i16,
    #[bitfield(name = "DeviceOffset_isUsed", ty = "u32", bits = "0..=0")]
    pub DeviceOffset_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub VehicleTravel: i16,
    #[bitfield(name = "VehicleTravel_isUsed", ty = "u32", bits = "0..=0")]
    pub VehicleTravel_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 5],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_VehicleCheckInResType {
    pub Header: iso20_MessageHeaderType,
    pub ResponseCode: iso20_responseCodeType,
    pub ParkingSpace: i16,
    #[bitfield(name = "ParkingSpace_isUsed", ty = "u32", bits = "0..=0")]
    pub ParkingSpace_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub DeviceLocation: i16,
    #[bitfield(name = "DeviceLocation_isUsed", ty = "u32", bits = "0..=0")]
    pub DeviceLocation_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub TargetDistance: i16,
    #[bitfield(name = "TargetDistance_isUsed", ty = "u32", bits = "0..=0")]
    pub TargetDistance_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
}
#[derive(Copy, Clone)]

pub struct iso20_VehicleCheckOutReqType {
    pub Header: iso20_MessageHeaderType,
    pub EVCheckOutStatus: iso20_evCheckOutStatusType,
    pub CheckOutTime: u64,
}
#[derive(Copy, Clone)]

pub struct iso20_VehicleCheckOutResType {
    pub Header: iso20_MessageHeaderType,
    pub ResponseCode: iso20_responseCodeType,
    pub EVSECheckOutStatus: iso20_evseCheckOutStatusType,
}
#[derive(Copy, Clone)]

pub struct iso20_CLReqControlModeType {
    pub _unused: i32,
}
#[derive(Copy, Clone)]

pub struct iso20_CLResControlModeType {
    pub _unused: i32,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_ManifestType {
    pub Id: C2RustUnnamed_118,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub Reference: C2RustUnnamed_117,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_117 {
    pub array: [iso20_ReferenceType; 4],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_118 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_SignaturePropertiesType {
    pub Id: C2RustUnnamed_119,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub SignatureProperty: iso20_SignaturePropertyType,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_119 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_exiDocument {
    pub c2rust_unnamed: C2RustUnnamed_120,
    #[bitfield(name = "SessionSetupReq_isUsed", ty = "u32", bits = "0..=0")]
    #[bitfield(name = "SessionSetupRes_isUsed", ty = "u32", bits = "1..=1")]
    #[bitfield(name = "AuthorizationSetupReq_isUsed", ty = "u32", bits = "2..=2")]
    #[bitfield(name = "AuthorizationSetupRes_isUsed", ty = "u32", bits = "3..=3")]
    #[bitfield(name = "AuthorizationReq_isUsed", ty = "u32", bits = "4..=4")]
    #[bitfield(name = "AuthorizationRes_isUsed", ty = "u32", bits = "5..=5")]
    #[bitfield(name = "ServiceDiscoveryReq_isUsed", ty = "u32", bits = "6..=6")]
    #[bitfield(name = "ServiceDiscoveryRes_isUsed", ty = "u32", bits = "7..=7")]
    #[bitfield(name = "ServiceDetailReq_isUsed", ty = "u32", bits = "8..=8")]
    #[bitfield(name = "ServiceDetailRes_isUsed", ty = "u32", bits = "9..=9")]
    #[bitfield(name = "ServiceSelectionReq_isUsed", ty = "u32", bits = "10..=10")]
    #[bitfield(name = "ServiceSelectionRes_isUsed", ty = "u32", bits = "11..=11")]
    #[bitfield(name = "ScheduleExchangeReq_isUsed", ty = "u32", bits = "12..=12")]
    #[bitfield(name = "ScheduleExchangeRes_isUsed", ty = "u32", bits = "13..=13")]
    #[bitfield(name = "PowerDeliveryReq_isUsed", ty = "u32", bits = "14..=14")]
    #[bitfield(name = "PowerDeliveryRes_isUsed", ty = "u32", bits = "15..=15")]
    #[bitfield(name = "MeteringConfirmationReq_isUsed", ty = "u32", bits = "16..=16")]
    #[bitfield(name = "MeteringConfirmationRes_isUsed", ty = "u32", bits = "17..=17")]
    #[bitfield(name = "SessionStopReq_isUsed", ty = "u32", bits = "18..=18")]
    #[bitfield(name = "SessionStopRes_isUsed", ty = "u32", bits = "19..=19")]
    #[bitfield(
        name = "CertificateInstallationReq_isUsed",
        ty = "u32",
        bits = "20..=20"
    )]
    #[bitfield(
        name = "CertificateInstallationRes_isUsed",
        ty = "u32",
        bits = "21..=21"
    )]
    #[bitfield(name = "VehicleCheckInReq_isUsed", ty = "u32", bits = "22..=22")]
    #[bitfield(name = "VehicleCheckInRes_isUsed", ty = "u32", bits = "23..=23")]
    #[bitfield(name = "VehicleCheckOutReq_isUsed", ty = "u32", bits = "24..=24")]
    #[bitfield(name = "VehicleCheckOutRes_isUsed", ty = "u32", bits = "25..=25")]
    #[bitfield(name = "SignedInstallationData_isUsed", ty = "u32", bits = "26..=26")]
    #[bitfield(name = "SignedMeteringData_isUsed", ty = "u32", bits = "27..=27")]
    #[bitfield(name = "CLReqControlMode_isUsed", ty = "u32", bits = "28..=28")]
    #[bitfield(name = "CLResControlMode_isUsed", ty = "u32", bits = "29..=29")]
    #[bitfield(name = "Signature_isUsed", ty = "u32", bits = "30..=30")]
    #[bitfield(name = "SignatureValue_isUsed", ty = "u32", bits = "31..=31")]
    #[bitfield(name = "SignedInfo_isUsed", ty = "u32", bits = "32..=32")]
    #[bitfield(name = "CanonicalizationMethod_isUsed", ty = "u32", bits = "33..=33")]
    #[bitfield(name = "SignatureMethod_isUsed", ty = "u32", bits = "34..=34")]
    #[bitfield(name = "Reference_isUsed", ty = "u32", bits = "35..=35")]
    #[bitfield(name = "Transforms_isUsed", ty = "u32", bits = "36..=36")]
    #[bitfield(name = "Transform_isUsed", ty = "u32", bits = "37..=37")]
    #[bitfield(name = "DigestMethod_isUsed", ty = "u32", bits = "38..=38")]
    #[bitfield(name = "KeyInfo_isUsed", ty = "u32", bits = "39..=39")]
    #[bitfield(name = "KeyValue_isUsed", ty = "u32", bits = "40..=40")]
    #[bitfield(name = "RetrievalMethod_isUsed", ty = "u32", bits = "41..=41")]
    #[bitfield(name = "X509Data_isUsed", ty = "u32", bits = "42..=42")]
    #[bitfield(name = "PGPData_isUsed", ty = "u32", bits = "43..=43")]
    #[bitfield(name = "SPKIData_isUsed", ty = "u32", bits = "44..=44")]
    #[bitfield(name = "Object_isUsed", ty = "u32", bits = "45..=45")]
    #[bitfield(name = "Manifest_isUsed", ty = "u32", bits = "46..=46")]
    #[bitfield(name = "SignatureProperties_isUsed", ty = "u32", bits = "47..=47")]
    #[bitfield(name = "SignatureProperty_isUsed", ty = "u32", bits = "48..=48")]
    #[bitfield(name = "DSAKeyValue_isUsed", ty = "u32", bits = "49..=49")]
    #[bitfield(name = "RSAKeyValue_isUsed", ty = "u32", bits = "50..=50")]
    pub SessionSetupReq_isUsed_SessionSetupRes_isUsed_AuthorizationSetupReq_isUsed_AuthorizationSetupRes_isUsed_AuthorizationReq_isUsed_AuthorizationRes_isUsed_ServiceDiscoveryReq_isUsed_ServiceDiscoveryRes_isUsed_ServiceDetailReq_isUsed_ServiceDetailRes_isUsed_ServiceSelectionReq_isUsed_ServiceSelectionRes_isUsed_ScheduleExchangeReq_isUsed_ScheduleExchangeRes_isUsed_PowerDeliveryReq_isUsed_PowerDeliveryRes_isUsed_MeteringConfirmationReq_isUsed_MeteringConfirmationRes_isUsed_SessionStopReq_isUsed_SessionStopRes_isUsed_CertificateInstallationReq_isUsed_CertificateInstallationRes_isUsed_VehicleCheckInReq_isUsed_VehicleCheckInRes_isUsed_VehicleCheckOutReq_isUsed_VehicleCheckOutRes_isUsed_SignedInstallationData_isUsed_SignedMeteringData_isUsed_CLReqControlMode_isUsed_CLResControlMode_isUsed_Signature_isUsed_SignatureValue_isUsed_SignedInfo_isUsed_CanonicalizationMethod_isUsed_SignatureMethod_isUsed_Reference_isUsed_Transforms_isUsed_Transform_isUsed_DigestMethod_isUsed_KeyInfo_isUsed_KeyValue_isUsed_RetrievalMethod_isUsed_X509Data_isUsed_PGPData_isUsed_SPKIData_isUsed_Object_isUsed_Manifest_isUsed_SignatureProperties_isUsed_SignatureProperty_isUsed_DSAKeyValue_isUsed_RSAKeyValue_isUsed:
        [u8; 7],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]

pub union C2RustUnnamed_120 {
    pub SessionSetupReq: iso20_SessionSetupReqType,
    pub SessionSetupRes: iso20_SessionSetupResType,
    pub AuthorizationSetupReq: iso20_AuthorizationSetupReqType,
    pub AuthorizationSetupRes: iso20_AuthorizationSetupResType,
    pub AuthorizationReq: iso20_AuthorizationReqType,
    pub AuthorizationRes: iso20_AuthorizationResType,
    pub ServiceDiscoveryReq: iso20_ServiceDiscoveryReqType,
    pub ServiceDiscoveryRes: iso20_ServiceDiscoveryResType,
    pub ServiceDetailReq: iso20_ServiceDetailReqType,
    pub ServiceDetailRes: iso20_ServiceDetailResType,
    pub ServiceSelectionReq: iso20_ServiceSelectionReqType,
    pub ServiceSelectionRes: iso20_ServiceSelectionResType,
    pub ScheduleExchangeReq: iso20_ScheduleExchangeReqType,
    pub ScheduleExchangeRes: iso20_ScheduleExchangeResType,
    pub PowerDeliveryReq: iso20_PowerDeliveryReqType,
    pub PowerDeliveryRes: iso20_PowerDeliveryResType,
    pub MeteringConfirmationReq: iso20_MeteringConfirmationReqType,
    pub MeteringConfirmationRes: iso20_MeteringConfirmationResType,
    pub SessionStopReq: iso20_SessionStopReqType,
    pub SessionStopRes: iso20_SessionStopResType,
    pub CertificateInstallationReq: iso20_CertificateInstallationReqType,
    pub CertificateInstallationRes: iso20_CertificateInstallationResType,
    pub VehicleCheckInReq: iso20_VehicleCheckInReqType,
    pub VehicleCheckInRes: iso20_VehicleCheckInResType,
    pub VehicleCheckOutReq: iso20_VehicleCheckOutReqType,
    pub VehicleCheckOutRes: iso20_VehicleCheckOutResType,
    pub SignedInstallationData: iso20_SignedInstallationDataType,
    pub SignedMeteringData: iso20_SignedMeteringDataType,
    pub CLReqControlMode: iso20_CLReqControlModeType,
    pub CLResControlMode: iso20_CLResControlModeType,
    pub Signature: iso20_SignatureType,
    pub SignatureValue: iso20_SignatureValueType,
    pub SignedInfo: iso20_SignedInfoType,
    pub CanonicalizationMethod: iso20_CanonicalizationMethodType,
    pub SignatureMethod: iso20_SignatureMethodType,
    pub Reference: iso20_ReferenceType,
    pub Transforms: iso20_TransformsType,
    pub Transform: iso20_TransformType,
    pub DigestMethod: iso20_DigestMethodType,
    pub KeyInfo: iso20_KeyInfoType,
    pub KeyValue: iso20_KeyValueType,
    pub RetrievalMethod: iso20_RetrievalMethodType,
    pub X509Data: iso20_X509DataType,
    pub PGPData: iso20_PGPDataType,
    pub SPKIData: iso20_SPKIDataType,
    pub Object: iso20_ObjectType,
    pub Manifest: iso20_ManifestType,
    pub SignatureProperties: iso20_SignaturePropertiesType,
    pub SignatureProperty: iso20_SignaturePropertyType,
    pub DSAKeyValue: iso20_DSAKeyValueType,
    pub RSAKeyValue: iso20_RSAKeyValueType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_exiFragment {
    pub c2rust_unnamed: C2RustUnnamed_121,
    #[bitfield(name = "AbsolutePriceSchedule_isUsed", ty = "u32", bits = "0..=0")]
    #[bitfield(name = "CertificateInstallationReq_isUsed", ty = "u32", bits = "1..=1")]
    #[bitfield(name = "MeteringConfirmationReq_isUsed", ty = "u32", bits = "2..=2")]
    #[bitfield(name = "PnC_AReqAuthorizationMode_isUsed", ty = "u32", bits = "3..=3")]
    #[bitfield(name = "SignedInfo_isUsed", ty = "u32", bits = "4..=4")]
    #[bitfield(name = "SignedInstallationData_isUsed", ty = "u32", bits = "5..=5")]
    pub AbsolutePriceSchedule_isUsed_CertificateInstallationReq_isUsed_MeteringConfirmationReq_isUsed_PnC_AReqAuthorizationMode_isUsed_SignedInfo_isUsed_SignedInstallationData_isUsed:
        [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]

pub union C2RustUnnamed_121 {
    pub AbsolutePriceSchedule: iso20_AbsolutePriceScheduleType,
    pub CertificateInstallationReq: iso20_CertificateInstallationReqType,
    pub MeteringConfirmationReq: iso20_MeteringConfirmationReqType,
    pub PnC_AReqAuthorizationMode: iso20_PnC_AReqAuthorizationModeType,
    pub SignedInfo: iso20_SignedInfoType,
    pub SignedInstallationData: iso20_SignedInstallationDataType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_xmldsigFragment {
    pub c2rust_unnamed: C2RustUnnamed_122,
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

pub union C2RustUnnamed_122 {
    pub CanonicalizationMethod: iso20_CanonicalizationMethodType,
    pub DSAKeyValue: iso20_DSAKeyValueType,
    pub DigestMethod: iso20_DigestMethodType,
    pub KeyInfo: iso20_KeyInfoType,
    pub KeyValue: iso20_KeyValueType,
    pub Manifest: iso20_ManifestType,
    pub Object: iso20_ObjectType,
    pub PGPData: iso20_PGPDataType,
    pub RSAKeyValue: iso20_RSAKeyValueType,
    pub Reference: iso20_ReferenceType,
    pub RetrievalMethod: iso20_RetrievalMethodType,
    pub SPKIData: iso20_SPKIDataType,
    pub Signature: iso20_SignatureType,
    pub SignatureMethod: iso20_SignatureMethodType,
    pub SignatureProperties: iso20_SignaturePropertiesType,
    pub SignatureProperty: iso20_SignaturePropertyType,
    pub SignatureValue: iso20_SignatureValueType,
    pub SignedInfo: iso20_SignedInfoType,
    pub Transform: iso20_TransformType,
    pub Transforms: iso20_TransformsType,
    pub X509Data: iso20_X509DataType,
    pub X509IssuerSerial: iso20_X509IssuerSerialType,
}
unsafe extern "C" fn encode_iso20_TransformType(
    stream: &mut ExiBitstream,
    mut TransformType: *const iso20_TransformType,
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
unsafe extern "C" fn encode_iso20_TransformsType(
    stream: &mut ExiBitstream,
    mut TransformsType: *const iso20_TransformsType,
) -> i32 {
    let mut grammar_id: i32 = 4 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            4 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_TransformType(stream, &(*TransformsType).Transform);
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
                        error = encode_iso20_TransformType(stream, &(*TransformsType).Transform);
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
unsafe extern "C" fn encode_iso20_DSAKeyValueType(
    stream: &mut ExiBitstream,
    mut DSAKeyValueType: *const iso20_DSAKeyValueType,
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
unsafe extern "C" fn encode_iso20_X509IssuerSerialType(
    stream: &mut ExiBitstream,
    mut X509IssuerSerialType: *const iso20_X509IssuerSerialType,
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
unsafe extern "C" fn encode_iso20_DigestMethodType(
    stream: &mut ExiBitstream,
    mut DigestMethodType: *const iso20_DigestMethodType,
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
unsafe extern "C" fn encode_iso20_RSAKeyValueType(
    stream: &mut ExiBitstream,
    mut RSAKeyValueType: *const iso20_RSAKeyValueType,
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
unsafe extern "C" fn encode_iso20_CanonicalizationMethodType(
    stream: &mut ExiBitstream,
    mut CanonicalizationMethodType: *const iso20_CanonicalizationMethodType,
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
unsafe extern "C" fn encode_iso20_PriceLevelScheduleEntryType(
    stream: &mut ExiBitstream,
    mut PriceLevelScheduleEntryType: *const iso20_PriceLevelScheduleEntryType,
) -> i32 {
    let mut grammar_id: i32 = 21 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            21 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_32(
                            stream,
                            (*PriceLevelScheduleEntryType).Duration,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 22 as i32;
                            }
                        }
                    }
                }
            }
            22 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            8 as i32 as usize,
                            (*PriceLevelScheduleEntryType).PriceLevel as u32,
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
unsafe extern "C" fn encode_iso20_SignatureMethodType(
    stream: &mut ExiBitstream,
    mut SignatureMethodType: *const iso20_SignatureMethodType,
) -> i32 {
    let mut grammar_id: i32 = 23 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            23 => {
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
                            grammar_id = 24 as i32;
                        }
                    }
                }
            }
            24 => {
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
                                    grammar_id = 25 as i32;
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
            25 => {
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
unsafe extern "C" fn encode_iso20_KeyValueType(
    stream: &mut ExiBitstream,
    mut KeyValueType: *const iso20_KeyValueType,
) -> i32 {
    let mut grammar_id: i32 = 26 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            26 => {
                if (*KeyValueType).DSAKeyValue_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_DSAKeyValueType(stream, &(*KeyValueType).DSAKeyValue);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyValueType).RSAKeyValue_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RSAKeyValueType(stream, &(*KeyValueType).RSAKeyValue);
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
unsafe extern "C" fn encode_iso20_ReferenceType(
    stream: &mut ExiBitstream,
    mut ReferenceType: *const iso20_ReferenceType,
) -> i32 {
    let mut grammar_id: i32 = 27 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            27 => {
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
                                grammar_id = 28 as i32;
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
                                grammar_id = 29 as i32;
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
                                grammar_id = 30 as i32;
                            }
                        }
                    }
                } else if (*ReferenceType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_TransformsType(stream, &(*ReferenceType).Transforms);
                        if error == 0 as i32 {
                            grammar_id = 31 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_DigestMethodType(stream, &(*ReferenceType).DigestMethod);
                        if error == 0 as i32 {
                            grammar_id = 32 as i32;
                        }
                    }
                }
            }
            28 => {
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
                                grammar_id = 29 as i32;
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
                                grammar_id = 30 as i32;
                            }
                        }
                    }
                } else if (*ReferenceType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_TransformsType(stream, &(*ReferenceType).Transforms);
                        if error == 0 as i32 {
                            grammar_id = 31 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_DigestMethodType(stream, &(*ReferenceType).DigestMethod);
                        if error == 0 as i32 {
                            grammar_id = 32 as i32;
                        }
                    }
                }
            }
            29 => {
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
                                grammar_id = 30 as i32;
                            }
                        }
                    }
                } else if (*ReferenceType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_TransformsType(stream, &(*ReferenceType).Transforms);
                        if error == 0 as i32 {
                            grammar_id = 31 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_DigestMethodType(stream, &(*ReferenceType).DigestMethod);
                        if error == 0 as i32 {
                            grammar_id = 32 as i32;
                        }
                    }
                }
            }
            30 => {
                if (*ReferenceType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_TransformsType(stream, &(*ReferenceType).Transforms);
                        if error == 0 as i32 {
                            grammar_id = 31 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_DigestMethodType(stream, &(*ReferenceType).DigestMethod);
                        if error == 0 as i32 {
                            grammar_id = 32 as i32;
                        }
                    }
                }
            }
            31 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_DigestMethodType(stream, &(*ReferenceType).DigestMethod);
                    if error == 0 as i32 {
                        grammar_id = 32 as i32;
                    }
                }
            }
            32 => {
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
unsafe extern "C" fn encode_iso20_RetrievalMethodType(
    stream: &mut ExiBitstream,
    mut RetrievalMethodType: *const iso20_RetrievalMethodType,
) -> i32 {
    let mut grammar_id: i32 = 33 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            33 => {
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
                                grammar_id = 34 as i32;
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
                                grammar_id = 35 as i32;
                            }
                        }
                    }
                } else if (*RetrievalMethodType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_TransformsType(stream, &(*RetrievalMethodType).Transforms);
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
            34 => {
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
                                grammar_id = 35 as i32;
                            }
                        }
                    }
                } else if (*RetrievalMethodType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_TransformsType(stream, &(*RetrievalMethodType).Transforms);
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
            35 => {
                if (*RetrievalMethodType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_TransformsType(stream, &(*RetrievalMethodType).Transforms);
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
unsafe extern "C" fn encode_iso20_X509DataType(
    stream: &mut ExiBitstream,
    mut X509DataType: *const iso20_X509DataType,
) -> i32 {
    let mut grammar_id: i32 = 36 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            36 => {
                if (*X509DataType).X509IssuerSerial_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_X509IssuerSerialType(
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
unsafe extern "C" fn encode_iso20_PGPDataType(
    stream: &mut ExiBitstream,
    mut PGPDataType: *const iso20_PGPDataType,
) -> i32 {
    let mut grammar_id: i32 = 37 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            37 => {
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
                                        grammar_id = 39 as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            38 => {
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
                                        grammar_id = 39 as i32;
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
                                        grammar_id = 40 as i32;
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
            39 => {
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
                                        grammar_id = 40 as i32;
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
            40 => {
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
                                    grammar_id = 41 as i32;
                                }
                            }
                        }
                    }
                }
            }
            41 => {
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
                                        grammar_id = 40 as i32;
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
unsafe extern "C" fn encode_iso20_RationalNumberType(
    stream: &mut ExiBitstream,
    mut RationalNumberType: *const iso20_RationalNumberType,
) -> i32 {
    let mut grammar_id: i32 = 42 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            42 => {
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
                                grammar_id = 43 as i32;
                            }
                        }
                    }
                }
            }
            43 => {
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
unsafe extern "C" fn encode_iso20_PowerScheduleEntryType(
    stream: &mut ExiBitstream,
    mut PowerScheduleEntryType: *const iso20_PowerScheduleEntryType,
) -> i32 {
    let mut grammar_id: i32 = 44 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            44 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_32(
                            stream,
                            (*PowerScheduleEntryType).Duration,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 45 as i32;
                            }
                        }
                    }
                }
            }
            45 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso20_RationalNumberType(stream, &(*PowerScheduleEntryType).Power);
                    if error == 0 as i32 {
                        grammar_id = 46 as i32;
                    }
                }
            }
            46 => {
                if (*PowerScheduleEntryType).Power_L2_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*PowerScheduleEntryType).Power_L2,
                        );
                        if error == 0 as i32 {
                            grammar_id = 47 as i32;
                        }
                    }
                } else if (*PowerScheduleEntryType).Power_L3_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*PowerScheduleEntryType).Power_L3,
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
            47 => {
                if (*PowerScheduleEntryType).Power_L3_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*PowerScheduleEntryType).Power_L3,
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
unsafe extern "C" fn encode_iso20_EVPriceRuleType(
    stream: &mut ExiBitstream,
    mut EVPriceRuleType: *const iso20_EVPriceRuleType,
) -> i32 {
    let mut grammar_id: i32 = 48 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            48 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_RationalNumberType(stream, &(*EVPriceRuleType).EnergyFee);
                    if error == 0 as i32 {
                        grammar_id = 49 as i32;
                    }
                }
            }
            49 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_RationalNumberType(
                        stream,
                        &(*EVPriceRuleType).PowerRangeStart,
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
unsafe extern "C" fn encode_iso20_EVPowerScheduleEntryType(
    stream: &mut ExiBitstream,
    mut EVPowerScheduleEntryType: *const iso20_EVPowerScheduleEntryType,
) -> i32 {
    let mut grammar_id: i32 = 50 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            50 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_32(
                            stream,
                            (*EVPowerScheduleEntryType).Duration,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 51 as i32;
                            }
                        }
                    }
                }
            }
            51 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso20_RationalNumberType(stream, &(*EVPowerScheduleEntryType).Power);
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
unsafe extern "C" fn encode_iso20_EVPriceRuleStackType(
    stream: &mut ExiBitstream,
    mut EVPriceRuleStackType: *const iso20_EVPriceRuleStackType,
) -> i32 {
    let mut grammar_id: i32 = 52 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut EVPriceRule_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            52 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            exi_basetypes_encoder_uint_32(stream, (*EVPriceRuleStackType).Duration);
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 53 as i32;
                            }
                        }
                    }
                }
            }
            53 => {
                if (EVPriceRule_currentIndex as i32)
                    < (*EVPriceRuleStackType).EVPriceRule.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh0 = EVPriceRule_currentIndex;
                        EVPriceRule_currentIndex = EVPriceRule_currentIndex.wrapping_add(1);
                        error = encode_iso20_EVPriceRuleType(
                            stream,
                            &*((*EVPriceRuleStackType).EVPriceRule.array)
                                .as_ptr()
                                .offset(fresh0 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 54 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            54 => {
                if (EVPriceRule_currentIndex as i32)
                    < (*EVPriceRuleStackType).EVPriceRule.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh1 = EVPriceRule_currentIndex;
                        EVPriceRule_currentIndex = EVPriceRule_currentIndex.wrapping_add(1);
                        error = encode_iso20_EVPriceRuleType(
                            stream,
                            &*((*EVPriceRuleStackType).EVPriceRule.array)
                                .as_ptr()
                                .offset(fresh1 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 54 as i32;
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
unsafe extern "C" fn encode_iso20_PriceRuleType(
    stream: &mut ExiBitstream,
    mut PriceRuleType: *const iso20_PriceRuleType,
) -> i32 {
    let mut grammar_id: i32 = 55 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            55 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_RationalNumberType(stream, &(*PriceRuleType).EnergyFee);
                    if error == 0 as i32 {
                        grammar_id = 56 as i32;
                    }
                }
            }
            56 => {
                if (*PriceRuleType).ParkingFee_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_RationalNumberType(stream, &(*PriceRuleType).ParkingFee);
                        if error == 0 as i32 {
                            grammar_id = 57 as i32;
                        }
                    }
                } else if (*PriceRuleType).ParkingFeePeriod_isUsed() == 1 as u32 {
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
                                (*PriceRuleType).ParkingFeePeriod,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 58 as i32;
                                }
                            }
                        }
                    }
                } else if (*PriceRuleType).CarbonDioxideEmission_isUsed() == 1 as u32 {
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
                                (*PriceRuleType).CarbonDioxideEmission,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 59 as i32;
                                }
                            }
                        }
                    }
                } else if (*PriceRuleType).RenewableGenerationPercentage_isUsed() == 1 as u32 {
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
                                8 as i32 as usize,
                                (*PriceRuleType).RenewableGenerationPercentage as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 60 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*PriceRuleType).PowerRangeStart,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                }
            }
            57 => {
                if (*PriceRuleType).ParkingFeePeriod_isUsed() == 1 as u32 {
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
                                (*PriceRuleType).ParkingFeePeriod,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 58 as i32;
                                }
                            }
                        }
                    }
                } else if (*PriceRuleType).CarbonDioxideEmission_isUsed() == 1 as u32 {
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
                                (*PriceRuleType).CarbonDioxideEmission,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 59 as i32;
                                }
                            }
                        }
                    }
                } else if (*PriceRuleType).RenewableGenerationPercentage_isUsed() == 1 as u32 {
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
                                8 as i32 as usize,
                                (*PriceRuleType).RenewableGenerationPercentage as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 60 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*PriceRuleType).PowerRangeStart,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                }
            }
            58 => {
                if (*PriceRuleType).CarbonDioxideEmission_isUsed() == 1 as u32 {
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
                                (*PriceRuleType).CarbonDioxideEmission,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 59 as i32;
                                }
                            }
                        }
                    }
                } else if (*PriceRuleType).RenewableGenerationPercentage_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                8 as i32 as usize,
                                (*PriceRuleType).RenewableGenerationPercentage as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 60 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*PriceRuleType).PowerRangeStart,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                }
            }
            59 => {
                if (*PriceRuleType).RenewableGenerationPercentage_isUsed() == 1 as u32 {
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
                                8 as i32 as usize,
                                (*PriceRuleType).RenewableGenerationPercentage as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 60 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*PriceRuleType).PowerRangeStart,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                }
            }
            60 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso20_RationalNumberType(stream, &(*PriceRuleType).PowerRangeStart);
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
unsafe extern "C" fn encode_iso20_PowerScheduleEntryListType(
    stream: &mut ExiBitstream,
    mut PowerScheduleEntryListType: *const iso20_PowerScheduleEntryListType,
) -> i32 {
    let mut grammar_id: i32 = 61 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut PowerScheduleEntry_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            61 => {
                if (PowerScheduleEntry_currentIndex as i32)
                    < (*PowerScheduleEntryListType).PowerScheduleEntry.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh2 = PowerScheduleEntry_currentIndex;
                        PowerScheduleEntry_currentIndex =
                            PowerScheduleEntry_currentIndex.wrapping_add(1);
                        error = encode_iso20_PowerScheduleEntryType(
                            stream,
                            &*((*PowerScheduleEntryListType).PowerScheduleEntry.array)
                                .as_ptr()
                                .offset(fresh2 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 62 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            62 => {
                if (PowerScheduleEntry_currentIndex as i32)
                    < (*PowerScheduleEntryListType).PowerScheduleEntry.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh3 = PowerScheduleEntry_currentIndex;
                        PowerScheduleEntry_currentIndex =
                            PowerScheduleEntry_currentIndex.wrapping_add(1);
                        error = encode_iso20_PowerScheduleEntryType(
                            stream,
                            &*((*PowerScheduleEntryListType).PowerScheduleEntry.array)
                                .as_ptr()
                                .offset(fresh3 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 62 as i32;
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
unsafe extern "C" fn encode_iso20_TaxRuleType(
    stream: &mut ExiBitstream,
    mut TaxRuleType: *const iso20_TaxRuleType,
) -> i32 {
    let mut grammar_id: i32 = 63 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            63 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_32(stream, (*TaxRuleType).TaxRuleID);
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 64 as i32;
                            }
                        }
                    }
                }
            }
            64 => {
                if (*TaxRuleType).TaxRuleName_isUsed() == 1 as u32 {
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
                                ((*TaxRuleType).TaxRuleName.charactersLen as i32 + 2 as i32) as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*TaxRuleType).TaxRuleName.charactersLen as usize,
                                    ((*TaxRuleType).TaxRuleName.characters).as_ptr(),
                                    (80 as i32 + 1 as i32) as usize,
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
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(stream, &(*TaxRuleType).TaxRate);
                        if error == 0 as i32 {
                            grammar_id = 66 as i32;
                        }
                    }
                }
            }
            65 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_RationalNumberType(stream, &(*TaxRuleType).TaxRate);
                    if error == 0 as i32 {
                        grammar_id = 66 as i32;
                    }
                }
            }
            66 => {
                if (*TaxRuleType).TaxIncludedInPrice_isUsed() == 1 as u32 {
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
                                (*TaxRuleType).TaxIncludedInPrice,
                            );
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
                                (*TaxRuleType).AppliesToEnergyFee,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 68 as i32;
                                }
                            }
                        }
                    }
                }
            }
            67 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            exi_basetypes_encoder_bool(stream, (*TaxRuleType).AppliesToEnergyFee);
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 68 as i32;
                            }
                        }
                    }
                }
            }
            68 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            exi_basetypes_encoder_bool(stream, (*TaxRuleType).AppliesToParkingFee);
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
            69 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            exi_basetypes_encoder_bool(stream, (*TaxRuleType).AppliesToOverstayFee);
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(
                            stream,
                            (*TaxRuleType).AppliesMinimumMaximumCost,
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
unsafe extern "C" fn encode_iso20_PriceRuleStackType(
    stream: &mut ExiBitstream,
    mut PriceRuleStackType: *const iso20_PriceRuleStackType,
) -> i32 {
    let mut grammar_id: i32 = 71 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut PriceRule_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            71 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            exi_basetypes_encoder_uint_32(stream, (*PriceRuleStackType).Duration);
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
                if (PriceRule_currentIndex as i32) < (*PriceRuleStackType).PriceRule.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh4 = PriceRule_currentIndex;
                        PriceRule_currentIndex = PriceRule_currentIndex.wrapping_add(1);
                        error = encode_iso20_PriceRuleType(
                            stream,
                            &*((*PriceRuleStackType).PriceRule.array)
                                .as_ptr()
                                .offset(fresh4 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 73 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            73 => {
                if (PriceRule_currentIndex as i32) < (*PriceRuleStackType).PriceRule.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh5 = PriceRule_currentIndex;
                        PriceRule_currentIndex = PriceRule_currentIndex.wrapping_add(1);
                        error = encode_iso20_PriceRuleType(
                            stream,
                            &*((*PriceRuleStackType).PriceRule.array)
                                .as_ptr()
                                .offset(fresh5 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 73 as i32;
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
unsafe extern "C" fn encode_iso20_AdditionalServiceType(
    stream: &mut ExiBitstream,
    mut AdditionalServiceType: *const iso20_AdditionalServiceType,
) -> i32 {
    let mut grammar_id: i32 = 74 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            74 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*AdditionalServiceType).ServiceName.charactersLen as i32 + 2 as i32)
                                as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*AdditionalServiceType).ServiceName.charactersLen as usize,
                                ((*AdditionalServiceType).ServiceName.characters).as_ptr(),
                                (80 as i32 + 1 as i32) as usize,
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
                }
            }
            75 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_RationalNumberType(
                        stream,
                        &(*AdditionalServiceType).ServiceFee,
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
unsafe extern "C" fn encode_iso20_PowerScheduleType(
    stream: &mut ExiBitstream,
    mut PowerScheduleType: *const iso20_PowerScheduleType,
) -> i32 {
    let mut grammar_id: i32 = 76 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            76 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            exi_basetypes_encoder_uint_64(stream, (*PowerScheduleType).TimeAnchor);
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
            }
            77 => {
                if (*PowerScheduleType).AvailableEnergy_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*PowerScheduleType).AvailableEnergy,
                        );
                        if error == 0 as i32 {
                            grammar_id = 78 as i32;
                        }
                    }
                } else if (*PowerScheduleType).PowerTolerance_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*PowerScheduleType).PowerTolerance,
                        );
                        if error == 0 as i32 {
                            grammar_id = 79 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_PowerScheduleEntryListType(
                            stream,
                            &(*PowerScheduleType).PowerScheduleEntries,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                }
            }
            78 => {
                if (*PowerScheduleType).PowerTolerance_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*PowerScheduleType).PowerTolerance,
                        );
                        if error == 0 as i32 {
                            grammar_id = 79 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_PowerScheduleEntryListType(
                            stream,
                            &(*PowerScheduleType).PowerScheduleEntries,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                }
            }
            79 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_PowerScheduleEntryListType(
                        stream,
                        &(*PowerScheduleType).PowerScheduleEntries,
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
unsafe extern "C" fn encode_iso20_EVPowerScheduleEntryListType(
    stream: &mut ExiBitstream,
    mut EVPowerScheduleEntryListType: *const iso20_EVPowerScheduleEntryListType,
) -> i32 {
    let mut grammar_id: i32 = 80 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut EVPowerScheduleEntry_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            80 => {
                if (EVPowerScheduleEntry_currentIndex as i32)
                    < (*EVPowerScheduleEntryListType)
                        .EVPowerScheduleEntry
                        .arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh6 = EVPowerScheduleEntry_currentIndex;
                        EVPowerScheduleEntry_currentIndex =
                            EVPowerScheduleEntry_currentIndex.wrapping_add(1);
                        error = encode_iso20_EVPowerScheduleEntryType(
                            stream,
                            &*((*EVPowerScheduleEntryListType).EVPowerScheduleEntry.array)
                                .as_ptr()
                                .offset(fresh6 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 81 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            81 => {
                if (EVPowerScheduleEntry_currentIndex as i32)
                    < (*EVPowerScheduleEntryListType)
                        .EVPowerScheduleEntry
                        .arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh7 = EVPowerScheduleEntry_currentIndex;
                        EVPowerScheduleEntry_currentIndex =
                            EVPowerScheduleEntry_currentIndex.wrapping_add(1);
                        error = encode_iso20_EVPowerScheduleEntryType(
                            stream,
                            &*((*EVPowerScheduleEntryListType).EVPowerScheduleEntry.array)
                                .as_ptr()
                                .offset(fresh7 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 81 as i32;
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
unsafe extern "C" fn encode_iso20_OverstayRuleType(
    stream: &mut ExiBitstream,
    mut OverstayRuleType: *const iso20_OverstayRuleType,
) -> i32 {
    let mut grammar_id: i32 = 82 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            82 => {
                if (*OverstayRuleType).OverstayRuleDescription_isUsed() == 1 as u32 {
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
                                ((*OverstayRuleType).OverstayRuleDescription.charactersLen as i32
                                    + 2 as i32) as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*OverstayRuleType).OverstayRuleDescription.charactersLen
                                        as usize,
                                    ((*OverstayRuleType).OverstayRuleDescription.characters)
                                        .as_ptr(),
                                    (160 as i32 + 1 as i32) as usize,
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
                            error = exi_basetypes_encoder_uint_32(
                                stream,
                                (*OverstayRuleType).StartTime,
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
            }
            83 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            exi_basetypes_encoder_uint_32(stream, (*OverstayRuleType).StartTime);
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
                        encode_iso20_RationalNumberType(stream, &(*OverstayRuleType).OverstayFee);
                    if error == 0 as i32 {
                        grammar_id = 85 as i32;
                    }
                }
            }
            85 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_32(
                            stream,
                            (*OverstayRuleType).OverstayFeePeriod,
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
unsafe extern "C" fn encode_iso20_EVPriceRuleStackListType(
    stream: &mut ExiBitstream,
    mut EVPriceRuleStackListType: *const iso20_EVPriceRuleStackListType,
) -> i32 {
    let mut grammar_id: i32 = 86 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut EVPriceRuleStack_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            86 => {
                if (EVPriceRuleStack_currentIndex as i32)
                    < (*EVPriceRuleStackListType).EVPriceRuleStack.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh8 = EVPriceRuleStack_currentIndex;
                        EVPriceRuleStack_currentIndex =
                            EVPriceRuleStack_currentIndex.wrapping_add(1);
                        error = encode_iso20_EVPriceRuleStackType(
                            stream,
                            &*((*EVPriceRuleStackListType).EVPriceRuleStack.array)
                                .as_ptr()
                                .offset(fresh8 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 87 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            87 => {
                if (EVPriceRuleStack_currentIndex as i32)
                    < (*EVPriceRuleStackListType).EVPriceRuleStack.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh9 = EVPriceRuleStack_currentIndex;
                        EVPriceRuleStack_currentIndex =
                            EVPriceRuleStack_currentIndex.wrapping_add(1);
                        error = encode_iso20_EVPriceRuleStackType(
                            stream,
                            &*((*EVPriceRuleStackListType).EVPriceRuleStack.array)
                                .as_ptr()
                                .offset(fresh9 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 87 as i32;
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
unsafe extern "C" fn encode_iso20_SPKIDataType(
    stream: &mut ExiBitstream,
    mut SPKIDataType: *const iso20_SPKIDataType,
) -> i32 {
    let mut grammar_id: i32 = 88 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            88 => {
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
                                    grammar_id = 89 as i32;
                                }
                            }
                        }
                    }
                }
            }
            89 => {
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
unsafe extern "C" fn encode_iso20_SignedInfoType(
    stream: &mut ExiBitstream,
    mut SignedInfoType: *const iso20_SignedInfoType,
) -> i32 {
    let mut grammar_id: i32 = 90 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut Reference_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            90 => {
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
                                grammar_id = 91 as i32;
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_CanonicalizationMethodType(
                            stream,
                            &(*SignedInfoType).CanonicalizationMethod,
                        );
                        if error == 0 as i32 {
                            grammar_id = 92 as i32;
                        }
                    }
                }
            }
            91 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_CanonicalizationMethodType(
                        stream,
                        &(*SignedInfoType).CanonicalizationMethod,
                    );
                    if error == 0 as i32 {
                        grammar_id = 92 as i32;
                    }
                }
            }
            92 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_SignatureMethodType(
                        stream,
                        &(*SignedInfoType).SignatureMethod,
                    );
                    if error == 0 as i32 {
                        grammar_id = 93 as i32;
                    }
                }
            }
            93 => {
                if (Reference_currentIndex as i32) < (*SignedInfoType).Reference.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh10 = Reference_currentIndex;
                        Reference_currentIndex = Reference_currentIndex.wrapping_add(1);
                        error = encode_iso20_ReferenceType(
                            stream,
                            &*((*SignedInfoType).Reference.array)
                                .as_ptr()
                                .offset(fresh10 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 94 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            94 => {
                if (Reference_currentIndex as i32) < (*SignedInfoType).Reference.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh11 = Reference_currentIndex;
                        Reference_currentIndex = Reference_currentIndex.wrapping_add(1);
                        error = encode_iso20_ReferenceType(
                            stream,
                            &*((*SignedInfoType).Reference.array)
                                .as_ptr()
                                .offset(fresh11 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 94 as i32;
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
unsafe extern "C" fn encode_iso20_EVPowerScheduleType(
    stream: &mut ExiBitstream,
    mut EVPowerScheduleType: *const iso20_EVPowerScheduleType,
) -> i32 {
    let mut grammar_id: i32 = 95 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            95 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_64(
                            stream,
                            (*EVPowerScheduleType).TimeAnchor,
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
                    error = encode_iso20_EVPowerScheduleEntryListType(
                        stream,
                        &(*EVPowerScheduleType).EVPowerScheduleEntries,
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
unsafe extern "C" fn encode_iso20_SignatureValueType(
    stream: &mut ExiBitstream,
    mut SignatureValueType: *const iso20_SignatureValueType,
) -> i32 {
    let mut grammar_id: i32 = 97 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            97 => {
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
                                grammar_id = 98 as i32;
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
            98 => {
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
unsafe extern "C" fn encode_iso20_SubCertificatesType(
    stream: &mut ExiBitstream,
    mut SubCertificatesType: *const iso20_SubCertificatesType,
) -> i32 {
    let mut grammar_id: i32 = 99 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut Certificate_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            99 => {
                if (Certificate_currentIndex as i32)
                    < (*SubCertificatesType).Certificate.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*SubCertificatesType).Certificate.array
                                    [Certificate_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*SubCertificatesType).Certificate.array
                                        [Certificate_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*SubCertificatesType).Certificate.array
                                        [Certificate_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    1600 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    Certificate_currentIndex =
                                        Certificate_currentIndex.wrapping_add(1);
                                    Certificate_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 100 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            100 => {
                if (Certificate_currentIndex as i32)
                    < (*SubCertificatesType).Certificate.arrayLen as i32
                {
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
                                (*SubCertificatesType).Certificate.array
                                    [Certificate_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*SubCertificatesType).Certificate.array
                                        [Certificate_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*SubCertificatesType).Certificate.array
                                        [Certificate_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    1600 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    Certificate_currentIndex =
                                        Certificate_currentIndex.wrapping_add(1);
                                    Certificate_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 100 as i32;
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
unsafe extern "C" fn encode_iso20_ParameterType(
    stream: &mut ExiBitstream,
    mut ParameterType: *const iso20_ParameterType,
) -> i32 {
    let mut grammar_id: i32 = 101 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            101 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = exi_basetypes_encoder_uint_16(
                        stream,
                        ((*ParameterType).Name.charactersLen as i32 + 2 as i32) as u16,
                    );
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_characters(
                            stream,
                            (*ParameterType).Name.charactersLen as usize,
                            ((*ParameterType).Name.characters).as_ptr(),
                            (80 as i32 + 1 as i32) as usize,
                        );
                        if error == 0 as i32 {
                            grammar_id = 102 as i32;
                        }
                    }
                }
            }
            102 => {
                if (*ParameterType).boolValue_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(stream, (*ParameterType).boolValue);
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
                } else if (*ParameterType).byteValue_isUsed() == 1 as u32 {
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
                                8 as i32 as usize,
                                ((*ParameterType).byteValue as i32 + -(128 as i32)) as u32,
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
                } else if (*ParameterType).shortValue_isUsed() == 1 as u32 {
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
                                (*ParameterType).shortValue,
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
                } else if (*ParameterType).intValue_isUsed() == 1 as u32 {
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
                                exi_basetypes_encoder_integer_32(stream, (*ParameterType).intValue);
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
                } else if (*ParameterType).rationalNumber_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*ParameterType).rationalNumber,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                ((*ParameterType).finiteString.charactersLen as i32 + 2 as i32)
                                    as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*ParameterType).finiteString.charactersLen as usize,
                                    ((*ParameterType).finiteString.characters).as_ptr(),
                                    (80 as i32 + 1 as i32) as usize,
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
unsafe extern "C" fn encode_iso20_EVAbsolutePriceScheduleType(
    stream: &mut ExiBitstream,
    mut EVAbsolutePriceScheduleType: *const iso20_EVAbsolutePriceScheduleType,
) -> i32 {
    let mut grammar_id: i32 = 103 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            103 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_64(
                            stream,
                            (*EVAbsolutePriceScheduleType).TimeAnchor,
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
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*EVAbsolutePriceScheduleType).Currency.charactersLen as i32
                                + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*EVAbsolutePriceScheduleType).Currency.charactersLen as usize,
                                ((*EVAbsolutePriceScheduleType).Currency.characters).as_ptr(),
                                (3 as i32 + 1 as i32) as usize,
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
            }
            105 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*EVAbsolutePriceScheduleType).PriceAlgorithm.charactersLen as i32
                                + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*EVAbsolutePriceScheduleType).PriceAlgorithm.charactersLen
                                    as usize,
                                ((*EVAbsolutePriceScheduleType).PriceAlgorithm.characters).as_ptr(),
                                (255 as i32 + 1 as i32) as usize,
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
            }
            106 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_EVPriceRuleStackListType(
                        stream,
                        &(*EVAbsolutePriceScheduleType).EVPriceRuleStacks,
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
unsafe extern "C" fn encode_iso20_DetailedCostType(
    stream: &mut ExiBitstream,
    mut DetailedCostType: *const iso20_DetailedCostType,
) -> i32 {
    let mut grammar_id: i32 = 107 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            107 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_RationalNumberType(stream, &(*DetailedCostType).Amount);
                    if error == 0 as i32 {
                        grammar_id = 108 as i32;
                    }
                }
            }
            108 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso20_RationalNumberType(stream, &(*DetailedCostType).CostPerUnit);
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
unsafe extern "C" fn encode_iso20_KeyInfoType(
    stream: &mut ExiBitstream,
    mut KeyInfoType: *const iso20_KeyInfoType,
) -> i32 {
    let mut grammar_id: i32 = 109 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            109 => {
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
                                grammar_id = 110 as i32;
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
                        error = encode_iso20_KeyValueType(stream, &(*KeyInfoType).KeyValue);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).RetrievalMethod_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RetrievalMethodType(
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
                        error = encode_iso20_X509DataType(stream, &(*KeyInfoType).X509Data);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).PGPData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_PGPDataType(stream, &(*KeyInfoType).PGPData);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).SPKIData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_SPKIDataType(stream, &(*KeyInfoType).SPKIData);
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
            110 => {
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
                        error = encode_iso20_KeyValueType(stream, &(*KeyInfoType).KeyValue);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).RetrievalMethod_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RetrievalMethodType(
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
                        error = encode_iso20_X509DataType(stream, &(*KeyInfoType).X509Data);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).PGPData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_PGPDataType(stream, &(*KeyInfoType).PGPData);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).SPKIData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_SPKIDataType(stream, &(*KeyInfoType).SPKIData);
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
unsafe extern "C" fn encode_iso20_ObjectType(
    stream: &mut ExiBitstream,
    mut ObjectType: *const iso20_ObjectType,
) -> i32 {
    let mut grammar_id: i32 = 111 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            111 => {
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
                                grammar_id = 112 as i32;
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
                                grammar_id = 113 as i32;
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
                                grammar_id = 114 as i32;
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
            112 => {
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
                                grammar_id = 113 as i32;
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
                                grammar_id = 114 as i32;
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
            113 => {
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
                                grammar_id = 114 as i32;
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
            114 => {
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
unsafe extern "C" fn encode_iso20_PriceLevelScheduleEntryListType(
    stream: &mut ExiBitstream,
    mut PriceLevelScheduleEntryListType: *const iso20_PriceLevelScheduleEntryListType,
) -> i32 {
    let mut grammar_id: i32 = 115 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut PriceLevelScheduleEntry_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            115 => {
                if (PriceLevelScheduleEntry_currentIndex as i32)
                    < (*PriceLevelScheduleEntryListType)
                        .PriceLevelScheduleEntry
                        .arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh12 = PriceLevelScheduleEntry_currentIndex;
                        PriceLevelScheduleEntry_currentIndex =
                            PriceLevelScheduleEntry_currentIndex.wrapping_add(1);
                        error = encode_iso20_PriceLevelScheduleEntryType(
                            stream,
                            &*((*PriceLevelScheduleEntryListType)
                                .PriceLevelScheduleEntry
                                .array)
                                .as_ptr()
                                .offset(fresh12 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 116 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            116 => {
                if (PriceLevelScheduleEntry_currentIndex as i32)
                    < (*PriceLevelScheduleEntryListType)
                        .PriceLevelScheduleEntry
                        .arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh13 = PriceLevelScheduleEntry_currentIndex;
                        PriceLevelScheduleEntry_currentIndex =
                            PriceLevelScheduleEntry_currentIndex.wrapping_add(1);
                        error = encode_iso20_PriceLevelScheduleEntryType(
                            stream,
                            &*((*PriceLevelScheduleEntryListType)
                                .PriceLevelScheduleEntry
                                .array)
                                .as_ptr()
                                .offset(fresh13 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 116 as i32;
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
unsafe extern "C" fn encode_iso20_DetailedTaxType(
    stream: &mut ExiBitstream,
    mut DetailedTaxType: *const iso20_DetailedTaxType,
) -> i32 {
    let mut grammar_id: i32 = 117 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            117 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_32(stream, (*DetailedTaxType).TaxRuleID);
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 118 as i32;
                            }
                        }
                    }
                }
            }
            118 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_RationalNumberType(stream, &(*DetailedTaxType).Amount);
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
unsafe extern "C" fn encode_iso20_TaxRuleListType(
    stream: &mut ExiBitstream,
    mut TaxRuleListType: *const iso20_TaxRuleListType,
) -> i32 {
    let mut grammar_id: i32 = 119 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut TaxRule_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            119 => {
                if (TaxRule_currentIndex as i32) < (*TaxRuleListType).TaxRule.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh14 = TaxRule_currentIndex;
                        TaxRule_currentIndex = TaxRule_currentIndex.wrapping_add(1);
                        error = encode_iso20_TaxRuleType(
                            stream,
                            &*((*TaxRuleListType).TaxRule.array)
                                .as_ptr()
                                .offset(fresh14 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 120 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            120 => {
                if (TaxRule_currentIndex as i32) < (*TaxRuleListType).TaxRule.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh15 = TaxRule_currentIndex;
                        TaxRule_currentIndex = TaxRule_currentIndex.wrapping_add(1);
                        error = encode_iso20_TaxRuleType(
                            stream,
                            &*((*TaxRuleListType).TaxRule.array)
                                .as_ptr()
                                .offset(fresh15 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 120 as i32;
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
unsafe extern "C" fn encode_iso20_PriceRuleStackListType(
    stream: &mut ExiBitstream,
    mut PriceRuleStackListType: *const iso20_PriceRuleStackListType,
) -> i32 {
    let mut grammar_id: i32 = 121 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut PriceRuleStack_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            121 => {
                if (PriceRuleStack_currentIndex as i32)
                    < (*PriceRuleStackListType).PriceRuleStack.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh16 = PriceRuleStack_currentIndex;
                        PriceRuleStack_currentIndex = PriceRuleStack_currentIndex.wrapping_add(1);
                        error = encode_iso20_PriceRuleStackType(
                            stream,
                            &*((*PriceRuleStackListType).PriceRuleStack.array)
                                .as_ptr()
                                .offset(fresh16 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 122 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            122 => {
                if (PriceRuleStack_currentIndex as i32)
                    < (*PriceRuleStackListType).PriceRuleStack.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh17 = PriceRuleStack_currentIndex;
                        PriceRuleStack_currentIndex = PriceRuleStack_currentIndex.wrapping_add(1);
                        error = encode_iso20_PriceRuleStackType(
                            stream,
                            &*((*PriceRuleStackListType).PriceRuleStack.array)
                                .as_ptr()
                                .offset(fresh17 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 122 as i32;
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
unsafe extern "C" fn encode_iso20_OverstayRuleListType(
    stream: &mut ExiBitstream,
    mut OverstayRuleListType: *const iso20_OverstayRuleListType,
) -> i32 {
    let mut grammar_id: i32 = 123 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut OverstayRule_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            123 => {
                if (*OverstayRuleListType).OverstayTimeThreshold_isUsed() == 1 as u32 {
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
                                (*OverstayRuleListType).OverstayTimeThreshold,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 125 as i32;
                                }
                            }
                        }
                    }
                } else if (*OverstayRuleListType).OverstayPowerThreshold_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*OverstayRuleListType).OverstayPowerThreshold,
                        );
                        if error == 0 as i32 {
                            grammar_id = 127 as i32;
                        }
                    }
                } else if (OverstayRule_currentIndex as i32)
                    < (*OverstayRuleListType).OverstayRule.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh18 = OverstayRule_currentIndex;
                        OverstayRule_currentIndex = OverstayRule_currentIndex.wrapping_add(1);
                        error = encode_iso20_OverstayRuleType(
                            stream,
                            &*((*OverstayRuleListType).OverstayRule.array)
                                .as_ptr()
                                .offset(fresh18 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 124 as i32;
                        }
                    }
                }
            }
            124 => {
                if (OverstayRule_currentIndex as i32)
                    < (*OverstayRuleListType).OverstayRule.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh19 = OverstayRule_currentIndex;
                        OverstayRule_currentIndex = OverstayRule_currentIndex.wrapping_add(1);
                        error = encode_iso20_OverstayRuleType(
                            stream,
                            &*((*OverstayRuleListType).OverstayRule.array)
                                .as_ptr()
                                .offset(fresh19 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 124 as i32;
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
            125 => {
                if (*OverstayRuleListType).OverstayPowerThreshold_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*OverstayRuleListType).OverstayPowerThreshold,
                        );
                        if error == 0 as i32 {
                            grammar_id = 127 as i32;
                        }
                    }
                } else if (OverstayRule_currentIndex as i32)
                    < (*OverstayRuleListType).OverstayRule.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh20 = OverstayRule_currentIndex;
                        OverstayRule_currentIndex = OverstayRule_currentIndex.wrapping_add(1);
                        error = encode_iso20_OverstayRuleType(
                            stream,
                            &*((*OverstayRuleListType).OverstayRule.array)
                                .as_ptr()
                                .offset(fresh20 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 126 as i32;
                        }
                    }
                }
            }
            126 => {
                if (OverstayRule_currentIndex as i32)
                    < (*OverstayRuleListType).OverstayRule.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh21 = OverstayRule_currentIndex;
                        OverstayRule_currentIndex = OverstayRule_currentIndex.wrapping_add(1);
                        error = encode_iso20_OverstayRuleType(
                            stream,
                            &*((*OverstayRuleListType).OverstayRule.array)
                                .as_ptr()
                                .offset(fresh21 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 126 as i32;
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
            127 => {
                if (OverstayRule_currentIndex as i32)
                    < (*OverstayRuleListType).OverstayRule.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh22 = OverstayRule_currentIndex;
                        OverstayRule_currentIndex = OverstayRule_currentIndex.wrapping_add(1);
                        error = encode_iso20_OverstayRuleType(
                            stream,
                            &*((*OverstayRuleListType).OverstayRule.array)
                                .as_ptr()
                                .offset(fresh22 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 128 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            128 => {
                if (OverstayRule_currentIndex as i32)
                    < (*OverstayRuleListType).OverstayRule.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh23 = OverstayRule_currentIndex;
                        OverstayRule_currentIndex = OverstayRule_currentIndex.wrapping_add(1);
                        error = encode_iso20_OverstayRuleType(
                            stream,
                            &*((*OverstayRuleListType).OverstayRule.array)
                                .as_ptr()
                                .offset(fresh23 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 128 as i32;
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
unsafe extern "C" fn encode_iso20_AdditionalServiceListType(
    stream: &mut ExiBitstream,
    mut AdditionalServiceListType: *const iso20_AdditionalServiceListType,
) -> i32 {
    let mut grammar_id: i32 = 129 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut AdditionalService_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            129 => {
                if (AdditionalService_currentIndex as i32)
                    < (*AdditionalServiceListType).AdditionalService.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh24 = AdditionalService_currentIndex;
                        AdditionalService_currentIndex =
                            AdditionalService_currentIndex.wrapping_add(1);
                        error = encode_iso20_AdditionalServiceType(
                            stream,
                            &*((*AdditionalServiceListType).AdditionalService.array)
                                .as_ptr()
                                .offset(fresh24 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 130 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            130 => {
                if (AdditionalService_currentIndex as i32)
                    < (*AdditionalServiceListType).AdditionalService.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh25 = AdditionalService_currentIndex;
                        AdditionalService_currentIndex =
                            AdditionalService_currentIndex.wrapping_add(1);
                        error = encode_iso20_AdditionalServiceType(
                            stream,
                            &*((*AdditionalServiceListType).AdditionalService.array)
                                .as_ptr()
                                .offset(fresh25 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 130 as i32;
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
unsafe extern "C" fn encode_iso20_ServiceType(
    stream: &mut ExiBitstream,
    mut ServiceType: *const iso20_ServiceType,
) -> i32 {
    let mut grammar_id: i32 = 131 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            131 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(stream, (*ServiceType).ServiceID);
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 132 as i32;
                            }
                        }
                    }
                }
            }
            132 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(stream, (*ServiceType).FreeService);
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
unsafe extern "C" fn encode_iso20_ParameterSetType(
    stream: &mut ExiBitstream,
    mut ParameterSetType: *const iso20_ParameterSetType,
) -> i32 {
    let mut grammar_id: i32 = 133 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut Parameter_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            133 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*ParameterSetType).ParameterSetID,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 134 as i32;
                            }
                        }
                    }
                }
            }
            134 => {
                if (Parameter_currentIndex as i32) < (*ParameterSetType).Parameter.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh26 = Parameter_currentIndex;
                        Parameter_currentIndex = Parameter_currentIndex.wrapping_add(1);
                        error = encode_iso20_ParameterType(
                            stream,
                            &*((*ParameterSetType).Parameter.array)
                                .as_ptr()
                                .offset(fresh26 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 135 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            135 => {
                if (Parameter_currentIndex as i32) < (*ParameterSetType).Parameter.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh27 = Parameter_currentIndex;
                        Parameter_currentIndex = Parameter_currentIndex.wrapping_add(1);
                        error = encode_iso20_ParameterType(
                            stream,
                            &*((*ParameterSetType).Parameter.array)
                                .as_ptr()
                                .offset(fresh27 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 135 as i32;
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
unsafe extern "C" fn encode_iso20_SupportedProvidersListType(
    stream: &mut ExiBitstream,
    mut SupportedProvidersListType: *const iso20_SupportedProvidersListType,
) -> i32 {
    let mut grammar_id: i32 = 136 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut ProviderID_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            136 => {
                if (ProviderID_currentIndex as i32)
                    < (*SupportedProvidersListType).ProviderID.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                ((*SupportedProvidersListType).ProviderID.array
                                    [ProviderID_currentIndex as usize]
                                    .charactersLen as i32
                                    + 2 as i32) as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*SupportedProvidersListType).ProviderID.array
                                        [ProviderID_currentIndex as usize]
                                        .charactersLen as usize,
                                    ((*SupportedProvidersListType).ProviderID.array
                                        [ProviderID_currentIndex as usize]
                                        .characters)
                                        .as_ptr(),
                                    (80 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    ProviderID_currentIndex =
                                        ProviderID_currentIndex.wrapping_add(1);
                                    ProviderID_currentIndex;
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
                } else {
                    error = -(150 as i32);
                }
            }
            137 => {
                if (ProviderID_currentIndex as i32)
                    < (*SupportedProvidersListType).ProviderID.arrayLen as i32
                {
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
                                ((*SupportedProvidersListType).ProviderID.array
                                    [ProviderID_currentIndex as usize]
                                    .charactersLen as i32
                                    + 2 as i32) as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*SupportedProvidersListType).ProviderID.array
                                        [ProviderID_currentIndex as usize]
                                        .charactersLen as usize,
                                    ((*SupportedProvidersListType).ProviderID.array
                                        [ProviderID_currentIndex as usize]
                                        .characters)
                                        .as_ptr(),
                                    (80 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    ProviderID_currentIndex =
                                        ProviderID_currentIndex.wrapping_add(1);
                                    ProviderID_currentIndex;
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
unsafe extern "C" fn encode_iso20_ContractCertificateChainType(
    stream: &mut ExiBitstream,
    mut ContractCertificateChainType: *const iso20_ContractCertificateChainType,
) -> i32 {
    let mut grammar_id: i32 = 138 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            138 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*ContractCertificateChainType).Certificate.bytesLen,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*ContractCertificateChainType).Certificate.bytesLen as usize,
                                ((*ContractCertificateChainType).Certificate.bytes).as_ptr(),
                                1600 as i32 as usize,
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
                }
            }
            139 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_SubCertificatesType(
                        stream,
                        &(*ContractCertificateChainType).SubCertificates,
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
unsafe extern "C" fn encode_iso20_Dynamic_EVPPTControlModeType(
    stream: &mut ExiBitstream,
    mut Dynamic_EVPPTControlModeType: *const iso20_Dynamic_EVPPTControlModeType,
) -> i32 {
    let mut error: i32 =
        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
    return error;
}
unsafe extern "C" fn encode_iso20_MeterInfoType(
    stream: &mut ExiBitstream,
    mut MeterInfoType: *const iso20_MeterInfoType,
) -> i32 {
    let mut grammar_id: i32 = 140 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            140 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
                                    grammar_id = 141 as i32;
                                }
                            }
                        }
                    }
                }
            }
            141 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
                                grammar_id = 142 as i32;
                            }
                        }
                    }
                }
            }
            142 => {
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
                                    grammar_id = 143 as i32;
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
                                    grammar_id = 144 as i32;
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
                                    grammar_id = 145 as i32;
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
                                        grammar_id = 146 as i32;
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
                                    grammar_id = 147 as i32;
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
            143 => {
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
                                    grammar_id = 144 as i32;
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
                                    grammar_id = 145 as i32;
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
                                        grammar_id = 146 as i32;
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
                                    grammar_id = 147 as i32;
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
            144 => {
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
                                    grammar_id = 145 as i32;
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
                                        grammar_id = 146 as i32;
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
                                    grammar_id = 147 as i32;
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
            145 => {
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
                                        grammar_id = 146 as i32;
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
                                    grammar_id = 147 as i32;
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
            146 => {
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
                                    grammar_id = 147 as i32;
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
            147 => {
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
unsafe extern "C" fn encode_iso20_SignatureType(
    stream: &mut ExiBitstream,
    mut SignatureType: *const iso20_SignatureType,
) -> i32 {
    let mut grammar_id: i32 = 148 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            148 => {
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
                                grammar_id = 149 as i32;
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_SignedInfoType(stream, &(*SignatureType).SignedInfo);
                        if error == 0 as i32 {
                            grammar_id = 150 as i32;
                        }
                    }
                }
            }
            149 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_SignedInfoType(stream, &(*SignatureType).SignedInfo);
                    if error == 0 as i32 {
                        grammar_id = 150 as i32;
                    }
                }
            }
            150 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso20_SignatureValueType(stream, &(*SignatureType).SignatureValue);
                    if error == 0 as i32 {
                        grammar_id = 151 as i32;
                    }
                }
            }
            151 => {
                if (*SignatureType).KeyInfo_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_KeyInfoType(stream, &(*SignatureType).KeyInfo);
                        if error == 0 as i32 {
                            grammar_id = 153 as i32;
                        }
                    }
                } else if (*SignatureType).Object_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ObjectType(stream, &(*SignatureType).Object);
                        if error == 0 as i32 {
                            grammar_id = 152 as i32;
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
            152 => {
                if 1 as i32 == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ObjectType(stream, &(*SignatureType).Object);
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
            153 => {
                if (*SignatureType).Object_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ObjectType(stream, &(*SignatureType).Object);
                        if error == 0 as i32 {
                            grammar_id = 154 as i32;
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
            154 => {
                if 1 as i32 == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ObjectType(stream, &(*SignatureType).Object);
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
unsafe extern "C" fn encode_iso20_Scheduled_EVPPTControlModeType(
    stream: &mut ExiBitstream,
    mut Scheduled_EVPPTControlModeType: *const iso20_Scheduled_EVPPTControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 155 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            155 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_32(
                            stream,
                            (*Scheduled_EVPPTControlModeType).SelectedScheduleTupleID,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 156 as i32;
                            }
                        }
                    }
                }
            }
            156 => {
                if (*Scheduled_EVPPTControlModeType).PowerToleranceAcceptance_isUsed() == 1 as u32 {
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
                                1 as i32 as usize,
                                (*Scheduled_EVPPTControlModeType).PowerToleranceAcceptance as u32,
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
unsafe extern "C" fn encode_iso20_ReceiptType(
    stream: &mut ExiBitstream,
    mut ReceiptType: *const iso20_ReceiptType,
) -> i32 {
    let mut grammar_id: i32 = 157 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut TaxCosts_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            157 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_64(stream, (*ReceiptType).TimeAnchor);
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 158 as i32;
                            }
                        }
                    }
                }
            }
            158 => {
                if (*ReceiptType).EnergyCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_DetailedCostType(stream, &(*ReceiptType).EnergyCosts);
                        if error == 0 as i32 {
                            grammar_id = 160 as i32;
                        }
                    }
                } else if (*ReceiptType).OccupancyCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_DetailedCostType(stream, &(*ReceiptType).OccupancyCosts);
                        if error == 0 as i32 {
                            grammar_id = 162 as i32;
                        }
                    }
                } else if (*ReceiptType).AdditionalServicesCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_DetailedCostType(
                            stream,
                            &(*ReceiptType).AdditionalServicesCosts,
                        );
                        if error == 0 as i32 {
                            grammar_id = 164 as i32;
                        }
                    }
                } else if (*ReceiptType).OverstayCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_DetailedCostType(stream, &(*ReceiptType).OverstayCosts);
                        if error == 0 as i32 {
                            grammar_id = 166 as i32;
                        }
                    }
                } else if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh28 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh28 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 159 as i32;
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
                if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh29 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh29 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 159 as i32;
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
            160 => {
                if (*ReceiptType).OccupancyCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_DetailedCostType(stream, &(*ReceiptType).OccupancyCosts);
                        if error == 0 as i32 {
                            grammar_id = 162 as i32;
                        }
                    }
                } else if (*ReceiptType).AdditionalServicesCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_DetailedCostType(
                            stream,
                            &(*ReceiptType).AdditionalServicesCosts,
                        );
                        if error == 0 as i32 {
                            grammar_id = 164 as i32;
                        }
                    }
                } else if (*ReceiptType).OverstayCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_DetailedCostType(stream, &(*ReceiptType).OverstayCosts);
                        if error == 0 as i32 {
                            grammar_id = 166 as i32;
                        }
                    }
                } else if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh30 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh30 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 161 as i32;
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
            161 => {
                if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh31 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh31 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 161 as i32;
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
            162 => {
                if (*ReceiptType).AdditionalServicesCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_DetailedCostType(
                            stream,
                            &(*ReceiptType).AdditionalServicesCosts,
                        );
                        if error == 0 as i32 {
                            grammar_id = 164 as i32;
                        }
                    }
                } else if (*ReceiptType).OverstayCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_DetailedCostType(stream, &(*ReceiptType).OverstayCosts);
                        if error == 0 as i32 {
                            grammar_id = 166 as i32;
                        }
                    }
                } else if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh32 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh32 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 163 as i32;
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
            163 => {
                if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh33 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh33 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 163 as i32;
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
            164 => {
                if (*ReceiptType).OverstayCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_DetailedCostType(stream, &(*ReceiptType).OverstayCosts);
                        if error == 0 as i32 {
                            grammar_id = 166 as i32;
                        }
                    }
                } else if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh34 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh34 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 165 as i32;
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
            165 => {
                if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh35 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh35 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 165 as i32;
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
            166 => {
                if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh36 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh36 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 167 as i32;
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
            167 => {
                if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh37 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh37 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 167 as i32;
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
unsafe extern "C" fn encode_iso20_AbsolutePriceScheduleType(
    stream: &mut ExiBitstream,
    mut AbsolutePriceScheduleType: *const iso20_AbsolutePriceScheduleType,
) -> i32 {
    let mut grammar_id: i32 = 168 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            168 => {
                if (*AbsolutePriceScheduleType).Id_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*AbsolutePriceScheduleType).Id.charactersLen as i32 + 2 as i32)
                                as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*AbsolutePriceScheduleType).Id.charactersLen as usize,
                                ((*AbsolutePriceScheduleType).Id.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 169 as i32;
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
                            error = exi_basetypes_encoder_uint_64(
                                stream,
                                (*AbsolutePriceScheduleType).TimeAnchor,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 170 as i32;
                                }
                            }
                        }
                    }
                }
            }
            169 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_64(
                            stream,
                            (*AbsolutePriceScheduleType).TimeAnchor,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 170 as i32;
                            }
                        }
                    }
                }
            }
            170 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_32(
                            stream,
                            (*AbsolutePriceScheduleType).PriceScheduleID,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 171 as i32;
                            }
                        }
                    }
                }
            }
            171 => {
                if (*AbsolutePriceScheduleType).PriceScheduleDescription_isUsed() == 1 as u32 {
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
                                ((*AbsolutePriceScheduleType)
                                    .PriceScheduleDescription
                                    .charactersLen as i32
                                    + 2 as i32) as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*AbsolutePriceScheduleType)
                                        .PriceScheduleDescription
                                        .charactersLen as usize,
                                    ((*AbsolutePriceScheduleType)
                                        .PriceScheduleDescription
                                        .characters)
                                        .as_ptr(),
                                    (160 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 172 as i32;
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
                                ((*AbsolutePriceScheduleType).Currency.charactersLen as i32
                                    + 2 as i32) as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*AbsolutePriceScheduleType).Currency.charactersLen as usize,
                                    ((*AbsolutePriceScheduleType).Currency.characters).as_ptr(),
                                    (3 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 173 as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            172 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*AbsolutePriceScheduleType).Currency.charactersLen as i32 + 2 as i32)
                                as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*AbsolutePriceScheduleType).Currency.charactersLen as usize,
                                ((*AbsolutePriceScheduleType).Currency.characters).as_ptr(),
                                (3 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 173 as i32;
                                }
                            }
                        }
                    }
                }
            }
            173 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*AbsolutePriceScheduleType).Language.charactersLen as i32 + 2 as i32)
                                as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*AbsolutePriceScheduleType).Language.charactersLen as usize,
                                ((*AbsolutePriceScheduleType).Language.characters).as_ptr(),
                                (3 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 174 as i32;
                                }
                            }
                        }
                    }
                }
            }
            174 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*AbsolutePriceScheduleType).PriceAlgorithm.charactersLen as i32
                                + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*AbsolutePriceScheduleType).PriceAlgorithm.charactersLen as usize,
                                ((*AbsolutePriceScheduleType).PriceAlgorithm.characters).as_ptr(),
                                (255 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 175 as i32;
                                }
                            }
                        }
                    }
                }
            }
            175 => {
                if (*AbsolutePriceScheduleType).MinimumCost_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*AbsolutePriceScheduleType).MinimumCost,
                        );
                        if error == 0 as i32 {
                            grammar_id = 176 as i32;
                        }
                    }
                } else if (*AbsolutePriceScheduleType).MaximumCost_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*AbsolutePriceScheduleType).MaximumCost,
                        );
                        if error == 0 as i32 {
                            grammar_id = 177 as i32;
                        }
                    }
                } else if (*AbsolutePriceScheduleType).TaxRules_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_TaxRuleListType(
                            stream,
                            &(*AbsolutePriceScheduleType).TaxRules,
                        );
                        if error == 0 as i32 {
                            grammar_id = 178 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_PriceRuleStackListType(
                            stream,
                            &(*AbsolutePriceScheduleType).PriceRuleStacks,
                        );
                        if error == 0 as i32 {
                            grammar_id = 179 as i32;
                        }
                    }
                }
            }
            176 => {
                if (*AbsolutePriceScheduleType).MaximumCost_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*AbsolutePriceScheduleType).MaximumCost,
                        );
                        if error == 0 as i32 {
                            grammar_id = 177 as i32;
                        }
                    }
                } else if (*AbsolutePriceScheduleType).TaxRules_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_TaxRuleListType(
                            stream,
                            &(*AbsolutePriceScheduleType).TaxRules,
                        );
                        if error == 0 as i32 {
                            grammar_id = 178 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_PriceRuleStackListType(
                            stream,
                            &(*AbsolutePriceScheduleType).PriceRuleStacks,
                        );
                        if error == 0 as i32 {
                            grammar_id = 179 as i32;
                        }
                    }
                }
            }
            177 => {
                if (*AbsolutePriceScheduleType).TaxRules_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_TaxRuleListType(
                            stream,
                            &(*AbsolutePriceScheduleType).TaxRules,
                        );
                        if error == 0 as i32 {
                            grammar_id = 178 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_PriceRuleStackListType(
                            stream,
                            &(*AbsolutePriceScheduleType).PriceRuleStacks,
                        );
                        if error == 0 as i32 {
                            grammar_id = 179 as i32;
                        }
                    }
                }
            }
            178 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_PriceRuleStackListType(
                        stream,
                        &(*AbsolutePriceScheduleType).PriceRuleStacks,
                    );
                    if error == 0 as i32 {
                        grammar_id = 179 as i32;
                    }
                }
            }
            179 => {
                if (*AbsolutePriceScheduleType).OverstayRules_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_OverstayRuleListType(
                            stream,
                            &(*AbsolutePriceScheduleType).OverstayRules,
                        );
                        if error == 0 as i32 {
                            grammar_id = 180 as i32;
                        }
                    }
                } else if (*AbsolutePriceScheduleType).AdditionalSelectedServices_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_AdditionalServiceListType(
                            stream,
                            &(*AbsolutePriceScheduleType).AdditionalSelectedServices,
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
            180 => {
                if (*AbsolutePriceScheduleType).AdditionalSelectedServices_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_AdditionalServiceListType(
                            stream,
                            &(*AbsolutePriceScheduleType).AdditionalSelectedServices,
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
unsafe extern "C" fn encode_iso20_EVPowerProfileEntryListType(
    stream: &mut ExiBitstream,
    mut EVPowerProfileEntryListType: *const iso20_EVPowerProfileEntryListType,
) -> i32 {
    let mut grammar_id: i32 = 181 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut EVPowerProfileEntry_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            181 => {
                if (EVPowerProfileEntry_currentIndex as i32)
                    < (*EVPowerProfileEntryListType).EVPowerProfileEntry.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh38 = EVPowerProfileEntry_currentIndex;
                        EVPowerProfileEntry_currentIndex =
                            EVPowerProfileEntry_currentIndex.wrapping_add(1);
                        error = encode_iso20_PowerScheduleEntryType(
                            stream,
                            &*((*EVPowerProfileEntryListType).EVPowerProfileEntry.array)
                                .as_ptr()
                                .offset(fresh38 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 182 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            182 => {
                if (EVPowerProfileEntry_currentIndex as i32)
                    < (*EVPowerProfileEntryListType).EVPowerProfileEntry.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh39 = EVPowerProfileEntry_currentIndex;
                        EVPowerProfileEntry_currentIndex =
                            EVPowerProfileEntry_currentIndex.wrapping_add(1);
                        error = encode_iso20_PowerScheduleEntryType(
                            stream,
                            &*((*EVPowerProfileEntryListType).EVPowerProfileEntry.array)
                                .as_ptr()
                                .offset(fresh39 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 182 as i32;
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
unsafe extern "C" fn encode_iso20_Dynamic_SMDTControlModeType(
    stream: &mut ExiBitstream,
    mut Dynamic_SMDTControlModeType: *const iso20_Dynamic_SMDTControlModeType,
) -> i32 {
    let mut error: i32 =
        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
    return error;
}
unsafe extern "C" fn encode_iso20_EVEnergyOfferType(
    stream: &mut ExiBitstream,
    mut EVEnergyOfferType: *const iso20_EVEnergyOfferType,
) -> i32 {
    let mut grammar_id: i32 = 183 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            183 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_EVPowerScheduleType(
                        stream,
                        &(*EVEnergyOfferType).EVPowerSchedule,
                    );
                    if error == 0 as i32 {
                        grammar_id = 184 as i32;
                    }
                }
            }
            184 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_EVAbsolutePriceScheduleType(
                        stream,
                        &(*EVEnergyOfferType).EVAbsolutePriceSchedule,
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
unsafe extern "C" fn encode_iso20_PriceLevelScheduleType(
    stream: &mut ExiBitstream,
    mut PriceLevelScheduleType: *const iso20_PriceLevelScheduleType,
) -> i32 {
    let mut grammar_id: i32 = 185 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            185 => {
                if (*PriceLevelScheduleType).Id_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*PriceLevelScheduleType).Id.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*PriceLevelScheduleType).Id.charactersLen as usize,
                                ((*PriceLevelScheduleType).Id.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 186 as i32;
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
                            error = exi_basetypes_encoder_uint_64(
                                stream,
                                (*PriceLevelScheduleType).TimeAnchor,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 187 as i32;
                                }
                            }
                        }
                    }
                }
            }
            186 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_64(
                            stream,
                            (*PriceLevelScheduleType).TimeAnchor,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 187 as i32;
                            }
                        }
                    }
                }
            }
            187 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_32(
                            stream,
                            (*PriceLevelScheduleType).PriceScheduleID,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 188 as i32;
                            }
                        }
                    }
                }
            }
            188 => {
                if (*PriceLevelScheduleType).PriceScheduleDescription_isUsed() == 1 as u32 {
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
                                ((*PriceLevelScheduleType)
                                    .PriceScheduleDescription
                                    .charactersLen as i32
                                    + 2 as i32) as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*PriceLevelScheduleType)
                                        .PriceScheduleDescription
                                        .charactersLen as usize,
                                    ((*PriceLevelScheduleType)
                                        .PriceScheduleDescription
                                        .characters)
                                        .as_ptr(),
                                    (160 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 189 as i32;
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
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                8 as i32 as usize,
                                (*PriceLevelScheduleType).NumberOfPriceLevels as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 190 as i32;
                                }
                            }
                        }
                    }
                }
            }
            189 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            8 as i32 as usize,
                            (*PriceLevelScheduleType).NumberOfPriceLevels as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 190 as i32;
                            }
                        }
                    }
                }
            }
            190 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_PriceLevelScheduleEntryListType(
                        stream,
                        &(*PriceLevelScheduleType).PriceLevelScheduleEntries,
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
unsafe extern "C" fn encode_iso20_ChargingScheduleType(
    stream: &mut ExiBitstream,
    mut ChargingScheduleType: *const iso20_ChargingScheduleType,
) -> i32 {
    let mut grammar_id: i32 = 191 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            191 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_PowerScheduleType(
                        stream,
                        &(*ChargingScheduleType).PowerSchedule,
                    );
                    if error == 0 as i32 {
                        grammar_id = 192 as i32;
                    }
                }
            }
            192 => {
                if (*ChargingScheduleType).AbsolutePriceSchedule_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_AbsolutePriceScheduleType(
                            stream,
                            &(*ChargingScheduleType).AbsolutePriceSchedule,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*ChargingScheduleType).PriceLevelSchedule_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_PriceLevelScheduleType(
                            stream,
                            &(*ChargingScheduleType).PriceLevelSchedule,
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
unsafe extern "C" fn encode_iso20_ScheduleTupleType(
    stream: &mut ExiBitstream,
    mut ScheduleTupleType: *const iso20_ScheduleTupleType,
) -> i32 {
    let mut grammar_id: i32 = 193 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            193 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_32(
                            stream,
                            (*ScheduleTupleType).ScheduleTupleID,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 194 as i32;
                            }
                        }
                    }
                }
            }
            194 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_ChargingScheduleType(
                        stream,
                        &(*ScheduleTupleType).ChargingSchedule,
                    );
                    if error == 0 as i32 {
                        grammar_id = 195 as i32;
                    }
                }
            }
            195 => {
                if (*ScheduleTupleType).DischargingSchedule_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ChargingScheduleType(
                            stream,
                            &(*ScheduleTupleType).DischargingSchedule,
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
unsafe extern "C" fn encode_iso20_Scheduled_SMDTControlModeType(
    stream: &mut ExiBitstream,
    mut Scheduled_SMDTControlModeType: *const iso20_Scheduled_SMDTControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 196 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            196 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_32(
                            stream,
                            (*Scheduled_SMDTControlModeType).SelectedScheduleTupleID,
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
unsafe extern "C" fn encode_iso20_MessageHeaderType(
    stream: &mut ExiBitstream,
    mut MessageHeaderType: *const iso20_MessageHeaderType,
) -> i32 {
    let mut grammar_id: i32 = 197 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            197 => {
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
                                    grammar_id = 198 as i32;
                                }
                            }
                        }
                    }
                }
            }
            198 => {
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
                                grammar_id = 199 as i32;
                            }
                        }
                    }
                }
            }
            199 => {
                if (*MessageHeaderType).Signature_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_SignatureType(stream, &(*MessageHeaderType).Signature);
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
unsafe extern "C" fn encode_iso20_SignaturePropertyType(
    stream: &mut ExiBitstream,
    mut SignaturePropertyType: *const iso20_SignaturePropertyType,
) -> i32 {
    let mut grammar_id: i32 = 200 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            200 => {
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
                                grammar_id = 201 as i32;
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
                                grammar_id = 202 as i32;
                            }
                        }
                    }
                }
            }
            201 => {
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
                            grammar_id = 202 as i32;
                        }
                    }
                }
            }
            202 => {
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
unsafe extern "C" fn encode_iso20_ServiceIDListType(
    stream: &mut ExiBitstream,
    mut ServiceIDListType: *const iso20_ServiceIDListType,
) -> i32 {
    let mut grammar_id: i32 = 203 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut ServiceID_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            203 => {
                if (ServiceID_currentIndex as i32) < (*ServiceIDListType).ServiceID.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            let fresh40 = ServiceID_currentIndex;
                            ServiceID_currentIndex = ServiceID_currentIndex.wrapping_add(1);
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*ServiceIDListType).ServiceID.array[fresh40 as usize],
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 204 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            204 => {
                if (ServiceID_currentIndex as i32) < (*ServiceIDListType).ServiceID.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            let fresh41 = ServiceID_currentIndex;
                            ServiceID_currentIndex = ServiceID_currentIndex.wrapping_add(1);
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*ServiceIDListType).ServiceID.array[fresh41 as usize],
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 204 as i32;
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
unsafe extern "C" fn encode_iso20_SelectedServiceType(
    stream: &mut ExiBitstream,
    mut SelectedServiceType: *const iso20_SelectedServiceType,
) -> i32 {
    let mut grammar_id: i32 = 205 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            205 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            exi_basetypes_encoder_uint_16(stream, (*SelectedServiceType).ServiceID);
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 206 as i32;
                            }
                        }
                    }
                }
            }
            206 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*SelectedServiceType).ParameterSetID,
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
unsafe extern "C" fn encode_iso20_SignedMeteringDataType(
    stream: &mut ExiBitstream,
    mut SignedMeteringDataType: *const iso20_SignedMeteringDataType,
) -> i32 {
    let mut grammar_id: i32 = 207 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            207 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = exi_basetypes_encoder_uint_16(
                        stream,
                        ((*SignedMeteringDataType).Id.charactersLen as i32 + 2 as i32) as u16,
                    );
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_characters(
                            stream,
                            (*SignedMeteringDataType).Id.charactersLen as usize,
                            ((*SignedMeteringDataType).Id.characters).as_ptr(),
                            (64 as i32 + 1 as i32) as usize,
                        );
                        if error == 0 as i32 {
                            grammar_id = 208 as i32;
                        }
                    }
                }
            }
            208 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*SignedMeteringDataType).SessionID.bytesLen,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*SignedMeteringDataType).SessionID.bytesLen as usize,
                                ((*SignedMeteringDataType).SessionID.bytes).as_ptr(),
                                8 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 209 as i32;
                                }
                            }
                        }
                    }
                }
            }
            209 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso20_MeterInfoType(stream, &(*SignedMeteringDataType).MeterInfo);
                    if error == 0 as i32 {
                        grammar_id = 210 as i32;
                    }
                }
            }
            210 => {
                if (*SignedMeteringDataType).Receipt_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_ReceiptType(stream, &(*SignedMeteringDataType).Receipt);
                        if error == 0 as i32 {
                            grammar_id = 211 as i32;
                        }
                    }
                } else if (*SignedMeteringDataType).Dynamic_SMDTControlMode_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_Dynamic_SMDTControlModeType(
                            stream,
                            &(*SignedMeteringDataType).Dynamic_SMDTControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_Scheduled_SMDTControlModeType(
                            stream,
                            &(*SignedMeteringDataType).Scheduled_SMDTControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                }
            }
            211 => {
                if (*SignedMeteringDataType).Dynamic_SMDTControlMode_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_Dynamic_SMDTControlModeType(
                            stream,
                            &(*SignedMeteringDataType).Dynamic_SMDTControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_Scheduled_SMDTControlModeType(
                            stream,
                            &(*SignedMeteringDataType).Scheduled_SMDTControlMode,
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
unsafe extern "C" fn encode_iso20_SignedCertificateChainType(
    stream: &mut ExiBitstream,
    mut SignedCertificateChainType: *const iso20_SignedCertificateChainType,
) -> i32 {
    let mut grammar_id: i32 = 212 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            212 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = exi_basetypes_encoder_uint_16(
                        stream,
                        ((*SignedCertificateChainType).Id.charactersLen as i32 + 2 as i32) as u16,
                    );
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_characters(
                            stream,
                            (*SignedCertificateChainType).Id.charactersLen as usize,
                            ((*SignedCertificateChainType).Id.characters).as_ptr(),
                            (64 as i32 + 1 as i32) as usize,
                        );
                        if error == 0 as i32 {
                            grammar_id = 213 as i32;
                        }
                    }
                }
            }
            213 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*SignedCertificateChainType).Certificate.bytesLen,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*SignedCertificateChainType).Certificate.bytesLen as usize,
                                ((*SignedCertificateChainType).Certificate.bytes).as_ptr(),
                                1600 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 214 as i32;
                                }
                            }
                        }
                    }
                }
            }
            214 => {
                if (*SignedCertificateChainType).SubCertificates_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_SubCertificatesType(
                            stream,
                            &(*SignedCertificateChainType).SubCertificates,
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
unsafe extern "C" fn encode_iso20_EIM_AReqAuthorizationModeType(
    stream: &mut ExiBitstream,
    mut EIM_AReqAuthorizationModeType: *const iso20_EIM_AReqAuthorizationModeType,
) -> i32 {
    let mut error: i32 =
        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
    return error;
}
unsafe extern "C" fn encode_iso20_SelectedServiceListType(
    stream: &mut ExiBitstream,
    mut SelectedServiceListType: *const iso20_SelectedServiceListType,
) -> i32 {
    let mut grammar_id: i32 = 215 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut SelectedService_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            215 => {
                if (SelectedService_currentIndex as i32)
                    < (*SelectedServiceListType).SelectedService.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh42 = SelectedService_currentIndex;
                        SelectedService_currentIndex = SelectedService_currentIndex.wrapping_add(1);
                        error = encode_iso20_SelectedServiceType(
                            stream,
                            &*((*SelectedServiceListType).SelectedService.array)
                                .as_ptr()
                                .offset(fresh42 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 216 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            216 => {
                if (SelectedService_currentIndex as i32)
                    < (*SelectedServiceListType).SelectedService.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh43 = SelectedService_currentIndex;
                        SelectedService_currentIndex = SelectedService_currentIndex.wrapping_add(1);
                        error = encode_iso20_SelectedServiceType(
                            stream,
                            &*((*SelectedServiceListType).SelectedService.array)
                                .as_ptr()
                                .offset(fresh43 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 216 as i32;
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
unsafe extern "C" fn encode_iso20_Dynamic_SEReqControlModeType(
    stream: &mut ExiBitstream,
    mut Dynamic_SEReqControlModeType: *const iso20_Dynamic_SEReqControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 217 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            217 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_32(
                            stream,
                            (*Dynamic_SEReqControlModeType).DepartureTime,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 218 as i32;
                            }
                        }
                    }
                }
            }
            218 => {
                if (*Dynamic_SEReqControlModeType).MinimumSOC_isUsed() == 1 as u32 {
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
                                (*Dynamic_SEReqControlModeType).MinimumSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 219 as i32;
                                }
                            }
                        }
                    }
                } else if (*Dynamic_SEReqControlModeType).TargetSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
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
                                (*Dynamic_SEReqControlModeType).TargetSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 220 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*Dynamic_SEReqControlModeType).EVTargetEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 221 as i32;
                        }
                    }
                }
            }
            219 => {
                if (*Dynamic_SEReqControlModeType).TargetSOC_isUsed() == 1 as u32 {
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
                                (*Dynamic_SEReqControlModeType).TargetSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 220 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*Dynamic_SEReqControlModeType).EVTargetEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 221 as i32;
                        }
                    }
                }
            }
            220 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_RationalNumberType(
                        stream,
                        &(*Dynamic_SEReqControlModeType).EVTargetEnergyRequest,
                    );
                    if error == 0 as i32 {
                        grammar_id = 221 as i32;
                    }
                }
            }
            221 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_RationalNumberType(
                        stream,
                        &(*Dynamic_SEReqControlModeType).EVMaximumEnergyRequest,
                    );
                    if error == 0 as i32 {
                        grammar_id = 222 as i32;
                    }
                }
            }
            222 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_RationalNumberType(
                        stream,
                        &(*Dynamic_SEReqControlModeType).EVMinimumEnergyRequest,
                    );
                    if error == 0 as i32 {
                        grammar_id = 223 as i32;
                    }
                }
            }
            223 => {
                if (*Dynamic_SEReqControlModeType).EVMaximumV2XEnergyRequest_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*Dynamic_SEReqControlModeType).EVMaximumV2XEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 224 as i32;
                        }
                    }
                } else if (*Dynamic_SEReqControlModeType).EVMinimumV2XEnergyRequest_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*Dynamic_SEReqControlModeType).EVMinimumV2XEnergyRequest,
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
            224 => {
                if (*Dynamic_SEReqControlModeType).EVMinimumV2XEnergyRequest_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*Dynamic_SEReqControlModeType).EVMinimumV2XEnergyRequest,
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
unsafe extern "C" fn encode_iso20_EVSEStatusType(
    stream: &mut ExiBitstream,
    mut EVSEStatusType: *const iso20_EVSEStatusType,
) -> i32 {
    let mut grammar_id: i32 = 225 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            225 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
                                grammar_id = 226 as i32;
                            }
                        }
                    }
                }
            }
            226 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
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
unsafe extern "C" fn encode_iso20_ListOfRootCertificateIDsType(
    stream: &mut ExiBitstream,
    mut ListOfRootCertificateIDsType: *const iso20_ListOfRootCertificateIDsType,
) -> i32 {
    let mut grammar_id: i32 = 227 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut RootCertificateID_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            227 => {
                if (RootCertificateID_currentIndex as i32)
                    < (*ListOfRootCertificateIDsType).RootCertificateID.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh44 = RootCertificateID_currentIndex;
                        RootCertificateID_currentIndex =
                            RootCertificateID_currentIndex.wrapping_add(1);
                        error = encode_iso20_X509IssuerSerialType(
                            stream,
                            &*((*ListOfRootCertificateIDsType).RootCertificateID.array)
                                .as_ptr()
                                .offset(fresh44 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 228 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            228 => {
                if (RootCertificateID_currentIndex as i32)
                    < (*ListOfRootCertificateIDsType).RootCertificateID.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh45 = RootCertificateID_currentIndex;
                        RootCertificateID_currentIndex =
                            RootCertificateID_currentIndex.wrapping_add(1);
                        error = encode_iso20_X509IssuerSerialType(
                            stream,
                            &*((*ListOfRootCertificateIDsType).RootCertificateID.array)
                                .as_ptr()
                                .offset(fresh45 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 228 as i32;
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
unsafe extern "C" fn encode_iso20_PnC_AReqAuthorizationModeType(
    stream: &mut ExiBitstream,
    mut PnC_AReqAuthorizationModeType: *const iso20_PnC_AReqAuthorizationModeType,
) -> i32 {
    let mut grammar_id: i32 = 229 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            229 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = exi_basetypes_encoder_uint_16(
                        stream,
                        ((*PnC_AReqAuthorizationModeType).Id.charactersLen as i32 + 2 as i32)
                            as u16,
                    );
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_characters(
                            stream,
                            (*PnC_AReqAuthorizationModeType).Id.charactersLen as usize,
                            ((*PnC_AReqAuthorizationModeType).Id.characters).as_ptr(),
                            (64 as i32 + 1 as i32) as usize,
                        );
                        if error == 0 as i32 {
                            grammar_id = 230 as i32;
                        }
                    }
                }
            }
            230 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*PnC_AReqAuthorizationModeType).GenChallenge.bytesLen,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*PnC_AReqAuthorizationModeType).GenChallenge.bytesLen as usize,
                                ((*PnC_AReqAuthorizationModeType).GenChallenge.bytes).as_ptr(),
                                16 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 231 as i32;
                                }
                            }
                        }
                    }
                }
            }
            231 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_ContractCertificateChainType(
                        stream,
                        &(*PnC_AReqAuthorizationModeType).ContractCertificateChain,
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
unsafe extern "C" fn encode_iso20_ServiceListType(
    stream: &mut ExiBitstream,
    mut ServiceListType: *const iso20_ServiceListType,
) -> i32 {
    let mut grammar_id: i32 = 232 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut Service_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            232 => {
                if (Service_currentIndex as i32) < (*ServiceListType).Service.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh46 = Service_currentIndex;
                        Service_currentIndex = Service_currentIndex.wrapping_add(1);
                        error = encode_iso20_ServiceType(
                            stream,
                            &*((*ServiceListType).Service.array)
                                .as_ptr()
                                .offset(fresh46 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 233 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            233 => {
                if (Service_currentIndex as i32) < (*ServiceListType).Service.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh47 = Service_currentIndex;
                        Service_currentIndex = Service_currentIndex.wrapping_add(1);
                        error = encode_iso20_ServiceType(
                            stream,
                            &*((*ServiceListType).Service.array)
                                .as_ptr()
                                .offset(fresh47 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 233 as i32;
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
unsafe extern "C" fn encode_iso20_ServiceParameterListType(
    stream: &mut ExiBitstream,
    mut ServiceParameterListType: *const iso20_ServiceParameterListType,
) -> i32 {
    let mut grammar_id: i32 = 234 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut ParameterSet_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            234 => {
                if (ParameterSet_currentIndex as i32)
                    < (*ServiceParameterListType).ParameterSet.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh48 = ParameterSet_currentIndex;
                        ParameterSet_currentIndex = ParameterSet_currentIndex.wrapping_add(1);
                        error = encode_iso20_ParameterSetType(
                            stream,
                            &*((*ServiceParameterListType).ParameterSet.array)
                                .as_ptr()
                                .offset(fresh48 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 235 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            235 => {
                if (ParameterSet_currentIndex as i32)
                    < (*ServiceParameterListType).ParameterSet.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh49 = ParameterSet_currentIndex;
                        ParameterSet_currentIndex = ParameterSet_currentIndex.wrapping_add(1);
                        error = encode_iso20_ParameterSetType(
                            stream,
                            &*((*ServiceParameterListType).ParameterSet.array)
                                .as_ptr()
                                .offset(fresh49 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 235 as i32;
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
unsafe extern "C" fn encode_iso20_Scheduled_SEReqControlModeType(
    stream: &mut ExiBitstream,
    mut Scheduled_SEReqControlModeType: *const iso20_Scheduled_SEReqControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 236 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            236 => {
                if (*Scheduled_SEReqControlModeType).DepartureTime_isUsed() == 1 as u32 {
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
                                (*Scheduled_SEReqControlModeType).DepartureTime,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 237 as i32;
                                }
                            }
                        }
                    }
                } else if (*Scheduled_SEReqControlModeType).EVTargetEnergyRequest_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*Scheduled_SEReqControlModeType).EVTargetEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 238 as i32;
                        }
                    }
                } else if (*Scheduled_SEReqControlModeType).EVMaximumEnergyRequest_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*Scheduled_SEReqControlModeType).EVMaximumEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 239 as i32;
                        }
                    }
                } else if (*Scheduled_SEReqControlModeType).EVMinimumEnergyRequest_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*Scheduled_SEReqControlModeType).EVMinimumEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 240 as i32;
                        }
                    }
                } else if (*Scheduled_SEReqControlModeType).EVEnergyOffer_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_EVEnergyOfferType(
                            stream,
                            &(*Scheduled_SEReqControlModeType).EVEnergyOffer,
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
                if (*Scheduled_SEReqControlModeType).EVTargetEnergyRequest_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*Scheduled_SEReqControlModeType).EVTargetEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 238 as i32;
                        }
                    }
                } else if (*Scheduled_SEReqControlModeType).EVMaximumEnergyRequest_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*Scheduled_SEReqControlModeType).EVMaximumEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 239 as i32;
                        }
                    }
                } else if (*Scheduled_SEReqControlModeType).EVMinimumEnergyRequest_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*Scheduled_SEReqControlModeType).EVMinimumEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 240 as i32;
                        }
                    }
                } else if (*Scheduled_SEReqControlModeType).EVEnergyOffer_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_EVEnergyOfferType(
                            stream,
                            &(*Scheduled_SEReqControlModeType).EVEnergyOffer,
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
                if (*Scheduled_SEReqControlModeType).EVMaximumEnergyRequest_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*Scheduled_SEReqControlModeType).EVMaximumEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 239 as i32;
                        }
                    }
                } else if (*Scheduled_SEReqControlModeType).EVMinimumEnergyRequest_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*Scheduled_SEReqControlModeType).EVMinimumEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 240 as i32;
                        }
                    }
                } else if (*Scheduled_SEReqControlModeType).EVEnergyOffer_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_EVEnergyOfferType(
                            stream,
                            &(*Scheduled_SEReqControlModeType).EVEnergyOffer,
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
                if (*Scheduled_SEReqControlModeType).EVMinimumEnergyRequest_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_RationalNumberType(
                            stream,
                            &(*Scheduled_SEReqControlModeType).EVMinimumEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 240 as i32;
                        }
                    }
                } else if (*Scheduled_SEReqControlModeType).EVEnergyOffer_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_EVEnergyOfferType(
                            stream,
                            &(*Scheduled_SEReqControlModeType).EVEnergyOffer,
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
                if (*Scheduled_SEReqControlModeType).EVEnergyOffer_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_EVEnergyOfferType(
                            stream,
                            &(*Scheduled_SEReqControlModeType).EVEnergyOffer,
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
unsafe extern "C" fn encode_iso20_EVPowerProfileType(
    stream: &mut ExiBitstream,
    mut EVPowerProfileType: *const iso20_EVPowerProfileType,
) -> i32 {
    let mut grammar_id: i32 = 241 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            241 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            exi_basetypes_encoder_uint_64(stream, (*EVPowerProfileType).TimeAnchor);
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
            }
            242 => {
                if (*EVPowerProfileType).Dynamic_EVPPTControlMode_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_Dynamic_EVPPTControlModeType(
                            stream,
                            &(*EVPowerProfileType).Dynamic_EVPPTControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 243 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_Scheduled_EVPPTControlModeType(
                            stream,
                            &(*EVPowerProfileType).Scheduled_EVPPTControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 243 as i32;
                        }
                    }
                }
            }
            243 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_EVPowerProfileEntryListType(
                        stream,
                        &(*EVPowerProfileType).EVPowerProfileEntries,
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
unsafe extern "C" fn encode_iso20_CertificateChainType(
    stream: &mut ExiBitstream,
    mut CertificateChainType: *const iso20_CertificateChainType,
) -> i32 {
    let mut grammar_id: i32 = 244 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            244 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*CertificateChainType).Certificate.bytesLen,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*CertificateChainType).Certificate.bytesLen as usize,
                                ((*CertificateChainType).Certificate.bytes).as_ptr(),
                                1600 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 245 as i32;
                                }
                            }
                        }
                    }
                }
            }
            245 => {
                if (*CertificateChainType).SubCertificates_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_SubCertificatesType(
                            stream,
                            &(*CertificateChainType).SubCertificates,
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
unsafe extern "C" fn encode_iso20_EIM_ASResAuthorizationModeType(
    stream: &mut ExiBitstream,
    mut EIM_ASResAuthorizationModeType: *const iso20_EIM_ASResAuthorizationModeType,
) -> i32 {
    let mut error: i32 =
        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
    return error;
}
unsafe extern "C" fn encode_iso20_Dynamic_SEResControlModeType(
    stream: &mut ExiBitstream,
    mut Dynamic_SEResControlModeType: *const iso20_Dynamic_SEResControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 246 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            246 => {
                if (*Dynamic_SEResControlModeType).DepartureTime_isUsed() == 1 as u32 {
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
                                (*Dynamic_SEResControlModeType).DepartureTime,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 247 as i32;
                                }
                            }
                        }
                    }
                } else if (*Dynamic_SEResControlModeType).MinimumSOC_isUsed() == 1 as u32 {
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
                                (*Dynamic_SEResControlModeType).MinimumSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 248 as i32;
                                }
                            }
                        }
                    }
                } else if (*Dynamic_SEResControlModeType).TargetSOC_isUsed() == 1 as u32 {
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
                                (*Dynamic_SEResControlModeType).TargetSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 249 as i32;
                                }
                            }
                        }
                    }
                } else if (*Dynamic_SEResControlModeType).AbsolutePriceSchedule_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_AbsolutePriceScheduleType(
                            stream,
                            &(*Dynamic_SEResControlModeType).AbsolutePriceSchedule,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*Dynamic_SEResControlModeType).PriceLevelSchedule_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_PriceLevelScheduleType(
                            stream,
                            &(*Dynamic_SEResControlModeType).PriceLevelSchedule,
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
            247 => {
                if (*Dynamic_SEResControlModeType).MinimumSOC_isUsed() == 1 as u32 {
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
                                (*Dynamic_SEResControlModeType).MinimumSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 248 as i32;
                                }
                            }
                        }
                    }
                } else if (*Dynamic_SEResControlModeType).TargetSOC_isUsed() == 1 as u32 {
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
                                (*Dynamic_SEResControlModeType).TargetSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 249 as i32;
                                }
                            }
                        }
                    }
                } else if (*Dynamic_SEResControlModeType).AbsolutePriceSchedule_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_AbsolutePriceScheduleType(
                            stream,
                            &(*Dynamic_SEResControlModeType).AbsolutePriceSchedule,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*Dynamic_SEResControlModeType).PriceLevelSchedule_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_PriceLevelScheduleType(
                            stream,
                            &(*Dynamic_SEResControlModeType).PriceLevelSchedule,
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
            248 => {
                if (*Dynamic_SEResControlModeType).TargetSOC_isUsed() == 1 as u32 {
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
                                (*Dynamic_SEResControlModeType).TargetSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 249 as i32;
                                }
                            }
                        }
                    }
                } else if (*Dynamic_SEResControlModeType).AbsolutePriceSchedule_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_AbsolutePriceScheduleType(
                            stream,
                            &(*Dynamic_SEResControlModeType).AbsolutePriceSchedule,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*Dynamic_SEResControlModeType).PriceLevelSchedule_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_PriceLevelScheduleType(
                            stream,
                            &(*Dynamic_SEResControlModeType).PriceLevelSchedule,
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
            249 => {
                if (*Dynamic_SEResControlModeType).AbsolutePriceSchedule_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_AbsolutePriceScheduleType(
                            stream,
                            &(*Dynamic_SEResControlModeType).AbsolutePriceSchedule,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*Dynamic_SEResControlModeType).PriceLevelSchedule_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_PriceLevelScheduleType(
                            stream,
                            &(*Dynamic_SEResControlModeType).PriceLevelSchedule,
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
unsafe extern "C" fn encode_iso20_EMAIDListType(
    stream: &mut ExiBitstream,
    mut EMAIDListType: *const iso20_EMAIDListType,
) -> i32 {
    let mut grammar_id: i32 = 250 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut EMAID_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            250 => {
                if (EMAID_currentIndex as i32) < (*EMAIDListType).EMAID.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                ((*EMAIDListType).EMAID.array[EMAID_currentIndex as usize]
                                    .charactersLen as i32
                                    + 2 as i32) as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*EMAIDListType).EMAID.array[EMAID_currentIndex as usize]
                                        .charactersLen as usize,
                                    ((*EMAIDListType).EMAID.array[EMAID_currentIndex as usize]
                                        .characters)
                                        .as_ptr(),
                                    (255 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    EMAID_currentIndex = EMAID_currentIndex.wrapping_add(1);
                                    EMAID_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 251 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            251 => {
                if (EMAID_currentIndex as i32) < (*EMAIDListType).EMAID.arrayLen as i32 {
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
                                ((*EMAIDListType).EMAID.array[EMAID_currentIndex as usize]
                                    .charactersLen as i32
                                    + 2 as i32) as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*EMAIDListType).EMAID.array[EMAID_currentIndex as usize]
                                        .charactersLen as usize,
                                    ((*EMAIDListType).EMAID.array[EMAID_currentIndex as usize]
                                        .characters)
                                        .as_ptr(),
                                    (255 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    EMAID_currentIndex = EMAID_currentIndex.wrapping_add(1);
                                    EMAID_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 251 as i32;
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
unsafe extern "C" fn encode_iso20_SignedInstallationDataType(
    stream: &mut ExiBitstream,
    mut SignedInstallationDataType: *const iso20_SignedInstallationDataType,
) -> i32 {
    let mut grammar_id: i32 = 252 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            252 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = exi_basetypes_encoder_uint_16(
                        stream,
                        ((*SignedInstallationDataType).Id.charactersLen as i32 + 2 as i32) as u16,
                    );
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_characters(
                            stream,
                            (*SignedInstallationDataType).Id.charactersLen as usize,
                            ((*SignedInstallationDataType).Id.characters).as_ptr(),
                            (64 as i32 + 1 as i32) as usize,
                        );
                        if error == 0 as i32 {
                            grammar_id = 253 as i32;
                        }
                    }
                }
            }
            253 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_ContractCertificateChainType(
                        stream,
                        &(*SignedInstallationDataType).ContractCertificateChain,
                    );
                    if error == 0 as i32 {
                        grammar_id = 254 as i32;
                    }
                }
            }
            254 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            (*SignedInstallationDataType).ECDHCurve as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 255 as i32;
                            }
                        }
                    }
                }
            }
            255 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*SignedInstallationDataType).DHPublicKey.bytesLen,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*SignedInstallationDataType).DHPublicKey.bytesLen as usize,
                                ((*SignedInstallationDataType).DHPublicKey.bytes).as_ptr(),
                                133 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 256 as i32;
                                }
                            }
                        }
                    }
                }
            }
            256 => {
                if (*SignedInstallationDataType).SECP521_EncryptedPrivateKey_isUsed() == 1 as u32 {
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
                                (*SignedInstallationDataType)
                                    .SECP521_EncryptedPrivateKey
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*SignedInstallationDataType)
                                        .SECP521_EncryptedPrivateKey
                                        .bytesLen as usize,
                                    ((*SignedInstallationDataType)
                                        .SECP521_EncryptedPrivateKey
                                        .bytes)
                                        .as_ptr(),
                                    94 as i32 as usize,
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
                } else if (*SignedInstallationDataType).X448_EncryptedPrivateKey_isUsed()
                    == 1 as u32
                {
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
                                (*SignedInstallationDataType)
                                    .X448_EncryptedPrivateKey
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*SignedInstallationDataType)
                                        .X448_EncryptedPrivateKey
                                        .bytesLen as usize,
                                    ((*SignedInstallationDataType).X448_EncryptedPrivateKey.bytes)
                                        .as_ptr(),
                                    84 as i32 as usize,
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
                                (*SignedInstallationDataType)
                                    .TPM_EncryptedPrivateKey
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*SignedInstallationDataType)
                                        .TPM_EncryptedPrivateKey
                                        .bytesLen as usize,
                                    ((*SignedInstallationDataType).TPM_EncryptedPrivateKey.bytes)
                                        .as_ptr(),
                                    206 as i32 as usize,
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
unsafe extern "C" fn encode_iso20_PnC_ASResAuthorizationModeType(
    stream: &mut ExiBitstream,
    mut PnC_ASResAuthorizationModeType: *const iso20_PnC_ASResAuthorizationModeType,
) -> i32 {
    let mut grammar_id: i32 = 257 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            257 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*PnC_ASResAuthorizationModeType).GenChallenge.bytesLen,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*PnC_ASResAuthorizationModeType).GenChallenge.bytesLen as usize,
                                ((*PnC_ASResAuthorizationModeType).GenChallenge.bytes).as_ptr(),
                                16 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 258 as i32;
                                }
                            }
                        }
                    }
                }
            }
            258 => {
                if (*PnC_ASResAuthorizationModeType).SupportedProviders_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_SupportedProvidersListType(
                            stream,
                            &(*PnC_ASResAuthorizationModeType).SupportedProviders,
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
unsafe extern "C" fn encode_iso20_Scheduled_SEResControlModeType(
    stream: &mut ExiBitstream,
    mut Scheduled_SEResControlModeType: *const iso20_Scheduled_SEResControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 259 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut ScheduleTuple_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            259 => {
                if (ScheduleTuple_currentIndex as i32)
                    < (*Scheduled_SEResControlModeType).ScheduleTuple.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh50 = ScheduleTuple_currentIndex;
                        ScheduleTuple_currentIndex = ScheduleTuple_currentIndex.wrapping_add(1);
                        error = encode_iso20_ScheduleTupleType(
                            stream,
                            &*((*Scheduled_SEResControlModeType).ScheduleTuple.array)
                                .as_ptr()
                                .offset(fresh50 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 260 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            260 => {
                if (ScheduleTuple_currentIndex as i32)
                    < (*Scheduled_SEResControlModeType).ScheduleTuple.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh51 = ScheduleTuple_currentIndex;
                        ScheduleTuple_currentIndex = ScheduleTuple_currentIndex.wrapping_add(1);
                        error = encode_iso20_ScheduleTupleType(
                            stream,
                            &*((*Scheduled_SEResControlModeType).ScheduleTuple.array)
                                .as_ptr()
                                .offset(fresh51 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 260 as i32;
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
unsafe extern "C" fn encode_iso20_SessionSetupReqType(
    stream: &mut ExiBitstream,
    mut SessionSetupReqType: *const iso20_SessionSetupReqType,
) -> i32 {
    let mut grammar_id: i32 = 261 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            261 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_MessageHeaderType(stream, &(*SessionSetupReqType).Header);
                    if error == 0 as i32 {
                        grammar_id = 262 as i32;
                    }
                }
            }
            262 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*SessionSetupReqType).EVCCID.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*SessionSetupReqType).EVCCID.charactersLen as usize,
                                ((*SessionSetupReqType).EVCCID.characters).as_ptr(),
                                (255 as i32 + 1 as i32) as usize,
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
unsafe extern "C" fn encode_iso20_SessionSetupResType(
    stream: &mut ExiBitstream,
    mut SessionSetupResType: *const iso20_SessionSetupResType,
) -> i32 {
    let mut grammar_id: i32 = 263 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            263 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_MessageHeaderType(stream, &(*SessionSetupResType).Header);
                    if error == 0 as i32 {
                        grammar_id = 264 as i32;
                    }
                }
            }
            264 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            6 as i32 as usize,
                            (*SessionSetupResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 265 as i32;
                            }
                        }
                    }
                }
            }
            265 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*SessionSetupResType).EVSEID.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*SessionSetupResType).EVSEID.charactersLen as usize,
                                ((*SessionSetupResType).EVSEID.characters).as_ptr(),
                                (255 as i32 + 1 as i32) as usize,
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
unsafe extern "C" fn encode_iso20_AuthorizationSetupReqType(
    stream: &mut ExiBitstream,
    mut AuthorizationSetupReqType: *const iso20_AuthorizationSetupReqType,
) -> i32 {
    let mut grammar_id: i32 = 266 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            266 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_MessageHeaderType(
                        stream,
                        &(*AuthorizationSetupReqType).Header,
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
unsafe extern "C" fn encode_iso20_AuthorizationSetupResType(
    stream: &mut ExiBitstream,
    mut AuthorizationSetupResType: *const iso20_AuthorizationSetupResType,
) -> i32 {
    let mut grammar_id: i32 = 267 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut AuthorizationServices_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            267 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_MessageHeaderType(
                        stream,
                        &(*AuthorizationSetupResType).Header,
                    );
                    if error == 0 as i32 {
                        grammar_id = 268 as i32;
                    }
                }
            }
            268 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            6 as i32 as usize,
                            (*AuthorizationSetupResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 269 as i32;
                            }
                        }
                    }
                }
            }
            269 => {
                if (AuthorizationServices_currentIndex as i32)
                    < (*AuthorizationSetupResType).AuthorizationServices.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            let fresh52 = AuthorizationServices_currentIndex;
                            AuthorizationServices_currentIndex =
                                AuthorizationServices_currentIndex.wrapping_add(1);
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                (*AuthorizationSetupResType).AuthorizationServices.array
                                    [fresh52 as usize] as u32,
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
                } else {
                    error = -(150 as i32);
                }
            }
            270 => {
                if (AuthorizationServices_currentIndex as i32)
                    < (*AuthorizationSetupResType).AuthorizationServices.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            let fresh53 = AuthorizationServices_currentIndex;
                            AuthorizationServices_currentIndex =
                                AuthorizationServices_currentIndex.wrapping_add(1);
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                (*AuthorizationSetupResType).AuthorizationServices.array
                                    [fresh53 as usize] as u32,
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
                                (*AuthorizationSetupResType).CertificateInstallationService,
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
                }
            }
            271 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(
                            stream,
                            (*AuthorizationSetupResType).CertificateInstallationService,
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
            }
            272 => {
                if (*AuthorizationSetupResType).EIM_ASResAuthorizationMode_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_EIM_ASResAuthorizationModeType(
                            stream,
                            &(*AuthorizationSetupResType).EIM_ASResAuthorizationMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_PnC_ASResAuthorizationModeType(
                            stream,
                            &(*AuthorizationSetupResType).PnC_ASResAuthorizationMode,
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
unsafe extern "C" fn encode_iso20_AuthorizationReqType(
    stream: &mut ExiBitstream,
    mut AuthorizationReqType: *const iso20_AuthorizationReqType,
) -> i32 {
    let mut grammar_id: i32 = 273 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            273 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_MessageHeaderType(stream, &(*AuthorizationReqType).Header);
                    if error == 0 as i32 {
                        grammar_id = 274 as i32;
                    }
                }
            }
            274 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            (*AuthorizationReqType).SelectedAuthorizationService as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 275 as i32;
                            }
                        }
                    }
                }
            }
            275 => {
                if (*AuthorizationReqType).EIM_AReqAuthorizationMode_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_EIM_AReqAuthorizationModeType(
                            stream,
                            &(*AuthorizationReqType).EIM_AReqAuthorizationMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_PnC_AReqAuthorizationModeType(
                            stream,
                            &(*AuthorizationReqType).PnC_AReqAuthorizationMode,
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
unsafe extern "C" fn encode_iso20_AuthorizationResType(
    stream: &mut ExiBitstream,
    mut AuthorizationResType: *const iso20_AuthorizationResType,
) -> i32 {
    let mut grammar_id: i32 = 276 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            276 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_MessageHeaderType(stream, &(*AuthorizationResType).Header);
                    if error == 0 as i32 {
                        grammar_id = 277 as i32;
                    }
                }
            }
            277 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            6 as i32 as usize,
                            (*AuthorizationResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 278 as i32;
                            }
                        }
                    }
                }
            }
            278 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*AuthorizationResType).EVSEProcessing as u32,
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
unsafe extern "C" fn encode_iso20_ServiceDiscoveryReqType(
    stream: &mut ExiBitstream,
    mut ServiceDiscoveryReqType: *const iso20_ServiceDiscoveryReqType,
) -> i32 {
    let mut grammar_id: i32 = 279 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            279 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso20_MessageHeaderType(stream, &(*ServiceDiscoveryReqType).Header);
                    if error == 0 as i32 {
                        grammar_id = 280 as i32;
                    }
                }
            }
            280 => {
                if (*ServiceDiscoveryReqType).SupportedServiceIDs_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ServiceIDListType(
                            stream,
                            &(*ServiceDiscoveryReqType).SupportedServiceIDs,
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
unsafe extern "C" fn encode_iso20_ServiceDiscoveryResType(
    stream: &mut ExiBitstream,
    mut ServiceDiscoveryResType: *const iso20_ServiceDiscoveryResType,
) -> i32 {
    let mut grammar_id: i32 = 281 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            281 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso20_MessageHeaderType(stream, &(*ServiceDiscoveryResType).Header);
                    if error == 0 as i32 {
                        grammar_id = 282 as i32;
                    }
                }
            }
            282 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            6 as i32 as usize,
                            (*ServiceDiscoveryResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 283 as i32;
                            }
                        }
                    }
                }
            }
            283 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(
                            stream,
                            (*ServiceDiscoveryResType).ServiceRenegotiationSupported,
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_ServiceListType(
                        stream,
                        &(*ServiceDiscoveryResType).EnergyTransferServiceList,
                    );
                    if error == 0 as i32 {
                        grammar_id = 285 as i32;
                    }
                }
            }
            285 => {
                if (*ServiceDiscoveryResType).VASList_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_ServiceListType(
                            stream,
                            &(*ServiceDiscoveryResType).VASList,
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
unsafe extern "C" fn encode_iso20_ServiceDetailReqType(
    stream: &mut ExiBitstream,
    mut ServiceDetailReqType: *const iso20_ServiceDetailReqType,
) -> i32 {
    let mut grammar_id: i32 = 286 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            286 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_MessageHeaderType(stream, &(*ServiceDetailReqType).Header);
                    if error == 0 as i32 {
                        grammar_id = 287 as i32;
                    }
                }
            }
            287 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*ServiceDetailReqType).ServiceID,
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
unsafe extern "C" fn encode_iso20_ServiceDetailResType(
    stream: &mut ExiBitstream,
    mut ServiceDetailResType: *const iso20_ServiceDetailResType,
) -> i32 {
    let mut grammar_id: i32 = 288 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            288 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_MessageHeaderType(stream, &(*ServiceDetailResType).Header);
                    if error == 0 as i32 {
                        grammar_id = 289 as i32;
                    }
                }
            }
            289 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            6 as i32 as usize,
                            (*ServiceDetailResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 290 as i32;
                            }
                        }
                    }
                }
            }
            290 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*ServiceDetailResType).ServiceID,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 291 as i32;
                            }
                        }
                    }
                }
            }
            291 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_ServiceParameterListType(
                        stream,
                        &(*ServiceDetailResType).ServiceParameterList,
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
unsafe extern "C" fn encode_iso20_ServiceSelectionReqType(
    stream: &mut ExiBitstream,
    mut ServiceSelectionReqType: *const iso20_ServiceSelectionReqType,
) -> i32 {
    let mut grammar_id: i32 = 292 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            292 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso20_MessageHeaderType(stream, &(*ServiceSelectionReqType).Header);
                    if error == 0 as i32 {
                        grammar_id = 293 as i32;
                    }
                }
            }
            293 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_SelectedServiceType(
                        stream,
                        &(*ServiceSelectionReqType).SelectedEnergyTransferService,
                    );
                    if error == 0 as i32 {
                        grammar_id = 294 as i32;
                    }
                }
            }
            294 => {
                if (*ServiceSelectionReqType).SelectedVASList_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_SelectedServiceListType(
                            stream,
                            &(*ServiceSelectionReqType).SelectedVASList,
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
unsafe extern "C" fn encode_iso20_ServiceSelectionResType(
    stream: &mut ExiBitstream,
    mut ServiceSelectionResType: *const iso20_ServiceSelectionResType,
) -> i32 {
    let mut grammar_id: i32 = 295 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            295 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso20_MessageHeaderType(stream, &(*ServiceSelectionResType).Header);
                    if error == 0 as i32 {
                        grammar_id = 296 as i32;
                    }
                }
            }
            296 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            6 as i32 as usize,
                            (*ServiceSelectionResType).ResponseCode as u32,
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
unsafe extern "C" fn encode_iso20_ScheduleExchangeReqType(
    stream: &mut ExiBitstream,
    mut ScheduleExchangeReqType: *const iso20_ScheduleExchangeReqType,
) -> i32 {
    let mut grammar_id: i32 = 297 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            297 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso20_MessageHeaderType(stream, &(*ScheduleExchangeReqType).Header);
                    if error == 0 as i32 {
                        grammar_id = 298 as i32;
                    }
                }
            }
            298 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            10 as i32 as usize,
                            ((*ScheduleExchangeReqType).MaximumSupportingPoints as u32)
                                .wrapping_sub(12 as i32 as u32),
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 299 as i32;
                            }
                        }
                    }
                }
            }
            299 => {
                if (*ScheduleExchangeReqType).Dynamic_SEReqControlMode_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_Dynamic_SEReqControlModeType(
                            stream,
                            &(*ScheduleExchangeReqType).Dynamic_SEReqControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_Scheduled_SEReqControlModeType(
                            stream,
                            &(*ScheduleExchangeReqType).Scheduled_SEReqControlMode,
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
unsafe extern "C" fn encode_iso20_ScheduleExchangeResType(
    stream: &mut ExiBitstream,
    mut ScheduleExchangeResType: *const iso20_ScheduleExchangeResType,
) -> i32 {
    let mut grammar_id: i32 = 300 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            300 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso20_MessageHeaderType(stream, &(*ScheduleExchangeResType).Header);
                    if error == 0 as i32 {
                        grammar_id = 301 as i32;
                    }
                }
            }
            301 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            6 as i32 as usize,
                            (*ScheduleExchangeResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 302 as i32;
                            }
                        }
                    }
                }
            }
            302 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*ScheduleExchangeResType).EVSEProcessing as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 303 as i32;
                            }
                        }
                    }
                }
            }
            303 => {
                if (*ScheduleExchangeResType).GoToPause_isUsed() == 1 as u32 {
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
                                (*ScheduleExchangeResType).GoToPause,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 304 as i32;
                                }
                            }
                        }
                    }
                } else if (*ScheduleExchangeResType).Dynamic_SEResControlMode_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_Dynamic_SEResControlModeType(
                            stream,
                            &(*ScheduleExchangeResType).Dynamic_SEResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_Scheduled_SEResControlModeType(
                            stream,
                            &(*ScheduleExchangeResType).Scheduled_SEResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                }
            }
            304 => {
                if (*ScheduleExchangeResType).Dynamic_SEResControlMode_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_Dynamic_SEResControlModeType(
                            stream,
                            &(*ScheduleExchangeResType).Dynamic_SEResControlMode,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_Scheduled_SEResControlModeType(
                            stream,
                            &(*ScheduleExchangeResType).Scheduled_SEResControlMode,
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
unsafe extern "C" fn encode_iso20_PowerDeliveryReqType(
    stream: &mut ExiBitstream,
    mut PowerDeliveryReqType: *const iso20_PowerDeliveryReqType,
) -> i32 {
    let mut grammar_id: i32 = 305 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            305 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_MessageHeaderType(stream, &(*PowerDeliveryReqType).Header);
                    if error == 0 as i32 {
                        grammar_id = 306 as i32;
                    }
                }
            }
            306 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*PowerDeliveryReqType).EVProcessing as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 307 as i32;
                            }
                        }
                    }
                }
            }
            307 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*PowerDeliveryReqType).ChargeProgress as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 308 as i32;
                            }
                        }
                    }
                }
            }
            308 => {
                if (*PowerDeliveryReqType).EVPowerProfile_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_EVPowerProfileType(
                            stream,
                            &(*PowerDeliveryReqType).EVPowerProfile,
                        );
                        if error == 0 as i32 {
                            grammar_id = 309 as i32;
                        }
                    }
                } else if (*PowerDeliveryReqType).BPT_ChannelSelection_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                (*PowerDeliveryReqType).BPT_ChannelSelection as u32,
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
            309 => {
                if (*PowerDeliveryReqType).BPT_ChannelSelection_isUsed() == 1 as u32 {
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
                                1 as i32 as usize,
                                (*PowerDeliveryReqType).BPT_ChannelSelection as u32,
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
unsafe extern "C" fn encode_iso20_PowerDeliveryResType(
    stream: &mut ExiBitstream,
    mut PowerDeliveryResType: *const iso20_PowerDeliveryResType,
) -> i32 {
    let mut grammar_id: i32 = 310 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            310 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_MessageHeaderType(stream, &(*PowerDeliveryResType).Header);
                    if error == 0 as i32 {
                        grammar_id = 311 as i32;
                    }
                }
            }
            311 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            6 as i32 as usize,
                            (*PowerDeliveryResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 312 as i32;
                            }
                        }
                    }
                }
            }
            312 => {
                if (*PowerDeliveryResType).EVSEStatus_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_EVSEStatusType(
                            stream,
                            &(*PowerDeliveryResType).EVSEStatus,
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
unsafe extern "C" fn encode_iso20_MeteringConfirmationReqType(
    stream: &mut ExiBitstream,
    mut MeteringConfirmationReqType: *const iso20_MeteringConfirmationReqType,
) -> i32 {
    let mut grammar_id: i32 = 313 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            313 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_MessageHeaderType(
                        stream,
                        &(*MeteringConfirmationReqType).Header,
                    );
                    if error == 0 as i32 {
                        grammar_id = 314 as i32;
                    }
                }
            }
            314 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_SignedMeteringDataType(
                        stream,
                        &(*MeteringConfirmationReqType).SignedMeteringData,
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
unsafe extern "C" fn encode_iso20_MeteringConfirmationResType(
    stream: &mut ExiBitstream,
    mut MeteringConfirmationResType: *const iso20_MeteringConfirmationResType,
) -> i32 {
    let mut grammar_id: i32 = 315 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            315 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_MessageHeaderType(
                        stream,
                        &(*MeteringConfirmationResType).Header,
                    );
                    if error == 0 as i32 {
                        grammar_id = 316 as i32;
                    }
                }
            }
            316 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            6 as i32 as usize,
                            (*MeteringConfirmationResType).ResponseCode as u32,
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
unsafe extern "C" fn encode_iso20_SessionStopReqType(
    stream: &mut ExiBitstream,
    mut SessionStopReqType: *const iso20_SessionStopReqType,
) -> i32 {
    let mut grammar_id: i32 = 317 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            317 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_MessageHeaderType(stream, &(*SessionStopReqType).Header);
                    if error == 0 as i32 {
                        grammar_id = 318 as i32;
                    }
                }
            }
            318 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*SessionStopReqType).ChargingSession as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 319 as i32;
                            }
                        }
                    }
                }
            }
            319 => {
                if (*SessionStopReqType).EVTerminationCode_isUsed() == 1 as u32 {
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
                                ((*SessionStopReqType).EVTerminationCode.charactersLen as i32
                                    + 2 as i32) as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*SessionStopReqType).EVTerminationCode.charactersLen as usize,
                                    ((*SessionStopReqType).EVTerminationCode.characters).as_ptr(),
                                    (80 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 320 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*SessionStopReqType).EVTerminationExplanation_isUsed() == 1 as u32 {
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
                                ((*SessionStopReqType).EVTerminationExplanation.charactersLen
                                    as i32
                                    + 2 as i32) as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*SessionStopReqType).EVTerminationExplanation.charactersLen
                                        as usize,
                                    ((*SessionStopReqType).EVTerminationExplanation.characters)
                                        .as_ptr(),
                                    (160 as i32 + 1 as i32) as usize,
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
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            320 => {
                if (*SessionStopReqType).EVTerminationExplanation_isUsed() == 1 as u32 {
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
                                ((*SessionStopReqType).EVTerminationExplanation.charactersLen
                                    as i32
                                    + 2 as i32) as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*SessionStopReqType).EVTerminationExplanation.charactersLen
                                        as usize,
                                    ((*SessionStopReqType).EVTerminationExplanation.characters)
                                        .as_ptr(),
                                    (160 as i32 + 1 as i32) as usize,
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
unsafe extern "C" fn encode_iso20_SessionStopResType(
    stream: &mut ExiBitstream,
    mut SessionStopResType: *const iso20_SessionStopResType,
) -> i32 {
    let mut grammar_id: i32 = 321 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            321 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_MessageHeaderType(stream, &(*SessionStopResType).Header);
                    if error == 0 as i32 {
                        grammar_id = 322 as i32;
                    }
                }
            }
            322 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            6 as i32 as usize,
                            (*SessionStopResType).ResponseCode as u32,
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
unsafe extern "C" fn encode_iso20_CertificateInstallationReqType(
    stream: &mut ExiBitstream,
    mut CertificateInstallationReqType: *const iso20_CertificateInstallationReqType,
) -> i32 {
    let mut grammar_id: i32 = 323 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            323 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_MessageHeaderType(
                        stream,
                        &(*CertificateInstallationReqType).Header,
                    );
                    if error == 0 as i32 {
                        grammar_id = 324 as i32;
                    }
                }
            }
            324 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_SignedCertificateChainType(
                        stream,
                        &(*CertificateInstallationReqType).OEMProvisioningCertificateChain,
                    );
                    if error == 0 as i32 {
                        grammar_id = 325 as i32;
                    }
                }
            }
            325 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_ListOfRootCertificateIDsType(
                        stream,
                        &(*CertificateInstallationReqType).ListOfRootCertificateIDs,
                    );
                    if error == 0 as i32 {
                        grammar_id = 326 as i32;
                    }
                }
            }
            326 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            8 as i32 as usize,
                            (*CertificateInstallationReqType).MaximumContractCertificateChains
                                as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 327 as i32;
                            }
                        }
                    }
                }
            }
            327 => {
                if (*CertificateInstallationReqType).PrioritizedEMAIDs_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_EMAIDListType(
                            stream,
                            &(*CertificateInstallationReqType).PrioritizedEMAIDs,
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
unsafe extern "C" fn encode_iso20_CertificateInstallationResType(
    stream: &mut ExiBitstream,
    mut CertificateInstallationResType: *const iso20_CertificateInstallationResType,
) -> i32 {
    let mut grammar_id: i32 = 328 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            328 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_MessageHeaderType(
                        stream,
                        &(*CertificateInstallationResType).Header,
                    );
                    if error == 0 as i32 {
                        grammar_id = 329 as i32;
                    }
                }
            }
            329 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            6 as i32 as usize,
                            (*CertificateInstallationResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 330 as i32;
                            }
                        }
                    }
                }
            }
            330 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*CertificateInstallationResType).EVSEProcessing as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 331 as i32;
                            }
                        }
                    }
                }
            }
            331 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_CertificateChainType(
                        stream,
                        &(*CertificateInstallationResType).CPSCertificateChain,
                    );
                    if error == 0 as i32 {
                        grammar_id = 332 as i32;
                    }
                }
            }
            332 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_SignedInstallationDataType(
                        stream,
                        &(*CertificateInstallationResType).SignedInstallationData,
                    );
                    if error == 0 as i32 {
                        grammar_id = 333 as i32;
                    }
                }
            }
            333 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            8 as i32 as usize,
                            (*CertificateInstallationResType).RemainingContractCertificateChains
                                as u32,
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
unsafe extern "C" fn encode_iso20_VehicleCheckInReqType(
    stream: &mut ExiBitstream,
    mut VehicleCheckInReqType: *const iso20_VehicleCheckInReqType,
) -> i32 {
    let mut grammar_id: i32 = 334 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            334 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso20_MessageHeaderType(stream, &(*VehicleCheckInReqType).Header);
                    if error == 0 as i32 {
                        grammar_id = 335 as i32;
                    }
                }
            }
            335 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*VehicleCheckInReqType).EVCheckInStatus as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 336 as i32;
                            }
                        }
                    }
                }
            }
            336 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*VehicleCheckInReqType).ParkingMethod as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 337 as i32;
                            }
                        }
                    }
                }
            }
            337 => {
                if (*VehicleCheckInReqType).VehicleFrame_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_integer_16(
                                stream,
                                (*VehicleCheckInReqType).VehicleFrame,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 338 as i32;
                                }
                            }
                        }
                    }
                } else if (*VehicleCheckInReqType).DeviceOffset_isUsed() == 1 as u32 {
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
                                (*VehicleCheckInReqType).DeviceOffset,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 339 as i32;
                                }
                            }
                        }
                    }
                } else if (*VehicleCheckInReqType).VehicleTravel_isUsed() == 1 as u32 {
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
                                (*VehicleCheckInReqType).VehicleTravel,
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
            338 => {
                if (*VehicleCheckInReqType).DeviceOffset_isUsed() == 1 as u32 {
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
                                (*VehicleCheckInReqType).DeviceOffset,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 339 as i32;
                                }
                            }
                        }
                    }
                } else if (*VehicleCheckInReqType).VehicleTravel_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_integer_16(
                                stream,
                                (*VehicleCheckInReqType).VehicleTravel,
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
            339 => {
                if (*VehicleCheckInReqType).VehicleTravel_isUsed() == 1 as u32 {
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
                                (*VehicleCheckInReqType).VehicleTravel,
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
unsafe extern "C" fn encode_iso20_VehicleCheckInResType(
    stream: &mut ExiBitstream,
    mut VehicleCheckInResType: *const iso20_VehicleCheckInResType,
) -> i32 {
    let mut grammar_id: i32 = 340 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            340 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso20_MessageHeaderType(stream, &(*VehicleCheckInResType).Header);
                    if error == 0 as i32 {
                        grammar_id = 341 as i32;
                    }
                }
            }
            341 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            6 as i32 as usize,
                            (*VehicleCheckInResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 342 as i32;
                            }
                        }
                    }
                }
            }
            342 => {
                if (*VehicleCheckInResType).ParkingSpace_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_integer_16(
                                stream,
                                (*VehicleCheckInResType).ParkingSpace,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 343 as i32;
                                }
                            }
                        }
                    }
                } else if (*VehicleCheckInResType).DeviceLocation_isUsed() == 1 as u32 {
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
                                (*VehicleCheckInResType).DeviceLocation,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 344 as i32;
                                }
                            }
                        }
                    }
                } else if (*VehicleCheckInResType).TargetDistance_isUsed() == 1 as u32 {
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
                                (*VehicleCheckInResType).TargetDistance,
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
            343 => {
                if (*VehicleCheckInResType).DeviceLocation_isUsed() == 1 as u32 {
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
                                (*VehicleCheckInResType).DeviceLocation,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 344 as i32;
                                }
                            }
                        }
                    }
                } else if (*VehicleCheckInResType).TargetDistance_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_integer_16(
                                stream,
                                (*VehicleCheckInResType).TargetDistance,
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
            344 => {
                if (*VehicleCheckInResType).TargetDistance_isUsed() == 1 as u32 {
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
                                (*VehicleCheckInResType).TargetDistance,
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
unsafe extern "C" fn encode_iso20_VehicleCheckOutReqType(
    stream: &mut ExiBitstream,
    mut VehicleCheckOutReqType: *const iso20_VehicleCheckOutReqType,
) -> i32 {
    let mut grammar_id: i32 = 345 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            345 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso20_MessageHeaderType(stream, &(*VehicleCheckOutReqType).Header);
                    if error == 0 as i32 {
                        grammar_id = 346 as i32;
                    }
                }
            }
            346 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*VehicleCheckOutReqType).EVCheckOutStatus as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 347 as i32;
                            }
                        }
                    }
                }
            }
            347 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_64(
                            stream,
                            (*VehicleCheckOutReqType).CheckOutTime,
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
unsafe extern "C" fn encode_iso20_VehicleCheckOutResType(
    stream: &mut ExiBitstream,
    mut VehicleCheckOutResType: *const iso20_VehicleCheckOutResType,
) -> i32 {
    let mut grammar_id: i32 = 348 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            348 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso20_MessageHeaderType(stream, &(*VehicleCheckOutResType).Header);
                    if error == 0 as i32 {
                        grammar_id = 349 as i32;
                    }
                }
            }
            349 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            6 as i32 as usize,
                            (*VehicleCheckOutResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 350 as i32;
                            }
                        }
                    }
                }
            }
            350 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            (*VehicleCheckOutResType).EVSECheckOutStatus as u32,
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
unsafe extern "C" fn encode_iso20_CLReqControlModeType(
    stream: &mut ExiBitstream,
    mut CLReqControlModeType: *const iso20_CLReqControlModeType,
) -> i32 {
    let mut error: i32 =
        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
    return error;
}
unsafe extern "C" fn encode_iso20_CLResControlModeType(
    stream: &mut ExiBitstream,
    mut CLResControlModeType: *const iso20_CLResControlModeType,
) -> i32 {
    let mut error: i32 =
        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
    return error;
}
unsafe extern "C" fn encode_iso20_ManifestType(
    stream: &mut ExiBitstream,
    mut ManifestType: *const iso20_ManifestType,
) -> i32 {
    let mut grammar_id: i32 = 351 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut Reference_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            351 => {
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
                                grammar_id = 353 as i32;
                            }
                        }
                    }
                } else if (Reference_currentIndex as i32)
                    < (*ManifestType).Reference.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh54 = Reference_currentIndex;
                        Reference_currentIndex = Reference_currentIndex.wrapping_add(1);
                        error = encode_iso20_ReferenceType(
                            stream,
                            &*((*ManifestType).Reference.array)
                                .as_ptr()
                                .offset(fresh54 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 352 as i32;
                        }
                    }
                }
            }
            352 => {
                if (Reference_currentIndex as i32) < (*ManifestType).Reference.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh55 = Reference_currentIndex;
                        Reference_currentIndex = Reference_currentIndex.wrapping_add(1);
                        error = encode_iso20_ReferenceType(
                            stream,
                            &*((*ManifestType).Reference.array)
                                .as_ptr()
                                .offset(fresh55 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 352 as i32;
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
            353 => {
                if (Reference_currentIndex as i32) < (*ManifestType).Reference.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh56 = Reference_currentIndex;
                        Reference_currentIndex = Reference_currentIndex.wrapping_add(1);
                        error = encode_iso20_ReferenceType(
                            stream,
                            &*((*ManifestType).Reference.array)
                                .as_ptr()
                                .offset(fresh56 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 354 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            354 => {
                if (Reference_currentIndex as i32) < (*ManifestType).Reference.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh57 = Reference_currentIndex;
                        Reference_currentIndex = Reference_currentIndex.wrapping_add(1);
                        error = encode_iso20_ReferenceType(
                            stream,
                            &*((*ManifestType).Reference.array)
                                .as_ptr()
                                .offset(fresh57 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 354 as i32;
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
unsafe extern "C" fn encode_iso20_SignaturePropertiesType(
    stream: &mut ExiBitstream,
    mut SignaturePropertiesType: *const iso20_SignaturePropertiesType,
) -> i32 {
    let mut grammar_id: i32 = 355 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            355 => {
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
                                grammar_id = 357 as i32;
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_SignaturePropertyType(
                            stream,
                            &(*SignaturePropertiesType).SignatureProperty,
                        );
                        if error == 0 as i32 {
                            grammar_id = 356 as i32;
                        }
                    }
                }
            }
            356 => {
                if 1 as i32 == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_SignaturePropertyType(
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
            357 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_SignaturePropertyType(
                        stream,
                        &(*SignaturePropertiesType).SignatureProperty,
                    );
                    if error == 0 as i32 {
                        grammar_id = 358 as i32;
                    }
                }
            }
            358 => {
                if 1 as i32 == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_SignaturePropertyType(
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

pub unsafe extern "C" fn encode_iso20_exiDocument(
    stream: &mut ExiBitstream,
    mut exiDoc: *mut iso20_exiDocument,
) -> i32 {
    let mut error: i32 = exi_header_write(stream);
    if error == 0 as i32 {
        if (*exiDoc).AuthorizationReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 0 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_AuthorizationReqType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.AuthorizationReq,
                );
            }
        } else if (*exiDoc).AuthorizationRes_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 1 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_AuthorizationResType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.AuthorizationRes,
                );
            }
        } else if (*exiDoc).AuthorizationSetupReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 2 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_AuthorizationSetupReqType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.AuthorizationSetupReq,
                );
            }
        } else if (*exiDoc).AuthorizationSetupRes_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 3 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_AuthorizationSetupResType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.AuthorizationSetupRes,
                );
            }
        } else if (*exiDoc).CLReqControlMode_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 4 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_CLReqControlModeType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.CLReqControlMode,
                );
            }
        } else if (*exiDoc).CLResControlMode_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 5 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_CLResControlModeType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.CLResControlMode,
                );
            }
        } else if (*exiDoc).CanonicalizationMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 6 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_CanonicalizationMethodType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.CanonicalizationMethod,
                );
            }
        } else if (*exiDoc).CertificateInstallationReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 7 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_CertificateInstallationReqType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.CertificateInstallationReq,
                );
            }
        } else if (*exiDoc).CertificateInstallationRes_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 8 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_CertificateInstallationResType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.CertificateInstallationRes,
                );
            }
        } else if (*exiDoc).DSAKeyValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 9 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_DSAKeyValueType(stream, &mut (*exiDoc).c2rust_unnamed.DSAKeyValue);
            }
        } else if (*exiDoc).DigestMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 10 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_DigestMethodType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.DigestMethod,
                );
            }
        } else if (*exiDoc).KeyInfo_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 12 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_KeyInfoType(stream, &mut (*exiDoc).c2rust_unnamed.KeyInfo);
            }
        } else if (*exiDoc).KeyValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 14 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_KeyValueType(stream, &mut (*exiDoc).c2rust_unnamed.KeyValue);
            }
        } else if (*exiDoc).Manifest_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 15 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ManifestType(stream, &mut (*exiDoc).c2rust_unnamed.Manifest);
            }
        } else if (*exiDoc).MeteringConfirmationReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 16 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_MeteringConfirmationReqType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.MeteringConfirmationReq,
                );
            }
        } else if (*exiDoc).MeteringConfirmationRes_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 17 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_MeteringConfirmationResType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.MeteringConfirmationRes,
                );
            }
        } else if (*exiDoc).Object_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 19 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ObjectType(stream, &mut (*exiDoc).c2rust_unnamed.Object);
            }
        } else if (*exiDoc).PGPData_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 20 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_PGPDataType(stream, &mut (*exiDoc).c2rust_unnamed.PGPData);
            }
        } else if (*exiDoc).PowerDeliveryReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 21 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_PowerDeliveryReqType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.PowerDeliveryReq,
                );
            }
        } else if (*exiDoc).PowerDeliveryRes_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 22 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_PowerDeliveryResType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.PowerDeliveryRes,
                );
            }
        } else if (*exiDoc).RSAKeyValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 23 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_RSAKeyValueType(stream, &mut (*exiDoc).c2rust_unnamed.RSAKeyValue);
            }
        } else if (*exiDoc).Reference_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 24 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ReferenceType(stream, &mut (*exiDoc).c2rust_unnamed.Reference);
            }
        } else if (*exiDoc).RetrievalMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 25 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_RetrievalMethodType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.RetrievalMethod,
                );
            }
        } else if (*exiDoc).SPKIData_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 26 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_SPKIDataType(stream, &mut (*exiDoc).c2rust_unnamed.SPKIData);
            }
        } else if (*exiDoc).ScheduleExchangeReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 27 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ScheduleExchangeReqType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.ScheduleExchangeReq,
                );
            }
        } else if (*exiDoc).ScheduleExchangeRes_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 28 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ScheduleExchangeResType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.ScheduleExchangeRes,
                );
            }
        } else if (*exiDoc).ServiceDetailReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 29 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ServiceDetailReqType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.ServiceDetailReq,
                );
            }
        } else if (*exiDoc).ServiceDetailRes_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 30 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ServiceDetailResType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.ServiceDetailRes,
                );
            }
        } else if (*exiDoc).ServiceDiscoveryReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 31 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ServiceDiscoveryReqType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.ServiceDiscoveryReq,
                );
            }
        } else if (*exiDoc).ServiceDiscoveryRes_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 32 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ServiceDiscoveryResType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.ServiceDiscoveryRes,
                );
            }
        } else if (*exiDoc).ServiceSelectionReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 33 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ServiceSelectionReqType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.ServiceSelectionReq,
                );
            }
        } else if (*exiDoc).ServiceSelectionRes_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 34 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ServiceSelectionResType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.ServiceSelectionRes,
                );
            }
        } else if (*exiDoc).SessionSetupReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 35 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_SessionSetupReqType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.SessionSetupReq,
                );
            }
        } else if (*exiDoc).SessionSetupRes_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 36 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_SessionSetupResType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.SessionSetupRes,
                );
            }
        } else if (*exiDoc).SessionStopReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 37 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_SessionStopReqType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.SessionStopReq,
                );
            }
        } else if (*exiDoc).SessionStopRes_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 38 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_SessionStopResType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.SessionStopRes,
                );
            }
        } else if (*exiDoc).SignatureMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 39 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_SignatureMethodType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.SignatureMethod,
                );
            }
        } else if (*exiDoc).SignatureProperties_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 40 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_SignaturePropertiesType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.SignatureProperties,
                );
            }
        } else if (*exiDoc).SignatureProperty_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 41 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_SignaturePropertyType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.SignatureProperty,
                );
            }
        } else if (*exiDoc).Signature_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 42 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_SignatureType(stream, &mut (*exiDoc).c2rust_unnamed.Signature);
            }
        } else if (*exiDoc).SignatureValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 43 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_SignatureValueType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.SignatureValue,
                );
            }
        } else if (*exiDoc).SignedInfo_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 44 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_SignedInfoType(stream, &mut (*exiDoc).c2rust_unnamed.SignedInfo);
            }
        } else if (*exiDoc).SignedInstallationData_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 45 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_SignedInstallationDataType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.SignedInstallationData,
                );
            }
        } else if (*exiDoc).SignedMeteringData_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 46 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_SignedMeteringDataType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.SignedMeteringData,
                );
            }
        } else if (*exiDoc).Transform_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 47 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_TransformType(stream, &mut (*exiDoc).c2rust_unnamed.Transform);
            }
        } else if (*exiDoc).Transforms_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 48 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_TransformsType(stream, &mut (*exiDoc).c2rust_unnamed.Transforms);
            }
        } else if (*exiDoc).VehicleCheckInReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 49 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_VehicleCheckInReqType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.VehicleCheckInReq,
                );
            }
        } else if (*exiDoc).VehicleCheckInRes_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 50 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_VehicleCheckInResType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.VehicleCheckInRes,
                );
            }
        } else if (*exiDoc).VehicleCheckOutReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 51 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_VehicleCheckOutReqType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.VehicleCheckOutReq,
                );
            }
        } else if (*exiDoc).VehicleCheckOutRes_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 52 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_VehicleCheckOutResType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.VehicleCheckOutRes,
                );
            }
        } else if (*exiDoc).X509Data_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 53 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_X509DataType(stream, &mut (*exiDoc).c2rust_unnamed.X509Data);
            }
        } else {
            error = -(70 as i32);
        }
    }
    return error;
}

pub unsafe extern "C" fn encode_iso20_exiFragment(
    stream: &mut ExiBitstream,
    mut exiFrag: *mut iso20_exiFragment,
) -> i32 {
    let mut error: i32 = exi_header_write(stream);
    if error == 0 as i32 {
        if (*exiFrag).AbsolutePriceSchedule_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 9 as i32 as usize, 0 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_AbsolutePriceScheduleType(
                    stream,
                    &mut (*exiFrag).c2rust_unnamed.AbsolutePriceSchedule,
                );
            }
        } else if (*exiFrag).CertificateInstallationReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 9 as i32 as usize, 27 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_CertificateInstallationReqType(
                    stream,
                    &mut (*exiFrag).c2rust_unnamed.CertificateInstallationReq,
                );
            }
        } else if (*exiFrag).MeteringConfirmationReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 9 as i32 as usize, 119 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_MeteringConfirmationReqType(
                    stream,
                    &mut (*exiFrag).c2rust_unnamed.MeteringConfirmationReq,
                );
            }
        } else if (*exiFrag).PnC_AReqAuthorizationMode_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 9 as i32 as usize, 151 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_PnC_AReqAuthorizationModeType(
                    stream,
                    &mut (*exiFrag).c2rust_unnamed.PnC_AReqAuthorizationMode,
                );
            }
        } else if (*exiFrag).SignedInfo_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 9 as i32 as usize, 230 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_SignedInfoType(stream, &mut (*exiFrag).c2rust_unnamed.SignedInfo);
            }
        } else if (*exiFrag).SignedInstallationData_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 9 as i32 as usize, 231 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_SignedInstallationDataType(
                    stream,
                    &mut (*exiFrag).c2rust_unnamed.SignedInstallationData,
                );
            }
        } else {
            error = -(70 as i32);
        }
        if error == 0 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 9 as i32 as usize, 282 as i32 as u32);
        }
    }
    return error;
}

pub unsafe extern "C" fn encode_iso20_xmldsigFragment(
    stream: &mut ExiBitstream,
    mut xmldsigFrag: *mut iso20_xmldsigFragment,
) -> i32 {
    let mut error: i32 = exi_header_write(stream);
    if error == 0 as i32 {
        if (*xmldsigFrag).CanonicalizationMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 0 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_CanonicalizationMethodType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.CanonicalizationMethod,
                );
            }
        } else if (*xmldsigFrag).DSAKeyValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 1 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_DSAKeyValueType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.DSAKeyValue,
                );
            }
        } else if (*xmldsigFrag).DigestMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 2 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_DigestMethodType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.DigestMethod,
                );
            }
        } else if (*xmldsigFrag).KeyInfo_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 8 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_KeyInfoType(stream, &mut (*xmldsigFrag).c2rust_unnamed.KeyInfo);
            }
        } else if (*xmldsigFrag).KeyValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 10 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_KeyValueType(stream, &mut (*xmldsigFrag).c2rust_unnamed.KeyValue);
            }
        } else if (*xmldsigFrag).Manifest_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 11 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_ManifestType(stream, &mut (*xmldsigFrag).c2rust_unnamed.Manifest);
            }
        } else if (*xmldsigFrag).Object_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 14 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ObjectType(stream, &mut (*xmldsigFrag).c2rust_unnamed.Object);
            }
        } else if (*xmldsigFrag).PGPData_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 16 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_PGPDataType(stream, &mut (*xmldsigFrag).c2rust_unnamed.PGPData);
            }
        } else if (*xmldsigFrag).RSAKeyValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 21 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_RSAKeyValueType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.RSAKeyValue,
                );
            }
        } else if (*xmldsigFrag).Reference_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 22 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_ReferenceType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.Reference,
                );
            }
        } else if (*xmldsigFrag).RetrievalMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 23 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_RetrievalMethodType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.RetrievalMethod,
                );
            }
        } else if (*xmldsigFrag).SPKIData_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 24 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_SPKIDataType(stream, &mut (*xmldsigFrag).c2rust_unnamed.SPKIData);
            }
        } else if (*xmldsigFrag).Signature_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 27 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_SignatureType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.Signature,
                );
            }
        } else if (*xmldsigFrag).SignatureMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 28 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_SignatureMethodType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.SignatureMethod,
                );
            }
        } else if (*xmldsigFrag).SignatureProperties_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 29 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_SignaturePropertiesType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.SignatureProperties,
                );
            }
        } else if (*xmldsigFrag).SignatureProperty_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 30 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_SignaturePropertyType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.SignatureProperty,
                );
            }
        } else if (*xmldsigFrag).SignatureValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 31 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_SignatureValueType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.SignatureValue,
                );
            }
        } else if (*xmldsigFrag).SignedInfo_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 32 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_SignedInfoType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.SignedInfo,
                );
            }
        } else if (*xmldsigFrag).Transform_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 33 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_TransformType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.Transform,
                );
            }
        } else if (*xmldsigFrag).Transforms_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 34 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_TransformsType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.Transforms,
                );
            }
        } else if (*xmldsigFrag).X509Data_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 37 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_X509DataType(stream, &mut (*xmldsigFrag).c2rust_unnamed.X509Data);
            }
        } else if (*xmldsigFrag).X509IssuerSerial_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 39 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_X509IssuerSerialType(
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
