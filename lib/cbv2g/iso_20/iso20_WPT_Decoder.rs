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
    fn init_iso20_wpt_exiDocument(exiDoc: *mut iso20_wpt_exiDocument);
    fn init_iso20_wpt_WPT_FinePositioningSetupReqType(
        WPT_FinePositioningSetupReq: *mut iso20_wpt_WPT_FinePositioningSetupReqType,
    );
    fn init_iso20_wpt_WPT_FinePositioningSetupResType(
        WPT_FinePositioningSetupRes: *mut iso20_wpt_WPT_FinePositioningSetupResType,
    );
    fn init_iso20_wpt_WPT_FinePositioningReqType(
        WPT_FinePositioningReq: *mut iso20_wpt_WPT_FinePositioningReqType,
    );
    fn init_iso20_wpt_WPT_FinePositioningResType(
        WPT_FinePositioningRes: *mut iso20_wpt_WPT_FinePositioningResType,
    );
    fn init_iso20_wpt_WPT_PairingReqType(WPT_PairingReq: *mut iso20_wpt_WPT_PairingReqType);
    fn init_iso20_wpt_WPT_PairingResType(WPT_PairingRes: *mut iso20_wpt_WPT_PairingResType);
    fn init_iso20_wpt_WPT_ChargeParameterDiscoveryReqType(
        WPT_ChargeParameterDiscoveryReq: *mut iso20_wpt_WPT_ChargeParameterDiscoveryReqType,
    );
    fn init_iso20_wpt_WPT_ChargeParameterDiscoveryResType(
        WPT_ChargeParameterDiscoveryRes: *mut iso20_wpt_WPT_ChargeParameterDiscoveryResType,
    );
    fn init_iso20_wpt_WPT_AlignmentCheckReqType(
        WPT_AlignmentCheckReq: *mut iso20_wpt_WPT_AlignmentCheckReqType,
    );
    fn init_iso20_wpt_WPT_AlignmentCheckResType(
        WPT_AlignmentCheckRes: *mut iso20_wpt_WPT_AlignmentCheckResType,
    );
    fn init_iso20_wpt_WPT_ChargeLoopReqType(
        WPT_ChargeLoopReq: *mut iso20_wpt_WPT_ChargeLoopReqType,
    );
    fn init_iso20_wpt_WPT_ChargeLoopResType(
        WPT_ChargeLoopRes: *mut iso20_wpt_WPT_ChargeLoopResType,
    );
    fn init_iso20_wpt_SignatureType(Signature: *mut iso20_wpt_SignatureType);
    fn init_iso20_wpt_SignatureValueType(SignatureValue: *mut iso20_wpt_SignatureValueType);
    fn init_iso20_wpt_SignedInfoType(SignedInfo: *mut iso20_wpt_SignedInfoType);
    fn init_iso20_wpt_CanonicalizationMethodType(
        CanonicalizationMethod: *mut iso20_wpt_CanonicalizationMethodType,
    );
    fn init_iso20_wpt_SignatureMethodType(SignatureMethod: *mut iso20_wpt_SignatureMethodType);
    fn init_iso20_wpt_ReferenceType(Reference: *mut iso20_wpt_ReferenceType);
    fn init_iso20_wpt_TransformsType(Transforms: *mut iso20_wpt_TransformsType);
    fn init_iso20_wpt_TransformType(Transform: *mut iso20_wpt_TransformType);
    fn init_iso20_wpt_DigestMethodType(DigestMethod: *mut iso20_wpt_DigestMethodType);
    fn init_iso20_wpt_KeyInfoType(KeyInfo: *mut iso20_wpt_KeyInfoType);
    fn init_iso20_wpt_KeyValueType(KeyValue: *mut iso20_wpt_KeyValueType);
    fn init_iso20_wpt_RetrievalMethodType(RetrievalMethod: *mut iso20_wpt_RetrievalMethodType);
    fn init_iso20_wpt_X509DataType(X509Data: *mut iso20_wpt_X509DataType);
    fn init_iso20_wpt_PGPDataType(PGPData: *mut iso20_wpt_PGPDataType);
    fn init_iso20_wpt_SPKIDataType(SPKIData: *mut iso20_wpt_SPKIDataType);
    fn init_iso20_wpt_ObjectType(Object: *mut iso20_wpt_ObjectType);
    fn init_iso20_wpt_ManifestType(Manifest: *mut iso20_wpt_ManifestType);
    fn init_iso20_wpt_SignaturePropertiesType(
        SignatureProperties: *mut iso20_wpt_SignaturePropertiesType,
    );
    fn init_iso20_wpt_SignaturePropertyType(
        SignatureProperty: *mut iso20_wpt_SignaturePropertyType,
    );
    fn init_iso20_wpt_DSAKeyValueType(DSAKeyValue: *mut iso20_wpt_DSAKeyValueType);
    fn init_iso20_wpt_RSAKeyValueType(RSAKeyValue: *mut iso20_wpt_RSAKeyValueType);
    fn init_iso20_wpt_WPT_LF_RxRSSIType(WPT_LF_RxRSSIType: *mut iso20_wpt_WPT_LF_RxRSSIType);
    fn init_iso20_wpt_X509IssuerSerialType(
        X509IssuerSerialType: *mut iso20_wpt_X509IssuerSerialType,
    );
    fn init_iso20_wpt_WPT_LF_RxRSSIListType(
        WPT_LF_RxRSSIListType: *mut iso20_wpt_WPT_LF_RxRSSIListType,
    );
    fn init_iso20_wpt_WPT_TxRxPulseOrderType(
        WPT_TxRxPulseOrderType: *mut iso20_wpt_WPT_TxRxPulseOrderType,
    );
    fn init_iso20_wpt_WPT_LF_TxDataType(WPT_LF_TxDataType: *mut iso20_wpt_WPT_LF_TxDataType);
    fn init_iso20_wpt_WPT_LF_RxDataType(WPT_LF_RxDataType: *mut iso20_wpt_WPT_LF_RxDataType);
    fn init_iso20_wpt_WPT_CoordinateXYZType(
        WPT_CoordinateXYZType: *mut iso20_wpt_WPT_CoordinateXYZType,
    );
    fn init_iso20_wpt_RationalNumberType(RationalNumberType: *mut iso20_wpt_RationalNumberType);
    fn init_iso20_wpt_WPT_LF_TxDataListType(
        WPT_LF_TxDataListType: *mut iso20_wpt_WPT_LF_TxDataListType,
    );
    fn init_iso20_wpt_WPT_TxRxSpecDataType(
        WPT_TxRxSpecDataType: *mut iso20_wpt_WPT_TxRxSpecDataType,
    );
    fn init_iso20_wpt_WPT_LF_RxDataListType(
        WPT_LF_RxDataListType: *mut iso20_wpt_WPT_LF_RxDataListType,
    );
    fn init_iso20_wpt_WPT_TxRxPackageSpecDataType(
        WPT_TxRxPackageSpecDataType: *mut iso20_wpt_WPT_TxRxPackageSpecDataType,
    );
    fn init_iso20_wpt_WPT_LF_TransmitterDataType(
        WPT_LF_TransmitterDataType: *mut iso20_wpt_WPT_LF_TransmitterDataType,
    );
    fn init_iso20_wpt_AlternativeSECCType(AlternativeSECCType: *mut iso20_wpt_AlternativeSECCType);
    fn init_iso20_wpt_WPT_LF_ReceiverDataType(
        WPT_LF_ReceiverDataType: *mut iso20_wpt_WPT_LF_ReceiverDataType,
    );
    fn init_iso20_wpt_WPT_LF_DataPackageType(
        WPT_LF_DataPackageType: *mut iso20_wpt_WPT_LF_DataPackageType,
    );
    fn init_iso20_wpt_DetailedCostType(DetailedCostType: *mut iso20_wpt_DetailedCostType);
    fn init_iso20_wpt_DetailedTaxType(DetailedTaxType: *mut iso20_wpt_DetailedTaxType);
    fn init_iso20_wpt_MessageHeaderType(MessageHeaderType: *mut iso20_wpt_MessageHeaderType);
    fn init_iso20_wpt_DisplayParametersType(
        DisplayParametersType: *mut iso20_wpt_DisplayParametersType,
    );
    fn init_iso20_wpt_WPT_FinePositioningMethodListType(
        WPT_FinePositioningMethodListType: *mut iso20_wpt_WPT_FinePositioningMethodListType,
    );
    fn init_iso20_wpt_EVSEStatusType(EVSEStatusType: *mut iso20_wpt_EVSEStatusType);
    fn init_iso20_wpt_WPT_PairingMethodListType(
        WPT_PairingMethodListType: *mut iso20_wpt_WPT_PairingMethodListType,
    );
    fn init_iso20_wpt_MeterInfoType(MeterInfoType: *mut iso20_wpt_MeterInfoType);
    fn init_iso20_wpt_WPT_AlignmentCheckMethodListType(
        WPT_AlignmentCheckMethodListType: *mut iso20_wpt_WPT_AlignmentCheckMethodListType,
    );
    fn init_iso20_wpt_WPT_LF_DataPackageListType(
        WPT_LF_DataPackageListType: *mut iso20_wpt_WPT_LF_DataPackageListType,
    );
    fn init_iso20_wpt_AlternativeSECCListType(
        AlternativeSECCListType: *mut iso20_wpt_AlternativeSECCListType,
    );
    fn init_iso20_wpt_ReceiptType(ReceiptType: *mut iso20_wpt_ReceiptType);
    fn init_iso20_wpt_WPT_LF_SystemSetupDataType(
        WPT_LF_SystemSetupDataType: *mut iso20_wpt_WPT_LF_SystemSetupDataType,
    );
    fn init_iso20_wpt_WPT_EVPCPowerControlParameterType(
        WPT_EVPCPowerControlParameterType: *mut iso20_wpt_WPT_EVPCPowerControlParameterType,
    );
    fn init_iso20_wpt_WPT_SPCPowerControlParameterType(
        WPT_SPCPowerControlParameterType: *mut iso20_wpt_WPT_SPCPowerControlParameterType,
    );
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
unsafe extern "C" fn decode_iso20_wpt_TransformType(
    stream: &mut ExiBitstream,
    mut TransformType: *mut iso20_wpt_TransformType,
) -> i32 {
    let mut grammar_id: i32 = 0 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_TransformType(TransformType);
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
unsafe extern "C" fn decode_iso20_wpt_TransformsType(
    stream: &mut ExiBitstream,
    mut TransformsType: *mut iso20_wpt_TransformsType,
) -> i32 {
    let mut grammar_id: i32 = 4 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_TransformsType(TransformsType);
    while done == 0 {
        match grammar_id {
            4 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_TransformType(
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
unsafe extern "C" fn decode_iso20_wpt_DSAKeyValueType(
    stream: &mut ExiBitstream,
    mut DSAKeyValueType: *mut iso20_wpt_DSAKeyValueType,
) -> i32 {
    let mut grammar_id: i32 = 6 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_DSAKeyValueType(DSAKeyValueType);
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
unsafe extern "C" fn decode_iso20_wpt_X509IssuerSerialType(
    stream: &mut ExiBitstream,
    mut X509IssuerSerialType: *mut iso20_wpt_X509IssuerSerialType,
) -> i32 {
    let mut grammar_id: i32 = 13 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_X509IssuerSerialType(X509IssuerSerialType);
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
unsafe extern "C" fn decode_iso20_wpt_DigestMethodType(
    stream: &mut ExiBitstream,
    mut DigestMethodType: *mut iso20_wpt_DigestMethodType,
) -> i32 {
    let mut grammar_id: i32 = 15 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_DigestMethodType(DigestMethodType);
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
unsafe extern "C" fn decode_iso20_wpt_RSAKeyValueType(
    stream: &mut ExiBitstream,
    mut RSAKeyValueType: *mut iso20_wpt_RSAKeyValueType,
) -> i32 {
    let mut grammar_id: i32 = 17 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_RSAKeyValueType(RSAKeyValueType);
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
unsafe extern "C" fn decode_iso20_wpt_CanonicalizationMethodType(
    stream: &mut ExiBitstream,
    mut CanonicalizationMethodType: *mut iso20_wpt_CanonicalizationMethodType,
) -> i32 {
    let mut grammar_id: i32 = 19 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_CanonicalizationMethodType(CanonicalizationMethodType);
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
unsafe extern "C" fn decode_iso20_wpt_WPT_TxRxPulseOrderType(
    stream: &mut ExiBitstream,
    mut WPT_TxRxPulseOrderType: *mut iso20_wpt_WPT_TxRxPulseOrderType,
) -> i32 {
    let mut grammar_id: i32 = 21 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_TxRxPulseOrderType(WPT_TxRxPulseOrderType);
    while done == 0 {
        match grammar_id {
            21 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*WPT_TxRxPulseOrderType).IndexNumber,
                            );
                            if error == 0 as i32 {
                                grammar_id = 22 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            22 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*WPT_TxRxPulseOrderType).TxRxIdentifier,
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
unsafe extern "C" fn decode_iso20_wpt_SignatureMethodType(
    stream: &mut ExiBitstream,
    mut SignatureMethodType: *mut iso20_wpt_SignatureMethodType,
) -> i32 {
    let mut grammar_id: i32 = 23 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_SignatureMethodType(SignatureMethodType);
    while done == 0 {
        match grammar_id {
            23 => {
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
                            grammar_id = 24 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            24 => {
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
                                    grammar_id = 25 as i32;
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
            25 => {
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
unsafe extern "C" fn decode_iso20_wpt_KeyValueType(
    stream: &mut ExiBitstream,
    mut KeyValueType: *mut iso20_wpt_KeyValueType,
) -> i32 {
    let mut grammar_id: i32 = 26 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_KeyValueType(KeyValueType);
    while done == 0 {
        match grammar_id {
            26 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_DSAKeyValueType(
                                stream,
                                &mut (*KeyValueType).DSAKeyValue,
                            );
                            if error == 0 as i32 {
                                (*KeyValueType).set_DSAKeyValue_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_wpt_RSAKeyValueType(
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
unsafe extern "C" fn decode_iso20_wpt_WPT_CoordinateXYZType(
    stream: &mut ExiBitstream,
    mut WPT_CoordinateXYZType: *mut iso20_wpt_WPT_CoordinateXYZType,
) -> i32 {
    let mut grammar_id: i32 = 27 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_CoordinateXYZType(WPT_CoordinateXYZType);
    while done == 0 {
        match grammar_id {
            27 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*WPT_CoordinateXYZType).Coord_X,
                            );
                            if error == 0 as i32 {
                                grammar_id = 28 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            28 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*WPT_CoordinateXYZType).Coord_Y,
                            );
                            if error == 0 as i32 {
                                grammar_id = 29 as i32;
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
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*WPT_CoordinateXYZType).Coord_Z,
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
unsafe extern "C" fn decode_iso20_wpt_ReferenceType(
    stream: &mut ExiBitstream,
    mut ReferenceType: *mut iso20_wpt_ReferenceType,
) -> i32 {
    let mut grammar_id: i32 = 30 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_ReferenceType(ReferenceType);
    while done == 0 {
        match grammar_id {
            30 => {
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
                            grammar_id = 31 as i32;
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
                            grammar_id = 32 as i32;
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
                            grammar_id = 33 as i32;
                        }
                        3 => {
                            error = decode_iso20_wpt_TransformsType(
                                stream,
                                &mut (*ReferenceType).Transforms,
                            );
                            if error == 0 as i32 {
                                (*ReferenceType).set_Transforms_isUsed(1 as u32);
                                grammar_id = 34 as i32;
                            }
                        }
                        4 => {
                            error = decode_iso20_wpt_DigestMethodType(
                                stream,
                                &mut (*ReferenceType).DigestMethod,
                            );
                            if error == 0 as i32 {
                                grammar_id = 35 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            31 => {
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
                            grammar_id = 32 as i32;
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
                            grammar_id = 33 as i32;
                        }
                        2 => {
                            error = decode_iso20_wpt_TransformsType(
                                stream,
                                &mut (*ReferenceType).Transforms,
                            );
                            if error == 0 as i32 {
                                (*ReferenceType).set_Transforms_isUsed(1 as u32);
                                grammar_id = 34 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_wpt_DigestMethodType(
                                stream,
                                &mut (*ReferenceType).DigestMethod,
                            );
                            if error == 0 as i32 {
                                grammar_id = 35 as i32;
                            }
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
                            grammar_id = 33 as i32;
                        }
                        1 => {
                            error = decode_iso20_wpt_TransformsType(
                                stream,
                                &mut (*ReferenceType).Transforms,
                            );
                            if error == 0 as i32 {
                                (*ReferenceType).set_Transforms_isUsed(1 as u32);
                                grammar_id = 34 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_wpt_DigestMethodType(
                                stream,
                                &mut (*ReferenceType).DigestMethod,
                            );
                            if error == 0 as i32 {
                                grammar_id = 35 as i32;
                            }
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
                            error = decode_iso20_wpt_TransformsType(
                                stream,
                                &mut (*ReferenceType).Transforms,
                            );
                            if error == 0 as i32 {
                                (*ReferenceType).set_Transforms_isUsed(1 as u32);
                                grammar_id = 34 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_wpt_DigestMethodType(
                                stream,
                                &mut (*ReferenceType).DigestMethod,
                            );
                            if error == 0 as i32 {
                                grammar_id = 35 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            34 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_DigestMethodType(
                                stream,
                                &mut (*ReferenceType).DigestMethod,
                            );
                            if error == 0 as i32 {
                                grammar_id = 35 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            35 => {
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
unsafe extern "C" fn decode_iso20_wpt_RetrievalMethodType(
    stream: &mut ExiBitstream,
    mut RetrievalMethodType: *mut iso20_wpt_RetrievalMethodType,
) -> i32 {
    let mut grammar_id: i32 = 36 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_RetrievalMethodType(RetrievalMethodType);
    while done == 0 {
        match grammar_id {
            36 => {
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
                            grammar_id = 37 as i32;
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
                            grammar_id = 38 as i32;
                        }
                        2 => {
                            error = decode_iso20_wpt_TransformsType(
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
            37 => {
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
                            grammar_id = 38 as i32;
                        }
                        1 => {
                            error = decode_iso20_wpt_TransformsType(
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
            38 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_TransformsType(
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
unsafe extern "C" fn decode_iso20_wpt_X509DataType(
    stream: &mut ExiBitstream,
    mut X509DataType: *mut iso20_wpt_X509DataType,
) -> i32 {
    let mut grammar_id: i32 = 39 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_X509DataType(X509DataType);
    while done == 0 {
        match grammar_id {
            39 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_X509IssuerSerialType(
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
unsafe extern "C" fn decode_iso20_wpt_PGPDataType(
    stream: &mut ExiBitstream,
    mut PGPDataType: *mut iso20_wpt_PGPDataType,
) -> i32 {
    let mut grammar_id: i32 = 40 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_PGPDataType(PGPDataType);
    while done == 0 {
        match grammar_id {
            40 => {
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
                                grammar_id = 41 as i32;
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
                                grammar_id = 42 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            41 => {
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
                                grammar_id = 42 as i32;
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
                                grammar_id = 43 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            42 => {
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
                                grammar_id = 43 as i32;
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
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*PGPDataType).c2rust_unnamed.choice_2.PGPKeyPacket.bytesLen,
                                &mut *((*PGPDataType).c2rust_unnamed.choice_2.PGPKeyPacket.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                350 as i32 as usize,
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
                                grammar_id = 43 as i32;
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
unsafe extern "C" fn decode_iso20_wpt_SPKIDataType(
    stream: &mut ExiBitstream,
    mut SPKIDataType: *mut iso20_wpt_SPKIDataType,
) -> i32 {
    let mut grammar_id: i32 = 45 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_SPKIDataType(SPKIDataType);
    while done == 0 {
        match grammar_id {
            45 => {
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
                                grammar_id = 46 as i32;
                            }
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
unsafe extern "C" fn decode_iso20_wpt_SignedInfoType(
    stream: &mut ExiBitstream,
    mut SignedInfoType: *mut iso20_wpt_SignedInfoType,
) -> i32 {
    let mut grammar_id: i32 = 47 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_SignedInfoType(SignedInfoType);
    while done == 0 {
        match grammar_id {
            47 => {
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
                            grammar_id = 48 as i32;
                        }
                        1 => {
                            error = decode_iso20_wpt_CanonicalizationMethodType(
                                stream,
                                &mut (*SignedInfoType).CanonicalizationMethod,
                            );
                            if error == 0 as i32 {
                                grammar_id = 49 as i32;
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
                            error = decode_iso20_wpt_CanonicalizationMethodType(
                                stream,
                                &mut (*SignedInfoType).CanonicalizationMethod,
                            );
                            if error == 0 as i32 {
                                grammar_id = 49 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            49 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_SignatureMethodType(
                                stream,
                                &mut (*SignedInfoType).SignatureMethod,
                            );
                            if error == 0 as i32 {
                                grammar_id = 50 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            50 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SignedInfoType).Reference.arrayLen as i32) < 4 as i32 {
                                let fresh0 = (*SignedInfoType).Reference.arrayLen;
                                (*SignedInfoType).Reference.arrayLen =
                                    ((*SignedInfoType).Reference.arrayLen).wrapping_add(1);
                                error = decode_iso20_wpt_ReferenceType(
                                    stream,
                                    &mut *((*SignedInfoType).Reference.array)
                                        .as_mut_ptr()
                                        .offset(fresh0 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 51 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            51 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SignedInfoType).Reference.arrayLen as i32) < 4 as i32 {
                                let fresh1 = (*SignedInfoType).Reference.arrayLen;
                                (*SignedInfoType).Reference.arrayLen =
                                    ((*SignedInfoType).Reference.arrayLen).wrapping_add(1);
                                error = decode_iso20_wpt_ReferenceType(
                                    stream,
                                    &mut *((*SignedInfoType).Reference.array)
                                        .as_mut_ptr()
                                        .offset(fresh1 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 51 as i32;
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
unsafe extern "C" fn decode_iso20_wpt_SignatureValueType(
    stream: &mut ExiBitstream,
    mut SignatureValueType: *mut iso20_wpt_SignatureValueType,
) -> i32 {
    let mut grammar_id: i32 = 52 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_SignatureValueType(SignatureValueType);
    while done == 0 {
        match grammar_id {
            52 => {
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
                            grammar_id = 53 as i32;
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
            53 => {
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
unsafe extern "C" fn decode_iso20_wpt_RationalNumberType(
    stream: &mut ExiBitstream,
    mut RationalNumberType: *mut iso20_wpt_RationalNumberType,
) -> i32 {
    let mut grammar_id: i32 = 54 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_RationalNumberType(RationalNumberType);
    while done == 0 {
        match grammar_id {
            54 => {
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
                                        grammar_id = 55 as i32;
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
            55 => {
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
unsafe extern "C" fn decode_iso20_wpt_WPT_LF_RxRSSIType(
    stream: &mut ExiBitstream,
    mut WPT_LF_RxRSSIType: *mut iso20_wpt_WPT_LF_RxRSSIType,
) -> i32 {
    let mut grammar_id: i32 = 56 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_LF_RxRSSIType(WPT_LF_RxRSSIType);
    while done == 0 {
        match grammar_id {
            56 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*WPT_LF_RxRSSIType).TxIdentifier,
                            );
                            if error == 0 as i32 {
                                grammar_id = 57 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            57 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_LF_RxRSSIType).RSSI,
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
unsafe extern "C" fn decode_iso20_wpt_WPT_LF_RxRSSIListType(
    stream: &mut ExiBitstream,
    mut WPT_LF_RxRSSIListType: *mut iso20_wpt_WPT_LF_RxRSSIListType,
) -> i32 {
    let mut grammar_id: i32 = 58 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_LF_RxRSSIListType(WPT_LF_RxRSSIListType);
    while done == 0 {
        match grammar_id {
            58 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_WPT_LF_RxRSSIType(
                                stream,
                                &mut (*WPT_LF_RxRSSIListType).RSSIDataList,
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
unsafe extern "C" fn decode_iso20_wpt_WPT_LF_TxDataType(
    stream: &mut ExiBitstream,
    mut WPT_LF_TxDataType: *mut iso20_wpt_WPT_LF_TxDataType,
) -> i32 {
    let mut grammar_id: i32 = 59 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_LF_TxDataType(WPT_LF_TxDataType);
    while done == 0 {
        match grammar_id {
            59 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*WPT_LF_TxDataType).TxIdentifier,
                            );
                            if error == 0 as i32 {
                                grammar_id = 60 as i32;
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
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_LF_TxDataType).EIRP,
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
unsafe extern "C" fn decode_iso20_wpt_WPT_LF_RxDataType(
    stream: &mut ExiBitstream,
    mut WPT_LF_RxDataType: *mut iso20_wpt_WPT_LF_RxDataType,
) -> i32 {
    let mut grammar_id: i32 = 61 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_LF_RxDataType(WPT_LF_RxDataType);
    while done == 0 {
        match grammar_id {
            61 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*WPT_LF_RxDataType).RxIdentifier,
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
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_WPT_LF_RxRSSIListType(
                                stream,
                                &mut (*WPT_LF_RxDataType).RSSIData,
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
unsafe extern "C" fn decode_iso20_wpt_WPT_LF_TxDataListType(
    stream: &mut ExiBitstream,
    mut WPT_LF_TxDataListType: *mut iso20_wpt_WPT_LF_TxDataListType,
) -> i32 {
    let mut grammar_id: i32 = 63 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_LF_TxDataListType(WPT_LF_TxDataListType);
    while done == 0 {
        match grammar_id {
            63 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_WPT_LF_TxDataType(
                                stream,
                                &mut (*WPT_LF_TxDataListType).WPT_LF_TxDataList,
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
unsafe extern "C" fn decode_iso20_wpt_KeyInfoType(
    stream: &mut ExiBitstream,
    mut KeyInfoType: *mut iso20_wpt_KeyInfoType,
) -> i32 {
    let mut grammar_id: i32 = 64 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_KeyInfoType(KeyInfoType);
    while done == 0 {
        match grammar_id {
            64 => {
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
                            grammar_id = 65 as i32;
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
                                decode_iso20_wpt_KeyValueType(stream, &mut (*KeyInfoType).KeyValue);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_KeyValue_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_wpt_RetrievalMethodType(
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
                                decode_iso20_wpt_X509DataType(stream, &mut (*KeyInfoType).X509Data);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_X509Data_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        5 => {
                            error =
                                decode_iso20_wpt_PGPDataType(stream, &mut (*KeyInfoType).PGPData);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_PGPData_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        6 => {
                            error =
                                decode_iso20_wpt_SPKIDataType(stream, &mut (*KeyInfoType).SPKIData);
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
            65 => {
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
                                decode_iso20_wpt_KeyValueType(stream, &mut (*KeyInfoType).KeyValue);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_KeyValue_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_wpt_RetrievalMethodType(
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
                                decode_iso20_wpt_X509DataType(stream, &mut (*KeyInfoType).X509Data);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_X509Data_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        4 => {
                            error =
                                decode_iso20_wpt_PGPDataType(stream, &mut (*KeyInfoType).PGPData);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_PGPData_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        5 => {
                            error =
                                decode_iso20_wpt_SPKIDataType(stream, &mut (*KeyInfoType).SPKIData);
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
unsafe extern "C" fn decode_iso20_wpt_WPT_TxRxSpecDataType(
    stream: &mut ExiBitstream,
    mut WPT_TxRxSpecDataType: *mut iso20_wpt_WPT_TxRxSpecDataType,
) -> i32 {
    let mut grammar_id: i32 = 66 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_TxRxSpecDataType(WPT_TxRxSpecDataType);
    while done == 0 {
        match grammar_id {
            66 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*WPT_TxRxSpecDataType).TxRxIdentifier,
                            );
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
                            error = decode_iso20_wpt_WPT_CoordinateXYZType(
                                stream,
                                &mut (*WPT_TxRxSpecDataType).TxRxPosition,
                            );
                            if error == 0 as i32 {
                                grammar_id = 68 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            68 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_WPT_CoordinateXYZType(
                                stream,
                                &mut (*WPT_TxRxSpecDataType).TxRxOrientation,
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
unsafe extern "C" fn decode_iso20_wpt_WPT_LF_RxDataListType(
    stream: &mut ExiBitstream,
    mut WPT_LF_RxDataListType: *mut iso20_wpt_WPT_LF_RxDataListType,
) -> i32 {
    let mut grammar_id: i32 = 69 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_LF_RxDataListType(WPT_LF_RxDataListType);
    while done == 0 {
        match grammar_id {
            69 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_WPT_LF_RxDataType(
                                stream,
                                &mut (*WPT_LF_RxDataListType).WPT_LF_RxDataList,
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
unsafe extern "C" fn decode_iso20_wpt_ObjectType(
    stream: &mut ExiBitstream,
    mut ObjectType: *mut iso20_wpt_ObjectType,
) -> i32 {
    let mut grammar_id: i32 = 70 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_ObjectType(ObjectType);
    while done == 0 {
        match grammar_id {
            70 => {
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
                            grammar_id = 71 as i32;
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
                            grammar_id = 72 as i32;
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
                            grammar_id = 73 as i32;
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
            71 => {
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
                            grammar_id = 72 as i32;
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
                            grammar_id = 73 as i32;
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
            72 => {
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
                            grammar_id = 73 as i32;
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
            73 => {
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
unsafe extern "C" fn decode_iso20_wpt_WPT_TxRxPackageSpecDataType(
    stream: &mut ExiBitstream,
    mut WPT_TxRxPackageSpecDataType: *mut iso20_wpt_WPT_TxRxPackageSpecDataType,
) -> i32 {
    let mut grammar_id: i32 = 74 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_TxRxPackageSpecDataType(WPT_TxRxPackageSpecDataType);
    while done == 0 {
        match grammar_id {
            74 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_TxRxPackageSpecDataType).PulseSequenceOrder.arrayLen as i32)
                                < 255 as i32
                            {
                                let fresh2 =
                                    (*WPT_TxRxPackageSpecDataType).PulseSequenceOrder.arrayLen;
                                (*WPT_TxRxPackageSpecDataType).PulseSequenceOrder.arrayLen =
                                    ((*WPT_TxRxPackageSpecDataType).PulseSequenceOrder.arrayLen)
                                        .wrapping_add(1);
                                error = decode_iso20_wpt_WPT_TxRxPulseOrderType(
                                    stream,
                                    &mut *((*WPT_TxRxPackageSpecDataType).PulseSequenceOrder.array)
                                        .as_mut_ptr()
                                        .offset(fresh2 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 75 as i32;
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
                            if ((*WPT_TxRxPackageSpecDataType).PulseSequenceOrder.arrayLen as i32)
                                < 255 as i32
                            {
                                let fresh3 =
                                    (*WPT_TxRxPackageSpecDataType).PulseSequenceOrder.arrayLen;
                                (*WPT_TxRxPackageSpecDataType).PulseSequenceOrder.arrayLen =
                                    ((*WPT_TxRxPackageSpecDataType).PulseSequenceOrder.arrayLen)
                                        .wrapping_add(1);
                                error = decode_iso20_wpt_WPT_TxRxPulseOrderType(
                                    stream,
                                    &mut *((*WPT_TxRxPackageSpecDataType).PulseSequenceOrder.array)
                                        .as_mut_ptr()
                                        .offset(fresh3 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*WPT_TxRxPackageSpecDataType).PulseSequenceOrder.arrayLen as i32)
                                < 255 as i32
                            {
                                grammar_id = 75 as i32;
                            } else {
                                grammar_id = -(1 as i32);
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
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*WPT_TxRxPackageSpecDataType).PulseSeparationTime,
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
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*WPT_TxRxPackageSpecDataType).PulseDuration,
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
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*WPT_TxRxPackageSpecDataType).PackageSeparationTime,
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
unsafe extern "C" fn decode_iso20_wpt_WPT_LF_TransmitterDataType(
    stream: &mut ExiBitstream,
    mut WPT_LF_TransmitterDataType: *mut iso20_wpt_WPT_LF_TransmitterDataType,
) -> i32 {
    let mut grammar_id: i32 = 79 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_LF_TransmitterDataType(WPT_LF_TransmitterDataType);
    while done == 0 {
        match grammar_id {
            79 => {
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
                                        (*WPT_LF_TransmitterDataType).NumberOfTransmitters =
                                            value as u8;
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
                                        grammar_id = 80 as i32;
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
            80 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_LF_TransmitterDataType).SignalFrequency,
                            );
                            if error == 0 as i32 {
                                grammar_id = 81 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            81 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_LF_TransmitterDataType).TxSpecData.arrayLen as i32)
                                < 255 as i32
                            {
                                let fresh4 = (*WPT_LF_TransmitterDataType).TxSpecData.arrayLen;
                                (*WPT_LF_TransmitterDataType).TxSpecData.arrayLen =
                                    ((*WPT_LF_TransmitterDataType).TxSpecData.arrayLen)
                                        .wrapping_add(1);
                                error = decode_iso20_wpt_WPT_TxRxSpecDataType(
                                    stream,
                                    &mut *((*WPT_LF_TransmitterDataType).TxSpecData.array)
                                        .as_mut_ptr()
                                        .offset(fresh4 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 82 as i32;
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
            82 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_LF_TransmitterDataType).TxSpecData.arrayLen as i32)
                                < 255 as i32
                            {
                                let fresh5 = (*WPT_LF_TransmitterDataType).TxSpecData.arrayLen;
                                (*WPT_LF_TransmitterDataType).TxSpecData.arrayLen =
                                    ((*WPT_LF_TransmitterDataType).TxSpecData.arrayLen)
                                        .wrapping_add(1);
                                error = decode_iso20_wpt_WPT_TxRxSpecDataType(
                                    stream,
                                    &mut *((*WPT_LF_TransmitterDataType).TxSpecData.array)
                                        .as_mut_ptr()
                                        .offset(fresh5 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*WPT_LF_TransmitterDataType).TxSpecData.arrayLen as i32)
                                < 255 as i32
                            {
                                grammar_id = 82 as i32;
                            } else {
                                grammar_id = -(1 as i32);
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            83 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_WPT_TxRxPackageSpecDataType(
                                stream,
                                &mut (*WPT_LF_TransmitterDataType).TxPackageSpecData,
                            );
                            if error == 0 as i32 {
                                (*WPT_LF_TransmitterDataType)
                                    .set_TxPackageSpecData_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_wpt_AlternativeSECCType(
    stream: &mut ExiBitstream,
    mut AlternativeSECCType: *mut iso20_wpt_AlternativeSECCType,
) -> i32 {
    let mut grammar_id: i32 = 84 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_AlternativeSECCType(AlternativeSECCType);
    while done == 0 {
        match grammar_id {
            84 => {
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
                                        &mut (*AlternativeSECCType).SSID.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*AlternativeSECCType).SSID.charactersLen as i32
                                            >= 2 as i32
                                        {
                                            (*AlternativeSECCType).SSID.charactersLen =
                                                ((*AlternativeSECCType).SSID.charactersLen as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*AlternativeSECCType).SSID.charactersLen as usize,
                                                ((*AlternativeSECCType).SSID.characters)
                                                    .as_mut_ptr(),
                                                (255 as i32 + 1 as i32) as usize,
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
                                        (*AlternativeSECCType).set_SSID_isUsed(1 as u32);
                                        grammar_id = 85 as i32;
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
                                    error = exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut (*AlternativeSECCType).BSSID.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*AlternativeSECCType).BSSID.charactersLen as i32
                                            >= 2 as i32
                                        {
                                            (*AlternativeSECCType).BSSID.charactersLen =
                                                ((*AlternativeSECCType).BSSID.charactersLen as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*AlternativeSECCType).BSSID.charactersLen as usize,
                                                ((*AlternativeSECCType).BSSID.characters)
                                                    .as_mut_ptr(),
                                                (12 as i32 + 1 as i32) as usize,
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
                                        (*AlternativeSECCType).set_BSSID_isUsed(1 as u32);
                                        grammar_id = 86 as i32;
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
                                    error = exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut (*AlternativeSECCType).IPAddress.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*AlternativeSECCType).IPAddress.charactersLen as i32
                                            >= 2 as i32
                                        {
                                            (*AlternativeSECCType).IPAddress.charactersLen =
                                                ((*AlternativeSECCType).IPAddress.charactersLen
                                                    as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*AlternativeSECCType).IPAddress.charactersLen
                                                    as usize,
                                                ((*AlternativeSECCType).IPAddress.characters)
                                                    .as_mut_ptr(),
                                                (39 as i32 + 1 as i32) as usize,
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
                                        (*AlternativeSECCType).set_IPAddress_isUsed(1 as u32);
                                        grammar_id = 87 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        3 => {
                            error =
                                decode_exi_type_uint16(stream, &mut (*AlternativeSECCType).Port);
                            if error == 0 as i32 {
                                (*AlternativeSECCType).set_Port_isUsed(1 as u32);
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
            85 => {
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
                                        &mut (*AlternativeSECCType).BSSID.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*AlternativeSECCType).BSSID.charactersLen as i32
                                            >= 2 as i32
                                        {
                                            (*AlternativeSECCType).BSSID.charactersLen =
                                                ((*AlternativeSECCType).BSSID.charactersLen as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*AlternativeSECCType).BSSID.charactersLen as usize,
                                                ((*AlternativeSECCType).BSSID.characters)
                                                    .as_mut_ptr(),
                                                (12 as i32 + 1 as i32) as usize,
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
                                        (*AlternativeSECCType).set_BSSID_isUsed(1 as u32);
                                        grammar_id = 86 as i32;
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
                                    error = exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut (*AlternativeSECCType).IPAddress.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*AlternativeSECCType).IPAddress.charactersLen as i32
                                            >= 2 as i32
                                        {
                                            (*AlternativeSECCType).IPAddress.charactersLen =
                                                ((*AlternativeSECCType).IPAddress.charactersLen
                                                    as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*AlternativeSECCType).IPAddress.charactersLen
                                                    as usize,
                                                ((*AlternativeSECCType).IPAddress.characters)
                                                    .as_mut_ptr(),
                                                (39 as i32 + 1 as i32) as usize,
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
                                        (*AlternativeSECCType).set_IPAddress_isUsed(1 as u32);
                                        grammar_id = 87 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        2 => {
                            error =
                                decode_exi_type_uint16(stream, &mut (*AlternativeSECCType).Port);
                            if error == 0 as i32 {
                                (*AlternativeSECCType).set_Port_isUsed(1 as u32);
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
            86 => {
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
                                    error = exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut (*AlternativeSECCType).IPAddress.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*AlternativeSECCType).IPAddress.charactersLen as i32
                                            >= 2 as i32
                                        {
                                            (*AlternativeSECCType).IPAddress.charactersLen =
                                                ((*AlternativeSECCType).IPAddress.charactersLen
                                                    as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*AlternativeSECCType).IPAddress.charactersLen
                                                    as usize,
                                                ((*AlternativeSECCType).IPAddress.characters)
                                                    .as_mut_ptr(),
                                                (39 as i32 + 1 as i32) as usize,
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
                                        (*AlternativeSECCType).set_IPAddress_isUsed(1 as u32);
                                        grammar_id = 87 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            error =
                                decode_exi_type_uint16(stream, &mut (*AlternativeSECCType).Port);
                            if error == 0 as i32 {
                                (*AlternativeSECCType).set_Port_isUsed(1 as u32);
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
            87 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error =
                                decode_exi_type_uint16(stream, &mut (*AlternativeSECCType).Port);
                            if error == 0 as i32 {
                                (*AlternativeSECCType).set_Port_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_wpt_WPT_LF_ReceiverDataType(
    stream: &mut ExiBitstream,
    mut WPT_LF_ReceiverDataType: *mut iso20_wpt_WPT_LF_ReceiverDataType,
) -> i32 {
    let mut grammar_id: i32 = 88 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_LF_ReceiverDataType(WPT_LF_ReceiverDataType);
    while done == 0 {
        match grammar_id {
            88 => {
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
                                        (*WPT_LF_ReceiverDataType).NumberOfReceivers = value as u8;
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
                            if ((*WPT_LF_ReceiverDataType).RxSpecData.arrayLen as i32) < 255 as i32
                            {
                                let fresh6 = (*WPT_LF_ReceiverDataType).RxSpecData.arrayLen;
                                (*WPT_LF_ReceiverDataType).RxSpecData.arrayLen =
                                    ((*WPT_LF_ReceiverDataType).RxSpecData.arrayLen)
                                        .wrapping_add(1);
                                error = decode_iso20_wpt_WPT_TxRxSpecDataType(
                                    stream,
                                    &mut *((*WPT_LF_ReceiverDataType).RxSpecData.array)
                                        .as_mut_ptr()
                                        .offset(fresh6 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 90 as i32;
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
            90 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_LF_ReceiverDataType).RxSpecData.arrayLen as i32) < 255 as i32
                            {
                                let fresh7 = (*WPT_LF_ReceiverDataType).RxSpecData.arrayLen;
                                (*WPT_LF_ReceiverDataType).RxSpecData.arrayLen =
                                    ((*WPT_LF_ReceiverDataType).RxSpecData.arrayLen)
                                        .wrapping_add(1);
                                error = decode_iso20_wpt_WPT_TxRxSpecDataType(
                                    stream,
                                    &mut *((*WPT_LF_ReceiverDataType).RxSpecData.array)
                                        .as_mut_ptr()
                                        .offset(fresh7 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*WPT_LF_ReceiverDataType).RxSpecData.arrayLen as i32) < 255 as i32
                            {
                                grammar_id = 90 as i32;
                            } else {
                                grammar_id = -(1 as i32);
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
unsafe extern "C" fn decode_iso20_wpt_WPT_LF_DataPackageType(
    stream: &mut ExiBitstream,
    mut WPT_LF_DataPackageType: *mut iso20_wpt_WPT_LF_DataPackageType,
) -> i32 {
    let mut grammar_id: i32 = 91 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_LF_DataPackageType(WPT_LF_DataPackageType);
    while done == 0 {
        match grammar_id {
            91 => {
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
                                        (*WPT_LF_DataPackageType).PackageIndex = value as u8;
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
                                        grammar_id = 92 as i32;
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
            92 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_WPT_LF_TxDataListType(
                                stream,
                                &mut (*WPT_LF_DataPackageType).LF_TxData,
                            );
                            if error == 0 as i32 {
                                (*WPT_LF_DataPackageType).set_LF_TxData_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_wpt_WPT_LF_RxDataListType(
                                stream,
                                &mut (*WPT_LF_DataPackageType).LF_RxData,
                            );
                            if error == 0 as i32 {
                                (*WPT_LF_DataPackageType).set_LF_RxData_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_wpt_DetailedCostType(
    stream: &mut ExiBitstream,
    mut DetailedCostType: *mut iso20_wpt_DetailedCostType,
) -> i32 {
    let mut grammar_id: i32 = 93 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_DetailedCostType(DetailedCostType);
    while done == 0 {
        match grammar_id {
            93 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*DetailedCostType).Amount,
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
                            error = decode_iso20_wpt_RationalNumberType(
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
unsafe extern "C" fn decode_iso20_wpt_SignatureType(
    stream: &mut ExiBitstream,
    mut SignatureType: *mut iso20_wpt_SignatureType,
) -> i32 {
    let mut grammar_id: i32 = 95 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_SignatureType(SignatureType);
    while done == 0 {
        match grammar_id {
            95 => {
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
                            grammar_id = 96 as i32;
                        }
                        1 => {
                            error = decode_iso20_wpt_SignedInfoType(
                                stream,
                                &mut (*SignatureType).SignedInfo,
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
            96 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_SignedInfoType(
                                stream,
                                &mut (*SignatureType).SignedInfo,
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
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_SignatureValueType(
                                stream,
                                &mut (*SignatureType).SignatureValue,
                            );
                            if error == 0 as i32 {
                                grammar_id = 98 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            98 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error =
                                decode_iso20_wpt_KeyInfoType(stream, &mut (*SignatureType).KeyInfo);
                            if error == 0 as i32 {
                                (*SignatureType).set_KeyInfo_isUsed(1 as u32);
                                grammar_id = 100 as i32;
                            }
                        }
                        1 => {
                            error =
                                decode_iso20_wpt_ObjectType(stream, &mut (*SignatureType).Object);
                            if error == 0 as i32 {
                                (*SignatureType).set_Object_isUsed(1 as u32);
                                grammar_id = 99 as i32;
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
            99 => {
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
            100 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error =
                                decode_iso20_wpt_ObjectType(stream, &mut (*SignatureType).Object);
                            if error == 0 as i32 {
                                (*SignatureType).set_Object_isUsed(1 as u32);
                                grammar_id = 101 as i32;
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
            101 => {
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
unsafe extern "C" fn decode_iso20_wpt_DetailedTaxType(
    stream: &mut ExiBitstream,
    mut DetailedTaxType: *mut iso20_wpt_DetailedTaxType,
) -> i32 {
    let mut grammar_id: i32 = 102 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_DetailedTaxType(DetailedTaxType);
    while done == 0 {
        match grammar_id {
            102 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error =
                                decode_exi_type_uint32(stream, &mut (*DetailedTaxType).TaxRuleID);
                            if error == 0 as i32 {
                                grammar_id = 103 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            103 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_RationalNumberType(
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
unsafe extern "C" fn decode_iso20_wpt_MessageHeaderType(
    stream: &mut ExiBitstream,
    mut MessageHeaderType: *mut iso20_wpt_MessageHeaderType,
) -> i32 {
    let mut grammar_id: i32 = 104 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_MessageHeaderType(MessageHeaderType);
    while done == 0 {
        match grammar_id {
            104 => {
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
                                grammar_id = 105 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            105 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error =
                                decode_exi_type_uint64(stream, &mut (*MessageHeaderType).TimeStamp);
                            if error == 0 as i32 {
                                grammar_id = 106 as i32;
                            }
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
                            error = decode_iso20_wpt_SignatureType(
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
unsafe extern "C" fn decode_iso20_wpt_SignaturePropertyType(
    stream: &mut ExiBitstream,
    mut SignaturePropertyType: *mut iso20_wpt_SignaturePropertyType,
) -> i32 {
    let mut grammar_id: i32 = 107 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_SignaturePropertyType(SignaturePropertyType);
    while done == 0 {
        match grammar_id {
            107 => {
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
                            grammar_id = 108 as i32;
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
                            grammar_id = 109 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            108 => {
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
                            grammar_id = 109 as i32;
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
unsafe extern "C" fn decode_iso20_wpt_DisplayParametersType(
    stream: &mut ExiBitstream,
    mut DisplayParametersType: *mut iso20_wpt_DisplayParametersType,
) -> i32 {
    let mut grammar_id: i32 = 110 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_DisplayParametersType(DisplayParametersType);
    while done == 0 {
        match grammar_id {
            110 => {
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
                                        grammar_id = 111 as i32;
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
                                        grammar_id = 112 as i32;
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
                                        grammar_id = 113 as i32;
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
                                        grammar_id = 114 as i32;
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
                                grammar_id = 115 as i32;
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
                                grammar_id = 116 as i32;
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
                                grammar_id = 117 as i32;
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
                                        grammar_id = 118 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        8 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*DisplayParametersType).BatteryEnergyCapacity,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType).set_BatteryEnergyCapacity_isUsed(1 as u32);
                                grammar_id = 119 as i32;
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
            111 => {
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
                                        grammar_id = 112 as i32;
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
                                        grammar_id = 113 as i32;
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
                                        grammar_id = 114 as i32;
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
                                grammar_id = 115 as i32;
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
                                grammar_id = 116 as i32;
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
                                grammar_id = 117 as i32;
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
                                        grammar_id = 118 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        7 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*DisplayParametersType).BatteryEnergyCapacity,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType).set_BatteryEnergyCapacity_isUsed(1 as u32);
                                grammar_id = 119 as i32;
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
            112 => {
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
                                        grammar_id = 113 as i32;
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
                                        grammar_id = 114 as i32;
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
                                grammar_id = 115 as i32;
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
                                grammar_id = 116 as i32;
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
                                grammar_id = 117 as i32;
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
                                        grammar_id = 118 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        6 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*DisplayParametersType).BatteryEnergyCapacity,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType).set_BatteryEnergyCapacity_isUsed(1 as u32);
                                grammar_id = 119 as i32;
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
            113 => {
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
                                        grammar_id = 114 as i32;
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
                                grammar_id = 115 as i32;
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
                                grammar_id = 116 as i32;
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
                                grammar_id = 117 as i32;
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
                                        grammar_id = 118 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        5 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*DisplayParametersType).BatteryEnergyCapacity,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType).set_BatteryEnergyCapacity_isUsed(1 as u32);
                                grammar_id = 119 as i32;
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
            114 => {
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
                                grammar_id = 115 as i32;
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
                                grammar_id = 116 as i32;
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
                                grammar_id = 117 as i32;
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
                                        grammar_id = 118 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        4 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*DisplayParametersType).BatteryEnergyCapacity,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType).set_BatteryEnergyCapacity_isUsed(1 as u32);
                                grammar_id = 119 as i32;
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
            115 => {
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
                                grammar_id = 116 as i32;
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
                                grammar_id = 117 as i32;
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
                                        grammar_id = 118 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        3 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*DisplayParametersType).BatteryEnergyCapacity,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType).set_BatteryEnergyCapacity_isUsed(1 as u32);
                                grammar_id = 119 as i32;
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
            116 => {
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
                                grammar_id = 117 as i32;
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
                                        grammar_id = 118 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        2 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*DisplayParametersType).BatteryEnergyCapacity,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType).set_BatteryEnergyCapacity_isUsed(1 as u32);
                                grammar_id = 119 as i32;
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
            117 => {
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
                                        grammar_id = 118 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*DisplayParametersType).BatteryEnergyCapacity,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType).set_BatteryEnergyCapacity_isUsed(1 as u32);
                                grammar_id = 119 as i32;
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
            118 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*DisplayParametersType).BatteryEnergyCapacity,
                            );
                            if error == 0 as i32 {
                                (*DisplayParametersType).set_BatteryEnergyCapacity_isUsed(1 as u32);
                                grammar_id = 119 as i32;
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
            119 => {
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
unsafe extern "C" fn decode_iso20_wpt_WPT_FinePositioningMethodListType(
    stream: &mut ExiBitstream,
    mut WPT_FinePositioningMethodListType: *mut iso20_wpt_WPT_FinePositioningMethodListType,
) -> i32 {
    let mut grammar_id: i32 = 120 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_FinePositioningMethodListType(WPT_FinePositioningMethodListType);
    while done == 0 {
        match grammar_id {
            120 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_FinePositioningMethodListType)
                                .WPT_FinePositioningMethod
                                .arrayLen as i32)
                                < 8 as i32
                            {
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
                                            (*WPT_FinePositioningMethodListType)
                                                .WPT_FinePositioningMethod
                                                .array
                                                [(*WPT_FinePositioningMethodListType)
                                                    .WPT_FinePositioningMethod
                                                    .arrayLen
                                                    as usize] =
                                                value as iso20_wpt_WPT_FinePositioningMethodType;
                                            (*WPT_FinePositioningMethodListType)
                                                .WPT_FinePositioningMethod
                                                .arrayLen = ((*WPT_FinePositioningMethodListType)
                                                .WPT_FinePositioningMethod
                                                .arrayLen)
                                                .wrapping_add(1);
                                            (*WPT_FinePositioningMethodListType)
                                                .WPT_FinePositioningMethod
                                                .arrayLen;
                                        }
                                    } else {
                                        error = -(151 as i32);
                                    }
                                }
                            } else {
                                error = -(110 as i32);
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 121 as i32;
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
            121 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_FinePositioningMethodListType)
                                .WPT_FinePositioningMethod
                                .arrayLen as i32)
                                < 8 as i32
                            {
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
                                            3 as i32 as usize,
                                            &mut value_0,
                                        );
                                        if error == 0 as i32 {
                                            (*WPT_FinePositioningMethodListType)
                                                .WPT_FinePositioningMethod
                                                .array
                                                [(*WPT_FinePositioningMethodListType)
                                                    .WPT_FinePositioningMethod
                                                    .arrayLen
                                                    as usize] =
                                                value_0 as iso20_wpt_WPT_FinePositioningMethodType;
                                            (*WPT_FinePositioningMethodListType)
                                                .WPT_FinePositioningMethod
                                                .arrayLen = ((*WPT_FinePositioningMethodListType)
                                                .WPT_FinePositioningMethod
                                                .arrayLen)
                                                .wrapping_add(1);
                                            (*WPT_FinePositioningMethodListType)
                                                .WPT_FinePositioningMethod
                                                .arrayLen;
                                        }
                                    } else {
                                        error = -(151 as i32);
                                    }
                                }
                            } else {
                                error = -(110 as i32);
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 121 as i32;
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
unsafe extern "C" fn decode_iso20_wpt_EVSEStatusType(
    stream: &mut ExiBitstream,
    mut EVSEStatusType: *mut iso20_wpt_EVSEStatusType,
) -> i32 {
    let mut grammar_id: i32 = 122 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_EVSEStatusType(EVSEStatusType);
    while done == 0 {
        match grammar_id {
            122 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*EVSEStatusType).NotificationMaxDelay,
                            );
                            if error == 0 as i32 {
                                grammar_id = 123 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            123 => {
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
                                            value as iso20_wpt_evseNotificationType;
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
unsafe extern "C" fn decode_iso20_wpt_WPT_PairingMethodListType(
    stream: &mut ExiBitstream,
    mut WPT_PairingMethodListType: *mut iso20_wpt_WPT_PairingMethodListType,
) -> i32 {
    let mut grammar_id: i32 = 124 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_PairingMethodListType(WPT_PairingMethodListType);
    while done == 0 {
        match grammar_id {
            124 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_PairingMethodListType).WPT_PairingMethod.arrayLen as i32)
                                < 8 as i32
                            {
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
                                            (*WPT_PairingMethodListType).WPT_PairingMethod.array
                                                [(*WPT_PairingMethodListType)
                                                    .WPT_PairingMethod
                                                    .arrayLen
                                                    as usize] =
                                                value as iso20_wpt_WPT_PairingMethodType;
                                            (*WPT_PairingMethodListType)
                                                .WPT_PairingMethod
                                                .arrayLen = ((*WPT_PairingMethodListType)
                                                .WPT_PairingMethod
                                                .arrayLen)
                                                .wrapping_add(1);
                                            (*WPT_PairingMethodListType).WPT_PairingMethod.arrayLen;
                                        }
                                    } else {
                                        error = -(151 as i32);
                                    }
                                }
                            } else {
                                error = -(110 as i32);
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 125 as i32;
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
            125 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_PairingMethodListType).WPT_PairingMethod.arrayLen as i32)
                                < 8 as i32
                            {
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
                                            3 as i32 as usize,
                                            &mut value_0,
                                        );
                                        if error == 0 as i32 {
                                            (*WPT_PairingMethodListType).WPT_PairingMethod.array
                                                [(*WPT_PairingMethodListType)
                                                    .WPT_PairingMethod
                                                    .arrayLen
                                                    as usize] =
                                                value_0 as iso20_wpt_WPT_PairingMethodType;
                                            (*WPT_PairingMethodListType)
                                                .WPT_PairingMethod
                                                .arrayLen = ((*WPT_PairingMethodListType)
                                                .WPT_PairingMethod
                                                .arrayLen)
                                                .wrapping_add(1);
                                            (*WPT_PairingMethodListType).WPT_PairingMethod.arrayLen;
                                        }
                                    } else {
                                        error = -(151 as i32);
                                    }
                                }
                            } else {
                                error = -(110 as i32);
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 125 as i32;
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
unsafe extern "C" fn decode_iso20_wpt_MeterInfoType(
    stream: &mut ExiBitstream,
    mut MeterInfoType: *mut iso20_wpt_MeterInfoType,
) -> i32 {
    let mut grammar_id: i32 = 126 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_MeterInfoType(MeterInfoType);
    while done == 0 {
        match grammar_id {
            126 => {
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
                                        grammar_id = 127 as i32;
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
            127 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint64(
                                stream,
                                &mut (*MeterInfoType).ChargedEnergyReadingWh,
                            );
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
                            error = decode_exi_type_uint64(
                                stream,
                                &mut (*MeterInfoType).BPT_DischargedEnergyReadingWh,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_BPT_DischargedEnergyReadingWh_isUsed(1 as u32);
                                grammar_id = 129 as i32;
                            }
                        }
                        1 => {
                            error = decode_exi_type_uint64(
                                stream,
                                &mut (*MeterInfoType).CapacitiveEnergyReadingVARh,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_CapacitiveEnergyReadingVARh_isUsed(1 as u32);
                                grammar_id = 130 as i32;
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
                                grammar_id = 131 as i32;
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
                                grammar_id = 132 as i32;
                            }
                        }
                        4 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*MeterInfoType).MeterStatus,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_MeterStatus_isUsed(1 as u32);
                                grammar_id = 133 as i32;
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
            129 => {
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
                                grammar_id = 130 as i32;
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
                                grammar_id = 131 as i32;
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
                                grammar_id = 132 as i32;
                            }
                        }
                        3 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*MeterInfoType).MeterStatus,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_MeterStatus_isUsed(1 as u32);
                                grammar_id = 133 as i32;
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
            130 => {
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
                                grammar_id = 131 as i32;
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
                                grammar_id = 132 as i32;
                            }
                        }
                        2 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*MeterInfoType).MeterStatus,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_MeterStatus_isUsed(1 as u32);
                                grammar_id = 133 as i32;
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
            131 => {
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
                                grammar_id = 132 as i32;
                            }
                        }
                        1 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*MeterInfoType).MeterStatus,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_MeterStatus_isUsed(1 as u32);
                                grammar_id = 133 as i32;
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
            132 => {
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
                                grammar_id = 133 as i32;
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
            133 => {
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
unsafe extern "C" fn decode_iso20_wpt_WPT_AlignmentCheckMethodListType(
    stream: &mut ExiBitstream,
    mut WPT_AlignmentCheckMethodListType: *mut iso20_wpt_WPT_AlignmentCheckMethodListType,
) -> i32 {
    let mut grammar_id: i32 = 134 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_AlignmentCheckMethodListType(WPT_AlignmentCheckMethodListType);
    while done == 0 {
        match grammar_id {
            134 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_AlignmentCheckMethodListType)
                                .WPT_AlignmentCheckMethod
                                .arrayLen as i32)
                                < 8 as i32
                            {
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
                                            (*WPT_AlignmentCheckMethodListType)
                                                .WPT_AlignmentCheckMethod
                                                .array
                                                [(*WPT_AlignmentCheckMethodListType)
                                                    .WPT_AlignmentCheckMethod
                                                    .arrayLen
                                                    as usize] =
                                                value as iso20_wpt_WPT_AlignmentCheckMethodType;
                                            (*WPT_AlignmentCheckMethodListType)
                                                .WPT_AlignmentCheckMethod
                                                .arrayLen = ((*WPT_AlignmentCheckMethodListType)
                                                .WPT_AlignmentCheckMethod
                                                .arrayLen)
                                                .wrapping_add(1);
                                            (*WPT_AlignmentCheckMethodListType)
                                                .WPT_AlignmentCheckMethod
                                                .arrayLen;
                                        }
                                    } else {
                                        error = -(151 as i32);
                                    }
                                }
                            } else {
                                error = -(110 as i32);
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 135 as i32;
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
            135 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_AlignmentCheckMethodListType)
                                .WPT_AlignmentCheckMethod
                                .arrayLen as i32)
                                < 8 as i32
                            {
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
                                            (*WPT_AlignmentCheckMethodListType)
                                                .WPT_AlignmentCheckMethod
                                                .array
                                                [(*WPT_AlignmentCheckMethodListType)
                                                    .WPT_AlignmentCheckMethod
                                                    .arrayLen
                                                    as usize] =
                                                value_0 as iso20_wpt_WPT_AlignmentCheckMethodType;
                                            (*WPT_AlignmentCheckMethodListType)
                                                .WPT_AlignmentCheckMethod
                                                .arrayLen = ((*WPT_AlignmentCheckMethodListType)
                                                .WPT_AlignmentCheckMethod
                                                .arrayLen)
                                                .wrapping_add(1);
                                            (*WPT_AlignmentCheckMethodListType)
                                                .WPT_AlignmentCheckMethod
                                                .arrayLen;
                                        }
                                    } else {
                                        error = -(151 as i32);
                                    }
                                }
                            } else {
                                error = -(110 as i32);
                            }
                            if error == 0 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        grammar_id = 135 as i32;
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
unsafe extern "C" fn decode_iso20_wpt_WPT_LF_DataPackageListType(
    stream: &mut ExiBitstream,
    mut WPT_LF_DataPackageListType: *mut iso20_wpt_WPT_LF_DataPackageListType,
) -> i32 {
    let mut grammar_id: i32 = 136 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_LF_DataPackageListType(WPT_LF_DataPackageListType);
    while done == 0 {
        match grammar_id {
            136 => {
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
                                        (*WPT_LF_DataPackageListType).NumPackages = value as u8;
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
                                        grammar_id = 137 as i32;
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
            137 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_WPT_LF_DataPackageType(
                                stream,
                                &mut (*WPT_LF_DataPackageListType).WPT_LF_DataPackage,
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
unsafe extern "C" fn decode_iso20_wpt_AlternativeSECCListType(
    stream: &mut ExiBitstream,
    mut AlternativeSECCListType: *mut iso20_wpt_AlternativeSECCListType,
) -> i32 {
    let mut grammar_id: i32 = 138 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_AlternativeSECCListType(AlternativeSECCListType);
    while done == 0 {
        match grammar_id {
            138 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*AlternativeSECCListType).AlternativeSECC.arrayLen as i32)
                                < 8 as i32
                            {
                                let fresh8 = (*AlternativeSECCListType).AlternativeSECC.arrayLen;
                                (*AlternativeSECCListType).AlternativeSECC.arrayLen =
                                    ((*AlternativeSECCListType).AlternativeSECC.arrayLen)
                                        .wrapping_add(1);
                                error = decode_iso20_wpt_AlternativeSECCType(
                                    stream,
                                    &mut *((*AlternativeSECCListType).AlternativeSECC.array)
                                        .as_mut_ptr()
                                        .offset(fresh8 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 139 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            139 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*AlternativeSECCListType).AlternativeSECC.arrayLen as i32)
                                < 8 as i32
                            {
                                let fresh9 = (*AlternativeSECCListType).AlternativeSECC.arrayLen;
                                (*AlternativeSECCListType).AlternativeSECC.arrayLen =
                                    ((*AlternativeSECCListType).AlternativeSECC.arrayLen)
                                        .wrapping_add(1);
                                error = decode_iso20_wpt_AlternativeSECCType(
                                    stream,
                                    &mut *((*AlternativeSECCListType).AlternativeSECC.array)
                                        .as_mut_ptr()
                                        .offset(fresh9 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*AlternativeSECCListType).AlternativeSECC.arrayLen as i32)
                                < 8 as i32
                            {
                                grammar_id = 139 as i32;
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
unsafe extern "C" fn decode_iso20_wpt_ReceiptType(
    stream: &mut ExiBitstream,
    mut ReceiptType: *mut iso20_wpt_ReceiptType,
) -> i32 {
    let mut grammar_id: i32 = 140 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_ReceiptType(ReceiptType);
    while done == 0 {
        match grammar_id {
            140 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint64(stream, &mut (*ReceiptType).TimeAnchor);
                            if error == 0 as i32 {
                                grammar_id = 141 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            141 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).EnergyCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_EnergyCosts_isUsed(1 as u32);
                                grammar_id = 143 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_wpt_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).OccupancyCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_OccupancyCosts_isUsed(1 as u32);
                                grammar_id = 145 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_wpt_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).AdditionalServicesCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_AdditionalServicesCosts_isUsed(1 as u32);
                                grammar_id = 147 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_wpt_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).OverstayCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_OverstayCosts_isUsed(1 as u32);
                                grammar_id = 149 as i32;
                            }
                        }
                        4 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh10 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_wpt_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh10 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 142 as i32;
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
            142 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh11 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_wpt_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh11 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                grammar_id = 142 as i32;
                            } else {
                                grammar_id = 143 as i32;
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
            143 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).OccupancyCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_OccupancyCosts_isUsed(1 as u32);
                                grammar_id = 145 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_wpt_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).AdditionalServicesCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_AdditionalServicesCosts_isUsed(1 as u32);
                                grammar_id = 147 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_wpt_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).OverstayCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_OverstayCosts_isUsed(1 as u32);
                                grammar_id = 149 as i32;
                            }
                        }
                        3 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh12 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_wpt_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh12 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 144 as i32;
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
            144 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh13 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_wpt_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh13 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                grammar_id = 144 as i32;
                            } else {
                                grammar_id = 145 as i32;
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
            145 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).AdditionalServicesCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_AdditionalServicesCosts_isUsed(1 as u32);
                                grammar_id = 147 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_wpt_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).OverstayCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_OverstayCosts_isUsed(1 as u32);
                                grammar_id = 149 as i32;
                            }
                        }
                        2 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh14 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_wpt_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh14 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 146 as i32;
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
            146 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh15 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_wpt_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh15 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                grammar_id = 146 as i32;
                            } else {
                                grammar_id = 147 as i32;
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
            147 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).OverstayCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_OverstayCosts_isUsed(1 as u32);
                                grammar_id = 149 as i32;
                            }
                        }
                        1 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh16 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_wpt_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh16 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 148 as i32;
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
            148 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh17 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_wpt_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh17 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                grammar_id = 148 as i32;
                            } else {
                                grammar_id = 149 as i32;
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
            149 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh18 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_wpt_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh18 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 150 as i32;
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
            150 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh19 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_wpt_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh19 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                grammar_id = 150 as i32;
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
unsafe extern "C" fn decode_iso20_wpt_WPT_LF_SystemSetupDataType(
    stream: &mut ExiBitstream,
    mut WPT_LF_SystemSetupDataType: *mut iso20_wpt_WPT_LF_SystemSetupDataType,
) -> i32 {
    let mut grammar_id: i32 = 151 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_LF_SystemSetupDataType(WPT_LF_SystemSetupDataType);
    while done == 0 {
        match grammar_id {
            151 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_WPT_LF_TransmitterDataType(
                                stream,
                                &mut (*WPT_LF_SystemSetupDataType).LF_TransmitterSetupData,
                            );
                            if error == 0 as i32 {
                                (*WPT_LF_SystemSetupDataType)
                                    .set_LF_TransmitterSetupData_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_wpt_WPT_LF_ReceiverDataType(
                                stream,
                                &mut (*WPT_LF_SystemSetupDataType).LF_ReceiverSetupData,
                            );
                            if error == 0 as i32 {
                                (*WPT_LF_SystemSetupDataType)
                                    .set_LF_ReceiverSetupData_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_wpt_WPT_EVPCPowerControlParameterType(
    stream: &mut ExiBitstream,
    mut WPT_EVPCPowerControlParameterType: *mut iso20_wpt_WPT_EVPCPowerControlParameterType,
) -> i32 {
    let mut grammar_id: i32 = 152 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_EVPCPowerControlParameterType(WPT_EVPCPowerControlParameterType);
    while done == 0 {
        match grammar_id {
            152 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_EVPCPowerControlParameterType).EVPCCoilCurrentRequest,
                            );
                            if error == 0 as i32 {
                                grammar_id = 153 as i32;
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
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_EVPCPowerControlParameterType)
                                    .EVPCCoilCurrentInformation,
                            );
                            if error == 0 as i32 {
                                grammar_id = 154 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            154 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_EVPCPowerControlParameterType)
                                    .EVPCCurrentOutputInformation,
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
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_EVPCPowerControlParameterType)
                                    .EVPCVoltageOutputInformation,
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
unsafe extern "C" fn decode_iso20_wpt_WPT_SPCPowerControlParameterType(
    stream: &mut ExiBitstream,
    mut WPT_SPCPowerControlParameterType: *mut iso20_wpt_WPT_SPCPowerControlParameterType,
) -> i32 {
    let mut grammar_id: i32 = 156 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_SPCPowerControlParameterType(WPT_SPCPowerControlParameterType);
    while done == 0 {
        match grammar_id {
            156 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_SPCPowerControlParameterType)
                                    .SPCPrimaryDeviceCoilCurrentInformation,
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
unsafe extern "C" fn decode_iso20_wpt_WPT_FinePositioningSetupReqType(
    stream: &mut ExiBitstream,
    mut WPT_FinePositioningSetupReqType: *mut iso20_wpt_WPT_FinePositioningSetupReqType,
) -> i32 {
    let mut grammar_id: i32 = 157 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_FinePositioningSetupReqType(WPT_FinePositioningSetupReqType);
    while done == 0 {
        match grammar_id {
            157 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_MessageHeaderType(
                                stream,
                                &mut (*WPT_FinePositioningSetupReqType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 158 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            158 => {
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
                                        (*WPT_FinePositioningSetupReqType).EVProcessing =
                                            value as iso20_wpt_processingType;
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
                                        grammar_id = 159 as i32;
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
            159 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_WPT_FinePositioningMethodListType(
                                stream,
                                &mut (*WPT_FinePositioningSetupReqType)
                                    .EVDeviceFinePositioningMethodList,
                            );
                            if error == 0 as i32 {
                                grammar_id = 160 as i32;
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
                            error = decode_iso20_wpt_WPT_PairingMethodListType(
                                stream,
                                &mut (*WPT_FinePositioningSetupReqType).EVDevicePairingMethodList,
                            );
                            if error == 0 as i32 {
                                grammar_id = 161 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            161 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_WPT_AlignmentCheckMethodListType(
                                stream,
                                &mut (*WPT_FinePositioningSetupReqType)
                                    .EVDeviceAlignmentCheckMethodList,
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
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*WPT_FinePositioningSetupReqType).NaturalOffset,
                            );
                            if error == 0 as i32 {
                                grammar_id = 163 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            163 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_FinePositioningSetupReqType)
                                .VendorSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_FinePositioningSetupReqType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_FinePositioningSetupReqType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_FinePositioningSetupReqType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_FinePositioningSetupReqType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_FinePositioningSetupReqType)
                                        .VendorSpecificDataContainer
                                        .arrayLen = ((*WPT_FinePositioningSetupReqType)
                                        .VendorSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_FinePositioningSetupReqType)
                                        .VendorSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_FinePositioningSetupReqType)
                                        .set_VendorSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 164 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            164 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_FinePositioningSetupReqType)
                                .VendorSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_FinePositioningSetupReqType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_FinePositioningSetupReqType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_FinePositioningSetupReqType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_FinePositioningSetupReqType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_FinePositioningSetupReqType)
                                        .VendorSpecificDataContainer
                                        .arrayLen = ((*WPT_FinePositioningSetupReqType)
                                        .VendorSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_FinePositioningSetupReqType)
                                        .VendorSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_FinePositioningSetupReqType)
                                        .set_VendorSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 165 as i32;
                                }
                            } else {
                                error = -(110 as i32);
                            }
                        }
                        1 => {
                            error = decode_iso20_wpt_WPT_LF_SystemSetupDataType(
                                stream,
                                &mut (*WPT_FinePositioningSetupReqType).LF_SystemSetupData,
                            );
                            if error == 0 as i32 {
                                (*WPT_FinePositioningSetupReqType)
                                    .set_LF_SystemSetupData_isUsed(1 as u32);
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
            165 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_WPT_LF_SystemSetupDataType(
                                stream,
                                &mut (*WPT_FinePositioningSetupReqType).LF_SystemSetupData,
                            );
                            if error == 0 as i32 {
                                (*WPT_FinePositioningSetupReqType)
                                    .set_LF_SystemSetupData_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_wpt_WPT_FinePositioningSetupResType(
    stream: &mut ExiBitstream,
    mut WPT_FinePositioningSetupResType: *mut iso20_wpt_WPT_FinePositioningSetupResType,
) -> i32 {
    let mut grammar_id: i32 = 166 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_FinePositioningSetupResType(WPT_FinePositioningSetupResType);
    while done == 0 {
        match grammar_id {
            166 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_MessageHeaderType(
                                stream,
                                &mut (*WPT_FinePositioningSetupResType).Header,
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
                                        (*WPT_FinePositioningSetupResType).ResponseCode =
                                            value as iso20_wpt_responseCodeType;
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
                                        grammar_id = 168 as i32;
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
            168 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_WPT_FinePositioningMethodListType(
                                stream,
                                &mut (*WPT_FinePositioningSetupResType)
                                    .PrimaryDeviceFinePositioningMethodList,
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
                            error = decode_iso20_wpt_WPT_PairingMethodListType(
                                stream,
                                &mut (*WPT_FinePositioningSetupResType)
                                    .PrimaryDevicePairingMethodList,
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
                            error = decode_iso20_wpt_WPT_AlignmentCheckMethodListType(
                                stream,
                                &mut (*WPT_FinePositioningSetupResType)
                                    .PrimaryDeviceAlignmentCheckMethodList,
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
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*WPT_FinePositioningSetupResType).NaturalOffset,
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
                            if ((*WPT_FinePositioningSetupResType)
                                .VendorSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_FinePositioningSetupResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_FinePositioningSetupResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_FinePositioningSetupResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_FinePositioningSetupResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_FinePositioningSetupResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen = ((*WPT_FinePositioningSetupResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_FinePositioningSetupResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_FinePositioningSetupResType)
                                        .set_VendorSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 173 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            173 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_FinePositioningSetupResType)
                                .VendorSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_FinePositioningSetupResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_FinePositioningSetupResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_FinePositioningSetupResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_FinePositioningSetupResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_FinePositioningSetupResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen = ((*WPT_FinePositioningSetupResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_FinePositioningSetupResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_FinePositioningSetupResType)
                                        .set_VendorSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 174 as i32;
                                }
                            } else {
                                error = -(110 as i32);
                            }
                        }
                        1 => {
                            error = decode_iso20_wpt_WPT_LF_SystemSetupDataType(
                                stream,
                                &mut (*WPT_FinePositioningSetupResType).LF_SystemSetupData,
                            );
                            if error == 0 as i32 {
                                (*WPT_FinePositioningSetupResType)
                                    .set_LF_SystemSetupData_isUsed(1 as u32);
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
            174 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_WPT_LF_SystemSetupDataType(
                                stream,
                                &mut (*WPT_FinePositioningSetupResType).LF_SystemSetupData,
                            );
                            if error == 0 as i32 {
                                (*WPT_FinePositioningSetupResType)
                                    .set_LF_SystemSetupData_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_wpt_WPT_FinePositioningReqType(
    stream: &mut ExiBitstream,
    mut WPT_FinePositioningReqType: *mut iso20_wpt_WPT_FinePositioningReqType,
) -> i32 {
    let mut grammar_id: i32 = 175 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_FinePositioningReqType(WPT_FinePositioningReqType);
    while done == 0 {
        match grammar_id {
            175 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_MessageHeaderType(
                                stream,
                                &mut (*WPT_FinePositioningReqType).Header,
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
                                        (*WPT_FinePositioningReqType).EVProcessing =
                                            value as iso20_wpt_processingType;
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
                                        grammar_id = 177 as i32;
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
            177 => {
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
                                        (*WPT_FinePositioningReqType).EVResultCode =
                                            value_0 as iso20_wpt_WPT_EVResultType;
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
                                        grammar_id = 178 as i32;
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
            178 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_FinePositioningReqType)
                                .VendorSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_FinePositioningReqType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_FinePositioningReqType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_FinePositioningReqType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_FinePositioningReqType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_FinePositioningReqType)
                                        .VendorSpecificDataContainer
                                        .arrayLen = ((*WPT_FinePositioningReqType)
                                        .VendorSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_FinePositioningReqType)
                                        .VendorSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_FinePositioningReqType)
                                        .set_VendorSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 179 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            179 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_FinePositioningReqType)
                                .VendorSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_FinePositioningReqType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_FinePositioningReqType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_FinePositioningReqType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_FinePositioningReqType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_FinePositioningReqType)
                                        .VendorSpecificDataContainer
                                        .arrayLen = ((*WPT_FinePositioningReqType)
                                        .VendorSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_FinePositioningReqType)
                                        .VendorSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_FinePositioningReqType)
                                        .set_VendorSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 180 as i32;
                                }
                            } else {
                                error = -(110 as i32);
                            }
                        }
                        1 => {
                            error = decode_iso20_wpt_WPT_LF_DataPackageListType(
                                stream,
                                &mut (*WPT_FinePositioningReqType).WPT_LF_DataPackageList,
                            );
                            if error == 0 as i32 {
                                (*WPT_FinePositioningReqType)
                                    .set_WPT_LF_DataPackageList_isUsed(1 as u32);
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
            180 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_WPT_LF_DataPackageListType(
                                stream,
                                &mut (*WPT_FinePositioningReqType).WPT_LF_DataPackageList,
                            );
                            if error == 0 as i32 {
                                (*WPT_FinePositioningReqType)
                                    .set_WPT_LF_DataPackageList_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_wpt_WPT_FinePositioningResType(
    stream: &mut ExiBitstream,
    mut WPT_FinePositioningResType: *mut iso20_wpt_WPT_FinePositioningResType,
) -> i32 {
    let mut grammar_id: i32 = 181 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_FinePositioningResType(WPT_FinePositioningResType);
    while done == 0 {
        match grammar_id {
            181 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_MessageHeaderType(
                                stream,
                                &mut (*WPT_FinePositioningResType).Header,
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
                                        (*WPT_FinePositioningResType).ResponseCode =
                                            value as iso20_wpt_responseCodeType;
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
                                        grammar_id = 183 as i32;
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
            183 => {
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
                                        (*WPT_FinePositioningResType).EVSEProcessing =
                                            value_0 as iso20_wpt_processingType;
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
                                        grammar_id = 184 as i32;
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
            184 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_FinePositioningResType)
                                .VendorSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_FinePositioningResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_FinePositioningResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_FinePositioningResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_FinePositioningResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_FinePositioningResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen = ((*WPT_FinePositioningResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_FinePositioningResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_FinePositioningResType)
                                        .set_VendorSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 185 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            185 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_FinePositioningResType)
                                .VendorSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_FinePositioningResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_FinePositioningResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_FinePositioningResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_FinePositioningResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_FinePositioningResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen = ((*WPT_FinePositioningResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_FinePositioningResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_FinePositioningResType)
                                        .set_VendorSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 186 as i32;
                                }
                            } else {
                                error = -(110 as i32);
                            }
                        }
                        1 => {
                            error = decode_iso20_wpt_WPT_LF_DataPackageListType(
                                stream,
                                &mut (*WPT_FinePositioningResType).WPT_LF_DataPackageList,
                            );
                            if error == 0 as i32 {
                                (*WPT_FinePositioningResType)
                                    .set_WPT_LF_DataPackageList_isUsed(1 as u32);
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
            186 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_WPT_LF_DataPackageListType(
                                stream,
                                &mut (*WPT_FinePositioningResType).WPT_LF_DataPackageList,
                            );
                            if error == 0 as i32 {
                                (*WPT_FinePositioningResType)
                                    .set_WPT_LF_DataPackageList_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_wpt_WPT_PairingReqType(
    stream: &mut ExiBitstream,
    mut WPT_PairingReqType: *mut iso20_wpt_WPT_PairingReqType,
) -> i32 {
    let mut grammar_id: i32 = 187 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_PairingReqType(WPT_PairingReqType);
    while done == 0 {
        match grammar_id {
            187 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_MessageHeaderType(
                                stream,
                                &mut (*WPT_PairingReqType).Header,
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
                                        (*WPT_PairingReqType).EVProcessing =
                                            value as iso20_wpt_processingType;
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
                                        grammar_id = 189 as i32;
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
            189 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*WPT_PairingReqType).ObservedIDCode,
                            );
                            if error == 0 as i32 {
                                (*WPT_PairingReqType).set_ObservedIDCode_isUsed(1 as u32);
                                grammar_id = 190 as i32;
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
                                        2 as i32 as usize,
                                        &mut value_0,
                                    );
                                    if error == 0 as i32 {
                                        (*WPT_PairingReqType).EVResultCode =
                                            value_0 as iso20_wpt_WPT_EVResultType;
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
                                        grammar_id = 191 as i32;
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
            190 => {
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
                                        2 as i32 as usize,
                                        &mut value_1,
                                    );
                                    if error == 0 as i32 {
                                        (*WPT_PairingReqType).EVResultCode =
                                            value_1 as iso20_wpt_WPT_EVResultType;
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
                                        grammar_id = 191 as i32;
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
            191 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_PairingReqType).VendorSpecificDataContainer.arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_PairingReqType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_PairingReqType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_PairingReqType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_PairingReqType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_PairingReqType).VendorSpecificDataContainer.arrayLen =
                                        ((*WPT_PairingReqType)
                                            .VendorSpecificDataContainer
                                            .arrayLen)
                                            .wrapping_add(1);
                                    (*WPT_PairingReqType).VendorSpecificDataContainer.arrayLen;
                                    (*WPT_PairingReqType)
                                        .set_VendorSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 192 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            192 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_PairingReqType).VendorSpecificDataContainer.arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_PairingReqType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_PairingReqType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_PairingReqType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_PairingReqType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_PairingReqType).VendorSpecificDataContainer.arrayLen =
                                        ((*WPT_PairingReqType)
                                            .VendorSpecificDataContainer
                                            .arrayLen)
                                            .wrapping_add(1);
                                    (*WPT_PairingReqType).VendorSpecificDataContainer.arrayLen;
                                    (*WPT_PairingReqType)
                                        .set_VendorSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 192 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
unsafe extern "C" fn decode_iso20_wpt_WPT_PairingResType(
    stream: &mut ExiBitstream,
    mut WPT_PairingResType: *mut iso20_wpt_WPT_PairingResType,
) -> i32 {
    let mut grammar_id: i32 = 193 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_PairingResType(WPT_PairingResType);
    while done == 0 {
        match grammar_id {
            193 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_MessageHeaderType(
                                stream,
                                &mut (*WPT_PairingResType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 194 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            194 => {
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
                                        (*WPT_PairingResType).ResponseCode =
                                            value as iso20_wpt_responseCodeType;
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
                                        grammar_id = 195 as i32;
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
            195 => {
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
                                        (*WPT_PairingResType).EVSEProcessing =
                                            value_0 as iso20_wpt_processingType;
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
                                        grammar_id = 196 as i32;
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
            196 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*WPT_PairingResType).ObservedIDCode,
                            );
                            if error == 0 as i32 {
                                (*WPT_PairingResType).set_ObservedIDCode_isUsed(1 as u32);
                                grammar_id = 198 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_wpt_AlternativeSECCListType(
                                stream,
                                &mut (*WPT_PairingResType).AlternativeSECCList,
                            );
                            if error == 0 as i32 {
                                (*WPT_PairingResType).set_AlternativeSECCList_isUsed(1 as u32);
                                grammar_id = 200 as i32;
                            }
                        }
                        2 => {
                            if ((*WPT_PairingResType).VendorSpecificDataContainer.arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_PairingResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_PairingResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_PairingResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_PairingResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_PairingResType).VendorSpecificDataContainer.arrayLen =
                                        ((*WPT_PairingResType)
                                            .VendorSpecificDataContainer
                                            .arrayLen)
                                            .wrapping_add(1);
                                    (*WPT_PairingResType).VendorSpecificDataContainer.arrayLen;
                                    (*WPT_PairingResType)
                                        .set_VendorSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 197 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            197 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_PairingResType).VendorSpecificDataContainer.arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_PairingResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_PairingResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_PairingResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_PairingResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_PairingResType).VendorSpecificDataContainer.arrayLen =
                                        ((*WPT_PairingResType)
                                            .VendorSpecificDataContainer
                                            .arrayLen)
                                            .wrapping_add(1);
                                    (*WPT_PairingResType).VendorSpecificDataContainer.arrayLen;
                                    (*WPT_PairingResType)
                                        .set_VendorSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 197 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            198 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_AlternativeSECCListType(
                                stream,
                                &mut (*WPT_PairingResType).AlternativeSECCList,
                            );
                            if error == 0 as i32 {
                                (*WPT_PairingResType).set_AlternativeSECCList_isUsed(1 as u32);
                                grammar_id = 200 as i32;
                            }
                        }
                        1 => {
                            if ((*WPT_PairingResType).VendorSpecificDataContainer.arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_PairingResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_PairingResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_PairingResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_PairingResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_PairingResType).VendorSpecificDataContainer.arrayLen =
                                        ((*WPT_PairingResType)
                                            .VendorSpecificDataContainer
                                            .arrayLen)
                                            .wrapping_add(1);
                                    (*WPT_PairingResType).VendorSpecificDataContainer.arrayLen;
                                    (*WPT_PairingResType)
                                        .set_VendorSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 199 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            199 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_PairingResType).VendorSpecificDataContainer.arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_PairingResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_PairingResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_PairingResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_PairingResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_PairingResType).VendorSpecificDataContainer.arrayLen =
                                        ((*WPT_PairingResType)
                                            .VendorSpecificDataContainer
                                            .arrayLen)
                                            .wrapping_add(1);
                                    (*WPT_PairingResType).VendorSpecificDataContainer.arrayLen;
                                    (*WPT_PairingResType)
                                        .set_VendorSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 199 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            200 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_PairingResType).VendorSpecificDataContainer.arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_PairingResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_PairingResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_PairingResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_PairingResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_PairingResType).VendorSpecificDataContainer.arrayLen =
                                        ((*WPT_PairingResType)
                                            .VendorSpecificDataContainer
                                            .arrayLen)
                                            .wrapping_add(1);
                                    (*WPT_PairingResType).VendorSpecificDataContainer.arrayLen;
                                    (*WPT_PairingResType)
                                        .set_VendorSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 201 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            201 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_PairingResType).VendorSpecificDataContainer.arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_PairingResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_PairingResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_PairingResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_PairingResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_PairingResType).VendorSpecificDataContainer.arrayLen =
                                        ((*WPT_PairingResType)
                                            .VendorSpecificDataContainer
                                            .arrayLen)
                                            .wrapping_add(1);
                                    (*WPT_PairingResType).VendorSpecificDataContainer.arrayLen;
                                    (*WPT_PairingResType)
                                        .set_VendorSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 201 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
unsafe extern "C" fn decode_iso20_wpt_WPT_ChargeParameterDiscoveryReqType(
    stream: &mut ExiBitstream,
    mut WPT_ChargeParameterDiscoveryReqType: *mut iso20_wpt_WPT_ChargeParameterDiscoveryReqType,
) -> i32 {
    let mut grammar_id: i32 = 202 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_ChargeParameterDiscoveryReqType(WPT_ChargeParameterDiscoveryReqType);
    while done == 0 {
        match grammar_id {
            202 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_MessageHeaderType(
                                stream,
                                &mut (*WPT_ChargeParameterDiscoveryReqType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 203 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            203 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_ChargeParameterDiscoveryReqType).EVPCMaxReceivablePower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 204 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            204 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*WPT_ChargeParameterDiscoveryReqType).SDMaxGroundClearence,
                            );
                            if error == 0 as i32 {
                                grammar_id = 205 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            205 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*WPT_ChargeParameterDiscoveryReqType).SDMinGroundClearence,
                            );
                            if error == 0 as i32 {
                                grammar_id = 206 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            206 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_ChargeParameterDiscoveryReqType).EVPCNaturalFrequency,
                            );
                            if error == 0 as i32 {
                                grammar_id = 207 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            207 => {
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
                                        1 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*WPT_ChargeParameterDiscoveryReqType)
                                            .EVPCDeviceLocalControl = value as i32;
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
                                        grammar_id = 208 as i32;
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
            208 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_ChargeParameterDiscoveryReqType)
                                .VendorSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_ChargeParameterDiscoveryReqType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeParameterDiscoveryReqType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_ChargeParameterDiscoveryReqType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeParameterDiscoveryReqType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_ChargeParameterDiscoveryReqType)
                                        .VendorSpecificDataContainer
                                        .arrayLen = ((*WPT_ChargeParameterDiscoveryReqType)
                                        .VendorSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_ChargeParameterDiscoveryReqType)
                                        .VendorSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_ChargeParameterDiscoveryReqType)
                                        .set_VendorSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 209 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            209 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_ChargeParameterDiscoveryReqType)
                                .VendorSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_ChargeParameterDiscoveryReqType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeParameterDiscoveryReqType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_ChargeParameterDiscoveryReqType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeParameterDiscoveryReqType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_ChargeParameterDiscoveryReqType)
                                        .VendorSpecificDataContainer
                                        .arrayLen = ((*WPT_ChargeParameterDiscoveryReqType)
                                        .VendorSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_ChargeParameterDiscoveryReqType)
                                        .VendorSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_ChargeParameterDiscoveryReqType)
                                        .set_VendorSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 209 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
unsafe extern "C" fn decode_iso20_wpt_WPT_ChargeParameterDiscoveryResType(
    stream: &mut ExiBitstream,
    mut WPT_ChargeParameterDiscoveryResType: *mut iso20_wpt_WPT_ChargeParameterDiscoveryResType,
) -> i32 {
    let mut grammar_id: i32 = 210 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_ChargeParameterDiscoveryResType(WPT_ChargeParameterDiscoveryResType);
    while done == 0 {
        match grammar_id {
            210 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_MessageHeaderType(
                                stream,
                                &mut (*WPT_ChargeParameterDiscoveryResType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 211 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            211 => {
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
                                        (*WPT_ChargeParameterDiscoveryResType).ResponseCode =
                                            value as iso20_wpt_responseCodeType;
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
                                        grammar_id = 212 as i32;
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
            212 => {
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
                                        (*WPT_ChargeParameterDiscoveryResType).PDInputPowerClass =
                                            value_0 as iso20_wpt_WPT_PowerClassType;
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
                                        grammar_id = 213 as i32;
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
            213 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_ChargeParameterDiscoveryResType).SDMinOutputPower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 214 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            214 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_ChargeParameterDiscoveryResType).SDMaxOutputPower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 215 as i32;
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
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*WPT_ChargeParameterDiscoveryResType)
                                    .SDMaxGroundClearanceSupport,
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
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*WPT_ChargeParameterDiscoveryResType)
                                    .SDMinGroundClearanceSupport,
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
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_ChargeParameterDiscoveryResType).PDMinCoilCurrent,
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
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_ChargeParameterDiscoveryResType).PDMaxCoilCurrent,
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
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_ChargeParameterDiscoveryResType)
                                .SDManufacturerSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_ChargeParameterDiscoveryResType)
                                        .SDManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeParameterDiscoveryResType)
                                                .SDManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_ChargeParameterDiscoveryResType)
                                        .SDManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeParameterDiscoveryResType)
                                                .SDManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_ChargeParameterDiscoveryResType)
                                        .SDManufacturerSpecificDataContainer
                                        .arrayLen = ((*WPT_ChargeParameterDiscoveryResType)
                                        .SDManufacturerSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_ChargeParameterDiscoveryResType)
                                        .SDManufacturerSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_ChargeParameterDiscoveryResType)
                                        .set_SDManufacturerSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 220 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            220 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_ChargeParameterDiscoveryResType)
                                .SDManufacturerSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_ChargeParameterDiscoveryResType)
                                        .SDManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeParameterDiscoveryResType)
                                                .SDManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_ChargeParameterDiscoveryResType)
                                        .SDManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeParameterDiscoveryResType)
                                                .SDManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_ChargeParameterDiscoveryResType)
                                        .SDManufacturerSpecificDataContainer
                                        .arrayLen = ((*WPT_ChargeParameterDiscoveryResType)
                                        .SDManufacturerSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_ChargeParameterDiscoveryResType)
                                        .SDManufacturerSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_ChargeParameterDiscoveryResType)
                                        .set_SDManufacturerSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 220 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
unsafe extern "C" fn decode_iso20_wpt_WPT_AlignmentCheckReqType(
    stream: &mut ExiBitstream,
    mut WPT_AlignmentCheckReqType: *mut iso20_wpt_WPT_AlignmentCheckReqType,
) -> i32 {
    let mut grammar_id: i32 = 221 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_AlignmentCheckReqType(WPT_AlignmentCheckReqType);
    while done == 0 {
        match grammar_id {
            221 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_MessageHeaderType(
                                stream,
                                &mut (*WPT_AlignmentCheckReqType).Header,
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
                                        (*WPT_AlignmentCheckReqType).EVProcessing =
                                            value as iso20_wpt_processingType;
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
                                        grammar_id = 223 as i32;
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
            223 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_AlignmentCheckReqType).TargetCoilCurrent,
                            );
                            if error == 0 as i32 {
                                (*WPT_AlignmentCheckReqType).set_TargetCoilCurrent_isUsed(1 as u32);
                                grammar_id = 224 as i32;
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
                                        2 as i32 as usize,
                                        &mut value_0,
                                    );
                                    if error == 0 as i32 {
                                        (*WPT_AlignmentCheckReqType).EVResultCode =
                                            value_0 as iso20_wpt_WPT_EVResultType;
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
                                        grammar_id = 225 as i32;
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
            224 => {
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
                                        2 as i32 as usize,
                                        &mut value_1,
                                    );
                                    if error == 0 as i32 {
                                        (*WPT_AlignmentCheckReqType).EVResultCode =
                                            value_1 as iso20_wpt_WPT_EVResultType;
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
                                        grammar_id = 225 as i32;
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
            225 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_AlignmentCheckReqType)
                                .VendorSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_AlignmentCheckReqType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_AlignmentCheckReqType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_AlignmentCheckReqType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_AlignmentCheckReqType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_AlignmentCheckReqType)
                                        .VendorSpecificDataContainer
                                        .arrayLen = ((*WPT_AlignmentCheckReqType)
                                        .VendorSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_AlignmentCheckReqType)
                                        .VendorSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_AlignmentCheckReqType)
                                        .set_VendorSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 226 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            226 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_AlignmentCheckReqType)
                                .VendorSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_AlignmentCheckReqType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_AlignmentCheckReqType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_AlignmentCheckReqType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_AlignmentCheckReqType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_AlignmentCheckReqType)
                                        .VendorSpecificDataContainer
                                        .arrayLen = ((*WPT_AlignmentCheckReqType)
                                        .VendorSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_AlignmentCheckReqType)
                                        .VendorSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_AlignmentCheckReqType)
                                        .set_VendorSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 226 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
unsafe extern "C" fn decode_iso20_wpt_WPT_AlignmentCheckResType(
    stream: &mut ExiBitstream,
    mut WPT_AlignmentCheckResType: *mut iso20_wpt_WPT_AlignmentCheckResType,
) -> i32 {
    let mut grammar_id: i32 = 227 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_AlignmentCheckResType(WPT_AlignmentCheckResType);
    while done == 0 {
        match grammar_id {
            227 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_MessageHeaderType(
                                stream,
                                &mut (*WPT_AlignmentCheckResType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 228 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            228 => {
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
                                        (*WPT_AlignmentCheckResType).ResponseCode =
                                            value as iso20_wpt_responseCodeType;
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
                                        grammar_id = 229 as i32;
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
            229 => {
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
                                        (*WPT_AlignmentCheckResType).EVSEProcessing =
                                            value_0 as iso20_wpt_processingType;
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
                                        grammar_id = 230 as i32;
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
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_AlignmentCheckResType).PowerTransmitted,
                            );
                            if error == 0 as i32 {
                                (*WPT_AlignmentCheckResType).set_PowerTransmitted_isUsed(1 as u32);
                                grammar_id = 232 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_AlignmentCheckResType).SupplyDeviceCurrent,
                            );
                            if error == 0 as i32 {
                                (*WPT_AlignmentCheckResType)
                                    .set_SupplyDeviceCurrent_isUsed(1 as u32);
                                grammar_id = 234 as i32;
                            }
                        }
                        2 => {
                            if ((*WPT_AlignmentCheckResType)
                                .VendorSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_AlignmentCheckResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_AlignmentCheckResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen = ((*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_AlignmentCheckResType)
                                        .set_VendorSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 231 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            231 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_AlignmentCheckResType)
                                .VendorSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_AlignmentCheckResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_AlignmentCheckResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen = ((*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_AlignmentCheckResType)
                                        .set_VendorSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 231 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            232 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_AlignmentCheckResType).SupplyDeviceCurrent,
                            );
                            if error == 0 as i32 {
                                (*WPT_AlignmentCheckResType)
                                    .set_SupplyDeviceCurrent_isUsed(1 as u32);
                                grammar_id = 234 as i32;
                            }
                        }
                        1 => {
                            if ((*WPT_AlignmentCheckResType)
                                .VendorSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_AlignmentCheckResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_AlignmentCheckResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen = ((*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_AlignmentCheckResType)
                                        .set_VendorSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 233 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            233 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_AlignmentCheckResType)
                                .VendorSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_AlignmentCheckResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_AlignmentCheckResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen = ((*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_AlignmentCheckResType)
                                        .set_VendorSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 233 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            234 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_AlignmentCheckResType)
                                .VendorSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_AlignmentCheckResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_AlignmentCheckResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen = ((*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_AlignmentCheckResType)
                                        .set_VendorSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 235 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            235 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_AlignmentCheckResType)
                                .VendorSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_AlignmentCheckResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_AlignmentCheckResType)
                                                .VendorSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen = ((*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_AlignmentCheckResType)
                                        .VendorSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_AlignmentCheckResType)
                                        .set_VendorSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 235 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
unsafe extern "C" fn decode_iso20_wpt_WPT_ChargeLoopReqType(
    stream: &mut ExiBitstream,
    mut WPT_ChargeLoopReqType: *mut iso20_wpt_WPT_ChargeLoopReqType,
) -> i32 {
    let mut grammar_id: i32 = 236 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_ChargeLoopReqType(WPT_ChargeLoopReqType);
    while done == 0 {
        match grammar_id {
            236 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_MessageHeaderType(
                                stream,
                                &mut (*WPT_ChargeLoopReqType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 237 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            237 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_DisplayParametersType(
                                stream,
                                &mut (*WPT_ChargeLoopReqType).DisplayParameters,
                            );
                            if error == 0 as i32 {
                                (*WPT_ChargeLoopReqType).set_DisplayParameters_isUsed(1 as u32);
                                grammar_id = 238 as i32;
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
                                        (*WPT_ChargeLoopReqType).MeterInfoRequested = value as i32;
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
                                        grammar_id = 239 as i32;
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
            238 => {
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
                                        (*WPT_ChargeLoopReqType).MeterInfoRequested =
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
                                        grammar_id = 239 as i32;
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
            239 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_ChargeLoopReqType).EVPCPowerRequest,
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
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_ChargeLoopReqType).EVPCPowerOutput,
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
                                        2 as i32 as usize,
                                        &mut value_1,
                                    );
                                    if error == 0 as i32 {
                                        (*WPT_ChargeLoopReqType).EVPCChargeDiagnostics =
                                            value_1 as iso20_wpt_WPT_EVPCChargeDiagnosticsType;
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
                                        grammar_id = 242 as i32;
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
            242 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_ChargeLoopReqType).EVPCOperatingFrequency,
                            );
                            if error == 0 as i32 {
                                (*WPT_ChargeLoopReqType)
                                    .set_EVPCOperatingFrequency_isUsed(1 as u32);
                                grammar_id = 244 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_wpt_WPT_EVPCPowerControlParameterType(
                                stream,
                                &mut (*WPT_ChargeLoopReqType).EVPCPowerControlParameter,
                            );
                            if error == 0 as i32 {
                                (*WPT_ChargeLoopReqType)
                                    .set_EVPCPowerControlParameter_isUsed(1 as u32);
                                grammar_id = 246 as i32;
                            }
                        }
                        2 => {
                            if ((*WPT_ChargeLoopReqType)
                                .ManufacturerSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeLoopReqType)
                                                .ManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeLoopReqType)
                                                .ManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen = ((*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_ChargeLoopReqType)
                                        .set_ManufacturerSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 243 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            243 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_ChargeLoopReqType)
                                .ManufacturerSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeLoopReqType)
                                                .ManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeLoopReqType)
                                                .ManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen = ((*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_ChargeLoopReqType)
                                        .set_ManufacturerSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 243 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            244 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_WPT_EVPCPowerControlParameterType(
                                stream,
                                &mut (*WPT_ChargeLoopReqType).EVPCPowerControlParameter,
                            );
                            if error == 0 as i32 {
                                (*WPT_ChargeLoopReqType)
                                    .set_EVPCPowerControlParameter_isUsed(1 as u32);
                                grammar_id = 246 as i32;
                            }
                        }
                        1 => {
                            if ((*WPT_ChargeLoopReqType)
                                .ManufacturerSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeLoopReqType)
                                                .ManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeLoopReqType)
                                                .ManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen = ((*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_ChargeLoopReqType)
                                        .set_ManufacturerSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 245 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            245 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_ChargeLoopReqType)
                                .ManufacturerSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeLoopReqType)
                                                .ManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeLoopReqType)
                                                .ManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen = ((*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_ChargeLoopReqType)
                                        .set_ManufacturerSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 245 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            246 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_ChargeLoopReqType)
                                .ManufacturerSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeLoopReqType)
                                                .ManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeLoopReqType)
                                                .ManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen = ((*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_ChargeLoopReqType)
                                        .set_ManufacturerSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 247 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            247 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_ChargeLoopReqType)
                                .ManufacturerSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeLoopReqType)
                                                .ManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeLoopReqType)
                                                .ManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen = ((*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_ChargeLoopReqType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_ChargeLoopReqType)
                                        .set_ManufacturerSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 247 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
unsafe extern "C" fn decode_iso20_wpt_WPT_ChargeLoopResType(
    stream: &mut ExiBitstream,
    mut WPT_ChargeLoopResType: *mut iso20_wpt_WPT_ChargeLoopResType,
) -> i32 {
    let mut grammar_id: i32 = 248 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_WPT_ChargeLoopResType(WPT_ChargeLoopResType);
    while done == 0 {
        match grammar_id {
            248 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_MessageHeaderType(
                                stream,
                                &mut (*WPT_ChargeLoopResType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 249 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            249 => {
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
                                        (*WPT_ChargeLoopResType).ResponseCode =
                                            value as iso20_wpt_responseCodeType;
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
                                        grammar_id = 250 as i32;
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
            250 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_EVSEStatusType(
                                stream,
                                &mut (*WPT_ChargeLoopResType).EVSEStatus,
                            );
                            if error == 0 as i32 {
                                (*WPT_ChargeLoopResType).set_EVSEStatus_isUsed(1 as u32);
                                grammar_id = 251 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_wpt_MeterInfoType(
                                stream,
                                &mut (*WPT_ChargeLoopResType).MeterInfo,
                            );
                            if error == 0 as i32 {
                                (*WPT_ChargeLoopResType).set_MeterInfo_isUsed(1 as u32);
                                grammar_id = 252 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_wpt_ReceiptType(
                                stream,
                                &mut (*WPT_ChargeLoopResType).Receipt,
                            );
                            if error == 0 as i32 {
                                (*WPT_ChargeLoopResType).set_Receipt_isUsed(1 as u32);
                                grammar_id = 253 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_ChargeLoopResType).EVPCPowerRequest,
                            );
                            if error == 0 as i32 {
                                grammar_id = 254 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            251 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_MeterInfoType(
                                stream,
                                &mut (*WPT_ChargeLoopResType).MeterInfo,
                            );
                            if error == 0 as i32 {
                                (*WPT_ChargeLoopResType).set_MeterInfo_isUsed(1 as u32);
                                grammar_id = 252 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_wpt_ReceiptType(
                                stream,
                                &mut (*WPT_ChargeLoopResType).Receipt,
                            );
                            if error == 0 as i32 {
                                (*WPT_ChargeLoopResType).set_Receipt_isUsed(1 as u32);
                                grammar_id = 253 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_ChargeLoopResType).EVPCPowerRequest,
                            );
                            if error == 0 as i32 {
                                grammar_id = 254 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            252 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_ReceiptType(
                                stream,
                                &mut (*WPT_ChargeLoopResType).Receipt,
                            );
                            if error == 0 as i32 {
                                (*WPT_ChargeLoopResType).set_Receipt_isUsed(1 as u32);
                                grammar_id = 253 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_ChargeLoopResType).EVPCPowerRequest,
                            );
                            if error == 0 as i32 {
                                grammar_id = 254 as i32;
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
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_ChargeLoopResType).EVPCPowerRequest,
                            );
                            if error == 0 as i32 {
                                grammar_id = 254 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            254 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_ChargeLoopResType).SDPowerInput,
                            );
                            if error == 0 as i32 {
                                (*WPT_ChargeLoopResType).set_SDPowerInput_isUsed(1 as u32);
                                grammar_id = 255 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_ChargeLoopResType).SPCMaxOutputPowerLimit,
                            );
                            if error == 0 as i32 {
                                grammar_id = 256 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            255 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_ChargeLoopResType).SPCMaxOutputPowerLimit,
                            );
                            if error == 0 as i32 {
                                grammar_id = 256 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            256 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_ChargeLoopResType).SPCMinOutputPowerLimit,
                            );
                            if error == 0 as i32 {
                                grammar_id = 257 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            257 => {
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
                                        3 as i32 as usize,
                                        &mut value_0,
                                    );
                                    if error == 0 as i32 {
                                        (*WPT_ChargeLoopResType).SPCChargeDiagnostics =
                                            value_0 as iso20_wpt_WPT_SPCChargeDiagnosticsType;
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
                                        grammar_id = 258 as i32;
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
            258 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_RationalNumberType(
                                stream,
                                &mut (*WPT_ChargeLoopResType).SPCOperatingFrequency,
                            );
                            if error == 0 as i32 {
                                (*WPT_ChargeLoopResType).set_SPCOperatingFrequency_isUsed(1 as u32);
                                grammar_id = 260 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_wpt_WPT_SPCPowerControlParameterType(
                                stream,
                                &mut (*WPT_ChargeLoopResType).SPCPowerControlParameter,
                            );
                            if error == 0 as i32 {
                                (*WPT_ChargeLoopResType)
                                    .set_SPCPowerControlParameter_isUsed(1 as u32);
                                grammar_id = 262 as i32;
                            }
                        }
                        2 => {
                            if ((*WPT_ChargeLoopResType)
                                .ManufacturerSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeLoopResType)
                                                .ManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeLoopResType)
                                                .ManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen = ((*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_ChargeLoopResType)
                                        .set_ManufacturerSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 259 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            259 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_ChargeLoopResType)
                                .ManufacturerSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeLoopResType)
                                                .ManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeLoopResType)
                                                .ManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen = ((*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_ChargeLoopResType)
                                        .set_ManufacturerSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 259 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            260 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_WPT_SPCPowerControlParameterType(
                                stream,
                                &mut (*WPT_ChargeLoopResType).SPCPowerControlParameter,
                            );
                            if error == 0 as i32 {
                                (*WPT_ChargeLoopResType)
                                    .set_SPCPowerControlParameter_isUsed(1 as u32);
                                grammar_id = 262 as i32;
                            }
                        }
                        1 => {
                            if ((*WPT_ChargeLoopResType)
                                .ManufacturerSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeLoopResType)
                                                .ManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeLoopResType)
                                                .ManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen = ((*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_ChargeLoopResType)
                                        .set_ManufacturerSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 261 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            261 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_ChargeLoopResType)
                                .ManufacturerSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeLoopResType)
                                                .ManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeLoopResType)
                                                .ManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen = ((*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_ChargeLoopResType)
                                        .set_ManufacturerSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 261 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            262 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_ChargeLoopResType)
                                .ManufacturerSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeLoopResType)
                                                .ManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeLoopResType)
                                                .ManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen = ((*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_ChargeLoopResType)
                                        .set_ManufacturerSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 263 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
            263 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*WPT_ChargeLoopResType)
                                .ManufacturerSpecificDataContainer
                                .arrayLen as i32)
                                < 16 as i32
                            {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeLoopResType)
                                                .ManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*WPT_ChargeLoopResType)
                                                .ManufacturerSpecificDataContainer
                                                .arrayLen
                                                as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    256 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen = ((*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen)
                                        .wrapping_add(1);
                                    (*WPT_ChargeLoopResType)
                                        .ManufacturerSpecificDataContainer
                                        .arrayLen;
                                    (*WPT_ChargeLoopResType)
                                        .set_ManufacturerSpecificDataContainer_isUsed(1 as u32);
                                    grammar_id = 263 as i32;
                                }
                            } else {
                                error = -(110 as i32);
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
unsafe extern "C" fn decode_iso20_wpt_CLReqControlModeType(
    stream: &mut ExiBitstream,
    mut CLReqControlModeType: *mut iso20_wpt_CLReqControlModeType,
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
unsafe extern "C" fn decode_iso20_wpt_CLResControlModeType(
    stream: &mut ExiBitstream,
    mut CLResControlModeType: *mut iso20_wpt_CLResControlModeType,
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
unsafe extern "C" fn decode_iso20_wpt_ManifestType(
    stream: &mut ExiBitstream,
    mut ManifestType: *mut iso20_wpt_ManifestType,
) -> i32 {
    let mut grammar_id: i32 = 264 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_ManifestType(ManifestType);
    while done == 0 {
        match grammar_id {
            264 => {
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
                            grammar_id = 266 as i32;
                        }
                        1 => {
                            if ((*ManifestType).Reference.arrayLen as i32) < 4 as i32 {
                                let fresh20 = (*ManifestType).Reference.arrayLen;
                                (*ManifestType).Reference.arrayLen =
                                    ((*ManifestType).Reference.arrayLen).wrapping_add(1);
                                error = decode_iso20_wpt_ReferenceType(
                                    stream,
                                    &mut *((*ManifestType).Reference.array)
                                        .as_mut_ptr()
                                        .offset(fresh20 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 265 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            265 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ManifestType).Reference.arrayLen as i32) < 4 as i32 {
                                let fresh21 = (*ManifestType).Reference.arrayLen;
                                (*ManifestType).Reference.arrayLen =
                                    ((*ManifestType).Reference.arrayLen).wrapping_add(1);
                                error = decode_iso20_wpt_ReferenceType(
                                    stream,
                                    &mut *((*ManifestType).Reference.array)
                                        .as_mut_ptr()
                                        .offset(fresh21 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 265 as i32;
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
            266 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ManifestType).Reference.arrayLen as i32) < 4 as i32 {
                                let fresh22 = (*ManifestType).Reference.arrayLen;
                                (*ManifestType).Reference.arrayLen =
                                    ((*ManifestType).Reference.arrayLen).wrapping_add(1);
                                error = decode_iso20_wpt_ReferenceType(
                                    stream,
                                    &mut *((*ManifestType).Reference.array)
                                        .as_mut_ptr()
                                        .offset(fresh22 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 267 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            267 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ManifestType).Reference.arrayLen as i32) < 4 as i32 {
                                let fresh23 = (*ManifestType).Reference.arrayLen;
                                (*ManifestType).Reference.arrayLen =
                                    ((*ManifestType).Reference.arrayLen).wrapping_add(1);
                                error = decode_iso20_wpt_ReferenceType(
                                    stream,
                                    &mut *((*ManifestType).Reference.array)
                                        .as_mut_ptr()
                                        .offset(fresh23 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 267 as i32;
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
unsafe extern "C" fn decode_iso20_wpt_SignaturePropertiesType(
    stream: &mut ExiBitstream,
    mut SignaturePropertiesType: *mut iso20_wpt_SignaturePropertiesType,
) -> i32 {
    let mut grammar_id: i32 = 268 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_wpt_SignaturePropertiesType(SignaturePropertiesType);
    while done == 0 {
        match grammar_id {
            268 => {
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
                            grammar_id = 270 as i32;
                        }
                        1 => {
                            error = decode_iso20_wpt_SignaturePropertyType(
                                stream,
                                &mut (*SignaturePropertiesType).SignatureProperty,
                            );
                            if error == 0 as i32 {
                                grammar_id = 269 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            269 => {
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
            270 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_wpt_SignaturePropertyType(
                                stream,
                                &mut (*SignaturePropertiesType).SignatureProperty,
                            );
                            if error == 0 as i32 {
                                grammar_id = 271 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            271 => {
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

pub unsafe extern "C" fn decode_iso20_wpt_exiDocument(
    stream: &mut ExiBitstream,
    mut exiDoc: *mut iso20_wpt_exiDocument,
) -> i32 {
    let mut eventCode: u32 = 0;
    let mut error: i32 = exi_header_read_and_check(stream);
    if error == 0 as i32 {
        init_iso20_wpt_exiDocument(exiDoc);
        error = exi_basetypes_decoder_nbit_uint(stream, 6 as i32 as usize, &mut eventCode);
        if error == 0 as i32 {
            match eventCode {
                0 => {
                    error = decode_iso20_wpt_CLReqControlModeType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.CLReqControlMode,
                    );
                    (*exiDoc).set_CLReqControlMode_isUsed(1 as u32);
                }
                1 => {
                    error = decode_iso20_wpt_CLResControlModeType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.CLResControlMode,
                    );
                    (*exiDoc).set_CLResControlMode_isUsed(1 as u32);
                }
                2 => {
                    error = decode_iso20_wpt_CanonicalizationMethodType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.CanonicalizationMethod,
                    );
                    (*exiDoc).set_CanonicalizationMethod_isUsed(1 as u32);
                }
                3 => {
                    error = decode_iso20_wpt_DSAKeyValueType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.DSAKeyValue,
                    );
                    (*exiDoc).set_DSAKeyValue_isUsed(1 as u32);
                }
                4 => {
                    error = decode_iso20_wpt_DigestMethodType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.DigestMethod,
                    );
                    (*exiDoc).set_DigestMethod_isUsed(1 as u32);
                }
                5 => {}
                6 => {
                    error =
                        decode_iso20_wpt_KeyInfoType(stream, &mut (*exiDoc).c2rust_unnamed.KeyInfo);
                    (*exiDoc).set_KeyInfo_isUsed(1 as u32);
                }
                7 => {}
                8 => {
                    error = decode_iso20_wpt_KeyValueType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.KeyValue,
                    );
                    (*exiDoc).set_KeyValue_isUsed(1 as u32);
                }
                9 => {
                    error = decode_iso20_wpt_ManifestType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.Manifest,
                    );
                    (*exiDoc).set_Manifest_isUsed(1 as u32);
                }
                10 => {}
                11 => {
                    error =
                        decode_iso20_wpt_ObjectType(stream, &mut (*exiDoc).c2rust_unnamed.Object);
                    (*exiDoc).set_Object_isUsed(1 as u32);
                }
                12 => {
                    error =
                        decode_iso20_wpt_PGPDataType(stream, &mut (*exiDoc).c2rust_unnamed.PGPData);
                    (*exiDoc).set_PGPData_isUsed(1 as u32);
                }
                13 => {
                    error = decode_iso20_wpt_RSAKeyValueType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.RSAKeyValue,
                    );
                    (*exiDoc).set_RSAKeyValue_isUsed(1 as u32);
                }
                14 => {
                    error = decode_iso20_wpt_ReferenceType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.Reference,
                    );
                    (*exiDoc).set_Reference_isUsed(1 as u32);
                }
                15 => {
                    error = decode_iso20_wpt_RetrievalMethodType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.RetrievalMethod,
                    );
                    (*exiDoc).set_RetrievalMethod_isUsed(1 as u32);
                }
                16 => {
                    error = decode_iso20_wpt_SPKIDataType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.SPKIData,
                    );
                    (*exiDoc).set_SPKIData_isUsed(1 as u32);
                }
                17 => {
                    error = decode_iso20_wpt_SignatureMethodType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.SignatureMethod,
                    );
                    (*exiDoc).set_SignatureMethod_isUsed(1 as u32);
                }
                18 => {
                    error = decode_iso20_wpt_SignaturePropertiesType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.SignatureProperties,
                    );
                    (*exiDoc).set_SignatureProperties_isUsed(1 as u32);
                }
                19 => {
                    error = decode_iso20_wpt_SignaturePropertyType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.SignatureProperty,
                    );
                    (*exiDoc).set_SignatureProperty_isUsed(1 as u32);
                }
                20 => {
                    error = decode_iso20_wpt_SignatureType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.Signature,
                    );
                    (*exiDoc).set_Signature_isUsed(1 as u32);
                }
                21 => {
                    error = decode_iso20_wpt_SignatureValueType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.SignatureValue,
                    );
                    (*exiDoc).set_SignatureValue_isUsed(1 as u32);
                }
                22 => {
                    error = decode_iso20_wpt_SignedInfoType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.SignedInfo,
                    );
                    (*exiDoc).set_SignedInfo_isUsed(1 as u32);
                }
                23 => {
                    error = decode_iso20_wpt_TransformType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.Transform,
                    );
                    (*exiDoc).set_Transform_isUsed(1 as u32);
                }
                24 => {
                    error = decode_iso20_wpt_TransformsType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.Transforms,
                    );
                    (*exiDoc).set_Transforms_isUsed(1 as u32);
                }
                25 => {
                    error = decode_iso20_wpt_WPT_AlignmentCheckReqType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.WPT_AlignmentCheckReq,
                    );
                    (*exiDoc).set_WPT_AlignmentCheckReq_isUsed(1 as u32);
                }
                26 => {
                    error = decode_iso20_wpt_WPT_AlignmentCheckResType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.WPT_AlignmentCheckRes,
                    );
                    (*exiDoc).set_WPT_AlignmentCheckRes_isUsed(1 as u32);
                }
                27 => {
                    error = decode_iso20_wpt_WPT_ChargeLoopReqType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.WPT_ChargeLoopReq,
                    );
                    (*exiDoc).set_WPT_ChargeLoopReq_isUsed(1 as u32);
                }
                28 => {
                    error = decode_iso20_wpt_WPT_ChargeLoopResType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.WPT_ChargeLoopRes,
                    );
                    (*exiDoc).set_WPT_ChargeLoopRes_isUsed(1 as u32);
                }
                29 => {
                    error = decode_iso20_wpt_WPT_ChargeParameterDiscoveryReqType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.WPT_ChargeParameterDiscoveryReq,
                    );
                    (*exiDoc).set_WPT_ChargeParameterDiscoveryReq_isUsed(1 as u32);
                }
                30 => {
                    error = decode_iso20_wpt_WPT_ChargeParameterDiscoveryResType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.WPT_ChargeParameterDiscoveryRes,
                    );
                    (*exiDoc).set_WPT_ChargeParameterDiscoveryRes_isUsed(1 as u32);
                }
                31 => {
                    error = decode_iso20_wpt_WPT_FinePositioningReqType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.WPT_FinePositioningReq,
                    );
                    (*exiDoc).set_WPT_FinePositioningReq_isUsed(1 as u32);
                }
                32 => {
                    error = decode_iso20_wpt_WPT_FinePositioningResType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.WPT_FinePositioningRes,
                    );
                    (*exiDoc).set_WPT_FinePositioningRes_isUsed(1 as u32);
                }
                33 => {
                    error = decode_iso20_wpt_WPT_FinePositioningSetupReqType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.WPT_FinePositioningSetupReq,
                    );
                    (*exiDoc).set_WPT_FinePositioningSetupReq_isUsed(1 as u32);
                }
                34 => {
                    error = decode_iso20_wpt_WPT_FinePositioningSetupResType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.WPT_FinePositioningSetupRes,
                    );
                    (*exiDoc).set_WPT_FinePositioningSetupRes_isUsed(1 as u32);
                }
                35 => {
                    error = decode_iso20_wpt_WPT_PairingReqType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.WPT_PairingReq,
                    );
                    (*exiDoc).set_WPT_PairingReq_isUsed(1 as u32);
                }
                36 => {
                    error = decode_iso20_wpt_WPT_PairingResType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.WPT_PairingRes,
                    );
                    (*exiDoc).set_WPT_PairingRes_isUsed(1 as u32);
                }
                37 => {
                    error = decode_iso20_wpt_X509DataType(
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
