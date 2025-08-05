use ::c2rust_bitfields;
use c2rust_bitfields::*;
pub type i8 = i8;
pub type __u8 = u8;
pub type __int16_t = i16;
pub type __u16 = u16;
pub type __u64 = u64;
pub type int8_t = i8;
pub type i16 = __int16_t;
pub type u8 = __u8;
pub type u16 = __u16;
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
    _unused: i32,
}
#[derive(Copy, Clone)]

pub struct iso20_acdp_CLResControlModeType {
    _unused: i32,
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

pub unsafe extern "C" fn init_iso20_acdp_exiDocument(mut exiDoc: *mut iso20_acdp_exiDocument) {
    (*exiDoc).set_ACDP_VehiclePositioningReq_isUsed(0 as u32);
    (*exiDoc).set_ACDP_VehiclePositioningRes_isUsed(0 as u32);
    (*exiDoc).set_ACDP_ConnectReq_isUsed(0 as u32);
    (*exiDoc).set_ACDP_ConnectRes_isUsed(0 as u32);
    (*exiDoc).set_ACDP_DisconnectReq_isUsed(0 as u32);
    (*exiDoc).set_ACDP_DisconnectRes_isUsed(0 as u32);
    (*exiDoc).set_ACDP_SystemStatusReq_isUsed(0 as u32);
    (*exiDoc).set_ACDP_SystemStatusRes_isUsed(0 as u32);
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

pub unsafe extern "C" fn init_iso20_acdp_TransformType(
    mut TransformType: *mut iso20_acdp_TransformType,
) {
    (*TransformType).set_ANY_isUsed(0 as u32);
    (*TransformType).set_XPath_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_acdp_TransformsType(
    mut TransformsType: *mut iso20_acdp_TransformsType,
) {
}

pub unsafe extern "C" fn init_iso20_acdp_DSAKeyValueType(
    mut DSAKeyValueType: *mut iso20_acdp_DSAKeyValueType,
) {
    (*DSAKeyValueType).set_P_isUsed(0 as u32);
    (*DSAKeyValueType).set_Q_isUsed(0 as u32);
    (*DSAKeyValueType).set_G_isUsed(0 as u32);
    (*DSAKeyValueType).set_J_isUsed(0 as u32);
    (*DSAKeyValueType).set_Seed_isUsed(0 as u32);
    (*DSAKeyValueType).set_PgenCounter_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_acdp_X509IssuerSerialType(
    mut X509IssuerSerialType: *mut iso20_acdp_X509IssuerSerialType,
) {
}

pub unsafe extern "C" fn init_iso20_acdp_DigestMethodType(
    mut DigestMethodType: *mut iso20_acdp_DigestMethodType,
) {
    (*DigestMethodType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_acdp_RSAKeyValueType(
    mut RSAKeyValueType: *mut iso20_acdp_RSAKeyValueType,
) {
}

pub unsafe extern "C" fn init_iso20_acdp_CanonicalizationMethodType(
    mut CanonicalizationMethodType: *mut iso20_acdp_CanonicalizationMethodType,
) {
    (*CanonicalizationMethodType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_acdp_SignatureMethodType(
    mut SignatureMethodType: *mut iso20_acdp_SignatureMethodType,
) {
    (*SignatureMethodType).set_HMACOutputLength_isUsed(0 as u32);
    (*SignatureMethodType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_acdp_KeyValueType(
    mut KeyValueType: *mut iso20_acdp_KeyValueType,
) {
    (*KeyValueType).set_DSAKeyValue_isUsed(0 as u32);
    (*KeyValueType).set_RSAKeyValue_isUsed(0 as u32);
    (*KeyValueType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_acdp_ReferenceType(
    mut ReferenceType: *mut iso20_acdp_ReferenceType,
) {
    (*ReferenceType).set_Id_isUsed(0 as u32);
    (*ReferenceType).set_Type_isUsed(0 as u32);
    (*ReferenceType).set_URI_isUsed(0 as u32);
    (*ReferenceType).set_Transforms_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_acdp_RetrievalMethodType(
    mut RetrievalMethodType: *mut iso20_acdp_RetrievalMethodType,
) {
    (*RetrievalMethodType).set_Type_isUsed(0 as u32);
    (*RetrievalMethodType).set_URI_isUsed(0 as u32);
    (*RetrievalMethodType).set_Transforms_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_acdp_X509DataType(
    mut X509DataType: *mut iso20_acdp_X509DataType,
) {
    (*X509DataType).set_X509IssuerSerial_isUsed(0 as u32);
    (*X509DataType).set_X509SKI_isUsed(0 as u32);
    (*X509DataType).set_X509SubjectName_isUsed(0 as u32);
    (*X509DataType).set_X509Certificate_isUsed(0 as u32);
    (*X509DataType).set_X509CRL_isUsed(0 as u32);
    (*X509DataType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_acdp_PGPDataType(mut PGPDataType: *mut iso20_acdp_PGPDataType) {
    ((*PGPDataType).c2rust_unnamed).choice_1_isUsed = 0 as u32;
    ((*PGPDataType).c2rust_unnamed).choice_2_isUsed = 0 as u32;
}

pub unsafe extern "C" fn init_iso20_acdp_SPKIDataType(
    mut SPKIDataType: *mut iso20_acdp_SPKIDataType,
) {
    (*SPKIDataType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_acdp_SignedInfoType(
    mut SignedInfoType: *mut iso20_acdp_SignedInfoType,
) {
    (*SignedInfoType).Reference.arrayLen = 0 as u32 as u16;
    (*SignedInfoType).set_Id_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_acdp_SignatureValueType(
    mut SignatureValueType: *mut iso20_acdp_SignatureValueType,
) {
    (*SignatureValueType).set_Id_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_acdp_KeyInfoType(mut KeyInfoType: *mut iso20_acdp_KeyInfoType) {
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

pub unsafe extern "C" fn init_iso20_acdp_ObjectType(mut ObjectType: *mut iso20_acdp_ObjectType) {
    (*ObjectType).set_Encoding_isUsed(0 as u32);
    (*ObjectType).set_Id_isUsed(0 as u32);
    (*ObjectType).set_MimeType_isUsed(0 as u32);
    (*ObjectType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_acdp_SignatureType(
    mut SignatureType: *mut iso20_acdp_SignatureType,
) {
    (*SignatureType).set_Id_isUsed(0 as u32);
    (*SignatureType).set_KeyInfo_isUsed(0 as u32);
    (*SignatureType).set_Object_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_acdp_RationalNumberType(
    mut RationalNumberType: *mut iso20_acdp_RationalNumberType,
) {
}

pub unsafe extern "C" fn init_iso20_acdp_MessageHeaderType(
    mut MessageHeaderType: *mut iso20_acdp_MessageHeaderType,
) {
    (*MessageHeaderType).set_Signature_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_acdp_SignaturePropertyType(
    mut SignaturePropertyType: *mut iso20_acdp_SignaturePropertyType,
) {
    (*SignaturePropertyType).set_Id_isUsed(0 as u32);
    (*SignaturePropertyType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_acdp_EVTechnicalStatusType(
    mut EVTechnicalStatusType: *mut iso20_acdp_EVTechnicalStatusType,
) {
    (*EVTechnicalStatusType).set_EVImmobilized_isUsed(0 as u32);
    (*EVTechnicalStatusType).set_EVWLANStrength_isUsed(0 as u32);
    (*EVTechnicalStatusType).set_EVCPStatus_isUsed(0 as u32);
    (*EVTechnicalStatusType).set_EVSOC_isUsed(0 as u32);
    (*EVTechnicalStatusType).set_EVErrorCode_isUsed(0 as u32);
    (*EVTechnicalStatusType).set_EVTimeout_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_acdp_ACDP_VehiclePositioningReqType(
    mut ACDP_VehiclePositioningReqType: *mut iso20_acdp_ACDP_VehiclePositioningReqType,
) {
}

pub unsafe extern "C" fn init_iso20_acdp_ACDP_VehiclePositioningResType(
    mut ACDP_VehiclePositioningResType: *mut iso20_acdp_ACDP_VehiclePositioningResType,
) {
}

pub unsafe extern "C" fn init_iso20_acdp_ACDP_ConnectReqType(
    mut ACDP_ConnectReqType: *mut iso20_acdp_ACDP_ConnectReqType,
) {
}

pub unsafe extern "C" fn init_iso20_acdp_ACDP_ConnectResType(
    mut ACDP_ConnectResType: *mut iso20_acdp_ACDP_ConnectResType,
) {
}

pub unsafe extern "C" fn init_iso20_acdp_ACDP_SystemStatusReqType(
    mut ACDP_SystemStatusReqType: *mut iso20_acdp_ACDP_SystemStatusReqType,
) {
}

pub unsafe extern "C" fn init_iso20_acdp_ACDP_SystemStatusResType(
    mut ACDP_SystemStatusResType: *mut iso20_acdp_ACDP_SystemStatusResType,
) {
}

pub unsafe extern "C" fn init_iso20_acdp_CLReqControlModeType(
    mut CLReqControlModeType: *mut iso20_acdp_CLReqControlModeType,
) {
}

pub unsafe extern "C" fn init_iso20_acdp_CLResControlModeType(
    mut CLResControlModeType: *mut iso20_acdp_CLResControlModeType,
) {
}

pub unsafe extern "C" fn init_iso20_acdp_ManifestType(
    mut ManifestType: *mut iso20_acdp_ManifestType,
) {
    (*ManifestType).Reference.arrayLen = 0 as u32 as u16;
    (*ManifestType).set_Id_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_acdp_SignaturePropertiesType(
    mut SignaturePropertiesType: *mut iso20_acdp_SignaturePropertiesType,
) {
    (*SignaturePropertiesType).set_Id_isUsed(0 as u32);
}
