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
pub type iso20_wpt_WPT_FinePositioningMethodType = u32;
pub const iso20_wpt_WPT_FinePositioningMethodType_Proprietary:
    iso20_wpt_WPT_FinePositioningMethodType = 4;
pub const iso20_wpt_WPT_FinePositioningMethodType_LPE: iso20_wpt_WPT_FinePositioningMethodType = 3;
pub const iso20_wpt_WPT_FinePositioningMethodType_LF_TxPrimaryDevice:
    iso20_wpt_WPT_FinePositioningMethodType = 2;
pub const iso20_wpt_WPT_FinePositioningMethodType_LF_TxEV: iso20_wpt_WPT_FinePositioningMethodType =
    1;
pub const iso20_wpt_WPT_FinePositioningMethodType_Manual: iso20_wpt_WPT_FinePositioningMethodType =
    0;
pub type iso20_wpt_WPT_PairingMethodType = u32;
pub const iso20_wpt_WPT_PairingMethodType_Proprietary: iso20_wpt_WPT_PairingMethodType = 5;
pub const iso20_wpt_WPT_PairingMethodType_Optical: iso20_wpt_WPT_PairingMethodType = 4;
pub const iso20_wpt_WPT_PairingMethodType_LF_TxPrimaryDevice: iso20_wpt_WPT_PairingMethodType = 3;
pub const iso20_wpt_WPT_PairingMethodType_LF_TxEV: iso20_wpt_WPT_PairingMethodType = 2;
pub const iso20_wpt_WPT_PairingMethodType_LPE: iso20_wpt_WPT_PairingMethodType = 1;
pub const iso20_wpt_WPT_PairingMethodType_External_confirmation: iso20_wpt_WPT_PairingMethodType =
    0;
pub type iso20_wpt_WPT_AlignmentCheckMethodType = u32;
pub const iso20_wpt_WPT_AlignmentCheckMethodType_Proprietary:
    iso20_wpt_WPT_AlignmentCheckMethodType = 2;
pub const iso20_wpt_WPT_AlignmentCheckMethodType_LPE: iso20_wpt_WPT_AlignmentCheckMethodType = 1;
pub const iso20_wpt_WPT_AlignmentCheckMethodType_PowerCheck:
    iso20_wpt_WPT_AlignmentCheckMethodType = 0;
pub type iso20_wpt_evseNotificationType = u32;
pub const iso20_wpt_evseNotificationType_MeteringConfirmation: iso20_wpt_evseNotificationType = 5;
pub const iso20_wpt_evseNotificationType_ServiceRenegotiation: iso20_wpt_evseNotificationType = 4;
pub const iso20_wpt_evseNotificationType_ScheduleRenegotiation: iso20_wpt_evseNotificationType = 3;
pub const iso20_wpt_evseNotificationType_Terminate: iso20_wpt_evseNotificationType = 2;
pub const iso20_wpt_evseNotificationType_ExitStandby: iso20_wpt_evseNotificationType = 1;
pub const iso20_wpt_evseNotificationType_Pause: iso20_wpt_evseNotificationType = 0;
pub type iso20_wpt_processingType = u32;
pub const iso20_wpt_processingType_Ongoing_WaitingForCustomerInteraction: iso20_wpt_processingType =
    2;
pub const iso20_wpt_processingType_Ongoing: iso20_wpt_processingType = 1;
pub const iso20_wpt_processingType_Finished: iso20_wpt_processingType = 0;
pub type iso20_wpt_responseCodeType = u32;
pub const iso20_wpt_responseCodeType_FAILED_WrongChargeParameter: iso20_wpt_responseCodeType = 39;
pub const iso20_wpt_responseCodeType_FAILED_UnknownSession: iso20_wpt_responseCodeType = 38;
pub const iso20_wpt_responseCodeType_FAILED_SignatureError: iso20_wpt_responseCodeType = 37;
pub const iso20_wpt_responseCodeType_FAILED_ServiceSelectionInvalid: iso20_wpt_responseCodeType =
    36;
pub const iso20_wpt_responseCodeType_FAILED_ServiceIDInvalid: iso20_wpt_responseCodeType = 35;
pub const iso20_wpt_responseCodeType_FAILED_SequenceError: iso20_wpt_responseCodeType = 34;
pub const iso20_wpt_responseCodeType_FAILED_ScheduleSelectionInvalid: iso20_wpt_responseCodeType =
    33;
pub const iso20_wpt_responseCodeType_FAILED_ScheduleRenegotiation: iso20_wpt_responseCodeType = 32;
pub const iso20_wpt_responseCodeType_FAILED_PowerToleranceNotConfirmed: iso20_wpt_responseCodeType =
    31;
pub const iso20_wpt_responseCodeType_FAILED_PowerDeliveryNotApplied: iso20_wpt_responseCodeType =
    30;
pub const iso20_wpt_responseCodeType_FAILED_PauseNotAllowed: iso20_wpt_responseCodeType = 29;
pub const iso20_wpt_responseCodeType_FAILED_NoServiceRenegotiationSupported:
    iso20_wpt_responseCodeType = 28;
pub const iso20_wpt_responseCodeType_FAILED_NoEnergyTransferServiceSelected:
    iso20_wpt_responseCodeType = 27;
pub const iso20_wpt_responseCodeType_FAILED_MeteringSignatureNotValid: iso20_wpt_responseCodeType =
    26;
pub const iso20_wpt_responseCodeType_FAILED_EVPowerProfileViolation: iso20_wpt_responseCodeType =
    25;
pub const iso20_wpt_responseCodeType_FAILED_EVPowerProfileInvalid: iso20_wpt_responseCodeType = 24;
pub const iso20_wpt_responseCodeType_FAILED_ContactorError: iso20_wpt_responseCodeType = 23;
pub const iso20_wpt_responseCodeType_FAILED_AssociationError: iso20_wpt_responseCodeType = 22;
pub const iso20_wpt_responseCodeType_FAILED: iso20_wpt_responseCodeType = 21;
pub const iso20_wpt_responseCodeType_WARNING_WPT: iso20_wpt_responseCodeType = 20;
pub const iso20_wpt_responseCodeType_WARNING_StandbyNotAllowed: iso20_wpt_responseCodeType = 19;
pub const iso20_wpt_responseCodeType_WARNING_ScheduleRenegotiationFailed:
    iso20_wpt_responseCodeType = 18;
pub const iso20_wpt_responseCodeType_WARNING_PowerToleranceNotConfirmed:
    iso20_wpt_responseCodeType = 17;
pub const iso20_wpt_responseCodeType_WARNING_NoContractMatchingPCIDFound:
    iso20_wpt_responseCodeType = 16;
pub const iso20_wpt_responseCodeType_WARNING_NoCertificateAvailable: iso20_wpt_responseCodeType =
    15;
pub const iso20_wpt_responseCodeType_WARNING_GeneralPnCAuthorizationError:
    iso20_wpt_responseCodeType = 14;
pub const iso20_wpt_responseCodeType_WARNING_EVPowerProfileViolation: iso20_wpt_responseCodeType =
    13;
pub const iso20_wpt_responseCodeType_WARNING_eMSPUnknown: iso20_wpt_responseCodeType = 12;
pub const iso20_wpt_responseCodeType_WARNING_EIMAuthorizationFailure: iso20_wpt_responseCodeType =
    11;
pub const iso20_wpt_responseCodeType_WARNING_ChallengeInvalid: iso20_wpt_responseCodeType = 10;
pub const iso20_wpt_responseCodeType_WARNING_CertificateValidationError:
    iso20_wpt_responseCodeType = 9;
pub const iso20_wpt_responseCodeType_WARNING_CertificateRevoked: iso20_wpt_responseCodeType = 8;
pub const iso20_wpt_responseCodeType_WARNING_CertificateNotYetValid: iso20_wpt_responseCodeType = 7;
pub const iso20_wpt_responseCodeType_WARNING_CertificateExpired: iso20_wpt_responseCodeType = 6;
pub const iso20_wpt_responseCodeType_WARNING_AuthorizationSelectionInvalid:
    iso20_wpt_responseCodeType = 5;
pub const iso20_wpt_responseCodeType_OK_PowerToleranceConfirmed: iso20_wpt_responseCodeType = 4;
pub const iso20_wpt_responseCodeType_OK_OldSessionJoined: iso20_wpt_responseCodeType = 3;
pub const iso20_wpt_responseCodeType_OK_NewSessionEstablished: iso20_wpt_responseCodeType = 2;
pub const iso20_wpt_responseCodeType_OK_CertificateExpiresSoon: iso20_wpt_responseCodeType = 1;
pub const iso20_wpt_responseCodeType_OK: iso20_wpt_responseCodeType = 0;
pub type iso20_wpt_WPT_EVResultType = u32;
pub const iso20_wpt_WPT_EVResultType_EVResultFailed: iso20_wpt_WPT_EVResultType = 2;
pub const iso20_wpt_WPT_EVResultType_EVResultSuccess: iso20_wpt_WPT_EVResultType = 1;
pub const iso20_wpt_WPT_EVResultType_EVResultUnknown: iso20_wpt_WPT_EVResultType = 0;
pub type iso20_wpt_WPT_PowerClassType = u32;
pub const iso20_wpt_WPT_PowerClassType_MF_WPT4: iso20_wpt_WPT_PowerClassType = 3;
pub const iso20_wpt_WPT_PowerClassType_MF_WPT3: iso20_wpt_WPT_PowerClassType = 2;
pub const iso20_wpt_WPT_PowerClassType_MF_WPT2: iso20_wpt_WPT_PowerClassType = 1;
pub const iso20_wpt_WPT_PowerClassType_MF_WPT1: iso20_wpt_WPT_PowerClassType = 0;
pub type iso20_wpt_WPT_EVPCChargeDiagnosticsType = u32;
pub const iso20_wpt_WPT_EVPCChargeDiagnosticsType_EVPCAnomalyDetected:
    iso20_wpt_WPT_EVPCChargeDiagnosticsType = 3;
pub const iso20_wpt_WPT_EVPCChargeDiagnosticsType_EVPCPowerTransferAnomalyDetected:
    iso20_wpt_WPT_EVPCChargeDiagnosticsType = 2;
pub const iso20_wpt_WPT_EVPCChargeDiagnosticsType_EVPCTempOverheatDetected:
    iso20_wpt_WPT_EVPCChargeDiagnosticsType = 1;
pub const iso20_wpt_WPT_EVPCChargeDiagnosticsType_EVPCNoIssue:
    iso20_wpt_WPT_EVPCChargeDiagnosticsType = 0;
pub type iso20_wpt_WPT_SPCChargeDiagnosticsType = u32;
pub const iso20_wpt_WPT_SPCChargeDiagnosticsType_SPCAnomalyDetected:
    iso20_wpt_WPT_SPCChargeDiagnosticsType = 5;
pub const iso20_wpt_WPT_SPCChargeDiagnosticsType_SPCPowerTransferAnomalyDetected:
    iso20_wpt_WPT_SPCChargeDiagnosticsType = 4;
pub const iso20_wpt_WPT_SPCChargeDiagnosticsType_SPCTempOverheatDetected:
    iso20_wpt_WPT_SPCChargeDiagnosticsType = 3;
pub const iso20_wpt_WPT_SPCChargeDiagnosticsType_SPCLOPDetected:
    iso20_wpt_WPT_SPCChargeDiagnosticsType = 2;
pub const iso20_wpt_WPT_SPCChargeDiagnosticsType_SPCFODDetected:
    iso20_wpt_WPT_SPCChargeDiagnosticsType = 1;
pub const iso20_wpt_WPT_SPCChargeDiagnosticsType_SPCNoIssue:
    iso20_wpt_WPT_SPCChargeDiagnosticsType = 0;
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_TransformType {
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

pub struct iso20_wpt_TransformsType {
    pub Transform: iso20_wpt_TransformType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_DSAKeyValueType {
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

pub struct iso20_wpt_X509IssuerSerialType {
    pub X509IssuerName: C2RustUnnamed_9,
    pub X509SerialNumber: ExiSigned,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_9 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_DigestMethodType {
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

pub struct iso20_wpt_RSAKeyValueType {
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

pub struct iso20_wpt_CanonicalizationMethodType {
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

pub struct iso20_wpt_WPT_TxRxPulseOrderType {
    pub IndexNumber: u16,
    pub TxRxIdentifier: u32,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_SignatureMethodType {
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

pub struct iso20_wpt_KeyValueType {
    pub DSAKeyValue: iso20_wpt_DSAKeyValueType,
    #[bitfield(name = "DSAKeyValue_isUsed", ty = "u32", bits = "0..=0")]
    pub DSAKeyValue_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub RSAKeyValue: iso20_wpt_RSAKeyValueType,
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
#[derive(Copy, Clone)]

pub struct iso20_wpt_WPT_CoordinateXYZType {
    pub Coord_X: i16,
    pub Coord_Y: i16,
    pub Coord_Z: i16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_ReferenceType {
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
    pub Transforms: iso20_wpt_TransformsType,
    #[bitfield(name = "Transforms_isUsed", ty = "u32", bits = "0..=0")]
    pub Transforms_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 3],
    pub DigestMethod: iso20_wpt_DigestMethodType,
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

pub struct iso20_wpt_RetrievalMethodType {
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
    pub Transforms: iso20_wpt_TransformsType,
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

pub struct iso20_wpt_X509DataType {
    pub X509IssuerSerial: iso20_wpt_X509IssuerSerialType,
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

pub struct iso20_wpt_PGPDataType {
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

pub struct iso20_wpt_SPKIDataType {
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

pub struct iso20_wpt_SignedInfoType {
    pub Id: C2RustUnnamed_41,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub CanonicalizationMethod: iso20_wpt_CanonicalizationMethodType,
    pub SignatureMethod: iso20_wpt_SignatureMethodType,
    pub Reference: C2RustUnnamed_40,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_40 {
    pub array: [iso20_wpt_ReferenceType; 4],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_41 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_SignatureValueType {
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
#[derive(Copy, Clone)]

pub struct iso20_wpt_RationalNumberType {
    pub Exponent: int8_t,
    pub Value: i16,
}
#[derive(Copy, Clone)]

pub struct iso20_wpt_WPT_LF_RxRSSIType {
    pub TxIdentifier: u32,
    pub RSSI: iso20_wpt_RationalNumberType,
}
#[derive(Copy, Clone)]

pub struct iso20_wpt_WPT_LF_RxRSSIListType {
    pub RSSIDataList: iso20_wpt_WPT_LF_RxRSSIType,
}
#[derive(Copy, Clone)]

pub struct iso20_wpt_WPT_LF_TxDataType {
    pub TxIdentifier: u32,
    pub EIRP: iso20_wpt_RationalNumberType,
}
#[derive(Copy, Clone)]

pub struct iso20_wpt_WPT_LF_RxDataType {
    pub RxIdentifier: u32,
    pub RSSIData: iso20_wpt_WPT_LF_RxRSSIListType,
}
#[derive(Copy, Clone)]

pub struct iso20_wpt_WPT_LF_TxDataListType {
    pub WPT_LF_TxDataList: iso20_wpt_WPT_LF_TxDataType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_KeyInfoType {
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
    pub KeyValue: iso20_wpt_KeyValueType,
    #[bitfield(name = "KeyValue_isUsed", ty = "u32", bits = "0..=0")]
    pub KeyValue_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 3],
    pub RetrievalMethod: iso20_wpt_RetrievalMethodType,
    #[bitfield(name = "RetrievalMethod_isUsed", ty = "u32", bits = "0..=0")]
    pub RetrievalMethod_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 3],
    pub X509Data: iso20_wpt_X509DataType,
    #[bitfield(name = "X509Data_isUsed", ty = "u32", bits = "0..=0")]
    pub X509Data_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 3],
    pub PGPData: iso20_wpt_PGPDataType,
    #[bitfield(name = "PGPData_isUsed", ty = "u32", bits = "0..=0")]
    pub PGPData_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 3],
    pub SPKIData: iso20_wpt_SPKIDataType,
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
#[derive(Copy, Clone)]

pub struct iso20_wpt_WPT_TxRxSpecDataType {
    pub TxRxIdentifier: u32,
    pub TxRxPosition: iso20_wpt_WPT_CoordinateXYZType,
    pub TxRxOrientation: iso20_wpt_WPT_CoordinateXYZType,
}
#[derive(Copy, Clone)]

pub struct iso20_wpt_WPT_LF_RxDataListType {
    pub WPT_LF_RxDataList: iso20_wpt_WPT_LF_RxDataType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_ObjectType {
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

pub struct iso20_wpt_WPT_TxRxPackageSpecDataType {
    pub PulseSequenceOrder: C2RustUnnamed_52,
    pub PulseSeparationTime: u16,
    pub PulseDuration: u16,
    pub PackageSeparationTime: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_52 {
    pub array: [iso20_wpt_WPT_TxRxPulseOrderType; 255],
    pub arrayLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_WPT_LF_TransmitterDataType {
    pub NumberOfTransmitters: u8,
    pub SignalFrequency: iso20_wpt_RationalNumberType,
    pub TxSpecData: C2RustUnnamed_53,
    pub TxPackageSpecData: iso20_wpt_WPT_TxRxPackageSpecDataType,
    #[bitfield(name = "TxPackageSpecData_isUsed", ty = "u32", bits = "0..=0")]
    pub TxPackageSpecData_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_53 {
    pub array: [iso20_wpt_WPT_TxRxSpecDataType; 255],
    pub arrayLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_AlternativeSECCType {
    pub SSID: C2RustUnnamed_56,
    #[bitfield(name = "SSID_isUsed", ty = "u32", bits = "0..=0")]
    pub SSID_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub BSSID: C2RustUnnamed_55,
    #[bitfield(name = "BSSID_isUsed", ty = "u32", bits = "0..=0")]
    pub BSSID_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub IPAddress: C2RustUnnamed_54,
    #[bitfield(name = "IPAddress_isUsed", ty = "u32", bits = "0..=0")]
    pub IPAddress_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub Port: u16,
    #[bitfield(name = "Port_isUsed", ty = "u32", bits = "0..=0")]
    pub Port_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 3],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_54 {
    pub characters: [i8; 40],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_55 {
    pub characters: [i8; 13],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_56 {
    pub characters: [i8; 256],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_wpt_WPT_LF_ReceiverDataType {
    pub NumberOfReceivers: u8,
    pub RxSpecData: C2RustUnnamed_57,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_57 {
    pub array: [iso20_wpt_WPT_TxRxSpecDataType; 255],
    pub arrayLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_WPT_LF_DataPackageType {
    pub PackageIndex: u8,
    pub LF_TxData: iso20_wpt_WPT_LF_TxDataListType,
    #[bitfield(name = "LF_TxData_isUsed", ty = "u32", bits = "0..=0")]
    pub LF_TxData_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub LF_RxData: iso20_wpt_WPT_LF_RxDataListType,
    #[bitfield(name = "LF_RxData_isUsed", ty = "u32", bits = "0..=0")]
    pub LF_RxData_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
}
#[derive(Copy, Clone)]

pub struct iso20_wpt_DetailedCostType {
    pub Amount: iso20_wpt_RationalNumberType,
    pub CostPerUnit: iso20_wpt_RationalNumberType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_SignatureType {
    pub Id: C2RustUnnamed_58,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub SignedInfo: iso20_wpt_SignedInfoType,
    pub SignatureValue: iso20_wpt_SignatureValueType,
    pub KeyInfo: iso20_wpt_KeyInfoType,
    #[bitfield(name = "KeyInfo_isUsed", ty = "u32", bits = "0..=0")]
    pub KeyInfo_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
    pub Object: iso20_wpt_ObjectType,
    #[bitfield(name = "Object_isUsed", ty = "u32", bits = "0..=0")]
    pub Object_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 7],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_58 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_wpt_DetailedTaxType {
    pub TaxRuleID: u32,
    pub Amount: iso20_wpt_RationalNumberType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_MessageHeaderType {
    pub SessionID: C2RustUnnamed_59,
    pub TimeStamp: u64,
    pub Signature: iso20_wpt_SignatureType,
    #[bitfield(name = "Signature_isUsed", ty = "u32", bits = "0..=0")]
    pub Signature_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_59 {
    pub bytes: [u8; 8],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_SignaturePropertyType {
    pub Id: C2RustUnnamed_62,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub Target: C2RustUnnamed_61,
    pub ANY: C2RustUnnamed_60,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_60 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_61 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_62 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_DisplayParametersType {
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
    pub BatteryEnergyCapacity: iso20_wpt_RationalNumberType,
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
#[derive(Copy, Clone)]

pub struct iso20_wpt_WPT_FinePositioningMethodListType {
    pub WPT_FinePositioningMethod: C2RustUnnamed_63,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_63 {
    pub array: [iso20_wpt_WPT_FinePositioningMethodType; 8],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_wpt_EVSEStatusType {
    pub NotificationMaxDelay: u16,
    pub EVSENotification: iso20_wpt_evseNotificationType,
}
#[derive(Copy, Clone)]

pub struct iso20_wpt_WPT_PairingMethodListType {
    pub WPT_PairingMethod: C2RustUnnamed_64,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_64 {
    pub array: [iso20_wpt_WPT_PairingMethodType; 8],
    pub arrayLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_MeterInfoType {
    pub MeterID: C2RustUnnamed_66,
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
    pub MeterSignature: C2RustUnnamed_65,
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

pub struct C2RustUnnamed_65 {
    pub bytes: [u8; 64],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_66 {
    pub characters: [i8; 33],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_wpt_WPT_AlignmentCheckMethodListType {
    pub WPT_AlignmentCheckMethod: C2RustUnnamed_67,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_67 {
    pub array: [iso20_wpt_WPT_AlignmentCheckMethodType; 8],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_wpt_WPT_LF_DataPackageListType {
    pub NumPackages: u8,
    pub WPT_LF_DataPackage: iso20_wpt_WPT_LF_DataPackageType,
}
#[derive(Copy, Clone)]

pub struct iso20_wpt_AlternativeSECCListType {
    pub AlternativeSECC: C2RustUnnamed_68,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_68 {
    pub array: [iso20_wpt_AlternativeSECCType; 8],
    pub arrayLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_ReceiptType {
    pub TimeAnchor: u64,
    pub EnergyCosts: iso20_wpt_DetailedCostType,
    #[bitfield(name = "EnergyCosts_isUsed", ty = "u32", bits = "0..=0")]
    pub EnergyCosts_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub OccupancyCosts: iso20_wpt_DetailedCostType,
    #[bitfield(name = "OccupancyCosts_isUsed", ty = "u32", bits = "0..=0")]
    pub OccupancyCosts_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub AdditionalServicesCosts: iso20_wpt_DetailedCostType,
    #[bitfield(name = "AdditionalServicesCosts_isUsed", ty = "u32", bits = "0..=0")]
    pub AdditionalServicesCosts_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub OverstayCosts: iso20_wpt_DetailedCostType,
    #[bitfield(name = "OverstayCosts_isUsed", ty = "u32", bits = "0..=0")]
    pub OverstayCosts_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub TaxCosts: C2RustUnnamed_69,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_69 {
    pub array: [iso20_wpt_DetailedTaxType; 10],
    pub arrayLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_WPT_LF_SystemSetupDataType {
    pub LF_TransmitterSetupData: iso20_wpt_WPT_LF_TransmitterDataType,
    #[bitfield(name = "LF_TransmitterSetupData_isUsed", ty = "u32", bits = "0..=0")]
    pub LF_TransmitterSetupData_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub LF_ReceiverSetupData: iso20_wpt_WPT_LF_ReceiverDataType,
    #[bitfield(name = "LF_ReceiverSetupData_isUsed", ty = "u32", bits = "0..=0")]
    pub LF_ReceiverSetupData_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
}
#[derive(Copy, Clone)]

pub struct iso20_wpt_WPT_EVPCPowerControlParameterType {
    pub EVPCCoilCurrentRequest: iso20_wpt_RationalNumberType,
    pub EVPCCoilCurrentInformation: iso20_wpt_RationalNumberType,
    pub EVPCCurrentOutputInformation: iso20_wpt_RationalNumberType,
    pub EVPCVoltageOutputInformation: iso20_wpt_RationalNumberType,
}
#[derive(Copy, Clone)]

pub struct iso20_wpt_WPT_SPCPowerControlParameterType {
    pub SPCPrimaryDeviceCoilCurrentInformation: iso20_wpt_RationalNumberType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_WPT_FinePositioningSetupReqType {
    pub Header: iso20_wpt_MessageHeaderType,
    pub EVProcessing: iso20_wpt_processingType,
    pub EVDeviceFinePositioningMethodList: iso20_wpt_WPT_FinePositioningMethodListType,
    pub EVDevicePairingMethodList: iso20_wpt_WPT_PairingMethodListType,
    pub EVDeviceAlignmentCheckMethodList: iso20_wpt_WPT_AlignmentCheckMethodListType,
    pub NaturalOffset: u16,
    pub VendorSpecificDataContainer: C2RustUnnamed_70,
    #[bitfield(
        name = "VendorSpecificDataContainer_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub VendorSpecificDataContainer_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub LF_SystemSetupData: iso20_wpt_WPT_LF_SystemSetupDataType,
    #[bitfield(name = "LF_SystemSetupData_isUsed", ty = "u32", bits = "0..=0")]
    pub LF_SystemSetupData_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_70 {
    pub array: [C2RustUnnamed_71; 16],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_71 {
    pub bytes: [u8; 256],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_WPT_FinePositioningSetupResType {
    pub Header: iso20_wpt_MessageHeaderType,
    pub ResponseCode: iso20_wpt_responseCodeType,
    pub PrimaryDeviceFinePositioningMethodList: iso20_wpt_WPT_FinePositioningMethodListType,
    pub PrimaryDevicePairingMethodList: iso20_wpt_WPT_PairingMethodListType,
    pub PrimaryDeviceAlignmentCheckMethodList: iso20_wpt_WPT_AlignmentCheckMethodListType,
    pub NaturalOffset: u16,
    pub VendorSpecificDataContainer: C2RustUnnamed_72,
    #[bitfield(
        name = "VendorSpecificDataContainer_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub VendorSpecificDataContainer_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub LF_SystemSetupData: iso20_wpt_WPT_LF_SystemSetupDataType,
    #[bitfield(name = "LF_SystemSetupData_isUsed", ty = "u32", bits = "0..=0")]
    pub LF_SystemSetupData_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_72 {
    pub array: [C2RustUnnamed_73; 16],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_73 {
    pub bytes: [u8; 256],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_WPT_FinePositioningReqType {
    pub Header: iso20_wpt_MessageHeaderType,
    pub EVProcessing: iso20_wpt_processingType,
    pub EVResultCode: iso20_wpt_WPT_EVResultType,
    pub VendorSpecificDataContainer: C2RustUnnamed_74,
    #[bitfield(
        name = "VendorSpecificDataContainer_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub VendorSpecificDataContainer_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub WPT_LF_DataPackageList: iso20_wpt_WPT_LF_DataPackageListType,
    #[bitfield(name = "WPT_LF_DataPackageList_isUsed", ty = "u32", bits = "0..=0")]
    pub WPT_LF_DataPackageList_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 7],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_74 {
    pub array: [C2RustUnnamed_75; 16],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_75 {
    pub bytes: [u8; 256],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_WPT_FinePositioningResType {
    pub Header: iso20_wpt_MessageHeaderType,
    pub ResponseCode: iso20_wpt_responseCodeType,
    pub EVSEProcessing: iso20_wpt_processingType,
    pub VendorSpecificDataContainer: C2RustUnnamed_76,
    #[bitfield(
        name = "VendorSpecificDataContainer_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub VendorSpecificDataContainer_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub WPT_LF_DataPackageList: iso20_wpt_WPT_LF_DataPackageListType,
    #[bitfield(name = "WPT_LF_DataPackageList_isUsed", ty = "u32", bits = "0..=0")]
    pub WPT_LF_DataPackageList_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 7],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_76 {
    pub array: [C2RustUnnamed_77; 16],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_77 {
    pub bytes: [u8; 256],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_WPT_PairingReqType {
    pub Header: iso20_wpt_MessageHeaderType,
    pub EVProcessing: iso20_wpt_processingType,
    pub ObservedIDCode: u32,
    #[bitfield(name = "ObservedIDCode_isUsed", ty = "u32", bits = "0..=0")]
    pub ObservedIDCode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub EVResultCode: iso20_wpt_WPT_EVResultType,
    pub VendorSpecificDataContainer: C2RustUnnamed_78,
    #[bitfield(
        name = "VendorSpecificDataContainer_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub VendorSpecificDataContainer_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 5],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_78 {
    pub array: [C2RustUnnamed_79; 16],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_79 {
    pub bytes: [u8; 256],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_WPT_PairingResType {
    pub Header: iso20_wpt_MessageHeaderType,
    pub ResponseCode: iso20_wpt_responseCodeType,
    pub EVSEProcessing: iso20_wpt_processingType,
    pub ObservedIDCode: u32,
    #[bitfield(name = "ObservedIDCode_isUsed", ty = "u32", bits = "0..=0")]
    pub ObservedIDCode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub AlternativeSECCList: iso20_wpt_AlternativeSECCListType,
    #[bitfield(name = "AlternativeSECCList_isUsed", ty = "u32", bits = "0..=0")]
    pub AlternativeSECCList_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub VendorSpecificDataContainer: C2RustUnnamed_80,
    #[bitfield(
        name = "VendorSpecificDataContainer_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub VendorSpecificDataContainer_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 7],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_80 {
    pub array: [C2RustUnnamed_81; 16],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_81 {
    pub bytes: [u8; 256],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_WPT_ChargeParameterDiscoveryReqType {
    pub Header: iso20_wpt_MessageHeaderType,
    pub EVPCMaxReceivablePower: iso20_wpt_RationalNumberType,
    pub SDMaxGroundClearence: u16,
    pub SDMinGroundClearence: u16,
    pub EVPCNaturalFrequency: iso20_wpt_RationalNumberType,
    pub EVPCDeviceLocalControl: i32,
    pub VendorSpecificDataContainer: C2RustUnnamed_82,
    #[bitfield(
        name = "VendorSpecificDataContainer_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub VendorSpecificDataContainer_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 5],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_82 {
    pub array: [C2RustUnnamed_83; 16],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_83 {
    pub bytes: [u8; 256],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_WPT_ChargeParameterDiscoveryResType {
    pub Header: iso20_wpt_MessageHeaderType,
    pub ResponseCode: iso20_wpt_responseCodeType,
    pub PDInputPowerClass: iso20_wpt_WPT_PowerClassType,
    pub SDMinOutputPower: iso20_wpt_RationalNumberType,
    pub SDMaxOutputPower: iso20_wpt_RationalNumberType,
    pub SDMaxGroundClearanceSupport: u16,
    pub SDMinGroundClearanceSupport: u16,
    pub PDMinCoilCurrent: iso20_wpt_RationalNumberType,
    pub PDMaxCoilCurrent: iso20_wpt_RationalNumberType,
    pub SDManufacturerSpecificDataContainer: C2RustUnnamed_84,
    #[bitfield(
        name = "SDManufacturerSpecificDataContainer_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub SDManufacturerSpecificDataContainer_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_84 {
    pub array: [C2RustUnnamed_85; 16],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_85 {
    pub bytes: [u8; 256],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_WPT_AlignmentCheckReqType {
    pub Header: iso20_wpt_MessageHeaderType,
    pub EVProcessing: iso20_wpt_processingType,
    pub TargetCoilCurrent: iso20_wpt_RationalNumberType,
    #[bitfield(name = "TargetCoilCurrent_isUsed", ty = "u32", bits = "0..=0")]
    pub TargetCoilCurrent_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub EVResultCode: iso20_wpt_WPT_EVResultType,
    pub VendorSpecificDataContainer: C2RustUnnamed_86,
    #[bitfield(
        name = "VendorSpecificDataContainer_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub VendorSpecificDataContainer_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 5],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_86 {
    pub array: [C2RustUnnamed_87; 16],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_87 {
    pub bytes: [u8; 256],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_WPT_AlignmentCheckResType {
    pub Header: iso20_wpt_MessageHeaderType,
    pub ResponseCode: iso20_wpt_responseCodeType,
    pub EVSEProcessing: iso20_wpt_processingType,
    pub PowerTransmitted: iso20_wpt_RationalNumberType,
    #[bitfield(name = "PowerTransmitted_isUsed", ty = "u32", bits = "0..=0")]
    pub PowerTransmitted_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub SupplyDeviceCurrent: iso20_wpt_RationalNumberType,
    #[bitfield(name = "SupplyDeviceCurrent_isUsed", ty = "u32", bits = "0..=0")]
    pub SupplyDeviceCurrent_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub VendorSpecificDataContainer: C2RustUnnamed_88,
    #[bitfield(
        name = "VendorSpecificDataContainer_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub VendorSpecificDataContainer_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_88 {
    pub array: [C2RustUnnamed_89; 16],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_89 {
    pub bytes: [u8; 256],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_WPT_ChargeLoopReqType {
    pub Header: iso20_wpt_MessageHeaderType,
    pub DisplayParameters: iso20_wpt_DisplayParametersType,
    #[bitfield(name = "DisplayParameters_isUsed", ty = "u32", bits = "0..=0")]
    pub DisplayParameters_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub MeterInfoRequested: i32,
    pub EVPCPowerRequest: iso20_wpt_RationalNumberType,
    pub EVPCPowerOutput: iso20_wpt_RationalNumberType,
    pub EVPCChargeDiagnostics: iso20_wpt_WPT_EVPCChargeDiagnosticsType,
    pub EVPCOperatingFrequency: iso20_wpt_RationalNumberType,
    #[bitfield(name = "EVPCOperatingFrequency_isUsed", ty = "u32", bits = "0..=0")]
    pub EVPCOperatingFrequency_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub EVPCPowerControlParameter: iso20_wpt_WPT_EVPCPowerControlParameterType,
    #[bitfield(name = "EVPCPowerControlParameter_isUsed", ty = "u32", bits = "0..=0")]
    pub EVPCPowerControlParameter_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub ManufacturerSpecificDataContainer: C2RustUnnamed_90,
    #[bitfield(
        name = "ManufacturerSpecificDataContainer_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub ManufacturerSpecificDataContainer_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 5],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_90 {
    pub array: [C2RustUnnamed_91; 16],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_91 {
    pub bytes: [u8; 256],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_WPT_ChargeLoopResType {
    pub Header: iso20_wpt_MessageHeaderType,
    pub ResponseCode: iso20_wpt_responseCodeType,
    pub EVSEStatus: iso20_wpt_EVSEStatusType,
    #[bitfield(name = "EVSEStatus_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEStatus_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub MeterInfo: iso20_wpt_MeterInfoType,
    #[bitfield(name = "MeterInfo_isUsed", ty = "u32", bits = "0..=0")]
    pub MeterInfo_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 7],
    pub Receipt: iso20_wpt_ReceiptType,
    #[bitfield(name = "Receipt_isUsed", ty = "u32", bits = "0..=0")]
    pub Receipt_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub EVPCPowerRequest: iso20_wpt_RationalNumberType,
    pub SDPowerInput: iso20_wpt_RationalNumberType,
    #[bitfield(name = "SDPowerInput_isUsed", ty = "u32", bits = "0..=0")]
    pub SDPowerInput_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub SPCMaxOutputPowerLimit: iso20_wpt_RationalNumberType,
    pub SPCMinOutputPowerLimit: iso20_wpt_RationalNumberType,
    pub SPCChargeDiagnostics: iso20_wpt_WPT_SPCChargeDiagnosticsType,
    pub SPCOperatingFrequency: iso20_wpt_RationalNumberType,
    #[bitfield(name = "SPCOperatingFrequency_isUsed", ty = "u32", bits = "0..=0")]
    pub SPCOperatingFrequency_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 1],
    pub SPCPowerControlParameter: iso20_wpt_WPT_SPCPowerControlParameterType,
    #[bitfield(name = "SPCPowerControlParameter_isUsed", ty = "u32", bits = "0..=0")]
    pub SPCPowerControlParameter_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 1],
    pub ManufacturerSpecificDataContainer: C2RustUnnamed_92,
    #[bitfield(
        name = "ManufacturerSpecificDataContainer_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub ManufacturerSpecificDataContainer_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_5: [u8; 1],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_92 {
    pub array: [C2RustUnnamed_93; 16],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_93 {
    pub bytes: [u8; 256],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct iso20_wpt_CLReqControlModeType {
    _unused: i32,
}
#[derive(Copy, Clone)]

pub struct iso20_wpt_CLResControlModeType {
    _unused: i32,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_ManifestType {
    pub Id: C2RustUnnamed_95,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub Reference: C2RustUnnamed_94,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_94 {
    pub array: [iso20_wpt_ReferenceType; 4],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_95 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_SignaturePropertiesType {
    pub Id: C2RustUnnamed_96,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub SignatureProperty: iso20_wpt_SignaturePropertyType,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_96 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_wpt_exiDocument {
    pub c2rust_unnamed: C2RustUnnamed_97,
    #[bitfield(
        name = "WPT_FinePositioningSetupReq_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    #[bitfield(
        name = "WPT_FinePositioningSetupRes_isUsed",
        ty = "u32",
        bits = "1..=1"
    )]
    #[bitfield(name = "WPT_FinePositioningReq_isUsed", ty = "u32", bits = "2..=2")]
    #[bitfield(name = "WPT_FinePositioningRes_isUsed", ty = "u32", bits = "3..=3")]
    #[bitfield(name = "WPT_PairingReq_isUsed", ty = "u32", bits = "4..=4")]
    #[bitfield(name = "WPT_PairingRes_isUsed", ty = "u32", bits = "5..=5")]
    #[bitfield(
        name = "WPT_ChargeParameterDiscoveryReq_isUsed",
        ty = "u32",
        bits = "6..=6"
    )]
    #[bitfield(
        name = "WPT_ChargeParameterDiscoveryRes_isUsed",
        ty = "u32",
        bits = "7..=7"
    )]
    #[bitfield(name = "WPT_AlignmentCheckReq_isUsed", ty = "u32", bits = "8..=8")]
    #[bitfield(name = "WPT_AlignmentCheckRes_isUsed", ty = "u32", bits = "9..=9")]
    #[bitfield(name = "WPT_ChargeLoopReq_isUsed", ty = "u32", bits = "10..=10")]
    #[bitfield(name = "WPT_ChargeLoopRes_isUsed", ty = "u32", bits = "11..=11")]
    #[bitfield(name = "CLReqControlMode_isUsed", ty = "u32", bits = "12..=12")]
    #[bitfield(name = "CLResControlMode_isUsed", ty = "u32", bits = "13..=13")]
    #[bitfield(name = "Signature_isUsed", ty = "u32", bits = "14..=14")]
    #[bitfield(name = "SignatureValue_isUsed", ty = "u32", bits = "15..=15")]
    #[bitfield(name = "SignedInfo_isUsed", ty = "u32", bits = "16..=16")]
    #[bitfield(name = "CanonicalizationMethod_isUsed", ty = "u32", bits = "17..=17")]
    #[bitfield(name = "SignatureMethod_isUsed", ty = "u32", bits = "18..=18")]
    #[bitfield(name = "Reference_isUsed", ty = "u32", bits = "19..=19")]
    #[bitfield(name = "Transforms_isUsed", ty = "u32", bits = "20..=20")]
    #[bitfield(name = "Transform_isUsed", ty = "u32", bits = "21..=21")]
    #[bitfield(name = "DigestMethod_isUsed", ty = "u32", bits = "22..=22")]
    #[bitfield(name = "KeyInfo_isUsed", ty = "u32", bits = "23..=23")]
    #[bitfield(name = "KeyValue_isUsed", ty = "u32", bits = "24..=24")]
    #[bitfield(name = "RetrievalMethod_isUsed", ty = "u32", bits = "25..=25")]
    #[bitfield(name = "X509Data_isUsed", ty = "u32", bits = "26..=26")]
    #[bitfield(name = "PGPData_isUsed", ty = "u32", bits = "27..=27")]
    #[bitfield(name = "SPKIData_isUsed", ty = "u32", bits = "28..=28")]
    #[bitfield(name = "Object_isUsed", ty = "u32", bits = "29..=29")]
    #[bitfield(name = "Manifest_isUsed", ty = "u32", bits = "30..=30")]
    #[bitfield(name = "SignatureProperties_isUsed", ty = "u32", bits = "31..=31")]
    #[bitfield(name = "SignatureProperty_isUsed", ty = "u32", bits = "32..=32")]
    #[bitfield(name = "DSAKeyValue_isUsed", ty = "u32", bits = "33..=33")]
    #[bitfield(name = "RSAKeyValue_isUsed", ty = "u32", bits = "34..=34")]
    pub WPT_FinePositioningSetupReq_isUsed_WPT_FinePositioningSetupRes_isUsed_WPT_FinePositioningReq_isUsed_WPT_FinePositioningRes_isUsed_WPT_PairingReq_isUsed_WPT_PairingRes_isUsed_WPT_ChargeParameterDiscoveryReq_isUsed_WPT_ChargeParameterDiscoveryRes_isUsed_WPT_AlignmentCheckReq_isUsed_WPT_AlignmentCheckRes_isUsed_WPT_ChargeLoopReq_isUsed_WPT_ChargeLoopRes_isUsed_CLReqControlMode_isUsed_CLResControlMode_isUsed_Signature_isUsed_SignatureValue_isUsed_SignedInfo_isUsed_CanonicalizationMethod_isUsed_SignatureMethod_isUsed_Reference_isUsed_Transforms_isUsed_Transform_isUsed_DigestMethod_isUsed_KeyInfo_isUsed_KeyValue_isUsed_RetrievalMethod_isUsed_X509Data_isUsed_PGPData_isUsed_SPKIData_isUsed_Object_isUsed_Manifest_isUsed_SignatureProperties_isUsed_SignatureProperty_isUsed_DSAKeyValue_isUsed_RSAKeyValue_isUsed:
        [u8; 5],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]

pub union C2RustUnnamed_97 {
    pub WPT_FinePositioningSetupReq: iso20_wpt_WPT_FinePositioningSetupReqType,
    pub WPT_FinePositioningSetupRes: iso20_wpt_WPT_FinePositioningSetupResType,
    pub WPT_FinePositioningReq: iso20_wpt_WPT_FinePositioningReqType,
    pub WPT_FinePositioningRes: iso20_wpt_WPT_FinePositioningResType,
    pub WPT_PairingReq: iso20_wpt_WPT_PairingReqType,
    pub WPT_PairingRes: iso20_wpt_WPT_PairingResType,
    pub WPT_ChargeParameterDiscoveryReq: iso20_wpt_WPT_ChargeParameterDiscoveryReqType,
    pub WPT_ChargeParameterDiscoveryRes: iso20_wpt_WPT_ChargeParameterDiscoveryResType,
    pub WPT_AlignmentCheckReq: iso20_wpt_WPT_AlignmentCheckReqType,
    pub WPT_AlignmentCheckRes: iso20_wpt_WPT_AlignmentCheckResType,
    pub WPT_ChargeLoopReq: iso20_wpt_WPT_ChargeLoopReqType,
    pub WPT_ChargeLoopRes: iso20_wpt_WPT_ChargeLoopResType,
    pub CLReqControlMode: iso20_wpt_CLReqControlModeType,
    pub CLResControlMode: iso20_wpt_CLResControlModeType,
    pub Signature: iso20_wpt_SignatureType,
    pub SignatureValue: iso20_wpt_SignatureValueType,
    pub SignedInfo: iso20_wpt_SignedInfoType,
    pub CanonicalizationMethod: iso20_wpt_CanonicalizationMethodType,
    pub SignatureMethod: iso20_wpt_SignatureMethodType,
    pub Reference: iso20_wpt_ReferenceType,
    pub Transforms: iso20_wpt_TransformsType,
    pub Transform: iso20_wpt_TransformType,
    pub DigestMethod: iso20_wpt_DigestMethodType,
    pub KeyInfo: iso20_wpt_KeyInfoType,
    pub KeyValue: iso20_wpt_KeyValueType,
    pub RetrievalMethod: iso20_wpt_RetrievalMethodType,
    pub X509Data: iso20_wpt_X509DataType,
    pub PGPData: iso20_wpt_PGPDataType,
    pub SPKIData: iso20_wpt_SPKIDataType,
    pub Object: iso20_wpt_ObjectType,
    pub Manifest: iso20_wpt_ManifestType,
    pub SignatureProperties: iso20_wpt_SignaturePropertiesType,
    pub SignatureProperty: iso20_wpt_SignaturePropertyType,
    pub DSAKeyValue: iso20_wpt_DSAKeyValueType,
    pub RSAKeyValue: iso20_wpt_RSAKeyValueType,
}
unsafe extern "C" fn encode_iso20_wpt_TransformType(
    stream: &mut ExiBitstream,
    mut TransformType: *const iso20_wpt_TransformType,
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
unsafe extern "C" fn encode_iso20_wpt_TransformsType(
    stream: &mut ExiBitstream,
    mut TransformsType: *const iso20_wpt_TransformsType,
) -> i32 {
    let mut grammar_id: i32 = 4 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            4 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_TransformType(stream, &(*TransformsType).Transform);
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
                            encode_iso20_wpt_TransformType(stream, &(*TransformsType).Transform);
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
unsafe extern "C" fn encode_iso20_wpt_DSAKeyValueType(
    stream: &mut ExiBitstream,
    mut DSAKeyValueType: *const iso20_wpt_DSAKeyValueType,
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
unsafe extern "C" fn encode_iso20_wpt_X509IssuerSerialType(
    stream: &mut ExiBitstream,
    mut X509IssuerSerialType: *const iso20_wpt_X509IssuerSerialType,
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
unsafe extern "C" fn encode_iso20_wpt_DigestMethodType(
    stream: &mut ExiBitstream,
    mut DigestMethodType: *const iso20_wpt_DigestMethodType,
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
unsafe extern "C" fn encode_iso20_wpt_RSAKeyValueType(
    stream: &mut ExiBitstream,
    mut RSAKeyValueType: *const iso20_wpt_RSAKeyValueType,
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
unsafe extern "C" fn encode_iso20_wpt_CanonicalizationMethodType(
    stream: &mut ExiBitstream,
    mut CanonicalizationMethodType: *const iso20_wpt_CanonicalizationMethodType,
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
unsafe extern "C" fn encode_iso20_wpt_WPT_TxRxPulseOrderType(
    stream: &mut ExiBitstream,
    mut WPT_TxRxPulseOrderType: *const iso20_wpt_WPT_TxRxPulseOrderType,
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
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*WPT_TxRxPulseOrderType).IndexNumber,
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
                        error = exi_basetypes_encoder_uint_32(
                            stream,
                            (*WPT_TxRxPulseOrderType).TxRxIdentifier,
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
unsafe extern "C" fn encode_iso20_wpt_SignatureMethodType(
    stream: &mut ExiBitstream,
    mut SignatureMethodType: *const iso20_wpt_SignatureMethodType,
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
unsafe extern "C" fn encode_iso20_wpt_KeyValueType(
    stream: &mut ExiBitstream,
    mut KeyValueType: *const iso20_wpt_KeyValueType,
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
                        error =
                            encode_iso20_wpt_DSAKeyValueType(stream, &(*KeyValueType).DSAKeyValue);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyValueType).RSAKeyValue_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_wpt_RSAKeyValueType(stream, &(*KeyValueType).RSAKeyValue);
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
unsafe extern "C" fn encode_iso20_wpt_WPT_CoordinateXYZType(
    stream: &mut ExiBitstream,
    mut WPT_CoordinateXYZType: *const iso20_wpt_WPT_CoordinateXYZType,
) -> i32 {
    let mut grammar_id: i32 = 27 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            27 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_integer_16(
                            stream,
                            (*WPT_CoordinateXYZType).Coord_X,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 28 as i32;
                            }
                        }
                    }
                }
            }
            28 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_integer_16(
                            stream,
                            (*WPT_CoordinateXYZType).Coord_Y,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 29 as i32;
                            }
                        }
                    }
                }
            }
            29 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_integer_16(
                            stream,
                            (*WPT_CoordinateXYZType).Coord_Z,
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
unsafe extern "C" fn encode_iso20_wpt_ReferenceType(
    stream: &mut ExiBitstream,
    mut ReferenceType: *const iso20_wpt_ReferenceType,
) -> i32 {
    let mut grammar_id: i32 = 30 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            30 => {
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
                                grammar_id = 31 as i32;
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
                                grammar_id = 32 as i32;
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
                                grammar_id = 33 as i32;
                            }
                        }
                    }
                } else if (*ReferenceType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_wpt_TransformsType(stream, &(*ReferenceType).Transforms);
                        if error == 0 as i32 {
                            grammar_id = 34 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_DigestMethodType(
                            stream,
                            &(*ReferenceType).DigestMethod,
                        );
                        if error == 0 as i32 {
                            grammar_id = 35 as i32;
                        }
                    }
                }
            }
            31 => {
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
                                grammar_id = 32 as i32;
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
                                grammar_id = 33 as i32;
                            }
                        }
                    }
                } else if (*ReferenceType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_wpt_TransformsType(stream, &(*ReferenceType).Transforms);
                        if error == 0 as i32 {
                            grammar_id = 34 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_DigestMethodType(
                            stream,
                            &(*ReferenceType).DigestMethod,
                        );
                        if error == 0 as i32 {
                            grammar_id = 35 as i32;
                        }
                    }
                }
            }
            32 => {
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
                                grammar_id = 33 as i32;
                            }
                        }
                    }
                } else if (*ReferenceType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_wpt_TransformsType(stream, &(*ReferenceType).Transforms);
                        if error == 0 as i32 {
                            grammar_id = 34 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_DigestMethodType(
                            stream,
                            &(*ReferenceType).DigestMethod,
                        );
                        if error == 0 as i32 {
                            grammar_id = 35 as i32;
                        }
                    }
                }
            }
            33 => {
                if (*ReferenceType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_wpt_TransformsType(stream, &(*ReferenceType).Transforms);
                        if error == 0 as i32 {
                            grammar_id = 34 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_DigestMethodType(
                            stream,
                            &(*ReferenceType).DigestMethod,
                        );
                        if error == 0 as i32 {
                            grammar_id = 35 as i32;
                        }
                    }
                }
            }
            34 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso20_wpt_DigestMethodType(stream, &(*ReferenceType).DigestMethod);
                    if error == 0 as i32 {
                        grammar_id = 35 as i32;
                    }
                }
            }
            35 => {
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
unsafe extern "C" fn encode_iso20_wpt_RetrievalMethodType(
    stream: &mut ExiBitstream,
    mut RetrievalMethodType: *const iso20_wpt_RetrievalMethodType,
) -> i32 {
    let mut grammar_id: i32 = 36 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            36 => {
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
                                grammar_id = 37 as i32;
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
                                grammar_id = 38 as i32;
                            }
                        }
                    }
                } else if (*RetrievalMethodType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_TransformsType(
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
            37 => {
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
                                grammar_id = 38 as i32;
                            }
                        }
                    }
                } else if (*RetrievalMethodType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_TransformsType(
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
            38 => {
                if (*RetrievalMethodType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_TransformsType(
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
unsafe extern "C" fn encode_iso20_wpt_X509DataType(
    stream: &mut ExiBitstream,
    mut X509DataType: *const iso20_wpt_X509DataType,
) -> i32 {
    let mut grammar_id: i32 = 39 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            39 => {
                if (*X509DataType).X509IssuerSerial_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_X509IssuerSerialType(
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
unsafe extern "C" fn encode_iso20_wpt_PGPDataType(
    stream: &mut ExiBitstream,
    mut PGPDataType: *const iso20_wpt_PGPDataType,
) -> i32 {
    let mut grammar_id: i32 = 40 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            40 => {
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
                                        grammar_id = 41 as i32;
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
                                        grammar_id = 42 as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            41 => {
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
                                        grammar_id = 42 as i32;
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
                                        grammar_id = 43 as i32;
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
            42 => {
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
                                        grammar_id = 43 as i32;
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
            43 => {
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
                                    grammar_id = 44 as i32;
                                }
                            }
                        }
                    }
                }
            }
            44 => {
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
                                        grammar_id = 43 as i32;
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
unsafe extern "C" fn encode_iso20_wpt_SPKIDataType(
    stream: &mut ExiBitstream,
    mut SPKIDataType: *const iso20_wpt_SPKIDataType,
) -> i32 {
    let mut grammar_id: i32 = 45 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            45 => {
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
                                    grammar_id = 46 as i32;
                                }
                            }
                        }
                    }
                }
            }
            46 => {
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
unsafe extern "C" fn encode_iso20_wpt_SignedInfoType(
    stream: &mut ExiBitstream,
    mut SignedInfoType: *const iso20_wpt_SignedInfoType,
) -> i32 {
    let mut grammar_id: i32 = 47 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut Reference_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            47 => {
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
                                grammar_id = 48 as i32;
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_CanonicalizationMethodType(
                            stream,
                            &(*SignedInfoType).CanonicalizationMethod,
                        );
                        if error == 0 as i32 {
                            grammar_id = 49 as i32;
                        }
                    }
                }
            }
            48 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_CanonicalizationMethodType(
                        stream,
                        &(*SignedInfoType).CanonicalizationMethod,
                    );
                    if error == 0 as i32 {
                        grammar_id = 49 as i32;
                    }
                }
            }
            49 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_SignatureMethodType(
                        stream,
                        &(*SignedInfoType).SignatureMethod,
                    );
                    if error == 0 as i32 {
                        grammar_id = 50 as i32;
                    }
                }
            }
            50 => {
                if (Reference_currentIndex as i32) < (*SignedInfoType).Reference.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh0 = Reference_currentIndex;
                        Reference_currentIndex = Reference_currentIndex.wrapping_add(1);
                        error = encode_iso20_wpt_ReferenceType(
                            stream,
                            &*((*SignedInfoType).Reference.array)
                                .as_ptr()
                                .offset(fresh0 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 51 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            51 => {
                if (Reference_currentIndex as i32) < (*SignedInfoType).Reference.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh1 = Reference_currentIndex;
                        Reference_currentIndex = Reference_currentIndex.wrapping_add(1);
                        error = encode_iso20_wpt_ReferenceType(
                            stream,
                            &*((*SignedInfoType).Reference.array)
                                .as_ptr()
                                .offset(fresh1 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 51 as i32;
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
unsafe extern "C" fn encode_iso20_wpt_SignatureValueType(
    stream: &mut ExiBitstream,
    mut SignatureValueType: *const iso20_wpt_SignatureValueType,
) -> i32 {
    let mut grammar_id: i32 = 52 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            52 => {
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
                                grammar_id = 53 as i32;
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
            53 => {
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
unsafe extern "C" fn encode_iso20_wpt_RationalNumberType(
    stream: &mut ExiBitstream,
    mut RationalNumberType: *const iso20_wpt_RationalNumberType,
) -> i32 {
    let mut grammar_id: i32 = 54 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            54 => {
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
                                grammar_id = 55 as i32;
                            }
                        }
                    }
                }
            }
            55 => {
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
unsafe extern "C" fn encode_iso20_wpt_WPT_LF_RxRSSIType(
    stream: &mut ExiBitstream,
    mut WPT_LF_RxRSSIType: *const iso20_wpt_WPT_LF_RxRSSIType,
) -> i32 {
    let mut grammar_id: i32 = 56 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            56 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_32(
                            stream,
                            (*WPT_LF_RxRSSIType).TxIdentifier,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 57 as i32;
                            }
                        }
                    }
                }
            }
            57 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_RationalNumberType(stream, &(*WPT_LF_RxRSSIType).RSSI);
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
unsafe extern "C" fn encode_iso20_wpt_WPT_LF_RxRSSIListType(
    stream: &mut ExiBitstream,
    mut WPT_LF_RxRSSIListType: *const iso20_wpt_WPT_LF_RxRSSIListType,
) -> i32 {
    let mut grammar_id: i32 = 58 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            58 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_WPT_LF_RxRSSIType(
                        stream,
                        &(*WPT_LF_RxRSSIListType).RSSIDataList,
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
unsafe extern "C" fn encode_iso20_wpt_WPT_LF_TxDataType(
    stream: &mut ExiBitstream,
    mut WPT_LF_TxDataType: *const iso20_wpt_WPT_LF_TxDataType,
) -> i32 {
    let mut grammar_id: i32 = 59 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            59 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_32(
                            stream,
                            (*WPT_LF_TxDataType).TxIdentifier,
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
            }
            60 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_RationalNumberType(stream, &(*WPT_LF_TxDataType).EIRP);
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
unsafe extern "C" fn encode_iso20_wpt_WPT_LF_RxDataType(
    stream: &mut ExiBitstream,
    mut WPT_LF_RxDataType: *const iso20_wpt_WPT_LF_RxDataType,
) -> i32 {
    let mut grammar_id: i32 = 61 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            61 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_32(
                            stream,
                            (*WPT_LF_RxDataType).RxIdentifier,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 62 as i32;
                            }
                        }
                    }
                }
            }
            62 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_WPT_LF_RxRSSIListType(
                        stream,
                        &(*WPT_LF_RxDataType).RSSIData,
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
unsafe extern "C" fn encode_iso20_wpt_WPT_LF_TxDataListType(
    stream: &mut ExiBitstream,
    mut WPT_LF_TxDataListType: *const iso20_wpt_WPT_LF_TxDataListType,
) -> i32 {
    let mut grammar_id: i32 = 63 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            63 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_WPT_LF_TxDataType(
                        stream,
                        &(*WPT_LF_TxDataListType).WPT_LF_TxDataList,
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
unsafe extern "C" fn encode_iso20_wpt_KeyInfoType(
    stream: &mut ExiBitstream,
    mut KeyInfoType: *const iso20_wpt_KeyInfoType,
) -> i32 {
    let mut grammar_id: i32 = 64 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            64 => {
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
                                grammar_id = 65 as i32;
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
                        error = encode_iso20_wpt_KeyValueType(stream, &(*KeyInfoType).KeyValue);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).RetrievalMethod_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_RetrievalMethodType(
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
                        error = encode_iso20_wpt_X509DataType(stream, &(*KeyInfoType).X509Data);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).PGPData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_PGPDataType(stream, &(*KeyInfoType).PGPData);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).SPKIData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_SPKIDataType(stream, &(*KeyInfoType).SPKIData);
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
            65 => {
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
                        error = encode_iso20_wpt_KeyValueType(stream, &(*KeyInfoType).KeyValue);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).RetrievalMethod_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_RetrievalMethodType(
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
                        error = encode_iso20_wpt_X509DataType(stream, &(*KeyInfoType).X509Data);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).PGPData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_PGPDataType(stream, &(*KeyInfoType).PGPData);
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else if (*KeyInfoType).SPKIData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_SPKIDataType(stream, &(*KeyInfoType).SPKIData);
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
unsafe extern "C" fn encode_iso20_wpt_WPT_TxRxSpecDataType(
    stream: &mut ExiBitstream,
    mut WPT_TxRxSpecDataType: *const iso20_wpt_WPT_TxRxSpecDataType,
) -> i32 {
    let mut grammar_id: i32 = 66 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            66 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_32(
                            stream,
                            (*WPT_TxRxSpecDataType).TxRxIdentifier,
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
            }
            67 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_WPT_CoordinateXYZType(
                        stream,
                        &(*WPT_TxRxSpecDataType).TxRxPosition,
                    );
                    if error == 0 as i32 {
                        grammar_id = 68 as i32;
                    }
                }
            }
            68 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_WPT_CoordinateXYZType(
                        stream,
                        &(*WPT_TxRxSpecDataType).TxRxOrientation,
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
unsafe extern "C" fn encode_iso20_wpt_WPT_LF_RxDataListType(
    stream: &mut ExiBitstream,
    mut WPT_LF_RxDataListType: *const iso20_wpt_WPT_LF_RxDataListType,
) -> i32 {
    let mut grammar_id: i32 = 69 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            69 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_WPT_LF_RxDataType(
                        stream,
                        &(*WPT_LF_RxDataListType).WPT_LF_RxDataList,
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
unsafe extern "C" fn encode_iso20_wpt_ObjectType(
    stream: &mut ExiBitstream,
    mut ObjectType: *const iso20_wpt_ObjectType,
) -> i32 {
    let mut grammar_id: i32 = 70 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            70 => {
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
                                grammar_id = 71 as i32;
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
                                grammar_id = 72 as i32;
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
                                grammar_id = 73 as i32;
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
            71 => {
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
                                grammar_id = 72 as i32;
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
                                grammar_id = 73 as i32;
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
            72 => {
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
                                grammar_id = 73 as i32;
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
            73 => {
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
unsafe extern "C" fn encode_iso20_wpt_WPT_TxRxPackageSpecDataType(
    stream: &mut ExiBitstream,
    mut WPT_TxRxPackageSpecDataType: *const iso20_wpt_WPT_TxRxPackageSpecDataType,
) -> i32 {
    let mut grammar_id: i32 = 74 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut PulseSequenceOrder_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            74 => {
                if (PulseSequenceOrder_currentIndex as i32)
                    < (*WPT_TxRxPackageSpecDataType).PulseSequenceOrder.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh2 = PulseSequenceOrder_currentIndex;
                        PulseSequenceOrder_currentIndex =
                            PulseSequenceOrder_currentIndex.wrapping_add(1);
                        error = encode_iso20_wpt_WPT_TxRxPulseOrderType(
                            stream,
                            &*((*WPT_TxRxPackageSpecDataType).PulseSequenceOrder.array)
                                .as_ptr()
                                .offset(fresh2 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 75 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            75 => {
                if (PulseSequenceOrder_currentIndex as i32)
                    < (*WPT_TxRxPackageSpecDataType).PulseSequenceOrder.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh3 = PulseSequenceOrder_currentIndex;
                        PulseSequenceOrder_currentIndex =
                            PulseSequenceOrder_currentIndex.wrapping_add(1);
                        error = encode_iso20_wpt_WPT_TxRxPulseOrderType(
                            stream,
                            &*((*WPT_TxRxPackageSpecDataType).PulseSequenceOrder.array)
                                .as_ptr()
                                .offset(fresh3 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 75 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            76 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*WPT_TxRxPackageSpecDataType).PulseSeparationTime,
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
            }
            77 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*WPT_TxRxPackageSpecDataType).PulseDuration,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 78 as i32;
                            }
                        }
                    }
                }
            }
            78 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*WPT_TxRxPackageSpecDataType).PackageSeparationTime,
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
unsafe extern "C" fn encode_iso20_wpt_WPT_LF_TransmitterDataType(
    stream: &mut ExiBitstream,
    mut WPT_LF_TransmitterDataType: *const iso20_wpt_WPT_LF_TransmitterDataType,
) -> i32 {
    let mut grammar_id: i32 = 79 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut TxSpecData_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            79 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            8 as i32 as usize,
                            (*WPT_LF_TransmitterDataType).NumberOfTransmitters as u32,
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
                    error = encode_iso20_wpt_RationalNumberType(
                        stream,
                        &(*WPT_LF_TransmitterDataType).SignalFrequency,
                    );
                    if error == 0 as i32 {
                        grammar_id = 81 as i32;
                    }
                }
            }
            81 => {
                if (TxSpecData_currentIndex as i32)
                    < (*WPT_LF_TransmitterDataType).TxSpecData.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh4 = TxSpecData_currentIndex;
                        TxSpecData_currentIndex = TxSpecData_currentIndex.wrapping_add(1);
                        error = encode_iso20_wpt_WPT_TxRxSpecDataType(
                            stream,
                            &*((*WPT_LF_TransmitterDataType).TxSpecData.array)
                                .as_ptr()
                                .offset(fresh4 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 82 as i32;
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
            82 => {
                if (TxSpecData_currentIndex as i32)
                    < (*WPT_LF_TransmitterDataType).TxSpecData.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh5 = TxSpecData_currentIndex;
                        TxSpecData_currentIndex = TxSpecData_currentIndex.wrapping_add(1);
                        error = encode_iso20_wpt_WPT_TxRxSpecDataType(
                            stream,
                            &*((*WPT_LF_TransmitterDataType).TxSpecData.array)
                                .as_ptr()
                                .offset(fresh5 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 82 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            83 => {
                if (*WPT_LF_TransmitterDataType).TxPackageSpecData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_WPT_TxRxPackageSpecDataType(
                            stream,
                            &(*WPT_LF_TransmitterDataType).TxPackageSpecData,
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
unsafe extern "C" fn encode_iso20_wpt_AlternativeSECCType(
    stream: &mut ExiBitstream,
    mut AlternativeSECCType: *const iso20_wpt_AlternativeSECCType,
) -> i32 {
    let mut grammar_id: i32 = 84 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            84 => {
                if (*AlternativeSECCType).SSID_isUsed() == 1 as u32 {
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
                                ((*AlternativeSECCType).SSID.charactersLen as i32 + 2 as i32)
                                    as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*AlternativeSECCType).SSID.charactersLen as usize,
                                    ((*AlternativeSECCType).SSID.characters).as_ptr(),
                                    (255 as i32 + 1 as i32) as usize,
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
                } else if (*AlternativeSECCType).BSSID_isUsed() == 1 as u32 {
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
                                ((*AlternativeSECCType).BSSID.charactersLen as i32 + 2 as i32)
                                    as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*AlternativeSECCType).BSSID.charactersLen as usize,
                                    ((*AlternativeSECCType).BSSID.characters).as_ptr(),
                                    (12 as i32 + 1 as i32) as usize,
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
                } else if (*AlternativeSECCType).IPAddress_isUsed() == 1 as u32 {
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
                                ((*AlternativeSECCType).IPAddress.charactersLen as i32 + 2 as i32)
                                    as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*AlternativeSECCType).IPAddress.charactersLen as usize,
                                    ((*AlternativeSECCType).IPAddress.characters).as_ptr(),
                                    (39 as i32 + 1 as i32) as usize,
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
                } else if (*AlternativeSECCType).Port_isUsed() == 1 as u32 {
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
                                exi_basetypes_encoder_uint_16(stream, (*AlternativeSECCType).Port);
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
            85 => {
                if (*AlternativeSECCType).BSSID_isUsed() == 1 as u32 {
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
                                ((*AlternativeSECCType).BSSID.charactersLen as i32 + 2 as i32)
                                    as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*AlternativeSECCType).BSSID.charactersLen as usize,
                                    ((*AlternativeSECCType).BSSID.characters).as_ptr(),
                                    (12 as i32 + 1 as i32) as usize,
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
                } else if (*AlternativeSECCType).IPAddress_isUsed() == 1 as u32 {
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
                                ((*AlternativeSECCType).IPAddress.charactersLen as i32 + 2 as i32)
                                    as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*AlternativeSECCType).IPAddress.charactersLen as usize,
                                    ((*AlternativeSECCType).IPAddress.characters).as_ptr(),
                                    (39 as i32 + 1 as i32) as usize,
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
                } else if (*AlternativeSECCType).Port_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error =
                                exi_basetypes_encoder_uint_16(stream, (*AlternativeSECCType).Port);
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
            86 => {
                if (*AlternativeSECCType).IPAddress_isUsed() == 1 as u32 {
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
                                ((*AlternativeSECCType).IPAddress.charactersLen as i32 + 2 as i32)
                                    as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*AlternativeSECCType).IPAddress.charactersLen as usize,
                                    ((*AlternativeSECCType).IPAddress.characters).as_ptr(),
                                    (39 as i32 + 1 as i32) as usize,
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
                } else if (*AlternativeSECCType).Port_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error =
                                exi_basetypes_encoder_uint_16(stream, (*AlternativeSECCType).Port);
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
            87 => {
                if (*AlternativeSECCType).Port_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error =
                                exi_basetypes_encoder_uint_16(stream, (*AlternativeSECCType).Port);
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
unsafe extern "C" fn encode_iso20_wpt_WPT_LF_ReceiverDataType(
    stream: &mut ExiBitstream,
    mut WPT_LF_ReceiverDataType: *const iso20_wpt_WPT_LF_ReceiverDataType,
) -> i32 {
    let mut grammar_id: i32 = 88 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut RxSpecData_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            88 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            8 as i32 as usize,
                            (*WPT_LF_ReceiverDataType).NumberOfReceivers as u32,
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
                if (RxSpecData_currentIndex as i32)
                    < (*WPT_LF_ReceiverDataType).RxSpecData.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh6 = RxSpecData_currentIndex;
                        RxSpecData_currentIndex = RxSpecData_currentIndex.wrapping_add(1);
                        error = encode_iso20_wpt_WPT_TxRxSpecDataType(
                            stream,
                            &*((*WPT_LF_ReceiverDataType).RxSpecData.array)
                                .as_ptr()
                                .offset(fresh6 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 90 as i32;
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
            90 => {
                if (RxSpecData_currentIndex as i32)
                    < (*WPT_LF_ReceiverDataType).RxSpecData.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh7 = RxSpecData_currentIndex;
                        RxSpecData_currentIndex = RxSpecData_currentIndex.wrapping_add(1);
                        error = encode_iso20_wpt_WPT_TxRxSpecDataType(
                            stream,
                            &*((*WPT_LF_ReceiverDataType).RxSpecData.array)
                                .as_ptr()
                                .offset(fresh7 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 90 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
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
unsafe extern "C" fn encode_iso20_wpt_WPT_LF_DataPackageType(
    stream: &mut ExiBitstream,
    mut WPT_LF_DataPackageType: *const iso20_wpt_WPT_LF_DataPackageType,
) -> i32 {
    let mut grammar_id: i32 = 91 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            91 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            8 as i32 as usize,
                            (*WPT_LF_DataPackageType).PackageIndex as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 92 as i32;
                            }
                        }
                    }
                }
            }
            92 => {
                if (*WPT_LF_DataPackageType).LF_TxData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_WPT_LF_TxDataListType(
                            stream,
                            &(*WPT_LF_DataPackageType).LF_TxData,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_WPT_LF_RxDataListType(
                            stream,
                            &(*WPT_LF_DataPackageType).LF_RxData,
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
unsafe extern "C" fn encode_iso20_wpt_DetailedCostType(
    stream: &mut ExiBitstream,
    mut DetailedCostType: *const iso20_wpt_DetailedCostType,
) -> i32 {
    let mut grammar_id: i32 = 93 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            93 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso20_wpt_RationalNumberType(stream, &(*DetailedCostType).Amount);
                    if error == 0 as i32 {
                        grammar_id = 94 as i32;
                    }
                }
            }
            94 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_RationalNumberType(
                        stream,
                        &(*DetailedCostType).CostPerUnit,
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
unsafe extern "C" fn encode_iso20_wpt_SignatureType(
    stream: &mut ExiBitstream,
    mut SignatureType: *const iso20_wpt_SignatureType,
) -> i32 {
    let mut grammar_id: i32 = 95 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            95 => {
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
                                grammar_id = 96 as i32;
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_wpt_SignedInfoType(stream, &(*SignatureType).SignedInfo);
                        if error == 0 as i32 {
                            grammar_id = 97 as i32;
                        }
                    }
                }
            }
            96 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_SignedInfoType(stream, &(*SignatureType).SignedInfo);
                    if error == 0 as i32 {
                        grammar_id = 97 as i32;
                    }
                }
            }
            97 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_SignatureValueType(
                        stream,
                        &(*SignatureType).SignatureValue,
                    );
                    if error == 0 as i32 {
                        grammar_id = 98 as i32;
                    }
                }
            }
            98 => {
                if (*SignatureType).KeyInfo_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_KeyInfoType(stream, &(*SignatureType).KeyInfo);
                        if error == 0 as i32 {
                            grammar_id = 100 as i32;
                        }
                    }
                } else if (*SignatureType).Object_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_ObjectType(stream, &(*SignatureType).Object);
                        if error == 0 as i32 {
                            grammar_id = 99 as i32;
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
            99 => {
                if 1 as i32 == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_ObjectType(stream, &(*SignatureType).Object);
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
            100 => {
                if (*SignatureType).Object_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_ObjectType(stream, &(*SignatureType).Object);
                        if error == 0 as i32 {
                            grammar_id = 101 as i32;
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
            101 => {
                if 1 as i32 == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_ObjectType(stream, &(*SignatureType).Object);
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
unsafe extern "C" fn encode_iso20_wpt_DetailedTaxType(
    stream: &mut ExiBitstream,
    mut DetailedTaxType: *const iso20_wpt_DetailedTaxType,
) -> i32 {
    let mut grammar_id: i32 = 102 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            102 => {
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
                                grammar_id = 103 as i32;
                            }
                        }
                    }
                }
            }
            103 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_RationalNumberType(stream, &(*DetailedTaxType).Amount);
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
unsafe extern "C" fn encode_iso20_wpt_MessageHeaderType(
    stream: &mut ExiBitstream,
    mut MessageHeaderType: *const iso20_wpt_MessageHeaderType,
) -> i32 {
    let mut grammar_id: i32 = 104 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            104 => {
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
                        error =
                            exi_basetypes_encoder_uint_64(stream, (*MessageHeaderType).TimeStamp);
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
                if (*MessageHeaderType).Signature_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_wpt_SignatureType(stream, &(*MessageHeaderType).Signature);
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
unsafe extern "C" fn encode_iso20_wpt_SignaturePropertyType(
    stream: &mut ExiBitstream,
    mut SignaturePropertyType: *const iso20_wpt_SignaturePropertyType,
) -> i32 {
    let mut grammar_id: i32 = 107 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            107 => {
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
                                grammar_id = 108 as i32;
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
                                grammar_id = 109 as i32;
                            }
                        }
                    }
                }
            }
            108 => {
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
                            grammar_id = 109 as i32;
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
unsafe extern "C" fn encode_iso20_wpt_DisplayParametersType(
    stream: &mut ExiBitstream,
    mut DisplayParametersType: *const iso20_wpt_DisplayParametersType,
) -> i32 {
    let mut grammar_id: i32 = 110 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            110 => {
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
                                    grammar_id = 111 as i32;
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
                                    grammar_id = 112 as i32;
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
                                    grammar_id = 113 as i32;
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
                                    grammar_id = 114 as i32;
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
                                    grammar_id = 115 as i32;
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
                                    grammar_id = 116 as i32;
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
                                    grammar_id = 117 as i32;
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
                                    grammar_id = 118 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).BatteryEnergyCapacity_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 8 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_RationalNumberType(
                            stream,
                            &(*DisplayParametersType).BatteryEnergyCapacity,
                        );
                        if error == 0 as i32 {
                            grammar_id = 119 as i32;
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
            111 => {
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
                                    grammar_id = 112 as i32;
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
                                    grammar_id = 113 as i32;
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
                                    grammar_id = 114 as i32;
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
                                    grammar_id = 115 as i32;
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
                                    grammar_id = 116 as i32;
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
                                    grammar_id = 117 as i32;
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
                                    grammar_id = 118 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).BatteryEnergyCapacity_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_RationalNumberType(
                            stream,
                            &(*DisplayParametersType).BatteryEnergyCapacity,
                        );
                        if error == 0 as i32 {
                            grammar_id = 119 as i32;
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
            112 => {
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
                                    grammar_id = 113 as i32;
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
                                    grammar_id = 114 as i32;
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
                                    grammar_id = 115 as i32;
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
                                    grammar_id = 116 as i32;
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
                                    grammar_id = 117 as i32;
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
                                    grammar_id = 118 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).BatteryEnergyCapacity_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_RationalNumberType(
                            stream,
                            &(*DisplayParametersType).BatteryEnergyCapacity,
                        );
                        if error == 0 as i32 {
                            grammar_id = 119 as i32;
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
            113 => {
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
                                    grammar_id = 114 as i32;
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
                                    grammar_id = 115 as i32;
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
                                    grammar_id = 116 as i32;
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
                                    grammar_id = 117 as i32;
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
                                    grammar_id = 118 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).BatteryEnergyCapacity_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_RationalNumberType(
                            stream,
                            &(*DisplayParametersType).BatteryEnergyCapacity,
                        );
                        if error == 0 as i32 {
                            grammar_id = 119 as i32;
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
            114 => {
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
                                    grammar_id = 115 as i32;
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
                                    grammar_id = 116 as i32;
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
                                    grammar_id = 117 as i32;
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
                                    grammar_id = 118 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).BatteryEnergyCapacity_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_RationalNumberType(
                            stream,
                            &(*DisplayParametersType).BatteryEnergyCapacity,
                        );
                        if error == 0 as i32 {
                            grammar_id = 119 as i32;
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
            115 => {
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
                                    grammar_id = 116 as i32;
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
                                    grammar_id = 117 as i32;
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
                                    grammar_id = 118 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).BatteryEnergyCapacity_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_RationalNumberType(
                            stream,
                            &(*DisplayParametersType).BatteryEnergyCapacity,
                        );
                        if error == 0 as i32 {
                            grammar_id = 119 as i32;
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
            116 => {
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
                                    grammar_id = 117 as i32;
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
                                    grammar_id = 118 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).BatteryEnergyCapacity_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_RationalNumberType(
                            stream,
                            &(*DisplayParametersType).BatteryEnergyCapacity,
                        );
                        if error == 0 as i32 {
                            grammar_id = 119 as i32;
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
            117 => {
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
                                    grammar_id = 118 as i32;
                                }
                            }
                        }
                    }
                } else if (*DisplayParametersType).BatteryEnergyCapacity_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_RationalNumberType(
                            stream,
                            &(*DisplayParametersType).BatteryEnergyCapacity,
                        );
                        if error == 0 as i32 {
                            grammar_id = 119 as i32;
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
            118 => {
                if (*DisplayParametersType).BatteryEnergyCapacity_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_RationalNumberType(
                            stream,
                            &(*DisplayParametersType).BatteryEnergyCapacity,
                        );
                        if error == 0 as i32 {
                            grammar_id = 119 as i32;
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
            119 => {
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
unsafe extern "C" fn encode_iso20_wpt_WPT_FinePositioningMethodListType(
    stream: &mut ExiBitstream,
    mut WPT_FinePositioningMethodListType: *const iso20_wpt_WPT_FinePositioningMethodListType,
) -> i32 {
    let mut grammar_id: i32 = 120 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut WPT_FinePositioningMethod_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            120 => {
                if (WPT_FinePositioningMethod_currentIndex as i32)
                    < (*WPT_FinePositioningMethodListType)
                        .WPT_FinePositioningMethod
                        .arrayLen as i32
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
                            let fresh8 = WPT_FinePositioningMethod_currentIndex;
                            WPT_FinePositioningMethod_currentIndex =
                                WPT_FinePositioningMethod_currentIndex.wrapping_add(1);
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                3 as i32 as usize,
                                (*WPT_FinePositioningMethodListType)
                                    .WPT_FinePositioningMethod
                                    .array[fresh8 as usize] as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 121 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            121 => {
                if (WPT_FinePositioningMethod_currentIndex as i32)
                    < (*WPT_FinePositioningMethodListType)
                        .WPT_FinePositioningMethod
                        .arrayLen as i32
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
                            let fresh9 = WPT_FinePositioningMethod_currentIndex;
                            WPT_FinePositioningMethod_currentIndex =
                                WPT_FinePositioningMethod_currentIndex.wrapping_add(1);
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                3 as i32 as usize,
                                (*WPT_FinePositioningMethodListType)
                                    .WPT_FinePositioningMethod
                                    .array[fresh9 as usize] as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 121 as i32;
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
unsafe extern "C" fn encode_iso20_wpt_EVSEStatusType(
    stream: &mut ExiBitstream,
    mut EVSEStatusType: *const iso20_wpt_EVSEStatusType,
) -> i32 {
    let mut grammar_id: i32 = 122 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            122 => {
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
                                grammar_id = 123 as i32;
                            }
                        }
                    }
                }
            }
            123 => {
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
unsafe extern "C" fn encode_iso20_wpt_WPT_PairingMethodListType(
    stream: &mut ExiBitstream,
    mut WPT_PairingMethodListType: *const iso20_wpt_WPT_PairingMethodListType,
) -> i32 {
    let mut grammar_id: i32 = 124 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut WPT_PairingMethod_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            124 => {
                if (WPT_PairingMethod_currentIndex as i32)
                    < (*WPT_PairingMethodListType).WPT_PairingMethod.arrayLen as i32
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
                            let fresh10 = WPT_PairingMethod_currentIndex;
                            WPT_PairingMethod_currentIndex =
                                WPT_PairingMethod_currentIndex.wrapping_add(1);
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                3 as i32 as usize,
                                (*WPT_PairingMethodListType).WPT_PairingMethod.array
                                    [fresh10 as usize] as u32,
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
                } else {
                    error = -(150 as i32);
                }
            }
            125 => {
                if (WPT_PairingMethod_currentIndex as i32)
                    < (*WPT_PairingMethodListType).WPT_PairingMethod.arrayLen as i32
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
                            let fresh11 = WPT_PairingMethod_currentIndex;
                            WPT_PairingMethod_currentIndex =
                                WPT_PairingMethod_currentIndex.wrapping_add(1);
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                3 as i32 as usize,
                                (*WPT_PairingMethodListType).WPT_PairingMethod.array
                                    [fresh11 as usize] as u32,
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
unsafe extern "C" fn encode_iso20_wpt_MeterInfoType(
    stream: &mut ExiBitstream,
    mut MeterInfoType: *const iso20_wpt_MeterInfoType,
) -> i32 {
    let mut grammar_id: i32 = 126 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            126 => {
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
                                    grammar_id = 127 as i32;
                                }
                            }
                        }
                    }
                }
            }
            127 => {
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
                                grammar_id = 128 as i32;
                            }
                        }
                    }
                }
            }
            128 => {
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
                                    grammar_id = 129 as i32;
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
                                    grammar_id = 130 as i32;
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
                                    grammar_id = 131 as i32;
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
                                        grammar_id = 132 as i32;
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
                                    grammar_id = 133 as i32;
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
            129 => {
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
                                    grammar_id = 130 as i32;
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
                                    grammar_id = 131 as i32;
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
                                        grammar_id = 132 as i32;
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
                                    grammar_id = 133 as i32;
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
            130 => {
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
                                    grammar_id = 131 as i32;
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
                                        grammar_id = 132 as i32;
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
                                    grammar_id = 133 as i32;
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
            131 => {
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
                                        grammar_id = 132 as i32;
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
                                    grammar_id = 133 as i32;
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
            132 => {
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
                                    grammar_id = 133 as i32;
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
            133 => {
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
unsafe extern "C" fn encode_iso20_wpt_WPT_AlignmentCheckMethodListType(
    stream: &mut ExiBitstream,
    mut WPT_AlignmentCheckMethodListType: *const iso20_wpt_WPT_AlignmentCheckMethodListType,
) -> i32 {
    let mut grammar_id: i32 = 134 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut WPT_AlignmentCheckMethod_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            134 => {
                if (WPT_AlignmentCheckMethod_currentIndex as i32)
                    < (*WPT_AlignmentCheckMethodListType)
                        .WPT_AlignmentCheckMethod
                        .arrayLen as i32
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
                            let fresh12 = WPT_AlignmentCheckMethod_currentIndex;
                            WPT_AlignmentCheckMethod_currentIndex =
                                WPT_AlignmentCheckMethod_currentIndex.wrapping_add(1);
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                2 as i32 as usize,
                                (*WPT_AlignmentCheckMethodListType)
                                    .WPT_AlignmentCheckMethod
                                    .array[fresh12 as usize] as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 135 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            135 => {
                if (WPT_AlignmentCheckMethod_currentIndex as i32)
                    < (*WPT_AlignmentCheckMethodListType)
                        .WPT_AlignmentCheckMethod
                        .arrayLen as i32
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
                            let fresh13 = WPT_AlignmentCheckMethod_currentIndex;
                            WPT_AlignmentCheckMethod_currentIndex =
                                WPT_AlignmentCheckMethod_currentIndex.wrapping_add(1);
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                2 as i32 as usize,
                                (*WPT_AlignmentCheckMethodListType)
                                    .WPT_AlignmentCheckMethod
                                    .array[fresh13 as usize] as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 135 as i32;
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
unsafe extern "C" fn encode_iso20_wpt_WPT_LF_DataPackageListType(
    stream: &mut ExiBitstream,
    mut WPT_LF_DataPackageListType: *const iso20_wpt_WPT_LF_DataPackageListType,
) -> i32 {
    let mut grammar_id: i32 = 136 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            136 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            8 as i32 as usize,
                            (*WPT_LF_DataPackageListType).NumPackages as u32,
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
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_WPT_LF_DataPackageType(
                        stream,
                        &(*WPT_LF_DataPackageListType).WPT_LF_DataPackage,
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
unsafe extern "C" fn encode_iso20_wpt_AlternativeSECCListType(
    stream: &mut ExiBitstream,
    mut AlternativeSECCListType: *const iso20_wpt_AlternativeSECCListType,
) -> i32 {
    let mut grammar_id: i32 = 138 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut AlternativeSECC_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            138 => {
                if (AlternativeSECC_currentIndex as i32)
                    < (*AlternativeSECCListType).AlternativeSECC.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh14 = AlternativeSECC_currentIndex;
                        AlternativeSECC_currentIndex = AlternativeSECC_currentIndex.wrapping_add(1);
                        error = encode_iso20_wpt_AlternativeSECCType(
                            stream,
                            &*((*AlternativeSECCListType).AlternativeSECC.array)
                                .as_ptr()
                                .offset(fresh14 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 139 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            139 => {
                if (AlternativeSECC_currentIndex as i32)
                    < (*AlternativeSECCListType).AlternativeSECC.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh15 = AlternativeSECC_currentIndex;
                        AlternativeSECC_currentIndex = AlternativeSECC_currentIndex.wrapping_add(1);
                        error = encode_iso20_wpt_AlternativeSECCType(
                            stream,
                            &*((*AlternativeSECCListType).AlternativeSECC.array)
                                .as_ptr()
                                .offset(fresh15 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 139 as i32;
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
unsafe extern "C" fn encode_iso20_wpt_ReceiptType(
    stream: &mut ExiBitstream,
    mut ReceiptType: *const iso20_wpt_ReceiptType,
) -> i32 {
    let mut grammar_id: i32 = 140 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut TaxCosts_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            140 => {
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
                                grammar_id = 141 as i32;
                            }
                        }
                    }
                }
            }
            141 => {
                if (*ReceiptType).EnergyCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_wpt_DetailedCostType(stream, &(*ReceiptType).EnergyCosts);
                        if error == 0 as i32 {
                            grammar_id = 143 as i32;
                        }
                    }
                } else if (*ReceiptType).OccupancyCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_DetailedCostType(
                            stream,
                            &(*ReceiptType).OccupancyCosts,
                        );
                        if error == 0 as i32 {
                            grammar_id = 145 as i32;
                        }
                    }
                } else if (*ReceiptType).AdditionalServicesCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_DetailedCostType(
                            stream,
                            &(*ReceiptType).AdditionalServicesCosts,
                        );
                        if error == 0 as i32 {
                            grammar_id = 147 as i32;
                        }
                    }
                } else if (*ReceiptType).OverstayCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_DetailedCostType(
                            stream,
                            &(*ReceiptType).OverstayCosts,
                        );
                        if error == 0 as i32 {
                            grammar_id = 149 as i32;
                        }
                    }
                } else if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh16 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_wpt_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh16 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 142 as i32;
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
            142 => {
                if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh17 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_wpt_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh17 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 142 as i32;
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
            143 => {
                if (*ReceiptType).OccupancyCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_DetailedCostType(
                            stream,
                            &(*ReceiptType).OccupancyCosts,
                        );
                        if error == 0 as i32 {
                            grammar_id = 145 as i32;
                        }
                    }
                } else if (*ReceiptType).AdditionalServicesCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_DetailedCostType(
                            stream,
                            &(*ReceiptType).AdditionalServicesCosts,
                        );
                        if error == 0 as i32 {
                            grammar_id = 147 as i32;
                        }
                    }
                } else if (*ReceiptType).OverstayCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_DetailedCostType(
                            stream,
                            &(*ReceiptType).OverstayCosts,
                        );
                        if error == 0 as i32 {
                            grammar_id = 149 as i32;
                        }
                    }
                } else if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh18 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_wpt_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh18 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 144 as i32;
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
            144 => {
                if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh19 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_wpt_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh19 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 144 as i32;
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
            145 => {
                if (*ReceiptType).AdditionalServicesCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_DetailedCostType(
                            stream,
                            &(*ReceiptType).AdditionalServicesCosts,
                        );
                        if error == 0 as i32 {
                            grammar_id = 147 as i32;
                        }
                    }
                } else if (*ReceiptType).OverstayCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_DetailedCostType(
                            stream,
                            &(*ReceiptType).OverstayCosts,
                        );
                        if error == 0 as i32 {
                            grammar_id = 149 as i32;
                        }
                    }
                } else if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh20 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_wpt_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh20 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 146 as i32;
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
                if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh21 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_wpt_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh21 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 146 as i32;
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
            147 => {
                if (*ReceiptType).OverstayCosts_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_DetailedCostType(
                            stream,
                            &(*ReceiptType).OverstayCosts,
                        );
                        if error == 0 as i32 {
                            grammar_id = 149 as i32;
                        }
                    }
                } else if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh22 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_wpt_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh22 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 148 as i32;
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
            148 => {
                if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh23 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_wpt_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh23 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 148 as i32;
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
            149 => {
                if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh24 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_wpt_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh24 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 150 as i32;
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
                if (TaxCosts_currentIndex as i32) < (*ReceiptType).TaxCosts.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh25 = TaxCosts_currentIndex;
                        TaxCosts_currentIndex = TaxCosts_currentIndex.wrapping_add(1);
                        error = encode_iso20_wpt_DetailedTaxType(
                            stream,
                            &*((*ReceiptType).TaxCosts.array)
                                .as_ptr()
                                .offset(fresh25 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 150 as i32;
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
unsafe extern "C" fn encode_iso20_wpt_WPT_LF_SystemSetupDataType(
    stream: &mut ExiBitstream,
    mut WPT_LF_SystemSetupDataType: *const iso20_wpt_WPT_LF_SystemSetupDataType,
) -> i32 {
    let mut grammar_id: i32 = 151 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            151 => {
                if (*WPT_LF_SystemSetupDataType).LF_TransmitterSetupData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_WPT_LF_TransmitterDataType(
                            stream,
                            &(*WPT_LF_SystemSetupDataType).LF_TransmitterSetupData,
                        );
                        if error == 0 as i32 {
                            grammar_id = 2 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_WPT_LF_ReceiverDataType(
                            stream,
                            &(*WPT_LF_SystemSetupDataType).LF_ReceiverSetupData,
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
unsafe extern "C" fn encode_iso20_wpt_WPT_EVPCPowerControlParameterType(
    stream: &mut ExiBitstream,
    mut WPT_EVPCPowerControlParameterType: *const iso20_wpt_WPT_EVPCPowerControlParameterType,
) -> i32 {
    let mut grammar_id: i32 = 152 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            152 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_RationalNumberType(
                        stream,
                        &(*WPT_EVPCPowerControlParameterType).EVPCCoilCurrentRequest,
                    );
                    if error == 0 as i32 {
                        grammar_id = 153 as i32;
                    }
                }
            }
            153 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_RationalNumberType(
                        stream,
                        &(*WPT_EVPCPowerControlParameterType).EVPCCoilCurrentInformation,
                    );
                    if error == 0 as i32 {
                        grammar_id = 154 as i32;
                    }
                }
            }
            154 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_RationalNumberType(
                        stream,
                        &(*WPT_EVPCPowerControlParameterType).EVPCCurrentOutputInformation,
                    );
                    if error == 0 as i32 {
                        grammar_id = 155 as i32;
                    }
                }
            }
            155 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_RationalNumberType(
                        stream,
                        &(*WPT_EVPCPowerControlParameterType).EVPCVoltageOutputInformation,
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
unsafe extern "C" fn encode_iso20_wpt_WPT_SPCPowerControlParameterType(
    stream: &mut ExiBitstream,
    mut WPT_SPCPowerControlParameterType: *const iso20_wpt_WPT_SPCPowerControlParameterType,
) -> i32 {
    let mut grammar_id: i32 = 156 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            156 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_RationalNumberType(
                        stream,
                        &(*WPT_SPCPowerControlParameterType).SPCPrimaryDeviceCoilCurrentInformation,
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
unsafe extern "C" fn encode_iso20_wpt_WPT_FinePositioningSetupReqType(
    stream: &mut ExiBitstream,
    mut WPT_FinePositioningSetupReqType: *const iso20_wpt_WPT_FinePositioningSetupReqType,
) -> i32 {
    let mut grammar_id: i32 = 157 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut VendorSpecificDataContainer_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            157 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_MessageHeaderType(
                        stream,
                        &(*WPT_FinePositioningSetupReqType).Header,
                    );
                    if error == 0 as i32 {
                        grammar_id = 158 as i32;
                    }
                }
            }
            158 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*WPT_FinePositioningSetupReqType).EVProcessing as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 159 as i32;
                            }
                        }
                    }
                }
            }
            159 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_WPT_FinePositioningMethodListType(
                        stream,
                        &(*WPT_FinePositioningSetupReqType).EVDeviceFinePositioningMethodList,
                    );
                    if error == 0 as i32 {
                        grammar_id = 160 as i32;
                    }
                }
            }
            160 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_WPT_PairingMethodListType(
                        stream,
                        &(*WPT_FinePositioningSetupReqType).EVDevicePairingMethodList,
                    );
                    if error == 0 as i32 {
                        grammar_id = 161 as i32;
                    }
                }
            }
            161 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_WPT_AlignmentCheckMethodListType(
                        stream,
                        &(*WPT_FinePositioningSetupReqType).EVDeviceAlignmentCheckMethodList,
                    );
                    if error == 0 as i32 {
                        grammar_id = 162 as i32;
                    }
                }
            }
            162 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*WPT_FinePositioningSetupReqType).NaturalOffset,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 163 as i32;
                            }
                        }
                    }
                }
            }
            163 => {
                if (VendorSpecificDataContainer_currentIndex as i32)
                    < (*WPT_FinePositioningSetupReqType)
                        .VendorSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_FinePositioningSetupReqType)
                                    .VendorSpecificDataContainer
                                    .array
                                    [VendorSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_FinePositioningSetupReqType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_FinePositioningSetupReqType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    VendorSpecificDataContainer_currentIndex =
                                        VendorSpecificDataContainer_currentIndex.wrapping_add(1);
                                    VendorSpecificDataContainer_currentIndex;
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
                if (VendorSpecificDataContainer_currentIndex as i32)
                    < (*WPT_FinePositioningSetupReqType)
                        .VendorSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_FinePositioningSetupReqType)
                                    .VendorSpecificDataContainer
                                    .array
                                    [VendorSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_FinePositioningSetupReqType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_FinePositioningSetupReqType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    VendorSpecificDataContainer_currentIndex =
                                        VendorSpecificDataContainer_currentIndex.wrapping_add(1);
                                    VendorSpecificDataContainer_currentIndex;
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
                    }
                } else if (*WPT_FinePositioningSetupReqType).LF_SystemSetupData_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_WPT_LF_SystemSetupDataType(
                            stream,
                            &(*WPT_FinePositioningSetupReqType).LF_SystemSetupData,
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
            165 => {
                if (*WPT_FinePositioningSetupReqType).LF_SystemSetupData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_WPT_LF_SystemSetupDataType(
                            stream,
                            &(*WPT_FinePositioningSetupReqType).LF_SystemSetupData,
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
unsafe extern "C" fn encode_iso20_wpt_WPT_FinePositioningSetupResType(
    stream: &mut ExiBitstream,
    mut WPT_FinePositioningSetupResType: *const iso20_wpt_WPT_FinePositioningSetupResType,
) -> i32 {
    let mut grammar_id: i32 = 166 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut VendorSpecificDataContainer_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            166 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_MessageHeaderType(
                        stream,
                        &(*WPT_FinePositioningSetupResType).Header,
                    );
                    if error == 0 as i32 {
                        grammar_id = 167 as i32;
                    }
                }
            }
            167 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            6 as i32 as usize,
                            (*WPT_FinePositioningSetupResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 168 as i32;
                            }
                        }
                    }
                }
            }
            168 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_WPT_FinePositioningMethodListType(
                        stream,
                        &(*WPT_FinePositioningSetupResType).PrimaryDeviceFinePositioningMethodList,
                    );
                    if error == 0 as i32 {
                        grammar_id = 169 as i32;
                    }
                }
            }
            169 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_WPT_PairingMethodListType(
                        stream,
                        &(*WPT_FinePositioningSetupResType).PrimaryDevicePairingMethodList,
                    );
                    if error == 0 as i32 {
                        grammar_id = 170 as i32;
                    }
                }
            }
            170 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_WPT_AlignmentCheckMethodListType(
                        stream,
                        &(*WPT_FinePositioningSetupResType).PrimaryDeviceAlignmentCheckMethodList,
                    );
                    if error == 0 as i32 {
                        grammar_id = 171 as i32;
                    }
                }
            }
            171 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*WPT_FinePositioningSetupResType).NaturalOffset,
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
            172 => {
                if (VendorSpecificDataContainer_currentIndex as i32)
                    < (*WPT_FinePositioningSetupResType)
                        .VendorSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_FinePositioningSetupResType)
                                    .VendorSpecificDataContainer
                                    .array
                                    [VendorSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_FinePositioningSetupResType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_FinePositioningSetupResType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    VendorSpecificDataContainer_currentIndex =
                                        VendorSpecificDataContainer_currentIndex.wrapping_add(1);
                                    VendorSpecificDataContainer_currentIndex;
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
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            173 => {
                if (VendorSpecificDataContainer_currentIndex as i32)
                    < (*WPT_FinePositioningSetupResType)
                        .VendorSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_FinePositioningSetupResType)
                                    .VendorSpecificDataContainer
                                    .array
                                    [VendorSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_FinePositioningSetupResType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_FinePositioningSetupResType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    VendorSpecificDataContainer_currentIndex =
                                        VendorSpecificDataContainer_currentIndex.wrapping_add(1);
                                    VendorSpecificDataContainer_currentIndex;
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
                } else if (*WPT_FinePositioningSetupResType).LF_SystemSetupData_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_WPT_LF_SystemSetupDataType(
                            stream,
                            &(*WPT_FinePositioningSetupResType).LF_SystemSetupData,
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
            174 => {
                if (*WPT_FinePositioningSetupResType).LF_SystemSetupData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_WPT_LF_SystemSetupDataType(
                            stream,
                            &(*WPT_FinePositioningSetupResType).LF_SystemSetupData,
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
unsafe extern "C" fn encode_iso20_wpt_WPT_FinePositioningReqType(
    stream: &mut ExiBitstream,
    mut WPT_FinePositioningReqType: *const iso20_wpt_WPT_FinePositioningReqType,
) -> i32 {
    let mut grammar_id: i32 = 175 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut VendorSpecificDataContainer_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            175 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_MessageHeaderType(
                        stream,
                        &(*WPT_FinePositioningReqType).Header,
                    );
                    if error == 0 as i32 {
                        grammar_id = 176 as i32;
                    }
                }
            }
            176 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*WPT_FinePositioningReqType).EVProcessing as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 177 as i32;
                            }
                        }
                    }
                }
            }
            177 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*WPT_FinePositioningReqType).EVResultCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 178 as i32;
                            }
                        }
                    }
                }
            }
            178 => {
                if (VendorSpecificDataContainer_currentIndex as i32)
                    < (*WPT_FinePositioningReqType)
                        .VendorSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_FinePositioningReqType)
                                    .VendorSpecificDataContainer
                                    .array
                                    [VendorSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_FinePositioningReqType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_FinePositioningReqType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    VendorSpecificDataContainer_currentIndex =
                                        VendorSpecificDataContainer_currentIndex.wrapping_add(1);
                                    VendorSpecificDataContainer_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 179 as i32;
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
            179 => {
                if (VendorSpecificDataContainer_currentIndex as i32)
                    < (*WPT_FinePositioningReqType)
                        .VendorSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_FinePositioningReqType)
                                    .VendorSpecificDataContainer
                                    .array
                                    [VendorSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_FinePositioningReqType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_FinePositioningReqType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    VendorSpecificDataContainer_currentIndex =
                                        VendorSpecificDataContainer_currentIndex.wrapping_add(1);
                                    VendorSpecificDataContainer_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 180 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*WPT_FinePositioningReqType).WPT_LF_DataPackageList_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_WPT_LF_DataPackageListType(
                            stream,
                            &(*WPT_FinePositioningReqType).WPT_LF_DataPackageList,
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
                if (*WPT_FinePositioningReqType).WPT_LF_DataPackageList_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_WPT_LF_DataPackageListType(
                            stream,
                            &(*WPT_FinePositioningReqType).WPT_LF_DataPackageList,
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
unsafe extern "C" fn encode_iso20_wpt_WPT_FinePositioningResType(
    stream: &mut ExiBitstream,
    mut WPT_FinePositioningResType: *const iso20_wpt_WPT_FinePositioningResType,
) -> i32 {
    let mut grammar_id: i32 = 181 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut VendorSpecificDataContainer_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            181 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_MessageHeaderType(
                        stream,
                        &(*WPT_FinePositioningResType).Header,
                    );
                    if error == 0 as i32 {
                        grammar_id = 182 as i32;
                    }
                }
            }
            182 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            6 as i32 as usize,
                            (*WPT_FinePositioningResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 183 as i32;
                            }
                        }
                    }
                }
            }
            183 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*WPT_FinePositioningResType).EVSEProcessing as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 184 as i32;
                            }
                        }
                    }
                }
            }
            184 => {
                if (VendorSpecificDataContainer_currentIndex as i32)
                    < (*WPT_FinePositioningResType)
                        .VendorSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_FinePositioningResType)
                                    .VendorSpecificDataContainer
                                    .array
                                    [VendorSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_FinePositioningResType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_FinePositioningResType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    VendorSpecificDataContainer_currentIndex =
                                        VendorSpecificDataContainer_currentIndex.wrapping_add(1);
                                    VendorSpecificDataContainer_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 185 as i32;
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
            185 => {
                if (VendorSpecificDataContainer_currentIndex as i32)
                    < (*WPT_FinePositioningResType)
                        .VendorSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_FinePositioningResType)
                                    .VendorSpecificDataContainer
                                    .array
                                    [VendorSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_FinePositioningResType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_FinePositioningResType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    VendorSpecificDataContainer_currentIndex =
                                        VendorSpecificDataContainer_currentIndex.wrapping_add(1);
                                    VendorSpecificDataContainer_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 186 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*WPT_FinePositioningResType).WPT_LF_DataPackageList_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_WPT_LF_DataPackageListType(
                            stream,
                            &(*WPT_FinePositioningResType).WPT_LF_DataPackageList,
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
            186 => {
                if (*WPT_FinePositioningResType).WPT_LF_DataPackageList_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_WPT_LF_DataPackageListType(
                            stream,
                            &(*WPT_FinePositioningResType).WPT_LF_DataPackageList,
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
unsafe extern "C" fn encode_iso20_wpt_WPT_PairingReqType(
    stream: &mut ExiBitstream,
    mut WPT_PairingReqType: *const iso20_wpt_WPT_PairingReqType,
) -> i32 {
    let mut grammar_id: i32 = 187 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut VendorSpecificDataContainer_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            187 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso20_wpt_MessageHeaderType(stream, &(*WPT_PairingReqType).Header);
                    if error == 0 as i32 {
                        grammar_id = 188 as i32;
                    }
                }
            }
            188 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*WPT_PairingReqType).EVProcessing as u32,
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
            189 => {
                if (*WPT_PairingReqType).ObservedIDCode_isUsed() == 1 as u32 {
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
                                (*WPT_PairingReqType).ObservedIDCode,
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
                                2 as i32 as usize,
                                (*WPT_PairingReqType).EVResultCode as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 191 as i32;
                                }
                            }
                        }
                    }
                }
            }
            190 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*WPT_PairingReqType).EVResultCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 191 as i32;
                            }
                        }
                    }
                }
            }
            191 => {
                if (VendorSpecificDataContainer_currentIndex as i32)
                    < (*WPT_PairingReqType).VendorSpecificDataContainer.arrayLen as i32
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
                                (*WPT_PairingReqType).VendorSpecificDataContainer.array
                                    [VendorSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_PairingReqType).VendorSpecificDataContainer.array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_PairingReqType).VendorSpecificDataContainer.array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    VendorSpecificDataContainer_currentIndex =
                                        VendorSpecificDataContainer_currentIndex.wrapping_add(1);
                                    VendorSpecificDataContainer_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 192 as i32;
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
            192 => {
                if (VendorSpecificDataContainer_currentIndex as i32)
                    < (*WPT_PairingReqType).VendorSpecificDataContainer.arrayLen as i32
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
                                (*WPT_PairingReqType).VendorSpecificDataContainer.array
                                    [VendorSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_PairingReqType).VendorSpecificDataContainer.array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_PairingReqType).VendorSpecificDataContainer.array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    VendorSpecificDataContainer_currentIndex =
                                        VendorSpecificDataContainer_currentIndex.wrapping_add(1);
                                    VendorSpecificDataContainer_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 192 as i32;
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
unsafe extern "C" fn encode_iso20_wpt_WPT_PairingResType(
    stream: &mut ExiBitstream,
    mut WPT_PairingResType: *const iso20_wpt_WPT_PairingResType,
) -> i32 {
    let mut grammar_id: i32 = 193 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut VendorSpecificDataContainer_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            193 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso20_wpt_MessageHeaderType(stream, &(*WPT_PairingResType).Header);
                    if error == 0 as i32 {
                        grammar_id = 194 as i32;
                    }
                }
            }
            194 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            6 as i32 as usize,
                            (*WPT_PairingResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 195 as i32;
                            }
                        }
                    }
                }
            }
            195 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*WPT_PairingResType).EVSEProcessing as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 196 as i32;
                            }
                        }
                    }
                }
            }
            196 => {
                if (*WPT_PairingResType).ObservedIDCode_isUsed() == 1 as u32 {
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
                                (*WPT_PairingResType).ObservedIDCode,
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
                } else if (*WPT_PairingResType).AlternativeSECCList_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_AlternativeSECCListType(
                            stream,
                            &(*WPT_PairingResType).AlternativeSECCList,
                        );
                        if error == 0 as i32 {
                            grammar_id = 200 as i32;
                        }
                    }
                } else if (VendorSpecificDataContainer_currentIndex as i32)
                    < (*WPT_PairingResType).VendorSpecificDataContainer.arrayLen as i32
                {
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
                                (*WPT_PairingResType).VendorSpecificDataContainer.array
                                    [VendorSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_PairingResType).VendorSpecificDataContainer.array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_PairingResType).VendorSpecificDataContainer.array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    VendorSpecificDataContainer_currentIndex =
                                        VendorSpecificDataContainer_currentIndex.wrapping_add(1);
                                    VendorSpecificDataContainer_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 197 as i32;
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
            197 => {
                if (VendorSpecificDataContainer_currentIndex as i32)
                    < (*WPT_PairingResType).VendorSpecificDataContainer.arrayLen as i32
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
                                (*WPT_PairingResType).VendorSpecificDataContainer.array
                                    [VendorSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_PairingResType).VendorSpecificDataContainer.array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_PairingResType).VendorSpecificDataContainer.array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    VendorSpecificDataContainer_currentIndex =
                                        VendorSpecificDataContainer_currentIndex.wrapping_add(1);
                                    VendorSpecificDataContainer_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 197 as i32;
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
            198 => {
                if (*WPT_PairingResType).AlternativeSECCList_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_AlternativeSECCListType(
                            stream,
                            &(*WPT_PairingResType).AlternativeSECCList,
                        );
                        if error == 0 as i32 {
                            grammar_id = 200 as i32;
                        }
                    }
                } else if (VendorSpecificDataContainer_currentIndex as i32)
                    < (*WPT_PairingResType).VendorSpecificDataContainer.arrayLen as i32
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
                                (*WPT_PairingResType).VendorSpecificDataContainer.array
                                    [VendorSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_PairingResType).VendorSpecificDataContainer.array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_PairingResType).VendorSpecificDataContainer.array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    VendorSpecificDataContainer_currentIndex =
                                        VendorSpecificDataContainer_currentIndex.wrapping_add(1);
                                    VendorSpecificDataContainer_currentIndex;
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
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            199 => {
                if (VendorSpecificDataContainer_currentIndex as i32)
                    < (*WPT_PairingResType).VendorSpecificDataContainer.arrayLen as i32
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
                                (*WPT_PairingResType).VendorSpecificDataContainer.array
                                    [VendorSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_PairingResType).VendorSpecificDataContainer.array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_PairingResType).VendorSpecificDataContainer.array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    VendorSpecificDataContainer_currentIndex =
                                        VendorSpecificDataContainer_currentIndex.wrapping_add(1);
                                    VendorSpecificDataContainer_currentIndex;
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
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            200 => {
                if (VendorSpecificDataContainer_currentIndex as i32)
                    < (*WPT_PairingResType).VendorSpecificDataContainer.arrayLen as i32
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
                                (*WPT_PairingResType).VendorSpecificDataContainer.array
                                    [VendorSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_PairingResType).VendorSpecificDataContainer.array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_PairingResType).VendorSpecificDataContainer.array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    VendorSpecificDataContainer_currentIndex =
                                        VendorSpecificDataContainer_currentIndex.wrapping_add(1);
                                    VendorSpecificDataContainer_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 201 as i32;
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
            201 => {
                if (VendorSpecificDataContainer_currentIndex as i32)
                    < (*WPT_PairingResType).VendorSpecificDataContainer.arrayLen as i32
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
                                (*WPT_PairingResType).VendorSpecificDataContainer.array
                                    [VendorSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_PairingResType).VendorSpecificDataContainer.array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_PairingResType).VendorSpecificDataContainer.array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    VendorSpecificDataContainer_currentIndex =
                                        VendorSpecificDataContainer_currentIndex.wrapping_add(1);
                                    VendorSpecificDataContainer_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 201 as i32;
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
unsafe extern "C" fn encode_iso20_wpt_WPT_ChargeParameterDiscoveryReqType(
    stream: &mut ExiBitstream,
    mut WPT_ChargeParameterDiscoveryReqType: *const iso20_wpt_WPT_ChargeParameterDiscoveryReqType,
) -> i32 {
    let mut grammar_id: i32 = 202 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut VendorSpecificDataContainer_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            202 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_MessageHeaderType(
                        stream,
                        &(*WPT_ChargeParameterDiscoveryReqType).Header,
                    );
                    if error == 0 as i32 {
                        grammar_id = 203 as i32;
                    }
                }
            }
            203 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_RationalNumberType(
                        stream,
                        &(*WPT_ChargeParameterDiscoveryReqType).EVPCMaxReceivablePower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 204 as i32;
                    }
                }
            }
            204 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*WPT_ChargeParameterDiscoveryReqType).SDMaxGroundClearence,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 205 as i32;
                            }
                        }
                    }
                }
            }
            205 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*WPT_ChargeParameterDiscoveryReqType).SDMinGroundClearence,
                        );
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
                    error = encode_iso20_wpt_RationalNumberType(
                        stream,
                        &(*WPT_ChargeParameterDiscoveryReqType).EVPCNaturalFrequency,
                    );
                    if error == 0 as i32 {
                        grammar_id = 207 as i32;
                    }
                }
            }
            207 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(
                            stream,
                            (*WPT_ChargeParameterDiscoveryReqType).EVPCDeviceLocalControl,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 208 as i32;
                            }
                        }
                    }
                }
            }
            208 => {
                if (VendorSpecificDataContainer_currentIndex as i32)
                    < (*WPT_ChargeParameterDiscoveryReqType)
                        .VendorSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_ChargeParameterDiscoveryReqType)
                                    .VendorSpecificDataContainer
                                    .array
                                    [VendorSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_ChargeParameterDiscoveryReqType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_ChargeParameterDiscoveryReqType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    VendorSpecificDataContainer_currentIndex =
                                        VendorSpecificDataContainer_currentIndex.wrapping_add(1);
                                    VendorSpecificDataContainer_currentIndex;
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
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            209 => {
                if (VendorSpecificDataContainer_currentIndex as i32)
                    < (*WPT_ChargeParameterDiscoveryReqType)
                        .VendorSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_ChargeParameterDiscoveryReqType)
                                    .VendorSpecificDataContainer
                                    .array
                                    [VendorSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_ChargeParameterDiscoveryReqType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_ChargeParameterDiscoveryReqType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    VendorSpecificDataContainer_currentIndex =
                                        VendorSpecificDataContainer_currentIndex.wrapping_add(1);
                                    VendorSpecificDataContainer_currentIndex;
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
unsafe extern "C" fn encode_iso20_wpt_WPT_ChargeParameterDiscoveryResType(
    stream: &mut ExiBitstream,
    mut WPT_ChargeParameterDiscoveryResType: *const iso20_wpt_WPT_ChargeParameterDiscoveryResType,
) -> i32 {
    let mut grammar_id: i32 = 210 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut SDManufacturerSpecificDataContainer_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            210 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_MessageHeaderType(
                        stream,
                        &(*WPT_ChargeParameterDiscoveryResType).Header,
                    );
                    if error == 0 as i32 {
                        grammar_id = 211 as i32;
                    }
                }
            }
            211 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            6 as i32 as usize,
                            (*WPT_ChargeParameterDiscoveryResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 212 as i32;
                            }
                        }
                    }
                }
            }
            212 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*WPT_ChargeParameterDiscoveryResType).PDInputPowerClass as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 213 as i32;
                            }
                        }
                    }
                }
            }
            213 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_RationalNumberType(
                        stream,
                        &(*WPT_ChargeParameterDiscoveryResType).SDMinOutputPower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 214 as i32;
                    }
                }
            }
            214 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_RationalNumberType(
                        stream,
                        &(*WPT_ChargeParameterDiscoveryResType).SDMaxOutputPower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 215 as i32;
                    }
                }
            }
            215 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*WPT_ChargeParameterDiscoveryResType).SDMaxGroundClearanceSupport,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 216 as i32;
                            }
                        }
                    }
                }
            }
            216 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*WPT_ChargeParameterDiscoveryResType).SDMinGroundClearanceSupport,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 217 as i32;
                            }
                        }
                    }
                }
            }
            217 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_RationalNumberType(
                        stream,
                        &(*WPT_ChargeParameterDiscoveryResType).PDMinCoilCurrent,
                    );
                    if error == 0 as i32 {
                        grammar_id = 218 as i32;
                    }
                }
            }
            218 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_RationalNumberType(
                        stream,
                        &(*WPT_ChargeParameterDiscoveryResType).PDMaxCoilCurrent,
                    );
                    if error == 0 as i32 {
                        grammar_id = 219 as i32;
                    }
                }
            }
            219 => {
                if (SDManufacturerSpecificDataContainer_currentIndex as i32)
                    < (*WPT_ChargeParameterDiscoveryResType)
                        .SDManufacturerSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_ChargeParameterDiscoveryResType)
                                    .SDManufacturerSpecificDataContainer
                                    .array
                                    [SDManufacturerSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_ChargeParameterDiscoveryResType)
                                        .SDManufacturerSpecificDataContainer
                                        .array
                                        [SDManufacturerSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_ChargeParameterDiscoveryResType)
                                        .SDManufacturerSpecificDataContainer
                                        .array
                                        [SDManufacturerSpecificDataContainer_currentIndex
                                            as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    SDManufacturerSpecificDataContainer_currentIndex =
                                        SDManufacturerSpecificDataContainer_currentIndex
                                            .wrapping_add(1);
                                    SDManufacturerSpecificDataContainer_currentIndex;
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
            220 => {
                if (SDManufacturerSpecificDataContainer_currentIndex as i32)
                    < (*WPT_ChargeParameterDiscoveryResType)
                        .SDManufacturerSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_ChargeParameterDiscoveryResType)
                                    .SDManufacturerSpecificDataContainer
                                    .array
                                    [SDManufacturerSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_ChargeParameterDiscoveryResType)
                                        .SDManufacturerSpecificDataContainer
                                        .array
                                        [SDManufacturerSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_ChargeParameterDiscoveryResType)
                                        .SDManufacturerSpecificDataContainer
                                        .array
                                        [SDManufacturerSpecificDataContainer_currentIndex
                                            as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    SDManufacturerSpecificDataContainer_currentIndex =
                                        SDManufacturerSpecificDataContainer_currentIndex
                                            .wrapping_add(1);
                                    SDManufacturerSpecificDataContainer_currentIndex;
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
unsafe extern "C" fn encode_iso20_wpt_WPT_AlignmentCheckReqType(
    stream: &mut ExiBitstream,
    mut WPT_AlignmentCheckReqType: *const iso20_wpt_WPT_AlignmentCheckReqType,
) -> i32 {
    let mut grammar_id: i32 = 221 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut VendorSpecificDataContainer_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            221 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_MessageHeaderType(
                        stream,
                        &(*WPT_AlignmentCheckReqType).Header,
                    );
                    if error == 0 as i32 {
                        grammar_id = 222 as i32;
                    }
                }
            }
            222 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*WPT_AlignmentCheckReqType).EVProcessing as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 223 as i32;
                            }
                        }
                    }
                }
            }
            223 => {
                if (*WPT_AlignmentCheckReqType).TargetCoilCurrent_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_RationalNumberType(
                            stream,
                            &(*WPT_AlignmentCheckReqType).TargetCoilCurrent,
                        );
                        if error == 0 as i32 {
                            grammar_id = 224 as i32;
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
                                2 as i32 as usize,
                                (*WPT_AlignmentCheckReqType).EVResultCode as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 225 as i32;
                                }
                            }
                        }
                    }
                }
            }
            224 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*WPT_AlignmentCheckReqType).EVResultCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 225 as i32;
                            }
                        }
                    }
                }
            }
            225 => {
                if (VendorSpecificDataContainer_currentIndex as i32)
                    < (*WPT_AlignmentCheckReqType)
                        .VendorSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_AlignmentCheckReqType)
                                    .VendorSpecificDataContainer
                                    .array
                                    [VendorSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_AlignmentCheckReqType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_AlignmentCheckReqType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    VendorSpecificDataContainer_currentIndex =
                                        VendorSpecificDataContainer_currentIndex.wrapping_add(1);
                                    VendorSpecificDataContainer_currentIndex;
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
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            226 => {
                if (VendorSpecificDataContainer_currentIndex as i32)
                    < (*WPT_AlignmentCheckReqType)
                        .VendorSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_AlignmentCheckReqType)
                                    .VendorSpecificDataContainer
                                    .array
                                    [VendorSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_AlignmentCheckReqType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_AlignmentCheckReqType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    VendorSpecificDataContainer_currentIndex =
                                        VendorSpecificDataContainer_currentIndex.wrapping_add(1);
                                    VendorSpecificDataContainer_currentIndex;
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
unsafe extern "C" fn encode_iso20_wpt_WPT_AlignmentCheckResType(
    stream: &mut ExiBitstream,
    mut WPT_AlignmentCheckResType: *const iso20_wpt_WPT_AlignmentCheckResType,
) -> i32 {
    let mut grammar_id: i32 = 227 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut VendorSpecificDataContainer_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            227 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_MessageHeaderType(
                        stream,
                        &(*WPT_AlignmentCheckResType).Header,
                    );
                    if error == 0 as i32 {
                        grammar_id = 228 as i32;
                    }
                }
            }
            228 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            6 as i32 as usize,
                            (*WPT_AlignmentCheckResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 229 as i32;
                            }
                        }
                    }
                }
            }
            229 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*WPT_AlignmentCheckResType).EVSEProcessing as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 230 as i32;
                            }
                        }
                    }
                }
            }
            230 => {
                if (*WPT_AlignmentCheckResType).PowerTransmitted_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_RationalNumberType(
                            stream,
                            &(*WPT_AlignmentCheckResType).PowerTransmitted,
                        );
                        if error == 0 as i32 {
                            grammar_id = 232 as i32;
                        }
                    }
                } else if (*WPT_AlignmentCheckResType).SupplyDeviceCurrent_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_RationalNumberType(
                            stream,
                            &(*WPT_AlignmentCheckResType).SupplyDeviceCurrent,
                        );
                        if error == 0 as i32 {
                            grammar_id = 234 as i32;
                        }
                    }
                } else if (VendorSpecificDataContainer_currentIndex as i32)
                    < (*WPT_AlignmentCheckResType)
                        .VendorSpecificDataContainer
                        .arrayLen as i32
                {
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
                                (*WPT_AlignmentCheckResType)
                                    .VendorSpecificDataContainer
                                    .array
                                    [VendorSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    VendorSpecificDataContainer_currentIndex =
                                        VendorSpecificDataContainer_currentIndex.wrapping_add(1);
                                    VendorSpecificDataContainer_currentIndex;
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
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            231 => {
                if (VendorSpecificDataContainer_currentIndex as i32)
                    < (*WPT_AlignmentCheckResType)
                        .VendorSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_AlignmentCheckResType)
                                    .VendorSpecificDataContainer
                                    .array
                                    [VendorSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    VendorSpecificDataContainer_currentIndex =
                                        VendorSpecificDataContainer_currentIndex.wrapping_add(1);
                                    VendorSpecificDataContainer_currentIndex;
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
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            232 => {
                if (*WPT_AlignmentCheckResType).SupplyDeviceCurrent_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_RationalNumberType(
                            stream,
                            &(*WPT_AlignmentCheckResType).SupplyDeviceCurrent,
                        );
                        if error == 0 as i32 {
                            grammar_id = 234 as i32;
                        }
                    }
                } else if (VendorSpecificDataContainer_currentIndex as i32)
                    < (*WPT_AlignmentCheckResType)
                        .VendorSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_AlignmentCheckResType)
                                    .VendorSpecificDataContainer
                                    .array
                                    [VendorSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    VendorSpecificDataContainer_currentIndex =
                                        VendorSpecificDataContainer_currentIndex.wrapping_add(1);
                                    VendorSpecificDataContainer_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 233 as i32;
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
            233 => {
                if (VendorSpecificDataContainer_currentIndex as i32)
                    < (*WPT_AlignmentCheckResType)
                        .VendorSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_AlignmentCheckResType)
                                    .VendorSpecificDataContainer
                                    .array
                                    [VendorSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    VendorSpecificDataContainer_currentIndex =
                                        VendorSpecificDataContainer_currentIndex.wrapping_add(1);
                                    VendorSpecificDataContainer_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 233 as i32;
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
            234 => {
                if (VendorSpecificDataContainer_currentIndex as i32)
                    < (*WPT_AlignmentCheckResType)
                        .VendorSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_AlignmentCheckResType)
                                    .VendorSpecificDataContainer
                                    .array
                                    [VendorSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    VendorSpecificDataContainer_currentIndex =
                                        VendorSpecificDataContainer_currentIndex.wrapping_add(1);
                                    VendorSpecificDataContainer_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 235 as i32;
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
            235 => {
                if (VendorSpecificDataContainer_currentIndex as i32)
                    < (*WPT_AlignmentCheckResType)
                        .VendorSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_AlignmentCheckResType)
                                    .VendorSpecificDataContainer
                                    .array
                                    [VendorSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .array
                                        [VendorSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    VendorSpecificDataContainer_currentIndex =
                                        VendorSpecificDataContainer_currentIndex.wrapping_add(1);
                                    VendorSpecificDataContainer_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 235 as i32;
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
unsafe extern "C" fn encode_iso20_wpt_WPT_ChargeLoopReqType(
    stream: &mut ExiBitstream,
    mut WPT_ChargeLoopReqType: *const iso20_wpt_WPT_ChargeLoopReqType,
) -> i32 {
    let mut grammar_id: i32 = 236 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut ManufacturerSpecificDataContainer_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            236 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_MessageHeaderType(
                        stream,
                        &(*WPT_ChargeLoopReqType).Header,
                    );
                    if error == 0 as i32 {
                        grammar_id = 237 as i32;
                    }
                }
            }
            237 => {
                if (*WPT_ChargeLoopReqType).DisplayParameters_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_DisplayParametersType(
                            stream,
                            &(*WPT_ChargeLoopReqType).DisplayParameters,
                        );
                        if error == 0 as i32 {
                            grammar_id = 238 as i32;
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
                                (*WPT_ChargeLoopReqType).MeterInfoRequested,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 239 as i32;
                                }
                            }
                        }
                    }
                }
            }
            238 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(
                            stream,
                            (*WPT_ChargeLoopReqType).MeterInfoRequested,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 239 as i32;
                            }
                        }
                    }
                }
            }
            239 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_RationalNumberType(
                        stream,
                        &(*WPT_ChargeLoopReqType).EVPCPowerRequest,
                    );
                    if error == 0 as i32 {
                        grammar_id = 240 as i32;
                    }
                }
            }
            240 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_RationalNumberType(
                        stream,
                        &(*WPT_ChargeLoopReqType).EVPCPowerOutput,
                    );
                    if error == 0 as i32 {
                        grammar_id = 241 as i32;
                    }
                }
            }
            241 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*WPT_ChargeLoopReqType).EVPCChargeDiagnostics as u32,
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
            }
            242 => {
                if (*WPT_ChargeLoopReqType).EVPCOperatingFrequency_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_RationalNumberType(
                            stream,
                            &(*WPT_ChargeLoopReqType).EVPCOperatingFrequency,
                        );
                        if error == 0 as i32 {
                            grammar_id = 244 as i32;
                        }
                    }
                } else if (*WPT_ChargeLoopReqType).EVPCPowerControlParameter_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_WPT_EVPCPowerControlParameterType(
                            stream,
                            &(*WPT_ChargeLoopReqType).EVPCPowerControlParameter,
                        );
                        if error == 0 as i32 {
                            grammar_id = 246 as i32;
                        }
                    }
                } else if (ManufacturerSpecificDataContainer_currentIndex as i32)
                    < (*WPT_ChargeLoopReqType)
                        .ManufacturerSpecificDataContainer
                        .arrayLen as i32
                {
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
                                (*WPT_ChargeLoopReqType)
                                    .ManufacturerSpecificDataContainer
                                    .array
                                    [ManufacturerSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .array
                                        [ManufacturerSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .array
                                        [ManufacturerSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    ManufacturerSpecificDataContainer_currentIndex =
                                        ManufacturerSpecificDataContainer_currentIndex
                                            .wrapping_add(1);
                                    ManufacturerSpecificDataContainer_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 243 as i32;
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
            243 => {
                if (ManufacturerSpecificDataContainer_currentIndex as i32)
                    < (*WPT_ChargeLoopReqType)
                        .ManufacturerSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_ChargeLoopReqType)
                                    .ManufacturerSpecificDataContainer
                                    .array
                                    [ManufacturerSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .array
                                        [ManufacturerSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .array
                                        [ManufacturerSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    ManufacturerSpecificDataContainer_currentIndex =
                                        ManufacturerSpecificDataContainer_currentIndex
                                            .wrapping_add(1);
                                    ManufacturerSpecificDataContainer_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 243 as i32;
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
            244 => {
                if (*WPT_ChargeLoopReqType).EVPCPowerControlParameter_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_WPT_EVPCPowerControlParameterType(
                            stream,
                            &(*WPT_ChargeLoopReqType).EVPCPowerControlParameter,
                        );
                        if error == 0 as i32 {
                            grammar_id = 246 as i32;
                        }
                    }
                } else if (ManufacturerSpecificDataContainer_currentIndex as i32)
                    < (*WPT_ChargeLoopReqType)
                        .ManufacturerSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_ChargeLoopReqType)
                                    .ManufacturerSpecificDataContainer
                                    .array
                                    [ManufacturerSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .array
                                        [ManufacturerSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .array
                                        [ManufacturerSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    ManufacturerSpecificDataContainer_currentIndex =
                                        ManufacturerSpecificDataContainer_currentIndex
                                            .wrapping_add(1);
                                    ManufacturerSpecificDataContainer_currentIndex;
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
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            245 => {
                if (ManufacturerSpecificDataContainer_currentIndex as i32)
                    < (*WPT_ChargeLoopReqType)
                        .ManufacturerSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_ChargeLoopReqType)
                                    .ManufacturerSpecificDataContainer
                                    .array
                                    [ManufacturerSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .array
                                        [ManufacturerSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .array
                                        [ManufacturerSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    ManufacturerSpecificDataContainer_currentIndex =
                                        ManufacturerSpecificDataContainer_currentIndex
                                            .wrapping_add(1);
                                    ManufacturerSpecificDataContainer_currentIndex;
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
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 3 as i32;
                    }
                }
            }
            246 => {
                if (ManufacturerSpecificDataContainer_currentIndex as i32)
                    < (*WPT_ChargeLoopReqType)
                        .ManufacturerSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_ChargeLoopReqType)
                                    .ManufacturerSpecificDataContainer
                                    .array
                                    [ManufacturerSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .array
                                        [ManufacturerSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .array
                                        [ManufacturerSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    ManufacturerSpecificDataContainer_currentIndex =
                                        ManufacturerSpecificDataContainer_currentIndex
                                            .wrapping_add(1);
                                    ManufacturerSpecificDataContainer_currentIndex;
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
            247 => {
                if (ManufacturerSpecificDataContainer_currentIndex as i32)
                    < (*WPT_ChargeLoopReqType)
                        .ManufacturerSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_ChargeLoopReqType)
                                    .ManufacturerSpecificDataContainer
                                    .array
                                    [ManufacturerSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .array
                                        [ManufacturerSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .array
                                        [ManufacturerSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    ManufacturerSpecificDataContainer_currentIndex =
                                        ManufacturerSpecificDataContainer_currentIndex
                                            .wrapping_add(1);
                                    ManufacturerSpecificDataContainer_currentIndex;
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
unsafe extern "C" fn encode_iso20_wpt_WPT_ChargeLoopResType(
    stream: &mut ExiBitstream,
    mut WPT_ChargeLoopResType: *const iso20_wpt_WPT_ChargeLoopResType,
) -> i32 {
    let mut grammar_id: i32 = 248 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut ManufacturerSpecificDataContainer_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            248 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_MessageHeaderType(
                        stream,
                        &(*WPT_ChargeLoopResType).Header,
                    );
                    if error == 0 as i32 {
                        grammar_id = 249 as i32;
                    }
                }
            }
            249 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            6 as i32 as usize,
                            (*WPT_ChargeLoopResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 250 as i32;
                            }
                        }
                    }
                }
            }
            250 => {
                if (*WPT_ChargeLoopResType).EVSEStatus_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_EVSEStatusType(
                            stream,
                            &(*WPT_ChargeLoopResType).EVSEStatus,
                        );
                        if error == 0 as i32 {
                            grammar_id = 251 as i32;
                        }
                    }
                } else if (*WPT_ChargeLoopResType).MeterInfo_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_MeterInfoType(
                            stream,
                            &(*WPT_ChargeLoopResType).MeterInfo,
                        );
                        if error == 0 as i32 {
                            grammar_id = 252 as i32;
                        }
                    }
                } else if (*WPT_ChargeLoopResType).Receipt_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_wpt_ReceiptType(stream, &(*WPT_ChargeLoopResType).Receipt);
                        if error == 0 as i32 {
                            grammar_id = 253 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_RationalNumberType(
                            stream,
                            &(*WPT_ChargeLoopResType).EVPCPowerRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 254 as i32;
                        }
                    }
                }
            }
            251 => {
                if (*WPT_ChargeLoopResType).MeterInfo_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_MeterInfoType(
                            stream,
                            &(*WPT_ChargeLoopResType).MeterInfo,
                        );
                        if error == 0 as i32 {
                            grammar_id = 252 as i32;
                        }
                    }
                } else if (*WPT_ChargeLoopResType).Receipt_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_wpt_ReceiptType(stream, &(*WPT_ChargeLoopResType).Receipt);
                        if error == 0 as i32 {
                            grammar_id = 253 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_RationalNumberType(
                            stream,
                            &(*WPT_ChargeLoopResType).EVPCPowerRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 254 as i32;
                        }
                    }
                }
            }
            252 => {
                if (*WPT_ChargeLoopResType).Receipt_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso20_wpt_ReceiptType(stream, &(*WPT_ChargeLoopResType).Receipt);
                        if error == 0 as i32 {
                            grammar_id = 253 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_RationalNumberType(
                            stream,
                            &(*WPT_ChargeLoopResType).EVPCPowerRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 254 as i32;
                        }
                    }
                }
            }
            253 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_RationalNumberType(
                        stream,
                        &(*WPT_ChargeLoopResType).EVPCPowerRequest,
                    );
                    if error == 0 as i32 {
                        grammar_id = 254 as i32;
                    }
                }
            }
            254 => {
                if (*WPT_ChargeLoopResType).SDPowerInput_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_RationalNumberType(
                            stream,
                            &(*WPT_ChargeLoopResType).SDPowerInput,
                        );
                        if error == 0 as i32 {
                            grammar_id = 255 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_RationalNumberType(
                            stream,
                            &(*WPT_ChargeLoopResType).SPCMaxOutputPowerLimit,
                        );
                        if error == 0 as i32 {
                            grammar_id = 256 as i32;
                        }
                    }
                }
            }
            255 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_RationalNumberType(
                        stream,
                        &(*WPT_ChargeLoopResType).SPCMaxOutputPowerLimit,
                    );
                    if error == 0 as i32 {
                        grammar_id = 256 as i32;
                    }
                }
            }
            256 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_RationalNumberType(
                        stream,
                        &(*WPT_ChargeLoopResType).SPCMinOutputPowerLimit,
                    );
                    if error == 0 as i32 {
                        grammar_id = 257 as i32;
                    }
                }
            }
            257 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            3 as i32 as usize,
                            (*WPT_ChargeLoopResType).SPCChargeDiagnostics as u32,
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
            258 => {
                if (*WPT_ChargeLoopResType).SPCOperatingFrequency_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_RationalNumberType(
                            stream,
                            &(*WPT_ChargeLoopResType).SPCOperatingFrequency,
                        );
                        if error == 0 as i32 {
                            grammar_id = 260 as i32;
                        }
                    }
                } else if (*WPT_ChargeLoopResType).SPCPowerControlParameter_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_WPT_SPCPowerControlParameterType(
                            stream,
                            &(*WPT_ChargeLoopResType).SPCPowerControlParameter,
                        );
                        if error == 0 as i32 {
                            grammar_id = 262 as i32;
                        }
                    }
                } else if (ManufacturerSpecificDataContainer_currentIndex as i32)
                    < (*WPT_ChargeLoopResType)
                        .ManufacturerSpecificDataContainer
                        .arrayLen as i32
                {
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
                                (*WPT_ChargeLoopResType)
                                    .ManufacturerSpecificDataContainer
                                    .array
                                    [ManufacturerSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .array
                                        [ManufacturerSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .array
                                        [ManufacturerSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    ManufacturerSpecificDataContainer_currentIndex =
                                        ManufacturerSpecificDataContainer_currentIndex
                                            .wrapping_add(1);
                                    ManufacturerSpecificDataContainer_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 259 as i32;
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
            259 => {
                if (ManufacturerSpecificDataContainer_currentIndex as i32)
                    < (*WPT_ChargeLoopResType)
                        .ManufacturerSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_ChargeLoopResType)
                                    .ManufacturerSpecificDataContainer
                                    .array
                                    [ManufacturerSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .array
                                        [ManufacturerSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .array
                                        [ManufacturerSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    ManufacturerSpecificDataContainer_currentIndex =
                                        ManufacturerSpecificDataContainer_currentIndex
                                            .wrapping_add(1);
                                    ManufacturerSpecificDataContainer_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 259 as i32;
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
            260 => {
                if (*WPT_ChargeLoopResType).SPCPowerControlParameter_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_WPT_SPCPowerControlParameterType(
                            stream,
                            &(*WPT_ChargeLoopResType).SPCPowerControlParameter,
                        );
                        if error == 0 as i32 {
                            grammar_id = 262 as i32;
                        }
                    }
                } else if (ManufacturerSpecificDataContainer_currentIndex as i32)
                    < (*WPT_ChargeLoopResType)
                        .ManufacturerSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_ChargeLoopResType)
                                    .ManufacturerSpecificDataContainer
                                    .array
                                    [ManufacturerSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .array
                                        [ManufacturerSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .array
                                        [ManufacturerSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    ManufacturerSpecificDataContainer_currentIndex =
                                        ManufacturerSpecificDataContainer_currentIndex
                                            .wrapping_add(1);
                                    ManufacturerSpecificDataContainer_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 261 as i32;
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
            261 => {
                if (ManufacturerSpecificDataContainer_currentIndex as i32)
                    < (*WPT_ChargeLoopResType)
                        .ManufacturerSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_ChargeLoopResType)
                                    .ManufacturerSpecificDataContainer
                                    .array
                                    [ManufacturerSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .array
                                        [ManufacturerSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .array
                                        [ManufacturerSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    ManufacturerSpecificDataContainer_currentIndex =
                                        ManufacturerSpecificDataContainer_currentIndex
                                            .wrapping_add(1);
                                    ManufacturerSpecificDataContainer_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 261 as i32;
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
            262 => {
                if (ManufacturerSpecificDataContainer_currentIndex as i32)
                    < (*WPT_ChargeLoopResType)
                        .ManufacturerSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_ChargeLoopResType)
                                    .ManufacturerSpecificDataContainer
                                    .array
                                    [ManufacturerSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .array
                                        [ManufacturerSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .array
                                        [ManufacturerSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    ManufacturerSpecificDataContainer_currentIndex =
                                        ManufacturerSpecificDataContainer_currentIndex
                                            .wrapping_add(1);
                                    ManufacturerSpecificDataContainer_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 263 as i32;
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
            263 => {
                if (ManufacturerSpecificDataContainer_currentIndex as i32)
                    < (*WPT_ChargeLoopResType)
                        .ManufacturerSpecificDataContainer
                        .arrayLen as i32
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
                                (*WPT_ChargeLoopResType)
                                    .ManufacturerSpecificDataContainer
                                    .array
                                    [ManufacturerSpecificDataContainer_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .array
                                        [ManufacturerSpecificDataContainer_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .array
                                        [ManufacturerSpecificDataContainer_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    ManufacturerSpecificDataContainer_currentIndex =
                                        ManufacturerSpecificDataContainer_currentIndex
                                            .wrapping_add(1);
                                    ManufacturerSpecificDataContainer_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 263 as i32;
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
unsafe extern "C" fn encode_iso20_wpt_CLReqControlModeType(
    stream: &mut ExiBitstream,
    mut CLReqControlModeType: *const iso20_wpt_CLReqControlModeType,
) -> i32 {
    let mut error: i32 =
        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
    return error;
}
unsafe extern "C" fn encode_iso20_wpt_CLResControlModeType(
    stream: &mut ExiBitstream,
    mut CLResControlModeType: *const iso20_wpt_CLResControlModeType,
) -> i32 {
    let mut error: i32 =
        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
    return error;
}
unsafe extern "C" fn encode_iso20_wpt_ManifestType(
    stream: &mut ExiBitstream,
    mut ManifestType: *const iso20_wpt_ManifestType,
) -> i32 {
    let mut grammar_id: i32 = 264 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut Reference_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            264 => {
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
                                grammar_id = 266 as i32;
                            }
                        }
                    }
                } else if (Reference_currentIndex as i32)
                    < (*ManifestType).Reference.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh26 = Reference_currentIndex;
                        Reference_currentIndex = Reference_currentIndex.wrapping_add(1);
                        error = encode_iso20_wpt_ReferenceType(
                            stream,
                            &*((*ManifestType).Reference.array)
                                .as_ptr()
                                .offset(fresh26 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 265 as i32;
                        }
                    }
                }
            }
            265 => {
                if (Reference_currentIndex as i32) < (*ManifestType).Reference.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh27 = Reference_currentIndex;
                        Reference_currentIndex = Reference_currentIndex.wrapping_add(1);
                        error = encode_iso20_wpt_ReferenceType(
                            stream,
                            &*((*ManifestType).Reference.array)
                                .as_ptr()
                                .offset(fresh27 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 265 as i32;
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
            266 => {
                if (Reference_currentIndex as i32) < (*ManifestType).Reference.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh28 = Reference_currentIndex;
                        Reference_currentIndex = Reference_currentIndex.wrapping_add(1);
                        error = encode_iso20_wpt_ReferenceType(
                            stream,
                            &*((*ManifestType).Reference.array)
                                .as_ptr()
                                .offset(fresh28 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 267 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            267 => {
                if (Reference_currentIndex as i32) < (*ManifestType).Reference.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh29 = Reference_currentIndex;
                        Reference_currentIndex = Reference_currentIndex.wrapping_add(1);
                        error = encode_iso20_wpt_ReferenceType(
                            stream,
                            &*((*ManifestType).Reference.array)
                                .as_ptr()
                                .offset(fresh29 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 267 as i32;
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
unsafe extern "C" fn encode_iso20_wpt_SignaturePropertiesType(
    stream: &mut ExiBitstream,
    mut SignaturePropertiesType: *const iso20_wpt_SignaturePropertiesType,
) -> i32 {
    let mut grammar_id: i32 = 268 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            268 => {
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
                                grammar_id = 270 as i32;
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_SignaturePropertyType(
                            stream,
                            &(*SignaturePropertiesType).SignatureProperty,
                        );
                        if error == 0 as i32 {
                            grammar_id = 269 as i32;
                        }
                    }
                }
            }
            269 => {
                if 1 as i32 == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_SignaturePropertyType(
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
            270 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso20_wpt_SignaturePropertyType(
                        stream,
                        &(*SignaturePropertiesType).SignatureProperty,
                    );
                    if error == 0 as i32 {
                        grammar_id = 271 as i32;
                    }
                }
            }
            271 => {
                if 1 as i32 == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso20_wpt_SignaturePropertyType(
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

pub unsafe extern "C" fn encode_iso20_wpt_exiDocument(
    stream: &mut ExiBitstream,
    mut exiDoc: *mut iso20_wpt_exiDocument,
) -> i32 {
    let mut error: i32 = exi_header_write(stream);
    if error == 0 as i32 {
        if (*exiDoc).CLReqControlMode_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 0 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_CLReqControlModeType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.CLReqControlMode,
                );
            }
        } else if (*exiDoc).CLResControlMode_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 1 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_CLResControlModeType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.CLResControlMode,
                );
            }
        } else if (*exiDoc).CanonicalizationMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 2 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_CanonicalizationMethodType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.CanonicalizationMethod,
                );
            }
        } else if (*exiDoc).DSAKeyValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 3 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_DSAKeyValueType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.DSAKeyValue,
                );
            }
        } else if (*exiDoc).DigestMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 4 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_DigestMethodType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.DigestMethod,
                );
            }
        } else if (*exiDoc).KeyInfo_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 6 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_KeyInfoType(stream, &mut (*exiDoc).c2rust_unnamed.KeyInfo);
            }
        } else if (*exiDoc).KeyValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 8 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_wpt_KeyValueType(stream, &mut (*exiDoc).c2rust_unnamed.KeyValue);
            }
        } else if (*exiDoc).Manifest_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 9 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_wpt_ManifestType(stream, &mut (*exiDoc).c2rust_unnamed.Manifest);
            }
        } else if (*exiDoc).Object_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 11 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_ObjectType(stream, &mut (*exiDoc).c2rust_unnamed.Object);
            }
        } else if (*exiDoc).PGPData_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 12 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_PGPDataType(stream, &mut (*exiDoc).c2rust_unnamed.PGPData);
            }
        } else if (*exiDoc).RSAKeyValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 13 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_RSAKeyValueType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.RSAKeyValue,
                );
            }
        } else if (*exiDoc).Reference_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 14 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_wpt_ReferenceType(stream, &mut (*exiDoc).c2rust_unnamed.Reference);
            }
        } else if (*exiDoc).RetrievalMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 15 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_RetrievalMethodType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.RetrievalMethod,
                );
            }
        } else if (*exiDoc).SPKIData_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 16 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_wpt_SPKIDataType(stream, &mut (*exiDoc).c2rust_unnamed.SPKIData);
            }
        } else if (*exiDoc).SignatureMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 17 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_SignatureMethodType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.SignatureMethod,
                );
            }
        } else if (*exiDoc).SignatureProperties_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 18 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_SignaturePropertiesType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.SignatureProperties,
                );
            }
        } else if (*exiDoc).SignatureProperty_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 19 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_SignaturePropertyType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.SignatureProperty,
                );
            }
        } else if (*exiDoc).Signature_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 20 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_wpt_SignatureType(stream, &mut (*exiDoc).c2rust_unnamed.Signature);
            }
        } else if (*exiDoc).SignatureValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 21 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_SignatureValueType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.SignatureValue,
                );
            }
        } else if (*exiDoc).SignedInfo_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 22 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_SignedInfoType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.SignedInfo,
                );
            }
        } else if (*exiDoc).Transform_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 23 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_wpt_TransformType(stream, &mut (*exiDoc).c2rust_unnamed.Transform);
            }
        } else if (*exiDoc).Transforms_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 24 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_TransformsType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.Transforms,
                );
            }
        } else if (*exiDoc).WPT_AlignmentCheckReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 25 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_WPT_AlignmentCheckReqType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.WPT_AlignmentCheckReq,
                );
            }
        } else if (*exiDoc).WPT_AlignmentCheckRes_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 26 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_WPT_AlignmentCheckResType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.WPT_AlignmentCheckRes,
                );
            }
        } else if (*exiDoc).WPT_ChargeLoopReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 27 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_WPT_ChargeLoopReqType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.WPT_ChargeLoopReq,
                );
            }
        } else if (*exiDoc).WPT_ChargeLoopRes_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 28 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_WPT_ChargeLoopResType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.WPT_ChargeLoopRes,
                );
            }
        } else if (*exiDoc).WPT_ChargeParameterDiscoveryReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 29 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_WPT_ChargeParameterDiscoveryReqType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.WPT_ChargeParameterDiscoveryReq,
                );
            }
        } else if (*exiDoc).WPT_ChargeParameterDiscoveryRes_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 30 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_WPT_ChargeParameterDiscoveryResType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.WPT_ChargeParameterDiscoveryRes,
                );
            }
        } else if (*exiDoc).WPT_FinePositioningReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 31 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_WPT_FinePositioningReqType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.WPT_FinePositioningReq,
                );
            }
        } else if (*exiDoc).WPT_FinePositioningRes_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 32 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_WPT_FinePositioningResType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.WPT_FinePositioningRes,
                );
            }
        } else if (*exiDoc).WPT_FinePositioningSetupReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 33 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_WPT_FinePositioningSetupReqType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.WPT_FinePositioningSetupReq,
                );
            }
        } else if (*exiDoc).WPT_FinePositioningSetupRes_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 34 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_WPT_FinePositioningSetupResType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.WPT_FinePositioningSetupRes,
                );
            }
        } else if (*exiDoc).WPT_PairingReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 35 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_WPT_PairingReqType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.WPT_PairingReq,
                );
            }
        } else if (*exiDoc).WPT_PairingRes_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 36 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso20_wpt_WPT_PairingResType(
                    stream,
                    &mut (*exiDoc).c2rust_unnamed.WPT_PairingRes,
                );
            }
        } else if (*exiDoc).X509Data_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 37 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso20_wpt_X509DataType(stream, &mut (*exiDoc).c2rust_unnamed.X509Data);
            }
        } else {
            error = -(70 as i32);
        }
    }
    return error;
}
