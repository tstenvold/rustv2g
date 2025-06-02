use ::c2rust_bitfields;
use c2rust_bitfields::*;
extern "C" {
    fn decode_exi_type_hex_binary(
        stream: *mut exi_bitstream_t,
        value_len: *mut u16,
        value_buffer: *mut u8,
        value_buffer_size: usize,
    ) -> i32;
    fn decode_exi_type_integer16(stream: *mut exi_bitstream_t, value: *mut i16) -> i32;
    fn decode_exi_type_uint16(stream: *mut exi_bitstream_t, value: *mut u16) -> i32;
    fn decode_exi_type_uint32(stream: *mut exi_bitstream_t, value: *mut u32) -> i32;
    fn decode_exi_type_uint64(stream: *mut exi_bitstream_t, value: *mut u64) -> i32;
    fn exi_basetypes_decoder_bytes(
        stream: *mut exi_bitstream_t,
        bytes_len: usize,
        bytes: *mut u8,
        bytes_size: usize,
    ) -> i32;
    fn exi_basetypes_decoder_nbit_uint(
        stream: *mut exi_bitstream_t,
        bit_count: usize,
        value: *mut u32,
    ) -> i32;
    fn exi_basetypes_decoder_uint_16(stream: *mut exi_bitstream_t, value: *mut u16) -> i32;
    fn exi_basetypes_decoder_signed(stream: *mut exi_bitstream_t, value: *mut ExiSigned) -> i32;
    fn exi_basetypes_decoder_characters(
        stream: *mut exi_bitstream_t,
        characters_len: usize,
        characters: *mut ExiChar,
        characters_size: usize,
    ) -> i32;
    fn exi_header_read_and_check(stream: *mut exi_bitstream_t) -> i32;
    fn init_iso20_dc_exiDocument(exiDoc: *mut iso20_dc_exiDocument);
    fn init_iso20_dc_DC_ChargeParameterDiscoveryReqType(
        DC_ChargeParameterDiscoveryReq: *mut iso20_dc_DC_ChargeParameterDiscoveryReqType,
    );
    fn init_iso20_dc_DC_ChargeParameterDiscoveryResType(
        DC_ChargeParameterDiscoveryRes: *mut iso20_dc_DC_ChargeParameterDiscoveryResType,
    );
    fn init_iso20_dc_DC_CableCheckReqType(DC_CableCheckReq: *mut iso20_dc_DC_CableCheckReqType);
    fn init_iso20_dc_DC_CableCheckResType(DC_CableCheckRes: *mut iso20_dc_DC_CableCheckResType);
    fn init_iso20_dc_DC_PreChargeReqType(DC_PreChargeReq: *mut iso20_dc_DC_PreChargeReqType);
    fn init_iso20_dc_DC_PreChargeResType(DC_PreChargeRes: *mut iso20_dc_DC_PreChargeResType);
    fn init_iso20_dc_DC_ChargeLoopReqType(DC_ChargeLoopReq: *mut iso20_dc_DC_ChargeLoopReqType);
    fn init_iso20_dc_DC_ChargeLoopResType(DC_ChargeLoopRes: *mut iso20_dc_DC_ChargeLoopResType);
    fn init_iso20_dc_DC_WeldingDetectionReqType(
        DC_WeldingDetectionReq: *mut iso20_dc_DC_WeldingDetectionReqType,
    );
    fn init_iso20_dc_DC_WeldingDetectionResType(
        DC_WeldingDetectionRes: *mut iso20_dc_DC_WeldingDetectionResType,
    );
    fn init_iso20_dc_DC_CPDReqEnergyTransferModeType(
        DC_CPDReqEnergyTransferMode: *mut iso20_dc_DC_CPDReqEnergyTransferModeType,
    );
    fn init_iso20_dc_DC_CPDResEnergyTransferModeType(
        DC_CPDResEnergyTransferMode: *mut iso20_dc_DC_CPDResEnergyTransferModeType,
    );
    fn init_iso20_dc_BPT_DC_CPDReqEnergyTransferModeType(
        BPT_DC_CPDReqEnergyTransferMode: *mut iso20_dc_BPT_DC_CPDReqEnergyTransferModeType,
    );
    fn init_iso20_dc_BPT_DC_CPDResEnergyTransferModeType(
        BPT_DC_CPDResEnergyTransferMode: *mut iso20_dc_BPT_DC_CPDResEnergyTransferModeType,
    );
    fn init_iso20_dc_Scheduled_DC_CLReqControlModeType(
        Scheduled_DC_CLReqControlMode: *mut iso20_dc_Scheduled_DC_CLReqControlModeType,
    );
    fn init_iso20_dc_Scheduled_DC_CLResControlModeType(
        Scheduled_DC_CLResControlMode: *mut iso20_dc_Scheduled_DC_CLResControlModeType,
    );
    fn init_iso20_dc_BPT_Scheduled_DC_CLReqControlModeType(
        BPT_Scheduled_DC_CLReqControlMode: *mut iso20_dc_BPT_Scheduled_DC_CLReqControlModeType,
    );
    fn init_iso20_dc_BPT_Scheduled_DC_CLResControlModeType(
        BPT_Scheduled_DC_CLResControlMode: *mut iso20_dc_BPT_Scheduled_DC_CLResControlModeType,
    );
    fn init_iso20_dc_Dynamic_DC_CLReqControlModeType(
        Dynamic_DC_CLReqControlMode: *mut iso20_dc_Dynamic_DC_CLReqControlModeType,
    );
    fn init_iso20_dc_Dynamic_DC_CLResControlModeType(
        Dynamic_DC_CLResControlMode: *mut iso20_dc_Dynamic_DC_CLResControlModeType,
    );
    fn init_iso20_dc_BPT_Dynamic_DC_CLReqControlModeType(
        BPT_Dynamic_DC_CLReqControlMode: *mut iso20_dc_BPT_Dynamic_DC_CLReqControlModeType,
    );
    fn init_iso20_dc_BPT_Dynamic_DC_CLResControlModeType(
        BPT_Dynamic_DC_CLResControlMode: *mut iso20_dc_BPT_Dynamic_DC_CLResControlModeType,
    );
    fn init_iso20_dc_SignatureType(Signature: *mut iso20_dc_SignatureType);
    fn init_iso20_dc_SignatureValueType(SignatureValue: *mut iso20_dc_SignatureValueType);
    fn init_iso20_dc_SignedInfoType(SignedInfo: *mut iso20_dc_SignedInfoType);
    fn init_iso20_dc_CanonicalizationMethodType(
        CanonicalizationMethod: *mut iso20_dc_CanonicalizationMethodType,
    );
    fn init_iso20_dc_SignatureMethodType(SignatureMethod: *mut iso20_dc_SignatureMethodType);
    fn init_iso20_dc_ReferenceType(Reference: *mut iso20_dc_ReferenceType);
    fn init_iso20_dc_TransformsType(Transforms: *mut iso20_dc_TransformsType);
    fn init_iso20_dc_TransformType(Transform: *mut iso20_dc_TransformType);
    fn init_iso20_dc_DigestMethodType(DigestMethod: *mut iso20_dc_DigestMethodType);
    fn init_iso20_dc_KeyInfoType(KeyInfo: *mut iso20_dc_KeyInfoType);
    fn init_iso20_dc_KeyValueType(KeyValue: *mut iso20_dc_KeyValueType);
    fn init_iso20_dc_RetrievalMethodType(RetrievalMethod: *mut iso20_dc_RetrievalMethodType);
    fn init_iso20_dc_X509DataType(X509Data: *mut iso20_dc_X509DataType);
    fn init_iso20_dc_PGPDataType(PGPData: *mut iso20_dc_PGPDataType);
    fn init_iso20_dc_SPKIDataType(SPKIData: *mut iso20_dc_SPKIDataType);
    fn init_iso20_dc_ObjectType(Object: *mut iso20_dc_ObjectType);
    fn init_iso20_dc_ManifestType(Manifest: *mut iso20_dc_ManifestType);
    fn init_iso20_dc_SignaturePropertiesType(
        SignatureProperties: *mut iso20_dc_SignaturePropertiesType,
    );
    fn init_iso20_dc_SignaturePropertyType(SignatureProperty: *mut iso20_dc_SignaturePropertyType);
    fn init_iso20_dc_DSAKeyValueType(DSAKeyValue: *mut iso20_dc_DSAKeyValueType);
    fn init_iso20_dc_RSAKeyValueType(RSAKeyValue: *mut iso20_dc_RSAKeyValueType);
    fn init_iso20_dc_X509IssuerSerialType(X509IssuerSerialType: *mut iso20_dc_X509IssuerSerialType);
    fn init_iso20_dc_RationalNumberType(RationalNumberType: *mut iso20_dc_RationalNumberType);
    fn init_iso20_dc_DetailedCostType(DetailedCostType: *mut iso20_dc_DetailedCostType);
    fn init_iso20_dc_DetailedTaxType(DetailedTaxType: *mut iso20_dc_DetailedTaxType);
    fn init_iso20_dc_MessageHeaderType(MessageHeaderType: *mut iso20_dc_MessageHeaderType);
    fn init_iso20_dc_DisplayParametersType(
        DisplayParametersType: *mut iso20_dc_DisplayParametersType,
    );
    fn init_iso20_dc_EVSEStatusType(EVSEStatusType: *mut iso20_dc_EVSEStatusType);
    fn init_iso20_dc_MeterInfoType(MeterInfoType: *mut iso20_dc_MeterInfoType);
    fn init_iso20_dc_ReceiptType(ReceiptType: *mut iso20_dc_ReceiptType);
    fn init_iso20_dc_exiFragment(exiFrag: *mut iso20_dc_exiFragment);
    fn init_iso20_dc_xmldsigFragment(xmldsigFrag: *mut iso20_dc_xmldsigFragment);
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
unsafe extern "C" fn decode_iso20_dc_TransformType(
    stream: &mut ExiBitstream,
    mut TransformType: *mut iso20_dc_TransformType,
) -> i32 {
    let mut grammar_id: i32 = 0 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_TransformType(TransformType);
    while done == 0 {
        match grammar_id {
            0 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*TransformType).Algorithm.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*TransformType).Algorithm.charactersLen as i32 >= 2 as i32 {
                                    (*TransformType).Algorithm.charactersLen = ((*TransformType)
                                        .Algorithm
                                        .charactersLen
                                        as i32
                                        - 2 as i32)
                                        as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*TransformType).Algorithm.charactersLen as usize,
                                        ((*TransformType).Algorithm.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            grammar_id = 1 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            1 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    error = exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut (*TransformType).XPath.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*TransformType).XPath.charactersLen as i32 >= 2 as i32 {
                                            (*TransformType).XPath.charactersLen =
                                                ((*TransformType).XPath.charactersLen as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*TransformType).XPath.charactersLen as usize,
                                                ((*TransformType).XPath.characters).as_mut_ptr(),
                                                (64 as i32 + 1 as i32) as usize,
                                            );
                                        } else {
                                            error = -(200 as i32);
                                        }
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        (*TransformType).set_XPath_isUsed(1 as u32);
                                        grammar_id = 2 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            error = -(50 as i32);
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        3 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*TransformType).ANY.bytesLen,
                                &mut *((*TransformType).ANY.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                4 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*TransformType).set_ANY_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_TransformsType(
    stream: &mut ExiBitstream,
    mut TransformsType: *mut iso20_dc_TransformsType,
) -> i32 {
    let mut grammar_id: i32 = 4 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_TransformsType(TransformsType);
    while done == 0 {
        match grammar_id {
            4 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_TransformType(
                                stream,
                                &mut (*TransformsType).Transform,
                            );
                            if error == 0 as i32 {
                                grammar_id = 5 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            5 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = -(110 as i32);
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_DSAKeyValueType(
    stream: &mut ExiBitstream,
    mut DSAKeyValueType: *mut iso20_dc_DSAKeyValueType,
) -> i32 {
    let mut grammar_id: i32 = 6 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_DSAKeyValueType(DSAKeyValueType);
    while done == 0 {
        match grammar_id {
            6 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*DSAKeyValueType).P.bytesLen,
                                &mut *((*DSAKeyValueType).P.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*DSAKeyValueType).set_P_isUsed(1 as u32);
                                grammar_id = 7 as i32;
                            }
                        }
                        1 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*DSAKeyValueType).G.bytesLen,
                                &mut *((*DSAKeyValueType).G.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*DSAKeyValueType).set_G_isUsed(1 as u32);
                                grammar_id = 9 as i32;
                            }
                        }
                        2 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*DSAKeyValueType).Y.bytesLen,
                                &mut *((*DSAKeyValueType).Y.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 10 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            7 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*DSAKeyValueType).Q.bytesLen,
                                &mut *((*DSAKeyValueType).Q.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*DSAKeyValueType).set_Q_isUsed(1 as u32);
                                grammar_id = 8 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            8 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*DSAKeyValueType).G.bytesLen,
                                &mut *((*DSAKeyValueType).G.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*DSAKeyValueType).set_G_isUsed(1 as u32);
                                grammar_id = 9 as i32;
                            }
                        }
                        1 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*DSAKeyValueType).Y.bytesLen,
                                &mut *((*DSAKeyValueType).Y.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 10 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            9 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*DSAKeyValueType).Y.bytesLen,
                                &mut *((*DSAKeyValueType).Y.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 10 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            10 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*DSAKeyValueType).J.bytesLen,
                                &mut *((*DSAKeyValueType).J.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*DSAKeyValueType).set_J_isUsed(1 as u32);
                                grammar_id = 11 as i32;
                            }
                        }
                        1 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*DSAKeyValueType).Seed.bytesLen,
                                &mut *((*DSAKeyValueType).Seed.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*DSAKeyValueType).set_Seed_isUsed(1 as u32);
                                grammar_id = 12 as i32;
                            }
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            11 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*DSAKeyValueType).Seed.bytesLen,
                                &mut *((*DSAKeyValueType).Seed.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*DSAKeyValueType).set_Seed_isUsed(1 as u32);
                                grammar_id = 12 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            12 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*DSAKeyValueType).PgenCounter.bytesLen,
                                &mut *((*DSAKeyValueType).PgenCounter.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*DSAKeyValueType).set_PgenCounter_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_X509IssuerSerialType(
    stream: &mut ExiBitstream,
    mut X509IssuerSerialType: *mut iso20_dc_X509IssuerSerialType,
) -> i32 {
    let mut grammar_id: i32 = 13 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_X509IssuerSerialType(X509IssuerSerialType);
    while done == 0 {
        match grammar_id {
            13 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    error = exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut (*X509IssuerSerialType).X509IssuerName.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*X509IssuerSerialType).X509IssuerName.charactersLen
                                            as i32
                                            >= 2 as i32
                                        {
                                            (*X509IssuerSerialType).X509IssuerName.charactersLen =
                                                ((*X509IssuerSerialType)
                                                    .X509IssuerName
                                                    .charactersLen
                                                    as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*X509IssuerSerialType).X509IssuerName.charactersLen
                                                    as usize,
                                                ((*X509IssuerSerialType).X509IssuerName.characters)
                                                    .as_mut_ptr(),
                                                (64 as i32 + 1 as i32) as usize,
                                            );
                                        } else {
                                            error = -(200 as i32);
                                        }
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 14 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            14 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_signed(
                                    stream,
                                    &mut (*X509IssuerSerialType).X509SerialNumber,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 2 as i32;
                                }
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_DigestMethodType(
    stream: &mut ExiBitstream,
    mut DigestMethodType: *mut iso20_dc_DigestMethodType,
) -> i32 {
    let mut grammar_id: i32 = 15 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_DigestMethodType(DigestMethodType);
    while done == 0 {
        match grammar_id {
            15 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*DigestMethodType).Algorithm.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*DigestMethodType).Algorithm.charactersLen as i32 >= 2 as i32 {
                                    (*DigestMethodType).Algorithm.charactersLen =
                                        ((*DigestMethodType).Algorithm.charactersLen as i32
                                            - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*DigestMethodType).Algorithm.charactersLen as usize,
                                        ((*DigestMethodType).Algorithm.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            grammar_id = 16 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            16 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = -(50 as i32);
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        2 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*DigestMethodType).ANY.bytesLen,
                                &mut *((*DigestMethodType).ANY.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                4 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*DigestMethodType).set_ANY_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_RSAKeyValueType(
    stream: &mut ExiBitstream,
    mut RSAKeyValueType: *mut iso20_dc_RSAKeyValueType,
) -> i32 {
    let mut grammar_id: i32 = 17 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_RSAKeyValueType(RSAKeyValueType);
    while done == 0 {
        match grammar_id {
            17 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*RSAKeyValueType).Modulus.bytesLen,
                                &mut *((*RSAKeyValueType).Modulus.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 18 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            18 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*RSAKeyValueType).Exponent.bytesLen,
                                &mut *((*RSAKeyValueType).Exponent.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_CanonicalizationMethodType(
    stream: &mut ExiBitstream,
    mut CanonicalizationMethodType: *mut iso20_dc_CanonicalizationMethodType,
) -> i32 {
    let mut grammar_id: i32 = 19 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_CanonicalizationMethodType(CanonicalizationMethodType);
    while done == 0 {
        match grammar_id {
            19 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*CanonicalizationMethodType).Algorithm.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*CanonicalizationMethodType).Algorithm.charactersLen as i32
                                    >= 2 as i32
                                {
                                    (*CanonicalizationMethodType).Algorithm.charactersLen =
                                        ((*CanonicalizationMethodType).Algorithm.charactersLen
                                            as i32
                                            - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*CanonicalizationMethodType).Algorithm.charactersLen
                                            as usize,
                                        ((*CanonicalizationMethodType).Algorithm.characters)
                                            .as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            grammar_id = 20 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            20 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = -(50 as i32);
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        2 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*CanonicalizationMethodType).ANY.bytesLen,
                                &mut *((*CanonicalizationMethodType).ANY.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                4 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*CanonicalizationMethodType).set_ANY_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_SignatureMethodType(
    stream: &mut ExiBitstream,
    mut SignatureMethodType: *mut iso20_dc_SignatureMethodType,
) -> i32 {
    let mut grammar_id: i32 = 21 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_SignatureMethodType(SignatureMethodType);
    while done == 0 {
        match grammar_id {
            21 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*SignatureMethodType).Algorithm.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*SignatureMethodType).Algorithm.charactersLen as i32 >= 2 as i32
                                {
                                    (*SignatureMethodType).Algorithm.charactersLen =
                                        ((*SignatureMethodType).Algorithm.charactersLen as i32
                                            - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*SignatureMethodType).Algorithm.charactersLen as usize,
                                        ((*SignatureMethodType).Algorithm.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            grammar_id = 22 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            22 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_signed(
                                    stream,
                                    &mut (*SignatureMethodType).HMACOutputLength,
                                );
                                if error == 0 as i32 {
                                    (*SignatureMethodType).set_HMACOutputLength_isUsed(1 as u32);
                                    grammar_id = 23 as i32;
                                }
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                            }
                        }
                        1 => {
                            error = -(50 as i32);
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        3 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*SignatureMethodType).ANY.bytesLen,
                                &mut *((*SignatureMethodType).ANY.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                4 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*SignatureMethodType).set_ANY_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            23 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = -(50 as i32);
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        2 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*SignatureMethodType).ANY.bytesLen,
                                &mut *((*SignatureMethodType).ANY.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                4 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*SignatureMethodType).set_ANY_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_KeyValueType(
    stream: &mut ExiBitstream,
    mut KeyValueType: *mut iso20_dc_KeyValueType,
) -> i32 {
    let mut grammar_id: i32 = 24 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_KeyValueType(KeyValueType);
    while done == 0 {
        match grammar_id {
            24 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_DSAKeyValueType(
                                stream,
                                &mut (*KeyValueType).DSAKeyValue,
                            );
                            if error == 0 as i32 {
                                (*KeyValueType).set_DSAKeyValue_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RSAKeyValueType(
                                stream,
                                &mut (*KeyValueType).RSAKeyValue,
                            );
                            if error == 0 as i32 {
                                (*KeyValueType).set_RSAKeyValue_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        2 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*KeyValueType).ANY.bytesLen,
                                &mut *((*KeyValueType).ANY.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                4 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*KeyValueType).set_ANY_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_ReferenceType(
    stream: &mut ExiBitstream,
    mut ReferenceType: *mut iso20_dc_ReferenceType,
) -> i32 {
    let mut grammar_id: i32 = 25 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_ReferenceType(ReferenceType);
    while done == 0 {
        match grammar_id {
            25 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*ReferenceType).Id.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*ReferenceType).Id.charactersLen as i32 >= 2 as i32 {
                                    (*ReferenceType).Id.charactersLen =
                                        ((*ReferenceType).Id.charactersLen as i32 - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*ReferenceType).Id.charactersLen as usize,
                                        ((*ReferenceType).Id.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*ReferenceType).set_Id_isUsed(1 as u32);
                            grammar_id = 26 as i32;
                        }
                        1 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*ReferenceType).Type.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*ReferenceType).Type.charactersLen as i32 >= 2 as i32 {
                                    (*ReferenceType).Type.charactersLen =
                                        ((*ReferenceType).Type.charactersLen as i32 - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*ReferenceType).Type.charactersLen as usize,
                                        ((*ReferenceType).Type.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*ReferenceType).set_Type_isUsed(1 as u32);
                            grammar_id = 27 as i32;
                        }
                        2 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*ReferenceType).URI.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*ReferenceType).URI.charactersLen as i32 >= 2 as i32 {
                                    (*ReferenceType).URI.charactersLen =
                                        ((*ReferenceType).URI.charactersLen as i32 - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*ReferenceType).URI.charactersLen as usize,
                                        ((*ReferenceType).URI.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*ReferenceType).set_URI_isUsed(1 as u32);
                            grammar_id = 28 as i32;
                        }
                        3 => {
                            error = decode_iso20_dc_TransformsType(
                                stream,
                                &mut (*ReferenceType).Transforms,
                            );
                            if error == 0 as i32 {
                                (*ReferenceType).set_Transforms_isUsed(1 as u32);
                                grammar_id = 29 as i32;
                            }
                        }
                        4 => {
                            error = decode_iso20_dc_DigestMethodType(
                                stream,
                                &mut (*ReferenceType).DigestMethod,
                            );
                            if error == 0 as i32 {
                                grammar_id = 30 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            26 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*ReferenceType).Type.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*ReferenceType).Type.charactersLen as i32 >= 2 as i32 {
                                    (*ReferenceType).Type.charactersLen =
                                        ((*ReferenceType).Type.charactersLen as i32 - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*ReferenceType).Type.charactersLen as usize,
                                        ((*ReferenceType).Type.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*ReferenceType).set_Type_isUsed(1 as u32);
                            grammar_id = 27 as i32;
                        }
                        1 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*ReferenceType).URI.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*ReferenceType).URI.charactersLen as i32 >= 2 as i32 {
                                    (*ReferenceType).URI.charactersLen =
                                        ((*ReferenceType).URI.charactersLen as i32 - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*ReferenceType).URI.charactersLen as usize,
                                        ((*ReferenceType).URI.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*ReferenceType).set_URI_isUsed(1 as u32);
                            grammar_id = 28 as i32;
                        }
                        2 => {
                            error = decode_iso20_dc_TransformsType(
                                stream,
                                &mut (*ReferenceType).Transforms,
                            );
                            if error == 0 as i32 {
                                (*ReferenceType).set_Transforms_isUsed(1 as u32);
                                grammar_id = 29 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_dc_DigestMethodType(
                                stream,
                                &mut (*ReferenceType).DigestMethod,
                            );
                            if error == 0 as i32 {
                                grammar_id = 30 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            27 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*ReferenceType).URI.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*ReferenceType).URI.charactersLen as i32 >= 2 as i32 {
                                    (*ReferenceType).URI.charactersLen =
                                        ((*ReferenceType).URI.charactersLen as i32 - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*ReferenceType).URI.charactersLen as usize,
                                        ((*ReferenceType).URI.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*ReferenceType).set_URI_isUsed(1 as u32);
                            grammar_id = 28 as i32;
                        }
                        1 => {
                            error = decode_iso20_dc_TransformsType(
                                stream,
                                &mut (*ReferenceType).Transforms,
                            );
                            if error == 0 as i32 {
                                (*ReferenceType).set_Transforms_isUsed(1 as u32);
                                grammar_id = 29 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_DigestMethodType(
                                stream,
                                &mut (*ReferenceType).DigestMethod,
                            );
                            if error == 0 as i32 {
                                grammar_id = 30 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            28 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_TransformsType(
                                stream,
                                &mut (*ReferenceType).Transforms,
                            );
                            if error == 0 as i32 {
                                (*ReferenceType).set_Transforms_isUsed(1 as u32);
                                grammar_id = 29 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_DigestMethodType(
                                stream,
                                &mut (*ReferenceType).DigestMethod,
                            );
                            if error == 0 as i32 {
                                grammar_id = 30 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            29 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_DigestMethodType(
                                stream,
                                &mut (*ReferenceType).DigestMethod,
                            );
                            if error == 0 as i32 {
                                grammar_id = 30 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            30 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*ReferenceType).DigestValue.bytesLen,
                                &mut *((*ReferenceType).DigestValue.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_RetrievalMethodType(
    stream: &mut ExiBitstream,
    mut RetrievalMethodType: *mut iso20_dc_RetrievalMethodType,
) -> i32 {
    let mut grammar_id: i32 = 31 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_RetrievalMethodType(RetrievalMethodType);
    while done == 0 {
        match grammar_id {
            31 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*RetrievalMethodType).Type.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*RetrievalMethodType).Type.charactersLen as i32 >= 2 as i32 {
                                    (*RetrievalMethodType).Type.charactersLen =
                                        ((*RetrievalMethodType).Type.charactersLen as i32
                                            - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*RetrievalMethodType).Type.charactersLen as usize,
                                        ((*RetrievalMethodType).Type.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*RetrievalMethodType).set_Type_isUsed(1 as u32);
                            grammar_id = 32 as i32;
                        }
                        1 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*RetrievalMethodType).URI.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*RetrievalMethodType).URI.charactersLen as i32 >= 2 as i32 {
                                    (*RetrievalMethodType).URI.charactersLen =
                                        ((*RetrievalMethodType).URI.charactersLen as i32 - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*RetrievalMethodType).URI.charactersLen as usize,
                                        ((*RetrievalMethodType).URI.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*RetrievalMethodType).set_URI_isUsed(1 as u32);
                            grammar_id = 33 as i32;
                        }
                        2 => {
                            error = decode_iso20_dc_TransformsType(
                                stream,
                                &mut (*RetrievalMethodType).Transforms,
                            );
                            if error == 0 as i32 {
                                (*RetrievalMethodType).set_Transforms_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        3 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            32 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*RetrievalMethodType).URI.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*RetrievalMethodType).URI.charactersLen as i32 >= 2 as i32 {
                                    (*RetrievalMethodType).URI.charactersLen =
                                        ((*RetrievalMethodType).URI.charactersLen as i32 - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*RetrievalMethodType).URI.charactersLen as usize,
                                        ((*RetrievalMethodType).URI.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*RetrievalMethodType).set_URI_isUsed(1 as u32);
                            grammar_id = 33 as i32;
                        }
                        1 => {
                            error = decode_iso20_dc_TransformsType(
                                stream,
                                &mut (*RetrievalMethodType).Transforms,
                            );
                            if error == 0 as i32 {
                                (*RetrievalMethodType).set_Transforms_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            33 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_TransformsType(
                                stream,
                                &mut (*RetrievalMethodType).Transforms,
                            );
                            if error == 0 as i32 {
                                (*RetrievalMethodType).set_Transforms_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_X509DataType(
    stream: &mut ExiBitstream,
    mut X509DataType: *mut iso20_dc_X509DataType,
) -> i32 {
    let mut grammar_id: i32 = 34 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_X509DataType(X509DataType);
    while done == 0 {
        match grammar_id {
            34 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_X509IssuerSerialType(
                                stream,
                                &mut (*X509DataType).X509IssuerSerial,
                            );
                            if error == 0 as i32 {
                                (*X509DataType).set_X509IssuerSerial_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*X509DataType).X509SKI.bytesLen,
                                &mut *((*X509DataType).X509SKI.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*X509DataType).set_X509SKI_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        2 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    error = exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut (*X509DataType).X509SubjectName.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*X509DataType).X509SubjectName.charactersLen as i32
                                            >= 2 as i32
                                        {
                                            (*X509DataType).X509SubjectName.charactersLen =
                                                ((*X509DataType).X509SubjectName.charactersLen
                                                    as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*X509DataType).X509SubjectName.charactersLen
                                                    as usize,
                                                ((*X509DataType).X509SubjectName.characters)
                                                    .as_mut_ptr(),
                                                (64 as i32 + 1 as i32) as usize,
                                            );
                                        } else {
                                            error = -(200 as i32);
                                        }
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        (*X509DataType).set_X509SubjectName_isUsed(1 as u32);
                                        grammar_id = 2 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        3 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*X509DataType).X509Certificate.bytesLen,
                                &mut *((*X509DataType).X509Certificate.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*X509DataType).set_X509Certificate_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        4 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*X509DataType).X509CRL.bytesLen,
                                &mut *((*X509DataType).X509CRL.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*X509DataType).set_X509CRL_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        5 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*X509DataType).ANY.bytesLen,
                                &mut *((*X509DataType).ANY.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                4 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*X509DataType).set_ANY_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_PGPDataType(
    stream: &mut ExiBitstream,
    mut PGPDataType: *mut iso20_dc_PGPDataType,
) -> i32 {
    let mut grammar_id: i32 = 35 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_PGPDataType(PGPDataType);
    while done == 0 {
        match grammar_id {
            35 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*PGPDataType).c2rust_unnamed.choice_1.PGPKeyID.bytesLen,
                                &mut *((*PGPDataType).c2rust_unnamed.choice_1.PGPKeyID.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 36 as i32;
                            }
                        }
                        1 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*PGPDataType).c2rust_unnamed.choice_1.PGPKeyPacket.bytesLen,
                                &mut *((*PGPDataType).c2rust_unnamed.choice_1.PGPKeyPacket.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                ((*PGPDataType).c2rust_unnamed.choice_1)
                                    .set_PGPKeyPacket_isUsed(1 as u32);
                                grammar_id = 37 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            36 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*PGPDataType).c2rust_unnamed.choice_1.PGPKeyPacket.bytesLen,
                                &mut *((*PGPDataType).c2rust_unnamed.choice_1.PGPKeyPacket.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                ((*PGPDataType).c2rust_unnamed.choice_1)
                                    .set_PGPKeyPacket_isUsed(1 as u32);
                                grammar_id = 37 as i32;
                            }
                        }
                        1 => {
                            error = -(50 as i32);
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        3 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*PGPDataType).c2rust_unnamed.choice_1.ANY.bytesLen,
                                &mut *((*PGPDataType).c2rust_unnamed.choice_1.ANY.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                4 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                ((*PGPDataType).c2rust_unnamed.choice_1).set_ANY_isUsed(1 as u32);
                                grammar_id = 38 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            37 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = -(50 as i32);
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        3 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*PGPDataType).c2rust_unnamed.choice_1.ANY.bytesLen,
                                &mut *((*PGPDataType).c2rust_unnamed.choice_1.ANY.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                4 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                ((*PGPDataType).c2rust_unnamed.choice_1).set_ANY_isUsed(1 as u32);
                                grammar_id = 38 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            38 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*PGPDataType).c2rust_unnamed.choice_2.PGPKeyPacket.bytesLen,
                                &mut *((*PGPDataType).c2rust_unnamed.choice_2.PGPKeyPacket.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 39 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            39 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = -(50 as i32);
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        2 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*PGPDataType).c2rust_unnamed.choice_2.ANY.bytesLen,
                                &mut *((*PGPDataType).c2rust_unnamed.choice_2.ANY.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                4 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                ((*PGPDataType).c2rust_unnamed.choice_2).set_ANY_isUsed(1 as u32);
                                grammar_id = 38 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_SPKIDataType(
    stream: &mut ExiBitstream,
    mut SPKIDataType: *mut iso20_dc_SPKIDataType,
) -> i32 {
    let mut grammar_id: i32 = 40 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_SPKIDataType(SPKIDataType);
    while done == 0 {
        match grammar_id {
            40 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*SPKIDataType).SPKISexp.bytesLen,
                                &mut *((*SPKIDataType).SPKISexp.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 41 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            41 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = -(50 as i32);
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        2 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*SPKIDataType).ANY.bytesLen,
                                &mut *((*SPKIDataType).ANY.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                4 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*SPKIDataType).set_ANY_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_SignedInfoType(
    stream: &mut ExiBitstream,
    mut SignedInfoType: *mut iso20_dc_SignedInfoType,
) -> i32 {
    let mut grammar_id: i32 = 42 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_SignedInfoType(SignedInfoType);
    while done == 0 {
        match grammar_id {
            42 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*SignedInfoType).Id.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*SignedInfoType).Id.charactersLen as i32 >= 2 as i32 {
                                    (*SignedInfoType).Id.charactersLen =
                                        ((*SignedInfoType).Id.charactersLen as i32 - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*SignedInfoType).Id.charactersLen as usize,
                                        ((*SignedInfoType).Id.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*SignedInfoType).set_Id_isUsed(1 as u32);
                            grammar_id = 43 as i32;
                        }
                        1 => {
                            error = decode_iso20_dc_CanonicalizationMethodType(
                                stream,
                                &mut (*SignedInfoType).CanonicalizationMethod,
                            );
                            if error == 0 as i32 {
                                grammar_id = 44 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            43 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_CanonicalizationMethodType(
                                stream,
                                &mut (*SignedInfoType).CanonicalizationMethod,
                            );
                            if error == 0 as i32 {
                                grammar_id = 44 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            44 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_SignatureMethodType(
                                stream,
                                &mut (*SignedInfoType).SignatureMethod,
                            );
                            if error == 0 as i32 {
                                grammar_id = 45 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            45 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SignedInfoType).Reference.arrayLen as i32) < 4 as i32 {
                                let fresh0 = (*SignedInfoType).Reference.arrayLen;
                                (*SignedInfoType).Reference.arrayLen =
                                    ((*SignedInfoType).Reference.arrayLen).wrapping_add(1);
                                error = decode_iso20_dc_ReferenceType(
                                    stream,
                                    &mut *((*SignedInfoType).Reference.array)
                                        .as_mut_ptr()
                                        .offset(fresh0 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 46 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            46 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SignedInfoType).Reference.arrayLen as i32) < 4 as i32 {
                                let fresh1 = (*SignedInfoType).Reference.arrayLen;
                                (*SignedInfoType).Reference.arrayLen =
                                    ((*SignedInfoType).Reference.arrayLen).wrapping_add(1);
                                error = decode_iso20_dc_ReferenceType(
                                    stream,
                                    &mut *((*SignedInfoType).Reference.array)
                                        .as_mut_ptr()
                                        .offset(fresh1 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 46 as i32;
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_SignatureValueType(
    stream: &mut ExiBitstream,
    mut SignatureValueType: *mut iso20_dc_SignatureValueType,
) -> i32 {
    let mut grammar_id: i32 = 47 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_SignatureValueType(SignatureValueType);
    while done == 0 {
        match grammar_id {
            47 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*SignatureValueType).Id.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*SignatureValueType).Id.charactersLen as i32 >= 2 as i32 {
                                    (*SignatureValueType).Id.charactersLen =
                                        ((*SignatureValueType).Id.charactersLen as i32 - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*SignatureValueType).Id.charactersLen as usize,
                                        ((*SignatureValueType).Id.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*SignatureValueType).set_Id_isUsed(1 as u32);
                            grammar_id = 48 as i32;
                        }
                        1 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*SignatureValueType).CONTENT.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_bytes(
                                    stream,
                                    (*SignatureValueType).CONTENT.bytesLen as usize,
                                    &mut *((*SignatureValueType).CONTENT.bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 2 as i32;
                                }
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            48 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*SignatureValueType).CONTENT.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_bytes(
                                    stream,
                                    (*SignatureValueType).CONTENT.bytesLen as usize,
                                    &mut *((*SignatureValueType).CONTENT.bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 2 as i32;
                                }
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_KeyInfoType(
    stream: &mut ExiBitstream,
    mut KeyInfoType: *mut iso20_dc_KeyInfoType,
) -> i32 {
    let mut grammar_id: i32 = 49 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_KeyInfoType(KeyInfoType);
    while done == 0 {
        match grammar_id {
            49 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 4 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*KeyInfoType).Id.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*KeyInfoType).Id.charactersLen as i32 >= 2 as i32 {
                                    (*KeyInfoType).Id.charactersLen =
                                        ((*KeyInfoType).Id.charactersLen as i32 - 2 as i32) as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*KeyInfoType).Id.charactersLen as usize,
                                        ((*KeyInfoType).Id.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*KeyInfoType).set_Id_isUsed(1 as u32);
                            grammar_id = 50 as i32;
                        }
                        1 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    error = exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut (*KeyInfoType).KeyName.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*KeyInfoType).KeyName.charactersLen as i32 >= 2 as i32 {
                                            (*KeyInfoType).KeyName.charactersLen =
                                                ((*KeyInfoType).KeyName.charactersLen as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*KeyInfoType).KeyName.charactersLen as usize,
                                                ((*KeyInfoType).KeyName.characters).as_mut_ptr(),
                                                (64 as i32 + 1 as i32) as usize,
                                            );
                                        } else {
                                            error = -(200 as i32);
                                        }
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        (*KeyInfoType).set_KeyName_isUsed(1 as u32);
                                        grammar_id = 2 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        2 => {
                            error =
                                decode_iso20_dc_KeyValueType(stream, &mut (*KeyInfoType).KeyValue);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_KeyValue_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_dc_RetrievalMethodType(
                                stream,
                                &mut (*KeyInfoType).RetrievalMethod,
                            );
                            if error == 0 as i32 {
                                (*KeyInfoType).set_RetrievalMethod_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        4 => {
                            error =
                                decode_iso20_dc_X509DataType(stream, &mut (*KeyInfoType).X509Data);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_X509Data_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        5 => {
                            error =
                                decode_iso20_dc_PGPDataType(stream, &mut (*KeyInfoType).PGPData);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_PGPData_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        6 => {
                            error =
                                decode_iso20_dc_SPKIDataType(stream, &mut (*KeyInfoType).SPKIData);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_SPKIData_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        7 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    error = exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut (*KeyInfoType).MgmtData.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*KeyInfoType).MgmtData.charactersLen as i32 >= 2 as i32
                                        {
                                            (*KeyInfoType).MgmtData.charactersLen =
                                                ((*KeyInfoType).MgmtData.charactersLen as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*KeyInfoType).MgmtData.charactersLen as usize,
                                                ((*KeyInfoType).MgmtData.characters).as_mut_ptr(),
                                                (64 as i32 + 1 as i32) as usize,
                                            );
                                        } else {
                                            error = -(200 as i32);
                                        }
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        (*KeyInfoType).set_MgmtData_isUsed(1 as u32);
                                        grammar_id = 2 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        8 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*KeyInfoType).ANY.bytesLen,
                                &mut *((*KeyInfoType).ANY.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                4 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*KeyInfoType).set_ANY_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            50 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 4 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    error = exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut (*KeyInfoType).KeyName.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*KeyInfoType).KeyName.charactersLen as i32 >= 2 as i32 {
                                            (*KeyInfoType).KeyName.charactersLen =
                                                ((*KeyInfoType).KeyName.charactersLen as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*KeyInfoType).KeyName.charactersLen as usize,
                                                ((*KeyInfoType).KeyName.characters).as_mut_ptr(),
                                                (64 as i32 + 1 as i32) as usize,
                                            );
                                        } else {
                                            error = -(200 as i32);
                                        }
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        (*KeyInfoType).set_KeyName_isUsed(1 as u32);
                                        grammar_id = 2 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            error =
                                decode_iso20_dc_KeyValueType(stream, &mut (*KeyInfoType).KeyValue);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_KeyValue_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_RetrievalMethodType(
                                stream,
                                &mut (*KeyInfoType).RetrievalMethod,
                            );
                            if error == 0 as i32 {
                                (*KeyInfoType).set_RetrievalMethod_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        3 => {
                            error =
                                decode_iso20_dc_X509DataType(stream, &mut (*KeyInfoType).X509Data);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_X509Data_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        4 => {
                            error =
                                decode_iso20_dc_PGPDataType(stream, &mut (*KeyInfoType).PGPData);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_PGPData_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        5 => {
                            error =
                                decode_iso20_dc_SPKIDataType(stream, &mut (*KeyInfoType).SPKIData);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_SPKIData_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        6 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    error = exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut (*KeyInfoType).MgmtData.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*KeyInfoType).MgmtData.charactersLen as i32 >= 2 as i32
                                        {
                                            (*KeyInfoType).MgmtData.charactersLen =
                                                ((*KeyInfoType).MgmtData.charactersLen as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*KeyInfoType).MgmtData.charactersLen as usize,
                                                ((*KeyInfoType).MgmtData.characters).as_mut_ptr(),
                                                (64 as i32 + 1 as i32) as usize,
                                            );
                                        } else {
                                            error = -(200 as i32);
                                        }
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        (*KeyInfoType).set_MgmtData_isUsed(1 as u32);
                                        grammar_id = 2 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        7 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*KeyInfoType).ANY.bytesLen,
                                &mut *((*KeyInfoType).ANY.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                4 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*KeyInfoType).set_ANY_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_ObjectType(
    stream: &mut ExiBitstream,
    mut ObjectType: *mut iso20_dc_ObjectType,
) -> i32 {
    let mut grammar_id: i32 = 51 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_ObjectType(ObjectType);
    while done == 0 {
        match grammar_id {
            51 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*ObjectType).Encoding.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*ObjectType).Encoding.charactersLen as i32 >= 2 as i32 {
                                    (*ObjectType).Encoding.charactersLen =
                                        ((*ObjectType).Encoding.charactersLen as i32 - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*ObjectType).Encoding.charactersLen as usize,
                                        ((*ObjectType).Encoding.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*ObjectType).set_Encoding_isUsed(1 as u32);
                            grammar_id = 52 as i32;
                        }
                        1 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*ObjectType).Id.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*ObjectType).Id.charactersLen as i32 >= 2 as i32 {
                                    (*ObjectType).Id.charactersLen =
                                        ((*ObjectType).Id.charactersLen as i32 - 2 as i32) as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*ObjectType).Id.charactersLen as usize,
                                        ((*ObjectType).Id.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*ObjectType).set_Id_isUsed(1 as u32);
                            grammar_id = 53 as i32;
                        }
                        2 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*ObjectType).MimeType.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*ObjectType).MimeType.charactersLen as i32 >= 2 as i32 {
                                    (*ObjectType).MimeType.charactersLen =
                                        ((*ObjectType).MimeType.charactersLen as i32 - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*ObjectType).MimeType.charactersLen as usize,
                                        ((*ObjectType).MimeType.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*ObjectType).set_MimeType_isUsed(1 as u32);
                            grammar_id = 54 as i32;
                        }
                        3 => {
                            error = -(50 as i32);
                        }
                        4 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        5 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*ObjectType).ANY.bytesLen,
                                &mut *((*ObjectType).ANY.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                4 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*ObjectType).set_ANY_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            52 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*ObjectType).Id.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*ObjectType).Id.charactersLen as i32 >= 2 as i32 {
                                    (*ObjectType).Id.charactersLen =
                                        ((*ObjectType).Id.charactersLen as i32 - 2 as i32) as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*ObjectType).Id.charactersLen as usize,
                                        ((*ObjectType).Id.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*ObjectType).set_Id_isUsed(1 as u32);
                            grammar_id = 53 as i32;
                        }
                        1 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*ObjectType).MimeType.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*ObjectType).MimeType.charactersLen as i32 >= 2 as i32 {
                                    (*ObjectType).MimeType.charactersLen =
                                        ((*ObjectType).MimeType.charactersLen as i32 - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*ObjectType).MimeType.charactersLen as usize,
                                        ((*ObjectType).MimeType.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*ObjectType).set_MimeType_isUsed(1 as u32);
                            grammar_id = 54 as i32;
                        }
                        2 => {
                            error = -(50 as i32);
                        }
                        3 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        4 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*ObjectType).ANY.bytesLen,
                                &mut *((*ObjectType).ANY.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                4 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*ObjectType).set_ANY_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            53 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*ObjectType).MimeType.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*ObjectType).MimeType.charactersLen as i32 >= 2 as i32 {
                                    (*ObjectType).MimeType.charactersLen =
                                        ((*ObjectType).MimeType.charactersLen as i32 - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*ObjectType).MimeType.charactersLen as usize,
                                        ((*ObjectType).MimeType.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*ObjectType).set_MimeType_isUsed(1 as u32);
                            grammar_id = 54 as i32;
                        }
                        1 => {
                            error = -(50 as i32);
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        3 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*ObjectType).ANY.bytesLen,
                                &mut *((*ObjectType).ANY.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                4 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*ObjectType).set_ANY_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            54 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = -(50 as i32);
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        2 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*ObjectType).ANY.bytesLen,
                                &mut *((*ObjectType).ANY.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                4 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*ObjectType).set_ANY_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_RationalNumberType(
    stream: &mut ExiBitstream,
    mut RationalNumberType: *mut iso20_dc_RationalNumberType,
) -> i32 {
    let mut grammar_id: i32 = 55 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_RationalNumberType(RationalNumberType);
    while done == 0 {
        match grammar_id {
            55 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        8 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*RationalNumberType).Exponent =
                                            value.wrapping_add(-(128 as i32) as u32) as int8_t;
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 56 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            56 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error =
                                decode_exi_type_integer16(stream, &mut (*RationalNumberType).Value);
                            if error == 0 as i32 {
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_DetailedCostType(
    stream: &mut ExiBitstream,
    mut DetailedCostType: *mut iso20_dc_DetailedCostType,
) -> i32 {
    let mut grammar_id: i32 = 57 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_DetailedCostType(DetailedCostType);
    while done == 0 {
        match grammar_id {
            57 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DetailedCostType).Amount,
                            );
                            if error == 0 as i32 {
                                grammar_id = 58 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            58 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DetailedCostType).CostPerUnit,
                            );
                            if error == 0 as i32 {
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_SignatureType(
    stream: &mut ExiBitstream,
    mut SignatureType: *mut iso20_dc_SignatureType,
) -> i32 {
    let mut grammar_id: i32 = 59 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_SignatureType(SignatureType);
    while done == 0 {
        match grammar_id {
            59 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*SignatureType).Id.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*SignatureType).Id.charactersLen as i32 >= 2 as i32 {
                                    (*SignatureType).Id.charactersLen =
                                        ((*SignatureType).Id.charactersLen as i32 - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*SignatureType).Id.charactersLen as usize,
                                        ((*SignatureType).Id.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*SignatureType).set_Id_isUsed(1 as u32);
                            grammar_id = 60 as i32;
                        }
                        1 => {
                            error = decode_iso20_dc_SignedInfoType(
                                stream,
                                &mut (*SignatureType).SignedInfo,
                            );
                            if error == 0 as i32 {
                                grammar_id = 61 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            60 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_SignedInfoType(
                                stream,
                                &mut (*SignatureType).SignedInfo,
                            );
                            if error == 0 as i32 {
                                grammar_id = 61 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            61 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_SignatureValueType(
                                stream,
                                &mut (*SignatureType).SignatureValue,
                            );
                            if error == 0 as i32 {
                                grammar_id = 62 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            62 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error =
                                decode_iso20_dc_KeyInfoType(stream, &mut (*SignatureType).KeyInfo);
                            if error == 0 as i32 {
                                (*SignatureType).set_KeyInfo_isUsed(1 as u32);
                                grammar_id = 64 as i32;
                            }
                        }
                        1 => {
                            error =
                                decode_iso20_dc_ObjectType(stream, &mut (*SignatureType).Object);
                            if error == 0 as i32 {
                                (*SignatureType).set_Object_isUsed(1 as u32);
                                grammar_id = 63 as i32;
                            }
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            63 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = -(110 as i32);
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            64 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error =
                                decode_iso20_dc_ObjectType(stream, &mut (*SignatureType).Object);
                            if error == 0 as i32 {
                                (*SignatureType).set_Object_isUsed(1 as u32);
                                grammar_id = 65 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            65 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = -(110 as i32);
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_DetailedTaxType(
    stream: &mut ExiBitstream,
    mut DetailedTaxType: *mut iso20_dc_DetailedTaxType,
) -> i32 {
    let mut grammar_id: i32 = 66 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_DetailedTaxType(DetailedTaxType);
    while done == 0 {
        match grammar_id {
            66 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error =
                                decode_exi_type_uint32(stream, &mut (*DetailedTaxType).TaxRuleID);
                            if error == 0 as i32 {
                                grammar_id = 67 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            67 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DetailedTaxType).Amount,
                            );
                            if error == 0 as i32 {
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_MessageHeaderType(
    stream: &mut ExiBitstream,
    mut MessageHeaderType: *mut iso20_dc_MessageHeaderType,
) -> i32 {
    let mut grammar_id: i32 = 68 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_MessageHeaderType(MessageHeaderType);
    while done == 0 {
        match grammar_id {
            68 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*MessageHeaderType).SessionID.bytesLen,
                                &mut *((*MessageHeaderType).SessionID.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                8 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 69 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            69 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error =
                                decode_exi_type_uint64(stream, &mut (*MessageHeaderType).TimeStamp);
                            if error == 0 as i32 {
                                grammar_id = 70 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            70 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_SignatureType(
                                stream,
                                &mut (*MessageHeaderType).Signature,
                            );
                            if error == 0 as i32 {
                                (*MessageHeaderType).set_Signature_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_SignaturePropertyType(
    stream: &mut ExiBitstream,
    mut SignaturePropertyType: *mut iso20_dc_SignaturePropertyType,
) -> i32 {
    let mut grammar_id: i32 = 71 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_SignaturePropertyType(SignaturePropertyType);
    while done == 0 {
        match grammar_id {
            71 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*SignaturePropertyType).Id.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*SignaturePropertyType).Id.charactersLen as i32 >= 2 as i32 {
                                    (*SignaturePropertyType).Id.charactersLen =
                                        ((*SignaturePropertyType).Id.charactersLen as i32
                                            - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*SignaturePropertyType).Id.charactersLen as usize,
                                        ((*SignaturePropertyType).Id.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*SignaturePropertyType).set_Id_isUsed(1 as u32);
                            grammar_id = 72 as i32;
                        }
                        1 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*SignaturePropertyType).Target.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*SignaturePropertyType).Target.charactersLen as i32 >= 2 as i32
                                {
                                    (*SignaturePropertyType).Target.charactersLen =
                                        ((*SignaturePropertyType).Target.charactersLen as i32
                                            - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*SignaturePropertyType).Target.charactersLen as usize,
                                        ((*SignaturePropertyType).Target.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            grammar_id = 73 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            72 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*SignaturePropertyType).Target.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*SignaturePropertyType).Target.charactersLen as i32 >= 2 as i32
                                {
                                    (*SignaturePropertyType).Target.charactersLen =
                                        ((*SignaturePropertyType).Target.charactersLen as i32
                                            - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*SignaturePropertyType).Target.charactersLen as usize,
                                        ((*SignaturePropertyType).Target.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            grammar_id = 73 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            73 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*SignaturePropertyType).ANY.bytesLen,
                                &mut *((*SignaturePropertyType).ANY.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                4 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*SignaturePropertyType).set_ANY_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_DC_CPDReqEnergyTransferModeType(
    stream: &mut ExiBitstream,
    mut DC_CPDReqEnergyTransferModeType: *mut iso20_dc_DC_CPDReqEnergyTransferModeType,
) -> i32 {
    let mut grammar_id: i32 = 74 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_DC_CPDReqEnergyTransferModeType(DC_CPDReqEnergyTransferModeType);
    while done == 0 {
        match grammar_id {
            74 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DC_CPDReqEnergyTransferModeType).EVMaximumChargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 75 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            75 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DC_CPDReqEnergyTransferModeType).EVMinimumChargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 76 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            76 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DC_CPDReqEnergyTransferModeType).EVMaximumChargeCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 77 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            77 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DC_CPDReqEnergyTransferModeType).EVMinimumChargeCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 78 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            78 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DC_CPDReqEnergyTransferModeType).EVMaximumVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 79 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            79 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DC_CPDReqEnergyTransferModeType).EVMinimumVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 80 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            80 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_CPDReqEnergyTransferModeType).TargetSOC =
                                            value as int8_t;
                                        (*DC_CPDReqEnergyTransferModeType)
                                            .set_TargetSOC_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 2 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_DisplayParametersType(
    stream: &mut ExiBitstream,
    mut DisplayParametersType: *mut iso20_dc_DisplayParametersType,
) -> i32 {
    let mut grammar_id: i32 = 81 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_DisplayParametersType(DisplayParametersType);
    while done == 0 {
        match grammar_id {
            81 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 4 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).PresentSOC = value as int8_t;
                                        (*DisplayParametersType).set_PresentSOC_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 82 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_0,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).MinimumSOC = value_0 as int8_t;
                                        (*DisplayParametersType).set_MinimumSOC_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 83 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        2 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_1: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_1,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).TargetSOC = value_1 as int8_t;
                                        (*DisplayParametersType).set_TargetSOC_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 84 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        3 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_2: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_2,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).MaximumSOC = value_2 as int8_t;
                                        (*DisplayParametersType).set_MaximumSOC_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 85 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        4 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*DisplayParametersType).RemainingTimeToMinimumSOC,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType)
                                    .set_RemainingTimeToMinimumSOC_isUsed(1 as u32);
                                grammar_id = 86 as i32;
                            }
                        }
                        5 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*DisplayParametersType).RemainingTimeToTargetSOC,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType)
                                    .set_RemainingTimeToTargetSOC_isUsed(1 as u32);
                                grammar_id = 87 as i32;
                            }
                        }
                        6 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*DisplayParametersType).RemainingTimeToMaximumSOC,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType)
                                    .set_RemainingTimeToMaximumSOC_isUsed(1 as u32);
                                grammar_id = 88 as i32;
                            }
                        }
                        7 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_3: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_3,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).ChargingComplete = value_3 as i32;
                                        (*DisplayParametersType)
                                            .set_ChargingComplete_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 89 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        8 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DisplayParametersType).BatteryEnergyCapacity,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType).set_BatteryEnergyCapacity_isUsed(1 as u32);
                                grammar_id = 90 as i32;
                            }
                        }
                        9 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_4: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_4,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).InletHot = value_4 as i32;
                                        (*DisplayParametersType).set_InletHot_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 2 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        10 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            82 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 4 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_5: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_5,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).MinimumSOC = value_5 as int8_t;
                                        (*DisplayParametersType).set_MinimumSOC_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 83 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_6: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_6,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).TargetSOC = value_6 as int8_t;
                                        (*DisplayParametersType).set_TargetSOC_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 84 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        2 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_7: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_7,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).MaximumSOC = value_7 as int8_t;
                                        (*DisplayParametersType).set_MaximumSOC_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 85 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        3 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*DisplayParametersType).RemainingTimeToMinimumSOC,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType)
                                    .set_RemainingTimeToMinimumSOC_isUsed(1 as u32);
                                grammar_id = 86 as i32;
                            }
                        }
                        4 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*DisplayParametersType).RemainingTimeToTargetSOC,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType)
                                    .set_RemainingTimeToTargetSOC_isUsed(1 as u32);
                                grammar_id = 87 as i32;
                            }
                        }
                        5 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*DisplayParametersType).RemainingTimeToMaximumSOC,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType)
                                    .set_RemainingTimeToMaximumSOC_isUsed(1 as u32);
                                grammar_id = 88 as i32;
                            }
                        }
                        6 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_8: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_8,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).ChargingComplete = value_8 as i32;
                                        (*DisplayParametersType)
                                            .set_ChargingComplete_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 89 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        7 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DisplayParametersType).BatteryEnergyCapacity,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType).set_BatteryEnergyCapacity_isUsed(1 as u32);
                                grammar_id = 90 as i32;
                            }
                        }
                        8 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_9: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_9,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).InletHot = value_9 as i32;
                                        (*DisplayParametersType).set_InletHot_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 2 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        9 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            83 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 4 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_10: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_10,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).TargetSOC = value_10 as int8_t;
                                        (*DisplayParametersType).set_TargetSOC_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 84 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_11: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_11,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).MaximumSOC = value_11 as int8_t;
                                        (*DisplayParametersType).set_MaximumSOC_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 85 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        2 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*DisplayParametersType).RemainingTimeToMinimumSOC,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType)
                                    .set_RemainingTimeToMinimumSOC_isUsed(1 as u32);
                                grammar_id = 86 as i32;
                            }
                        }
                        3 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*DisplayParametersType).RemainingTimeToTargetSOC,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType)
                                    .set_RemainingTimeToTargetSOC_isUsed(1 as u32);
                                grammar_id = 87 as i32;
                            }
                        }
                        4 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*DisplayParametersType).RemainingTimeToMaximumSOC,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType)
                                    .set_RemainingTimeToMaximumSOC_isUsed(1 as u32);
                                grammar_id = 88 as i32;
                            }
                        }
                        5 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_12: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_12,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).ChargingComplete = value_12 as i32;
                                        (*DisplayParametersType)
                                            .set_ChargingComplete_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 89 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        6 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DisplayParametersType).BatteryEnergyCapacity,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType).set_BatteryEnergyCapacity_isUsed(1 as u32);
                                grammar_id = 90 as i32;
                            }
                        }
                        7 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_13: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_13,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).InletHot = value_13 as i32;
                                        (*DisplayParametersType).set_InletHot_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 2 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        8 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            84 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 4 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_14: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_14,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).MaximumSOC = value_14 as int8_t;
                                        (*DisplayParametersType).set_MaximumSOC_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 85 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*DisplayParametersType).RemainingTimeToMinimumSOC,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType)
                                    .set_RemainingTimeToMinimumSOC_isUsed(1 as u32);
                                grammar_id = 86 as i32;
                            }
                        }
                        2 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*DisplayParametersType).RemainingTimeToTargetSOC,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType)
                                    .set_RemainingTimeToTargetSOC_isUsed(1 as u32);
                                grammar_id = 87 as i32;
                            }
                        }
                        3 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*DisplayParametersType).RemainingTimeToMaximumSOC,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType)
                                    .set_RemainingTimeToMaximumSOC_isUsed(1 as u32);
                                grammar_id = 88 as i32;
                            }
                        }
                        4 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_15: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_15,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).ChargingComplete = value_15 as i32;
                                        (*DisplayParametersType)
                                            .set_ChargingComplete_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 89 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        5 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DisplayParametersType).BatteryEnergyCapacity,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType).set_BatteryEnergyCapacity_isUsed(1 as u32);
                                grammar_id = 90 as i32;
                            }
                        }
                        6 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_16: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_16,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).InletHot = value_16 as i32;
                                        (*DisplayParametersType).set_InletHot_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 2 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        7 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            85 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*DisplayParametersType).RemainingTimeToMinimumSOC,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType)
                                    .set_RemainingTimeToMinimumSOC_isUsed(1 as u32);
                                grammar_id = 86 as i32;
                            }
                        }
                        1 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*DisplayParametersType).RemainingTimeToTargetSOC,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType)
                                    .set_RemainingTimeToTargetSOC_isUsed(1 as u32);
                                grammar_id = 87 as i32;
                            }
                        }
                        2 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*DisplayParametersType).RemainingTimeToMaximumSOC,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType)
                                    .set_RemainingTimeToMaximumSOC_isUsed(1 as u32);
                                grammar_id = 88 as i32;
                            }
                        }
                        3 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_17: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_17,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).ChargingComplete = value_17 as i32;
                                        (*DisplayParametersType)
                                            .set_ChargingComplete_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 89 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        4 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DisplayParametersType).BatteryEnergyCapacity,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType).set_BatteryEnergyCapacity_isUsed(1 as u32);
                                grammar_id = 90 as i32;
                            }
                        }
                        5 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_18: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_18,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).InletHot = value_18 as i32;
                                        (*DisplayParametersType).set_InletHot_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 2 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        6 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            86 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*DisplayParametersType).RemainingTimeToTargetSOC,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType)
                                    .set_RemainingTimeToTargetSOC_isUsed(1 as u32);
                                grammar_id = 87 as i32;
                            }
                        }
                        1 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*DisplayParametersType).RemainingTimeToMaximumSOC,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType)
                                    .set_RemainingTimeToMaximumSOC_isUsed(1 as u32);
                                grammar_id = 88 as i32;
                            }
                        }
                        2 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_19: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_19,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).ChargingComplete = value_19 as i32;
                                        (*DisplayParametersType)
                                            .set_ChargingComplete_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 89 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        3 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DisplayParametersType).BatteryEnergyCapacity,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType).set_BatteryEnergyCapacity_isUsed(1 as u32);
                                grammar_id = 90 as i32;
                            }
                        }
                        4 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_20: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_20,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).InletHot = value_20 as i32;
                                        (*DisplayParametersType).set_InletHot_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 2 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        5 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            87 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*DisplayParametersType).RemainingTimeToMaximumSOC,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType)
                                    .set_RemainingTimeToMaximumSOC_isUsed(1 as u32);
                                grammar_id = 88 as i32;
                            }
                        }
                        1 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_21: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_21,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).ChargingComplete = value_21 as i32;
                                        (*DisplayParametersType)
                                            .set_ChargingComplete_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 89 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DisplayParametersType).BatteryEnergyCapacity,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType).set_BatteryEnergyCapacity_isUsed(1 as u32);
                                grammar_id = 90 as i32;
                            }
                        }
                        3 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_22: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_22,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).InletHot = value_22 as i32;
                                        (*DisplayParametersType).set_InletHot_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 2 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        4 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            88 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_23: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_23,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).ChargingComplete = value_23 as i32;
                                        (*DisplayParametersType)
                                            .set_ChargingComplete_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 89 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DisplayParametersType).BatteryEnergyCapacity,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType).set_BatteryEnergyCapacity_isUsed(1 as u32);
                                grammar_id = 90 as i32;
                            }
                        }
                        2 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_24: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_24,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).InletHot = value_24 as i32;
                                        (*DisplayParametersType).set_InletHot_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 2 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        3 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            89 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DisplayParametersType).BatteryEnergyCapacity,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType).set_BatteryEnergyCapacity_isUsed(1 as u32);
                                grammar_id = 90 as i32;
                            }
                        }
                        1 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_25: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_25,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).InletHot = value_25 as i32;
                                        (*DisplayParametersType).set_InletHot_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 2 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            90 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_26: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_26,
                                    );
                                    if error == 0 as i32 {
                                        (*DisplayParametersType).InletHot = value_26 as i32;
                                        (*DisplayParametersType).set_InletHot_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 2 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_DC_CPDResEnergyTransferModeType(
    stream: &mut ExiBitstream,
    mut DC_CPDResEnergyTransferModeType: *mut iso20_dc_DC_CPDResEnergyTransferModeType,
) -> i32 {
    let mut grammar_id: i32 = 91 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_DC_CPDResEnergyTransferModeType(DC_CPDResEnergyTransferModeType);
    while done == 0 {
        match grammar_id {
            91 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DC_CPDResEnergyTransferModeType).EVSEMaximumChargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 92 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            92 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DC_CPDResEnergyTransferModeType).EVSEMinimumChargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 93 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            93 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DC_CPDResEnergyTransferModeType).EVSEMaximumChargeCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 94 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            94 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DC_CPDResEnergyTransferModeType).EVSEMinimumChargeCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 95 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            95 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DC_CPDResEnergyTransferModeType).EVSEMaximumVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 96 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            96 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DC_CPDResEnergyTransferModeType).EVSEMinimumVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 97 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            97 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DC_CPDResEnergyTransferModeType).EVSEPowerRampLimitation,
                            );
                            if error == 0 as i32 {
                                (*DC_CPDResEnergyTransferModeType)
                                    .set_EVSEPowerRampLimitation_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_EVSEStatusType(
    stream: &mut ExiBitstream,
    mut EVSEStatusType: *mut iso20_dc_EVSEStatusType,
) -> i32 {
    let mut grammar_id: i32 = 98 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_EVSEStatusType(EVSEStatusType);
    while done == 0 {
        match grammar_id {
            98 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*EVSEStatusType).NotificationMaxDelay,
                            );
                            if error == 0 as i32 {
                                grammar_id = 99 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            99 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        3 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*EVSEStatusType).EVSENotification =
                                            value as iso20_dc_evseNotificationType;
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 2 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_MeterInfoType(
    stream: &mut ExiBitstream,
    mut MeterInfoType: *mut iso20_dc_MeterInfoType,
) -> i32 {
    let mut grammar_id: i32 = 100 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_MeterInfoType(MeterInfoType);
    while done == 0 {
        match grammar_id {
            100 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    error = exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut (*MeterInfoType).MeterID.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*MeterInfoType).MeterID.charactersLen as i32 >= 2 as i32
                                        {
                                            (*MeterInfoType).MeterID.charactersLen =
                                                ((*MeterInfoType).MeterID.charactersLen as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*MeterInfoType).MeterID.charactersLen as usize,
                                                ((*MeterInfoType).MeterID.characters).as_mut_ptr(),
                                                (32 as i32 + 1 as i32) as usize,
                                            );
                                        } else {
                                            error = -(200 as i32);
                                        }
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 101 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            101 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint64(
                                stream,
                                &mut (*MeterInfoType).ChargedEnergyReadingWh,
                            );
                            if error == 0 as i32 {
                                grammar_id = 102 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            102 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint64(
                                stream,
                                &mut (*MeterInfoType).BPT_DischargedEnergyReadingWh,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_BPT_DischargedEnergyReadingWh_isUsed(1 as u32);
                                grammar_id = 103 as i32;
                            }
                        }
                        1 => {
                            error = decode_exi_type_uint64(
                                stream,
                                &mut (*MeterInfoType).CapacitiveEnergyReadingVARh,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_CapacitiveEnergyReadingVARh_isUsed(1 as u32);
                                grammar_id = 104 as i32;
                            }
                        }
                        2 => {
                            error = decode_exi_type_uint64(
                                stream,
                                &mut (*MeterInfoType).BPT_InductiveEnergyReadingVARh,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType)
                                    .set_BPT_InductiveEnergyReadingVARh_isUsed(1 as u32);
                                grammar_id = 105 as i32;
                            }
                        }
                        3 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*MeterInfoType).MeterSignature.bytesLen,
                                &mut *((*MeterInfoType).MeterSignature.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                64 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_MeterSignature_isUsed(1 as u32);
                                grammar_id = 106 as i32;
                            }
                        }
                        4 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*MeterInfoType).MeterStatus,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_MeterStatus_isUsed(1 as u32);
                                grammar_id = 107 as i32;
                            }
                        }
                        5 => {
                            error = decode_exi_type_uint64(
                                stream,
                                &mut (*MeterInfoType).MeterTimestamp,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_MeterTimestamp_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        6 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            103 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint64(
                                stream,
                                &mut (*MeterInfoType).CapacitiveEnergyReadingVARh,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_CapacitiveEnergyReadingVARh_isUsed(1 as u32);
                                grammar_id = 104 as i32;
                            }
                        }
                        1 => {
                            error = decode_exi_type_uint64(
                                stream,
                                &mut (*MeterInfoType).BPT_InductiveEnergyReadingVARh,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType)
                                    .set_BPT_InductiveEnergyReadingVARh_isUsed(1 as u32);
                                grammar_id = 105 as i32;
                            }
                        }
                        2 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*MeterInfoType).MeterSignature.bytesLen,
                                &mut *((*MeterInfoType).MeterSignature.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                64 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_MeterSignature_isUsed(1 as u32);
                                grammar_id = 106 as i32;
                            }
                        }
                        3 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*MeterInfoType).MeterStatus,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_MeterStatus_isUsed(1 as u32);
                                grammar_id = 107 as i32;
                            }
                        }
                        4 => {
                            error = decode_exi_type_uint64(
                                stream,
                                &mut (*MeterInfoType).MeterTimestamp,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_MeterTimestamp_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        5 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            104 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint64(
                                stream,
                                &mut (*MeterInfoType).BPT_InductiveEnergyReadingVARh,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType)
                                    .set_BPT_InductiveEnergyReadingVARh_isUsed(1 as u32);
                                grammar_id = 105 as i32;
                            }
                        }
                        1 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*MeterInfoType).MeterSignature.bytesLen,
                                &mut *((*MeterInfoType).MeterSignature.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                64 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_MeterSignature_isUsed(1 as u32);
                                grammar_id = 106 as i32;
                            }
                        }
                        2 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*MeterInfoType).MeterStatus,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_MeterStatus_isUsed(1 as u32);
                                grammar_id = 107 as i32;
                            }
                        }
                        3 => {
                            error = decode_exi_type_uint64(
                                stream,
                                &mut (*MeterInfoType).MeterTimestamp,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_MeterTimestamp_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        4 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            105 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*MeterInfoType).MeterSignature.bytesLen,
                                &mut *((*MeterInfoType).MeterSignature.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                64 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_MeterSignature_isUsed(1 as u32);
                                grammar_id = 106 as i32;
                            }
                        }
                        1 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*MeterInfoType).MeterStatus,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_MeterStatus_isUsed(1 as u32);
                                grammar_id = 107 as i32;
                            }
                        }
                        2 => {
                            error = decode_exi_type_uint64(
                                stream,
                                &mut (*MeterInfoType).MeterTimestamp,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_MeterTimestamp_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        3 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            106 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*MeterInfoType).MeterStatus,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_MeterStatus_isUsed(1 as u32);
                                grammar_id = 107 as i32;
                            }
                        }
                        1 => {
                            error = decode_exi_type_uint64(
                                stream,
                                &mut (*MeterInfoType).MeterTimestamp,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_MeterTimestamp_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            107 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint64(
                                stream,
                                &mut (*MeterInfoType).MeterTimestamp,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_MeterTimestamp_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_Dynamic_DC_CLReqControlModeType(
    stream: &mut ExiBitstream,
    mut Dynamic_DC_CLReqControlModeType: *mut iso20_dc_Dynamic_DC_CLReqControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 108 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_Dynamic_DC_CLReqControlModeType(Dynamic_DC_CLReqControlModeType);
    while done == 0 {
        match grammar_id {
            108 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*Dynamic_DC_CLReqControlModeType).DepartureTime,
                            );
                            if error == 0 as i32 {
                                (*Dynamic_DC_CLReqControlModeType)
                                    .set_DepartureTime_isUsed(1 as u32);
                                grammar_id = 109 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Dynamic_DC_CLReqControlModeType).EVTargetEnergyRequest,
                            );
                            if error == 0 as i32 {
                                grammar_id = 110 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            109 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Dynamic_DC_CLReqControlModeType).EVTargetEnergyRequest,
                            );
                            if error == 0 as i32 {
                                grammar_id = 110 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            110 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Dynamic_DC_CLReqControlModeType).EVMaximumEnergyRequest,
                            );
                            if error == 0 as i32 {
                                grammar_id = 111 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            111 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Dynamic_DC_CLReqControlModeType).EVMinimumEnergyRequest,
                            );
                            if error == 0 as i32 {
                                grammar_id = 112 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            112 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Dynamic_DC_CLReqControlModeType).EVMaximumChargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 113 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            113 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Dynamic_DC_CLReqControlModeType).EVMinimumChargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 114 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            114 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Dynamic_DC_CLReqControlModeType).EVMaximumChargeCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 115 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            115 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Dynamic_DC_CLReqControlModeType).EVMaximumVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 116 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            116 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Dynamic_DC_CLReqControlModeType).EVMinimumVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_Scheduled_DC_CLReqControlModeType(
    stream: &mut ExiBitstream,
    mut Scheduled_DC_CLReqControlModeType: *mut iso20_dc_Scheduled_DC_CLReqControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 117 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_Scheduled_DC_CLReqControlModeType(Scheduled_DC_CLReqControlModeType);
    while done == 0 {
        match grammar_id {
            117 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLReqControlModeType).EVTargetEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLReqControlModeType)
                                    .set_EVTargetEnergyRequest_isUsed(1 as u32);
                                grammar_id = 118 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLReqControlModeType).EVMaximumEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumEnergyRequest_isUsed(1 as u32);
                                grammar_id = 119 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLReqControlModeType).EVMinimumEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLReqControlModeType)
                                    .set_EVMinimumEnergyRequest_isUsed(1 as u32);
                                grammar_id = 120 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLReqControlModeType).EVTargetCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 121 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            118 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLReqControlModeType).EVMaximumEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumEnergyRequest_isUsed(1 as u32);
                                grammar_id = 119 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLReqControlModeType).EVMinimumEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLReqControlModeType)
                                    .set_EVMinimumEnergyRequest_isUsed(1 as u32);
                                grammar_id = 120 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLReqControlModeType).EVTargetCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 121 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            119 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLReqControlModeType).EVMinimumEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLReqControlModeType)
                                    .set_EVMinimumEnergyRequest_isUsed(1 as u32);
                                grammar_id = 120 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLReqControlModeType).EVTargetCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 121 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            120 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLReqControlModeType).EVTargetCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 121 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            121 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLReqControlModeType).EVTargetVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 122 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            122 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLReqControlModeType).EVMaximumChargePower,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumChargePower_isUsed(1 as u32);
                                grammar_id = 123 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLReqControlModeType).EVMinimumChargePower,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLReqControlModeType)
                                    .set_EVMinimumChargePower_isUsed(1 as u32);
                                grammar_id = 124 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLReqControlModeType).EVMaximumChargeCurrent,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumChargeCurrent_isUsed(1 as u32);
                                grammar_id = 125 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLReqControlModeType).EVMaximumVoltage,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumVoltage_isUsed(1 as u32);
                                grammar_id = 126 as i32;
                            }
                        }
                        4 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLReqControlModeType).EVMinimumVoltage,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLReqControlModeType)
                                    .set_EVMinimumVoltage_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        5 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            123 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLReqControlModeType).EVMinimumChargePower,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLReqControlModeType)
                                    .set_EVMinimumChargePower_isUsed(1 as u32);
                                grammar_id = 124 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLReqControlModeType).EVMaximumChargeCurrent,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumChargeCurrent_isUsed(1 as u32);
                                grammar_id = 125 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLReqControlModeType).EVMaximumVoltage,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumVoltage_isUsed(1 as u32);
                                grammar_id = 126 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLReqControlModeType).EVMinimumVoltage,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLReqControlModeType)
                                    .set_EVMinimumVoltage_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        4 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            124 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLReqControlModeType).EVMaximumChargeCurrent,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumChargeCurrent_isUsed(1 as u32);
                                grammar_id = 125 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLReqControlModeType).EVMaximumVoltage,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumVoltage_isUsed(1 as u32);
                                grammar_id = 126 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLReqControlModeType).EVMinimumVoltage,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLReqControlModeType)
                                    .set_EVMinimumVoltage_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        3 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            125 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLReqControlModeType).EVMaximumVoltage,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumVoltage_isUsed(1 as u32);
                                grammar_id = 126 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLReqControlModeType).EVMinimumVoltage,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLReqControlModeType)
                                    .set_EVMinimumVoltage_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            126 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLReqControlModeType).EVMinimumVoltage,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLReqControlModeType)
                                    .set_EVMinimumVoltage_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_CLReqControlModeType(
    stream: &mut ExiBitstream,
    mut CLReqControlModeType: *mut iso20_dc_CLReqControlModeType,
) -> i32 {
    let mut eventCode: u32 = 0;
    let mut error: i32 = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
    if error == 0 as i32 {
        if eventCode != 0 as i32 as u32 {
            error = -(150 as i32);
        }
    }
    return error;
}
unsafe extern "C" fn decode_iso20_dc_ReceiptType(
    stream: &mut ExiBitstream,
    mut ReceiptType: *mut iso20_dc_ReceiptType,
) -> i32 {
    let mut grammar_id: i32 = 127 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_ReceiptType(ReceiptType);
    while done == 0 {
        match grammar_id {
            127 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint64(stream, &mut (*ReceiptType).TimeAnchor);
                            if error == 0 as i32 {
                                grammar_id = 128 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            128 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).EnergyCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_EnergyCosts_isUsed(1 as u32);
                                grammar_id = 130 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).OccupancyCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_OccupancyCosts_isUsed(1 as u32);
                                grammar_id = 132 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).AdditionalServicesCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_AdditionalServicesCosts_isUsed(1 as u32);
                                grammar_id = 134 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_dc_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).OverstayCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_OverstayCosts_isUsed(1 as u32);
                                grammar_id = 136 as i32;
                            }
                        }
                        4 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh2 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_dc_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh2 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 129 as i32;
                        }
                        5 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            129 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh3 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_dc_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh3 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                grammar_id = 129 as i32;
                            } else {
                                grammar_id = 130 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            130 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).OccupancyCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_OccupancyCosts_isUsed(1 as u32);
                                grammar_id = 132 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).AdditionalServicesCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_AdditionalServicesCosts_isUsed(1 as u32);
                                grammar_id = 134 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).OverstayCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_OverstayCosts_isUsed(1 as u32);
                                grammar_id = 136 as i32;
                            }
                        }
                        3 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh4 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_dc_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh4 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 131 as i32;
                        }
                        4 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            131 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh5 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_dc_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh5 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                grammar_id = 131 as i32;
                            } else {
                                grammar_id = 132 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            132 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).AdditionalServicesCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_AdditionalServicesCosts_isUsed(1 as u32);
                                grammar_id = 134 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).OverstayCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_OverstayCosts_isUsed(1 as u32);
                                grammar_id = 136 as i32;
                            }
                        }
                        2 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh6 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_dc_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh6 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 133 as i32;
                        }
                        3 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            133 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh7 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_dc_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh7 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                grammar_id = 133 as i32;
                            } else {
                                grammar_id = 134 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            134 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).OverstayCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_OverstayCosts_isUsed(1 as u32);
                                grammar_id = 136 as i32;
                            }
                        }
                        1 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh8 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_dc_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh8 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 135 as i32;
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            135 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh9 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_dc_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh9 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                grammar_id = 135 as i32;
                            } else {
                                grammar_id = 136 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            136 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh10 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_dc_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh10 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 137 as i32;
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            137 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh11 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_dc_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh11 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                grammar_id = 137 as i32;
                            } else {
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_Dynamic_DC_CLResControlModeType(
    stream: &mut ExiBitstream,
    mut Dynamic_DC_CLResControlModeType: *mut iso20_dc_Dynamic_DC_CLResControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 138 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_Dynamic_DC_CLResControlModeType(Dynamic_DC_CLResControlModeType);
    while done == 0 {
        match grammar_id {
            138 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*Dynamic_DC_CLResControlModeType).DepartureTime,
                            );
                            if error == 0 as i32 {
                                (*Dynamic_DC_CLResControlModeType)
                                    .set_DepartureTime_isUsed(1 as u32);
                                grammar_id = 139 as i32;
                            }
                        }
                        1 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*Dynamic_DC_CLResControlModeType).MinimumSOC =
                                            value as int8_t;
                                        (*Dynamic_DC_CLResControlModeType)
                                            .set_MinimumSOC_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 140 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        2 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_0,
                                    );
                                    if error == 0 as i32 {
                                        (*Dynamic_DC_CLResControlModeType).TargetSOC =
                                            value_0 as int8_t;
                                        (*Dynamic_DC_CLResControlModeType)
                                            .set_TargetSOC_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 141 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        3 => {
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*Dynamic_DC_CLResControlModeType).AckMaxDelay,
                            );
                            if error == 0 as i32 {
                                (*Dynamic_DC_CLResControlModeType).set_AckMaxDelay_isUsed(1 as u32);
                                grammar_id = 142 as i32;
                            }
                        }
                        4 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Dynamic_DC_CLResControlModeType).EVSEMaximumChargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 143 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            139 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_1: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_1,
                                    );
                                    if error == 0 as i32 {
                                        (*Dynamic_DC_CLResControlModeType).MinimumSOC =
                                            value_1 as int8_t;
                                        (*Dynamic_DC_CLResControlModeType)
                                            .set_MinimumSOC_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 140 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_2: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_2,
                                    );
                                    if error == 0 as i32 {
                                        (*Dynamic_DC_CLResControlModeType).TargetSOC =
                                            value_2 as int8_t;
                                        (*Dynamic_DC_CLResControlModeType)
                                            .set_TargetSOC_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 141 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        2 => {
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*Dynamic_DC_CLResControlModeType).AckMaxDelay,
                            );
                            if error == 0 as i32 {
                                (*Dynamic_DC_CLResControlModeType).set_AckMaxDelay_isUsed(1 as u32);
                                grammar_id = 142 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Dynamic_DC_CLResControlModeType).EVSEMaximumChargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 143 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            140 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_3: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_3,
                                    );
                                    if error == 0 as i32 {
                                        (*Dynamic_DC_CLResControlModeType).TargetSOC =
                                            value_3 as int8_t;
                                        (*Dynamic_DC_CLResControlModeType)
                                            .set_TargetSOC_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 141 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*Dynamic_DC_CLResControlModeType).AckMaxDelay,
                            );
                            if error == 0 as i32 {
                                (*Dynamic_DC_CLResControlModeType).set_AckMaxDelay_isUsed(1 as u32);
                                grammar_id = 142 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Dynamic_DC_CLResControlModeType).EVSEMaximumChargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 143 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            141 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*Dynamic_DC_CLResControlModeType).AckMaxDelay,
                            );
                            if error == 0 as i32 {
                                (*Dynamic_DC_CLResControlModeType).set_AckMaxDelay_isUsed(1 as u32);
                                grammar_id = 142 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Dynamic_DC_CLResControlModeType).EVSEMaximumChargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 143 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            142 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Dynamic_DC_CLResControlModeType).EVSEMaximumChargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 143 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            143 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Dynamic_DC_CLResControlModeType).EVSEMinimumChargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 144 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            144 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Dynamic_DC_CLResControlModeType).EVSEMaximumChargeCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 145 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            145 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Dynamic_DC_CLResControlModeType).EVSEMaximumVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_Scheduled_DC_CLResControlModeType(
    stream: &mut ExiBitstream,
    mut Scheduled_DC_CLResControlModeType: *mut iso20_dc_Scheduled_DC_CLResControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 146 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_Scheduled_DC_CLResControlModeType(Scheduled_DC_CLResControlModeType);
    while done == 0 {
        match grammar_id {
            146 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLResControlModeType).EVSEMaximumChargePower,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumChargePower_isUsed(1 as u32);
                                grammar_id = 147 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLResControlModeType).EVSEMinimumChargePower,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMinimumChargePower_isUsed(1 as u32);
                                grammar_id = 148 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLResControlModeType).EVSEMaximumChargeCurrent,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumChargeCurrent_isUsed(1 as u32);
                                grammar_id = 149 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLResControlModeType).EVSEMaximumVoltage,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumVoltage_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        4 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            147 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLResControlModeType).EVSEMinimumChargePower,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMinimumChargePower_isUsed(1 as u32);
                                grammar_id = 148 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLResControlModeType).EVSEMaximumChargeCurrent,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumChargeCurrent_isUsed(1 as u32);
                                grammar_id = 149 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLResControlModeType).EVSEMaximumVoltage,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumVoltage_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        3 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            148 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLResControlModeType).EVSEMaximumChargeCurrent,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumChargeCurrent_isUsed(1 as u32);
                                grammar_id = 149 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLResControlModeType).EVSEMaximumVoltage,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumVoltage_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            149 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*Scheduled_DC_CLResControlModeType).EVSEMaximumVoltage,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumVoltage_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_CLResControlModeType(
    stream: &mut ExiBitstream,
    mut CLResControlModeType: *mut iso20_dc_CLResControlModeType,
) -> i32 {
    let mut eventCode: u32 = 0;
    let mut error: i32 = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
    if error == 0 as i32 {
        if eventCode != 0 as i32 as u32 {
            error = -(150 as i32);
        }
    }
    return error;
}
unsafe extern "C" fn decode_iso20_dc_DC_CableCheckReqType(
    stream: &mut ExiBitstream,
    mut DC_CableCheckReqType: *mut iso20_dc_DC_CableCheckReqType,
) -> i32 {
    let mut grammar_id: i32 = 150 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_DC_CableCheckReqType(DC_CableCheckReqType);
    while done == 0 {
        match grammar_id {
            150 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_MessageHeaderType(
                                stream,
                                &mut (*DC_CableCheckReqType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_DC_CableCheckResType(
    stream: &mut ExiBitstream,
    mut DC_CableCheckResType: *mut iso20_dc_DC_CableCheckResType,
) -> i32 {
    let mut grammar_id: i32 = 151 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_DC_CableCheckResType(DC_CableCheckResType);
    while done == 0 {
        match grammar_id {
            151 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_MessageHeaderType(
                                stream,
                                &mut (*DC_CableCheckResType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 152 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            152 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        6 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_CableCheckResType).ResponseCode =
                                            value as iso20_dc_responseCodeType;
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 153 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            153 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value_0,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_CableCheckResType).EVSEProcessing =
                                            value_0 as iso20_dc_processingType;
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 2 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_DC_PreChargeReqType(
    stream: &mut ExiBitstream,
    mut DC_PreChargeReqType: *mut iso20_dc_DC_PreChargeReqType,
) -> i32 {
    let mut grammar_id: i32 = 154 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_DC_PreChargeReqType(DC_PreChargeReqType);
    while done == 0 {
        match grammar_id {
            154 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_MessageHeaderType(
                                stream,
                                &mut (*DC_PreChargeReqType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 155 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            155 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_PreChargeReqType).EVProcessing =
                                            value as iso20_dc_processingType;
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 156 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            156 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DC_PreChargeReqType).EVPresentVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 157 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            157 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DC_PreChargeReqType).EVTargetVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_DC_PreChargeResType(
    stream: &mut ExiBitstream,
    mut DC_PreChargeResType: *mut iso20_dc_DC_PreChargeResType,
) -> i32 {
    let mut grammar_id: i32 = 158 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_DC_PreChargeResType(DC_PreChargeResType);
    while done == 0 {
        match grammar_id {
            158 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_MessageHeaderType(
                                stream,
                                &mut (*DC_PreChargeResType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 159 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            159 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        6 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_PreChargeResType).ResponseCode =
                                            value as iso20_dc_responseCodeType;
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 160 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            160 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DC_PreChargeResType).EVSEPresentVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_DC_WeldingDetectionReqType(
    stream: &mut ExiBitstream,
    mut DC_WeldingDetectionReqType: *mut iso20_dc_DC_WeldingDetectionReqType,
) -> i32 {
    let mut grammar_id: i32 = 161 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_DC_WeldingDetectionReqType(DC_WeldingDetectionReqType);
    while done == 0 {
        match grammar_id {
            161 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_MessageHeaderType(
                                stream,
                                &mut (*DC_WeldingDetectionReqType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 162 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            162 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_WeldingDetectionReqType).EVProcessing =
                                            value as iso20_dc_processingType;
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 2 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_DC_WeldingDetectionResType(
    stream: &mut ExiBitstream,
    mut DC_WeldingDetectionResType: *mut iso20_dc_DC_WeldingDetectionResType,
) -> i32 {
    let mut grammar_id: i32 = 163 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_DC_WeldingDetectionResType(DC_WeldingDetectionResType);
    while done == 0 {
        match grammar_id {
            163 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_MessageHeaderType(
                                stream,
                                &mut (*DC_WeldingDetectionResType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 164 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            164 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        6 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_WeldingDetectionResType).ResponseCode =
                                            value as iso20_dc_responseCodeType;
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 165 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            165 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DC_WeldingDetectionResType).EVSEPresentVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_BPT_DC_CPDReqEnergyTransferModeType(
    stream: &mut ExiBitstream,
    mut BPT_DC_CPDReqEnergyTransferModeType: *mut iso20_dc_BPT_DC_CPDReqEnergyTransferModeType,
) -> i32 {
    let mut grammar_id: i32 = 166 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_BPT_DC_CPDReqEnergyTransferModeType(BPT_DC_CPDReqEnergyTransferModeType);
    while done == 0 {
        match grammar_id {
            166 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_DC_CPDReqEnergyTransferModeType).EVMaximumChargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 167 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            167 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_DC_CPDReqEnergyTransferModeType).EVMinimumChargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 168 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            168 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_DC_CPDReqEnergyTransferModeType).EVMaximumChargeCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 169 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            169 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_DC_CPDReqEnergyTransferModeType).EVMinimumChargeCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 170 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            170 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_DC_CPDReqEnergyTransferModeType).EVMaximumVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 171 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            171 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_DC_CPDReqEnergyTransferModeType).EVMinimumVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 172 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            172 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*BPT_DC_CPDReqEnergyTransferModeType).TargetSOC =
                                            value as int8_t;
                                        (*BPT_DC_CPDReqEnergyTransferModeType)
                                            .set_TargetSOC_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 173 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_DC_CPDReqEnergyTransferModeType).EVMaximumDischargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 174 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            173 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_DC_CPDReqEnergyTransferModeType).EVMaximumDischargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 174 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            174 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_DC_CPDReqEnergyTransferModeType).EVMinimumDischargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 175 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            175 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_DC_CPDReqEnergyTransferModeType)
                                    .EVMaximumDischargeCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 176 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            176 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_DC_CPDReqEnergyTransferModeType)
                                    .EVMinimumDischargeCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_DC_ChargeParameterDiscoveryReqType(
    stream: &mut ExiBitstream,
    mut DC_ChargeParameterDiscoveryReqType: *mut iso20_dc_DC_ChargeParameterDiscoveryReqType,
) -> i32 {
    let mut grammar_id: i32 = 177 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_DC_ChargeParameterDiscoveryReqType(DC_ChargeParameterDiscoveryReqType);
    while done == 0 {
        match grammar_id {
            177 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_MessageHeaderType(
                                stream,
                                &mut (*DC_ChargeParameterDiscoveryReqType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 178 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            178 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_BPT_DC_CPDReqEnergyTransferModeType(
                                stream,
                                &mut (*DC_ChargeParameterDiscoveryReqType)
                                    .BPT_DC_CPDReqEnergyTransferMode,
                            );
                            if error == 0 as i32 {
                                (*DC_ChargeParameterDiscoveryReqType)
                                    .set_BPT_DC_CPDReqEnergyTransferMode_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_DC_CPDReqEnergyTransferModeType(
                                stream,
                                &mut (*DC_ChargeParameterDiscoveryReqType)
                                    .DC_CPDReqEnergyTransferMode,
                            );
                            if error == 0 as i32 {
                                (*DC_ChargeParameterDiscoveryReqType)
                                    .set_DC_CPDReqEnergyTransferMode_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_BPT_DC_CPDResEnergyTransferModeType(
    stream: &mut ExiBitstream,
    mut BPT_DC_CPDResEnergyTransferModeType: *mut iso20_dc_BPT_DC_CPDResEnergyTransferModeType,
) -> i32 {
    let mut grammar_id: i32 = 179 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_BPT_DC_CPDResEnergyTransferModeType(BPT_DC_CPDResEnergyTransferModeType);
    while done == 0 {
        match grammar_id {
            179 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_DC_CPDResEnergyTransferModeType).EVSEMaximumChargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 180 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            180 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_DC_CPDResEnergyTransferModeType).EVSEMinimumChargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 181 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            181 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_DC_CPDResEnergyTransferModeType)
                                    .EVSEMaximumChargeCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 182 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            182 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_DC_CPDResEnergyTransferModeType)
                                    .EVSEMinimumChargeCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 183 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            183 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_DC_CPDResEnergyTransferModeType).EVSEMaximumVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 184 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            184 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_DC_CPDResEnergyTransferModeType).EVSEMinimumVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 185 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            185 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_DC_CPDResEnergyTransferModeType).EVSEPowerRampLimitation,
                            );
                            if error == 0 as i32 {
                                (*BPT_DC_CPDResEnergyTransferModeType)
                                    .set_EVSEPowerRampLimitation_isUsed(1 as u32);
                                grammar_id = 186 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_DC_CPDResEnergyTransferModeType)
                                    .EVSEMaximumDischargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 187 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            186 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_DC_CPDResEnergyTransferModeType)
                                    .EVSEMaximumDischargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 187 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            187 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_DC_CPDResEnergyTransferModeType)
                                    .EVSEMinimumDischargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 188 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            188 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_DC_CPDResEnergyTransferModeType)
                                    .EVSEMaximumDischargeCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 189 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            189 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_DC_CPDResEnergyTransferModeType)
                                    .EVSEMinimumDischargeCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_DC_ChargeParameterDiscoveryResType(
    stream: &mut ExiBitstream,
    mut DC_ChargeParameterDiscoveryResType: *mut iso20_dc_DC_ChargeParameterDiscoveryResType,
) -> i32 {
    let mut grammar_id: i32 = 190 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_DC_ChargeParameterDiscoveryResType(DC_ChargeParameterDiscoveryResType);
    while done == 0 {
        match grammar_id {
            190 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_MessageHeaderType(
                                stream,
                                &mut (*DC_ChargeParameterDiscoveryResType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 191 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            191 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        6 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_ChargeParameterDiscoveryResType).ResponseCode =
                                            value as iso20_dc_responseCodeType;
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 192 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            192 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_BPT_DC_CPDResEnergyTransferModeType(
                                stream,
                                &mut (*DC_ChargeParameterDiscoveryResType)
                                    .BPT_DC_CPDResEnergyTransferMode,
                            );
                            if error == 0 as i32 {
                                (*DC_ChargeParameterDiscoveryResType)
                                    .set_BPT_DC_CPDResEnergyTransferMode_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_DC_CPDResEnergyTransferModeType(
                                stream,
                                &mut (*DC_ChargeParameterDiscoveryResType)
                                    .DC_CPDResEnergyTransferMode,
                            );
                            if error == 0 as i32 {
                                (*DC_ChargeParameterDiscoveryResType)
                                    .set_DC_CPDResEnergyTransferMode_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_BPT_Scheduled_DC_CLReqControlModeType(
    stream: &mut ExiBitstream,
    mut BPT_Scheduled_DC_CLReqControlModeType: *mut iso20_dc_BPT_Scheduled_DC_CLReqControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 193 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_BPT_Scheduled_DC_CLReqControlModeType(BPT_Scheduled_DC_CLReqControlModeType);
    while done == 0 {
        match grammar_id {
            193 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType).EVTargetEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVTargetEnergyRequest_isUsed(1 as u32);
                                grammar_id = 194 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMaximumEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumEnergyRequest_isUsed(1 as u32);
                                grammar_id = 195 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMinimumEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMinimumEnergyRequest_isUsed(1 as u32);
                                grammar_id = 196 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType).EVTargetCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 197 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            194 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMaximumEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumEnergyRequest_isUsed(1 as u32);
                                grammar_id = 195 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMinimumEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMinimumEnergyRequest_isUsed(1 as u32);
                                grammar_id = 196 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType).EVTargetCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 197 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            195 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMinimumEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMinimumEnergyRequest_isUsed(1 as u32);
                                grammar_id = 196 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType).EVTargetCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 197 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            196 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType).EVTargetCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 197 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            197 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType).EVTargetVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 198 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            198 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 4 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType).EVMaximumChargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumChargePower_isUsed(1 as u32);
                                grammar_id = 199 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType).EVMinimumChargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMinimumChargePower_isUsed(1 as u32);
                                grammar_id = 200 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMaximumChargeCurrent,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumChargeCurrent_isUsed(1 as u32);
                                grammar_id = 201 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType).EVMaximumVoltage,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumVoltage_isUsed(1 as u32);
                                grammar_id = 202 as i32;
                            }
                        }
                        4 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType).EVMinimumVoltage,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMinimumVoltage_isUsed(1 as u32);
                                grammar_id = 203 as i32;
                            }
                        }
                        5 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMaximumDischargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumDischargePower_isUsed(1 as u32);
                                grammar_id = 204 as i32;
                            }
                        }
                        6 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMinimumDischargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMinimumDischargePower_isUsed(1 as u32);
                                grammar_id = 205 as i32;
                            }
                        }
                        7 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMaximumDischargeCurrent,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumDischargeCurrent_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        8 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            199 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 4 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType).EVMinimumChargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMinimumChargePower_isUsed(1 as u32);
                                grammar_id = 200 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMaximumChargeCurrent,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumChargeCurrent_isUsed(1 as u32);
                                grammar_id = 201 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType).EVMaximumVoltage,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumVoltage_isUsed(1 as u32);
                                grammar_id = 202 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType).EVMinimumVoltage,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMinimumVoltage_isUsed(1 as u32);
                                grammar_id = 203 as i32;
                            }
                        }
                        4 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMaximumDischargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumDischargePower_isUsed(1 as u32);
                                grammar_id = 204 as i32;
                            }
                        }
                        5 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMinimumDischargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMinimumDischargePower_isUsed(1 as u32);
                                grammar_id = 205 as i32;
                            }
                        }
                        6 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMaximumDischargeCurrent,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumDischargeCurrent_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        7 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            200 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMaximumChargeCurrent,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumChargeCurrent_isUsed(1 as u32);
                                grammar_id = 201 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType).EVMaximumVoltage,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumVoltage_isUsed(1 as u32);
                                grammar_id = 202 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType).EVMinimumVoltage,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMinimumVoltage_isUsed(1 as u32);
                                grammar_id = 203 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMaximumDischargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumDischargePower_isUsed(1 as u32);
                                grammar_id = 204 as i32;
                            }
                        }
                        4 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMinimumDischargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMinimumDischargePower_isUsed(1 as u32);
                                grammar_id = 205 as i32;
                            }
                        }
                        5 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMaximumDischargeCurrent,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumDischargeCurrent_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        6 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            201 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType).EVMaximumVoltage,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumVoltage_isUsed(1 as u32);
                                grammar_id = 202 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType).EVMinimumVoltage,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMinimumVoltage_isUsed(1 as u32);
                                grammar_id = 203 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMaximumDischargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumDischargePower_isUsed(1 as u32);
                                grammar_id = 204 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMinimumDischargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMinimumDischargePower_isUsed(1 as u32);
                                grammar_id = 205 as i32;
                            }
                        }
                        4 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMaximumDischargeCurrent,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumDischargeCurrent_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        5 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            202 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType).EVMinimumVoltage,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMinimumVoltage_isUsed(1 as u32);
                                grammar_id = 203 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMaximumDischargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumDischargePower_isUsed(1 as u32);
                                grammar_id = 204 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMinimumDischargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMinimumDischargePower_isUsed(1 as u32);
                                grammar_id = 205 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMaximumDischargeCurrent,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumDischargeCurrent_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        4 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            203 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMaximumDischargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumDischargePower_isUsed(1 as u32);
                                grammar_id = 204 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMinimumDischargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMinimumDischargePower_isUsed(1 as u32);
                                grammar_id = 205 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMaximumDischargeCurrent,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumDischargeCurrent_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        3 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            204 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMinimumDischargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMinimumDischargePower_isUsed(1 as u32);
                                grammar_id = 205 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMaximumDischargeCurrent,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumDischargeCurrent_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            205 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .EVMaximumDischargeCurrent,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLReqControlModeType)
                                    .set_EVMaximumDischargeCurrent_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_BPT_Scheduled_DC_CLResControlModeType(
    stream: &mut ExiBitstream,
    mut BPT_Scheduled_DC_CLResControlModeType: *mut iso20_dc_BPT_Scheduled_DC_CLResControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 206 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_BPT_Scheduled_DC_CLResControlModeType(BPT_Scheduled_DC_CLResControlModeType);
    while done == 0 {
        match grammar_id {
            206 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 4 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType)
                                    .EVSEMaximumChargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumChargePower_isUsed(1 as u32);
                                grammar_id = 207 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType)
                                    .EVSEMinimumChargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMinimumChargePower_isUsed(1 as u32);
                                grammar_id = 208 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType)
                                    .EVSEMaximumChargeCurrent,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumChargeCurrent_isUsed(1 as u32);
                                grammar_id = 209 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType).EVSEMaximumVoltage,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumVoltage_isUsed(1 as u32);
                                grammar_id = 210 as i32;
                            }
                        }
                        4 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType)
                                    .EVSEMaximumDischargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumDischargePower_isUsed(1 as u32);
                                grammar_id = 211 as i32;
                            }
                        }
                        5 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType)
                                    .EVSEMinimumDischargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMinimumDischargePower_isUsed(1 as u32);
                                grammar_id = 212 as i32;
                            }
                        }
                        6 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType)
                                    .EVSEMaximumDischargeCurrent,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumDischargeCurrent_isUsed(1 as u32);
                                grammar_id = 213 as i32;
                            }
                        }
                        7 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType).EVSEMinimumVoltage,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMinimumVoltage_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        8 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            207 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 4 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType)
                                    .EVSEMinimumChargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMinimumChargePower_isUsed(1 as u32);
                                grammar_id = 208 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType)
                                    .EVSEMaximumChargeCurrent,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumChargeCurrent_isUsed(1 as u32);
                                grammar_id = 209 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType).EVSEMaximumVoltage,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumVoltage_isUsed(1 as u32);
                                grammar_id = 210 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType)
                                    .EVSEMaximumDischargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumDischargePower_isUsed(1 as u32);
                                grammar_id = 211 as i32;
                            }
                        }
                        4 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType)
                                    .EVSEMinimumDischargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMinimumDischargePower_isUsed(1 as u32);
                                grammar_id = 212 as i32;
                            }
                        }
                        5 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType)
                                    .EVSEMaximumDischargeCurrent,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumDischargeCurrent_isUsed(1 as u32);
                                grammar_id = 213 as i32;
                            }
                        }
                        6 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType).EVSEMinimumVoltage,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMinimumVoltage_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        7 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            208 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType)
                                    .EVSEMaximumChargeCurrent,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumChargeCurrent_isUsed(1 as u32);
                                grammar_id = 209 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType).EVSEMaximumVoltage,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumVoltage_isUsed(1 as u32);
                                grammar_id = 210 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType)
                                    .EVSEMaximumDischargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumDischargePower_isUsed(1 as u32);
                                grammar_id = 211 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType)
                                    .EVSEMinimumDischargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMinimumDischargePower_isUsed(1 as u32);
                                grammar_id = 212 as i32;
                            }
                        }
                        4 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType)
                                    .EVSEMaximumDischargeCurrent,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumDischargeCurrent_isUsed(1 as u32);
                                grammar_id = 213 as i32;
                            }
                        }
                        5 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType).EVSEMinimumVoltage,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMinimumVoltage_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        6 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            209 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType).EVSEMaximumVoltage,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumVoltage_isUsed(1 as u32);
                                grammar_id = 210 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType)
                                    .EVSEMaximumDischargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumDischargePower_isUsed(1 as u32);
                                grammar_id = 211 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType)
                                    .EVSEMinimumDischargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMinimumDischargePower_isUsed(1 as u32);
                                grammar_id = 212 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType)
                                    .EVSEMaximumDischargeCurrent,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumDischargeCurrent_isUsed(1 as u32);
                                grammar_id = 213 as i32;
                            }
                        }
                        4 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType).EVSEMinimumVoltage,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMinimumVoltage_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        5 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            210 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType)
                                    .EVSEMaximumDischargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumDischargePower_isUsed(1 as u32);
                                grammar_id = 211 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType)
                                    .EVSEMinimumDischargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMinimumDischargePower_isUsed(1 as u32);
                                grammar_id = 212 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType)
                                    .EVSEMaximumDischargeCurrent,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumDischargeCurrent_isUsed(1 as u32);
                                grammar_id = 213 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType).EVSEMinimumVoltage,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMinimumVoltage_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        4 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            211 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType)
                                    .EVSEMinimumDischargePower,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMinimumDischargePower_isUsed(1 as u32);
                                grammar_id = 212 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType)
                                    .EVSEMaximumDischargeCurrent,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumDischargeCurrent_isUsed(1 as u32);
                                grammar_id = 213 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType).EVSEMinimumVoltage,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMinimumVoltage_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        3 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            212 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType)
                                    .EVSEMaximumDischargeCurrent,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMaximumDischargeCurrent_isUsed(1 as u32);
                                grammar_id = 213 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType).EVSEMinimumVoltage,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMinimumVoltage_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            213 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Scheduled_DC_CLResControlModeType).EVSEMinimumVoltage,
                            );
                            if error == 0 as i32 {
                                (*BPT_Scheduled_DC_CLResControlModeType)
                                    .set_EVSEMinimumVoltage_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_BPT_Dynamic_DC_CLReqControlModeType(
    stream: &mut ExiBitstream,
    mut BPT_Dynamic_DC_CLReqControlModeType: *mut iso20_dc_BPT_Dynamic_DC_CLReqControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 214 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_BPT_Dynamic_DC_CLReqControlModeType(BPT_Dynamic_DC_CLReqControlModeType);
    while done == 0 {
        match grammar_id {
            214 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLReqControlModeType).DepartureTime,
                            );
                            if error == 0 as i32 {
                                (*BPT_Dynamic_DC_CLReqControlModeType)
                                    .set_DepartureTime_isUsed(1 as u32);
                                grammar_id = 215 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLReqControlModeType).EVTargetEnergyRequest,
                            );
                            if error == 0 as i32 {
                                grammar_id = 216 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            215 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLReqControlModeType).EVTargetEnergyRequest,
                            );
                            if error == 0 as i32 {
                                grammar_id = 216 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            216 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLReqControlModeType).EVMaximumEnergyRequest,
                            );
                            if error == 0 as i32 {
                                grammar_id = 217 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            217 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLReqControlModeType).EVMinimumEnergyRequest,
                            );
                            if error == 0 as i32 {
                                grammar_id = 218 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            218 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLReqControlModeType).EVMaximumChargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 219 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            219 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLReqControlModeType).EVMinimumChargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 220 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            220 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLReqControlModeType).EVMaximumChargeCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 221 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            221 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLReqControlModeType).EVMaximumVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 222 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            222 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLReqControlModeType).EVMinimumVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 223 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            223 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLReqControlModeType).EVMaximumDischargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 224 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            224 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLReqControlModeType).EVMinimumDischargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 225 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            225 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLReqControlModeType)
                                    .EVMaximumDischargeCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 226 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            226 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLReqControlModeType)
                                    .EVMaximumV2XEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*BPT_Dynamic_DC_CLReqControlModeType)
                                    .set_EVMaximumV2XEnergyRequest_isUsed(1 as u32);
                                grammar_id = 227 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLReqControlModeType)
                                    .EVMinimumV2XEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*BPT_Dynamic_DC_CLReqControlModeType)
                                    .set_EVMinimumV2XEnergyRequest_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            227 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLReqControlModeType)
                                    .EVMinimumV2XEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*BPT_Dynamic_DC_CLReqControlModeType)
                                    .set_EVMinimumV2XEnergyRequest_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_DC_ChargeLoopReqType(
    stream: &mut ExiBitstream,
    mut DC_ChargeLoopReqType: *mut iso20_dc_DC_ChargeLoopReqType,
) -> i32 {
    let mut grammar_id: i32 = 228 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_DC_ChargeLoopReqType(DC_ChargeLoopReqType);
    while done == 0 {
        match grammar_id {
            228 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_MessageHeaderType(
                                stream,
                                &mut (*DC_ChargeLoopReqType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 229 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            229 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_DisplayParametersType(
                                stream,
                                &mut (*DC_ChargeLoopReqType).DisplayParameters,
                            );
                            if error == 0 as i32 {
                                (*DC_ChargeLoopReqType).set_DisplayParameters_isUsed(1 as u32);
                                grammar_id = 230 as i32;
                            }
                        }
                        1 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_ChargeLoopReqType).MeterInfoRequested = value as i32;
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 231 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            230 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_0,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_ChargeLoopReqType).MeterInfoRequested = value_0 as i32;
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 231 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            231 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DC_ChargeLoopReqType).EVPresentVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 232 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            232 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_BPT_Dynamic_DC_CLReqControlModeType(
                                stream,
                                &mut (*DC_ChargeLoopReqType).BPT_Dynamic_DC_CLReqControlMode,
                            );
                            if error == 0 as i32 {
                                (*DC_ChargeLoopReqType)
                                    .set_BPT_Dynamic_DC_CLReqControlMode_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_BPT_Scheduled_DC_CLReqControlModeType(
                                stream,
                                &mut (*DC_ChargeLoopReqType).BPT_Scheduled_DC_CLReqControlMode,
                            );
                            if error == 0 as i32 {
                                (*DC_ChargeLoopReqType)
                                    .set_BPT_Scheduled_DC_CLReqControlMode_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_CLReqControlModeType(
                                stream,
                                &mut (*DC_ChargeLoopReqType).CLReqControlMode,
                            );
                            if error == 0 as i32 {
                                (*DC_ChargeLoopReqType).set_CLReqControlMode_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_dc_Dynamic_DC_CLReqControlModeType(
                                stream,
                                &mut (*DC_ChargeLoopReqType).Dynamic_DC_CLReqControlMode,
                            );
                            if error == 0 as i32 {
                                (*DC_ChargeLoopReqType)
                                    .set_Dynamic_DC_CLReqControlMode_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        4 => {
                            error = decode_iso20_dc_Scheduled_DC_CLReqControlModeType(
                                stream,
                                &mut (*DC_ChargeLoopReqType).Scheduled_DC_CLReqControlMode,
                            );
                            if error == 0 as i32 {
                                (*DC_ChargeLoopReqType)
                                    .set_Scheduled_DC_CLReqControlMode_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_BPT_Dynamic_DC_CLResControlModeType(
    stream: &mut ExiBitstream,
    mut BPT_Dynamic_DC_CLResControlModeType: *mut iso20_dc_BPT_Dynamic_DC_CLResControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 233 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_BPT_Dynamic_DC_CLResControlModeType(BPT_Dynamic_DC_CLResControlModeType);
    while done == 0 {
        match grammar_id {
            233 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLResControlModeType).DepartureTime,
                            );
                            if error == 0 as i32 {
                                (*BPT_Dynamic_DC_CLResControlModeType)
                                    .set_DepartureTime_isUsed(1 as u32);
                                grammar_id = 234 as i32;
                            }
                        }
                        1 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*BPT_Dynamic_DC_CLResControlModeType).MinimumSOC =
                                            value as int8_t;
                                        (*BPT_Dynamic_DC_CLResControlModeType)
                                            .set_MinimumSOC_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 235 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        2 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_0,
                                    );
                                    if error == 0 as i32 {
                                        (*BPT_Dynamic_DC_CLResControlModeType).TargetSOC =
                                            value_0 as int8_t;
                                        (*BPT_Dynamic_DC_CLResControlModeType)
                                            .set_TargetSOC_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 236 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        3 => {
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLResControlModeType).AckMaxDelay,
                            );
                            if error == 0 as i32 {
                                (*BPT_Dynamic_DC_CLResControlModeType)
                                    .set_AckMaxDelay_isUsed(1 as u32);
                                grammar_id = 237 as i32;
                            }
                        }
                        4 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLResControlModeType).EVSEMaximumChargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 238 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            234 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_1: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_1,
                                    );
                                    if error == 0 as i32 {
                                        (*BPT_Dynamic_DC_CLResControlModeType).MinimumSOC =
                                            value_1 as int8_t;
                                        (*BPT_Dynamic_DC_CLResControlModeType)
                                            .set_MinimumSOC_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 235 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_2: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_2,
                                    );
                                    if error == 0 as i32 {
                                        (*BPT_Dynamic_DC_CLResControlModeType).TargetSOC =
                                            value_2 as int8_t;
                                        (*BPT_Dynamic_DC_CLResControlModeType)
                                            .set_TargetSOC_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 236 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        2 => {
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLResControlModeType).AckMaxDelay,
                            );
                            if error == 0 as i32 {
                                (*BPT_Dynamic_DC_CLResControlModeType)
                                    .set_AckMaxDelay_isUsed(1 as u32);
                                grammar_id = 237 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLResControlModeType).EVSEMaximumChargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 238 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            235 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_3: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_3,
                                    );
                                    if error == 0 as i32 {
                                        (*BPT_Dynamic_DC_CLResControlModeType).TargetSOC =
                                            value_3 as int8_t;
                                        (*BPT_Dynamic_DC_CLResControlModeType)
                                            .set_TargetSOC_isUsed(1 as u32);
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 236 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLResControlModeType).AckMaxDelay,
                            );
                            if error == 0 as i32 {
                                (*BPT_Dynamic_DC_CLResControlModeType)
                                    .set_AckMaxDelay_isUsed(1 as u32);
                                grammar_id = 237 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLResControlModeType).EVSEMaximumChargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 238 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            236 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLResControlModeType).AckMaxDelay,
                            );
                            if error == 0 as i32 {
                                (*BPT_Dynamic_DC_CLResControlModeType)
                                    .set_AckMaxDelay_isUsed(1 as u32);
                                grammar_id = 237 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLResControlModeType).EVSEMaximumChargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 238 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            237 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLResControlModeType).EVSEMaximumChargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 238 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            238 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLResControlModeType).EVSEMinimumChargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 239 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            239 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLResControlModeType)
                                    .EVSEMaximumChargeCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 240 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            240 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLResControlModeType).EVSEMaximumVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 241 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            241 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLResControlModeType)
                                    .EVSEMaximumDischargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 242 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            242 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLResControlModeType)
                                    .EVSEMinimumDischargePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 243 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            243 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLResControlModeType)
                                    .EVSEMaximumDischargeCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 244 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            244 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*BPT_Dynamic_DC_CLResControlModeType).EVSEMinimumVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_DC_ChargeLoopResType(
    stream: &mut ExiBitstream,
    mut DC_ChargeLoopResType: *mut iso20_dc_DC_ChargeLoopResType,
) -> i32 {
    let mut grammar_id: i32 = 245 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_DC_ChargeLoopResType(DC_ChargeLoopResType);
    while done == 0 {
        match grammar_id {
            245 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_MessageHeaderType(
                                stream,
                                &mut (*DC_ChargeLoopResType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 246 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            246 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        6 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_ChargeLoopResType).ResponseCode =
                                            value as iso20_dc_responseCodeType;
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 247 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            247 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_EVSEStatusType(
                                stream,
                                &mut (*DC_ChargeLoopResType).EVSEStatus,
                            );
                            if error == 0 as i32 {
                                (*DC_ChargeLoopResType).set_EVSEStatus_isUsed(1 as u32);
                                grammar_id = 248 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_MeterInfoType(
                                stream,
                                &mut (*DC_ChargeLoopResType).MeterInfo,
                            );
                            if error == 0 as i32 {
                                (*DC_ChargeLoopResType).set_MeterInfo_isUsed(1 as u32);
                                grammar_id = 249 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_ReceiptType(
                                stream,
                                &mut (*DC_ChargeLoopResType).Receipt,
                            );
                            if error == 0 as i32 {
                                (*DC_ChargeLoopResType).set_Receipt_isUsed(1 as u32);
                                grammar_id = 250 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DC_ChargeLoopResType).EVSEPresentCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 251 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            248 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_MeterInfoType(
                                stream,
                                &mut (*DC_ChargeLoopResType).MeterInfo,
                            );
                            if error == 0 as i32 {
                                (*DC_ChargeLoopResType).set_MeterInfo_isUsed(1 as u32);
                                grammar_id = 249 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_ReceiptType(
                                stream,
                                &mut (*DC_ChargeLoopResType).Receipt,
                            );
                            if error == 0 as i32 {
                                (*DC_ChargeLoopResType).set_Receipt_isUsed(1 as u32);
                                grammar_id = 250 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DC_ChargeLoopResType).EVSEPresentCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 251 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            249 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_ReceiptType(
                                stream,
                                &mut (*DC_ChargeLoopResType).Receipt,
                            );
                            if error == 0 as i32 {
                                (*DC_ChargeLoopResType).set_Receipt_isUsed(1 as u32);
                                grammar_id = 250 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DC_ChargeLoopResType).EVSEPresentCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 251 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            250 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DC_ChargeLoopResType).EVSEPresentCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 251 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            251 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_RationalNumberType(
                                stream,
                                &mut (*DC_ChargeLoopResType).EVSEPresentVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 252 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            252 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_0: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_0,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_ChargeLoopResType).EVSEPowerLimitAchieved =
                                            value_0 as i32;
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 253 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            253 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_1: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_1,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_ChargeLoopResType).EVSECurrentLimitAchieved =
                                            value_1 as i32;
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 254 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            254 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    let mut value_2: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_2,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_ChargeLoopResType).EVSEVoltageLimitAchieved =
                                            value_2 as i32;
                                    }
                                } else {
                                    error = -(151 as i32);
                                }
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 255 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            255 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_BPT_Dynamic_DC_CLResControlModeType(
                                stream,
                                &mut (*DC_ChargeLoopResType).BPT_Dynamic_DC_CLResControlMode,
                            );
                            if error == 0 as i32 {
                                (*DC_ChargeLoopResType)
                                    .set_BPT_Dynamic_DC_CLResControlMode_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_dc_BPT_Scheduled_DC_CLResControlModeType(
                                stream,
                                &mut (*DC_ChargeLoopResType).BPT_Scheduled_DC_CLResControlMode,
                            );
                            if error == 0 as i32 {
                                (*DC_ChargeLoopResType)
                                    .set_BPT_Scheduled_DC_CLResControlMode_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_dc_CLResControlModeType(
                                stream,
                                &mut (*DC_ChargeLoopResType).CLResControlMode,
                            );
                            if error == 0 as i32 {
                                (*DC_ChargeLoopResType).set_CLResControlMode_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_dc_Dynamic_DC_CLResControlModeType(
                                stream,
                                &mut (*DC_ChargeLoopResType).Dynamic_DC_CLResControlMode,
                            );
                            if error == 0 as i32 {
                                (*DC_ChargeLoopResType)
                                    .set_Dynamic_DC_CLResControlMode_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        4 => {
                            error = decode_iso20_dc_Scheduled_DC_CLResControlModeType(
                                stream,
                                &mut (*DC_ChargeLoopResType).Scheduled_DC_CLResControlMode,
                            );
                            if error == 0 as i32 {
                                (*DC_ChargeLoopResType)
                                    .set_Scheduled_DC_CLResControlMode_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_ManifestType(
    stream: &mut ExiBitstream,
    mut ManifestType: *mut iso20_dc_ManifestType,
) -> i32 {
    let mut grammar_id: i32 = 256 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_ManifestType(ManifestType);
    while done == 0 {
        match grammar_id {
            256 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*ManifestType).Id.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*ManifestType).Id.charactersLen as i32 >= 2 as i32 {
                                    (*ManifestType).Id.charactersLen =
                                        ((*ManifestType).Id.charactersLen as i32 - 2 as i32) as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*ManifestType).Id.charactersLen as usize,
                                        ((*ManifestType).Id.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*ManifestType).set_Id_isUsed(1 as u32);
                            grammar_id = 258 as i32;
                        }
                        1 => {
                            if ((*ManifestType).Reference.arrayLen as i32) < 4 as i32 {
                                let fresh12 = (*ManifestType).Reference.arrayLen;
                                (*ManifestType).Reference.arrayLen =
                                    ((*ManifestType).Reference.arrayLen).wrapping_add(1);
                                error = decode_iso20_dc_ReferenceType(
                                    stream,
                                    &mut *((*ManifestType).Reference.array)
                                        .as_mut_ptr()
                                        .offset(fresh12 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 257 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            257 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ManifestType).Reference.arrayLen as i32) < 4 as i32 {
                                let fresh13 = (*ManifestType).Reference.arrayLen;
                                (*ManifestType).Reference.arrayLen =
                                    ((*ManifestType).Reference.arrayLen).wrapping_add(1);
                                error = decode_iso20_dc_ReferenceType(
                                    stream,
                                    &mut *((*ManifestType).Reference.array)
                                        .as_mut_ptr()
                                        .offset(fresh13 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 257 as i32;
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            258 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ManifestType).Reference.arrayLen as i32) < 4 as i32 {
                                let fresh14 = (*ManifestType).Reference.arrayLen;
                                (*ManifestType).Reference.arrayLen =
                                    ((*ManifestType).Reference.arrayLen).wrapping_add(1);
                                error = decode_iso20_dc_ReferenceType(
                                    stream,
                                    &mut *((*ManifestType).Reference.array)
                                        .as_mut_ptr()
                                        .offset(fresh14 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 259 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            259 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ManifestType).Reference.arrayLen as i32) < 4 as i32 {
                                let fresh15 = (*ManifestType).Reference.arrayLen;
                                (*ManifestType).Reference.arrayLen =
                                    ((*ManifestType).Reference.arrayLen).wrapping_add(1);
                                error = decode_iso20_dc_ReferenceType(
                                    stream,
                                    &mut *((*ManifestType).Reference.array)
                                        .as_mut_ptr()
                                        .offset(fresh15 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 259 as i32;
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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
unsafe extern "C" fn decode_iso20_dc_SignaturePropertiesType(
    stream: &mut ExiBitstream,
    mut SignaturePropertiesType: *mut iso20_dc_SignaturePropertiesType,
) -> i32 {
    let mut grammar_id: i32 = 260 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_dc_SignaturePropertiesType(SignaturePropertiesType);
    while done == 0 {
        match grammar_id {
            260 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*SignaturePropertiesType).Id.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*SignaturePropertiesType).Id.charactersLen as i32 >= 2 as i32 {
                                    (*SignaturePropertiesType).Id.charactersLen =
                                        ((*SignaturePropertiesType).Id.charactersLen as i32
                                            - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*SignaturePropertiesType).Id.charactersLen as usize,
                                        ((*SignaturePropertiesType).Id.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*SignaturePropertiesType).set_Id_isUsed(1 as u32);
                            grammar_id = 262 as i32;
                        }
                        1 => {
                            error = decode_iso20_dc_SignaturePropertyType(
                                stream,
                                &mut (*SignaturePropertiesType).SignatureProperty,
                            );
                            if error == 0 as i32 {
                                grammar_id = 261 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            261 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = -(110 as i32);
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            262 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_dc_SignaturePropertyType(
                                stream,
                                &mut (*SignaturePropertiesType).SignatureProperty,
                            );
                            if error == 0 as i32 {
                                grammar_id = 263 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            263 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = -(110 as i32);
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            2 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 3 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
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

pub unsafe extern "C" fn decode_iso20_dc_exiDocument(
    stream: &mut ExiBitstream,
    mut exiDoc: *mut iso20_dc_exiDocument,
) -> i32 {
    let mut eventCode: u32 = 0;
    let mut error: i32 = exi_header_read_and_check(stream);
    if error == 0 as i32 {
        init_iso20_dc_exiDocument(exiDoc);
        error = exi_basetypes_decoder_nbit_uint(stream, 6 as i32 as usize, &mut eventCode);
        if error == 0 as i32 {
            match eventCode {
                0 => {
                    error = decode_iso20_dc_BPT_DC_CPDReqEnergyTransferModeType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.BPT_DC_CPDReqEnergyTransferMode,
                    );
                    (*exiDoc).set_BPT_DC_CPDReqEnergyTransferMode_isUsed(1 as u32);
                }
                1 => {
                    error = decode_iso20_dc_BPT_DC_CPDResEnergyTransferModeType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.BPT_DC_CPDResEnergyTransferMode,
                    );
                    (*exiDoc).set_BPT_DC_CPDResEnergyTransferMode_isUsed(1 as u32);
                }
                2 => {
                    error = decode_iso20_dc_BPT_Dynamic_DC_CLReqControlModeType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.BPT_Dynamic_DC_CLReqControlMode,
                    );
                    (*exiDoc).set_BPT_Dynamic_DC_CLReqControlMode_isUsed(1 as u32);
                }
                3 => {
                    error = decode_iso20_dc_BPT_Dynamic_DC_CLResControlModeType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.BPT_Dynamic_DC_CLResControlMode,
                    );
                    (*exiDoc).set_BPT_Dynamic_DC_CLResControlMode_isUsed(1 as u32);
                }
                4 => {
                    error = decode_iso20_dc_BPT_Scheduled_DC_CLReqControlModeType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.BPT_Scheduled_DC_CLReqControlMode,
                    );
                    (*exiDoc).set_BPT_Scheduled_DC_CLReqControlMode_isUsed(1 as u32);
                }
                5 => {
                    error = decode_iso20_dc_BPT_Scheduled_DC_CLResControlModeType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.BPT_Scheduled_DC_CLResControlMode,
                    );
                    (*exiDoc).set_BPT_Scheduled_DC_CLResControlMode_isUsed(1 as u32);
                }
                6 => {
                    error = decode_iso20_dc_CLReqControlModeType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.CLReqControlMode,
                    );
                    (*exiDoc).set_CLReqControlMode_isUsed(1 as u32);
                }
                7 => {
                    error = decode_iso20_dc_CLResControlModeType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.CLResControlMode,
                    );
                    (*exiDoc).set_CLResControlMode_isUsed(1 as u32);
                }
                8 => {
                    error = decode_iso20_dc_CanonicalizationMethodType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.CanonicalizationMethod,
                    );
                    (*exiDoc).set_CanonicalizationMethod_isUsed(1 as u32);
                }
                9 => {
                    error = decode_iso20_dc_DC_CPDReqEnergyTransferModeType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.DC_CPDReqEnergyTransferMode,
                    );
                    (*exiDoc).set_DC_CPDReqEnergyTransferMode_isUsed(1 as u32);
                }
                10 => {
                    error = decode_iso20_dc_DC_CPDResEnergyTransferModeType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.DC_CPDResEnergyTransferMode,
                    );
                    (*exiDoc).set_DC_CPDResEnergyTransferMode_isUsed(1 as u32);
                }
                11 => {
                    error = decode_iso20_dc_DC_CableCheckReqType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.DC_CableCheckReq,
                    );
                    (*exiDoc).set_DC_CableCheckReq_isUsed(1 as u32);
                }
                12 => {
                    error = decode_iso20_dc_DC_CableCheckResType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.DC_CableCheckRes,
                    );
                    (*exiDoc).set_DC_CableCheckRes_isUsed(1 as u32);
                }
                13 => {
                    error = decode_iso20_dc_DC_ChargeLoopReqType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.DC_ChargeLoopReq,
                    );
                    (*exiDoc).set_DC_ChargeLoopReq_isUsed(1 as u32);
                }
                14 => {
                    error = decode_iso20_dc_DC_ChargeLoopResType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.DC_ChargeLoopRes,
                    );
                    (*exiDoc).set_DC_ChargeLoopRes_isUsed(1 as u32);
                }
                15 => {
                    error = decode_iso20_dc_DC_ChargeParameterDiscoveryReqType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.DC_ChargeParameterDiscoveryReq,
                    );
                    (*exiDoc).set_DC_ChargeParameterDiscoveryReq_isUsed(1 as u32);
                }
                16 => {
                    error = decode_iso20_dc_DC_ChargeParameterDiscoveryResType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.DC_ChargeParameterDiscoveryRes,
                    );
                    (*exiDoc).set_DC_ChargeParameterDiscoveryRes_isUsed(1 as u32);
                }
                17 => {
                    error = decode_iso20_dc_DC_PreChargeReqType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.DC_PreChargeReq,
                    );
                    (*exiDoc).set_DC_PreChargeReq_isUsed(1 as u32);
                }
                18 => {
                    error = decode_iso20_dc_DC_PreChargeResType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.DC_PreChargeRes,
                    );
                    (*exiDoc).set_DC_PreChargeRes_isUsed(1 as u32);
                }
                19 => {
                    error = decode_iso20_dc_DC_WeldingDetectionReqType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.DC_WeldingDetectionReq,
                    );
                    (*exiDoc).set_DC_WeldingDetectionReq_isUsed(1 as u32);
                }
                20 => {
                    error = decode_iso20_dc_DC_WeldingDetectionResType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.DC_WeldingDetectionRes,
                    );
                    (*exiDoc).set_DC_WeldingDetectionRes_isUsed(1 as u32);
                }
                21 => {
                    error = decode_iso20_dc_DSAKeyValueType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.DSAKeyValue,
                    );
                    (*exiDoc).set_DSAKeyValue_isUsed(1 as u32);
                }
                22 => {
                    error = decode_iso20_dc_DigestMethodType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.DigestMethod,
                    );
                    (*exiDoc).set_DigestMethod_isUsed(1 as u32);
                }
                23 => {}
                24 => {
                    error = decode_iso20_dc_Dynamic_DC_CLReqControlModeType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.Dynamic_DC_CLReqControlMode,
                    );
                    (*exiDoc).set_Dynamic_DC_CLReqControlMode_isUsed(1 as u32);
                }
                25 => {
                    error = decode_iso20_dc_Dynamic_DC_CLResControlModeType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.Dynamic_DC_CLResControlMode,
                    );
                    (*exiDoc).set_Dynamic_DC_CLResControlMode_isUsed(1 as u32);
                }
                26 => {
                    error =
                        decode_iso20_dc_KeyInfoType(stream, &mut (*exiDoc).c2rust_unnamed.KeyInfo);
                    (*exiDoc).set_KeyInfo_isUsed(1 as u32);
                }
                27 => {}
                28 => {
                    error = decode_iso20_dc_KeyValueType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.KeyValue,
                    );
                    (*exiDoc).set_KeyValue_isUsed(1 as u32);
                }
                29 => {
                    error = decode_iso20_dc_ManifestType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.Manifest,
                    );
                    (*exiDoc).set_Manifest_isUsed(1 as u32);
                }
                30 => {}
                31 => {
                    error =
                        decode_iso20_dc_ObjectType(stream, &mut (*exiDoc).c2rust_unnamed.Object);
                    (*exiDoc).set_Object_isUsed(1 as u32);
                }
                32 => {
                    error =
                        decode_iso20_dc_PGPDataType(stream, &mut (*exiDoc).c2rust_unnamed.PGPData);
                    (*exiDoc).set_PGPData_isUsed(1 as u32);
                }
                33 => {
                    error = decode_iso20_dc_RSAKeyValueType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.RSAKeyValue,
                    );
                    (*exiDoc).set_RSAKeyValue_isUsed(1 as u32);
                }
                34 => {
                    error = decode_iso20_dc_ReferenceType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.Reference,
                    );
                    (*exiDoc).set_Reference_isUsed(1 as u32);
                }
                35 => {
                    error = decode_iso20_dc_RetrievalMethodType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.RetrievalMethod,
                    );
                    (*exiDoc).set_RetrievalMethod_isUsed(1 as u32);
                }
                36 => {
                    error = decode_iso20_dc_SPKIDataType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.SPKIData,
                    );
                    (*exiDoc).set_SPKIData_isUsed(1 as u32);
                }
                37 => {
                    error = decode_iso20_dc_Scheduled_DC_CLReqControlModeType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.Scheduled_DC_CLReqControlMode,
                    );
                    (*exiDoc).set_Scheduled_DC_CLReqControlMode_isUsed(1 as u32);
                }
                38 => {
                    error = decode_iso20_dc_Scheduled_DC_CLResControlModeType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.Scheduled_DC_CLResControlMode,
                    );
                    (*exiDoc).set_Scheduled_DC_CLResControlMode_isUsed(1 as u32);
                }
                39 => {
                    error = decode_iso20_dc_SignatureMethodType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.SignatureMethod,
                    );
                    (*exiDoc).set_SignatureMethod_isUsed(1 as u32);
                }
                40 => {
                    error = decode_iso20_dc_SignaturePropertiesType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.SignatureProperties,
                    );
                    (*exiDoc).set_SignatureProperties_isUsed(1 as u32);
                }
                41 => {
                    error = decode_iso20_dc_SignaturePropertyType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.SignatureProperty,
                    );
                    (*exiDoc).set_SignatureProperty_isUsed(1 as u32);
                }
                42 => {
                    error = decode_iso20_dc_SignatureType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.Signature,
                    );
                    (*exiDoc).set_Signature_isUsed(1 as u32);
                }
                43 => {
                    error = decode_iso20_dc_SignatureValueType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.SignatureValue,
                    );
                    (*exiDoc).set_SignatureValue_isUsed(1 as u32);
                }
                44 => {
                    error = decode_iso20_dc_SignedInfoType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.SignedInfo,
                    );
                    (*exiDoc).set_SignedInfo_isUsed(1 as u32);
                }
                45 => {
                    error = decode_iso20_dc_TransformType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.Transform,
                    );
                    (*exiDoc).set_Transform_isUsed(1 as u32);
                }
                46 => {
                    error = decode_iso20_dc_TransformsType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.Transforms,
                    );
                    (*exiDoc).set_Transforms_isUsed(1 as u32);
                }
                47 => {
                    error = decode_iso20_dc_X509DataType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.X509Data,
                    );
                    (*exiDoc).set_X509Data_isUsed(1 as u32);
                }
                _ => {
                    error = -(151 as i32);
                }
            }
        }
    }
    return error;
}

pub unsafe extern "C" fn decode_iso20_dc_exiFragment(
    stream: &mut ExiBitstream,
    mut exiFrag: *mut iso20_dc_exiFragment,
) -> i32 {
    let mut eventCode: u32 = 0;
    let mut error: i32 = exi_header_read_and_check(stream);
    if error == 0 as i32 {
        init_iso20_dc_exiFragment(exiFrag);
        error = exi_basetypes_decoder_nbit_uint(stream, 8 as i32 as usize, &mut eventCode);
        if error == 0 as i32 {
            error = -(299 as i32);
            match eventCode {
                0 => {}
                1 => {}
                2 => {}
                3 => {}
                4 => {}
                5 => {}
                6 => {}
                7 => {}
                8 => {}
                9 => {}
                10 => {}
                11 => {}
                12 => {}
                13 => {}
                14 => {}
                15 => {}
                16 => {}
                17 => {}
                18 => {}
                19 => {}
                20 => {}
                21 => {}
                22 => {}
                23 => {}
                24 => {}
                25 => {}
                26 => {
                    error = decode_iso20_dc_DC_ChargeParameterDiscoveryResType(
                        stream,
                        &mut (*exiFrag).c2rust_unnamed.DC_ChargeParameterDiscoveryRes,
                    );
                    (*exiFrag).set_DC_ChargeParameterDiscoveryRes_isUsed(1 as u32);
                }
                27 => {}
                28 => {}
                29 => {}
                30 => {}
                31 => {}
                32 => {}
                33 => {}
                34 => {}
                35 => {}
                36 => {}
                37 => {}
                38 => {}
                39 => {}
                40 => {}
                41 => {}
                42 => {}
                43 => {}
                44 => {}
                45 => {}
                46 => {}
                47 => {}
                48 => {}
                49 => {}
                50 => {}
                51 => {}
                52 => {}
                53 => {}
                54 => {}
                55 => {}
                56 => {}
                57 => {}
                58 => {}
                59 => {}
                60 => {}
                61 => {}
                62 => {}
                63 => {}
                64 => {}
                65 => {}
                66 => {}
                67 => {}
                68 => {}
                69 => {}
                70 => {}
                71 => {}
                72 => {}
                73 => {}
                74 => {}
                75 => {}
                76 => {}
                77 => {}
                78 => {}
                79 => {}
                80 => {}
                81 => {}
                82 => {}
                83 => {}
                84 => {}
                85 => {}
                86 => {}
                87 => {}
                88 => {}
                89 => {}
                90 => {}
                91 => {}
                92 => {}
                93 => {}
                94 => {}
                95 => {}
                96 => {}
                97 => {}
                98 => {}
                99 => {}
                100 => {}
                101 => {}
                102 => {}
                103 => {}
                104 => {}
                105 => {}
                106 => {}
                107 => {}
                108 => {}
                109 => {}
                110 => {}
                111 => {}
                112 => {}
                113 => {}
                114 => {}
                115 => {}
                116 => {}
                117 => {}
                118 => {}
                119 => {}
                120 => {}
                121 => {}
                122 => {}
                123 => {}
                124 => {}
                125 => {}
                126 => {}
                127 => {}
                128 => {}
                129 => {
                    error = decode_iso20_dc_SignedInfoType(
                        stream,
                        &mut (*exiFrag).c2rust_unnamed.SignedInfo,
                    );
                    (*exiFrag).set_SignedInfo_isUsed(1 as u32);
                }
                130 => {}
                131 => {}
                132 => {}
                133 => {}
                134 => {}
                135 => {}
                136 => {}
                137 => {}
                138 => {}
                139 => {}
                140 => {}
                141 => {}
                142 => {}
                143 => {}
                144 => {}
                145 => {}
                146 => {}
                147 => {}
                148 => {}
                _ => {
                    error = -(151 as i32);
                }
            }
            if error == 0 as i32 {
                error = exi_basetypes_decoder_nbit_uint(stream, 8 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    if eventCode != 150 as i32 as u32 {
                        error = -(230 as i32);
                    }
                }
            }
        }
    }
    return error;
}

pub unsafe extern "C" fn decode_iso20_dc_xmldsigFragment(
    stream: &mut ExiBitstream,
    mut xmldsigFrag: *mut iso20_dc_xmldsigFragment,
) -> i32 {
    let mut eventCode: u32 = 0;
    let mut error: i32 = exi_header_read_and_check(stream);
    if error == 0 as i32 {
        init_iso20_dc_xmldsigFragment(xmldsigFrag);
        error = exi_basetypes_decoder_nbit_uint(stream, 6 as i32 as usize, &mut eventCode);
        if error == 0 as i32 {
            error = -(299 as i32);
            match eventCode {
                0 => {
                    error = decode_iso20_dc_CanonicalizationMethodType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.CanonicalizationMethod,
                    );
                    (*xmldsigFrag).set_CanonicalizationMethod_isUsed(1 as u32);
                }
                1 => {
                    error = decode_iso20_dc_DSAKeyValueType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.DSAKeyValue,
                    );
                    (*xmldsigFrag).set_DSAKeyValue_isUsed(1 as u32);
                }
                2 => {
                    error = decode_iso20_dc_DigestMethodType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.DigestMethod,
                    );
                    (*xmldsigFrag).set_DigestMethod_isUsed(1 as u32);
                }
                3 => {}
                4 => {}
                5 => {}
                6 => {}
                7 => {}
                8 => {
                    error = decode_iso20_dc_KeyInfoType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.KeyInfo,
                    );
                    (*xmldsigFrag).set_KeyInfo_isUsed(1 as u32);
                }
                9 => {}
                10 => {
                    error = decode_iso20_dc_KeyValueType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.KeyValue,
                    );
                    (*xmldsigFrag).set_KeyValue_isUsed(1 as u32);
                }
                11 => {
                    error = decode_iso20_dc_ManifestType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.Manifest,
                    );
                    (*xmldsigFrag).set_Manifest_isUsed(1 as u32);
                }
                12 => {}
                13 => {}
                14 => {
                    error = decode_iso20_dc_ObjectType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.Object,
                    );
                    (*xmldsigFrag).set_Object_isUsed(1 as u32);
                }
                15 => {}
                16 => {
                    error = decode_iso20_dc_PGPDataType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.PGPData,
                    );
                    (*xmldsigFrag).set_PGPData_isUsed(1 as u32);
                }
                17 => {}
                18 => {}
                19 => {}
                20 => {}
                21 => {
                    error = decode_iso20_dc_RSAKeyValueType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.RSAKeyValue,
                    );
                    (*xmldsigFrag).set_RSAKeyValue_isUsed(1 as u32);
                }
                22 => {
                    error = decode_iso20_dc_ReferenceType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.Reference,
                    );
                    (*xmldsigFrag).set_Reference_isUsed(1 as u32);
                }
                23 => {
                    error = decode_iso20_dc_RetrievalMethodType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.RetrievalMethod,
                    );
                    (*xmldsigFrag).set_RetrievalMethod_isUsed(1 as u32);
                }
                24 => {
                    error = decode_iso20_dc_SPKIDataType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.SPKIData,
                    );
                    (*xmldsigFrag).set_SPKIData_isUsed(1 as u32);
                }
                25 => {}
                26 => {}
                27 => {
                    error = decode_iso20_dc_SignatureType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.Signature,
                    );
                    (*xmldsigFrag).set_Signature_isUsed(1 as u32);
                }
                28 => {
                    error = decode_iso20_dc_SignatureMethodType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.SignatureMethod,
                    );
                    (*xmldsigFrag).set_SignatureMethod_isUsed(1 as u32);
                }
                29 => {
                    error = decode_iso20_dc_SignaturePropertiesType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.SignatureProperties,
                    );
                    (*xmldsigFrag).set_SignatureProperties_isUsed(1 as u32);
                }
                30 => {
                    error = decode_iso20_dc_SignaturePropertyType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.SignatureProperty,
                    );
                    (*xmldsigFrag).set_SignatureProperty_isUsed(1 as u32);
                }
                31 => {
                    error = decode_iso20_dc_SignatureValueType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.SignatureValue,
                    );
                    (*xmldsigFrag).set_SignatureValue_isUsed(1 as u32);
                }
                32 => {
                    error = decode_iso20_dc_SignedInfoType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.SignedInfo,
                    );
                    (*xmldsigFrag).set_SignedInfo_isUsed(1 as u32);
                }
                33 => {
                    error = decode_iso20_dc_TransformType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.Transform,
                    );
                    (*xmldsigFrag).set_Transform_isUsed(1 as u32);
                }
                34 => {
                    error = decode_iso20_dc_TransformsType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.Transforms,
                    );
                    (*xmldsigFrag).set_Transforms_isUsed(1 as u32);
                }
                35 => {}
                36 => {}
                37 => {
                    error = decode_iso20_dc_X509DataType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.X509Data,
                    );
                    (*xmldsigFrag).set_X509Data_isUsed(1 as u32);
                }
                38 => {}
                39 => {
                    error = decode_iso20_dc_X509IssuerSerialType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.X509IssuerSerial,
                    );
                    (*xmldsigFrag).set_X509IssuerSerial_isUsed(1 as u32);
                }
                40 => {}
                41 => {}
                42 => {}
                43 => {}
                44 => {}
                _ => {
                    error = -(151 as i32);
                }
            }
            if error == 0 as i32 {
                error = exi_basetypes_decoder_nbit_uint(stream, 6 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    if eventCode != 46 as i32 as u32 {
                        error = -(230 as i32);
                    }
                }
            }
        }
    }
    return error;
}
