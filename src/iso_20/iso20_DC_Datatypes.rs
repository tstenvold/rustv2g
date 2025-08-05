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
pub type iso20_dc_evseNotificationType = u32;
pub const iso20_dc_evseNotificationType_MeteringConfirmation: iso20_dc_evseNotificationType = 5;
pub const iso20_dc_evseNotificationType_ServiceRenegotiation: iso20_dc_evseNotificationType = 4;
pub const iso20_dc_evseNotificationType_ScheduleRenegotiation: iso20_dc_evseNotificationType = 3;
pub const iso20_dc_evseNotificationType_Terminate: iso20_dc_evseNotificationType = 2;
pub const iso20_dc_evseNotificationType_ExitStandby: iso20_dc_evseNotificationType = 1;
pub const iso20_dc_evseNotificationType_Pause: iso20_dc_evseNotificationType = 0;
pub type iso20_dc_responseCodeType = u32;
pub const iso20_dc_responseCodeType_FAILED_WrongChargeParameter: iso20_dc_responseCodeType = 39;
pub const iso20_dc_responseCodeType_FAILED_UnknownSession: iso20_dc_responseCodeType = 38;
pub const iso20_dc_responseCodeType_FAILED_SignatureError: iso20_dc_responseCodeType = 37;
pub const iso20_dc_responseCodeType_FAILED_ServiceSelectionInvalid: iso20_dc_responseCodeType = 36;
pub const iso20_dc_responseCodeType_FAILED_ServiceIDInvalid: iso20_dc_responseCodeType = 35;
pub const iso20_dc_responseCodeType_FAILED_SequenceError: iso20_dc_responseCodeType = 34;
pub const iso20_dc_responseCodeType_FAILED_ScheduleSelectionInvalid: iso20_dc_responseCodeType = 33;
pub const iso20_dc_responseCodeType_FAILED_ScheduleRenegotiation: iso20_dc_responseCodeType = 32;
pub const iso20_dc_responseCodeType_FAILED_PowerToleranceNotConfirmed: iso20_dc_responseCodeType =
    31;
pub const iso20_dc_responseCodeType_FAILED_PowerDeliveryNotApplied: iso20_dc_responseCodeType = 30;
pub const iso20_dc_responseCodeType_FAILED_PauseNotAllowed: iso20_dc_responseCodeType = 29;
pub const iso20_dc_responseCodeType_FAILED_NoServiceRenegotiationSupported:
    iso20_dc_responseCodeType = 28;
pub const iso20_dc_responseCodeType_FAILED_NoEnergyTransferServiceSelected:
    iso20_dc_responseCodeType = 27;
pub const iso20_dc_responseCodeType_FAILED_MeteringSignatureNotValid: iso20_dc_responseCodeType =
    26;
pub const iso20_dc_responseCodeType_FAILED_EVPowerProfileViolation: iso20_dc_responseCodeType = 25;
pub const iso20_dc_responseCodeType_FAILED_EVPowerProfileInvalid: iso20_dc_responseCodeType = 24;
pub const iso20_dc_responseCodeType_FAILED_ContactorError: iso20_dc_responseCodeType = 23;
pub const iso20_dc_responseCodeType_FAILED_AssociationError: iso20_dc_responseCodeType = 22;
pub const iso20_dc_responseCodeType_FAILED: iso20_dc_responseCodeType = 21;
pub const iso20_dc_responseCodeType_WARNING_WPT: iso20_dc_responseCodeType = 20;
pub const iso20_dc_responseCodeType_WARNING_StandbyNotAllowed: iso20_dc_responseCodeType = 19;
pub const iso20_dc_responseCodeType_WARNING_ScheduleRenegotiationFailed: iso20_dc_responseCodeType =
    18;
pub const iso20_dc_responseCodeType_WARNING_PowerToleranceNotConfirmed: iso20_dc_responseCodeType =
    17;
pub const iso20_dc_responseCodeType_WARNING_NoContractMatchingPCIDFound: iso20_dc_responseCodeType =
    16;
pub const iso20_dc_responseCodeType_WARNING_NoCertificateAvailable: iso20_dc_responseCodeType = 15;
pub const iso20_dc_responseCodeType_WARNING_GeneralPnCAuthorizationError:
    iso20_dc_responseCodeType = 14;
pub const iso20_dc_responseCodeType_WARNING_EVPowerProfileViolation: iso20_dc_responseCodeType = 13;
pub const iso20_dc_responseCodeType_WARNING_eMSPUnknown: iso20_dc_responseCodeType = 12;
pub const iso20_dc_responseCodeType_WARNING_EIMAuthorizationFailure: iso20_dc_responseCodeType = 11;
pub const iso20_dc_responseCodeType_WARNING_ChallengeInvalid: iso20_dc_responseCodeType = 10;
pub const iso20_dc_responseCodeType_WARNING_CertificateValidationError: iso20_dc_responseCodeType =
    9;
pub const iso20_dc_responseCodeType_WARNING_CertificateRevoked: iso20_dc_responseCodeType = 8;
pub const iso20_dc_responseCodeType_WARNING_CertificateNotYetValid: iso20_dc_responseCodeType = 7;
pub const iso20_dc_responseCodeType_WARNING_CertificateExpired: iso20_dc_responseCodeType = 6;
pub const iso20_dc_responseCodeType_WARNING_AuthorizationSelectionInvalid:
    iso20_dc_responseCodeType = 5;
pub const iso20_dc_responseCodeType_OK_PowerToleranceConfirmed: iso20_dc_responseCodeType = 4;
pub const iso20_dc_responseCodeType_OK_OldSessionJoined: iso20_dc_responseCodeType = 3;
pub const iso20_dc_responseCodeType_OK_NewSessionEstablished: iso20_dc_responseCodeType = 2;
pub const iso20_dc_responseCodeType_OK_CertificateExpiresSoon: iso20_dc_responseCodeType = 1;
pub const iso20_dc_responseCodeType_OK: iso20_dc_responseCodeType = 0;
pub type iso20_dc_processingType = u32;
pub const iso20_dc_processingType_Ongoing_WaitingForCustomerInteraction: iso20_dc_processingType =
    2;
pub const iso20_dc_processingType_Ongoing: iso20_dc_processingType = 1;
pub const iso20_dc_processingType_Finished: iso20_dc_processingType = 0;
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_dc_TransformType {
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

pub struct iso20_dc_TransformsType {
    pub Transform: iso20_dc_TransformType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_dc_DSAKeyValueType {
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

pub struct iso20_dc_X509IssuerSerialType {
    pub X509IssuerName: C2RustUnnamed_9,
    pub X509SerialNumber: ExiSigned,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_9 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_dc_DigestMethodType {
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

pub struct iso20_dc_RSAKeyValueType {
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

pub struct iso20_dc_CanonicalizationMethodType {
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

pub struct iso20_dc_SignatureMethodType {
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

pub struct iso20_dc_KeyValueType {
    pub DSAKeyValue: iso20_dc_DSAKeyValueType,
    #[bitfield(name = "DSAKeyValue_isUsed", ty = "u32", bits = "0..=0")]
    pub DSAKeyValue_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub RSAKeyValue: iso20_dc_RSAKeyValueType,
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

pub struct iso20_dc_ReferenceType {
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
    pub Transforms: iso20_dc_TransformsType,
    #[bitfield(name = "Transforms_isUsed", ty = "u32", bits = "0..=0")]
    pub Transforms_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 3],
    pub DigestMethod: iso20_dc_DigestMethodType,
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

pub struct iso20_dc_RetrievalMethodType {
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
    pub Transforms: iso20_dc_TransformsType,
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

pub struct iso20_dc_X509DataType {
    pub X509IssuerSerial: iso20_dc_X509IssuerSerialType,
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

pub struct iso20_dc_PGPDataType {
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

pub struct iso20_dc_SPKIDataType {
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

pub struct iso20_dc_SignedInfoType {
    pub Id: C2RustUnnamed_41,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub CanonicalizationMethod: iso20_dc_CanonicalizationMethodType,
    pub SignatureMethod: iso20_dc_SignatureMethodType,
    pub Reference: C2RustUnnamed_40,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_40 {
    pub array: [iso20_dc_ReferenceType; 4],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_41 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_dc_SignatureValueType {
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

pub struct iso20_dc_KeyInfoType {
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
    pub KeyValue: iso20_dc_KeyValueType,
    #[bitfield(name = "KeyValue_isUsed", ty = "u32", bits = "0..=0")]
    pub KeyValue_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 3],
    pub RetrievalMethod: iso20_dc_RetrievalMethodType,
    #[bitfield(name = "RetrievalMethod_isUsed", ty = "u32", bits = "0..=0")]
    pub RetrievalMethod_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 3],
    pub X509Data: iso20_dc_X509DataType,
    #[bitfield(name = "X509Data_isUsed", ty = "u32", bits = "0..=0")]
    pub X509Data_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 3],
    pub PGPData: iso20_dc_PGPDataType,
    #[bitfield(name = "PGPData_isUsed", ty = "u32", bits = "0..=0")]
    pub PGPData_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 3],
    pub SPKIData: iso20_dc_SPKIDataType,
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

pub struct iso20_dc_ObjectType {
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

pub struct iso20_dc_RationalNumberType {
    pub Exponent: int8_t,
    pub Value: i16,
}
#[derive(Copy, Clone)]

pub struct iso20_dc_DetailedCostType {
    pub Amount: iso20_dc_RationalNumberType,
    pub CostPerUnit: iso20_dc_RationalNumberType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_dc_SignatureType {
    pub Id: C2RustUnnamed_52,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub SignedInfo: iso20_dc_SignedInfoType,
    pub SignatureValue: iso20_dc_SignatureValueType,
    pub KeyInfo: iso20_dc_KeyInfoType,
    #[bitfield(name = "KeyInfo_isUsed", ty = "u32", bits = "0..=0")]
    pub KeyInfo_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
    pub Object: iso20_dc_ObjectType,
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

pub struct iso20_dc_DetailedTaxType {
    pub TaxRuleID: u32,
    pub Amount: iso20_dc_RationalNumberType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_dc_MessageHeaderType {
    pub SessionID: C2RustUnnamed_53,
    pub TimeStamp: u64,
    pub Signature: iso20_dc_SignatureType,
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

pub struct iso20_dc_SignaturePropertyType {
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

pub struct iso20_dc_DC_CPDReqEnergyTransferModeType {
    pub EVMaximumChargePower: iso20_dc_RationalNumberType,
    pub EVMinimumChargePower: iso20_dc_RationalNumberType,
    pub EVMaximumChargeCurrent: iso20_dc_RationalNumberType,
    pub EVMinimumChargeCurrent: iso20_dc_RationalNumberType,
    pub EVMaximumVoltage: iso20_dc_RationalNumberType,
    pub EVMinimumVoltage: iso20_dc_RationalNumberType,
    pub TargetSOC: int8_t,
    #[bitfield(name = "TargetSOC_isUsed", ty = "u32", bits = "0..=0")]
    pub TargetSOC_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_dc_DisplayParametersType {
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
    pub BatteryEnergyCapacity: iso20_dc_RationalNumberType,
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

pub struct iso20_dc_DC_CPDResEnergyTransferModeType {
    pub EVSEMaximumChargePower: iso20_dc_RationalNumberType,
    pub EVSEMinimumChargePower: iso20_dc_RationalNumberType,
    pub EVSEMaximumChargeCurrent: iso20_dc_RationalNumberType,
    pub EVSEMinimumChargeCurrent: iso20_dc_RationalNumberType,
    pub EVSEMaximumVoltage: iso20_dc_RationalNumberType,
    pub EVSEMinimumVoltage: iso20_dc_RationalNumberType,
    pub EVSEPowerRampLimitation: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVSEPowerRampLimitation_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEPowerRampLimitation_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]

pub struct iso20_dc_EVSEStatusType {
    pub NotificationMaxDelay: u16,
    pub EVSENotification: iso20_dc_evseNotificationType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_dc_MeterInfoType {
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

pub struct iso20_dc_Dynamic_DC_CLReqControlModeType {
    pub DepartureTime: u32,
    #[bitfield(name = "DepartureTime_isUsed", ty = "u32", bits = "0..=0")]
    pub DepartureTime_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub EVTargetEnergyRequest: iso20_dc_RationalNumberType,
    pub EVMaximumEnergyRequest: iso20_dc_RationalNumberType,
    pub EVMinimumEnergyRequest: iso20_dc_RationalNumberType,
    pub EVMaximumChargePower: iso20_dc_RationalNumberType,
    pub EVMinimumChargePower: iso20_dc_RationalNumberType,
    pub EVMaximumChargeCurrent: iso20_dc_RationalNumberType,
    pub EVMaximumVoltage: iso20_dc_RationalNumberType,
    pub EVMinimumVoltage: iso20_dc_RationalNumberType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_dc_Scheduled_DC_CLReqControlModeType {
    pub EVTargetEnergyRequest: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVTargetEnergyRequest_isUsed", ty = "u32", bits = "0..=0")]
    pub EVTargetEnergyRequest_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub EVMaximumEnergyRequest: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVMaximumEnergyRequest_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumEnergyRequest_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub EVMinimumEnergyRequest: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVMinimumEnergyRequest_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumEnergyRequest_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub EVTargetCurrent: iso20_dc_RationalNumberType,
    pub EVTargetVoltage: iso20_dc_RationalNumberType,
    pub EVMaximumChargePower: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVMaximumChargePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumChargePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub EVMinimumChargePower: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVMinimumChargePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumChargePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 1],
    pub EVMaximumChargeCurrent: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVMaximumChargeCurrent_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumChargeCurrent_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 1],
    pub EVMaximumVoltage: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVMaximumVoltage_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumVoltage_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_5: [u8; 1],
    pub EVMinimumVoltage: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVMinimumVoltage_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumVoltage_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_6: [u8; 1],
}
#[derive(Copy, Clone)]

pub struct iso20_dc_CLReqControlModeType {
    pub _unused: i32,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_dc_ReceiptType {
    pub TimeAnchor: u64,
    pub EnergyCosts: iso20_dc_DetailedCostType,
    #[bitfield(name = "EnergyCosts_isUsed", ty = "u32", bits = "0..=0")]
    pub EnergyCosts_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub OccupancyCosts: iso20_dc_DetailedCostType,
    #[bitfield(name = "OccupancyCosts_isUsed", ty = "u32", bits = "0..=0")]
    pub OccupancyCosts_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub AdditionalServicesCosts: iso20_dc_DetailedCostType,
    #[bitfield(name = "AdditionalServicesCosts_isUsed", ty = "u32", bits = "0..=0")]
    pub AdditionalServicesCosts_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub OverstayCosts: iso20_dc_DetailedCostType,
    #[bitfield(name = "OverstayCosts_isUsed", ty = "u32", bits = "0..=0")]
    pub OverstayCosts_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub TaxCosts: C2RustUnnamed_59,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_59 {
    pub array: [iso20_dc_DetailedTaxType; 10],
    pub arrayLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_dc_Dynamic_DC_CLResControlModeType {
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
    pub EVSEMaximumChargePower: iso20_dc_RationalNumberType,
    pub EVSEMinimumChargePower: iso20_dc_RationalNumberType,
    pub EVSEMaximumChargeCurrent: iso20_dc_RationalNumberType,
    pub EVSEMaximumVoltage: iso20_dc_RationalNumberType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_dc_Scheduled_DC_CLResControlModeType {
    pub EVSEMaximumChargePower: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVSEMaximumChargePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEMaximumChargePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub EVSEMinimumChargePower: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVSEMinimumChargePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEMinimumChargePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub EVSEMaximumChargeCurrent: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVSEMaximumChargeCurrent_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEMaximumChargeCurrent_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub EVSEMaximumVoltage: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVSEMaximumVoltage_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEMaximumVoltage_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
}
#[derive(Copy, Clone)]

pub struct iso20_dc_CLResControlModeType {
    pub _unused: i32,
}
#[derive(Copy, Clone)]

pub struct iso20_dc_DC_CableCheckReqType {
    pub Header: iso20_dc_MessageHeaderType,
}
#[derive(Copy, Clone)]

pub struct iso20_dc_DC_CableCheckResType {
    pub Header: iso20_dc_MessageHeaderType,
    pub ResponseCode: iso20_dc_responseCodeType,
    pub EVSEProcessing: iso20_dc_processingType,
}
#[derive(Copy, Clone)]

pub struct iso20_dc_DC_PreChargeReqType {
    pub Header: iso20_dc_MessageHeaderType,
    pub EVProcessing: iso20_dc_processingType,
    pub EVPresentVoltage: iso20_dc_RationalNumberType,
    pub EVTargetVoltage: iso20_dc_RationalNumberType,
}
#[derive(Copy, Clone)]

pub struct iso20_dc_DC_PreChargeResType {
    pub Header: iso20_dc_MessageHeaderType,
    pub ResponseCode: iso20_dc_responseCodeType,
    pub EVSEPresentVoltage: iso20_dc_RationalNumberType,
}
#[derive(Copy, Clone)]

pub struct iso20_dc_DC_WeldingDetectionReqType {
    pub Header: iso20_dc_MessageHeaderType,
    pub EVProcessing: iso20_dc_processingType,
}
#[derive(Copy, Clone)]

pub struct iso20_dc_DC_WeldingDetectionResType {
    pub Header: iso20_dc_MessageHeaderType,
    pub ResponseCode: iso20_dc_responseCodeType,
    pub EVSEPresentVoltage: iso20_dc_RationalNumberType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_dc_BPT_DC_CPDReqEnergyTransferModeType {
    pub EVMaximumChargePower: iso20_dc_RationalNumberType,
    pub EVMinimumChargePower: iso20_dc_RationalNumberType,
    pub EVMaximumChargeCurrent: iso20_dc_RationalNumberType,
    pub EVMinimumChargeCurrent: iso20_dc_RationalNumberType,
    pub EVMaximumVoltage: iso20_dc_RationalNumberType,
    pub EVMinimumVoltage: iso20_dc_RationalNumberType,
    pub TargetSOC: int8_t,
    #[bitfield(name = "TargetSOC_isUsed", ty = "u32", bits = "0..=0")]
    pub TargetSOC_isUsed: [u8; 1],
    pub EVMaximumDischargePower: iso20_dc_RationalNumberType,
    pub EVMinimumDischargePower: iso20_dc_RationalNumberType,
    pub EVMaximumDischargeCurrent: iso20_dc_RationalNumberType,
    pub EVMinimumDischargeCurrent: iso20_dc_RationalNumberType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_dc_DC_ChargeParameterDiscoveryReqType {
    pub Header: iso20_dc_MessageHeaderType,
    pub BPT_DC_CPDReqEnergyTransferMode: iso20_dc_BPT_DC_CPDReqEnergyTransferModeType,
    #[bitfield(
        name = "BPT_DC_CPDReqEnergyTransferMode_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub BPT_DC_CPDReqEnergyTransferMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub DC_CPDReqEnergyTransferMode: iso20_dc_DC_CPDReqEnergyTransferModeType,
    #[bitfield(
        name = "DC_CPDReqEnergyTransferMode_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub DC_CPDReqEnergyTransferMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_dc_BPT_DC_CPDResEnergyTransferModeType {
    pub EVSEMaximumChargePower: iso20_dc_RationalNumberType,
    pub EVSEMinimumChargePower: iso20_dc_RationalNumberType,
    pub EVSEMaximumChargeCurrent: iso20_dc_RationalNumberType,
    pub EVSEMinimumChargeCurrent: iso20_dc_RationalNumberType,
    pub EVSEMaximumVoltage: iso20_dc_RationalNumberType,
    pub EVSEMinimumVoltage: iso20_dc_RationalNumberType,
    pub EVSEPowerRampLimitation: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVSEPowerRampLimitation_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEPowerRampLimitation_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub EVSEMaximumDischargePower: iso20_dc_RationalNumberType,
    pub EVSEMinimumDischargePower: iso20_dc_RationalNumberType,
    pub EVSEMaximumDischargeCurrent: iso20_dc_RationalNumberType,
    pub EVSEMinimumDischargeCurrent: iso20_dc_RationalNumberType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_dc_DC_ChargeParameterDiscoveryResType {
    pub Header: iso20_dc_MessageHeaderType,
    pub ResponseCode: iso20_dc_responseCodeType,
    pub BPT_DC_CPDResEnergyTransferMode: iso20_dc_BPT_DC_CPDResEnergyTransferModeType,
    #[bitfield(
        name = "BPT_DC_CPDResEnergyTransferMode_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub BPT_DC_CPDResEnergyTransferMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub DC_CPDResEnergyTransferMode: iso20_dc_DC_CPDResEnergyTransferModeType,
    #[bitfield(
        name = "DC_CPDResEnergyTransferMode_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub DC_CPDResEnergyTransferMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 7],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_dc_BPT_Scheduled_DC_CLReqControlModeType {
    pub EVTargetEnergyRequest: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVTargetEnergyRequest_isUsed", ty = "u32", bits = "0..=0")]
    pub EVTargetEnergyRequest_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub EVMaximumEnergyRequest: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVMaximumEnergyRequest_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumEnergyRequest_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub EVMinimumEnergyRequest: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVMinimumEnergyRequest_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumEnergyRequest_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub EVTargetCurrent: iso20_dc_RationalNumberType,
    pub EVTargetVoltage: iso20_dc_RationalNumberType,
    pub EVMaximumChargePower: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVMaximumChargePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumChargePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub EVMinimumChargePower: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVMinimumChargePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumChargePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 1],
    pub EVMaximumChargeCurrent: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVMaximumChargeCurrent_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumChargeCurrent_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 1],
    pub EVMaximumVoltage: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVMaximumVoltage_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumVoltage_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_5: [u8; 1],
    pub EVMinimumVoltage: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVMinimumVoltage_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumVoltage_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_6: [u8; 1],
    pub EVMaximumDischargePower: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVMaximumDischargePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumDischargePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_7: [u8; 1],
    pub EVMinimumDischargePower: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVMinimumDischargePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumDischargePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_8: [u8; 1],
    pub EVMaximumDischargeCurrent: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVMaximumDischargeCurrent_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumDischargeCurrent_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_9: [u8; 3],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_dc_BPT_Scheduled_DC_CLResControlModeType {
    pub EVSEMaximumChargePower: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVSEMaximumChargePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEMaximumChargePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub EVSEMinimumChargePower: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVSEMinimumChargePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEMinimumChargePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub EVSEMaximumChargeCurrent: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVSEMaximumChargeCurrent_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEMaximumChargeCurrent_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub EVSEMaximumVoltage: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVSEMaximumVoltage_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEMaximumVoltage_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub EVSEMaximumDischargePower: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVSEMaximumDischargePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEMaximumDischargePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 1],
    pub EVSEMinimumDischargePower: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVSEMinimumDischargePower_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEMinimumDischargePower_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 1],
    pub EVSEMaximumDischargeCurrent: iso20_dc_RationalNumberType,
    #[bitfield(
        name = "EVSEMaximumDischargeCurrent_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub EVSEMaximumDischargeCurrent_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_5: [u8; 1],
    pub EVSEMinimumVoltage: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVSEMinimumVoltage_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEMinimumVoltage_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_6: [u8; 1],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_dc_BPT_Dynamic_DC_CLReqControlModeType {
    pub DepartureTime: u32,
    #[bitfield(name = "DepartureTime_isUsed", ty = "u32", bits = "0..=0")]
    pub DepartureTime_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub EVTargetEnergyRequest: iso20_dc_RationalNumberType,
    pub EVMaximumEnergyRequest: iso20_dc_RationalNumberType,
    pub EVMinimumEnergyRequest: iso20_dc_RationalNumberType,
    pub EVMaximumChargePower: iso20_dc_RationalNumberType,
    pub EVMinimumChargePower: iso20_dc_RationalNumberType,
    pub EVMaximumChargeCurrent: iso20_dc_RationalNumberType,
    pub EVMaximumVoltage: iso20_dc_RationalNumberType,
    pub EVMinimumVoltage: iso20_dc_RationalNumberType,
    pub EVMaximumDischargePower: iso20_dc_RationalNumberType,
    pub EVMinimumDischargePower: iso20_dc_RationalNumberType,
    pub EVMaximumDischargeCurrent: iso20_dc_RationalNumberType,
    pub EVMaximumV2XEnergyRequest: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVMaximumV2XEnergyRequest_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumV2XEnergyRequest_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub EVMinimumV2XEnergyRequest: iso20_dc_RationalNumberType,
    #[bitfield(name = "EVMinimumV2XEnergyRequest_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMinimumV2XEnergyRequest_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 3],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_dc_DC_ChargeLoopReqType {
    pub Header: iso20_dc_MessageHeaderType,
    pub DisplayParameters: iso20_dc_DisplayParametersType,
    #[bitfield(name = "DisplayParameters_isUsed", ty = "u32", bits = "0..=0")]
    pub DisplayParameters_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub MeterInfoRequested: i32,
    pub EVPresentVoltage: iso20_dc_RationalNumberType,
    pub BPT_Dynamic_DC_CLReqControlMode: iso20_dc_BPT_Dynamic_DC_CLReqControlModeType,
    #[bitfield(
        name = "BPT_Dynamic_DC_CLReqControlMode_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub BPT_Dynamic_DC_CLReqControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
    pub BPT_Scheduled_DC_CLReqControlMode: iso20_dc_BPT_Scheduled_DC_CLReqControlModeType,
    #[bitfield(
        name = "BPT_Scheduled_DC_CLReqControlMode_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub BPT_Scheduled_DC_CLReqControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 3],
    pub CLReqControlMode: iso20_dc_CLReqControlModeType,
    #[bitfield(name = "CLReqControlMode_isUsed", ty = "u32", bits = "0..=0")]
    pub CLReqControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 3],
    pub Dynamic_DC_CLReqControlMode: iso20_dc_Dynamic_DC_CLReqControlModeType,
    #[bitfield(
        name = "Dynamic_DC_CLReqControlMode_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub Dynamic_DC_CLReqControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 3],
    pub Scheduled_DC_CLReqControlMode: iso20_dc_Scheduled_DC_CLReqControlModeType,
    #[bitfield(
        name = "Scheduled_DC_CLReqControlMode_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub Scheduled_DC_CLReqControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 7],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_dc_BPT_Dynamic_DC_CLResControlModeType {
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
    pub EVSEMaximumChargePower: iso20_dc_RationalNumberType,
    pub EVSEMinimumChargePower: iso20_dc_RationalNumberType,
    pub EVSEMaximumChargeCurrent: iso20_dc_RationalNumberType,
    pub EVSEMaximumVoltage: iso20_dc_RationalNumberType,
    pub EVSEMaximumDischargePower: iso20_dc_RationalNumberType,
    pub EVSEMinimumDischargePower: iso20_dc_RationalNumberType,
    pub EVSEMaximumDischargeCurrent: iso20_dc_RationalNumberType,
    pub EVSEMinimumVoltage: iso20_dc_RationalNumberType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_dc_DC_ChargeLoopResType {
    pub Header: iso20_dc_MessageHeaderType,
    pub ResponseCode: iso20_dc_responseCodeType,
    pub EVSEStatus: iso20_dc_EVSEStatusType,
    #[bitfield(name = "EVSEStatus_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEStatus_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub MeterInfo: iso20_dc_MeterInfoType,
    #[bitfield(name = "MeterInfo_isUsed", ty = "u32", bits = "0..=0")]
    pub MeterInfo_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 7],
    pub Receipt: iso20_dc_ReceiptType,
    #[bitfield(name = "Receipt_isUsed", ty = "u32", bits = "0..=0")]
    pub Receipt_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub EVSEPresentCurrent: iso20_dc_RationalNumberType,
    pub EVSEPresentVoltage: iso20_dc_RationalNumberType,
    pub EVSEPowerLimitAchieved: i32,
    pub EVSECurrentLimitAchieved: i32,
    pub EVSEVoltageLimitAchieved: i32,
    pub BPT_Dynamic_DC_CLResControlMode: iso20_dc_BPT_Dynamic_DC_CLResControlModeType,
    #[bitfield(
        name = "BPT_Dynamic_DC_CLResControlMode_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub BPT_Dynamic_DC_CLResControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 3],
    pub BPT_Scheduled_DC_CLResControlMode: iso20_dc_BPT_Scheduled_DC_CLResControlModeType,
    #[bitfield(
        name = "BPT_Scheduled_DC_CLResControlMode_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub BPT_Scheduled_DC_CLResControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 3],
    pub CLResControlMode: iso20_dc_CLResControlModeType,
    #[bitfield(name = "CLResControlMode_isUsed", ty = "u32", bits = "0..=0")]
    pub CLResControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 3],
    pub Dynamic_DC_CLResControlMode: iso20_dc_Dynamic_DC_CLResControlModeType,
    #[bitfield(
        name = "Dynamic_DC_CLResControlMode_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub Dynamic_DC_CLResControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_5: [u8; 3],
    pub Scheduled_DC_CLResControlMode: iso20_dc_Scheduled_DC_CLResControlModeType,
    #[bitfield(
        name = "Scheduled_DC_CLResControlMode_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub Scheduled_DC_CLResControlMode_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_6: [u8; 3],
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_dc_ManifestType {
    pub Id: C2RustUnnamed_61,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub Reference: C2RustUnnamed_60,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_60 {
    pub array: [iso20_dc_ReferenceType; 4],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_61 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_dc_SignaturePropertiesType {
    pub Id: C2RustUnnamed_62,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub SignatureProperty: iso20_dc_SignaturePropertyType,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_62 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_dc_exiDocument {
    pub c2rust_unnamed: C2RustUnnamed_63,
    #[bitfield(
        name = "DC_ChargeParameterDiscoveryReq_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    #[bitfield(
        name = "DC_ChargeParameterDiscoveryRes_isUsed",
        ty = "u32",
        bits = "1..=1"
    )]
    #[bitfield(name = "DC_CableCheckReq_isUsed", ty = "u32", bits = "2..=2")]
    #[bitfield(name = "DC_CableCheckRes_isUsed", ty = "u32", bits = "3..=3")]
    #[bitfield(name = "DC_PreChargeReq_isUsed", ty = "u32", bits = "4..=4")]
    #[bitfield(name = "DC_PreChargeRes_isUsed", ty = "u32", bits = "5..=5")]
    #[bitfield(name = "DC_ChargeLoopReq_isUsed", ty = "u32", bits = "6..=6")]
    #[bitfield(name = "DC_ChargeLoopRes_isUsed", ty = "u32", bits = "7..=7")]
    #[bitfield(name = "DC_WeldingDetectionReq_isUsed", ty = "u32", bits = "8..=8")]
    #[bitfield(name = "DC_WeldingDetectionRes_isUsed", ty = "u32", bits = "9..=9")]
    #[bitfield(
        name = "DC_CPDReqEnergyTransferMode_isUsed",
        ty = "u32",
        bits = "10..=10"
    )]
    #[bitfield(
        name = "DC_CPDResEnergyTransferMode_isUsed",
        ty = "u32",
        bits = "11..=11"
    )]
    #[bitfield(
        name = "BPT_DC_CPDReqEnergyTransferMode_isUsed",
        ty = "u32",
        bits = "12..=12"
    )]
    #[bitfield(
        name = "BPT_DC_CPDResEnergyTransferMode_isUsed",
        ty = "u32",
        bits = "13..=13"
    )]
    #[bitfield(
        name = "Scheduled_DC_CLReqControlMode_isUsed",
        ty = "u32",
        bits = "14..=14"
    )]
    #[bitfield(
        name = "Scheduled_DC_CLResControlMode_isUsed",
        ty = "u32",
        bits = "15..=15"
    )]
    #[bitfield(
        name = "BPT_Scheduled_DC_CLReqControlMode_isUsed",
        ty = "u32",
        bits = "16..=16"
    )]
    #[bitfield(
        name = "BPT_Scheduled_DC_CLResControlMode_isUsed",
        ty = "u32",
        bits = "17..=17"
    )]
    #[bitfield(
        name = "Dynamic_DC_CLReqControlMode_isUsed",
        ty = "u32",
        bits = "18..=18"
    )]
    #[bitfield(
        name = "Dynamic_DC_CLResControlMode_isUsed",
        ty = "u32",
        bits = "19..=19"
    )]
    #[bitfield(
        name = "BPT_Dynamic_DC_CLReqControlMode_isUsed",
        ty = "u32",
        bits = "20..=20"
    )]
    #[bitfield(
        name = "BPT_Dynamic_DC_CLResControlMode_isUsed",
        ty = "u32",
        bits = "21..=21"
    )]
    #[bitfield(name = "CLReqControlMode_isUsed", ty = "u32", bits = "22..=22")]
    #[bitfield(name = "CLResControlMode_isUsed", ty = "u32", bits = "23..=23")]
    #[bitfield(name = "Signature_isUsed", ty = "u32", bits = "24..=24")]
    #[bitfield(name = "SignatureValue_isUsed", ty = "u32", bits = "25..=25")]
    #[bitfield(name = "SignedInfo_isUsed", ty = "u32", bits = "26..=26")]
    #[bitfield(name = "CanonicalizationMethod_isUsed", ty = "u32", bits = "27..=27")]
    #[bitfield(name = "SignatureMethod_isUsed", ty = "u32", bits = "28..=28")]
    #[bitfield(name = "Reference_isUsed", ty = "u32", bits = "29..=29")]
    #[bitfield(name = "Transforms_isUsed", ty = "u32", bits = "30..=30")]
    #[bitfield(name = "Transform_isUsed", ty = "u32", bits = "31..=31")]
    #[bitfield(name = "DigestMethod_isUsed", ty = "u32", bits = "32..=32")]
    #[bitfield(name = "KeyInfo_isUsed", ty = "u32", bits = "33..=33")]
    #[bitfield(name = "KeyValue_isUsed", ty = "u32", bits = "34..=34")]
    #[bitfield(name = "RetrievalMethod_isUsed", ty = "u32", bits = "35..=35")]
    #[bitfield(name = "X509Data_isUsed", ty = "u32", bits = "36..=36")]
    #[bitfield(name = "PGPData_isUsed", ty = "u32", bits = "37..=37")]
    #[bitfield(name = "SPKIData_isUsed", ty = "u32", bits = "38..=38")]
    #[bitfield(name = "Object_isUsed", ty = "u32", bits = "39..=39")]
    #[bitfield(name = "Manifest_isUsed", ty = "u32", bits = "40..=40")]
    #[bitfield(name = "SignatureProperties_isUsed", ty = "u32", bits = "41..=41")]
    #[bitfield(name = "SignatureProperty_isUsed", ty = "u32", bits = "42..=42")]
    #[bitfield(name = "DSAKeyValue_isUsed", ty = "u32", bits = "43..=43")]
    #[bitfield(name = "RSAKeyValue_isUsed", ty = "u32", bits = "44..=44")]
    pub DC_ChargeParameterDiscoveryReq_isUsed_DC_ChargeParameterDiscoveryRes_isUsed_DC_CableCheckReq_isUsed_DC_CableCheckRes_isUsed_DC_PreChargeReq_isUsed_DC_PreChargeRes_isUsed_DC_ChargeLoopReq_isUsed_DC_ChargeLoopRes_isUsed_DC_WeldingDetectionReq_isUsed_DC_WeldingDetectionRes_isUsed_DC_CPDReqEnergyTransferMode_isUsed_DC_CPDResEnergyTransferMode_isUsed_BPT_DC_CPDReqEnergyTransferMode_isUsed_BPT_DC_CPDResEnergyTransferMode_isUsed_Scheduled_DC_CLReqControlMode_isUsed_Scheduled_DC_CLResControlMode_isUsed_BPT_Scheduled_DC_CLReqControlMode_isUsed_BPT_Scheduled_DC_CLResControlMode_isUsed_Dynamic_DC_CLReqControlMode_isUsed_Dynamic_DC_CLResControlMode_isUsed_BPT_Dynamic_DC_CLReqControlMode_isUsed_BPT_Dynamic_DC_CLResControlMode_isUsed_CLReqControlMode_isUsed_CLResControlMode_isUsed_Signature_isUsed_SignatureValue_isUsed_SignedInfo_isUsed_CanonicalizationMethod_isUsed_SignatureMethod_isUsed_Reference_isUsed_Transforms_isUsed_Transform_isUsed_DigestMethod_isUsed_KeyInfo_isUsed_KeyValue_isUsed_RetrievalMethod_isUsed_X509Data_isUsed_PGPData_isUsed_SPKIData_isUsed_Object_isUsed_Manifest_isUsed_SignatureProperties_isUsed_SignatureProperty_isUsed_DSAKeyValue_isUsed_RSAKeyValue_isUsed:
        [u8; 6],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
}
#[derive(Copy, Clone)]

pub union C2RustUnnamed_63 {
    pub DC_ChargeParameterDiscoveryReq: iso20_dc_DC_ChargeParameterDiscoveryReqType,
    pub DC_ChargeParameterDiscoveryRes: iso20_dc_DC_ChargeParameterDiscoveryResType,
    pub DC_CableCheckReq: iso20_dc_DC_CableCheckReqType,
    pub DC_CableCheckRes: iso20_dc_DC_CableCheckResType,
    pub DC_PreChargeReq: iso20_dc_DC_PreChargeReqType,
    pub DC_PreChargeRes: iso20_dc_DC_PreChargeResType,
    pub DC_ChargeLoopReq: iso20_dc_DC_ChargeLoopReqType,
    pub DC_ChargeLoopRes: iso20_dc_DC_ChargeLoopResType,
    pub DC_WeldingDetectionReq: iso20_dc_DC_WeldingDetectionReqType,
    pub DC_WeldingDetectionRes: iso20_dc_DC_WeldingDetectionResType,
    pub DC_CPDReqEnergyTransferMode: iso20_dc_DC_CPDReqEnergyTransferModeType,
    pub DC_CPDResEnergyTransferMode: iso20_dc_DC_CPDResEnergyTransferModeType,
    pub BPT_DC_CPDReqEnergyTransferMode: iso20_dc_BPT_DC_CPDReqEnergyTransferModeType,
    pub BPT_DC_CPDResEnergyTransferMode: iso20_dc_BPT_DC_CPDResEnergyTransferModeType,
    pub Scheduled_DC_CLReqControlMode: iso20_dc_Scheduled_DC_CLReqControlModeType,
    pub Scheduled_DC_CLResControlMode: iso20_dc_Scheduled_DC_CLResControlModeType,
    pub BPT_Scheduled_DC_CLReqControlMode: iso20_dc_BPT_Scheduled_DC_CLReqControlModeType,
    pub BPT_Scheduled_DC_CLResControlMode: iso20_dc_BPT_Scheduled_DC_CLResControlModeType,
    pub Dynamic_DC_CLReqControlMode: iso20_dc_Dynamic_DC_CLReqControlModeType,
    pub Dynamic_DC_CLResControlMode: iso20_dc_Dynamic_DC_CLResControlModeType,
    pub BPT_Dynamic_DC_CLReqControlMode: iso20_dc_BPT_Dynamic_DC_CLReqControlModeType,
    pub BPT_Dynamic_DC_CLResControlMode: iso20_dc_BPT_Dynamic_DC_CLResControlModeType,
    pub CLReqControlMode: iso20_dc_CLReqControlModeType,
    pub CLResControlMode: iso20_dc_CLResControlModeType,
    pub Signature: iso20_dc_SignatureType,
    pub SignatureValue: iso20_dc_SignatureValueType,
    pub SignedInfo: iso20_dc_SignedInfoType,
    pub CanonicalizationMethod: iso20_dc_CanonicalizationMethodType,
    pub SignatureMethod: iso20_dc_SignatureMethodType,
    pub Reference: iso20_dc_ReferenceType,
    pub Transforms: iso20_dc_TransformsType,
    pub Transform: iso20_dc_TransformType,
    pub DigestMethod: iso20_dc_DigestMethodType,
    pub KeyInfo: iso20_dc_KeyInfoType,
    pub KeyValue: iso20_dc_KeyValueType,
    pub RetrievalMethod: iso20_dc_RetrievalMethodType,
    pub X509Data: iso20_dc_X509DataType,
    pub PGPData: iso20_dc_PGPDataType,
    pub SPKIData: iso20_dc_SPKIDataType,
    pub Object: iso20_dc_ObjectType,
    pub Manifest: iso20_dc_ManifestType,
    pub SignatureProperties: iso20_dc_SignaturePropertiesType,
    pub SignatureProperty: iso20_dc_SignaturePropertyType,
    pub DSAKeyValue: iso20_dc_DSAKeyValueType,
    pub RSAKeyValue: iso20_dc_RSAKeyValueType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_dc_exiFragment {
    pub c2rust_unnamed: C2RustUnnamed_64,
    #[bitfield(
        name = "DC_ChargeParameterDiscoveryRes_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    #[bitfield(name = "SignedInfo_isUsed", ty = "u32", bits = "1..=1")]
    pub DC_ChargeParameterDiscoveryRes_isUsed_SignedInfo_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]

pub union C2RustUnnamed_64 {
    pub DC_ChargeParameterDiscoveryRes: iso20_dc_DC_ChargeParameterDiscoveryResType,
    pub SignedInfo: iso20_dc_SignedInfoType,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct iso20_dc_xmldsigFragment {
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
    pub CanonicalizationMethod: iso20_dc_CanonicalizationMethodType,
    pub DSAKeyValue: iso20_dc_DSAKeyValueType,
    pub DigestMethod: iso20_dc_DigestMethodType,
    pub KeyInfo: iso20_dc_KeyInfoType,
    pub KeyValue: iso20_dc_KeyValueType,
    pub Manifest: iso20_dc_ManifestType,
    pub Object: iso20_dc_ObjectType,
    pub PGPData: iso20_dc_PGPDataType,
    pub RSAKeyValue: iso20_dc_RSAKeyValueType,
    pub Reference: iso20_dc_ReferenceType,
    pub RetrievalMethod: iso20_dc_RetrievalMethodType,
    pub SPKIData: iso20_dc_SPKIDataType,
    pub Signature: iso20_dc_SignatureType,
    pub SignatureMethod: iso20_dc_SignatureMethodType,
    pub SignatureProperties: iso20_dc_SignaturePropertiesType,
    pub SignatureProperty: iso20_dc_SignaturePropertyType,
    pub SignatureValue: iso20_dc_SignatureValueType,
    pub SignedInfo: iso20_dc_SignedInfoType,
    pub Transform: iso20_dc_TransformType,
    pub Transforms: iso20_dc_TransformsType,
    pub X509Data: iso20_dc_X509DataType,
    pub X509IssuerSerial: iso20_dc_X509IssuerSerialType,
}

pub unsafe extern "C" fn init_iso20_dc_exiDocument(mut exiDoc: *mut iso20_dc_exiDocument) {
    (*exiDoc).set_DC_ChargeParameterDiscoveryReq_isUsed(0 as u32);
    (*exiDoc).set_DC_ChargeParameterDiscoveryRes_isUsed(0 as u32);
    (*exiDoc).set_DC_CableCheckReq_isUsed(0 as u32);
    (*exiDoc).set_DC_CableCheckRes_isUsed(0 as u32);
    (*exiDoc).set_DC_PreChargeReq_isUsed(0 as u32);
    (*exiDoc).set_DC_PreChargeRes_isUsed(0 as u32);
    (*exiDoc).set_DC_ChargeLoopReq_isUsed(0 as u32);
    (*exiDoc).set_DC_ChargeLoopRes_isUsed(0 as u32);
    (*exiDoc).set_DC_WeldingDetectionReq_isUsed(0 as u32);
    (*exiDoc).set_DC_WeldingDetectionRes_isUsed(0 as u32);
    (*exiDoc).set_DC_CPDReqEnergyTransferMode_isUsed(0 as u32);
    (*exiDoc).set_DC_CPDResEnergyTransferMode_isUsed(0 as u32);
    (*exiDoc).set_BPT_DC_CPDReqEnergyTransferMode_isUsed(0 as u32);
    (*exiDoc).set_BPT_DC_CPDResEnergyTransferMode_isUsed(0 as u32);
    (*exiDoc).set_Scheduled_DC_CLReqControlMode_isUsed(0 as u32);
    (*exiDoc).set_Scheduled_DC_CLResControlMode_isUsed(0 as u32);
    (*exiDoc).set_BPT_Scheduled_DC_CLReqControlMode_isUsed(0 as u32);
    (*exiDoc).set_BPT_Scheduled_DC_CLResControlMode_isUsed(0 as u32);
    (*exiDoc).set_Dynamic_DC_CLReqControlMode_isUsed(0 as u32);
    (*exiDoc).set_Dynamic_DC_CLResControlMode_isUsed(0 as u32);
    (*exiDoc).set_BPT_Dynamic_DC_CLReqControlMode_isUsed(0 as u32);
    (*exiDoc).set_BPT_Dynamic_DC_CLResControlMode_isUsed(0 as u32);
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

pub unsafe extern "C" fn init_iso20_dc_TransformType(
    mut TransformType: *mut iso20_dc_TransformType,
) {
    (*TransformType).set_ANY_isUsed(0 as u32);
    (*TransformType).set_XPath_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_TransformsType(
    mut TransformsType: *mut iso20_dc_TransformsType,
) {
}

pub unsafe extern "C" fn init_iso20_dc_DSAKeyValueType(
    mut DSAKeyValueType: *mut iso20_dc_DSAKeyValueType,
) {
    (*DSAKeyValueType).set_P_isUsed(0 as u32);
    (*DSAKeyValueType).set_Q_isUsed(0 as u32);
    (*DSAKeyValueType).set_G_isUsed(0 as u32);
    (*DSAKeyValueType).set_J_isUsed(0 as u32);
    (*DSAKeyValueType).set_Seed_isUsed(0 as u32);
    (*DSAKeyValueType).set_PgenCounter_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_X509IssuerSerialType(
    mut X509IssuerSerialType: *mut iso20_dc_X509IssuerSerialType,
) {
}

pub unsafe extern "C" fn init_iso20_dc_DigestMethodType(
    mut DigestMethodType: *mut iso20_dc_DigestMethodType,
) {
    (*DigestMethodType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_RSAKeyValueType(
    mut RSAKeyValueType: *mut iso20_dc_RSAKeyValueType,
) {
}

pub unsafe extern "C" fn init_iso20_dc_CanonicalizationMethodType(
    mut CanonicalizationMethodType: *mut iso20_dc_CanonicalizationMethodType,
) {
    (*CanonicalizationMethodType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_SignatureMethodType(
    mut SignatureMethodType: *mut iso20_dc_SignatureMethodType,
) {
    (*SignatureMethodType).set_HMACOutputLength_isUsed(0 as u32);
    (*SignatureMethodType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_KeyValueType(mut KeyValueType: *mut iso20_dc_KeyValueType) {
    (*KeyValueType).set_DSAKeyValue_isUsed(0 as u32);
    (*KeyValueType).set_RSAKeyValue_isUsed(0 as u32);
    (*KeyValueType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_ReferenceType(
    mut ReferenceType: *mut iso20_dc_ReferenceType,
) {
    (*ReferenceType).set_Id_isUsed(0 as u32);
    (*ReferenceType).set_Type_isUsed(0 as u32);
    (*ReferenceType).set_URI_isUsed(0 as u32);
    (*ReferenceType).set_Transforms_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_RetrievalMethodType(
    mut RetrievalMethodType: *mut iso20_dc_RetrievalMethodType,
) {
    (*RetrievalMethodType).set_Type_isUsed(0 as u32);
    (*RetrievalMethodType).set_URI_isUsed(0 as u32);
    (*RetrievalMethodType).set_Transforms_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_X509DataType(mut X509DataType: *mut iso20_dc_X509DataType) {
    (*X509DataType).set_X509IssuerSerial_isUsed(0 as u32);
    (*X509DataType).set_X509SKI_isUsed(0 as u32);
    (*X509DataType).set_X509SubjectName_isUsed(0 as u32);
    (*X509DataType).set_X509Certificate_isUsed(0 as u32);
    (*X509DataType).set_X509CRL_isUsed(0 as u32);
    (*X509DataType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_PGPDataType(mut PGPDataType: *mut iso20_dc_PGPDataType) {
    ((*PGPDataType).c2rust_unnamed).choice_1_isUsed = 0 as u32;
    ((*PGPDataType).c2rust_unnamed).choice_2_isUsed = 0 as u32;
}

pub unsafe extern "C" fn init_iso20_dc_SPKIDataType(mut SPKIDataType: *mut iso20_dc_SPKIDataType) {
    (*SPKIDataType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_SignedInfoType(
    mut SignedInfoType: *mut iso20_dc_SignedInfoType,
) {
    (*SignedInfoType).Reference.arrayLen = 0 as u32 as u16;
    (*SignedInfoType).set_Id_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_SignatureValueType(
    mut SignatureValueType: *mut iso20_dc_SignatureValueType,
) {
    (*SignatureValueType).set_Id_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_KeyInfoType(mut KeyInfoType: *mut iso20_dc_KeyInfoType) {
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

pub unsafe extern "C" fn init_iso20_dc_ObjectType(mut ObjectType: *mut iso20_dc_ObjectType) {
    (*ObjectType).set_Encoding_isUsed(0 as u32);
    (*ObjectType).set_Id_isUsed(0 as u32);
    (*ObjectType).set_MimeType_isUsed(0 as u32);
    (*ObjectType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_RationalNumberType(
    mut RationalNumberType: *mut iso20_dc_RationalNumberType,
) {
}

pub unsafe extern "C" fn init_iso20_dc_DetailedCostType(
    mut DetailedCostType: *mut iso20_dc_DetailedCostType,
) {
}

pub unsafe extern "C" fn init_iso20_dc_SignatureType(
    mut SignatureType: *mut iso20_dc_SignatureType,
) {
    (*SignatureType).set_Id_isUsed(0 as u32);
    (*SignatureType).set_KeyInfo_isUsed(0 as u32);
    (*SignatureType).set_Object_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_DetailedTaxType(
    mut DetailedTaxType: *mut iso20_dc_DetailedTaxType,
) {
}

pub unsafe extern "C" fn init_iso20_dc_MessageHeaderType(
    mut MessageHeaderType: *mut iso20_dc_MessageHeaderType,
) {
    (*MessageHeaderType).set_Signature_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_SignaturePropertyType(
    mut SignaturePropertyType: *mut iso20_dc_SignaturePropertyType,
) {
    (*SignaturePropertyType).set_Id_isUsed(0 as u32);
    (*SignaturePropertyType).set_ANY_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_DC_CPDReqEnergyTransferModeType(
    mut DC_CPDReqEnergyTransferModeType: *mut iso20_dc_DC_CPDReqEnergyTransferModeType,
) {
    (*DC_CPDReqEnergyTransferModeType).set_TargetSOC_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_DisplayParametersType(
    mut DisplayParametersType: *mut iso20_dc_DisplayParametersType,
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

pub unsafe extern "C" fn init_iso20_dc_DC_CPDResEnergyTransferModeType(
    mut DC_CPDResEnergyTransferModeType: *mut iso20_dc_DC_CPDResEnergyTransferModeType,
) {
    (*DC_CPDResEnergyTransferModeType).set_EVSEPowerRampLimitation_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_EVSEStatusType(
    mut EVSEStatusType: *mut iso20_dc_EVSEStatusType,
) {
}

pub unsafe extern "C" fn init_iso20_dc_MeterInfoType(
    mut MeterInfoType: *mut iso20_dc_MeterInfoType,
) {
    (*MeterInfoType).set_BPT_DischargedEnergyReadingWh_isUsed(0 as u32);
    (*MeterInfoType).set_CapacitiveEnergyReadingVARh_isUsed(0 as u32);
    (*MeterInfoType).set_BPT_InductiveEnergyReadingVARh_isUsed(0 as u32);
    (*MeterInfoType).set_MeterSignature_isUsed(0 as u32);
    (*MeterInfoType).set_MeterStatus_isUsed(0 as u32);
    (*MeterInfoType).set_MeterTimestamp_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_Dynamic_DC_CLReqControlModeType(
    mut Dynamic_DC_CLReqControlModeType: *mut iso20_dc_Dynamic_DC_CLReqControlModeType,
) {
    (*Dynamic_DC_CLReqControlModeType).set_DepartureTime_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_Scheduled_DC_CLReqControlModeType(
    mut Scheduled_DC_CLReqControlModeType: *mut iso20_dc_Scheduled_DC_CLReqControlModeType,
) {
    (*Scheduled_DC_CLReqControlModeType).set_EVTargetEnergyRequest_isUsed(0 as u32);
    (*Scheduled_DC_CLReqControlModeType).set_EVMaximumEnergyRequest_isUsed(0 as u32);
    (*Scheduled_DC_CLReqControlModeType).set_EVMinimumEnergyRequest_isUsed(0 as u32);
    (*Scheduled_DC_CLReqControlModeType).set_EVMaximumChargePower_isUsed(0 as u32);
    (*Scheduled_DC_CLReqControlModeType).set_EVMinimumChargePower_isUsed(0 as u32);
    (*Scheduled_DC_CLReqControlModeType).set_EVMaximumChargeCurrent_isUsed(0 as u32);
    (*Scheduled_DC_CLReqControlModeType).set_EVMaximumVoltage_isUsed(0 as u32);
    (*Scheduled_DC_CLReqControlModeType).set_EVMinimumVoltage_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_CLReqControlModeType(
    mut CLReqControlModeType: *mut iso20_dc_CLReqControlModeType,
) {
}

pub unsafe extern "C" fn init_iso20_dc_ReceiptType(mut ReceiptType: *mut iso20_dc_ReceiptType) {
    (*ReceiptType).TaxCosts.arrayLen = 0 as u32 as u16;
    (*ReceiptType).set_EnergyCosts_isUsed(0 as u32);
    (*ReceiptType).set_OccupancyCosts_isUsed(0 as u32);
    (*ReceiptType).set_AdditionalServicesCosts_isUsed(0 as u32);
    (*ReceiptType).set_OverstayCosts_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_Dynamic_DC_CLResControlModeType(
    mut Dynamic_DC_CLResControlModeType: *mut iso20_dc_Dynamic_DC_CLResControlModeType,
) {
    (*Dynamic_DC_CLResControlModeType).set_DepartureTime_isUsed(0 as u32);
    (*Dynamic_DC_CLResControlModeType).set_MinimumSOC_isUsed(0 as u32);
    (*Dynamic_DC_CLResControlModeType).set_TargetSOC_isUsed(0 as u32);
    (*Dynamic_DC_CLResControlModeType).set_AckMaxDelay_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_Scheduled_DC_CLResControlModeType(
    mut Scheduled_DC_CLResControlModeType: *mut iso20_dc_Scheduled_DC_CLResControlModeType,
) {
    (*Scheduled_DC_CLResControlModeType).set_EVSEMaximumChargePower_isUsed(0 as u32);
    (*Scheduled_DC_CLResControlModeType).set_EVSEMinimumChargePower_isUsed(0 as u32);
    (*Scheduled_DC_CLResControlModeType).set_EVSEMaximumChargeCurrent_isUsed(0 as u32);
    (*Scheduled_DC_CLResControlModeType).set_EVSEMaximumVoltage_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_CLResControlModeType(
    mut CLResControlModeType: *mut iso20_dc_CLResControlModeType,
) {
}

pub unsafe extern "C" fn init_iso20_dc_DC_ChargeParameterDiscoveryReqType(
    mut DC_ChargeParameterDiscoveryReqType: *mut iso20_dc_DC_ChargeParameterDiscoveryReqType,
) {
    (*DC_ChargeParameterDiscoveryReqType).set_BPT_DC_CPDReqEnergyTransferMode_isUsed(0 as u32);
    (*DC_ChargeParameterDiscoveryReqType).set_DC_CPDReqEnergyTransferMode_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_DC_ChargeParameterDiscoveryResType(
    mut DC_ChargeParameterDiscoveryResType: *mut iso20_dc_DC_ChargeParameterDiscoveryResType,
) {
    (*DC_ChargeParameterDiscoveryResType).set_BPT_DC_CPDResEnergyTransferMode_isUsed(0 as u32);
    (*DC_ChargeParameterDiscoveryResType).set_DC_CPDResEnergyTransferMode_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_DC_CableCheckReqType(
    mut DC_CableCheckReqType: *mut iso20_dc_DC_CableCheckReqType,
) {
}

pub unsafe extern "C" fn init_iso20_dc_DC_CableCheckResType(
    mut DC_CableCheckResType: *mut iso20_dc_DC_CableCheckResType,
) {
}

pub unsafe extern "C" fn init_iso20_dc_DC_PreChargeReqType(
    mut DC_PreChargeReqType: *mut iso20_dc_DC_PreChargeReqType,
) {
}

pub unsafe extern "C" fn init_iso20_dc_DC_PreChargeResType(
    mut DC_PreChargeResType: *mut iso20_dc_DC_PreChargeResType,
) {
}

pub unsafe extern "C" fn init_iso20_dc_DC_ChargeLoopReqType(
    mut DC_ChargeLoopReqType: *mut iso20_dc_DC_ChargeLoopReqType,
) {
    (*DC_ChargeLoopReqType).set_DisplayParameters_isUsed(0 as u32);
    (*DC_ChargeLoopReqType).set_BPT_Dynamic_DC_CLReqControlMode_isUsed(0 as u32);
    (*DC_ChargeLoopReqType).set_BPT_Scheduled_DC_CLReqControlMode_isUsed(0 as u32);
    (*DC_ChargeLoopReqType).set_CLReqControlMode_isUsed(0 as u32);
    (*DC_ChargeLoopReqType).set_Dynamic_DC_CLReqControlMode_isUsed(0 as u32);
    (*DC_ChargeLoopReqType).set_Scheduled_DC_CLReqControlMode_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_DC_ChargeLoopResType(
    mut DC_ChargeLoopResType: *mut iso20_dc_DC_ChargeLoopResType,
) {
    (*DC_ChargeLoopResType).set_EVSEStatus_isUsed(0 as u32);
    (*DC_ChargeLoopResType).set_MeterInfo_isUsed(0 as u32);
    (*DC_ChargeLoopResType).set_Receipt_isUsed(0 as u32);
    (*DC_ChargeLoopResType).set_BPT_Dynamic_DC_CLResControlMode_isUsed(0 as u32);
    (*DC_ChargeLoopResType).set_BPT_Scheduled_DC_CLResControlMode_isUsed(0 as u32);
    (*DC_ChargeLoopResType).set_CLResControlMode_isUsed(0 as u32);
    (*DC_ChargeLoopResType).set_Dynamic_DC_CLResControlMode_isUsed(0 as u32);
    (*DC_ChargeLoopResType).set_Scheduled_DC_CLResControlMode_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_DC_WeldingDetectionReqType(
    mut DC_WeldingDetectionReqType: *mut iso20_dc_DC_WeldingDetectionReqType,
) {
}

pub unsafe extern "C" fn init_iso20_dc_DC_WeldingDetectionResType(
    mut DC_WeldingDetectionResType: *mut iso20_dc_DC_WeldingDetectionResType,
) {
}

pub unsafe extern "C" fn init_iso20_dc_BPT_DC_CPDReqEnergyTransferModeType(
    mut BPT_DC_CPDReqEnergyTransferModeType: *mut iso20_dc_BPT_DC_CPDReqEnergyTransferModeType,
) {
    (*BPT_DC_CPDReqEnergyTransferModeType).set_TargetSOC_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_BPT_DC_CPDResEnergyTransferModeType(
    mut BPT_DC_CPDResEnergyTransferModeType: *mut iso20_dc_BPT_DC_CPDResEnergyTransferModeType,
) {
    (*BPT_DC_CPDResEnergyTransferModeType).set_EVSEPowerRampLimitation_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_BPT_Scheduled_DC_CLReqControlModeType(
    mut BPT_Scheduled_DC_CLReqControlModeType: *mut iso20_dc_BPT_Scheduled_DC_CLReqControlModeType,
) {
    (*BPT_Scheduled_DC_CLReqControlModeType).set_EVTargetEnergyRequest_isUsed(0 as u32);
    (*BPT_Scheduled_DC_CLReqControlModeType).set_EVMaximumEnergyRequest_isUsed(0 as u32);
    (*BPT_Scheduled_DC_CLReqControlModeType).set_EVMinimumEnergyRequest_isUsed(0 as u32);
    (*BPT_Scheduled_DC_CLReqControlModeType).set_EVMaximumChargePower_isUsed(0 as u32);
    (*BPT_Scheduled_DC_CLReqControlModeType).set_EVMinimumChargePower_isUsed(0 as u32);
    (*BPT_Scheduled_DC_CLReqControlModeType).set_EVMaximumChargeCurrent_isUsed(0 as u32);
    (*BPT_Scheduled_DC_CLReqControlModeType).set_EVMaximumVoltage_isUsed(0 as u32);
    (*BPT_Scheduled_DC_CLReqControlModeType).set_EVMinimumVoltage_isUsed(0 as u32);
    (*BPT_Scheduled_DC_CLReqControlModeType).set_EVMaximumDischargePower_isUsed(0 as u32);
    (*BPT_Scheduled_DC_CLReqControlModeType).set_EVMinimumDischargePower_isUsed(0 as u32);
    (*BPT_Scheduled_DC_CLReqControlModeType).set_EVMaximumDischargeCurrent_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_BPT_Scheduled_DC_CLResControlModeType(
    mut BPT_Scheduled_DC_CLResControlModeType: *mut iso20_dc_BPT_Scheduled_DC_CLResControlModeType,
) {
    (*BPT_Scheduled_DC_CLResControlModeType).set_EVSEMaximumChargePower_isUsed(0 as u32);
    (*BPT_Scheduled_DC_CLResControlModeType).set_EVSEMinimumChargePower_isUsed(0 as u32);
    (*BPT_Scheduled_DC_CLResControlModeType).set_EVSEMaximumChargeCurrent_isUsed(0 as u32);
    (*BPT_Scheduled_DC_CLResControlModeType).set_EVSEMaximumVoltage_isUsed(0 as u32);
    (*BPT_Scheduled_DC_CLResControlModeType).set_EVSEMaximumDischargePower_isUsed(0 as u32);
    (*BPT_Scheduled_DC_CLResControlModeType).set_EVSEMinimumDischargePower_isUsed(0 as u32);
    (*BPT_Scheduled_DC_CLResControlModeType).set_EVSEMaximumDischargeCurrent_isUsed(0 as u32);
    (*BPT_Scheduled_DC_CLResControlModeType).set_EVSEMinimumVoltage_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_BPT_Dynamic_DC_CLReqControlModeType(
    mut BPT_Dynamic_DC_CLReqControlModeType: *mut iso20_dc_BPT_Dynamic_DC_CLReqControlModeType,
) {
    (*BPT_Dynamic_DC_CLReqControlModeType).set_DepartureTime_isUsed(0 as u32);
    (*BPT_Dynamic_DC_CLReqControlModeType).set_EVMaximumV2XEnergyRequest_isUsed(0 as u32);
    (*BPT_Dynamic_DC_CLReqControlModeType).set_EVMinimumV2XEnergyRequest_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_BPT_Dynamic_DC_CLResControlModeType(
    mut BPT_Dynamic_DC_CLResControlModeType: *mut iso20_dc_BPT_Dynamic_DC_CLResControlModeType,
) {
    (*BPT_Dynamic_DC_CLResControlModeType).set_DepartureTime_isUsed(0 as u32);
    (*BPT_Dynamic_DC_CLResControlModeType).set_MinimumSOC_isUsed(0 as u32);
    (*BPT_Dynamic_DC_CLResControlModeType).set_TargetSOC_isUsed(0 as u32);
    (*BPT_Dynamic_DC_CLResControlModeType).set_AckMaxDelay_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_ManifestType(mut ManifestType: *mut iso20_dc_ManifestType) {
    (*ManifestType).Reference.arrayLen = 0 as u32 as u16;
    (*ManifestType).set_Id_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_SignaturePropertiesType(
    mut SignaturePropertiesType: *mut iso20_dc_SignaturePropertiesType,
) {
    (*SignaturePropertiesType).set_Id_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_exiFragment(mut exiFrag: *mut iso20_dc_exiFragment) {
    (*exiFrag).set_DC_ChargeParameterDiscoveryRes_isUsed(0 as u32);
    (*exiFrag).set_SignedInfo_isUsed(0 as u32);
}

pub unsafe extern "C" fn init_iso20_dc_xmldsigFragment(
    mut xmldsigFrag: *mut iso20_dc_xmldsigFragment,
) {
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
