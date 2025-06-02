use ::c2rust_bitfields;
use c2rust_bitfields::*;
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

pub unsafe extern "C" fn init_iso20_exiDocument(mut exiDoc: *mut iso20_exiDocument) {
    (*exiDoc).set_SessionSetupReq_isUsed(0 as u32);
    (*exiDoc).set_SessionSetupRes_isUsed(0 as u32);
    (*exiDoc).set_AuthorizationSetupReq_isUsed(0 as u32);
    (*exiDoc).set_AuthorizationSetupRes_isUsed(0 as u32);
    (*exiDoc).set_AuthorizationReq_isUsed(0 as u32);
    (*exiDoc).set_AuthorizationRes_isUsed(0 as u32);
    (*exiDoc).set_ServiceDiscoveryReq_isUsed(0 as u32);
    (*exiDoc).set_ServiceDiscoveryRes_isUsed(0 as u32);
    (*exiDoc).set_ServiceDetailReq_isUsed(0 as u32);
    (*exiDoc).set_ServiceDetailRes_isUsed(0 as u32);
    (*exiDoc).set_ServiceSelectionReq_isUsed(0 as u32);
    (*exiDoc).set_ServiceSelectionRes_isUsed(0 as u32);
    (*exiDoc).set_ScheduleExchangeReq_isUsed(0 as u32);
    (*exiDoc).set_ScheduleExchangeRes_isUsed(0 as u32);
    (*exiDoc).set_PowerDeliveryReq_isUsed(0 as u32);
    (*exiDoc).set_PowerDeliveryRes_isUsed(0 as u32);
    (*exiDoc).set_MeteringConfirmationReq_isUsed(0 as u32);
    (*exiDoc).set_MeteringConfirmationRes_isUsed(0 as u32);
    (*exiDoc).set_SessionStopReq_isUsed(0 as u32);
    (*exiDoc).set_SessionStopRes_isUsed(0 as u32);
    (*exiDoc).set_CertificateInstallationReq_isUsed(0 as u32);
    (*exiDoc).set_CertificateInstallationRes_isUsed(0 as u32);
    (*exiDoc).set_VehicleCheckInReq_isUsed(0 as u32);
    (*exiDoc).set_VehicleCheckInRes_isUsed(0 as u32);
    (*exiDoc).set_VehicleCheckOutReq_isUsed(0 as u32);
    (*exiDoc).set_VehicleCheckOutRes_isUsed(0 as u32);
    (*exiDoc).set_SignedInstallationData_isUsed(0 as u32);
    (*exiDoc).set_SignedMeteringData_isUsed(0 as u32);
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

pub unsafe extern "C" fn init_iso20_TransformType(mut TransformType: *mut iso20_TransformType) {
    (*TransformType).set_ANY_isUsed(0 as u32);
    (*TransformType).set_XPath_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_PowerScheduleEntryType(
    mut PowerScheduleEntryType: *mut iso20_PowerScheduleEntryType,
) {
    (*PowerScheduleEntryType).set_Power_L2_isUsed(0 as u32);
    (*PowerScheduleEntryType).set_Power_L3_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_EVPriceRuleType(
    mut EVPriceRuleType: *mut iso20_EVPriceRuleType,
) {
}

pub unsafe extern "C" fn init_iso20_TransformsType(mut TransformsType: *mut iso20_TransformsType) {}

pub unsafe extern "C" fn init_iso20_DSAKeyValueType(
    mut DSAKeyValueType: *mut iso20_DSAKeyValueType,
) {
    (*DSAKeyValueType).set_P_isUsed(0 as u32);
    (*DSAKeyValueType).set_Q_isUsed(0 as u32);
    (*DSAKeyValueType).set_G_isUsed(0 as u32);
    (*DSAKeyValueType).set_J_isUsed(0 as u32);
    (*DSAKeyValueType).set_Seed_isUsed(0 as u32);
    (*DSAKeyValueType).set_PgenCounter_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_X509IssuerSerialType(
    mut X509IssuerSerialType: *mut iso20_X509IssuerSerialType,
) {
}

pub unsafe extern "C" fn init_iso20_EVPowerScheduleEntryType(
    mut EVPowerScheduleEntryType: *mut iso20_EVPowerScheduleEntryType,
) {
}

pub unsafe extern "C" fn init_iso20_EVPriceRuleStackType(
    mut EVPriceRuleStackType: *mut iso20_EVPriceRuleStackType,
) {
    (*EVPriceRuleStackType).EVPriceRule.arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_DigestMethodType(
    mut DigestMethodType: *mut iso20_DigestMethodType,
) {
    (*DigestMethodType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_RSAKeyValueType(
    mut RSAKeyValueType: *mut iso20_RSAKeyValueType,
) {
}

pub unsafe extern "C" fn init_iso20_PriceRuleType(mut PriceRuleType: *mut iso20_PriceRuleType) {
    (*PriceRuleType).set_ParkingFee_isUsed(0 as u32);
    (*PriceRuleType).set_ParkingFeePeriod_isUsed(0 as u32);
    (*PriceRuleType).set_CarbonDioxideEmission_isUsed(0 as u32);
    (*PriceRuleType).set_RenewableGenerationPercentage_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_PowerScheduleEntryListType(
    mut PowerScheduleEntryListType: *mut iso20_PowerScheduleEntryListType,
) {
    (*PowerScheduleEntryListType).PowerScheduleEntry.arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_CanonicalizationMethodType(
    mut CanonicalizationMethodType: *mut iso20_CanonicalizationMethodType,
) {
    (*CanonicalizationMethodType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_TaxRuleType(mut TaxRuleType: *mut iso20_TaxRuleType) {
    (*TaxRuleType).set_TaxRuleName_isUsed(0 as u32);
    (*TaxRuleType).set_TaxIncludedInPrice_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_PriceRuleStackType(
    mut PriceRuleStackType: *mut iso20_PriceRuleStackType,
) {
    (*PriceRuleStackType).PriceRule.arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_AdditionalServiceType(
    mut AdditionalServiceType: *mut iso20_AdditionalServiceType,
) {
}

pub unsafe extern "C" fn init_iso20_PriceLevelScheduleEntryType(
    mut PriceLevelScheduleEntryType: *mut iso20_PriceLevelScheduleEntryType,
) {
}

pub unsafe extern "C" fn init_iso20_PowerScheduleType(
    mut PowerScheduleType: *mut iso20_PowerScheduleType,
) {
    (*PowerScheduleType).set_AvailableEnergy_isUsed(0 as u32);
    (*PowerScheduleType).set_PowerTolerance_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_SignatureMethodType(
    mut SignatureMethodType: *mut iso20_SignatureMethodType,
) {
    (*SignatureMethodType).set_HMACOutputLength_isUsed(0 as u32);
    (*SignatureMethodType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_KeyValueType(mut KeyValueType: *mut iso20_KeyValueType) {
    (*KeyValueType).set_DSAKeyValue_isUsed(0 as u32);
    (*KeyValueType).set_RSAKeyValue_isUsed(0 as u32);
    (*KeyValueType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_EVPowerScheduleEntryListType(
    mut EVPowerScheduleEntryListType: *mut iso20_EVPowerScheduleEntryListType,
) {
    (*EVPowerScheduleEntryListType)
        .EVPowerScheduleEntry
        .arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_ReferenceType(mut ReferenceType: *mut iso20_ReferenceType) {
    (*ReferenceType).set_Id_isUsed(0 as u32);
    (*ReferenceType).set_Type_isUsed(0 as u32);
    (*ReferenceType).set_URI_isUsed(0 as u32);
    (*ReferenceType).set_Transforms_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_RetrievalMethodType(
    mut RetrievalMethodType: *mut iso20_RetrievalMethodType,
) {
    (*RetrievalMethodType).set_Type_isUsed(0 as u32);
    (*RetrievalMethodType).set_URI_isUsed(0 as u32);
    (*RetrievalMethodType).set_Transforms_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_OverstayRuleType(
    mut OverstayRuleType: *mut iso20_OverstayRuleType,
) {
    (*OverstayRuleType).set_OverstayRuleDescription_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_X509DataType(mut X509DataType: *mut iso20_X509DataType) {
    (*X509DataType).set_X509IssuerSerial_isUsed(0 as u32);
    (*X509DataType).set_X509SKI_isUsed(0 as u32);
    (*X509DataType).set_X509SubjectName_isUsed(0 as u32);
    (*X509DataType).set_X509Certificate_isUsed(0 as u32);
    (*X509DataType).set_X509CRL_isUsed(0 as u32);
    (*X509DataType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_EVPriceRuleStackListType(
    mut EVPriceRuleStackListType: *mut iso20_EVPriceRuleStackListType,
) {
    (*EVPriceRuleStackListType).EVPriceRuleStack.arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_PGPDataType(mut PGPDataType: *mut iso20_PGPDataType) {
    ((*PGPDataType).c2rust_unnamed).choice_1_isUsed = 0 as u32;
    ((*PGPDataType).c2rust_unnamed).choice_2_isUsed = 0 as u32;
}

pub unsafe extern "C" fn init_iso20_RationalNumberType(
    mut RationalNumberType: *mut iso20_RationalNumberType,
) {
}

pub unsafe extern "C" fn init_iso20_SPKIDataType(mut SPKIDataType: *mut iso20_SPKIDataType) {
    (*SPKIDataType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_SignedInfoType(mut SignedInfoType: *mut iso20_SignedInfoType) {
    (*SignedInfoType).Reference.arrayLen = 0 as u32 as u16;
    (*SignedInfoType).set_Id_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_EVPowerScheduleType(
    mut EVPowerScheduleType: *mut iso20_EVPowerScheduleType,
) {
}

pub unsafe extern "C" fn init_iso20_SignatureValueType(
    mut SignatureValueType: *mut iso20_SignatureValueType,
) {
    (*SignatureValueType).set_Id_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_SubCertificatesType(
    mut SubCertificatesType: *mut iso20_SubCertificatesType,
) {
    (*SubCertificatesType).Certificate.arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_ParameterType(mut ParameterType: *mut iso20_ParameterType) {
    (*ParameterType).set_boolValue_isUsed(0 as u32);
    (*ParameterType).set_byteValue_isUsed(0 as u32);
    (*ParameterType).set_shortValue_isUsed(0 as u32);
    (*ParameterType).set_intValue_isUsed(0 as u32);
    (*ParameterType).set_rationalNumber_isUsed(0 as u32);
    (*ParameterType).set_finiteString_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_EVAbsolutePriceScheduleType(
    mut EVAbsolutePriceScheduleType: *mut iso20_EVAbsolutePriceScheduleType,
) {
}

pub unsafe extern "C" fn init_iso20_ChargingScheduleType(
    mut ChargingScheduleType: *mut iso20_ChargingScheduleType,
) {
    (*ChargingScheduleType).set_AbsolutePriceSchedule_isUsed(0 as u32);
    (*ChargingScheduleType).set_PriceLevelSchedule_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_DetailedCostType(
    mut DetailedCostType: *mut iso20_DetailedCostType,
) {
}

pub unsafe extern "C" fn init_iso20_KeyInfoType(mut KeyInfoType: *mut iso20_KeyInfoType) {
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

pub unsafe extern "C" fn init_iso20_ObjectType(mut ObjectType: *mut iso20_ObjectType) {
    (*ObjectType).set_Encoding_isUsed(0 as u32);
    (*ObjectType).set_Id_isUsed(0 as u32);
    (*ObjectType).set_MimeType_isUsed(0 as u32);
    (*ObjectType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_PriceLevelScheduleEntryListType(
    mut PriceLevelScheduleEntryListType: *mut iso20_PriceLevelScheduleEntryListType,
) {
    (*PriceLevelScheduleEntryListType)
        .PriceLevelScheduleEntry
        .arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_DetailedTaxType(
    mut DetailedTaxType: *mut iso20_DetailedTaxType,
) {
}

pub unsafe extern "C" fn init_iso20_TaxRuleListType(
    mut TaxRuleListType: *mut iso20_TaxRuleListType,
) {
    (*TaxRuleListType).TaxRule.arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_PriceRuleStackListType(
    mut PriceRuleStackListType: *mut iso20_PriceRuleStackListType,
) {
    (*PriceRuleStackListType).PriceRuleStack.arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_OverstayRuleListType(
    mut OverstayRuleListType: *mut iso20_OverstayRuleListType,
) {
    (*OverstayRuleListType).OverstayRule.arrayLen = 0 as u32 as u16;
    (*OverstayRuleListType).set_OverstayTimeThreshold_isUsed(0 as u32);
    (*OverstayRuleListType).set_OverstayPowerThreshold_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_AdditionalServiceListType(
    mut AdditionalServiceListType: *mut iso20_AdditionalServiceListType,
) {
    (*AdditionalServiceListType).AdditionalService.arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_ServiceType(mut ServiceType: *mut iso20_ServiceType) {}

pub unsafe extern "C" fn init_iso20_ParameterSetType(
    mut ParameterSetType: *mut iso20_ParameterSetType,
) {
    (*ParameterSetType).Parameter.arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_ScheduleTupleType(
    mut ScheduleTupleType: *mut iso20_ScheduleTupleType,
) {
    (*ScheduleTupleType).set_DischargingSchedule_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_SupportedProvidersListType(
    mut SupportedProvidersListType: *mut iso20_SupportedProvidersListType,
) {
    (*SupportedProvidersListType).ProviderID.arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_ContractCertificateChainType(
    mut ContractCertificateChainType: *mut iso20_ContractCertificateChainType,
) {
}

pub unsafe extern "C" fn init_iso20_Dynamic_EVPPTControlModeType(
    mut Dynamic_EVPPTControlModeType: *mut iso20_Dynamic_EVPPTControlModeType,
) {
}

pub unsafe extern "C" fn init_iso20_MeterInfoType(mut MeterInfoType: *mut iso20_MeterInfoType) {
    (*MeterInfoType).set_BPT_DischargedEnergyReadingWh_isUsed(0 as u32);
    (*MeterInfoType).set_CapacitiveEnergyReadingVARh_isUsed(0 as u32);
    (*MeterInfoType).set_BPT_InductiveEnergyReadingVARh_isUsed(0 as u32);
    (*MeterInfoType).set_MeterSignature_isUsed(0 as u32);
    (*MeterInfoType).set_MeterStatus_isUsed(0 as u32);
    (*MeterInfoType).set_MeterTimestamp_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_SignatureType(mut SignatureType: *mut iso20_SignatureType) {
    (*SignatureType).set_Id_isUsed(0 as u32);
    (*SignatureType).set_KeyInfo_isUsed(0 as u32);
    (*SignatureType).set_Object_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_Scheduled_EVPPTControlModeType(
    mut Scheduled_EVPPTControlModeType: *mut iso20_Scheduled_EVPPTControlModeType,
) {
    (*Scheduled_EVPPTControlModeType).set_PowerToleranceAcceptance_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_ReceiptType(mut ReceiptType: *mut iso20_ReceiptType) {
    (*ReceiptType).TaxCosts.arrayLen = 0 as u32 as u16;
    (*ReceiptType).set_EnergyCosts_isUsed(0 as u32);
    (*ReceiptType).set_OccupancyCosts_isUsed(0 as u32);
    (*ReceiptType).set_AdditionalServicesCosts_isUsed(0 as u32);
    (*ReceiptType).set_OverstayCosts_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_AbsolutePriceScheduleType(
    mut AbsolutePriceScheduleType: *mut iso20_AbsolutePriceScheduleType,
) {
    (*AbsolutePriceScheduleType).set_Id_isUsed(0 as u32);
    (*AbsolutePriceScheduleType).set_PriceScheduleDescription_isUsed(0 as u32);
    (*AbsolutePriceScheduleType).set_MinimumCost_isUsed(0 as u32);
    (*AbsolutePriceScheduleType).set_MaximumCost_isUsed(0 as u32);
    (*AbsolutePriceScheduleType).set_TaxRules_isUsed(0 as u32);
    (*AbsolutePriceScheduleType).set_OverstayRules_isUsed(0 as u32);
    (*AbsolutePriceScheduleType).set_AdditionalSelectedServices_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_EVPowerProfileEntryListType(
    mut EVPowerProfileEntryListType: *mut iso20_EVPowerProfileEntryListType,
) {
    (*EVPowerProfileEntryListType).EVPowerProfileEntry.arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_Dynamic_SMDTControlModeType(
    mut Dynamic_SMDTControlModeType: *mut iso20_Dynamic_SMDTControlModeType,
) {
}

pub unsafe extern "C" fn init_iso20_EVEnergyOfferType(
    mut EVEnergyOfferType: *mut iso20_EVEnergyOfferType,
) {
}

pub unsafe extern "C" fn init_iso20_PriceLevelScheduleType(
    mut PriceLevelScheduleType: *mut iso20_PriceLevelScheduleType,
) {
    (*PriceLevelScheduleType).set_Id_isUsed(0 as u32);
    (*PriceLevelScheduleType).set_PriceScheduleDescription_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_Scheduled_SMDTControlModeType(
    mut Scheduled_SMDTControlModeType: *mut iso20_Scheduled_SMDTControlModeType,
) {
}

pub unsafe extern "C" fn init_iso20_MessageHeaderType(
    mut MessageHeaderType: *mut iso20_MessageHeaderType,
) {
    (*MessageHeaderType).set_Signature_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_SignaturePropertyType(
    mut SignaturePropertyType: *mut iso20_SignaturePropertyType,
) {
    (*SignaturePropertyType).set_Id_isUsed(0 as u32);
    (*SignaturePropertyType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_ServiceIDListType(
    mut ServiceIDListType: *mut iso20_ServiceIDListType,
) {
    (*ServiceIDListType).ServiceID.arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_SelectedServiceType(
    mut SelectedServiceType: *mut iso20_SelectedServiceType,
) {
}

pub unsafe extern "C" fn init_iso20_SignedMeteringDataType(
    mut SignedMeteringDataType: *mut iso20_SignedMeteringDataType,
) {
    (*SignedMeteringDataType).set_Receipt_isUsed(0 as u32);
    (*SignedMeteringDataType).set_Dynamic_SMDTControlMode_isUsed(0 as u32);
    (*SignedMeteringDataType).set_Scheduled_SMDTControlMode_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_SignedCertificateChainType(
    mut SignedCertificateChainType: *mut iso20_SignedCertificateChainType,
) {
    (*SignedCertificateChainType).set_SubCertificates_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_EIM_AReqAuthorizationModeType(
    mut EIM_AReqAuthorizationModeType: *mut iso20_EIM_AReqAuthorizationModeType,
) {
}

pub unsafe extern "C" fn init_iso20_SelectedServiceListType(
    mut SelectedServiceListType: *mut iso20_SelectedServiceListType,
) {
    (*SelectedServiceListType).SelectedService.arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_Dynamic_SEReqControlModeType(
    mut Dynamic_SEReqControlModeType: *mut iso20_Dynamic_SEReqControlModeType,
) {
    (*Dynamic_SEReqControlModeType).set_MinimumSOC_isUsed(0 as u32);
    (*Dynamic_SEReqControlModeType).set_TargetSOC_isUsed(0 as u32);
    (*Dynamic_SEReqControlModeType).set_EVMaximumV2XEnergyRequest_isUsed(0 as u32);
    (*Dynamic_SEReqControlModeType).set_EVMinimumV2XEnergyRequest_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_EVSEStatusType(mut EVSEStatusType: *mut iso20_EVSEStatusType) {}

pub unsafe extern "C" fn init_iso20_ListOfRootCertificateIDsType(
    mut ListOfRootCertificateIDsType: *mut iso20_ListOfRootCertificateIDsType,
) {
    (*ListOfRootCertificateIDsType).RootCertificateID.arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_PnC_AReqAuthorizationModeType(
    mut PnC_AReqAuthorizationModeType: *mut iso20_PnC_AReqAuthorizationModeType,
) {
}

pub unsafe extern "C" fn init_iso20_ServiceListType(
    mut ServiceListType: *mut iso20_ServiceListType,
) {
    (*ServiceListType).Service.arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_ServiceParameterListType(
    mut ServiceParameterListType: *mut iso20_ServiceParameterListType,
) {
    (*ServiceParameterListType).ParameterSet.arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_Scheduled_SEReqControlModeType(
    mut Scheduled_SEReqControlModeType: *mut iso20_Scheduled_SEReqControlModeType,
) {
    (*Scheduled_SEReqControlModeType).set_DepartureTime_isUsed(0 as u32);
    (*Scheduled_SEReqControlModeType).set_EVTargetEnergyRequest_isUsed(0 as u32);
    (*Scheduled_SEReqControlModeType).set_EVMaximumEnergyRequest_isUsed(0 as u32);
    (*Scheduled_SEReqControlModeType).set_EVMinimumEnergyRequest_isUsed(0 as u32);
    (*Scheduled_SEReqControlModeType).set_EVEnergyOffer_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_EVPowerProfileType(
    mut EVPowerProfileType: *mut iso20_EVPowerProfileType,
) {
    (*EVPowerProfileType).set_Dynamic_EVPPTControlMode_isUsed(0 as u32);
    (*EVPowerProfileType).set_Scheduled_EVPPTControlMode_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_CertificateChainType(
    mut CertificateChainType: *mut iso20_CertificateChainType,
) {
    (*CertificateChainType).set_SubCertificates_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_EIM_ASResAuthorizationModeType(
    mut EIM_ASResAuthorizationModeType: *mut iso20_EIM_ASResAuthorizationModeType,
) {
}

pub unsafe extern "C" fn init_iso20_Dynamic_SEResControlModeType(
    mut Dynamic_SEResControlModeType: *mut iso20_Dynamic_SEResControlModeType,
) {
    (*Dynamic_SEResControlModeType).set_DepartureTime_isUsed(0 as u32);
    (*Dynamic_SEResControlModeType).set_MinimumSOC_isUsed(0 as u32);
    (*Dynamic_SEResControlModeType).set_TargetSOC_isUsed(0 as u32);
    (*Dynamic_SEResControlModeType).set_AbsolutePriceSchedule_isUsed(0 as u32);
    (*Dynamic_SEResControlModeType).set_PriceLevelSchedule_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_EMAIDListType(mut EMAIDListType: *mut iso20_EMAIDListType) {
    (*EMAIDListType).EMAID.arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_SignedInstallationDataType(
    mut SignedInstallationDataType: *mut iso20_SignedInstallationDataType,
) {
    (*SignedInstallationDataType).set_SECP521_EncryptedPrivateKey_isUsed(0 as u32);
    (*SignedInstallationDataType).set_X448_EncryptedPrivateKey_isUsed(0 as u32);
    (*SignedInstallationDataType).set_TPM_EncryptedPrivateKey_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_PnC_ASResAuthorizationModeType(
    mut PnC_ASResAuthorizationModeType: *mut iso20_PnC_ASResAuthorizationModeType,
) {
    (*PnC_ASResAuthorizationModeType).set_SupportedProviders_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_Scheduled_SEResControlModeType(
    mut Scheduled_SEResControlModeType: *mut iso20_Scheduled_SEResControlModeType,
) {
    (*Scheduled_SEResControlModeType).ScheduleTuple.arrayLen = 0 as u32 as u16;
}

pub unsafe extern "C" fn init_iso20_SessionSetupReqType(
    mut SessionSetupReqType: *mut iso20_SessionSetupReqType,
) {
}

pub unsafe extern "C" fn init_iso20_SessionSetupResType(
    mut SessionSetupResType: *mut iso20_SessionSetupResType,
) {
}

pub unsafe extern "C" fn init_iso20_AuthorizationSetupReqType(
    mut AuthorizationSetupReqType: *mut iso20_AuthorizationSetupReqType,
) {
}

pub unsafe extern "C" fn init_iso20_AuthorizationSetupResType(
    mut AuthorizationSetupResType: *mut iso20_AuthorizationSetupResType,
) {
    (*AuthorizationSetupResType).AuthorizationServices.arrayLen = 0 as u32 as u16;
    (*AuthorizationSetupResType).set_EIM_ASResAuthorizationMode_isUsed(0 as u32);
    (*AuthorizationSetupResType).set_PnC_ASResAuthorizationMode_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_AuthorizationReqType(
    mut AuthorizationReqType: *mut iso20_AuthorizationReqType,
) {
    (*AuthorizationReqType).set_EIM_AReqAuthorizationMode_isUsed(0 as u32);
    (*AuthorizationReqType).set_PnC_AReqAuthorizationMode_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_AuthorizationResType(
    mut AuthorizationResType: *mut iso20_AuthorizationResType,
) {
}

pub unsafe extern "C" fn init_iso20_ServiceDiscoveryReqType(
    mut ServiceDiscoveryReqType: *mut iso20_ServiceDiscoveryReqType,
) {
    (*ServiceDiscoveryReqType).set_SupportedServiceIDs_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_ServiceDiscoveryResType(
    mut ServiceDiscoveryResType: *mut iso20_ServiceDiscoveryResType,
) {
    (*ServiceDiscoveryResType).set_VASList_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_ServiceDetailReqType(
    mut ServiceDetailReqType: *mut iso20_ServiceDetailReqType,
) {
}

pub unsafe extern "C" fn init_iso20_ServiceDetailResType(
    mut ServiceDetailResType: *mut iso20_ServiceDetailResType,
) {
}

pub unsafe extern "C" fn init_iso20_ServiceSelectionReqType(
    mut ServiceSelectionReqType: *mut iso20_ServiceSelectionReqType,
) {
    (*ServiceSelectionReqType).set_SelectedVASList_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_ServiceSelectionResType(
    mut ServiceSelectionResType: *mut iso20_ServiceSelectionResType,
) {
}

pub unsafe extern "C" fn init_iso20_ScheduleExchangeReqType(
    mut ScheduleExchangeReqType: *mut iso20_ScheduleExchangeReqType,
) {
    (*ScheduleExchangeReqType).set_Dynamic_SEReqControlMode_isUsed(0 as u32);
    (*ScheduleExchangeReqType).set_Scheduled_SEReqControlMode_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_ScheduleExchangeResType(
    mut ScheduleExchangeResType: *mut iso20_ScheduleExchangeResType,
) {
    (*ScheduleExchangeResType).set_GoToPause_isUsed(0 as u32);
    (*ScheduleExchangeResType).set_Dynamic_SEResControlMode_isUsed(0 as u32);
    (*ScheduleExchangeResType).set_Scheduled_SEResControlMode_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_PowerDeliveryReqType(
    mut PowerDeliveryReqType: *mut iso20_PowerDeliveryReqType,
) {
    (*PowerDeliveryReqType).set_EVPowerProfile_isUsed(0 as u32);
    (*PowerDeliveryReqType).set_BPT_ChannelSelection_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_PowerDeliveryResType(
    mut PowerDeliveryResType: *mut iso20_PowerDeliveryResType,
) {
    (*PowerDeliveryResType).set_EVSEStatus_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_MeteringConfirmationReqType(
    mut MeteringConfirmationReqType: *mut iso20_MeteringConfirmationReqType,
) {
}

pub unsafe extern "C" fn init_iso20_MeteringConfirmationResType(
    mut MeteringConfirmationResType: *mut iso20_MeteringConfirmationResType,
) {
}

pub unsafe extern "C" fn init_iso20_SessionStopReqType(
    mut SessionStopReqType: *mut iso20_SessionStopReqType,
) {
    (*SessionStopReqType).set_EVTerminationCode_isUsed(0 as u32);
    (*SessionStopReqType).set_EVTerminationExplanation_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_SessionStopResType(
    mut SessionStopResType: *mut iso20_SessionStopResType,
) {
}

pub unsafe extern "C" fn init_iso20_CertificateInstallationReqType(
    mut CertificateInstallationReqType: *mut iso20_CertificateInstallationReqType,
) {
    (*CertificateInstallationReqType).set_PrioritizedEMAIDs_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_CertificateInstallationResType(
    mut CertificateInstallationResType: *mut iso20_CertificateInstallationResType,
) {
}

pub unsafe extern "C" fn init_iso20_VehicleCheckInReqType(
    mut VehicleCheckInReqType: *mut iso20_VehicleCheckInReqType,
) {
    (*VehicleCheckInReqType).set_VehicleFrame_isUsed(0 as u32);
    (*VehicleCheckInReqType).set_DeviceOffset_isUsed(0 as u32);
    (*VehicleCheckInReqType).set_VehicleTravel_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_VehicleCheckInResType(
    mut VehicleCheckInResType: *mut iso20_VehicleCheckInResType,
) {
    (*VehicleCheckInResType).set_ParkingSpace_isUsed(0 as u32);
    (*VehicleCheckInResType).set_DeviceLocation_isUsed(0 as u32);
    (*VehicleCheckInResType).set_TargetDistance_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_VehicleCheckOutReqType(
    mut VehicleCheckOutReqType: *mut iso20_VehicleCheckOutReqType,
) {
}

pub unsafe extern "C" fn init_iso20_VehicleCheckOutResType(
    mut VehicleCheckOutResType: *mut iso20_VehicleCheckOutResType,
) {
}

pub unsafe extern "C" fn init_iso20_CLReqControlModeType(
    mut CLReqControlModeType: *mut iso20_CLReqControlModeType,
) {
}

pub unsafe extern "C" fn init_iso20_CLResControlModeType(
    mut CLResControlModeType: *mut iso20_CLResControlModeType,
) {
}

pub unsafe extern "C" fn init_iso20_ManifestType(mut ManifestType: *mut iso20_ManifestType) {
    (*ManifestType).Reference.arrayLen = 0 as u32 as u16;
    (*ManifestType).set_Id_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_SignaturePropertiesType(
    mut SignaturePropertiesType: *mut iso20_SignaturePropertiesType,
) {
    (*SignaturePropertiesType).set_Id_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_exiFragment(mut exiFrag: *mut iso20_exiFragment) {
    (*exiFrag).set_AbsolutePriceSchedule_isUsed(0 as u32);
    (*exiFrag).set_CertificateInstallationReq_isUsed(0 as u32);
    (*exiFrag).set_MeteringConfirmationReq_isUsed(0 as u32);
    (*exiFrag).set_PnC_AReqAuthorizationMode_isUsed(0 as u32);
    (*exiFrag).set_SignedInfo_isUsed(0 as u32);
    (*exiFrag).set_SignedInstallationData_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_xmldsigFragment(mut xmldsigFrag: *mut iso20_xmldsigFragment) {
    (*xmldsigFrag).set_CanonicalizationMethod_isUsed(0 as u32);
    (*xmldsigFrag).set_DSAKeyValue_isUsed(0 as u32);
    (*xmldsigFrag).set_DigestMethod_isUsed(0 as u32);
    (*xmldsigFrag).set_KeyInfo_isUsed(0 as u32);
    (*xmldsigFrag).set_KeyValue_isUsed(0 as u32);
    (*xmldsigFrag).set_Manifest_isUsed(0 as u32);
    (*xmldsigFrag).set_Object_isUsed(0 as u32);
    (*xmldsigFrag).set_PGPData_isUsed(0 as u32);
    (*xmldsigFrag).set_RSAKeyValue_isUsed(0 as u32);
    (*xmldsigFrag).set_Reference_isUsed(0 as u32);
    (*xmldsigFrag).set_RetrievalMethod_isUsed(0 as u32);
    (*xmldsigFrag).set_SPKIData_isUsed(0 as u32);
    (*xmldsigFrag).set_Signature_isUsed(0 as u32);
    (*xmldsigFrag).set_SignatureMethod_isUsed(0 as u32);
    (*xmldsigFrag).set_SignatureProperties_isUsed(0 as u32);
    (*xmldsigFrag).set_SignatureProperty_isUsed(0 as u32);
    (*xmldsigFrag).set_SignatureValue_isUsed(0 as u32);
    (*xmldsigFrag).set_SignedInfo_isUsed(0 as u32);
    (*xmldsigFrag).set_Transform_isUsed(0 as u32);
    (*xmldsigFrag).set_Transforms_isUsed(0 as u32);
    (*xmldsigFrag).set_X509Data_isUsed(0 as u32);
    (*xmldsigFrag).set_X509IssuerSerial_isUsed(0 as u32);
}
