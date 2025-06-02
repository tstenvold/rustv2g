use ::c2rust_bitfields;
use c2rust_bitfields::*;
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
    pub _unused: i32,
}
#[derive(Copy, Clone)]

pub struct iso20_wpt_CLResControlModeType {
    pub _unused: i32,
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

pub unsafe extern "C" fn init_iso20_wpt_exiDocument(mut exiDoc: *mut iso20_wpt_exiDocument) {
    (*exiDoc).set_WPT_FinePositioningSetupReq_isUsed(0 as u32);
    (*exiDoc).set_WPT_FinePositioningSetupRes_isUsed(0 as u32);
    (*exiDoc).set_WPT_FinePositioningReq_isUsed(0 as u32);
    (*exiDoc).set_WPT_FinePositioningRes_isUsed(0 as u32);
    (*exiDoc).set_WPT_PairingReq_isUsed(0 as u32);
    (*exiDoc).set_WPT_PairingRes_isUsed(0 as u32);
    (*exiDoc).set_WPT_ChargeParameterDiscoveryReq_isUsed(0 as u32);
    (*exiDoc).set_WPT_ChargeParameterDiscoveryRes_isUsed(0 as u32);
    (*exiDoc).set_WPT_AlignmentCheckReq_isUsed(0 as u32);
    (*exiDoc).set_WPT_AlignmentCheckRes_isUsed(0 as u32);
    (*exiDoc).set_WPT_ChargeLoopReq_isUsed(0 as u32);
    (*exiDoc).set_WPT_ChargeLoopRes_isUsed(0 as u32);
    (*exiDoc).set_CLReqControlMode_isUsed(0 as u32);
    (*exiDoc).set_CLResControlMode_isUsed(0 as u32);
    (*exiDoc).set_Signature_isUsed(0 as u32);
    (*exiDoc).set_SignatureValue_isUsed(0 as u32);
    (*exiDoc).set_SignedInfo_isUsed(0 as u32);
    (*exiDoc).set_CanonicalizationMethod_isUsed(0 as u32);
    (*exiDoc).set_SignatureMethod_isUsed(0 as u32);
    (*exiDoc).set_Reference_isUsed(0 as u32);
    (*exiDoc).set_Transforms_isUsed(0 as u32);
    (*exiDoc).set_Transform_isUsed(0 as u32);
    (*exiDoc).set_DigestMethod_isUsed(0 as u32);
    (*exiDoc).set_KeyInfo_isUsed(0 as u32);
    (*exiDoc).set_KeyValue_isUsed(0 as u32);
    (*exiDoc).set_RetrievalMethod_isUsed(0 as u32);
    (*exiDoc).set_X509Data_isUsed(0 as u32);
    (*exiDoc).set_PGPData_isUsed(0 as u32);
    (*exiDoc).set_SPKIData_isUsed(0 as u32);
    (*exiDoc).set_Object_isUsed(0 as u32);
    (*exiDoc).set_Manifest_isUsed(0 as u32);
    (*exiDoc).set_SignatureProperties_isUsed(0 as u32);
    (*exiDoc).set_SignatureProperty_isUsed(0 as u32);
    (*exiDoc).set_DSAKeyValue_isUsed(0 as u32);
    (*exiDoc).set_RSAKeyValue_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_TransformType(
    mut TransformType: *mut iso20_wpt_TransformType,
) {
    (*TransformType).set_ANY_isUsed(0 as u32);
    (*TransformType).set_XPath_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_LF_RxRSSIType(
    mut WPT_LF_RxRSSIType: *mut iso20_wpt_WPT_LF_RxRSSIType,
) {
}

pub unsafe extern "C" fn init_iso20_wpt_TransformsType(
    mut TransformsType: *mut iso20_wpt_TransformsType,
) {
}

pub unsafe extern "C" fn init_iso20_wpt_DSAKeyValueType(
    mut DSAKeyValueType: *mut iso20_wpt_DSAKeyValueType,
) {
    (*DSAKeyValueType).set_P_isUsed(0 as u32);
    (*DSAKeyValueType).set_Q_isUsed(0 as u32);
    (*DSAKeyValueType).set_G_isUsed(0 as u32);
    (*DSAKeyValueType).set_J_isUsed(0 as u32);
    (*DSAKeyValueType).set_Seed_isUsed(0 as u32);
    (*DSAKeyValueType).set_PgenCounter_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_X509IssuerSerialType(
    mut X509IssuerSerialType: *mut iso20_wpt_X509IssuerSerialType,
) {
}

pub unsafe extern "C" fn init_iso20_wpt_DigestMethodType(
    mut DigestMethodType: *mut iso20_wpt_DigestMethodType,
) {
    (*DigestMethodType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_RSAKeyValueType(
    mut RSAKeyValueType: *mut iso20_wpt_RSAKeyValueType,
) {
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_LF_RxRSSIListType(
    mut WPT_LF_RxRSSIListType: *mut iso20_wpt_WPT_LF_RxRSSIListType,
) {
}

pub unsafe extern "C" fn init_iso20_wpt_CanonicalizationMethodType(
    mut CanonicalizationMethodType: *mut iso20_wpt_CanonicalizationMethodType,
) {
    (*CanonicalizationMethodType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_TxRxPulseOrderType(
    mut WPT_TxRxPulseOrderType: *mut iso20_wpt_WPT_TxRxPulseOrderType,
) {
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_LF_TxDataType(
    mut WPT_LF_TxDataType: *mut iso20_wpt_WPT_LF_TxDataType,
) {
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_LF_RxDataType(
    mut WPT_LF_RxDataType: *mut iso20_wpt_WPT_LF_RxDataType,
) {
}

pub unsafe extern "C" fn init_iso20_wpt_SignatureMethodType(
    mut SignatureMethodType: *mut iso20_wpt_SignatureMethodType,
) {
    (*SignatureMethodType).set_HMACOutputLength_isUsed(0 as u32);
    (*SignatureMethodType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_KeyValueType(
    mut KeyValueType: *mut iso20_wpt_KeyValueType,
) {
    (*KeyValueType).set_DSAKeyValue_isUsed(0 as u32);
    (*KeyValueType).set_RSAKeyValue_isUsed(0 as u32);
    (*KeyValueType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_CoordinateXYZType(
    mut WPT_CoordinateXYZType: *mut iso20_wpt_WPT_CoordinateXYZType,
) {
}

pub unsafe extern "C" fn init_iso20_wpt_ReferenceType(
    mut ReferenceType: *mut iso20_wpt_ReferenceType,
) {
    (*ReferenceType).set_Id_isUsed(0 as u32);
    (*ReferenceType).set_Type_isUsed(0 as u32);
    (*ReferenceType).set_URI_isUsed(0 as u32);
    (*ReferenceType).set_Transforms_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_RetrievalMethodType(
    mut RetrievalMethodType: *mut iso20_wpt_RetrievalMethodType,
) {
    (*RetrievalMethodType).set_Type_isUsed(0 as u32);
    (*RetrievalMethodType).set_URI_isUsed(0 as u32);
    (*RetrievalMethodType).set_Transforms_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_X509DataType(
    mut X509DataType: *mut iso20_wpt_X509DataType,
) {
    (*X509DataType).set_X509IssuerSerial_isUsed(0 as u32);
    (*X509DataType).set_X509SKI_isUsed(0 as u32);
    (*X509DataType).set_X509SubjectName_isUsed(0 as u32);
    (*X509DataType).set_X509Certificate_isUsed(0 as u32);
    (*X509DataType).set_X509CRL_isUsed(0 as u32);
    (*X509DataType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_PGPDataType(mut PGPDataType: *mut iso20_wpt_PGPDataType) {
    ((*PGPDataType).c2rust_unnamed).choice_1_isUsed = 1 as u32;
    ((*PGPDataType).c2rust_unnamed).choice_2_isUsed = 0 as u32;
}

pub unsafe extern "C" fn init_iso20_wpt_SPKIDataType(
    mut SPKIDataType: *mut iso20_wpt_SPKIDataType,
) {
    (*SPKIDataType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_SignedInfoType(
    mut SignedInfoType: *mut iso20_wpt_SignedInfoType,
) {
    (*SignedInfoType).Reference.arrayLen = 0 as u32 as u16;
    (*SignedInfoType).set_Id_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_SignatureValueType(
    mut SignatureValueType: *mut iso20_wpt_SignatureValueType,
) {
    (*SignatureValueType).set_Id_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_RationalNumberType(
    mut RationalNumberType: *mut iso20_wpt_RationalNumberType,
) {
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_LF_TxDataListType(
    mut WPT_LF_TxDataListType: *mut iso20_wpt_WPT_LF_TxDataListType,
) {
}

pub unsafe extern "C" fn init_iso20_wpt_KeyInfoType(mut KeyInfoType: *mut iso20_wpt_KeyInfoType) {
    (*KeyInfoType).set_Id_isUsed(0 as u32);
    (*KeyInfoType).set_KeyName_isUsed(0 as u32);
    (*KeyInfoType).set_KeyValue_isUsed(0 as u32);
    (*KeyInfoType).set_RetrievalMethod_isUsed(0 as u32);
    (*KeyInfoType).set_X509Data_isUsed(0 as u32);
    (*KeyInfoType).set_PGPData_isUsed(0 as u32);
    (*KeyInfoType).set_SPKIData_isUsed(0 as u32);
    (*KeyInfoType).set_MgmtData_isUsed(0 as u32);
    (*KeyInfoType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_TxRxSpecDataType(
    mut WPT_TxRxSpecDataType: *mut iso20_wpt_WPT_TxRxSpecDataType,
) {
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_LF_RxDataListType(
    mut WPT_LF_RxDataListType: *mut iso20_wpt_WPT_LF_RxDataListType,
) {
}

pub unsafe extern "C" fn init_iso20_wpt_ObjectType(mut ObjectType: *mut iso20_wpt_ObjectType) {
    (*ObjectType).set_Encoding_isUsed(0 as u32);
    (*ObjectType).set_Id_isUsed(0 as u32);
    (*ObjectType).set_MimeType_isUsed(0 as u32);
    (*ObjectType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_TxRxPackageSpecDataType(
    mut WPT_TxRxPackageSpecDataType: *mut iso20_wpt_WPT_TxRxPackageSpecDataType,
) {
    (*WPT_TxRxPackageSpecDataType).PulseSequenceOrder.arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_LF_TransmitterDataType(
    mut WPT_LF_TransmitterDataType: *mut iso20_wpt_WPT_LF_TransmitterDataType,
) {
    (*WPT_LF_TransmitterDataType).TxSpecData.arrayLen = 0 as u32 as u16;
    (*WPT_LF_TransmitterDataType).set_TxPackageSpecData_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_AlternativeSECCType(
    mut AlternativeSECCType: *mut iso20_wpt_AlternativeSECCType,
) {
    (*AlternativeSECCType).set_SSID_isUsed(0 as u32);
    (*AlternativeSECCType).set_BSSID_isUsed(0 as u32);
    (*AlternativeSECCType).set_IPAddress_isUsed(0 as u32);
    (*AlternativeSECCType).set_Port_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_LF_ReceiverDataType(
    mut WPT_LF_ReceiverDataType: *mut iso20_wpt_WPT_LF_ReceiverDataType,
) {
    (*WPT_LF_ReceiverDataType).RxSpecData.arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_LF_DataPackageType(
    mut WPT_LF_DataPackageType: *mut iso20_wpt_WPT_LF_DataPackageType,
) {
    (*WPT_LF_DataPackageType).set_LF_TxData_isUsed(0 as u32);
    (*WPT_LF_DataPackageType).set_LF_RxData_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_DetailedCostType(
    mut DetailedCostType: *mut iso20_wpt_DetailedCostType,
) {
}

pub unsafe extern "C" fn init_iso20_wpt_SignatureType(
    mut SignatureType: *mut iso20_wpt_SignatureType,
) {
    (*SignatureType).set_Id_isUsed(0 as u32);
    (*SignatureType).set_KeyInfo_isUsed(0 as u32);
    (*SignatureType).set_Object_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_DetailedTaxType(
    mut DetailedTaxType: *mut iso20_wpt_DetailedTaxType,
) {
}

pub unsafe extern "C" fn init_iso20_wpt_MessageHeaderType(
    mut MessageHeaderType: *mut iso20_wpt_MessageHeaderType,
) {
    (*MessageHeaderType).set_Signature_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_SignaturePropertyType(
    mut SignaturePropertyType: *mut iso20_wpt_SignaturePropertyType,
) {
    (*SignaturePropertyType).set_Id_isUsed(0 as u32);
    (*SignaturePropertyType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_DisplayParametersType(
    mut DisplayParametersType: *mut iso20_wpt_DisplayParametersType,
) {
    (*DisplayParametersType).set_PresentSOC_isUsed(0 as u32);
    (*DisplayParametersType).set_MinimumSOC_isUsed(0 as u32);
    (*DisplayParametersType).set_TargetSOC_isUsed(0 as u32);
    (*DisplayParametersType).set_MaximumSOC_isUsed(0 as u32);
    (*DisplayParametersType).set_RemainingTimeToMinimumSOC_isUsed(0 as u32);
    (*DisplayParametersType).set_RemainingTimeToTargetSOC_isUsed(0 as u32);
    (*DisplayParametersType).set_RemainingTimeToMaximumSOC_isUsed(0 as u32);
    (*DisplayParametersType).set_ChargingComplete_isUsed(0 as u32);
    (*DisplayParametersType).set_BatteryEnergyCapacity_isUsed(0 as u32);
    (*DisplayParametersType).set_InletHot_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_FinePositioningMethodListType(
    mut WPT_FinePositioningMethodListType: *mut iso20_wpt_WPT_FinePositioningMethodListType,
) {
    (*WPT_FinePositioningMethodListType)
        .WPT_FinePositioningMethod
        .arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_wpt_EVSEStatusType(
    mut EVSEStatusType: *mut iso20_wpt_EVSEStatusType,
) {
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_PairingMethodListType(
    mut WPT_PairingMethodListType: *mut iso20_wpt_WPT_PairingMethodListType,
) {
    (*WPT_PairingMethodListType).WPT_PairingMethod.arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_wpt_MeterInfoType(
    mut MeterInfoType: *mut iso20_wpt_MeterInfoType,
) {
    (*MeterInfoType).set_BPT_DischargedEnergyReadingWh_isUsed(0 as u32);
    (*MeterInfoType).set_CapacitiveEnergyReadingVARh_isUsed(0 as u32);
    (*MeterInfoType).set_BPT_InductiveEnergyReadingVARh_isUsed(0 as u32);
    (*MeterInfoType).set_MeterSignature_isUsed(0 as u32);
    (*MeterInfoType).set_MeterStatus_isUsed(0 as u32);
    (*MeterInfoType).set_MeterTimestamp_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_AlignmentCheckMethodListType(
    mut WPT_AlignmentCheckMethodListType: *mut iso20_wpt_WPT_AlignmentCheckMethodListType,
) {
    (*WPT_AlignmentCheckMethodListType)
        .WPT_AlignmentCheckMethod
        .arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_LF_DataPackageListType(
    mut WPT_LF_DataPackageListType: *mut iso20_wpt_WPT_LF_DataPackageListType,
) {
}

pub unsafe extern "C" fn init_iso20_wpt_AlternativeSECCListType(
    mut AlternativeSECCListType: *mut iso20_wpt_AlternativeSECCListType,
) {
    (*AlternativeSECCListType).AlternativeSECC.arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_wpt_ReceiptType(mut ReceiptType: *mut iso20_wpt_ReceiptType) {
    (*ReceiptType).TaxCosts.arrayLen = 0 as u32 as u16;
    (*ReceiptType).set_EnergyCosts_isUsed(0 as u32);
    (*ReceiptType).set_OccupancyCosts_isUsed(0 as u32);
    (*ReceiptType).set_AdditionalServicesCosts_isUsed(0 as u32);
    (*ReceiptType).set_OverstayCosts_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_LF_SystemSetupDataType(
    mut WPT_LF_SystemSetupDataType: *mut iso20_wpt_WPT_LF_SystemSetupDataType,
) {
    (*WPT_LF_SystemSetupDataType).set_LF_TransmitterSetupData_isUsed(0 as u32);
    (*WPT_LF_SystemSetupDataType).set_LF_ReceiverSetupData_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_EVPCPowerControlParameterType(
    mut WPT_EVPCPowerControlParameterType: *mut iso20_wpt_WPT_EVPCPowerControlParameterType,
) {
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_SPCPowerControlParameterType(
    mut WPT_SPCPowerControlParameterType: *mut iso20_wpt_WPT_SPCPowerControlParameterType,
) {
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_FinePositioningSetupReqType(
    mut WPT_FinePositioningSetupReqType: *mut iso20_wpt_WPT_FinePositioningSetupReqType,
) {
    (*WPT_FinePositioningSetupReqType)
        .VendorSpecificDataContainer
        .arrayLen = 0 as u32 as u16;
    (*WPT_FinePositioningSetupReqType).set_LF_SystemSetupData_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_FinePositioningSetupResType(
    mut WPT_FinePositioningSetupResType: *mut iso20_wpt_WPT_FinePositioningSetupResType,
) {
    (*WPT_FinePositioningSetupResType)
        .VendorSpecificDataContainer
        .arrayLen = 0 as u32 as u16;
    (*WPT_FinePositioningSetupResType).set_LF_SystemSetupData_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_FinePositioningReqType(
    mut WPT_FinePositioningReqType: *mut iso20_wpt_WPT_FinePositioningReqType,
) {
    (*WPT_FinePositioningReqType)
        .VendorSpecificDataContainer
        .arrayLen = 0 as u32 as u16;
    (*WPT_FinePositioningReqType).set_WPT_LF_DataPackageList_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_FinePositioningResType(
    mut WPT_FinePositioningResType: *mut iso20_wpt_WPT_FinePositioningResType,
) {
    (*WPT_FinePositioningResType)
        .VendorSpecificDataContainer
        .arrayLen = 0 as u32 as u16;
    (*WPT_FinePositioningResType).set_WPT_LF_DataPackageList_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_PairingReqType(
    mut WPT_PairingReqType: *mut iso20_wpt_WPT_PairingReqType,
) {
    (*WPT_PairingReqType).VendorSpecificDataContainer.arrayLen = 0 as u32 as u16;
    (*WPT_PairingReqType).set_ObservedIDCode_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_PairingResType(
    mut WPT_PairingResType: *mut iso20_wpt_WPT_PairingResType,
) {
    (*WPT_PairingResType).VendorSpecificDataContainer.arrayLen = 0 as u32 as u16;
    (*WPT_PairingResType).set_ObservedIDCode_isUsed(0 as u32);
    (*WPT_PairingResType).set_AlternativeSECCList_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_ChargeParameterDiscoveryReqType(
    mut WPT_ChargeParameterDiscoveryReqType: *mut iso20_wpt_WPT_ChargeParameterDiscoveryReqType,
) {
    (*WPT_ChargeParameterDiscoveryReqType)
        .VendorSpecificDataContainer
        .arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_ChargeParameterDiscoveryResType(
    mut WPT_ChargeParameterDiscoveryResType: *mut iso20_wpt_WPT_ChargeParameterDiscoveryResType,
) {
    (*WPT_ChargeParameterDiscoveryResType)
        .SDManufacturerSpecificDataContainer
        .arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_AlignmentCheckReqType(
    mut WPT_AlignmentCheckReqType: *mut iso20_wpt_WPT_AlignmentCheckReqType,
) {
    (*WPT_AlignmentCheckReqType)
        .VendorSpecificDataContainer
        .arrayLen = 0 as u32 as u16;
    (*WPT_AlignmentCheckReqType).set_TargetCoilCurrent_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_AlignmentCheckResType(
    mut WPT_AlignmentCheckResType: *mut iso20_wpt_WPT_AlignmentCheckResType,
) {
    (*WPT_AlignmentCheckResType)
        .VendorSpecificDataContainer
        .arrayLen = 0 as u32 as u16;
    (*WPT_AlignmentCheckResType).set_PowerTransmitted_isUsed(0 as u32);
    (*WPT_AlignmentCheckResType).set_SupplyDeviceCurrent_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_ChargeLoopReqType(
    mut WPT_ChargeLoopReqType: *mut iso20_wpt_WPT_ChargeLoopReqType,
) {
    (*WPT_ChargeLoopReqType)
        .ManufacturerSpecificDataContainer
        .arrayLen = 0 as u32 as u16;
    (*WPT_ChargeLoopReqType).set_DisplayParameters_isUsed(0 as u32);
    (*WPT_ChargeLoopReqType).set_EVPCOperatingFrequency_isUsed(0 as u32);
    (*WPT_ChargeLoopReqType).set_EVPCPowerControlParameter_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_WPT_ChargeLoopResType(
    mut WPT_ChargeLoopResType: *mut iso20_wpt_WPT_ChargeLoopResType,
) {
    (*WPT_ChargeLoopResType)
        .ManufacturerSpecificDataContainer
        .arrayLen = 0 as u32 as u16;
    (*WPT_ChargeLoopResType).set_EVSEStatus_isUsed(0 as u32);
    (*WPT_ChargeLoopResType).set_MeterInfo_isUsed(0 as u32);
    (*WPT_ChargeLoopResType).set_Receipt_isUsed(0 as u32);
    (*WPT_ChargeLoopResType).set_SDPowerInput_isUsed(0 as u32);
    (*WPT_ChargeLoopResType).set_SPCOperatingFrequency_isUsed(0 as u32);
    (*WPT_ChargeLoopResType).set_SPCPowerControlParameter_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_CLReqControlModeType(
    mut CLReqControlModeType: *mut iso20_wpt_CLReqControlModeType,
) {
}

pub unsafe extern "C" fn init_iso20_wpt_CLResControlModeType(
    mut CLResControlModeType: *mut iso20_wpt_CLResControlModeType,
) {
}

pub unsafe extern "C" fn init_iso20_wpt_ManifestType(
    mut ManifestType: *mut iso20_wpt_ManifestType,
) {
    (*ManifestType).Reference.arrayLen = 0 as u32 as u16;
    (*ManifestType).set_Id_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_wpt_SignaturePropertiesType(
    mut SignaturePropertiesType: *mut iso20_wpt_SignaturePropertiesType,
) {
    (*SignaturePropertiesType).set_Id_isUsed(0 as u32);
}
