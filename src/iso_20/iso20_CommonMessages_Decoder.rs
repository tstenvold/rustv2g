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
    fn decode_exi_type_integer32(stream: *mut exi_bitstream_t, value: *mut i32) -> i32;
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
    fn init_iso20_exiDocument(exiDoc: *mut iso20_exiDocument);
    fn init_iso20_SessionSetupReqType(SessionSetupReq: *mut iso20_SessionSetupReqType);
    fn init_iso20_SessionSetupResType(SessionSetupRes: *mut iso20_SessionSetupResType);
    fn init_iso20_AuthorizationSetupReqType(
        AuthorizationSetupReq: *mut iso20_AuthorizationSetupReqType,
    );
    fn init_iso20_AuthorizationSetupResType(
        AuthorizationSetupRes: *mut iso20_AuthorizationSetupResType,
    );
    fn init_iso20_AuthorizationReqType(AuthorizationReq: *mut iso20_AuthorizationReqType);
    fn init_iso20_AuthorizationResType(AuthorizationRes: *mut iso20_AuthorizationResType);
    fn init_iso20_ServiceDiscoveryReqType(ServiceDiscoveryReq: *mut iso20_ServiceDiscoveryReqType);
    fn init_iso20_ServiceDiscoveryResType(ServiceDiscoveryRes: *mut iso20_ServiceDiscoveryResType);
    fn init_iso20_ServiceDetailReqType(ServiceDetailReq: *mut iso20_ServiceDetailReqType);
    fn init_iso20_ServiceDetailResType(ServiceDetailRes: *mut iso20_ServiceDetailResType);
    fn init_iso20_ServiceSelectionReqType(ServiceSelectionReq: *mut iso20_ServiceSelectionReqType);
    fn init_iso20_ServiceSelectionResType(ServiceSelectionRes: *mut iso20_ServiceSelectionResType);
    fn init_iso20_ScheduleExchangeReqType(ScheduleExchangeReq: *mut iso20_ScheduleExchangeReqType);
    fn init_iso20_ScheduleExchangeResType(ScheduleExchangeRes: *mut iso20_ScheduleExchangeResType);
    fn init_iso20_PowerDeliveryReqType(PowerDeliveryReq: *mut iso20_PowerDeliveryReqType);
    fn init_iso20_PowerDeliveryResType(PowerDeliveryRes: *mut iso20_PowerDeliveryResType);
    fn init_iso20_MeteringConfirmationReqType(
        MeteringConfirmationReq: *mut iso20_MeteringConfirmationReqType,
    );
    fn init_iso20_MeteringConfirmationResType(
        MeteringConfirmationRes: *mut iso20_MeteringConfirmationResType,
    );
    fn init_iso20_SessionStopReqType(SessionStopReq: *mut iso20_SessionStopReqType);
    fn init_iso20_SessionStopResType(SessionStopRes: *mut iso20_SessionStopResType);
    fn init_iso20_CertificateInstallationReqType(
        CertificateInstallationReq: *mut iso20_CertificateInstallationReqType,
    );
    fn init_iso20_CertificateInstallationResType(
        CertificateInstallationRes: *mut iso20_CertificateInstallationResType,
    );
    fn init_iso20_VehicleCheckInReqType(VehicleCheckInReq: *mut iso20_VehicleCheckInReqType);
    fn init_iso20_VehicleCheckInResType(VehicleCheckInRes: *mut iso20_VehicleCheckInResType);
    fn init_iso20_VehicleCheckOutReqType(VehicleCheckOutReq: *mut iso20_VehicleCheckOutReqType);
    fn init_iso20_VehicleCheckOutResType(VehicleCheckOutRes: *mut iso20_VehicleCheckOutResType);
    fn init_iso20_SignedInstallationDataType(
        SignedInstallationData: *mut iso20_SignedInstallationDataType,
    );
    fn init_iso20_SignedMeteringDataType(SignedMeteringData: *mut iso20_SignedMeteringDataType);
    fn init_iso20_SignatureType(Signature: *mut iso20_SignatureType);
    fn init_iso20_SignatureValueType(SignatureValue: *mut iso20_SignatureValueType);
    fn init_iso20_SignedInfoType(SignedInfo: *mut iso20_SignedInfoType);
    fn init_iso20_CanonicalizationMethodType(
        CanonicalizationMethod: *mut iso20_CanonicalizationMethodType,
    );
    fn init_iso20_SignatureMethodType(SignatureMethod: *mut iso20_SignatureMethodType);
    fn init_iso20_ReferenceType(Reference: *mut iso20_ReferenceType);
    fn init_iso20_TransformsType(Transforms: *mut iso20_TransformsType);
    fn init_iso20_TransformType(Transform: *mut iso20_TransformType);
    fn init_iso20_DigestMethodType(DigestMethod: *mut iso20_DigestMethodType);
    fn init_iso20_KeyInfoType(KeyInfo: *mut iso20_KeyInfoType);
    fn init_iso20_KeyValueType(KeyValue: *mut iso20_KeyValueType);
    fn init_iso20_RetrievalMethodType(RetrievalMethod: *mut iso20_RetrievalMethodType);
    fn init_iso20_X509DataType(X509Data: *mut iso20_X509DataType);
    fn init_iso20_PGPDataType(PGPData: *mut iso20_PGPDataType);
    fn init_iso20_SPKIDataType(SPKIData: *mut iso20_SPKIDataType);
    fn init_iso20_ObjectType(Object: *mut iso20_ObjectType);
    fn init_iso20_ManifestType(Manifest: *mut iso20_ManifestType);
    fn init_iso20_SignaturePropertiesType(SignatureProperties: *mut iso20_SignaturePropertiesType);
    fn init_iso20_SignaturePropertyType(SignatureProperty: *mut iso20_SignaturePropertyType);
    fn init_iso20_DSAKeyValueType(DSAKeyValue: *mut iso20_DSAKeyValueType);
    fn init_iso20_RSAKeyValueType(RSAKeyValue: *mut iso20_RSAKeyValueType);
    fn init_iso20_PowerScheduleEntryType(PowerScheduleEntryType: *mut iso20_PowerScheduleEntryType);
    fn init_iso20_EVPriceRuleType(EVPriceRuleType: *mut iso20_EVPriceRuleType);
    fn init_iso20_X509IssuerSerialType(X509IssuerSerialType: *mut iso20_X509IssuerSerialType);
    fn init_iso20_EVPowerScheduleEntryType(
        EVPowerScheduleEntryType: *mut iso20_EVPowerScheduleEntryType,
    );
    fn init_iso20_EVPriceRuleStackType(EVPriceRuleStackType: *mut iso20_EVPriceRuleStackType);
    fn init_iso20_PriceRuleType(PriceRuleType: *mut iso20_PriceRuleType);
    fn init_iso20_PowerScheduleEntryListType(
        PowerScheduleEntryListType: *mut iso20_PowerScheduleEntryListType,
    );
    fn init_iso20_TaxRuleType(TaxRuleType: *mut iso20_TaxRuleType);
    fn init_iso20_PriceRuleStackType(PriceRuleStackType: *mut iso20_PriceRuleStackType);
    fn init_iso20_AdditionalServiceType(AdditionalServiceType: *mut iso20_AdditionalServiceType);
    fn init_iso20_PriceLevelScheduleEntryType(
        PriceLevelScheduleEntryType: *mut iso20_PriceLevelScheduleEntryType,
    );
    fn init_iso20_PowerScheduleType(PowerScheduleType: *mut iso20_PowerScheduleType);
    fn init_iso20_EVPowerScheduleEntryListType(
        EVPowerScheduleEntryListType: *mut iso20_EVPowerScheduleEntryListType,
    );
    fn init_iso20_OverstayRuleType(OverstayRuleType: *mut iso20_OverstayRuleType);
    fn init_iso20_EVPriceRuleStackListType(
        EVPriceRuleStackListType: *mut iso20_EVPriceRuleStackListType,
    );
    fn init_iso20_RationalNumberType(RationalNumberType: *mut iso20_RationalNumberType);
    fn init_iso20_EVPowerScheduleType(EVPowerScheduleType: *mut iso20_EVPowerScheduleType);
    fn init_iso20_SubCertificatesType(SubCertificatesType: *mut iso20_SubCertificatesType);
    fn init_iso20_ParameterType(ParameterType: *mut iso20_ParameterType);
    fn init_iso20_EVAbsolutePriceScheduleType(
        EVAbsolutePriceScheduleType: *mut iso20_EVAbsolutePriceScheduleType,
    );
    fn init_iso20_ChargingScheduleType(ChargingScheduleType: *mut iso20_ChargingScheduleType);
    fn init_iso20_DetailedCostType(DetailedCostType: *mut iso20_DetailedCostType);
    fn init_iso20_PriceLevelScheduleEntryListType(
        PriceLevelScheduleEntryListType: *mut iso20_PriceLevelScheduleEntryListType,
    );
    fn init_iso20_DetailedTaxType(DetailedTaxType: *mut iso20_DetailedTaxType);
    fn init_iso20_TaxRuleListType(TaxRuleListType: *mut iso20_TaxRuleListType);
    fn init_iso20_PriceRuleStackListType(PriceRuleStackListType: *mut iso20_PriceRuleStackListType);
    fn init_iso20_OverstayRuleListType(OverstayRuleListType: *mut iso20_OverstayRuleListType);
    fn init_iso20_AdditionalServiceListType(
        AdditionalServiceListType: *mut iso20_AdditionalServiceListType,
    );
    fn init_iso20_ServiceType(ServiceType: *mut iso20_ServiceType);
    fn init_iso20_ParameterSetType(ParameterSetType: *mut iso20_ParameterSetType);
    fn init_iso20_ScheduleTupleType(ScheduleTupleType: *mut iso20_ScheduleTupleType);
    fn init_iso20_SupportedProvidersListType(
        SupportedProvidersListType: *mut iso20_SupportedProvidersListType,
    );
    fn init_iso20_ContractCertificateChainType(
        ContractCertificateChainType: *mut iso20_ContractCertificateChainType,
    );
    fn init_iso20_MeterInfoType(MeterInfoType: *mut iso20_MeterInfoType);
    fn init_iso20_Scheduled_EVPPTControlModeType(
        Scheduled_EVPPTControlModeType: *mut iso20_Scheduled_EVPPTControlModeType,
    );
    fn init_iso20_ReceiptType(ReceiptType: *mut iso20_ReceiptType);
    fn init_iso20_AbsolutePriceScheduleType(
        AbsolutePriceScheduleType: *mut iso20_AbsolutePriceScheduleType,
    );
    fn init_iso20_EVPowerProfileEntryListType(
        EVPowerProfileEntryListType: *mut iso20_EVPowerProfileEntryListType,
    );
    fn init_iso20_EVEnergyOfferType(EVEnergyOfferType: *mut iso20_EVEnergyOfferType);
    fn init_iso20_PriceLevelScheduleType(PriceLevelScheduleType: *mut iso20_PriceLevelScheduleType);
    fn init_iso20_Scheduled_SMDTControlModeType(
        Scheduled_SMDTControlModeType: *mut iso20_Scheduled_SMDTControlModeType,
    );
    fn init_iso20_MessageHeaderType(MessageHeaderType: *mut iso20_MessageHeaderType);
    fn init_iso20_ServiceIDListType(ServiceIDListType: *mut iso20_ServiceIDListType);
    fn init_iso20_SelectedServiceType(SelectedServiceType: *mut iso20_SelectedServiceType);
    fn init_iso20_SignedCertificateChainType(
        SignedCertificateChainType: *mut iso20_SignedCertificateChainType,
    );
    fn init_iso20_SelectedServiceListType(
        SelectedServiceListType: *mut iso20_SelectedServiceListType,
    );
    fn init_iso20_Dynamic_SEReqControlModeType(
        Dynamic_SEReqControlModeType: *mut iso20_Dynamic_SEReqControlModeType,
    );
    fn init_iso20_EVSEStatusType(EVSEStatusType: *mut iso20_EVSEStatusType);
    fn init_iso20_ListOfRootCertificateIDsType(
        ListOfRootCertificateIDsType: *mut iso20_ListOfRootCertificateIDsType,
    );
    fn init_iso20_PnC_AReqAuthorizationModeType(
        PnC_AReqAuthorizationModeType: *mut iso20_PnC_AReqAuthorizationModeType,
    );
    fn init_iso20_ServiceListType(ServiceListType: *mut iso20_ServiceListType);
    fn init_iso20_ServiceParameterListType(
        ServiceParameterListType: *mut iso20_ServiceParameterListType,
    );
    fn init_iso20_Scheduled_SEReqControlModeType(
        Scheduled_SEReqControlModeType: *mut iso20_Scheduled_SEReqControlModeType,
    );
    fn init_iso20_EVPowerProfileType(EVPowerProfileType: *mut iso20_EVPowerProfileType);
    fn init_iso20_CertificateChainType(CertificateChainType: *mut iso20_CertificateChainType);
    fn init_iso20_Dynamic_SEResControlModeType(
        Dynamic_SEResControlModeType: *mut iso20_Dynamic_SEResControlModeType,
    );
    fn init_iso20_EMAIDListType(EMAIDListType: *mut iso20_EMAIDListType);
    fn init_iso20_PnC_ASResAuthorizationModeType(
        PnC_ASResAuthorizationModeType: *mut iso20_PnC_ASResAuthorizationModeType,
    );
    fn init_iso20_Scheduled_SEResControlModeType(
        Scheduled_SEResControlModeType: *mut iso20_Scheduled_SEResControlModeType,
    );
    fn init_iso20_exiFragment(exiFrag: *mut iso20_exiFragment);
    fn init_iso20_xmldsigFragment(xmldsigFrag: *mut iso20_xmldsigFragment);
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
    _unused: i32,
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
    _unused: i32,
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
    _unused: i32,
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
    _unused: i32,
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
    _unused: i32,
}
#[derive(Copy, Clone)]

pub struct iso20_CLResControlModeType {
    _unused: i32,
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
unsafe extern "C" fn decode_iso20_TransformType(
    stream: &mut ExiBitstream,
    mut TransformType: *mut iso20_TransformType,
) -> i32 {
    let mut grammar_id: i32 = 0 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_TransformType(TransformType);
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
unsafe extern "C" fn decode_iso20_TransformsType(
    stream: &mut ExiBitstream,
    mut TransformsType: *mut iso20_TransformsType,
) -> i32 {
    let mut grammar_id: i32 = 4 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_TransformsType(TransformsType);
    while done == 0 {
        match grammar_id {
            4 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_TransformType(
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
unsafe extern "C" fn decode_iso20_DSAKeyValueType(
    stream: &mut ExiBitstream,
    mut DSAKeyValueType: *mut iso20_DSAKeyValueType,
) -> i32 {
    let mut grammar_id: i32 = 6 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_DSAKeyValueType(DSAKeyValueType);
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
unsafe extern "C" fn decode_iso20_X509IssuerSerialType(
    stream: &mut ExiBitstream,
    mut X509IssuerSerialType: *mut iso20_X509IssuerSerialType,
) -> i32 {
    let mut grammar_id: i32 = 13 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_X509IssuerSerialType(X509IssuerSerialType);
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
unsafe extern "C" fn decode_iso20_DigestMethodType(
    stream: &mut ExiBitstream,
    mut DigestMethodType: *mut iso20_DigestMethodType,
) -> i32 {
    let mut grammar_id: i32 = 15 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_DigestMethodType(DigestMethodType);
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
unsafe extern "C" fn decode_iso20_RSAKeyValueType(
    stream: &mut ExiBitstream,
    mut RSAKeyValueType: *mut iso20_RSAKeyValueType,
) -> i32 {
    let mut grammar_id: i32 = 17 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_RSAKeyValueType(RSAKeyValueType);
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
unsafe extern "C" fn decode_iso20_CanonicalizationMethodType(
    stream: &mut ExiBitstream,
    mut CanonicalizationMethodType: *mut iso20_CanonicalizationMethodType,
) -> i32 {
    let mut grammar_id: i32 = 19 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_CanonicalizationMethodType(CanonicalizationMethodType);
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
unsafe extern "C" fn decode_iso20_PriceLevelScheduleEntryType(
    stream: &mut ExiBitstream,
    mut PriceLevelScheduleEntryType: *mut iso20_PriceLevelScheduleEntryType,
) -> i32 {
    let mut grammar_id: i32 = 21 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_PriceLevelScheduleEntryType(PriceLevelScheduleEntryType);
    while done == 0 {
        match grammar_id {
            21 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*PriceLevelScheduleEntryType).Duration,
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
                                        (*PriceLevelScheduleEntryType).PriceLevel = value as u8;
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
unsafe extern "C" fn decode_iso20_SignatureMethodType(
    stream: &mut ExiBitstream,
    mut SignatureMethodType: *mut iso20_SignatureMethodType,
) -> i32 {
    let mut grammar_id: i32 = 23 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_SignatureMethodType(SignatureMethodType);
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
unsafe extern "C" fn decode_iso20_KeyValueType(
    stream: &mut ExiBitstream,
    mut KeyValueType: *mut iso20_KeyValueType,
) -> i32 {
    let mut grammar_id: i32 = 26 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_KeyValueType(KeyValueType);
    while done == 0 {
        match grammar_id {
            26 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_DSAKeyValueType(
                                stream,
                                &mut (*KeyValueType).DSAKeyValue,
                            );
                            if error == 0 as i32 {
                                (*KeyValueType).set_DSAKeyValue_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_RSAKeyValueType(
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
unsafe extern "C" fn decode_iso20_ReferenceType(
    stream: &mut ExiBitstream,
    mut ReferenceType: *mut iso20_ReferenceType,
) -> i32 {
    let mut grammar_id: i32 = 27 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_ReferenceType(ReferenceType);
    while done == 0 {
        match grammar_id {
            27 => {
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
                            grammar_id = 28 as i32;
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
                            grammar_id = 29 as i32;
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
                            grammar_id = 30 as i32;
                        }
                        3 => {
                            error = decode_iso20_TransformsType(
                                stream,
                                &mut (*ReferenceType).Transforms,
                            );
                            if error == 0 as i32 {
                                (*ReferenceType).set_Transforms_isUsed(1 as u32);
                                grammar_id = 31 as i32;
                            }
                        }
                        4 => {
                            error = decode_iso20_DigestMethodType(
                                stream,
                                &mut (*ReferenceType).DigestMethod,
                            );
                            if error == 0 as i32 {
                                grammar_id = 32 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            28 => {
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
                            grammar_id = 29 as i32;
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
                            grammar_id = 30 as i32;
                        }
                        2 => {
                            error = decode_iso20_TransformsType(
                                stream,
                                &mut (*ReferenceType).Transforms,
                            );
                            if error == 0 as i32 {
                                (*ReferenceType).set_Transforms_isUsed(1 as u32);
                                grammar_id = 31 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_DigestMethodType(
                                stream,
                                &mut (*ReferenceType).DigestMethod,
                            );
                            if error == 0 as i32 {
                                grammar_id = 32 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            29 => {
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
                            grammar_id = 30 as i32;
                        }
                        1 => {
                            error = decode_iso20_TransformsType(
                                stream,
                                &mut (*ReferenceType).Transforms,
                            );
                            if error == 0 as i32 {
                                (*ReferenceType).set_Transforms_isUsed(1 as u32);
                                grammar_id = 31 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_DigestMethodType(
                                stream,
                                &mut (*ReferenceType).DigestMethod,
                            );
                            if error == 0 as i32 {
                                grammar_id = 32 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            30 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_TransformsType(
                                stream,
                                &mut (*ReferenceType).Transforms,
                            );
                            if error == 0 as i32 {
                                (*ReferenceType).set_Transforms_isUsed(1 as u32);
                                grammar_id = 31 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_DigestMethodType(
                                stream,
                                &mut (*ReferenceType).DigestMethod,
                            );
                            if error == 0 as i32 {
                                grammar_id = 32 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            31 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_DigestMethodType(
                                stream,
                                &mut (*ReferenceType).DigestMethod,
                            );
                            if error == 0 as i32 {
                                grammar_id = 32 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            32 => {
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
unsafe extern "C" fn decode_iso20_RetrievalMethodType(
    stream: &mut ExiBitstream,
    mut RetrievalMethodType: *mut iso20_RetrievalMethodType,
) -> i32 {
    let mut grammar_id: i32 = 33 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_RetrievalMethodType(RetrievalMethodType);
    while done == 0 {
        match grammar_id {
            33 => {
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
                            grammar_id = 34 as i32;
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
                            grammar_id = 35 as i32;
                        }
                        2 => {
                            error = decode_iso20_TransformsType(
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
            34 => {
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
                            grammar_id = 35 as i32;
                        }
                        1 => {
                            error = decode_iso20_TransformsType(
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
            35 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_TransformsType(
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
unsafe extern "C" fn decode_iso20_X509DataType(
    stream: &mut ExiBitstream,
    mut X509DataType: *mut iso20_X509DataType,
) -> i32 {
    let mut grammar_id: i32 = 36 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_X509DataType(X509DataType);
    while done == 0 {
        match grammar_id {
            36 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_X509IssuerSerialType(
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
unsafe extern "C" fn decode_iso20_PGPDataType(
    stream: &mut ExiBitstream,
    mut PGPDataType: *mut iso20_PGPDataType,
) -> i32 {
    let mut grammar_id: i32 = 37 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_PGPDataType(PGPDataType);
    while done == 0 {
        match grammar_id {
            37 => {
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
                                grammar_id = 38 as i32;
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
                                grammar_id = 39 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            38 => {
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
                                grammar_id = 39 as i32;
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
                                grammar_id = 40 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            39 => {
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
                                grammar_id = 40 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            40 => {
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
                                &mut (*PGPDataType).c2rust_unnamed.choice_2.ANY.bytesLen,
                                &mut *((*PGPDataType).c2rust_unnamed.choice_2.ANY.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                4 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                ((*PGPDataType).c2rust_unnamed.choice_2).set_ANY_isUsed(1 as u32);
                                grammar_id = 40 as i32;
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
unsafe extern "C" fn decode_iso20_RationalNumberType(
    stream: &mut ExiBitstream,
    mut RationalNumberType: *mut iso20_RationalNumberType,
) -> i32 {
    let mut grammar_id: i32 = 42 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_RationalNumberType(RationalNumberType);
    while done == 0 {
        match grammar_id {
            42 => {
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
                                        grammar_id = 43 as i32;
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
            43 => {
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
unsafe extern "C" fn decode_iso20_PowerScheduleEntryType(
    stream: &mut ExiBitstream,
    mut PowerScheduleEntryType: *mut iso20_PowerScheduleEntryType,
) -> i32 {
    let mut grammar_id: i32 = 44 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_PowerScheduleEntryType(PowerScheduleEntryType);
    while done == 0 {
        match grammar_id {
            44 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*PowerScheduleEntryType).Duration,
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
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*PowerScheduleEntryType).Power,
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
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*PowerScheduleEntryType).Power_L2,
                            );
                            if error == 0 as i32 {
                                (*PowerScheduleEntryType).set_Power_L2_isUsed(1 as u32);
                                grammar_id = 47 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*PowerScheduleEntryType).Power_L3,
                            );
                            if error == 0 as i32 {
                                (*PowerScheduleEntryType).set_Power_L3_isUsed(1 as u32);
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
            47 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*PowerScheduleEntryType).Power_L3,
                            );
                            if error == 0 as i32 {
                                (*PowerScheduleEntryType).set_Power_L3_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_EVPriceRuleType(
    stream: &mut ExiBitstream,
    mut EVPriceRuleType: *mut iso20_EVPriceRuleType,
) -> i32 {
    let mut grammar_id: i32 = 48 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_EVPriceRuleType(EVPriceRuleType);
    while done == 0 {
        match grammar_id {
            48 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*EVPriceRuleType).EnergyFee,
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
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*EVPriceRuleType).PowerRangeStart,
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
unsafe extern "C" fn decode_iso20_EVPowerScheduleEntryType(
    stream: &mut ExiBitstream,
    mut EVPowerScheduleEntryType: *mut iso20_EVPowerScheduleEntryType,
) -> i32 {
    let mut grammar_id: i32 = 50 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_EVPowerScheduleEntryType(EVPowerScheduleEntryType);
    while done == 0 {
        match grammar_id {
            50 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*EVPowerScheduleEntryType).Duration,
                            );
                            if error == 0 as i32 {
                                grammar_id = 51 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            51 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*EVPowerScheduleEntryType).Power,
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
unsafe extern "C" fn decode_iso20_EVPriceRuleStackType(
    stream: &mut ExiBitstream,
    mut EVPriceRuleStackType: *mut iso20_EVPriceRuleStackType,
) -> i32 {
    let mut grammar_id: i32 = 52 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_EVPriceRuleStackType(EVPriceRuleStackType);
    while done == 0 {
        match grammar_id {
            52 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*EVPriceRuleStackType).Duration,
                            );
                            if error == 0 as i32 {
                                grammar_id = 53 as i32;
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
                            if ((*EVPriceRuleStackType).EVPriceRule.arrayLen as i32) < 8 as i32 {
                                let fresh0 = (*EVPriceRuleStackType).EVPriceRule.arrayLen;
                                (*EVPriceRuleStackType).EVPriceRule.arrayLen =
                                    ((*EVPriceRuleStackType).EVPriceRule.arrayLen).wrapping_add(1);
                                error = decode_iso20_EVPriceRuleType(
                                    stream,
                                    &mut *((*EVPriceRuleStackType).EVPriceRule.array)
                                        .as_mut_ptr()
                                        .offset(fresh0 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 54 as i32;
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
                            if ((*EVPriceRuleStackType).EVPriceRule.arrayLen as i32) < 8 as i32 {
                                let fresh1 = (*EVPriceRuleStackType).EVPriceRule.arrayLen;
                                (*EVPriceRuleStackType).EVPriceRule.arrayLen =
                                    ((*EVPriceRuleStackType).EVPriceRule.arrayLen).wrapping_add(1);
                                error = decode_iso20_EVPriceRuleType(
                                    stream,
                                    &mut *((*EVPriceRuleStackType).EVPriceRule.array)
                                        .as_mut_ptr()
                                        .offset(fresh1 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*EVPriceRuleStackType).EVPriceRule.arrayLen as i32) < 8 as i32 {
                                grammar_id = 54 as i32;
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
unsafe extern "C" fn decode_iso20_PriceRuleType(
    stream: &mut ExiBitstream,
    mut PriceRuleType: *mut iso20_PriceRuleType,
) -> i32 {
    let mut grammar_id: i32 = 55 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_PriceRuleType(PriceRuleType);
    while done == 0 {
        match grammar_id {
            55 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*PriceRuleType).EnergyFee,
                            );
                            if error == 0 as i32 {
                                grammar_id = 56 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            56 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*PriceRuleType).ParkingFee,
                            );
                            if error == 0 as i32 {
                                (*PriceRuleType).set_ParkingFee_isUsed(1 as u32);
                                grammar_id = 57 as i32;
                            }
                        }
                        1 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*PriceRuleType).ParkingFeePeriod,
                            );
                            if error == 0 as i32 {
                                (*PriceRuleType).set_ParkingFeePeriod_isUsed(1 as u32);
                                grammar_id = 58 as i32;
                            }
                        }
                        2 => {
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*PriceRuleType).CarbonDioxideEmission,
                            );
                            if error == 0 as i32 {
                                (*PriceRuleType).set_CarbonDioxideEmission_isUsed(1 as u32);
                                grammar_id = 59 as i32;
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
                                    let mut value: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        8 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*PriceRuleType).RenewableGenerationPercentage =
                                            value as u8;
                                        (*PriceRuleType)
                                            .set_RenewableGenerationPercentage_isUsed(1 as u32);
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
                                        grammar_id = 60 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        4 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*PriceRuleType).PowerRangeStart,
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
            57 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*PriceRuleType).ParkingFeePeriod,
                            );
                            if error == 0 as i32 {
                                (*PriceRuleType).set_ParkingFeePeriod_isUsed(1 as u32);
                                grammar_id = 58 as i32;
                            }
                        }
                        1 => {
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*PriceRuleType).CarbonDioxideEmission,
                            );
                            if error == 0 as i32 {
                                (*PriceRuleType).set_CarbonDioxideEmission_isUsed(1 as u32);
                                grammar_id = 59 as i32;
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
                                        8 as i32 as usize,
                                        &mut value_0,
                                    );
                                    if error == 0 as i32 {
                                        (*PriceRuleType).RenewableGenerationPercentage =
                                            value_0 as u8;
                                        (*PriceRuleType)
                                            .set_RenewableGenerationPercentage_isUsed(1 as u32);
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
                                        grammar_id = 60 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        3 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*PriceRuleType).PowerRangeStart,
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
            58 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*PriceRuleType).CarbonDioxideEmission,
                            );
                            if error == 0 as i32 {
                                (*PriceRuleType).set_CarbonDioxideEmission_isUsed(1 as u32);
                                grammar_id = 59 as i32;
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
                                    let mut value_1: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        8 as i32 as usize,
                                        &mut value_1,
                                    );
                                    if error == 0 as i32 {
                                        (*PriceRuleType).RenewableGenerationPercentage =
                                            value_1 as u8;
                                        (*PriceRuleType)
                                            .set_RenewableGenerationPercentage_isUsed(1 as u32);
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
                                        grammar_id = 60 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        2 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*PriceRuleType).PowerRangeStart,
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
            59 => {
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
                                    let mut value_2: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        8 as i32 as usize,
                                        &mut value_2,
                                    );
                                    if error == 0 as i32 {
                                        (*PriceRuleType).RenewableGenerationPercentage =
                                            value_2 as u8;
                                        (*PriceRuleType)
                                            .set_RenewableGenerationPercentage_isUsed(1 as u32);
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
                                        grammar_id = 60 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*PriceRuleType).PowerRangeStart,
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
            60 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*PriceRuleType).PowerRangeStart,
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
unsafe extern "C" fn decode_iso20_PowerScheduleEntryListType(
    stream: &mut ExiBitstream,
    mut PowerScheduleEntryListType: *mut iso20_PowerScheduleEntryListType,
) -> i32 {
    let mut grammar_id: i32 = 61 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_PowerScheduleEntryListType(PowerScheduleEntryListType);
    while done == 0 {
        match grammar_id {
            61 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*PowerScheduleEntryListType).PowerScheduleEntry.arrayLen as i32)
                                < 1024 as i32
                            {
                                let fresh2 =
                                    (*PowerScheduleEntryListType).PowerScheduleEntry.arrayLen;
                                (*PowerScheduleEntryListType).PowerScheduleEntry.arrayLen =
                                    ((*PowerScheduleEntryListType).PowerScheduleEntry.arrayLen)
                                        .wrapping_add(1);
                                error = decode_iso20_PowerScheduleEntryType(
                                    stream,
                                    &mut *((*PowerScheduleEntryListType).PowerScheduleEntry.array)
                                        .as_mut_ptr()
                                        .offset(fresh2 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 62 as i32;
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
                            if ((*PowerScheduleEntryListType).PowerScheduleEntry.arrayLen as i32)
                                < 1024 as i32
                            {
                                let fresh3 =
                                    (*PowerScheduleEntryListType).PowerScheduleEntry.arrayLen;
                                (*PowerScheduleEntryListType).PowerScheduleEntry.arrayLen =
                                    ((*PowerScheduleEntryListType).PowerScheduleEntry.arrayLen)
                                        .wrapping_add(1);
                                error = decode_iso20_PowerScheduleEntryType(
                                    stream,
                                    &mut *((*PowerScheduleEntryListType).PowerScheduleEntry.array)
                                        .as_mut_ptr()
                                        .offset(fresh3 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*PowerScheduleEntryListType).PowerScheduleEntry.arrayLen as i32)
                                < 1024 as i32
                            {
                                grammar_id = 62 as i32;
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
unsafe extern "C" fn decode_iso20_TaxRuleType(
    stream: &mut ExiBitstream,
    mut TaxRuleType: *mut iso20_TaxRuleType,
) -> i32 {
    let mut grammar_id: i32 = 63 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_TaxRuleType(TaxRuleType);
    while done == 0 {
        match grammar_id {
            63 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(stream, &mut (*TaxRuleType).TaxRuleID);
                            if error == 0 as i32 {
                                grammar_id = 64 as i32;
                            }
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
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    error = exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut (*TaxRuleType).TaxRuleName.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*TaxRuleType).TaxRuleName.charactersLen as i32
                                            >= 2 as i32
                                        {
                                            (*TaxRuleType).TaxRuleName.charactersLen =
                                                ((*TaxRuleType).TaxRuleName.charactersLen as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*TaxRuleType).TaxRuleName.charactersLen as usize,
                                                ((*TaxRuleType).TaxRuleName.characters)
                                                    .as_mut_ptr(),
                                                (80 as i32 + 1 as i32) as usize,
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
                                        (*TaxRuleType).set_TaxRuleName_isUsed(1 as u32);
                                        grammar_id = 65 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*TaxRuleType).TaxRate,
                            );
                            if error == 0 as i32 {
                                grammar_id = 66 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            65 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*TaxRuleType).TaxRate,
                            );
                            if error == 0 as i32 {
                                grammar_id = 66 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            66 => {
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
                                        1 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*TaxRuleType).TaxIncludedInPrice = value as i32;
                                        (*TaxRuleType).set_TaxIncludedInPrice_isUsed(1 as u32);
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
                                        grammar_id = 67 as i32;
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
                                        1 as i32 as usize,
                                        &mut value_0,
                                    );
                                    if error == 0 as i32 {
                                        (*TaxRuleType).AppliesToEnergyFee = value_0 as i32;
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
                                        grammar_id = 68 as i32;
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
            67 => {
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
                                        (*TaxRuleType).AppliesToEnergyFee = value_1 as i32;
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
                                        grammar_id = 68 as i32;
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
            68 => {
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
                                        (*TaxRuleType).AppliesToParkingFee = value_2 as i32;
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
                                        grammar_id = 69 as i32;
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
            69 => {
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
                                    let mut value_3: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_3,
                                    );
                                    if error == 0 as i32 {
                                        (*TaxRuleType).AppliesToOverstayFee = value_3 as i32;
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
                                        grammar_id = 70 as i32;
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
            70 => {
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
                                    let mut value_4: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_4,
                                    );
                                    if error == 0 as i32 {
                                        (*TaxRuleType).AppliesMinimumMaximumCost = value_4 as i32;
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
unsafe extern "C" fn decode_iso20_PriceRuleStackType(
    stream: &mut ExiBitstream,
    mut PriceRuleStackType: *mut iso20_PriceRuleStackType,
) -> i32 {
    let mut grammar_id: i32 = 71 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_PriceRuleStackType(PriceRuleStackType);
    while done == 0 {
        match grammar_id {
            71 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error =
                                decode_exi_type_uint32(stream, &mut (*PriceRuleStackType).Duration);
                            if error == 0 as i32 {
                                grammar_id = 72 as i32;
                            }
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
                            if ((*PriceRuleStackType).PriceRule.arrayLen as i32) < 8 as i32 {
                                let fresh4 = (*PriceRuleStackType).PriceRule.arrayLen;
                                (*PriceRuleStackType).PriceRule.arrayLen =
                                    ((*PriceRuleStackType).PriceRule.arrayLen).wrapping_add(1);
                                error = decode_iso20_PriceRuleType(
                                    stream,
                                    &mut *((*PriceRuleStackType).PriceRule.array)
                                        .as_mut_ptr()
                                        .offset(fresh4 as isize),
                                );
                            } else {
                                error = -(110 as i32);
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
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*PriceRuleStackType).PriceRule.arrayLen as i32) < 8 as i32 {
                                let fresh5 = (*PriceRuleStackType).PriceRule.arrayLen;
                                (*PriceRuleStackType).PriceRule.arrayLen =
                                    ((*PriceRuleStackType).PriceRule.arrayLen).wrapping_add(1);
                                error = decode_iso20_PriceRuleType(
                                    stream,
                                    &mut *((*PriceRuleStackType).PriceRule.array)
                                        .as_mut_ptr()
                                        .offset(fresh5 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*PriceRuleStackType).PriceRule.arrayLen as i32) < 8 as i32 {
                                grammar_id = 73 as i32;
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
unsafe extern "C" fn decode_iso20_AdditionalServiceType(
    stream: &mut ExiBitstream,
    mut AdditionalServiceType: *mut iso20_AdditionalServiceType,
) -> i32 {
    let mut grammar_id: i32 = 74 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_AdditionalServiceType(AdditionalServiceType);
    while done == 0 {
        match grammar_id {
            74 => {
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
                                        &mut (*AdditionalServiceType).ServiceName.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*AdditionalServiceType).ServiceName.charactersLen as i32
                                            >= 2 as i32
                                        {
                                            (*AdditionalServiceType).ServiceName.charactersLen =
                                                ((*AdditionalServiceType).ServiceName.charactersLen
                                                    as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*AdditionalServiceType).ServiceName.charactersLen
                                                    as usize,
                                                ((*AdditionalServiceType).ServiceName.characters)
                                                    .as_mut_ptr(),
                                                (80 as i32 + 1 as i32) as usize,
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
                                        grammar_id = 75 as i32;
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
            75 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*AdditionalServiceType).ServiceFee,
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
unsafe extern "C" fn decode_iso20_PowerScheduleType(
    stream: &mut ExiBitstream,
    mut PowerScheduleType: *mut iso20_PowerScheduleType,
) -> i32 {
    let mut grammar_id: i32 = 76 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_PowerScheduleType(PowerScheduleType);
    while done == 0 {
        match grammar_id {
            76 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint64(
                                stream,
                                &mut (*PowerScheduleType).TimeAnchor,
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
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*PowerScheduleType).AvailableEnergy,
                            );
                            if error == 0 as i32 {
                                (*PowerScheduleType).set_AvailableEnergy_isUsed(1 as u32);
                                grammar_id = 78 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*PowerScheduleType).PowerTolerance,
                            );
                            if error == 0 as i32 {
                                (*PowerScheduleType).set_PowerTolerance_isUsed(1 as u32);
                                grammar_id = 79 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_PowerScheduleEntryListType(
                                stream,
                                &mut (*PowerScheduleType).PowerScheduleEntries,
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
            78 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*PowerScheduleType).PowerTolerance,
                            );
                            if error == 0 as i32 {
                                (*PowerScheduleType).set_PowerTolerance_isUsed(1 as u32);
                                grammar_id = 79 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_PowerScheduleEntryListType(
                                stream,
                                &mut (*PowerScheduleType).PowerScheduleEntries,
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
            79 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_PowerScheduleEntryListType(
                                stream,
                                &mut (*PowerScheduleType).PowerScheduleEntries,
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
unsafe extern "C" fn decode_iso20_EVPowerScheduleEntryListType(
    stream: &mut ExiBitstream,
    mut EVPowerScheduleEntryListType: *mut iso20_EVPowerScheduleEntryListType,
) -> i32 {
    let mut grammar_id: i32 = 80 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_EVPowerScheduleEntryListType(EVPowerScheduleEntryListType);
    while done == 0 {
        match grammar_id {
            80 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*EVPowerScheduleEntryListType)
                                .EVPowerScheduleEntry
                                .arrayLen as i32)
                                < 1024 as i32
                            {
                                let fresh6 = (*EVPowerScheduleEntryListType)
                                    .EVPowerScheduleEntry
                                    .arrayLen;
                                (*EVPowerScheduleEntryListType)
                                    .EVPowerScheduleEntry
                                    .arrayLen = ((*EVPowerScheduleEntryListType)
                                    .EVPowerScheduleEntry
                                    .arrayLen)
                                    .wrapping_add(1);
                                error = decode_iso20_EVPowerScheduleEntryType(
                                    stream,
                                    &mut *((*EVPowerScheduleEntryListType)
                                        .EVPowerScheduleEntry
                                        .array)
                                        .as_mut_ptr()
                                        .offset(fresh6 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 81 as i32;
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
                            if ((*EVPowerScheduleEntryListType)
                                .EVPowerScheduleEntry
                                .arrayLen as i32)
                                < 1024 as i32
                            {
                                let fresh7 = (*EVPowerScheduleEntryListType)
                                    .EVPowerScheduleEntry
                                    .arrayLen;
                                (*EVPowerScheduleEntryListType)
                                    .EVPowerScheduleEntry
                                    .arrayLen = ((*EVPowerScheduleEntryListType)
                                    .EVPowerScheduleEntry
                                    .arrayLen)
                                    .wrapping_add(1);
                                error = decode_iso20_EVPowerScheduleEntryType(
                                    stream,
                                    &mut *((*EVPowerScheduleEntryListType)
                                        .EVPowerScheduleEntry
                                        .array)
                                        .as_mut_ptr()
                                        .offset(fresh7 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*EVPowerScheduleEntryListType)
                                .EVPowerScheduleEntry
                                .arrayLen as i32)
                                < 1024 as i32
                            {
                                grammar_id = 81 as i32;
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
unsafe extern "C" fn decode_iso20_OverstayRuleType(
    stream: &mut ExiBitstream,
    mut OverstayRuleType: *mut iso20_OverstayRuleType,
) -> i32 {
    let mut grammar_id: i32 = 82 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_OverstayRuleType(OverstayRuleType);
    while done == 0 {
        match grammar_id {
            82 => {
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
                                        &mut (*OverstayRuleType)
                                            .OverstayRuleDescription
                                            .charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*OverstayRuleType).OverstayRuleDescription.charactersLen
                                            as i32
                                            >= 2 as i32
                                        {
                                            (*OverstayRuleType)
                                                .OverstayRuleDescription
                                                .charactersLen = ((*OverstayRuleType)
                                                .OverstayRuleDescription
                                                .charactersLen
                                                as i32
                                                - 2 as i32)
                                                as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*OverstayRuleType)
                                                    .OverstayRuleDescription
                                                    .charactersLen
                                                    as usize,
                                                ((*OverstayRuleType)
                                                    .OverstayRuleDescription
                                                    .characters)
                                                    .as_mut_ptr(),
                                                (160 as i32 + 1 as i32) as usize,
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
                                        (*OverstayRuleType)
                                            .set_OverstayRuleDescription_isUsed(1 as u32);
                                        grammar_id = 83 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            error =
                                decode_exi_type_uint32(stream, &mut (*OverstayRuleType).StartTime);
                            if error == 0 as i32 {
                                grammar_id = 84 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            83 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error =
                                decode_exi_type_uint32(stream, &mut (*OverstayRuleType).StartTime);
                            if error == 0 as i32 {
                                grammar_id = 84 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            84 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*OverstayRuleType).OverstayFee,
                            );
                            if error == 0 as i32 {
                                grammar_id = 85 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            85 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*OverstayRuleType).OverstayFeePeriod,
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
unsafe extern "C" fn decode_iso20_EVPriceRuleStackListType(
    stream: &mut ExiBitstream,
    mut EVPriceRuleStackListType: *mut iso20_EVPriceRuleStackListType,
) -> i32 {
    let mut grammar_id: i32 = 86 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_EVPriceRuleStackListType(EVPriceRuleStackListType);
    while done == 0 {
        match grammar_id {
            86 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*EVPriceRuleStackListType).EVPriceRuleStack.arrayLen as i32)
                                < 1024 as i32
                            {
                                let fresh8 = (*EVPriceRuleStackListType).EVPriceRuleStack.arrayLen;
                                (*EVPriceRuleStackListType).EVPriceRuleStack.arrayLen =
                                    ((*EVPriceRuleStackListType).EVPriceRuleStack.arrayLen)
                                        .wrapping_add(1);
                                error = decode_iso20_EVPriceRuleStackType(
                                    stream,
                                    &mut *((*EVPriceRuleStackListType).EVPriceRuleStack.array)
                                        .as_mut_ptr()
                                        .offset(fresh8 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 87 as i32;
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
                            if ((*EVPriceRuleStackListType).EVPriceRuleStack.arrayLen as i32)
                                < 1024 as i32
                            {
                                let fresh9 = (*EVPriceRuleStackListType).EVPriceRuleStack.arrayLen;
                                (*EVPriceRuleStackListType).EVPriceRuleStack.arrayLen =
                                    ((*EVPriceRuleStackListType).EVPriceRuleStack.arrayLen)
                                        .wrapping_add(1);
                                error = decode_iso20_EVPriceRuleStackType(
                                    stream,
                                    &mut *((*EVPriceRuleStackListType).EVPriceRuleStack.array)
                                        .as_mut_ptr()
                                        .offset(fresh9 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*EVPriceRuleStackListType).EVPriceRuleStack.arrayLen as i32)
                                < 1024 as i32
                            {
                                grammar_id = 87 as i32;
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
unsafe extern "C" fn decode_iso20_SPKIDataType(
    stream: &mut ExiBitstream,
    mut SPKIDataType: *mut iso20_SPKIDataType,
) -> i32 {
    let mut grammar_id: i32 = 88 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_SPKIDataType(SPKIDataType);
    while done == 0 {
        match grammar_id {
            88 => {
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
                                grammar_id = 89 as i32;
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
unsafe extern "C" fn decode_iso20_SignedInfoType(
    stream: &mut ExiBitstream,
    mut SignedInfoType: *mut iso20_SignedInfoType,
) -> i32 {
    let mut grammar_id: i32 = 90 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_SignedInfoType(SignedInfoType);
    while done == 0 {
        match grammar_id {
            90 => {
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
                            grammar_id = 91 as i32;
                        }
                        1 => {
                            error = decode_iso20_CanonicalizationMethodType(
                                stream,
                                &mut (*SignedInfoType).CanonicalizationMethod,
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
            91 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_CanonicalizationMethodType(
                                stream,
                                &mut (*SignedInfoType).CanonicalizationMethod,
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
                            error = decode_iso20_SignatureMethodType(
                                stream,
                                &mut (*SignedInfoType).SignatureMethod,
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
                            if ((*SignedInfoType).Reference.arrayLen as i32) < 4 as i32 {
                                let fresh10 = (*SignedInfoType).Reference.arrayLen;
                                (*SignedInfoType).Reference.arrayLen =
                                    ((*SignedInfoType).Reference.arrayLen).wrapping_add(1);
                                error = decode_iso20_ReferenceType(
                                    stream,
                                    &mut *((*SignedInfoType).Reference.array)
                                        .as_mut_ptr()
                                        .offset(fresh10 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 94 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            94 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SignedInfoType).Reference.arrayLen as i32) < 4 as i32 {
                                let fresh11 = (*SignedInfoType).Reference.arrayLen;
                                (*SignedInfoType).Reference.arrayLen =
                                    ((*SignedInfoType).Reference.arrayLen).wrapping_add(1);
                                error = decode_iso20_ReferenceType(
                                    stream,
                                    &mut *((*SignedInfoType).Reference.array)
                                        .as_mut_ptr()
                                        .offset(fresh11 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 94 as i32;
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
unsafe extern "C" fn decode_iso20_EVPowerScheduleType(
    stream: &mut ExiBitstream,
    mut EVPowerScheduleType: *mut iso20_EVPowerScheduleType,
) -> i32 {
    let mut grammar_id: i32 = 95 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_EVPowerScheduleType(EVPowerScheduleType);
    while done == 0 {
        match grammar_id {
            95 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint64(
                                stream,
                                &mut (*EVPowerScheduleType).TimeAnchor,
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
                            error = decode_iso20_EVPowerScheduleEntryListType(
                                stream,
                                &mut (*EVPowerScheduleType).EVPowerScheduleEntries,
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
unsafe extern "C" fn decode_iso20_SignatureValueType(
    stream: &mut ExiBitstream,
    mut SignatureValueType: *mut iso20_SignatureValueType,
) -> i32 {
    let mut grammar_id: i32 = 97 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_SignatureValueType(SignatureValueType);
    while done == 0 {
        match grammar_id {
            97 => {
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
                            grammar_id = 98 as i32;
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
            98 => {
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
unsafe extern "C" fn decode_iso20_SubCertificatesType(
    stream: &mut ExiBitstream,
    mut SubCertificatesType: *mut iso20_SubCertificatesType,
) -> i32 {
    let mut grammar_id: i32 = 99 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_SubCertificatesType(SubCertificatesType);
    while done == 0 {
        match grammar_id {
            99 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SubCertificatesType).Certificate.arrayLen as i32) < 3 as i32 {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*SubCertificatesType).Certificate.array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*SubCertificatesType).Certificate.arrayLen as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*SubCertificatesType).Certificate.array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*SubCertificatesType).Certificate.arrayLen as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    1600 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*SubCertificatesType).Certificate.arrayLen =
                                        ((*SubCertificatesType).Certificate.arrayLen)
                                            .wrapping_add(1);
                                    (*SubCertificatesType).Certificate.arrayLen;
                                    grammar_id = 100 as i32;
                                }
                            } else {
                                error = -(110 as i32);
                            }
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
                            if ((*SubCertificatesType).Certificate.arrayLen as i32) < 3 as i32 {
                                error = decode_exi_type_hex_binary(
                                    stream,
                                    &mut (*((*SubCertificatesType).Certificate.array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*SubCertificatesType).Certificate.arrayLen as isize,
                                        ))
                                    .bytesLen,
                                    &mut *((*((*SubCertificatesType).Certificate.array)
                                        .as_mut_ptr()
                                        .offset(
                                            (*SubCertificatesType).Certificate.arrayLen as isize,
                                        ))
                                    .bytes)
                                        .as_mut_ptr()
                                        .offset(0 as i32 as isize),
                                    1600 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    (*SubCertificatesType).Certificate.arrayLen =
                                        ((*SubCertificatesType).Certificate.arrayLen)
                                            .wrapping_add(1);
                                    (*SubCertificatesType).Certificate.arrayLen;
                                    grammar_id = 100 as i32;
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
unsafe extern "C" fn decode_iso20_ParameterType(
    stream: &mut ExiBitstream,
    mut ParameterType: *mut iso20_ParameterType,
) -> i32 {
    let mut grammar_id: i32 = 101 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_ParameterType(ParameterType);
    while done == 0 {
        match grammar_id {
            101 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*ParameterType).Name.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*ParameterType).Name.charactersLen as i32 >= 2 as i32 {
                                    (*ParameterType).Name.charactersLen =
                                        ((*ParameterType).Name.charactersLen as i32 - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*ParameterType).Name.charactersLen as usize,
                                        ((*ParameterType).Name.characters).as_mut_ptr(),
                                        (80 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            grammar_id = 102 as i32;
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
                                        (*ParameterType).boolValue = value as i32;
                                        (*ParameterType).set_boolValue_isUsed(1 as u32);
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
                                        8 as i32 as usize,
                                        &mut value_0,
                                    );
                                    if error == 0 as i32 {
                                        (*ParameterType).byteValue =
                                            value_0.wrapping_add(-(128 as i32) as u32) as int8_t;
                                        (*ParameterType).set_byteValue_isUsed(1 as u32);
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
                            error =
                                decode_exi_type_integer16(stream, &mut (*ParameterType).shortValue);
                            if error == 0 as i32 {
                                (*ParameterType).set_shortValue_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        3 => {
                            error =
                                decode_exi_type_integer32(stream, &mut (*ParameterType).intValue);
                            if error == 0 as i32 {
                                (*ParameterType).set_intValue_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        4 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*ParameterType).rationalNumber,
                            );
                            if error == 0 as i32 {
                                (*ParameterType).set_rationalNumber_isUsed(1 as u32);
                                grammar_id = 2 as i32;
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
                                    error = exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut (*ParameterType).finiteString.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*ParameterType).finiteString.charactersLen as i32
                                            >= 2 as i32
                                        {
                                            (*ParameterType).finiteString.charactersLen =
                                                ((*ParameterType).finiteString.charactersLen as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*ParameterType).finiteString.charactersLen
                                                    as usize,
                                                ((*ParameterType).finiteString.characters)
                                                    .as_mut_ptr(),
                                                (80 as i32 + 1 as i32) as usize,
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
                                        (*ParameterType).set_finiteString_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_EVAbsolutePriceScheduleType(
    stream: &mut ExiBitstream,
    mut EVAbsolutePriceScheduleType: *mut iso20_EVAbsolutePriceScheduleType,
) -> i32 {
    let mut grammar_id: i32 = 103 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_EVAbsolutePriceScheduleType(EVAbsolutePriceScheduleType);
    while done == 0 {
        match grammar_id {
            103 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint64(
                                stream,
                                &mut (*EVAbsolutePriceScheduleType).TimeAnchor,
                            );
                            if error == 0 as i32 {
                                grammar_id = 104 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            104 => {
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
                                        &mut (*EVAbsolutePriceScheduleType).Currency.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*EVAbsolutePriceScheduleType).Currency.charactersLen
                                            as i32
                                            >= 2 as i32
                                        {
                                            (*EVAbsolutePriceScheduleType).Currency.charactersLen =
                                                ((*EVAbsolutePriceScheduleType)
                                                    .Currency
                                                    .charactersLen
                                                    as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*EVAbsolutePriceScheduleType)
                                                    .Currency
                                                    .charactersLen
                                                    as usize,
                                                ((*EVAbsolutePriceScheduleType)
                                                    .Currency
                                                    .characters)
                                                    .as_mut_ptr(),
                                                (3 as i32 + 1 as i32) as usize,
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
                                        grammar_id = 105 as i32;
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
            105 => {
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
                                        &mut (*EVAbsolutePriceScheduleType)
                                            .PriceAlgorithm
                                            .charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*EVAbsolutePriceScheduleType)
                                            .PriceAlgorithm
                                            .charactersLen
                                            as i32
                                            >= 2 as i32
                                        {
                                            (*EVAbsolutePriceScheduleType)
                                                .PriceAlgorithm
                                                .charactersLen = ((*EVAbsolutePriceScheduleType)
                                                .PriceAlgorithm
                                                .charactersLen
                                                as i32
                                                - 2 as i32)
                                                as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*EVAbsolutePriceScheduleType)
                                                    .PriceAlgorithm
                                                    .charactersLen
                                                    as usize,
                                                ((*EVAbsolutePriceScheduleType)
                                                    .PriceAlgorithm
                                                    .characters)
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
                                        grammar_id = 106 as i32;
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
            106 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_EVPriceRuleStackListType(
                                stream,
                                &mut (*EVAbsolutePriceScheduleType).EVPriceRuleStacks,
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
unsafe extern "C" fn decode_iso20_DetailedCostType(
    stream: &mut ExiBitstream,
    mut DetailedCostType: *mut iso20_DetailedCostType,
) -> i32 {
    let mut grammar_id: i32 = 107 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_DetailedCostType(DetailedCostType);
    while done == 0 {
        match grammar_id {
            107 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*DetailedCostType).Amount,
                            );
                            if error == 0 as i32 {
                                grammar_id = 108 as i32;
                            }
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
                            error = decode_iso20_RationalNumberType(
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
unsafe extern "C" fn decode_iso20_KeyInfoType(
    stream: &mut ExiBitstream,
    mut KeyInfoType: *mut iso20_KeyInfoType,
) -> i32 {
    let mut grammar_id: i32 = 109 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_KeyInfoType(KeyInfoType);
    while done == 0 {
        match grammar_id {
            109 => {
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
                            grammar_id = 110 as i32;
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
                            error = decode_iso20_KeyValueType(stream, &mut (*KeyInfoType).KeyValue);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_KeyValue_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_RetrievalMethodType(
                                stream,
                                &mut (*KeyInfoType).RetrievalMethod,
                            );
                            if error == 0 as i32 {
                                (*KeyInfoType).set_RetrievalMethod_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        4 => {
                            error = decode_iso20_X509DataType(stream, &mut (*KeyInfoType).X509Data);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_X509Data_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        5 => {
                            error = decode_iso20_PGPDataType(stream, &mut (*KeyInfoType).PGPData);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_PGPData_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        6 => {
                            error = decode_iso20_SPKIDataType(stream, &mut (*KeyInfoType).SPKIData);
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
                            error = decode_iso20_KeyValueType(stream, &mut (*KeyInfoType).KeyValue);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_KeyValue_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_RetrievalMethodType(
                                stream,
                                &mut (*KeyInfoType).RetrievalMethod,
                            );
                            if error == 0 as i32 {
                                (*KeyInfoType).set_RetrievalMethod_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_X509DataType(stream, &mut (*KeyInfoType).X509Data);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_X509Data_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        4 => {
                            error = decode_iso20_PGPDataType(stream, &mut (*KeyInfoType).PGPData);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_PGPData_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        5 => {
                            error = decode_iso20_SPKIDataType(stream, &mut (*KeyInfoType).SPKIData);
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
unsafe extern "C" fn decode_iso20_ObjectType(
    stream: &mut ExiBitstream,
    mut ObjectType: *mut iso20_ObjectType,
) -> i32 {
    let mut grammar_id: i32 = 111 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_ObjectType(ObjectType);
    while done == 0 {
        match grammar_id {
            111 => {
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
                            grammar_id = 112 as i32;
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
                            grammar_id = 113 as i32;
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
                            grammar_id = 114 as i32;
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
            112 => {
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
                            grammar_id = 113 as i32;
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
                            grammar_id = 114 as i32;
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
            113 => {
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
                            grammar_id = 114 as i32;
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
            114 => {
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
unsafe extern "C" fn decode_iso20_PriceLevelScheduleEntryListType(
    stream: &mut ExiBitstream,
    mut PriceLevelScheduleEntryListType: *mut iso20_PriceLevelScheduleEntryListType,
) -> i32 {
    let mut grammar_id: i32 = 115 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_PriceLevelScheduleEntryListType(PriceLevelScheduleEntryListType);
    while done == 0 {
        match grammar_id {
            115 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*PriceLevelScheduleEntryListType)
                                .PriceLevelScheduleEntry
                                .arrayLen as i32)
                                < 1024 as i32
                            {
                                let fresh12 = (*PriceLevelScheduleEntryListType)
                                    .PriceLevelScheduleEntry
                                    .arrayLen;
                                (*PriceLevelScheduleEntryListType)
                                    .PriceLevelScheduleEntry
                                    .arrayLen = ((*PriceLevelScheduleEntryListType)
                                    .PriceLevelScheduleEntry
                                    .arrayLen)
                                    .wrapping_add(1);
                                error = decode_iso20_PriceLevelScheduleEntryType(
                                    stream,
                                    &mut *((*PriceLevelScheduleEntryListType)
                                        .PriceLevelScheduleEntry
                                        .array)
                                        .as_mut_ptr()
                                        .offset(fresh12 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 116 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            116 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*PriceLevelScheduleEntryListType)
                                .PriceLevelScheduleEntry
                                .arrayLen as i32)
                                < 1024 as i32
                            {
                                let fresh13 = (*PriceLevelScheduleEntryListType)
                                    .PriceLevelScheduleEntry
                                    .arrayLen;
                                (*PriceLevelScheduleEntryListType)
                                    .PriceLevelScheduleEntry
                                    .arrayLen = ((*PriceLevelScheduleEntryListType)
                                    .PriceLevelScheduleEntry
                                    .arrayLen)
                                    .wrapping_add(1);
                                error = decode_iso20_PriceLevelScheduleEntryType(
                                    stream,
                                    &mut *((*PriceLevelScheduleEntryListType)
                                        .PriceLevelScheduleEntry
                                        .array)
                                        .as_mut_ptr()
                                        .offset(fresh13 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*PriceLevelScheduleEntryListType)
                                .PriceLevelScheduleEntry
                                .arrayLen as i32)
                                < 1024 as i32
                            {
                                grammar_id = 116 as i32;
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
unsafe extern "C" fn decode_iso20_DetailedTaxType(
    stream: &mut ExiBitstream,
    mut DetailedTaxType: *mut iso20_DetailedTaxType,
) -> i32 {
    let mut grammar_id: i32 = 117 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_DetailedTaxType(DetailedTaxType);
    while done == 0 {
        match grammar_id {
            117 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error =
                                decode_exi_type_uint32(stream, &mut (*DetailedTaxType).TaxRuleID);
                            if error == 0 as i32 {
                                grammar_id = 118 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            118 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_RationalNumberType(
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
unsafe extern "C" fn decode_iso20_TaxRuleListType(
    stream: &mut ExiBitstream,
    mut TaxRuleListType: *mut iso20_TaxRuleListType,
) -> i32 {
    let mut grammar_id: i32 = 119 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_TaxRuleListType(TaxRuleListType);
    while done == 0 {
        match grammar_id {
            119 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*TaxRuleListType).TaxRule.arrayLen as i32) < 10 as i32 {
                                let fresh14 = (*TaxRuleListType).TaxRule.arrayLen;
                                (*TaxRuleListType).TaxRule.arrayLen =
                                    ((*TaxRuleListType).TaxRule.arrayLen).wrapping_add(1);
                                error = decode_iso20_TaxRuleType(
                                    stream,
                                    &mut *((*TaxRuleListType).TaxRule.array)
                                        .as_mut_ptr()
                                        .offset(fresh14 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 120 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            120 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*TaxRuleListType).TaxRule.arrayLen as i32) < 10 as i32 {
                                let fresh15 = (*TaxRuleListType).TaxRule.arrayLen;
                                (*TaxRuleListType).TaxRule.arrayLen =
                                    ((*TaxRuleListType).TaxRule.arrayLen).wrapping_add(1);
                                error = decode_iso20_TaxRuleType(
                                    stream,
                                    &mut *((*TaxRuleListType).TaxRule.array)
                                        .as_mut_ptr()
                                        .offset(fresh15 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*TaxRuleListType).TaxRule.arrayLen as i32) < 10 as i32 {
                                grammar_id = 120 as i32;
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
unsafe extern "C" fn decode_iso20_PriceRuleStackListType(
    stream: &mut ExiBitstream,
    mut PriceRuleStackListType: *mut iso20_PriceRuleStackListType,
) -> i32 {
    let mut grammar_id: i32 = 121 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_PriceRuleStackListType(PriceRuleStackListType);
    while done == 0 {
        match grammar_id {
            121 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*PriceRuleStackListType).PriceRuleStack.arrayLen as i32)
                                < 64 as i32
                            {
                                let fresh16 = (*PriceRuleStackListType).PriceRuleStack.arrayLen;
                                (*PriceRuleStackListType).PriceRuleStack.arrayLen =
                                    ((*PriceRuleStackListType).PriceRuleStack.arrayLen)
                                        .wrapping_add(1);
                                error = decode_iso20_PriceRuleStackType(
                                    stream,
                                    &mut *((*PriceRuleStackListType).PriceRuleStack.array)
                                        .as_mut_ptr()
                                        .offset(fresh16 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 122 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            122 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*PriceRuleStackListType).PriceRuleStack.arrayLen as i32)
                                < 64 as i32
                            {
                                let fresh17 = (*PriceRuleStackListType).PriceRuleStack.arrayLen;
                                (*PriceRuleStackListType).PriceRuleStack.arrayLen =
                                    ((*PriceRuleStackListType).PriceRuleStack.arrayLen)
                                        .wrapping_add(1);
                                error = decode_iso20_PriceRuleStackType(
                                    stream,
                                    &mut *((*PriceRuleStackListType).PriceRuleStack.array)
                                        .as_mut_ptr()
                                        .offset(fresh17 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*PriceRuleStackListType).PriceRuleStack.arrayLen as i32)
                                < 1024 as i32
                            {
                                grammar_id = 122 as i32;
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
unsafe extern "C" fn decode_iso20_OverstayRuleListType(
    stream: &mut ExiBitstream,
    mut OverstayRuleListType: *mut iso20_OverstayRuleListType,
) -> i32 {
    let mut grammar_id: i32 = 123 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_OverstayRuleListType(OverstayRuleListType);
    while done == 0 {
        match grammar_id {
            123 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*OverstayRuleListType).OverstayTimeThreshold,
                            );
                            if error == 0 as i32 {
                                (*OverstayRuleListType).set_OverstayTimeThreshold_isUsed(1 as u32);
                                grammar_id = 125 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*OverstayRuleListType).OverstayPowerThreshold,
                            );
                            if error == 0 as i32 {
                                (*OverstayRuleListType).set_OverstayPowerThreshold_isUsed(1 as u32);
                                grammar_id = 127 as i32;
                            }
                        }
                        2 => {
                            if ((*OverstayRuleListType).OverstayRule.arrayLen as i32) < 5 as i32 {
                                let fresh18 = (*OverstayRuleListType).OverstayRule.arrayLen;
                                (*OverstayRuleListType).OverstayRule.arrayLen =
                                    ((*OverstayRuleListType).OverstayRule.arrayLen).wrapping_add(1);
                                error = decode_iso20_OverstayRuleType(
                                    stream,
                                    &mut *((*OverstayRuleListType).OverstayRule.array)
                                        .as_mut_ptr()
                                        .offset(fresh18 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 124 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            124 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*OverstayRuleListType).OverstayRule.arrayLen as i32) < 5 as i32 {
                                let fresh19 = (*OverstayRuleListType).OverstayRule.arrayLen;
                                (*OverstayRuleListType).OverstayRule.arrayLen =
                                    ((*OverstayRuleListType).OverstayRule.arrayLen).wrapping_add(1);
                                error = decode_iso20_OverstayRuleType(
                                    stream,
                                    &mut *((*OverstayRuleListType).OverstayRule.array)
                                        .as_mut_ptr()
                                        .offset(fresh19 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*OverstayRuleListType).OverstayRule.arrayLen as i32) < 5 as i32 {
                                grammar_id = 124 as i32;
                            } else {
                                grammar_id = 125 as i32;
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
            125 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*OverstayRuleListType).OverstayPowerThreshold,
                            );
                            if error == 0 as i32 {
                                (*OverstayRuleListType).set_OverstayPowerThreshold_isUsed(1 as u32);
                                grammar_id = 127 as i32;
                            }
                        }
                        1 => {
                            if ((*OverstayRuleListType).OverstayRule.arrayLen as i32) < 5 as i32 {
                                let fresh20 = (*OverstayRuleListType).OverstayRule.arrayLen;
                                (*OverstayRuleListType).OverstayRule.arrayLen =
                                    ((*OverstayRuleListType).OverstayRule.arrayLen).wrapping_add(1);
                                error = decode_iso20_OverstayRuleType(
                                    stream,
                                    &mut *((*OverstayRuleListType).OverstayRule.array)
                                        .as_mut_ptr()
                                        .offset(fresh20 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 126 as i32;
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
                            if ((*OverstayRuleListType).OverstayRule.arrayLen as i32) < 5 as i32 {
                                let fresh21 = (*OverstayRuleListType).OverstayRule.arrayLen;
                                (*OverstayRuleListType).OverstayRule.arrayLen =
                                    ((*OverstayRuleListType).OverstayRule.arrayLen).wrapping_add(1);
                                error = decode_iso20_OverstayRuleType(
                                    stream,
                                    &mut *((*OverstayRuleListType).OverstayRule.array)
                                        .as_mut_ptr()
                                        .offset(fresh21 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*OverstayRuleListType).OverstayRule.arrayLen as i32) < 5 as i32 {
                                grammar_id = 126 as i32;
                            } else {
                                grammar_id = 127 as i32;
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
            127 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*OverstayRuleListType).OverstayRule.arrayLen as i32) < 5 as i32 {
                                let fresh22 = (*OverstayRuleListType).OverstayRule.arrayLen;
                                (*OverstayRuleListType).OverstayRule.arrayLen =
                                    ((*OverstayRuleListType).OverstayRule.arrayLen).wrapping_add(1);
                                error = decode_iso20_OverstayRuleType(
                                    stream,
                                    &mut *((*OverstayRuleListType).OverstayRule.array)
                                        .as_mut_ptr()
                                        .offset(fresh22 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 128 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            128 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*OverstayRuleListType).OverstayRule.arrayLen as i32) < 5 as i32 {
                                let fresh23 = (*OverstayRuleListType).OverstayRule.arrayLen;
                                (*OverstayRuleListType).OverstayRule.arrayLen =
                                    ((*OverstayRuleListType).OverstayRule.arrayLen).wrapping_add(1);
                                error = decode_iso20_OverstayRuleType(
                                    stream,
                                    &mut *((*OverstayRuleListType).OverstayRule.array)
                                        .as_mut_ptr()
                                        .offset(fresh23 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*OverstayRuleListType).OverstayRule.arrayLen as i32) < 5 as i32 {
                                grammar_id = 128 as i32;
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
unsafe extern "C" fn decode_iso20_AdditionalServiceListType(
    stream: &mut ExiBitstream,
    mut AdditionalServiceListType: *mut iso20_AdditionalServiceListType,
) -> i32 {
    let mut grammar_id: i32 = 129 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_AdditionalServiceListType(AdditionalServiceListType);
    while done == 0 {
        match grammar_id {
            129 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*AdditionalServiceListType).AdditionalService.arrayLen as i32)
                                < 5 as i32
                            {
                                let fresh24 =
                                    (*AdditionalServiceListType).AdditionalService.arrayLen;
                                (*AdditionalServiceListType).AdditionalService.arrayLen =
                                    ((*AdditionalServiceListType).AdditionalService.arrayLen)
                                        .wrapping_add(1);
                                error = decode_iso20_AdditionalServiceType(
                                    stream,
                                    &mut *((*AdditionalServiceListType).AdditionalService.array)
                                        .as_mut_ptr()
                                        .offset(fresh24 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 130 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            130 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*AdditionalServiceListType).AdditionalService.arrayLen as i32)
                                < 5 as i32
                            {
                                let fresh25 =
                                    (*AdditionalServiceListType).AdditionalService.arrayLen;
                                (*AdditionalServiceListType).AdditionalService.arrayLen =
                                    ((*AdditionalServiceListType).AdditionalService.arrayLen)
                                        .wrapping_add(1);
                                error = decode_iso20_AdditionalServiceType(
                                    stream,
                                    &mut *((*AdditionalServiceListType).AdditionalService.array)
                                        .as_mut_ptr()
                                        .offset(fresh25 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*AdditionalServiceListType).AdditionalService.arrayLen as i32)
                                < 5 as i32
                            {
                                grammar_id = 130 as i32;
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
unsafe extern "C" fn decode_iso20_ServiceType(
    stream: &mut ExiBitstream,
    mut ServiceType: *mut iso20_ServiceType,
) -> i32 {
    let mut grammar_id: i32 = 131 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_ServiceType(ServiceType);
    while done == 0 {
        match grammar_id {
            131 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint16(stream, &mut (*ServiceType).ServiceID);
                            if error == 0 as i32 {
                                grammar_id = 132 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            132 => {
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
                                        (*ServiceType).FreeService = value as i32;
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
unsafe extern "C" fn decode_iso20_ParameterSetType(
    stream: &mut ExiBitstream,
    mut ParameterSetType: *mut iso20_ParameterSetType,
) -> i32 {
    let mut grammar_id: i32 = 133 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_ParameterSetType(ParameterSetType);
    while done == 0 {
        match grammar_id {
            133 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*ParameterSetType).ParameterSetID,
                            );
                            if error == 0 as i32 {
                                grammar_id = 134 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            134 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ParameterSetType).Parameter.arrayLen as i32) < 8 as i32 {
                                let fresh26 = (*ParameterSetType).Parameter.arrayLen;
                                (*ParameterSetType).Parameter.arrayLen =
                                    ((*ParameterSetType).Parameter.arrayLen).wrapping_add(1);
                                error = decode_iso20_ParameterType(
                                    stream,
                                    &mut *((*ParameterSetType).Parameter.array)
                                        .as_mut_ptr()
                                        .offset(fresh26 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 135 as i32;
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
                            if ((*ParameterSetType).Parameter.arrayLen as i32) < 8 as i32 {
                                let fresh27 = (*ParameterSetType).Parameter.arrayLen;
                                (*ParameterSetType).Parameter.arrayLen =
                                    ((*ParameterSetType).Parameter.arrayLen).wrapping_add(1);
                                error = decode_iso20_ParameterType(
                                    stream,
                                    &mut *((*ParameterSetType).Parameter.array)
                                        .as_mut_ptr()
                                        .offset(fresh27 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*ParameterSetType).Parameter.arrayLen as i32) < 32 as i32 {
                                grammar_id = 135 as i32;
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
unsafe extern "C" fn decode_iso20_SupportedProvidersListType(
    stream: &mut ExiBitstream,
    mut SupportedProvidersListType: *mut iso20_SupportedProvidersListType,
) -> i32 {
    let mut grammar_id: i32 = 136 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_SupportedProvidersListType(SupportedProvidersListType);
    while done == 0 {
        match grammar_id {
            136 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SupportedProvidersListType).ProviderID.arrayLen as i32)
                                < 128 as i32
                            {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        error = exi_basetypes_decoder_uint_16(
                                            stream,
                                            &mut (*((*SupportedProvidersListType)
                                                .ProviderID
                                                .array)
                                                .as_mut_ptr()
                                                .offset(
                                                    (*SupportedProvidersListType)
                                                        .ProviderID
                                                        .arrayLen
                                                        as isize,
                                                ))
                                            .charactersLen,
                                        );
                                        if error == 0 as i32 {
                                            if (*SupportedProvidersListType).ProviderID.array
                                                [(*SupportedProvidersListType).ProviderID.arrayLen
                                                    as usize]
                                                .charactersLen
                                                as i32
                                                >= 2 as i32
                                            {
                                                (*SupportedProvidersListType).ProviderID.array
                                                    [(*SupportedProvidersListType)
                                                        .ProviderID
                                                        .arrayLen
                                                        as usize]
                                                    .charactersLen =
                                                    ((*SupportedProvidersListType).ProviderID.array
                                                        [(*SupportedProvidersListType)
                                                            .ProviderID
                                                            .arrayLen
                                                            as usize]
                                                        .charactersLen
                                                        as i32
                                                        - 2 as i32)
                                                        as u16;
                                                error = exi_basetypes_decoder_characters(
                                                    stream,
                                                    (*SupportedProvidersListType).ProviderID.array
                                                        [(*SupportedProvidersListType)
                                                            .ProviderID
                                                            .arrayLen
                                                            as usize]
                                                        .charactersLen
                                                        as usize,
                                                    ((*SupportedProvidersListType)
                                                        .ProviderID
                                                        .array
                                                        [(*SupportedProvidersListType)
                                                            .ProviderID
                                                            .arrayLen
                                                            as usize]
                                                        .characters)
                                                        .as_mut_ptr(),
                                                    (80 as i32 + 1 as i32) as usize,
                                                );
                                                if error == 0 as i32 {
                                                    (*SupportedProvidersListType)
                                                        .ProviderID
                                                        .arrayLen = ((*SupportedProvidersListType)
                                                        .ProviderID
                                                        .arrayLen)
                                                        .wrapping_add(1);
                                                    (*SupportedProvidersListType)
                                                        .ProviderID
                                                        .arrayLen;
                                                }
                                            } else {
                                                error = -(200 as i32);
                                            }
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
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SupportedProvidersListType).ProviderID.arrayLen as i32)
                                < 128 as i32
                            {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        error = exi_basetypes_decoder_uint_16(
                                            stream,
                                            &mut (*((*SupportedProvidersListType)
                                                .ProviderID
                                                .array)
                                                .as_mut_ptr()
                                                .offset(
                                                    (*SupportedProvidersListType)
                                                        .ProviderID
                                                        .arrayLen
                                                        as isize,
                                                ))
                                            .charactersLen,
                                        );
                                        if error == 0 as i32 {
                                            if (*SupportedProvidersListType).ProviderID.array
                                                [(*SupportedProvidersListType).ProviderID.arrayLen
                                                    as usize]
                                                .charactersLen
                                                as i32
                                                >= 2 as i32
                                            {
                                                (*SupportedProvidersListType).ProviderID.array
                                                    [(*SupportedProvidersListType)
                                                        .ProviderID
                                                        .arrayLen
                                                        as usize]
                                                    .charactersLen =
                                                    ((*SupportedProvidersListType).ProviderID.array
                                                        [(*SupportedProvidersListType)
                                                            .ProviderID
                                                            .arrayLen
                                                            as usize]
                                                        .charactersLen
                                                        as i32
                                                        - 2 as i32)
                                                        as u16;
                                                error = exi_basetypes_decoder_characters(
                                                    stream,
                                                    (*SupportedProvidersListType).ProviderID.array
                                                        [(*SupportedProvidersListType)
                                                            .ProviderID
                                                            .arrayLen
                                                            as usize]
                                                        .charactersLen
                                                        as usize,
                                                    ((*SupportedProvidersListType)
                                                        .ProviderID
                                                        .array
                                                        [(*SupportedProvidersListType)
                                                            .ProviderID
                                                            .arrayLen
                                                            as usize]
                                                        .characters)
                                                        .as_mut_ptr(),
                                                    (80 as i32 + 1 as i32) as usize,
                                                );
                                                if error == 0 as i32 {
                                                    (*SupportedProvidersListType)
                                                        .ProviderID
                                                        .arrayLen = ((*SupportedProvidersListType)
                                                        .ProviderID
                                                        .arrayLen)
                                                        .wrapping_add(1);
                                                    (*SupportedProvidersListType)
                                                        .ProviderID
                                                        .arrayLen;
                                                }
                                            } else {
                                                error = -(200 as i32);
                                            }
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
                                        grammar_id = 137 as i32;
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
unsafe extern "C" fn decode_iso20_ContractCertificateChainType(
    stream: &mut ExiBitstream,
    mut ContractCertificateChainType: *mut iso20_ContractCertificateChainType,
) -> i32 {
    let mut grammar_id: i32 = 138 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_ContractCertificateChainType(ContractCertificateChainType);
    while done == 0 {
        match grammar_id {
            138 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*ContractCertificateChainType).Certificate.bytesLen,
                                &mut *((*ContractCertificateChainType).Certificate.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                1600 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 139 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            139 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_SubCertificatesType(
                                stream,
                                &mut (*ContractCertificateChainType).SubCertificates,
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
unsafe extern "C" fn decode_iso20_Dynamic_EVPPTControlModeType(
    stream: &mut ExiBitstream,
    mut Dynamic_EVPPTControlModeType: *mut iso20_Dynamic_EVPPTControlModeType,
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
unsafe extern "C" fn decode_iso20_MeterInfoType(
    stream: &mut ExiBitstream,
    mut MeterInfoType: *mut iso20_MeterInfoType,
) -> i32 {
    let mut grammar_id: i32 = 140 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_MeterInfoType(MeterInfoType);
    while done == 0 {
        match grammar_id {
            140 => {
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
                                        grammar_id = 141 as i32;
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
            141 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint64(
                                stream,
                                &mut (*MeterInfoType).ChargedEnergyReadingWh,
                            );
                            if error == 0 as i32 {
                                grammar_id = 142 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            142 => {
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
                                grammar_id = 143 as i32;
                            }
                        }
                        1 => {
                            error = decode_exi_type_uint64(
                                stream,
                                &mut (*MeterInfoType).CapacitiveEnergyReadingVARh,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_CapacitiveEnergyReadingVARh_isUsed(1 as u32);
                                grammar_id = 144 as i32;
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
                                grammar_id = 145 as i32;
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
                                grammar_id = 146 as i32;
                            }
                        }
                        4 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*MeterInfoType).MeterStatus,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_MeterStatus_isUsed(1 as u32);
                                grammar_id = 147 as i32;
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
            143 => {
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
                                grammar_id = 144 as i32;
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
                                grammar_id = 145 as i32;
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
                                grammar_id = 146 as i32;
                            }
                        }
                        3 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*MeterInfoType).MeterStatus,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_MeterStatus_isUsed(1 as u32);
                                grammar_id = 147 as i32;
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
            144 => {
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
                                grammar_id = 145 as i32;
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
                                grammar_id = 146 as i32;
                            }
                        }
                        2 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*MeterInfoType).MeterStatus,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_MeterStatus_isUsed(1 as u32);
                                grammar_id = 147 as i32;
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
            145 => {
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
                                grammar_id = 146 as i32;
                            }
                        }
                        1 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*MeterInfoType).MeterStatus,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_MeterStatus_isUsed(1 as u32);
                                grammar_id = 147 as i32;
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
            146 => {
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
                                grammar_id = 147 as i32;
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
            147 => {
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
unsafe extern "C" fn decode_iso20_SignatureType(
    stream: &mut ExiBitstream,
    mut SignatureType: *mut iso20_SignatureType,
) -> i32 {
    let mut grammar_id: i32 = 148 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_SignatureType(SignatureType);
    while done == 0 {
        match grammar_id {
            148 => {
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
                            grammar_id = 149 as i32;
                        }
                        1 => {
                            error = decode_iso20_SignedInfoType(
                                stream,
                                &mut (*SignatureType).SignedInfo,
                            );
                            if error == 0 as i32 {
                                grammar_id = 150 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            149 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_SignedInfoType(
                                stream,
                                &mut (*SignatureType).SignedInfo,
                            );
                            if error == 0 as i32 {
                                grammar_id = 150 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            150 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_SignatureValueType(
                                stream,
                                &mut (*SignatureType).SignatureValue,
                            );
                            if error == 0 as i32 {
                                grammar_id = 151 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            151 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_KeyInfoType(stream, &mut (*SignatureType).KeyInfo);
                            if error == 0 as i32 {
                                (*SignatureType).set_KeyInfo_isUsed(1 as u32);
                                grammar_id = 153 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_ObjectType(stream, &mut (*SignatureType).Object);
                            if error == 0 as i32 {
                                (*SignatureType).set_Object_isUsed(1 as u32);
                                grammar_id = 152 as i32;
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
            152 => {
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
            153 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_ObjectType(stream, &mut (*SignatureType).Object);
                            if error == 0 as i32 {
                                (*SignatureType).set_Object_isUsed(1 as u32);
                                grammar_id = 154 as i32;
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
            154 => {
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
unsafe extern "C" fn decode_iso20_Scheduled_EVPPTControlModeType(
    stream: &mut ExiBitstream,
    mut Scheduled_EVPPTControlModeType: *mut iso20_Scheduled_EVPPTControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 155 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_Scheduled_EVPPTControlModeType(Scheduled_EVPPTControlModeType);
    while done == 0 {
        match grammar_id {
            155 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*Scheduled_EVPPTControlModeType).SelectedScheduleTupleID,
                            );
                            if error == 0 as i32 {
                                grammar_id = 156 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            156 => {
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
                                        1 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*Scheduled_EVPPTControlModeType)
                                            .PowerToleranceAcceptance =
                                            value as iso20_powerToleranceAcceptanceType;
                                        (*Scheduled_EVPPTControlModeType)
                                            .set_PowerToleranceAcceptance_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_ReceiptType(
    stream: &mut ExiBitstream,
    mut ReceiptType: *mut iso20_ReceiptType,
) -> i32 {
    let mut grammar_id: i32 = 157 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_ReceiptType(ReceiptType);
    while done == 0 {
        match grammar_id {
            157 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint64(stream, &mut (*ReceiptType).TimeAnchor);
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
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).EnergyCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_EnergyCosts_isUsed(1 as u32);
                                grammar_id = 160 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).OccupancyCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_OccupancyCosts_isUsed(1 as u32);
                                grammar_id = 162 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).AdditionalServicesCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_AdditionalServicesCosts_isUsed(1 as u32);
                                grammar_id = 164 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).OverstayCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_OverstayCosts_isUsed(1 as u32);
                                grammar_id = 166 as i32;
                            }
                        }
                        4 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh28 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh28 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 159 as i32;
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
            159 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh29 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh29 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                grammar_id = 159 as i32;
                            } else {
                                grammar_id = 160 as i32;
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
            160 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).OccupancyCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_OccupancyCosts_isUsed(1 as u32);
                                grammar_id = 162 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).AdditionalServicesCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_AdditionalServicesCosts_isUsed(1 as u32);
                                grammar_id = 164 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).OverstayCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_OverstayCosts_isUsed(1 as u32);
                                grammar_id = 166 as i32;
                            }
                        }
                        3 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh30 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh30 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 161 as i32;
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
            161 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh31 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh31 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                grammar_id = 161 as i32;
                            } else {
                                grammar_id = 162 as i32;
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
            162 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).AdditionalServicesCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_AdditionalServicesCosts_isUsed(1 as u32);
                                grammar_id = 164 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).OverstayCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_OverstayCosts_isUsed(1 as u32);
                                grammar_id = 166 as i32;
                            }
                        }
                        2 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh32 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh32 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 163 as i32;
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
            163 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh33 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh33 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                grammar_id = 163 as i32;
                            } else {
                                grammar_id = 164 as i32;
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
                            error = decode_iso20_DetailedCostType(
                                stream,
                                &mut (*ReceiptType).OverstayCosts,
                            );
                            if error == 0 as i32 {
                                (*ReceiptType).set_OverstayCosts_isUsed(1 as u32);
                                grammar_id = 166 as i32;
                            }
                        }
                        1 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh34 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh34 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 165 as i32;
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
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh35 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh35 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                grammar_id = 165 as i32;
                            } else {
                                grammar_id = 166 as i32;
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
            166 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh36 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh36 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 167 as i32;
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
            167 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                let fresh37 = (*ReceiptType).TaxCosts.arrayLen;
                                (*ReceiptType).TaxCosts.arrayLen =
                                    ((*ReceiptType).TaxCosts.arrayLen).wrapping_add(1);
                                error = decode_iso20_DetailedTaxType(
                                    stream,
                                    &mut *((*ReceiptType).TaxCosts.array)
                                        .as_mut_ptr()
                                        .offset(fresh37 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*ReceiptType).TaxCosts.arrayLen as i32) < 10 as i32 {
                                grammar_id = 167 as i32;
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
unsafe extern "C" fn decode_iso20_AbsolutePriceScheduleType(
    stream: &mut ExiBitstream,
    mut AbsolutePriceScheduleType: *mut iso20_AbsolutePriceScheduleType,
) -> i32 {
    let mut grammar_id: i32 = 168 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_AbsolutePriceScheduleType(AbsolutePriceScheduleType);
    while done == 0 {
        match grammar_id {
            168 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*AbsolutePriceScheduleType).Id.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*AbsolutePriceScheduleType).Id.charactersLen as i32 >= 2 as i32
                                {
                                    (*AbsolutePriceScheduleType).Id.charactersLen =
                                        ((*AbsolutePriceScheduleType).Id.charactersLen as i32
                                            - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*AbsolutePriceScheduleType).Id.charactersLen as usize,
                                        ((*AbsolutePriceScheduleType).Id.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*AbsolutePriceScheduleType).set_Id_isUsed(1 as u32);
                            grammar_id = 169 as i32;
                        }
                        1 => {
                            error = decode_exi_type_uint64(
                                stream,
                                &mut (*AbsolutePriceScheduleType).TimeAnchor,
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
            169 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint64(
                                stream,
                                &mut (*AbsolutePriceScheduleType).TimeAnchor,
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
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*AbsolutePriceScheduleType).PriceScheduleID,
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
                                        &mut (*AbsolutePriceScheduleType)
                                            .PriceScheduleDescription
                                            .charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*AbsolutePriceScheduleType)
                                            .PriceScheduleDescription
                                            .charactersLen
                                            as i32
                                            >= 2 as i32
                                        {
                                            (*AbsolutePriceScheduleType)
                                                .PriceScheduleDescription
                                                .charactersLen = ((*AbsolutePriceScheduleType)
                                                .PriceScheduleDescription
                                                .charactersLen
                                                as i32
                                                - 2 as i32)
                                                as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*AbsolutePriceScheduleType)
                                                    .PriceScheduleDescription
                                                    .charactersLen
                                                    as usize,
                                                ((*AbsolutePriceScheduleType)
                                                    .PriceScheduleDescription
                                                    .characters)
                                                    .as_mut_ptr(),
                                                (160 as i32 + 1 as i32) as usize,
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
                                        (*AbsolutePriceScheduleType)
                                            .set_PriceScheduleDescription_isUsed(1 as u32);
                                        grammar_id = 172 as i32;
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
                                        &mut (*AbsolutePriceScheduleType).Currency.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*AbsolutePriceScheduleType).Currency.charactersLen
                                            as i32
                                            >= 2 as i32
                                        {
                                            (*AbsolutePriceScheduleType).Currency.charactersLen =
                                                ((*AbsolutePriceScheduleType).Currency.charactersLen
                                                    as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*AbsolutePriceScheduleType).Currency.charactersLen
                                                    as usize,
                                                ((*AbsolutePriceScheduleType).Currency.characters)
                                                    .as_mut_ptr(),
                                                (3 as i32 + 1 as i32) as usize,
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
                                        grammar_id = 173 as i32;
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
            172 => {
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
                                        &mut (*AbsolutePriceScheduleType).Currency.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*AbsolutePriceScheduleType).Currency.charactersLen
                                            as i32
                                            >= 2 as i32
                                        {
                                            (*AbsolutePriceScheduleType).Currency.charactersLen =
                                                ((*AbsolutePriceScheduleType).Currency.charactersLen
                                                    as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*AbsolutePriceScheduleType).Currency.charactersLen
                                                    as usize,
                                                ((*AbsolutePriceScheduleType).Currency.characters)
                                                    .as_mut_ptr(),
                                                (3 as i32 + 1 as i32) as usize,
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
                                        grammar_id = 173 as i32;
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
            173 => {
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
                                        &mut (*AbsolutePriceScheduleType).Language.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*AbsolutePriceScheduleType).Language.charactersLen
                                            as i32
                                            >= 2 as i32
                                        {
                                            (*AbsolutePriceScheduleType).Language.charactersLen =
                                                ((*AbsolutePriceScheduleType).Language.charactersLen
                                                    as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*AbsolutePriceScheduleType).Language.charactersLen
                                                    as usize,
                                                ((*AbsolutePriceScheduleType).Language.characters)
                                                    .as_mut_ptr(),
                                                (3 as i32 + 1 as i32) as usize,
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
                                        grammar_id = 174 as i32;
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
            174 => {
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
                                        &mut (*AbsolutePriceScheduleType)
                                            .PriceAlgorithm
                                            .charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*AbsolutePriceScheduleType).PriceAlgorithm.charactersLen
                                            as i32
                                            >= 2 as i32
                                        {
                                            (*AbsolutePriceScheduleType)
                                                .PriceAlgorithm
                                                .charactersLen = ((*AbsolutePriceScheduleType)
                                                .PriceAlgorithm
                                                .charactersLen
                                                as i32
                                                - 2 as i32)
                                                as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*AbsolutePriceScheduleType)
                                                    .PriceAlgorithm
                                                    .charactersLen
                                                    as usize,
                                                ((*AbsolutePriceScheduleType)
                                                    .PriceAlgorithm
                                                    .characters)
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
                                        grammar_id = 175 as i32;
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
            175 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*AbsolutePriceScheduleType).MinimumCost,
                            );
                            if error == 0 as i32 {
                                (*AbsolutePriceScheduleType).set_MinimumCost_isUsed(1 as u32);
                                grammar_id = 176 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*AbsolutePriceScheduleType).MaximumCost,
                            );
                            if error == 0 as i32 {
                                (*AbsolutePriceScheduleType).set_MaximumCost_isUsed(1 as u32);
                                grammar_id = 177 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_TaxRuleListType(
                                stream,
                                &mut (*AbsolutePriceScheduleType).TaxRules,
                            );
                            if error == 0 as i32 {
                                (*AbsolutePriceScheduleType).set_TaxRules_isUsed(1 as u32);
                                grammar_id = 178 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_PriceRuleStackListType(
                                stream,
                                &mut (*AbsolutePriceScheduleType).PriceRuleStacks,
                            );
                            if error == 0 as i32 {
                                grammar_id = 179 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            176 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*AbsolutePriceScheduleType).MaximumCost,
                            );
                            if error == 0 as i32 {
                                (*AbsolutePriceScheduleType).set_MaximumCost_isUsed(1 as u32);
                                grammar_id = 177 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_TaxRuleListType(
                                stream,
                                &mut (*AbsolutePriceScheduleType).TaxRules,
                            );
                            if error == 0 as i32 {
                                (*AbsolutePriceScheduleType).set_TaxRules_isUsed(1 as u32);
                                grammar_id = 178 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_PriceRuleStackListType(
                                stream,
                                &mut (*AbsolutePriceScheduleType).PriceRuleStacks,
                            );
                            if error == 0 as i32 {
                                grammar_id = 179 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            177 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_TaxRuleListType(
                                stream,
                                &mut (*AbsolutePriceScheduleType).TaxRules,
                            );
                            if error == 0 as i32 {
                                (*AbsolutePriceScheduleType).set_TaxRules_isUsed(1 as u32);
                                grammar_id = 178 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_PriceRuleStackListType(
                                stream,
                                &mut (*AbsolutePriceScheduleType).PriceRuleStacks,
                            );
                            if error == 0 as i32 {
                                grammar_id = 179 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            178 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_PriceRuleStackListType(
                                stream,
                                &mut (*AbsolutePriceScheduleType).PriceRuleStacks,
                            );
                            if error == 0 as i32 {
                                grammar_id = 179 as i32;
                            }
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
                            error = decode_iso20_OverstayRuleListType(
                                stream,
                                &mut (*AbsolutePriceScheduleType).OverstayRules,
                            );
                            if error == 0 as i32 {
                                (*AbsolutePriceScheduleType).set_OverstayRules_isUsed(1 as u32);
                                grammar_id = 180 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_AdditionalServiceListType(
                                stream,
                                &mut (*AbsolutePriceScheduleType).AdditionalSelectedServices,
                            );
                            if error == 0 as i32 {
                                (*AbsolutePriceScheduleType)
                                    .set_AdditionalSelectedServices_isUsed(1 as u32);
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
                            error = decode_iso20_AdditionalServiceListType(
                                stream,
                                &mut (*AbsolutePriceScheduleType).AdditionalSelectedServices,
                            );
                            if error == 0 as i32 {
                                (*AbsolutePriceScheduleType)
                                    .set_AdditionalSelectedServices_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_EVPowerProfileEntryListType(
    stream: &mut ExiBitstream,
    mut EVPowerProfileEntryListType: *mut iso20_EVPowerProfileEntryListType,
) -> i32 {
    let mut grammar_id: i32 = 181 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_EVPowerProfileEntryListType(EVPowerProfileEntryListType);
    while done == 0 {
        match grammar_id {
            181 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*EVPowerProfileEntryListType).EVPowerProfileEntry.arrayLen as i32)
                                < 2048 as i32
                            {
                                let fresh38 =
                                    (*EVPowerProfileEntryListType).EVPowerProfileEntry.arrayLen;
                                (*EVPowerProfileEntryListType).EVPowerProfileEntry.arrayLen =
                                    ((*EVPowerProfileEntryListType).EVPowerProfileEntry.arrayLen)
                                        .wrapping_add(1);
                                error = decode_iso20_PowerScheduleEntryType(
                                    stream,
                                    &mut *((*EVPowerProfileEntryListType)
                                        .EVPowerProfileEntry
                                        .array)
                                        .as_mut_ptr()
                                        .offset(fresh38 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 182 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            182 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*EVPowerProfileEntryListType).EVPowerProfileEntry.arrayLen as i32)
                                < 2048 as i32
                            {
                                let fresh39 =
                                    (*EVPowerProfileEntryListType).EVPowerProfileEntry.arrayLen;
                                (*EVPowerProfileEntryListType).EVPowerProfileEntry.arrayLen =
                                    ((*EVPowerProfileEntryListType).EVPowerProfileEntry.arrayLen)
                                        .wrapping_add(1);
                                error = decode_iso20_PowerScheduleEntryType(
                                    stream,
                                    &mut *((*EVPowerProfileEntryListType)
                                        .EVPowerProfileEntry
                                        .array)
                                        .as_mut_ptr()
                                        .offset(fresh39 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*EVPowerProfileEntryListType).EVPowerProfileEntry.arrayLen as i32)
                                < 2048 as i32
                            {
                                grammar_id = 182 as i32;
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
unsafe extern "C" fn decode_iso20_Dynamic_SMDTControlModeType(
    stream: &mut ExiBitstream,
    mut Dynamic_SMDTControlModeType: *mut iso20_Dynamic_SMDTControlModeType,
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
unsafe extern "C" fn decode_iso20_EVEnergyOfferType(
    stream: &mut ExiBitstream,
    mut EVEnergyOfferType: *mut iso20_EVEnergyOfferType,
) -> i32 {
    let mut grammar_id: i32 = 183 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_EVEnergyOfferType(EVEnergyOfferType);
    while done == 0 {
        match grammar_id {
            183 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_EVPowerScheduleType(
                                stream,
                                &mut (*EVEnergyOfferType).EVPowerSchedule,
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
                            error = decode_iso20_EVAbsolutePriceScheduleType(
                                stream,
                                &mut (*EVEnergyOfferType).EVAbsolutePriceSchedule,
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
unsafe extern "C" fn decode_iso20_PriceLevelScheduleType(
    stream: &mut ExiBitstream,
    mut PriceLevelScheduleType: *mut iso20_PriceLevelScheduleType,
) -> i32 {
    let mut grammar_id: i32 = 185 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_PriceLevelScheduleType(PriceLevelScheduleType);
    while done == 0 {
        match grammar_id {
            185 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*PriceLevelScheduleType).Id.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*PriceLevelScheduleType).Id.charactersLen as i32 >= 2 as i32 {
                                    (*PriceLevelScheduleType).Id.charactersLen =
                                        ((*PriceLevelScheduleType).Id.charactersLen as i32
                                            - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*PriceLevelScheduleType).Id.charactersLen as usize,
                                        ((*PriceLevelScheduleType).Id.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*PriceLevelScheduleType).set_Id_isUsed(1 as u32);
                            grammar_id = 186 as i32;
                        }
                        1 => {
                            error = decode_exi_type_uint64(
                                stream,
                                &mut (*PriceLevelScheduleType).TimeAnchor,
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
                            error = decode_exi_type_uint64(
                                stream,
                                &mut (*PriceLevelScheduleType).TimeAnchor,
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
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*PriceLevelScheduleType).PriceScheduleID,
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
                                        &mut (*PriceLevelScheduleType)
                                            .PriceScheduleDescription
                                            .charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*PriceLevelScheduleType)
                                            .PriceScheduleDescription
                                            .charactersLen
                                            as i32
                                            >= 2 as i32
                                        {
                                            (*PriceLevelScheduleType)
                                                .PriceScheduleDescription
                                                .charactersLen = ((*PriceLevelScheduleType)
                                                .PriceScheduleDescription
                                                .charactersLen
                                                as i32
                                                - 2 as i32)
                                                as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*PriceLevelScheduleType)
                                                    .PriceScheduleDescription
                                                    .charactersLen
                                                    as usize,
                                                ((*PriceLevelScheduleType)
                                                    .PriceScheduleDescription
                                                    .characters)
                                                    .as_mut_ptr(),
                                                (160 as i32 + 1 as i32) as usize,
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
                                        (*PriceLevelScheduleType)
                                            .set_PriceScheduleDescription_isUsed(1 as u32);
                                        grammar_id = 189 as i32;
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
                                    let mut value: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        8 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*PriceLevelScheduleType).NumberOfPriceLevels = value as u8;
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
                                        grammar_id = 190 as i32;
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
                                        8 as i32 as usize,
                                        &mut value_0,
                                    );
                                    if error == 0 as i32 {
                                        (*PriceLevelScheduleType).NumberOfPriceLevels =
                                            value_0 as u8;
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
                                        grammar_id = 190 as i32;
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
                            error = decode_iso20_PriceLevelScheduleEntryListType(
                                stream,
                                &mut (*PriceLevelScheduleType).PriceLevelScheduleEntries,
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
unsafe extern "C" fn decode_iso20_ChargingScheduleType(
    stream: &mut ExiBitstream,
    mut ChargingScheduleType: *mut iso20_ChargingScheduleType,
) -> i32 {
    let mut grammar_id: i32 = 191 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_ChargingScheduleType(ChargingScheduleType);
    while done == 0 {
        match grammar_id {
            191 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_PowerScheduleType(
                                stream,
                                &mut (*ChargingScheduleType).PowerSchedule,
                            );
                            if error == 0 as i32 {
                                grammar_id = 192 as i32;
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
                            error = decode_iso20_AbsolutePriceScheduleType(
                                stream,
                                &mut (*ChargingScheduleType).AbsolutePriceSchedule,
                            );
                            if error == 0 as i32 {
                                (*ChargingScheduleType).set_AbsolutePriceSchedule_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_PriceLevelScheduleType(
                                stream,
                                &mut (*ChargingScheduleType).PriceLevelSchedule,
                            );
                            if error == 0 as i32 {
                                (*ChargingScheduleType).set_PriceLevelSchedule_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_ScheduleTupleType(
    stream: &mut ExiBitstream,
    mut ScheduleTupleType: *mut iso20_ScheduleTupleType,
) -> i32 {
    let mut grammar_id: i32 = 193 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_ScheduleTupleType(ScheduleTupleType);
    while done == 0 {
        match grammar_id {
            193 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*ScheduleTupleType).ScheduleTupleID,
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
                            error = decode_iso20_ChargingScheduleType(
                                stream,
                                &mut (*ScheduleTupleType).ChargingSchedule,
                            );
                            if error == 0 as i32 {
                                grammar_id = 195 as i32;
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
                            error = decode_iso20_ChargingScheduleType(
                                stream,
                                &mut (*ScheduleTupleType).DischargingSchedule,
                            );
                            if error == 0 as i32 {
                                (*ScheduleTupleType).set_DischargingSchedule_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_Scheduled_SMDTControlModeType(
    stream: &mut ExiBitstream,
    mut Scheduled_SMDTControlModeType: *mut iso20_Scheduled_SMDTControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 196 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_Scheduled_SMDTControlModeType(Scheduled_SMDTControlModeType);
    while done == 0 {
        match grammar_id {
            196 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*Scheduled_SMDTControlModeType).SelectedScheduleTupleID,
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
unsafe extern "C" fn decode_iso20_MessageHeaderType(
    stream: &mut ExiBitstream,
    mut MessageHeaderType: *mut iso20_MessageHeaderType,
) -> i32 {
    let mut grammar_id: i32 = 197 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_MessageHeaderType(MessageHeaderType);
    while done == 0 {
        match grammar_id {
            197 => {
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
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error =
                                decode_exi_type_uint64(stream, &mut (*MessageHeaderType).TimeStamp);
                            if error == 0 as i32 {
                                grammar_id = 199 as i32;
                            }
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
                            error = decode_iso20_SignatureType(
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
unsafe extern "C" fn decode_iso20_SignaturePropertyType(
    stream: &mut ExiBitstream,
    mut SignaturePropertyType: *mut iso20_SignaturePropertyType,
) -> i32 {
    let mut grammar_id: i32 = 200 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_SignaturePropertyType(SignaturePropertyType);
    while done == 0 {
        match grammar_id {
            200 => {
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
                            grammar_id = 201 as i32;
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
                            grammar_id = 202 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            201 => {
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
                            grammar_id = 202 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            202 => {
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
unsafe extern "C" fn decode_iso20_ServiceIDListType(
    stream: &mut ExiBitstream,
    mut ServiceIDListType: *mut iso20_ServiceIDListType,
) -> i32 {
    let mut grammar_id: i32 = 203 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_ServiceIDListType(ServiceIDListType);
    while done == 0 {
        match grammar_id {
            203 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ServiceIDListType).ServiceID.arrayLen as i32) < 16 as i32 {
                                let fresh40 = (*ServiceIDListType).ServiceID.arrayLen;
                                (*ServiceIDListType).ServiceID.arrayLen =
                                    ((*ServiceIDListType).ServiceID.arrayLen).wrapping_add(1);
                                error = decode_exi_type_uint16(
                                    stream,
                                    &mut *((*ServiceIDListType).ServiceID.array)
                                        .as_mut_ptr()
                                        .offset(fresh40 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 204 as i32;
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
                            if ((*ServiceIDListType).ServiceID.arrayLen as i32) < 16 as i32 {
                                let fresh41 = (*ServiceIDListType).ServiceID.arrayLen;
                                (*ServiceIDListType).ServiceID.arrayLen =
                                    ((*ServiceIDListType).ServiceID.arrayLen).wrapping_add(1);
                                error = decode_exi_type_uint16(
                                    stream,
                                    &mut *((*ServiceIDListType).ServiceID.array)
                                        .as_mut_ptr()
                                        .offset(fresh41 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 204 as i32;
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
unsafe extern "C" fn decode_iso20_SelectedServiceType(
    stream: &mut ExiBitstream,
    mut SelectedServiceType: *mut iso20_SelectedServiceType,
) -> i32 {
    let mut grammar_id: i32 = 205 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_SelectedServiceType(SelectedServiceType);
    while done == 0 {
        match grammar_id {
            205 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*SelectedServiceType).ServiceID,
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
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*SelectedServiceType).ParameterSetID,
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
unsafe extern "C" fn decode_iso20_SignedMeteringDataType(
    stream: &mut ExiBitstream,
    mut SignedMeteringDataType: *mut iso20_SignedMeteringDataType,
) -> i32 {
    let mut grammar_id: i32 = 207 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_SignedMeteringDataType(SignedMeteringDataType);
    while done == 0 {
        match grammar_id {
            207 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*SignedMeteringDataType).Id.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*SignedMeteringDataType).Id.charactersLen as i32 >= 2 as i32 {
                                    (*SignedMeteringDataType).Id.charactersLen =
                                        ((*SignedMeteringDataType).Id.charactersLen as i32
                                            - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*SignedMeteringDataType).Id.charactersLen as usize,
                                        ((*SignedMeteringDataType).Id.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            grammar_id = 208 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            208 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*SignedMeteringDataType).SessionID.bytesLen,
                                &mut *((*SignedMeteringDataType).SessionID.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                8 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 209 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            209 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_MeterInfoType(
                                stream,
                                &mut (*SignedMeteringDataType).MeterInfo,
                            );
                            if error == 0 as i32 {
                                grammar_id = 210 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            210 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_ReceiptType(
                                stream,
                                &mut (*SignedMeteringDataType).Receipt,
                            );
                            if error == 0 as i32 {
                                (*SignedMeteringDataType).set_Receipt_isUsed(1 as u32);
                                grammar_id = 211 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_Dynamic_SMDTControlModeType(
                                stream,
                                &mut (*SignedMeteringDataType).Dynamic_SMDTControlMode,
                            );
                            if error == 0 as i32 {
                                (*SignedMeteringDataType)
                                    .set_Dynamic_SMDTControlMode_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_Scheduled_SMDTControlModeType(
                                stream,
                                &mut (*SignedMeteringDataType).Scheduled_SMDTControlMode,
                            );
                            if error == 0 as i32 {
                                (*SignedMeteringDataType)
                                    .set_Scheduled_SMDTControlMode_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            211 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_Dynamic_SMDTControlModeType(
                                stream,
                                &mut (*SignedMeteringDataType).Dynamic_SMDTControlMode,
                            );
                            if error == 0 as i32 {
                                (*SignedMeteringDataType)
                                    .set_Dynamic_SMDTControlMode_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_Scheduled_SMDTControlModeType(
                                stream,
                                &mut (*SignedMeteringDataType).Scheduled_SMDTControlMode,
                            );
                            if error == 0 as i32 {
                                (*SignedMeteringDataType)
                                    .set_Scheduled_SMDTControlMode_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_SignedCertificateChainType(
    stream: &mut ExiBitstream,
    mut SignedCertificateChainType: *mut iso20_SignedCertificateChainType,
) -> i32 {
    let mut grammar_id: i32 = 212 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_SignedCertificateChainType(SignedCertificateChainType);
    while done == 0 {
        match grammar_id {
            212 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*SignedCertificateChainType).Id.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*SignedCertificateChainType).Id.charactersLen as i32 >= 2 as i32
                                {
                                    (*SignedCertificateChainType).Id.charactersLen =
                                        ((*SignedCertificateChainType).Id.charactersLen as i32
                                            - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*SignedCertificateChainType).Id.charactersLen as usize,
                                        ((*SignedCertificateChainType).Id.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            grammar_id = 213 as i32;
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
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*SignedCertificateChainType).Certificate.bytesLen,
                                &mut *((*SignedCertificateChainType).Certificate.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                1600 as i32 as usize,
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
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_SubCertificatesType(
                                stream,
                                &mut (*SignedCertificateChainType).SubCertificates,
                            );
                            if error == 0 as i32 {
                                (*SignedCertificateChainType).set_SubCertificates_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_EIM_AReqAuthorizationModeType(
    stream: &mut ExiBitstream,
    mut EIM_AReqAuthorizationModeType: *mut iso20_EIM_AReqAuthorizationModeType,
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
unsafe extern "C" fn decode_iso20_SelectedServiceListType(
    stream: &mut ExiBitstream,
    mut SelectedServiceListType: *mut iso20_SelectedServiceListType,
) -> i32 {
    let mut grammar_id: i32 = 215 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_SelectedServiceListType(SelectedServiceListType);
    while done == 0 {
        match grammar_id {
            215 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SelectedServiceListType).SelectedService.arrayLen as i32)
                                < 16 as i32
                            {
                                let fresh42 = (*SelectedServiceListType).SelectedService.arrayLen;
                                (*SelectedServiceListType).SelectedService.arrayLen =
                                    ((*SelectedServiceListType).SelectedService.arrayLen)
                                        .wrapping_add(1);
                                error = decode_iso20_SelectedServiceType(
                                    stream,
                                    &mut *((*SelectedServiceListType).SelectedService.array)
                                        .as_mut_ptr()
                                        .offset(fresh42 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 216 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            216 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SelectedServiceListType).SelectedService.arrayLen as i32)
                                < 16 as i32
                            {
                                let fresh43 = (*SelectedServiceListType).SelectedService.arrayLen;
                                (*SelectedServiceListType).SelectedService.arrayLen =
                                    ((*SelectedServiceListType).SelectedService.arrayLen)
                                        .wrapping_add(1);
                                error = decode_iso20_SelectedServiceType(
                                    stream,
                                    &mut *((*SelectedServiceListType).SelectedService.array)
                                        .as_mut_ptr()
                                        .offset(fresh43 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*SelectedServiceListType).SelectedService.arrayLen as i32)
                                < 16 as i32
                            {
                                grammar_id = 216 as i32;
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
unsafe extern "C" fn decode_iso20_Dynamic_SEReqControlModeType(
    stream: &mut ExiBitstream,
    mut Dynamic_SEReqControlModeType: *mut iso20_Dynamic_SEReqControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 217 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_Dynamic_SEReqControlModeType(Dynamic_SEReqControlModeType);
    while done == 0 {
        match grammar_id {
            217 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*Dynamic_SEReqControlModeType).DepartureTime,
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
                                        (*Dynamic_SEReqControlModeType).MinimumSOC =
                                            value as int8_t;
                                        (*Dynamic_SEReqControlModeType)
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
                                        grammar_id = 219 as i32;
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
                                        (*Dynamic_SEReqControlModeType).TargetSOC =
                                            value_0 as int8_t;
                                        (*Dynamic_SEReqControlModeType)
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
                                        grammar_id = 220 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        2 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*Dynamic_SEReqControlModeType).EVTargetEnergyRequest,
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
            219 => {
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
                                    let mut value_1: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_1,
                                    );
                                    if error == 0 as i32 {
                                        (*Dynamic_SEReqControlModeType).TargetSOC =
                                            value_1 as int8_t;
                                        (*Dynamic_SEReqControlModeType)
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
                                        grammar_id = 220 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*Dynamic_SEReqControlModeType).EVTargetEnergyRequest,
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
            220 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*Dynamic_SEReqControlModeType).EVTargetEnergyRequest,
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
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*Dynamic_SEReqControlModeType).EVMaximumEnergyRequest,
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
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*Dynamic_SEReqControlModeType).EVMinimumEnergyRequest,
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
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*Dynamic_SEReqControlModeType).EVMaximumV2XEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*Dynamic_SEReqControlModeType)
                                    .set_EVMaximumV2XEnergyRequest_isUsed(1 as u32);
                                grammar_id = 224 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*Dynamic_SEReqControlModeType).EVMinimumV2XEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*Dynamic_SEReqControlModeType)
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
            224 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*Dynamic_SEReqControlModeType).EVMinimumV2XEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*Dynamic_SEReqControlModeType)
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
unsafe extern "C" fn decode_iso20_EVSEStatusType(
    stream: &mut ExiBitstream,
    mut EVSEStatusType: *mut iso20_EVSEStatusType,
) -> i32 {
    let mut grammar_id: i32 = 225 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_EVSEStatusType(EVSEStatusType);
    while done == 0 {
        match grammar_id {
            225 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*EVSEStatusType).NotificationMaxDelay,
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
                                            value as iso20_evseNotificationType;
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
unsafe extern "C" fn decode_iso20_ListOfRootCertificateIDsType(
    stream: &mut ExiBitstream,
    mut ListOfRootCertificateIDsType: *mut iso20_ListOfRootCertificateIDsType,
) -> i32 {
    let mut grammar_id: i32 = 227 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_ListOfRootCertificateIDsType(ListOfRootCertificateIDsType);
    while done == 0 {
        match grammar_id {
            227 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ListOfRootCertificateIDsType).RootCertificateID.arrayLen as i32)
                                < 20 as i32
                            {
                                let fresh44 =
                                    (*ListOfRootCertificateIDsType).RootCertificateID.arrayLen;
                                (*ListOfRootCertificateIDsType).RootCertificateID.arrayLen =
                                    ((*ListOfRootCertificateIDsType).RootCertificateID.arrayLen)
                                        .wrapping_add(1);
                                error = decode_iso20_X509IssuerSerialType(
                                    stream,
                                    &mut *((*ListOfRootCertificateIDsType).RootCertificateID.array)
                                        .as_mut_ptr()
                                        .offset(fresh44 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 228 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            228 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ListOfRootCertificateIDsType).RootCertificateID.arrayLen as i32)
                                < 20 as i32
                            {
                                let fresh45 =
                                    (*ListOfRootCertificateIDsType).RootCertificateID.arrayLen;
                                (*ListOfRootCertificateIDsType).RootCertificateID.arrayLen =
                                    ((*ListOfRootCertificateIDsType).RootCertificateID.arrayLen)
                                        .wrapping_add(1);
                                error = decode_iso20_X509IssuerSerialType(
                                    stream,
                                    &mut *((*ListOfRootCertificateIDsType).RootCertificateID.array)
                                        .as_mut_ptr()
                                        .offset(fresh45 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*ListOfRootCertificateIDsType).RootCertificateID.arrayLen as i32)
                                < 20 as i32
                            {
                                grammar_id = 228 as i32;
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
unsafe extern "C" fn decode_iso20_PnC_AReqAuthorizationModeType(
    stream: &mut ExiBitstream,
    mut PnC_AReqAuthorizationModeType: *mut iso20_PnC_AReqAuthorizationModeType,
) -> i32 {
    let mut grammar_id: i32 = 229 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_PnC_AReqAuthorizationModeType(PnC_AReqAuthorizationModeType);
    while done == 0 {
        match grammar_id {
            229 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*PnC_AReqAuthorizationModeType).Id.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*PnC_AReqAuthorizationModeType).Id.charactersLen as i32
                                    >= 2 as i32
                                {
                                    (*PnC_AReqAuthorizationModeType).Id.charactersLen =
                                        ((*PnC_AReqAuthorizationModeType).Id.charactersLen as i32
                                            - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*PnC_AReqAuthorizationModeType).Id.charactersLen as usize,
                                        ((*PnC_AReqAuthorizationModeType).Id.characters)
                                            .as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            grammar_id = 230 as i32;
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
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*PnC_AReqAuthorizationModeType).GenChallenge.bytesLen,
                                &mut *((*PnC_AReqAuthorizationModeType).GenChallenge.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                16 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 231 as i32;
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
                            error = decode_iso20_ContractCertificateChainType(
                                stream,
                                &mut (*PnC_AReqAuthorizationModeType).ContractCertificateChain,
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
unsafe extern "C" fn decode_iso20_ServiceListType(
    stream: &mut ExiBitstream,
    mut ServiceListType: *mut iso20_ServiceListType,
) -> i32 {
    let mut grammar_id: i32 = 232 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_ServiceListType(ServiceListType);
    while done == 0 {
        match grammar_id {
            232 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ServiceListType).Service.arrayLen as i32) < 8 as i32 {
                                let fresh46 = (*ServiceListType).Service.arrayLen;
                                (*ServiceListType).Service.arrayLen =
                                    ((*ServiceListType).Service.arrayLen).wrapping_add(1);
                                error = decode_iso20_ServiceType(
                                    stream,
                                    &mut *((*ServiceListType).Service.array)
                                        .as_mut_ptr()
                                        .offset(fresh46 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 233 as i32;
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
                            if ((*ServiceListType).Service.arrayLen as i32) < 8 as i32 {
                                let fresh47 = (*ServiceListType).Service.arrayLen;
                                (*ServiceListType).Service.arrayLen =
                                    ((*ServiceListType).Service.arrayLen).wrapping_add(1);
                                error = decode_iso20_ServiceType(
                                    stream,
                                    &mut *((*ServiceListType).Service.array)
                                        .as_mut_ptr()
                                        .offset(fresh47 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*ServiceListType).Service.arrayLen as i32) < 8 as i32 {
                                grammar_id = 233 as i32;
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
unsafe extern "C" fn decode_iso20_ServiceParameterListType(
    stream: &mut ExiBitstream,
    mut ServiceParameterListType: *mut iso20_ServiceParameterListType,
) -> i32 {
    let mut grammar_id: i32 = 234 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_ServiceParameterListType(ServiceParameterListType);
    while done == 0 {
        match grammar_id {
            234 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ServiceParameterListType).ParameterSet.arrayLen as i32) < 4 as i32
                            {
                                let fresh48 = (*ServiceParameterListType).ParameterSet.arrayLen;
                                (*ServiceParameterListType).ParameterSet.arrayLen =
                                    ((*ServiceParameterListType).ParameterSet.arrayLen)
                                        .wrapping_add(1);
                                error = decode_iso20_ParameterSetType(
                                    stream,
                                    &mut *((*ServiceParameterListType).ParameterSet.array)
                                        .as_mut_ptr()
                                        .offset(fresh48 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 235 as i32;
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
                            if ((*ServiceParameterListType).ParameterSet.arrayLen as i32) < 4 as i32
                            {
                                let fresh49 = (*ServiceParameterListType).ParameterSet.arrayLen;
                                (*ServiceParameterListType).ParameterSet.arrayLen =
                                    ((*ServiceParameterListType).ParameterSet.arrayLen)
                                        .wrapping_add(1);
                                error = decode_iso20_ParameterSetType(
                                    stream,
                                    &mut *((*ServiceParameterListType).ParameterSet.array)
                                        .as_mut_ptr()
                                        .offset(fresh49 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*ServiceParameterListType).ParameterSet.arrayLen as i32)
                                < 32 as i32
                            {
                                grammar_id = 235 as i32;
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
unsafe extern "C" fn decode_iso20_Scheduled_SEReqControlModeType(
    stream: &mut ExiBitstream,
    mut Scheduled_SEReqControlModeType: *mut iso20_Scheduled_SEReqControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 236 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_Scheduled_SEReqControlModeType(Scheduled_SEReqControlModeType);
    while done == 0 {
        match grammar_id {
            236 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*Scheduled_SEReqControlModeType).DepartureTime,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_SEReqControlModeType)
                                    .set_DepartureTime_isUsed(1 as u32);
                                grammar_id = 237 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*Scheduled_SEReqControlModeType).EVTargetEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_SEReqControlModeType)
                                    .set_EVTargetEnergyRequest_isUsed(1 as u32);
                                grammar_id = 238 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*Scheduled_SEReqControlModeType).EVMaximumEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_SEReqControlModeType)
                                    .set_EVMaximumEnergyRequest_isUsed(1 as u32);
                                grammar_id = 239 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*Scheduled_SEReqControlModeType).EVMinimumEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_SEReqControlModeType)
                                    .set_EVMinimumEnergyRequest_isUsed(1 as u32);
                                grammar_id = 240 as i32;
                            }
                        }
                        4 => {
                            error = decode_iso20_EVEnergyOfferType(
                                stream,
                                &mut (*Scheduled_SEReqControlModeType).EVEnergyOffer,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_SEReqControlModeType)
                                    .set_EVEnergyOffer_isUsed(1 as u32);
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
            237 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*Scheduled_SEReqControlModeType).EVTargetEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_SEReqControlModeType)
                                    .set_EVTargetEnergyRequest_isUsed(1 as u32);
                                grammar_id = 238 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*Scheduled_SEReqControlModeType).EVMaximumEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_SEReqControlModeType)
                                    .set_EVMaximumEnergyRequest_isUsed(1 as u32);
                                grammar_id = 239 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*Scheduled_SEReqControlModeType).EVMinimumEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_SEReqControlModeType)
                                    .set_EVMinimumEnergyRequest_isUsed(1 as u32);
                                grammar_id = 240 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_EVEnergyOfferType(
                                stream,
                                &mut (*Scheduled_SEReqControlModeType).EVEnergyOffer,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_SEReqControlModeType)
                                    .set_EVEnergyOffer_isUsed(1 as u32);
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
            238 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*Scheduled_SEReqControlModeType).EVMaximumEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_SEReqControlModeType)
                                    .set_EVMaximumEnergyRequest_isUsed(1 as u32);
                                grammar_id = 239 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*Scheduled_SEReqControlModeType).EVMinimumEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_SEReqControlModeType)
                                    .set_EVMinimumEnergyRequest_isUsed(1 as u32);
                                grammar_id = 240 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_EVEnergyOfferType(
                                stream,
                                &mut (*Scheduled_SEReqControlModeType).EVEnergyOffer,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_SEReqControlModeType)
                                    .set_EVEnergyOffer_isUsed(1 as u32);
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
            239 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_RationalNumberType(
                                stream,
                                &mut (*Scheduled_SEReqControlModeType).EVMinimumEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_SEReqControlModeType)
                                    .set_EVMinimumEnergyRequest_isUsed(1 as u32);
                                grammar_id = 240 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_EVEnergyOfferType(
                                stream,
                                &mut (*Scheduled_SEReqControlModeType).EVEnergyOffer,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_SEReqControlModeType)
                                    .set_EVEnergyOffer_isUsed(1 as u32);
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
            240 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_EVEnergyOfferType(
                                stream,
                                &mut (*Scheduled_SEReqControlModeType).EVEnergyOffer,
                            );
                            if error == 0 as i32 {
                                (*Scheduled_SEReqControlModeType)
                                    .set_EVEnergyOffer_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_EVPowerProfileType(
    stream: &mut ExiBitstream,
    mut EVPowerProfileType: *mut iso20_EVPowerProfileType,
) -> i32 {
    let mut grammar_id: i32 = 241 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_EVPowerProfileType(EVPowerProfileType);
    while done == 0 {
        match grammar_id {
            241 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint64(
                                stream,
                                &mut (*EVPowerProfileType).TimeAnchor,
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
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_Dynamic_EVPPTControlModeType(
                                stream,
                                &mut (*EVPowerProfileType).Dynamic_EVPPTControlMode,
                            );
                            if error == 0 as i32 {
                                (*EVPowerProfileType).set_Dynamic_EVPPTControlMode_isUsed(1 as u32);
                                grammar_id = 243 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_Scheduled_EVPPTControlModeType(
                                stream,
                                &mut (*EVPowerProfileType).Scheduled_EVPPTControlMode,
                            );
                            if error == 0 as i32 {
                                (*EVPowerProfileType)
                                    .set_Scheduled_EVPPTControlMode_isUsed(1 as u32);
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
                            error = decode_iso20_EVPowerProfileEntryListType(
                                stream,
                                &mut (*EVPowerProfileType).EVPowerProfileEntries,
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
unsafe extern "C" fn decode_iso20_CertificateChainType(
    stream: &mut ExiBitstream,
    mut CertificateChainType: *mut iso20_CertificateChainType,
) -> i32 {
    let mut grammar_id: i32 = 244 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_CertificateChainType(CertificateChainType);
    while done == 0 {
        match grammar_id {
            244 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*CertificateChainType).Certificate.bytesLen,
                                &mut *((*CertificateChainType).Certificate.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                1600 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 245 as i32;
                            }
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
                            error = decode_iso20_SubCertificatesType(
                                stream,
                                &mut (*CertificateChainType).SubCertificates,
                            );
                            if error == 0 as i32 {
                                (*CertificateChainType).set_SubCertificates_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_EIM_ASResAuthorizationModeType(
    stream: &mut ExiBitstream,
    mut EIM_ASResAuthorizationModeType: *mut iso20_EIM_ASResAuthorizationModeType,
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
unsafe extern "C" fn decode_iso20_Dynamic_SEResControlModeType(
    stream: &mut ExiBitstream,
    mut Dynamic_SEResControlModeType: *mut iso20_Dynamic_SEResControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 246 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_Dynamic_SEResControlModeType(Dynamic_SEResControlModeType);
    while done == 0 {
        match grammar_id {
            246 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*Dynamic_SEResControlModeType).DepartureTime,
                            );
                            if error == 0 as i32 {
                                (*Dynamic_SEResControlModeType).set_DepartureTime_isUsed(1 as u32);
                                grammar_id = 247 as i32;
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
                                        (*Dynamic_SEResControlModeType).MinimumSOC =
                                            value as int8_t;
                                        (*Dynamic_SEResControlModeType)
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
                                        grammar_id = 248 as i32;
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
                                        (*Dynamic_SEResControlModeType).TargetSOC =
                                            value_0 as int8_t;
                                        (*Dynamic_SEResControlModeType)
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
                                        grammar_id = 249 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        3 => {
                            error = decode_iso20_AbsolutePriceScheduleType(
                                stream,
                                &mut (*Dynamic_SEResControlModeType).AbsolutePriceSchedule,
                            );
                            if error == 0 as i32 {
                                (*Dynamic_SEResControlModeType)
                                    .set_AbsolutePriceSchedule_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        4 => {
                            error = decode_iso20_PriceLevelScheduleType(
                                stream,
                                &mut (*Dynamic_SEResControlModeType).PriceLevelSchedule,
                            );
                            if error == 0 as i32 {
                                (*Dynamic_SEResControlModeType)
                                    .set_PriceLevelSchedule_isUsed(1 as u32);
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
            247 => {
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
                                        (*Dynamic_SEResControlModeType).MinimumSOC =
                                            value_1 as int8_t;
                                        (*Dynamic_SEResControlModeType)
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
                                        grammar_id = 248 as i32;
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
                                        (*Dynamic_SEResControlModeType).TargetSOC =
                                            value_2 as int8_t;
                                        (*Dynamic_SEResControlModeType)
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
                                        grammar_id = 249 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        2 => {
                            error = decode_iso20_AbsolutePriceScheduleType(
                                stream,
                                &mut (*Dynamic_SEResControlModeType).AbsolutePriceSchedule,
                            );
                            if error == 0 as i32 {
                                (*Dynamic_SEResControlModeType)
                                    .set_AbsolutePriceSchedule_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        3 => {
                            error = decode_iso20_PriceLevelScheduleType(
                                stream,
                                &mut (*Dynamic_SEResControlModeType).PriceLevelSchedule,
                            );
                            if error == 0 as i32 {
                                (*Dynamic_SEResControlModeType)
                                    .set_PriceLevelSchedule_isUsed(1 as u32);
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
            248 => {
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
                                    let mut value_3: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_3,
                                    );
                                    if error == 0 as i32 {
                                        (*Dynamic_SEResControlModeType).TargetSOC =
                                            value_3 as int8_t;
                                        (*Dynamic_SEResControlModeType)
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
                                        grammar_id = 249 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            error = decode_iso20_AbsolutePriceScheduleType(
                                stream,
                                &mut (*Dynamic_SEResControlModeType).AbsolutePriceSchedule,
                            );
                            if error == 0 as i32 {
                                (*Dynamic_SEResControlModeType)
                                    .set_AbsolutePriceSchedule_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_PriceLevelScheduleType(
                                stream,
                                &mut (*Dynamic_SEResControlModeType).PriceLevelSchedule,
                            );
                            if error == 0 as i32 {
                                (*Dynamic_SEResControlModeType)
                                    .set_PriceLevelSchedule_isUsed(1 as u32);
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
            249 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_AbsolutePriceScheduleType(
                                stream,
                                &mut (*Dynamic_SEResControlModeType).AbsolutePriceSchedule,
                            );
                            if error == 0 as i32 {
                                (*Dynamic_SEResControlModeType)
                                    .set_AbsolutePriceSchedule_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_PriceLevelScheduleType(
                                stream,
                                &mut (*Dynamic_SEResControlModeType).PriceLevelSchedule,
                            );
                            if error == 0 as i32 {
                                (*Dynamic_SEResControlModeType)
                                    .set_PriceLevelSchedule_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_EMAIDListType(
    stream: &mut ExiBitstream,
    mut EMAIDListType: *mut iso20_EMAIDListType,
) -> i32 {
    let mut grammar_id: i32 = 250 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_EMAIDListType(EMAIDListType);
    while done == 0 {
        match grammar_id {
            250 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*EMAIDListType).EMAID.arrayLen as i32) < 8 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        error = exi_basetypes_decoder_uint_16(
                                            stream,
                                            &mut (*((*EMAIDListType).EMAID.array)
                                                .as_mut_ptr()
                                                .offset((*EMAIDListType).EMAID.arrayLen as isize))
                                            .charactersLen,
                                        );
                                        if error == 0 as i32 {
                                            if (*EMAIDListType).EMAID.array
                                                [(*EMAIDListType).EMAID.arrayLen as usize]
                                                .charactersLen
                                                as i32
                                                >= 2 as i32
                                            {
                                                (*EMAIDListType).EMAID.array
                                                    [(*EMAIDListType).EMAID.arrayLen as usize]
                                                    .charactersLen = ((*EMAIDListType).EMAID.array
                                                    [(*EMAIDListType).EMAID.arrayLen as usize]
                                                    .charactersLen
                                                    as i32
                                                    - 2 as i32)
                                                    as u16;
                                                error = exi_basetypes_decoder_characters(
                                                    stream,
                                                    (*EMAIDListType).EMAID.array
                                                        [(*EMAIDListType).EMAID.arrayLen as usize]
                                                        .charactersLen
                                                        as usize,
                                                    ((*EMAIDListType).EMAID.array
                                                        [(*EMAIDListType).EMAID.arrayLen as usize]
                                                        .characters)
                                                        .as_mut_ptr(),
                                                    (255 as i32 + 1 as i32) as usize,
                                                );
                                                if error == 0 as i32 {
                                                    (*EMAIDListType).EMAID.arrayLen =
                                                        ((*EMAIDListType).EMAID.arrayLen)
                                                            .wrapping_add(1);
                                                    (*EMAIDListType).EMAID.arrayLen;
                                                }
                                            } else {
                                                error = -(200 as i32);
                                            }
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
                                        grammar_id = 251 as i32;
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
            251 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*EMAIDListType).EMAID.arrayLen as i32) < 8 as i32 {
                                error = exi_basetypes_decoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    &mut eventCode,
                                );
                                if error == 0 as i32 {
                                    if eventCode == 0 as i32 as u32 {
                                        error = exi_basetypes_decoder_uint_16(
                                            stream,
                                            &mut (*((*EMAIDListType).EMAID.array)
                                                .as_mut_ptr()
                                                .offset((*EMAIDListType).EMAID.arrayLen as isize))
                                            .charactersLen,
                                        );
                                        if error == 0 as i32 {
                                            if (*EMAIDListType).EMAID.array
                                                [(*EMAIDListType).EMAID.arrayLen as usize]
                                                .charactersLen
                                                as i32
                                                >= 2 as i32
                                            {
                                                (*EMAIDListType).EMAID.array
                                                    [(*EMAIDListType).EMAID.arrayLen as usize]
                                                    .charactersLen = ((*EMAIDListType).EMAID.array
                                                    [(*EMAIDListType).EMAID.arrayLen as usize]
                                                    .charactersLen
                                                    as i32
                                                    - 2 as i32)
                                                    as u16;
                                                error = exi_basetypes_decoder_characters(
                                                    stream,
                                                    (*EMAIDListType).EMAID.array
                                                        [(*EMAIDListType).EMAID.arrayLen as usize]
                                                        .charactersLen
                                                        as usize,
                                                    ((*EMAIDListType).EMAID.array
                                                        [(*EMAIDListType).EMAID.arrayLen as usize]
                                                        .characters)
                                                        .as_mut_ptr(),
                                                    (255 as i32 + 1 as i32) as usize,
                                                );
                                                if error == 0 as i32 {
                                                    (*EMAIDListType).EMAID.arrayLen =
                                                        ((*EMAIDListType).EMAID.arrayLen)
                                                            .wrapping_add(1);
                                                    (*EMAIDListType).EMAID.arrayLen;
                                                }
                                            } else {
                                                error = -(200 as i32);
                                            }
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
                                        grammar_id = 251 as i32;
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
unsafe extern "C" fn decode_iso20_SignedInstallationDataType(
    stream: &mut ExiBitstream,
    mut SignedInstallationDataType: *mut iso20_SignedInstallationDataType,
) -> i32 {
    let mut grammar_id: i32 = 252 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_SignedInstallationDataType(SignedInstallationDataType);
    while done == 0 {
        match grammar_id {
            252 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*SignedInstallationDataType).Id.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*SignedInstallationDataType).Id.charactersLen as i32 >= 2 as i32
                                {
                                    (*SignedInstallationDataType).Id.charactersLen =
                                        ((*SignedInstallationDataType).Id.charactersLen as i32
                                            - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*SignedInstallationDataType).Id.charactersLen as usize,
                                        ((*SignedInstallationDataType).Id.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            grammar_id = 253 as i32;
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
                            error = decode_iso20_ContractCertificateChainType(
                                stream,
                                &mut (*SignedInstallationDataType).ContractCertificateChain,
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
                                        (*SignedInstallationDataType).ECDHCurve =
                                            value as iso20_ecdhCurveType;
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
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*SignedInstallationDataType).DHPublicKey.bytesLen,
                                &mut *((*SignedInstallationDataType).DHPublicKey.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                133 as i32 as usize,
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
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*SignedInstallationDataType)
                                    .SECP521_EncryptedPrivateKey
                                    .bytesLen,
                                &mut *((*SignedInstallationDataType)
                                    .SECP521_EncryptedPrivateKey
                                    .bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                94 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*SignedInstallationDataType)
                                    .set_SECP521_EncryptedPrivateKey_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*SignedInstallationDataType)
                                    .X448_EncryptedPrivateKey
                                    .bytesLen,
                                &mut *((*SignedInstallationDataType)
                                    .X448_EncryptedPrivateKey
                                    .bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                84 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*SignedInstallationDataType)
                                    .set_X448_EncryptedPrivateKey_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        2 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*SignedInstallationDataType)
                                    .TPM_EncryptedPrivateKey
                                    .bytesLen,
                                &mut *((*SignedInstallationDataType).TPM_EncryptedPrivateKey.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                206 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*SignedInstallationDataType)
                                    .set_TPM_EncryptedPrivateKey_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_PnC_ASResAuthorizationModeType(
    stream: &mut ExiBitstream,
    mut PnC_ASResAuthorizationModeType: *mut iso20_PnC_ASResAuthorizationModeType,
) -> i32 {
    let mut grammar_id: i32 = 257 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_PnC_ASResAuthorizationModeType(PnC_ASResAuthorizationModeType);
    while done == 0 {
        match grammar_id {
            257 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*PnC_ASResAuthorizationModeType).GenChallenge.bytesLen,
                                &mut *((*PnC_ASResAuthorizationModeType).GenChallenge.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                16 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 258 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            258 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_SupportedProvidersListType(
                                stream,
                                &mut (*PnC_ASResAuthorizationModeType).SupportedProviders,
                            );
                            if error == 0 as i32 {
                                (*PnC_ASResAuthorizationModeType)
                                    .set_SupportedProviders_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_Scheduled_SEResControlModeType(
    stream: &mut ExiBitstream,
    mut Scheduled_SEResControlModeType: *mut iso20_Scheduled_SEResControlModeType,
) -> i32 {
    let mut grammar_id: i32 = 259 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_Scheduled_SEResControlModeType(Scheduled_SEResControlModeType);
    while done == 0 {
        match grammar_id {
            259 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*Scheduled_SEResControlModeType).ScheduleTuple.arrayLen as i32)
                                < 3 as i32
                            {
                                let fresh50 =
                                    (*Scheduled_SEResControlModeType).ScheduleTuple.arrayLen;
                                (*Scheduled_SEResControlModeType).ScheduleTuple.arrayLen =
                                    ((*Scheduled_SEResControlModeType).ScheduleTuple.arrayLen)
                                        .wrapping_add(1);
                                error = decode_iso20_ScheduleTupleType(
                                    stream,
                                    &mut *((*Scheduled_SEResControlModeType).ScheduleTuple.array)
                                        .as_mut_ptr()
                                        .offset(fresh50 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 260 as i32;
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
                            if ((*Scheduled_SEResControlModeType).ScheduleTuple.arrayLen as i32)
                                < 3 as i32
                            {
                                let fresh51 =
                                    (*Scheduled_SEResControlModeType).ScheduleTuple.arrayLen;
                                (*Scheduled_SEResControlModeType).ScheduleTuple.arrayLen =
                                    ((*Scheduled_SEResControlModeType).ScheduleTuple.arrayLen)
                                        .wrapping_add(1);
                                error = decode_iso20_ScheduleTupleType(
                                    stream,
                                    &mut *((*Scheduled_SEResControlModeType).ScheduleTuple.array)
                                        .as_mut_ptr()
                                        .offset(fresh51 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            if ((*Scheduled_SEResControlModeType).ScheduleTuple.arrayLen as i32)
                                < 3 as i32
                            {
                                grammar_id = 260 as i32;
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
unsafe extern "C" fn decode_iso20_SessionSetupReqType(
    stream: &mut ExiBitstream,
    mut SessionSetupReqType: *mut iso20_SessionSetupReqType,
) -> i32 {
    let mut grammar_id: i32 = 261 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_SessionSetupReqType(SessionSetupReqType);
    while done == 0 {
        match grammar_id {
            261 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_MessageHeaderType(
                                stream,
                                &mut (*SessionSetupReqType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 262 as i32;
                            }
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
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    error = exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut (*SessionSetupReqType).EVCCID.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*SessionSetupReqType).EVCCID.charactersLen as i32
                                            >= 2 as i32
                                        {
                                            (*SessionSetupReqType).EVCCID.charactersLen =
                                                ((*SessionSetupReqType).EVCCID.charactersLen as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*SessionSetupReqType).EVCCID.charactersLen
                                                    as usize,
                                                ((*SessionSetupReqType).EVCCID.characters)
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
unsafe extern "C" fn decode_iso20_SessionSetupResType(
    stream: &mut ExiBitstream,
    mut SessionSetupResType: *mut iso20_SessionSetupResType,
) -> i32 {
    let mut grammar_id: i32 = 263 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_SessionSetupResType(SessionSetupResType);
    while done == 0 {
        match grammar_id {
            263 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_MessageHeaderType(
                                stream,
                                &mut (*SessionSetupResType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 264 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            264 => {
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
                                        (*SessionSetupResType).ResponseCode =
                                            value as iso20_responseCodeType;
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
                                        grammar_id = 265 as i32;
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
            265 => {
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
                                        &mut (*SessionSetupResType).EVSEID.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*SessionSetupResType).EVSEID.charactersLen as i32
                                            >= 2 as i32
                                        {
                                            (*SessionSetupResType).EVSEID.charactersLen =
                                                ((*SessionSetupResType).EVSEID.charactersLen as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*SessionSetupResType).EVSEID.charactersLen
                                                    as usize,
                                                ((*SessionSetupResType).EVSEID.characters)
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
unsafe extern "C" fn decode_iso20_AuthorizationSetupReqType(
    stream: &mut ExiBitstream,
    mut AuthorizationSetupReqType: *mut iso20_AuthorizationSetupReqType,
) -> i32 {
    let mut grammar_id: i32 = 266 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_AuthorizationSetupReqType(AuthorizationSetupReqType);
    while done == 0 {
        match grammar_id {
            266 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_MessageHeaderType(
                                stream,
                                &mut (*AuthorizationSetupReqType).Header,
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
unsafe extern "C" fn decode_iso20_AuthorizationSetupResType(
    stream: &mut ExiBitstream,
    mut AuthorizationSetupResType: *mut iso20_AuthorizationSetupResType,
) -> i32 {
    let mut grammar_id: i32 = 267 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_AuthorizationSetupResType(AuthorizationSetupResType);
    while done == 0 {
        match grammar_id {
            267 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_MessageHeaderType(
                                stream,
                                &mut (*AuthorizationSetupResType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 268 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            268 => {
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
                                        (*AuthorizationSetupResType).ResponseCode =
                                            value as iso20_responseCodeType;
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
                                        grammar_id = 269 as i32;
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
            269 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*AuthorizationSetupResType).AuthorizationServices.arrayLen as i32)
                                < 2 as i32
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
                                            1 as i32 as usize,
                                            &mut value_0,
                                        );
                                        if error == 0 as i32 {
                                            (*AuthorizationSetupResType)
                                                .AuthorizationServices
                                                .array
                                                [(*AuthorizationSetupResType)
                                                    .AuthorizationServices
                                                    .arrayLen
                                                    as usize] = value_0 as iso20_authorizationType;
                                            (*AuthorizationSetupResType)
                                                .AuthorizationServices
                                                .arrayLen = ((*AuthorizationSetupResType)
                                                .AuthorizationServices
                                                .arrayLen)
                                                .wrapping_add(1);
                                            (*AuthorizationSetupResType)
                                                .AuthorizationServices
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
                                        grammar_id = 270 as i32;
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
            270 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*AuthorizationSetupResType).AuthorizationServices.arrayLen as i32)
                                < 2 as i32
                            {
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
                                            (*AuthorizationSetupResType)
                                                .AuthorizationServices
                                                .array
                                                [(*AuthorizationSetupResType)
                                                    .AuthorizationServices
                                                    .arrayLen
                                                    as usize] = value_1 as iso20_authorizationType;
                                            (*AuthorizationSetupResType)
                                                .AuthorizationServices
                                                .arrayLen = ((*AuthorizationSetupResType)
                                                .AuthorizationServices
                                                .arrayLen)
                                                .wrapping_add(1);
                                            (*AuthorizationSetupResType)
                                                .AuthorizationServices
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
                                        grammar_id = 271 as i32;
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
                                        1 as i32 as usize,
                                        &mut value_2,
                                    );
                                    if error == 0 as i32 {
                                        (*AuthorizationSetupResType)
                                            .CertificateInstallationService = value_2 as i32;
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
                                        grammar_id = 272 as i32;
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
            271 => {
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
                                    let mut value_3: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_3,
                                    );
                                    if error == 0 as i32 {
                                        (*AuthorizationSetupResType)
                                            .CertificateInstallationService = value_3 as i32;
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
                                        grammar_id = 272 as i32;
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
            272 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_EIM_ASResAuthorizationModeType(
                                stream,
                                &mut (*AuthorizationSetupResType).EIM_ASResAuthorizationMode,
                            );
                            if error == 0 as i32 {
                                (*AuthorizationSetupResType)
                                    .set_EIM_ASResAuthorizationMode_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_PnC_ASResAuthorizationModeType(
                                stream,
                                &mut (*AuthorizationSetupResType).PnC_ASResAuthorizationMode,
                            );
                            if error == 0 as i32 {
                                (*AuthorizationSetupResType)
                                    .set_PnC_ASResAuthorizationMode_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_AuthorizationReqType(
    stream: &mut ExiBitstream,
    mut AuthorizationReqType: *mut iso20_AuthorizationReqType,
) -> i32 {
    let mut grammar_id: i32 = 273 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_AuthorizationReqType(AuthorizationReqType);
    while done == 0 {
        match grammar_id {
            273 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_MessageHeaderType(
                                stream,
                                &mut (*AuthorizationReqType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 274 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            274 => {
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
                                        (*AuthorizationReqType).SelectedAuthorizationService =
                                            value as iso20_authorizationType;
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
                                        grammar_id = 275 as i32;
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
            275 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_EIM_AReqAuthorizationModeType(
                                stream,
                                &mut (*AuthorizationReqType).EIM_AReqAuthorizationMode,
                            );
                            if error == 0 as i32 {
                                (*AuthorizationReqType)
                                    .set_EIM_AReqAuthorizationMode_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_PnC_AReqAuthorizationModeType(
                                stream,
                                &mut (*AuthorizationReqType).PnC_AReqAuthorizationMode,
                            );
                            if error == 0 as i32 {
                                (*AuthorizationReqType)
                                    .set_PnC_AReqAuthorizationMode_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_AuthorizationResType(
    stream: &mut ExiBitstream,
    mut AuthorizationResType: *mut iso20_AuthorizationResType,
) -> i32 {
    let mut grammar_id: i32 = 276 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_AuthorizationResType(AuthorizationResType);
    while done == 0 {
        match grammar_id {
            276 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_MessageHeaderType(
                                stream,
                                &mut (*AuthorizationResType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 277 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            277 => {
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
                                        (*AuthorizationResType).ResponseCode =
                                            value as iso20_responseCodeType;
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
                                        grammar_id = 278 as i32;
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
            278 => {
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
                                        (*AuthorizationResType).EVSEProcessing =
                                            value_0 as iso20_processingType;
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
unsafe extern "C" fn decode_iso20_ServiceDiscoveryReqType(
    stream: &mut ExiBitstream,
    mut ServiceDiscoveryReqType: *mut iso20_ServiceDiscoveryReqType,
) -> i32 {
    let mut grammar_id: i32 = 279 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_ServiceDiscoveryReqType(ServiceDiscoveryReqType);
    while done == 0 {
        match grammar_id {
            279 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_MessageHeaderType(
                                stream,
                                &mut (*ServiceDiscoveryReqType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 280 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            280 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_ServiceIDListType(
                                stream,
                                &mut (*ServiceDiscoveryReqType).SupportedServiceIDs,
                            );
                            if error == 0 as i32 {
                                (*ServiceDiscoveryReqType).set_SupportedServiceIDs_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_ServiceDiscoveryResType(
    stream: &mut ExiBitstream,
    mut ServiceDiscoveryResType: *mut iso20_ServiceDiscoveryResType,
) -> i32 {
    let mut grammar_id: i32 = 281 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_ServiceDiscoveryResType(ServiceDiscoveryResType);
    while done == 0 {
        match grammar_id {
            281 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_MessageHeaderType(
                                stream,
                                &mut (*ServiceDiscoveryResType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 282 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            282 => {
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
                                        (*ServiceDiscoveryResType).ResponseCode =
                                            value as iso20_responseCodeType;
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
                                        grammar_id = 283 as i32;
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
            283 => {
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
                                        (*ServiceDiscoveryResType).ServiceRenegotiationSupported =
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
                                        grammar_id = 284 as i32;
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
            284 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_ServiceListType(
                                stream,
                                &mut (*ServiceDiscoveryResType).EnergyTransferServiceList,
                            );
                            if error == 0 as i32 {
                                grammar_id = 285 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            285 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_ServiceListType(
                                stream,
                                &mut (*ServiceDiscoveryResType).VASList,
                            );
                            if error == 0 as i32 {
                                (*ServiceDiscoveryResType).set_VASList_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_ServiceDetailReqType(
    stream: &mut ExiBitstream,
    mut ServiceDetailReqType: *mut iso20_ServiceDetailReqType,
) -> i32 {
    let mut grammar_id: i32 = 286 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_ServiceDetailReqType(ServiceDetailReqType);
    while done == 0 {
        match grammar_id {
            286 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_MessageHeaderType(
                                stream,
                                &mut (*ServiceDetailReqType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 287 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            287 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*ServiceDetailReqType).ServiceID,
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
unsafe extern "C" fn decode_iso20_ServiceDetailResType(
    stream: &mut ExiBitstream,
    mut ServiceDetailResType: *mut iso20_ServiceDetailResType,
) -> i32 {
    let mut grammar_id: i32 = 288 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_ServiceDetailResType(ServiceDetailResType);
    while done == 0 {
        match grammar_id {
            288 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_MessageHeaderType(
                                stream,
                                &mut (*ServiceDetailResType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 289 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            289 => {
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
                                        (*ServiceDetailResType).ResponseCode =
                                            value as iso20_responseCodeType;
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
                                        grammar_id = 290 as i32;
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
            290 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*ServiceDetailResType).ServiceID,
                            );
                            if error == 0 as i32 {
                                grammar_id = 291 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            291 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_ServiceParameterListType(
                                stream,
                                &mut (*ServiceDetailResType).ServiceParameterList,
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
unsafe extern "C" fn decode_iso20_ServiceSelectionReqType(
    stream: &mut ExiBitstream,
    mut ServiceSelectionReqType: *mut iso20_ServiceSelectionReqType,
) -> i32 {
    let mut grammar_id: i32 = 292 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_ServiceSelectionReqType(ServiceSelectionReqType);
    while done == 0 {
        match grammar_id {
            292 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_MessageHeaderType(
                                stream,
                                &mut (*ServiceSelectionReqType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 293 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            293 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_SelectedServiceType(
                                stream,
                                &mut (*ServiceSelectionReqType).SelectedEnergyTransferService,
                            );
                            if error == 0 as i32 {
                                grammar_id = 294 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            294 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_SelectedServiceListType(
                                stream,
                                &mut (*ServiceSelectionReqType).SelectedVASList,
                            );
                            if error == 0 as i32 {
                                (*ServiceSelectionReqType).set_SelectedVASList_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_ServiceSelectionResType(
    stream: &mut ExiBitstream,
    mut ServiceSelectionResType: *mut iso20_ServiceSelectionResType,
) -> i32 {
    let mut grammar_id: i32 = 295 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_ServiceSelectionResType(ServiceSelectionResType);
    while done == 0 {
        match grammar_id {
            295 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_MessageHeaderType(
                                stream,
                                &mut (*ServiceSelectionResType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 296 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            296 => {
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
                                        (*ServiceSelectionResType).ResponseCode =
                                            value as iso20_responseCodeType;
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
unsafe extern "C" fn decode_iso20_ScheduleExchangeReqType(
    stream: &mut ExiBitstream,
    mut ScheduleExchangeReqType: *mut iso20_ScheduleExchangeReqType,
) -> i32 {
    let mut grammar_id: i32 = 297 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_ScheduleExchangeReqType(ScheduleExchangeReqType);
    while done == 0 {
        match grammar_id {
            297 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_MessageHeaderType(
                                stream,
                                &mut (*ScheduleExchangeReqType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 298 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            298 => {
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
                                        10 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*ScheduleExchangeReqType).MaximumSupportingPoints =
                                            value.wrapping_add(12 as i32 as u32) as u16;
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
                                        grammar_id = 299 as i32;
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
            299 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_Dynamic_SEReqControlModeType(
                                stream,
                                &mut (*ScheduleExchangeReqType).Dynamic_SEReqControlMode,
                            );
                            if error == 0 as i32 {
                                (*ScheduleExchangeReqType)
                                    .set_Dynamic_SEReqControlMode_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_Scheduled_SEReqControlModeType(
                                stream,
                                &mut (*ScheduleExchangeReqType).Scheduled_SEReqControlMode,
                            );
                            if error == 0 as i32 {
                                (*ScheduleExchangeReqType)
                                    .set_Scheduled_SEReqControlMode_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_ScheduleExchangeResType(
    stream: &mut ExiBitstream,
    mut ScheduleExchangeResType: *mut iso20_ScheduleExchangeResType,
) -> i32 {
    let mut grammar_id: i32 = 300 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_ScheduleExchangeResType(ScheduleExchangeResType);
    while done == 0 {
        match grammar_id {
            300 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_MessageHeaderType(
                                stream,
                                &mut (*ScheduleExchangeResType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 301 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            301 => {
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
                                        (*ScheduleExchangeResType).ResponseCode =
                                            value as iso20_responseCodeType;
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
                                        grammar_id = 302 as i32;
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
            302 => {
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
                                        (*ScheduleExchangeResType).EVSEProcessing =
                                            value_0 as iso20_processingType;
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
                                        grammar_id = 303 as i32;
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
            303 => {
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
                                    let mut value_1: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_1,
                                    );
                                    if error == 0 as i32 {
                                        (*ScheduleExchangeResType).GoToPause = value_1 as i32;
                                        (*ScheduleExchangeResType).set_GoToPause_isUsed(1 as u32);
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
                                        grammar_id = 304 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            error = decode_iso20_Dynamic_SEResControlModeType(
                                stream,
                                &mut (*ScheduleExchangeResType).Dynamic_SEResControlMode,
                            );
                            if error == 0 as i32 {
                                (*ScheduleExchangeResType)
                                    .set_Dynamic_SEResControlMode_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        2 => {
                            error = decode_iso20_Scheduled_SEResControlModeType(
                                stream,
                                &mut (*ScheduleExchangeResType).Scheduled_SEResControlMode,
                            );
                            if error == 0 as i32 {
                                (*ScheduleExchangeResType)
                                    .set_Scheduled_SEResControlMode_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            304 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_Dynamic_SEResControlModeType(
                                stream,
                                &mut (*ScheduleExchangeResType).Dynamic_SEResControlMode,
                            );
                            if error == 0 as i32 {
                                (*ScheduleExchangeResType)
                                    .set_Dynamic_SEResControlMode_isUsed(1 as u32);
                                grammar_id = 2 as i32;
                            }
                        }
                        1 => {
                            error = decode_iso20_Scheduled_SEResControlModeType(
                                stream,
                                &mut (*ScheduleExchangeResType).Scheduled_SEResControlMode,
                            );
                            if error == 0 as i32 {
                                (*ScheduleExchangeResType)
                                    .set_Scheduled_SEResControlMode_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_PowerDeliveryReqType(
    stream: &mut ExiBitstream,
    mut PowerDeliveryReqType: *mut iso20_PowerDeliveryReqType,
) -> i32 {
    let mut grammar_id: i32 = 305 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_PowerDeliveryReqType(PowerDeliveryReqType);
    while done == 0 {
        match grammar_id {
            305 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_MessageHeaderType(
                                stream,
                                &mut (*PowerDeliveryReqType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 306 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            306 => {
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
                                        (*PowerDeliveryReqType).EVProcessing =
                                            value as iso20_processingType;
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
                                        grammar_id = 307 as i32;
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
            307 => {
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
                                        (*PowerDeliveryReqType).ChargeProgress =
                                            value_0 as iso20_chargeProgressType;
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
                                        grammar_id = 308 as i32;
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
            308 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_EVPowerProfileType(
                                stream,
                                &mut (*PowerDeliveryReqType).EVPowerProfile,
                            );
                            if error == 0 as i32 {
                                (*PowerDeliveryReqType).set_EVPowerProfile_isUsed(1 as u32);
                                grammar_id = 309 as i32;
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
                                    let mut value_1: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_1,
                                    );
                                    if error == 0 as i32 {
                                        (*PowerDeliveryReqType).BPT_ChannelSelection =
                                            value_1 as iso20_channelSelectionType;
                                        (*PowerDeliveryReqType)
                                            .set_BPT_ChannelSelection_isUsed(1 as u32);
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
            309 => {
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
                                    let mut value_2: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_2,
                                    );
                                    if error == 0 as i32 {
                                        (*PowerDeliveryReqType).BPT_ChannelSelection =
                                            value_2 as iso20_channelSelectionType;
                                        (*PowerDeliveryReqType)
                                            .set_BPT_ChannelSelection_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_PowerDeliveryResType(
    stream: &mut ExiBitstream,
    mut PowerDeliveryResType: *mut iso20_PowerDeliveryResType,
) -> i32 {
    let mut grammar_id: i32 = 310 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_PowerDeliveryResType(PowerDeliveryResType);
    while done == 0 {
        match grammar_id {
            310 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_MessageHeaderType(
                                stream,
                                &mut (*PowerDeliveryResType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 311 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            311 => {
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
                                        (*PowerDeliveryResType).ResponseCode =
                                            value as iso20_responseCodeType;
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
                                        grammar_id = 312 as i32;
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
            312 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_EVSEStatusType(
                                stream,
                                &mut (*PowerDeliveryResType).EVSEStatus,
                            );
                            if error == 0 as i32 {
                                (*PowerDeliveryResType).set_EVSEStatus_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_MeteringConfirmationReqType(
    stream: &mut ExiBitstream,
    mut MeteringConfirmationReqType: *mut iso20_MeteringConfirmationReqType,
) -> i32 {
    let mut grammar_id: i32 = 313 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_MeteringConfirmationReqType(MeteringConfirmationReqType);
    while done == 0 {
        match grammar_id {
            313 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_MessageHeaderType(
                                stream,
                                &mut (*MeteringConfirmationReqType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 314 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            314 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_SignedMeteringDataType(
                                stream,
                                &mut (*MeteringConfirmationReqType).SignedMeteringData,
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
unsafe extern "C" fn decode_iso20_MeteringConfirmationResType(
    stream: &mut ExiBitstream,
    mut MeteringConfirmationResType: *mut iso20_MeteringConfirmationResType,
) -> i32 {
    let mut grammar_id: i32 = 315 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_MeteringConfirmationResType(MeteringConfirmationResType);
    while done == 0 {
        match grammar_id {
            315 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_MessageHeaderType(
                                stream,
                                &mut (*MeteringConfirmationResType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 316 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            316 => {
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
                                        (*MeteringConfirmationResType).ResponseCode =
                                            value as iso20_responseCodeType;
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
unsafe extern "C" fn decode_iso20_SessionStopReqType(
    stream: &mut ExiBitstream,
    mut SessionStopReqType: *mut iso20_SessionStopReqType,
) -> i32 {
    let mut grammar_id: i32 = 317 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_SessionStopReqType(SessionStopReqType);
    while done == 0 {
        match grammar_id {
            317 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_MessageHeaderType(
                                stream,
                                &mut (*SessionStopReqType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 318 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            318 => {
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
                                        (*SessionStopReqType).ChargingSession =
                                            value as iso20_chargingSessionType;
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
                                        grammar_id = 319 as i32;
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
            319 => {
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
                                        &mut (*SessionStopReqType).EVTerminationCode.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*SessionStopReqType).EVTerminationCode.charactersLen
                                            as i32
                                            >= 2 as i32
                                        {
                                            (*SessionStopReqType).EVTerminationCode.charactersLen =
                                                ((*SessionStopReqType)
                                                    .EVTerminationCode
                                                    .charactersLen
                                                    as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*SessionStopReqType)
                                                    .EVTerminationCode
                                                    .charactersLen
                                                    as usize,
                                                ((*SessionStopReqType)
                                                    .EVTerminationCode
                                                    .characters)
                                                    .as_mut_ptr(),
                                                (80 as i32 + 1 as i32) as usize,
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
                                        (*SessionStopReqType)
                                            .set_EVTerminationCode_isUsed(1 as u32);
                                        grammar_id = 320 as i32;
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
                                        &mut (*SessionStopReqType)
                                            .EVTerminationExplanation
                                            .charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*SessionStopReqType)
                                            .EVTerminationExplanation
                                            .charactersLen
                                            as i32
                                            >= 2 as i32
                                        {
                                            (*SessionStopReqType)
                                                .EVTerminationExplanation
                                                .charactersLen = ((*SessionStopReqType)
                                                .EVTerminationExplanation
                                                .charactersLen
                                                as i32
                                                - 2 as i32)
                                                as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*SessionStopReqType)
                                                    .EVTerminationExplanation
                                                    .charactersLen
                                                    as usize,
                                                ((*SessionStopReqType)
                                                    .EVTerminationExplanation
                                                    .characters)
                                                    .as_mut_ptr(),
                                                (160 as i32 + 1 as i32) as usize,
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
                                        (*SessionStopReqType)
                                            .set_EVTerminationExplanation_isUsed(1 as u32);
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
            320 => {
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
                                        &mut (*SessionStopReqType)
                                            .EVTerminationExplanation
                                            .charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*SessionStopReqType)
                                            .EVTerminationExplanation
                                            .charactersLen
                                            as i32
                                            >= 2 as i32
                                        {
                                            (*SessionStopReqType)
                                                .EVTerminationExplanation
                                                .charactersLen = ((*SessionStopReqType)
                                                .EVTerminationExplanation
                                                .charactersLen
                                                as i32
                                                - 2 as i32)
                                                as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*SessionStopReqType)
                                                    .EVTerminationExplanation
                                                    .charactersLen
                                                    as usize,
                                                ((*SessionStopReqType)
                                                    .EVTerminationExplanation
                                                    .characters)
                                                    .as_mut_ptr(),
                                                (160 as i32 + 1 as i32) as usize,
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
                                        (*SessionStopReqType)
                                            .set_EVTerminationExplanation_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_SessionStopResType(
    stream: &mut ExiBitstream,
    mut SessionStopResType: *mut iso20_SessionStopResType,
) -> i32 {
    let mut grammar_id: i32 = 321 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_SessionStopResType(SessionStopResType);
    while done == 0 {
        match grammar_id {
            321 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_MessageHeaderType(
                                stream,
                                &mut (*SessionStopResType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 322 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            322 => {
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
                                        (*SessionStopResType).ResponseCode =
                                            value as iso20_responseCodeType;
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
unsafe extern "C" fn decode_iso20_CertificateInstallationReqType(
    stream: &mut ExiBitstream,
    mut CertificateInstallationReqType: *mut iso20_CertificateInstallationReqType,
) -> i32 {
    let mut grammar_id: i32 = 323 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_CertificateInstallationReqType(CertificateInstallationReqType);
    while done == 0 {
        match grammar_id {
            323 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_MessageHeaderType(
                                stream,
                                &mut (*CertificateInstallationReqType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 324 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            324 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_SignedCertificateChainType(
                                stream,
                                &mut (*CertificateInstallationReqType)
                                    .OEMProvisioningCertificateChain,
                            );
                            if error == 0 as i32 {
                                grammar_id = 325 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            325 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_ListOfRootCertificateIDsType(
                                stream,
                                &mut (*CertificateInstallationReqType).ListOfRootCertificateIDs,
                            );
                            if error == 0 as i32 {
                                grammar_id = 326 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            326 => {
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
                                        (*CertificateInstallationReqType)
                                            .MaximumContractCertificateChains = value as u8;
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
                                        grammar_id = 327 as i32;
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
            327 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_EMAIDListType(
                                stream,
                                &mut (*CertificateInstallationReqType).PrioritizedEMAIDs,
                            );
                            if error == 0 as i32 {
                                (*CertificateInstallationReqType)
                                    .set_PrioritizedEMAIDs_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_CertificateInstallationResType(
    stream: &mut ExiBitstream,
    mut CertificateInstallationResType: *mut iso20_CertificateInstallationResType,
) -> i32 {
    let mut grammar_id: i32 = 328 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_CertificateInstallationResType(CertificateInstallationResType);
    while done == 0 {
        match grammar_id {
            328 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_MessageHeaderType(
                                stream,
                                &mut (*CertificateInstallationResType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 329 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            329 => {
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
                                        (*CertificateInstallationResType).ResponseCode =
                                            value as iso20_responseCodeType;
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
                                        grammar_id = 330 as i32;
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
            330 => {
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
                                        (*CertificateInstallationResType).EVSEProcessing =
                                            value_0 as iso20_processingType;
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
                                        grammar_id = 331 as i32;
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
            331 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_CertificateChainType(
                                stream,
                                &mut (*CertificateInstallationResType).CPSCertificateChain,
                            );
                            if error == 0 as i32 {
                                grammar_id = 332 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            332 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_SignedInstallationDataType(
                                stream,
                                &mut (*CertificateInstallationResType).SignedInstallationData,
                            );
                            if error == 0 as i32 {
                                grammar_id = 333 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            333 => {
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
                                        8 as i32 as usize,
                                        &mut value_1,
                                    );
                                    if error == 0 as i32 {
                                        (*CertificateInstallationResType)
                                            .RemainingContractCertificateChains = value_1 as u8;
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
unsafe extern "C" fn decode_iso20_VehicleCheckInReqType(
    stream: &mut ExiBitstream,
    mut VehicleCheckInReqType: *mut iso20_VehicleCheckInReqType,
) -> i32 {
    let mut grammar_id: i32 = 334 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_VehicleCheckInReqType(VehicleCheckInReqType);
    while done == 0 {
        match grammar_id {
            334 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_MessageHeaderType(
                                stream,
                                &mut (*VehicleCheckInReqType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 335 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            335 => {
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
                                        (*VehicleCheckInReqType).EVCheckInStatus =
                                            value as iso20_evCheckInStatusType;
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
                                        grammar_id = 336 as i32;
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
            336 => {
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
                                        (*VehicleCheckInReqType).ParkingMethod =
                                            value_0 as iso20_parkingMethodType;
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
                                        grammar_id = 337 as i32;
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
            337 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*VehicleCheckInReqType).VehicleFrame,
                            );
                            if error == 0 as i32 {
                                (*VehicleCheckInReqType).set_VehicleFrame_isUsed(1 as u32);
                                grammar_id = 338 as i32;
                            }
                        }
                        1 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*VehicleCheckInReqType).DeviceOffset,
                            );
                            if error == 0 as i32 {
                                (*VehicleCheckInReqType).set_DeviceOffset_isUsed(1 as u32);
                                grammar_id = 339 as i32;
                            }
                        }
                        2 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*VehicleCheckInReqType).VehicleTravel,
                            );
                            if error == 0 as i32 {
                                (*VehicleCheckInReqType).set_VehicleTravel_isUsed(1 as u32);
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
            338 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*VehicleCheckInReqType).DeviceOffset,
                            );
                            if error == 0 as i32 {
                                (*VehicleCheckInReqType).set_DeviceOffset_isUsed(1 as u32);
                                grammar_id = 339 as i32;
                            }
                        }
                        1 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*VehicleCheckInReqType).VehicleTravel,
                            );
                            if error == 0 as i32 {
                                (*VehicleCheckInReqType).set_VehicleTravel_isUsed(1 as u32);
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
            339 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*VehicleCheckInReqType).VehicleTravel,
                            );
                            if error == 0 as i32 {
                                (*VehicleCheckInReqType).set_VehicleTravel_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_VehicleCheckInResType(
    stream: &mut ExiBitstream,
    mut VehicleCheckInResType: *mut iso20_VehicleCheckInResType,
) -> i32 {
    let mut grammar_id: i32 = 340 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_VehicleCheckInResType(VehicleCheckInResType);
    while done == 0 {
        match grammar_id {
            340 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_MessageHeaderType(
                                stream,
                                &mut (*VehicleCheckInResType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 341 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            341 => {
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
                                        (*VehicleCheckInResType).ResponseCode =
                                            value as iso20_responseCodeType;
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
                                        grammar_id = 342 as i32;
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
            342 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*VehicleCheckInResType).ParkingSpace,
                            );
                            if error == 0 as i32 {
                                (*VehicleCheckInResType).set_ParkingSpace_isUsed(1 as u32);
                                grammar_id = 343 as i32;
                            }
                        }
                        1 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*VehicleCheckInResType).DeviceLocation,
                            );
                            if error == 0 as i32 {
                                (*VehicleCheckInResType).set_DeviceLocation_isUsed(1 as u32);
                                grammar_id = 344 as i32;
                            }
                        }
                        2 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*VehicleCheckInResType).TargetDistance,
                            );
                            if error == 0 as i32 {
                                (*VehicleCheckInResType).set_TargetDistance_isUsed(1 as u32);
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
            343 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*VehicleCheckInResType).DeviceLocation,
                            );
                            if error == 0 as i32 {
                                (*VehicleCheckInResType).set_DeviceLocation_isUsed(1 as u32);
                                grammar_id = 344 as i32;
                            }
                        }
                        1 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*VehicleCheckInResType).TargetDistance,
                            );
                            if error == 0 as i32 {
                                (*VehicleCheckInResType).set_TargetDistance_isUsed(1 as u32);
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
            344 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*VehicleCheckInResType).TargetDistance,
                            );
                            if error == 0 as i32 {
                                (*VehicleCheckInResType).set_TargetDistance_isUsed(1 as u32);
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
unsafe extern "C" fn decode_iso20_VehicleCheckOutReqType(
    stream: &mut ExiBitstream,
    mut VehicleCheckOutReqType: *mut iso20_VehicleCheckOutReqType,
) -> i32 {
    let mut grammar_id: i32 = 345 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_VehicleCheckOutReqType(VehicleCheckOutReqType);
    while done == 0 {
        match grammar_id {
            345 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_MessageHeaderType(
                                stream,
                                &mut (*VehicleCheckOutReqType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 346 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            346 => {
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
                                        (*VehicleCheckOutReqType).EVCheckOutStatus =
                                            value as iso20_evCheckOutStatusType;
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
                                        grammar_id = 347 as i32;
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
            347 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint64(
                                stream,
                                &mut (*VehicleCheckOutReqType).CheckOutTime,
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
unsafe extern "C" fn decode_iso20_VehicleCheckOutResType(
    stream: &mut ExiBitstream,
    mut VehicleCheckOutResType: *mut iso20_VehicleCheckOutResType,
) -> i32 {
    let mut grammar_id: i32 = 348 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_VehicleCheckOutResType(VehicleCheckOutResType);
    while done == 0 {
        match grammar_id {
            348 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_MessageHeaderType(
                                stream,
                                &mut (*VehicleCheckOutResType).Header,
                            );
                            if error == 0 as i32 {
                                grammar_id = 349 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            349 => {
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
                                        (*VehicleCheckOutResType).ResponseCode =
                                            value as iso20_responseCodeType;
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
                                        grammar_id = 350 as i32;
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
            350 => {
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
                                        (*VehicleCheckOutResType).EVSECheckOutStatus =
                                            value_0 as iso20_evseCheckOutStatusType;
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
unsafe extern "C" fn decode_iso20_CLReqControlModeType(
    stream: &mut ExiBitstream,
    mut CLReqControlModeType: *mut iso20_CLReqControlModeType,
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
unsafe extern "C" fn decode_iso20_CLResControlModeType(
    stream: &mut ExiBitstream,
    mut CLResControlModeType: *mut iso20_CLResControlModeType,
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
unsafe extern "C" fn decode_iso20_ManifestType(
    stream: &mut ExiBitstream,
    mut ManifestType: *mut iso20_ManifestType,
) -> i32 {
    let mut grammar_id: i32 = 351 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_ManifestType(ManifestType);
    while done == 0 {
        match grammar_id {
            351 => {
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
                            grammar_id = 353 as i32;
                        }
                        1 => {
                            if ((*ManifestType).Reference.arrayLen as i32) < 4 as i32 {
                                let fresh52 = (*ManifestType).Reference.arrayLen;
                                (*ManifestType).Reference.arrayLen =
                                    ((*ManifestType).Reference.arrayLen).wrapping_add(1);
                                error = decode_iso20_ReferenceType(
                                    stream,
                                    &mut *((*ManifestType).Reference.array)
                                        .as_mut_ptr()
                                        .offset(fresh52 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 352 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            352 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ManifestType).Reference.arrayLen as i32) < 4 as i32 {
                                let fresh53 = (*ManifestType).Reference.arrayLen;
                                (*ManifestType).Reference.arrayLen =
                                    ((*ManifestType).Reference.arrayLen).wrapping_add(1);
                                error = decode_iso20_ReferenceType(
                                    stream,
                                    &mut *((*ManifestType).Reference.array)
                                        .as_mut_ptr()
                                        .offset(fresh53 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 352 as i32;
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
            353 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ManifestType).Reference.arrayLen as i32) < 4 as i32 {
                                let fresh54 = (*ManifestType).Reference.arrayLen;
                                (*ManifestType).Reference.arrayLen =
                                    ((*ManifestType).Reference.arrayLen).wrapping_add(1);
                                error = decode_iso20_ReferenceType(
                                    stream,
                                    &mut *((*ManifestType).Reference.array)
                                        .as_mut_ptr()
                                        .offset(fresh54 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 354 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            354 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ManifestType).Reference.arrayLen as i32) < 4 as i32 {
                                let fresh55 = (*ManifestType).Reference.arrayLen;
                                (*ManifestType).Reference.arrayLen =
                                    ((*ManifestType).Reference.arrayLen).wrapping_add(1);
                                error = decode_iso20_ReferenceType(
                                    stream,
                                    &mut *((*ManifestType).Reference.array)
                                        .as_mut_ptr()
                                        .offset(fresh55 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 354 as i32;
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
unsafe extern "C" fn decode_iso20_SignaturePropertiesType(
    stream: &mut ExiBitstream,
    mut SignaturePropertiesType: *mut iso20_SignaturePropertiesType,
) -> i32 {
    let mut grammar_id: i32 = 355 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_iso20_SignaturePropertiesType(SignaturePropertiesType);
    while done == 0 {
        match grammar_id {
            355 => {
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
                            grammar_id = 357 as i32;
                        }
                        1 => {
                            error = decode_iso20_SignaturePropertyType(
                                stream,
                                &mut (*SignaturePropertiesType).SignatureProperty,
                            );
                            if error == 0 as i32 {
                                grammar_id = 356 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            356 => {
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
            357 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_iso20_SignaturePropertyType(
                                stream,
                                &mut (*SignaturePropertiesType).SignatureProperty,
                            );
                            if error == 0 as i32 {
                                grammar_id = 358 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            358 => {
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

pub unsafe extern "C" fn decode_iso20_exiDocument(
    stream: &mut ExiBitstream,
    mut exiDoc: *mut iso20_exiDocument,
) -> i32 {
    let mut eventCode: u32 = 0;
    let mut error: i32 = stream.read_and_check()?;
    if error == 0 as i32 {
        init_iso20_exiDocument(exiDoc);
        error = exi_basetypes_decoder_nbit_uint(stream, 6 as i32 as usize, &mut eventCode);
        if error == 0 as i32 {
            match eventCode {
                0 => {
                    error = decode_iso20_AuthorizationReqType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.AuthorizationReq,
                    );
                    (*exiDoc).set_AuthorizationReq_isUsed(1 as u32);
                }
                1 => {
                    error = decode_iso20_AuthorizationResType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.AuthorizationRes,
                    );
                    (*exiDoc).set_AuthorizationRes_isUsed(1 as u32);
                }
                2 => {
                    error = decode_iso20_AuthorizationSetupReqType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.AuthorizationSetupReq,
                    );
                    (*exiDoc).set_AuthorizationSetupReq_isUsed(1 as u32);
                }
                3 => {
                    error = decode_iso20_AuthorizationSetupResType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.AuthorizationSetupRes,
                    );
                    (*exiDoc).set_AuthorizationSetupRes_isUsed(1 as u32);
                }
                4 => {
                    error = decode_iso20_CLReqControlModeType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.CLReqControlMode,
                    );
                    (*exiDoc).set_CLReqControlMode_isUsed(1 as u32);
                }
                5 => {
                    error = decode_iso20_CLResControlModeType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.CLResControlMode,
                    );
                    (*exiDoc).set_CLResControlMode_isUsed(1 as u32);
                }
                6 => {
                    error = decode_iso20_CanonicalizationMethodType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.CanonicalizationMethod,
                    );
                    (*exiDoc).set_CanonicalizationMethod_isUsed(1 as u32);
                }
                7 => {
                    error = decode_iso20_CertificateInstallationReqType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.CertificateInstallationReq,
                    );
                    (*exiDoc).set_CertificateInstallationReq_isUsed(1 as u32);
                }
                8 => {
                    error = decode_iso20_CertificateInstallationResType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.CertificateInstallationRes,
                    );
                    (*exiDoc).set_CertificateInstallationRes_isUsed(1 as u32);
                }
                9 => {
                    error = decode_iso20_DSAKeyValueType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.DSAKeyValue,
                    );
                    (*exiDoc).set_DSAKeyValue_isUsed(1 as u32);
                }
                10 => {
                    error = decode_iso20_DigestMethodType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.DigestMethod,
                    );
                    (*exiDoc).set_DigestMethod_isUsed(1 as u32);
                }
                11 => {}
                12 => {
                    error = decode_iso20_KeyInfoType(stream, &mut (*exiDoc).c2rust_unnamed.KeyInfo);
                    (*exiDoc).set_KeyInfo_isUsed(1 as u32);
                }
                13 => {}
                14 => {
                    error =
                        decode_iso20_KeyValueType(stream, &mut (*exiDoc).c2rust_unnamed.KeyValue);
                    (*exiDoc).set_KeyValue_isUsed(1 as u32);
                }
                15 => {
                    error =
                        decode_iso20_ManifestType(stream, &mut (*exiDoc).c2rust_unnamed.Manifest);
                    (*exiDoc).set_Manifest_isUsed(1 as u32);
                }
                16 => {
                    error = decode_iso20_MeteringConfirmationReqType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.MeteringConfirmationReq,
                    );
                    (*exiDoc).set_MeteringConfirmationReq_isUsed(1 as u32);
                }
                17 => {
                    error = decode_iso20_MeteringConfirmationResType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.MeteringConfirmationRes,
                    );
                    (*exiDoc).set_MeteringConfirmationRes_isUsed(1 as u32);
                }
                18 => {}
                19 => {
                    error = decode_iso20_ObjectType(stream, &mut (*exiDoc).c2rust_unnamed.Object);
                    (*exiDoc).set_Object_isUsed(1 as u32);
                }
                20 => {
                    error = decode_iso20_PGPDataType(stream, &mut (*exiDoc).c2rust_unnamed.PGPData);
                    (*exiDoc).set_PGPData_isUsed(1 as u32);
                }
                21 => {
                    error = decode_iso20_PowerDeliveryReqType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.PowerDeliveryReq,
                    );
                    (*exiDoc).set_PowerDeliveryReq_isUsed(1 as u32);
                }
                22 => {
                    error = decode_iso20_PowerDeliveryResType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.PowerDeliveryRes,
                    );
                    (*exiDoc).set_PowerDeliveryRes_isUsed(1 as u32);
                }
                23 => {
                    error = decode_iso20_RSAKeyValueType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.RSAKeyValue,
                    );
                    (*exiDoc).set_RSAKeyValue_isUsed(1 as u32);
                }
                24 => {
                    error =
                        decode_iso20_ReferenceType(stream, &mut (*exiDoc).c2rust_unnamed.Reference);
                    (*exiDoc).set_Reference_isUsed(1 as u32);
                }
                25 => {
                    error = decode_iso20_RetrievalMethodType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.RetrievalMethod,
                    );
                    (*exiDoc).set_RetrievalMethod_isUsed(1 as u32);
                }
                26 => {
                    error =
                        decode_iso20_SPKIDataType(stream, &mut (*exiDoc).c2rust_unnamed.SPKIData);
                    (*exiDoc).set_SPKIData_isUsed(1 as u32);
                }
                27 => {
                    error = decode_iso20_ScheduleExchangeReqType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.ScheduleExchangeReq,
                    );
                    (*exiDoc).set_ScheduleExchangeReq_isUsed(1 as u32);
                }
                28 => {
                    error = decode_iso20_ScheduleExchangeResType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.ScheduleExchangeRes,
                    );
                    (*exiDoc).set_ScheduleExchangeRes_isUsed(1 as u32);
                }
                29 => {
                    error = decode_iso20_ServiceDetailReqType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.ServiceDetailReq,
                    );
                    (*exiDoc).set_ServiceDetailReq_isUsed(1 as u32);
                }
                30 => {
                    error = decode_iso20_ServiceDetailResType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.ServiceDetailRes,
                    );
                    (*exiDoc).set_ServiceDetailRes_isUsed(1 as u32);
                }
                31 => {
                    error = decode_iso20_ServiceDiscoveryReqType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.ServiceDiscoveryReq,
                    );
                    (*exiDoc).set_ServiceDiscoveryReq_isUsed(1 as u32);
                }
                32 => {
                    error = decode_iso20_ServiceDiscoveryResType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.ServiceDiscoveryRes,
                    );
                    (*exiDoc).set_ServiceDiscoveryRes_isUsed(1 as u32);
                }
                33 => {
                    error = decode_iso20_ServiceSelectionReqType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.ServiceSelectionReq,
                    );
                    (*exiDoc).set_ServiceSelectionReq_isUsed(1 as u32);
                }
                34 => {
                    error = decode_iso20_ServiceSelectionResType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.ServiceSelectionRes,
                    );
                    (*exiDoc).set_ServiceSelectionRes_isUsed(1 as u32);
                }
                35 => {
                    error = decode_iso20_SessionSetupReqType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.SessionSetupReq,
                    );
                    (*exiDoc).set_SessionSetupReq_isUsed(1 as u32);
                }
                36 => {
                    error = decode_iso20_SessionSetupResType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.SessionSetupRes,
                    );
                    (*exiDoc).set_SessionSetupRes_isUsed(1 as u32);
                }
                37 => {
                    error = decode_iso20_SessionStopReqType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.SessionStopReq,
                    );
                    (*exiDoc).set_SessionStopReq_isUsed(1 as u32);
                }
                38 => {
                    error = decode_iso20_SessionStopResType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.SessionStopRes,
                    );
                    (*exiDoc).set_SessionStopRes_isUsed(1 as u32);
                }
                39 => {
                    error = decode_iso20_SignatureMethodType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.SignatureMethod,
                    );
                    (*exiDoc).set_SignatureMethod_isUsed(1 as u32);
                }
                40 => {
                    error = decode_iso20_SignaturePropertiesType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.SignatureProperties,
                    );
                    (*exiDoc).set_SignatureProperties_isUsed(1 as u32);
                }
                41 => {
                    error = decode_iso20_SignaturePropertyType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.SignatureProperty,
                    );
                    (*exiDoc).set_SignatureProperty_isUsed(1 as u32);
                }
                42 => {
                    error =
                        decode_iso20_SignatureType(stream, &mut (*exiDoc).c2rust_unnamed.Signature);
                    (*exiDoc).set_Signature_isUsed(1 as u32);
                }
                43 => {
                    error = decode_iso20_SignatureValueType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.SignatureValue,
                    );
                    (*exiDoc).set_SignatureValue_isUsed(1 as u32);
                }
                44 => {
                    error = decode_iso20_SignedInfoType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.SignedInfo,
                    );
                    (*exiDoc).set_SignedInfo_isUsed(1 as u32);
                }
                45 => {
                    error = decode_iso20_SignedInstallationDataType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.SignedInstallationData,
                    );
                    (*exiDoc).set_SignedInstallationData_isUsed(1 as u32);
                }
                46 => {
                    error = decode_iso20_SignedMeteringDataType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.SignedMeteringData,
                    );
                    (*exiDoc).set_SignedMeteringData_isUsed(1 as u32);
                }
                47 => {
                    error =
                        decode_iso20_TransformType(stream, &mut (*exiDoc).c2rust_unnamed.Transform);
                    (*exiDoc).set_Transform_isUsed(1 as u32);
                }
                48 => {
                    error = decode_iso20_TransformsType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.Transforms,
                    );
                    (*exiDoc).set_Transforms_isUsed(1 as u32);
                }
                49 => {
                    error = decode_iso20_VehicleCheckInReqType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.VehicleCheckInReq,
                    );
                    (*exiDoc).set_VehicleCheckInReq_isUsed(1 as u32);
                }
                50 => {
                    error = decode_iso20_VehicleCheckInResType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.VehicleCheckInRes,
                    );
                    (*exiDoc).set_VehicleCheckInRes_isUsed(1 as u32);
                }
                51 => {
                    error = decode_iso20_VehicleCheckOutReqType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.VehicleCheckOutReq,
                    );
                    (*exiDoc).set_VehicleCheckOutReq_isUsed(1 as u32);
                }
                52 => {
                    error = decode_iso20_VehicleCheckOutResType(
                        stream,
                        &mut (*exiDoc).c2rust_unnamed.VehicleCheckOutRes,
                    );
                    (*exiDoc).set_VehicleCheckOutRes_isUsed(1 as u32);
                }
                53 => {
                    error =
                        decode_iso20_X509DataType(stream, &mut (*exiDoc).c2rust_unnamed.X509Data);
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

pub unsafe extern "C" fn decode_iso20_exiFragment(
    stream: &mut ExiBitstream,
    mut exiFrag: *mut iso20_exiFragment,
) -> i32 {
    let mut eventCode: u32 = 0;
    let mut error: i32 = stream.read_and_check()?;
    if error == 0 as i32 {
        init_iso20_exiFragment(exiFrag);
        error = exi_basetypes_decoder_nbit_uint(stream, 9 as i32 as usize, &mut eventCode);
        if error == 0 as i32 {
            error = -(299 as i32);
            match eventCode {
                0 => {
                    error = decode_iso20_AbsolutePriceScheduleType(
                        stream,
                        &mut (*exiFrag).c2rust_unnamed.AbsolutePriceSchedule,
                    );
                    (*exiFrag).set_AbsolutePriceSchedule_isUsed(1 as u32);
                }
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
                26 => {}
                27 => {
                    error = decode_iso20_CertificateInstallationReqType(
                        stream,
                        &mut (*exiFrag).c2rust_unnamed.CertificateInstallationReq,
                    );
                    (*exiFrag).set_CertificateInstallationReq_isUsed(1 as u32);
                }
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
                119 => {
                    error = decode_iso20_MeteringConfirmationReqType(
                        stream,
                        &mut (*exiFrag).c2rust_unnamed.MeteringConfirmationReq,
                    );
                    (*exiFrag).set_MeteringConfirmationReq_isUsed(1 as u32);
                }
                120 => {}
                121 => {}
                122 => {}
                123 => {}
                124 => {}
                125 => {}
                126 => {}
                127 => {}
                128 => {}
                129 => {}
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
                149 => {}
                150 => {}
                151 => {
                    error = decode_iso20_PnC_AReqAuthorizationModeType(
                        stream,
                        &mut (*exiFrag).c2rust_unnamed.PnC_AReqAuthorizationMode,
                    );
                    (*exiFrag).set_PnC_AReqAuthorizationMode_isUsed(1 as u32);
                }
                152 => {}
                153 => {}
                154 => {}
                155 => {}
                156 => {}
                157 => {}
                158 => {}
                159 => {}
                160 => {}
                161 => {}
                162 => {}
                163 => {}
                164 => {}
                165 => {}
                166 => {}
                167 => {}
                168 => {}
                169 => {}
                170 => {}
                171 => {}
                172 => {}
                173 => {}
                174 => {}
                175 => {}
                176 => {}
                177 => {}
                178 => {}
                179 => {}
                180 => {}
                181 => {}
                182 => {}
                183 => {}
                184 => {}
                185 => {}
                186 => {}
                187 => {}
                188 => {}
                189 => {}
                190 => {}
                191 => {}
                192 => {}
                193 => {}
                194 => {}
                195 => {}
                196 => {}
                197 => {}
                198 => {}
                199 => {}
                200 => {}
                201 => {}
                202 => {}
                203 => {}
                204 => {}
                205 => {}
                206 => {}
                207 => {}
                208 => {}
                209 => {}
                210 => {}
                211 => {}
                212 => {}
                213 => {}
                214 => {}
                215 => {}
                216 => {}
                217 => {}
                218 => {}
                219 => {}
                220 => {}
                221 => {}
                222 => {}
                223 => {}
                224 => {}
                225 => {}
                226 => {}
                227 => {}
                228 => {}
                229 => {}
                230 => {
                    error = decode_iso20_SignedInfoType(
                        stream,
                        &mut (*exiFrag).c2rust_unnamed.SignedInfo,
                    );
                    (*exiFrag).set_SignedInfo_isUsed(1 as u32);
                }
                231 => {
                    error = decode_iso20_SignedInstallationDataType(
                        stream,
                        &mut (*exiFrag).c2rust_unnamed.SignedInstallationData,
                    );
                    (*exiFrag).set_SignedInstallationData_isUsed(1 as u32);
                }
                232 => {}
                233 => {}
                234 => {}
                235 => {}
                236 => {}
                237 => {}
                238 => {}
                239 => {}
                240 => {}
                241 => {}
                242 => {}
                243 => {}
                244 => {}
                245 => {}
                246 => {}
                247 => {}
                248 => {}
                249 => {}
                250 => {}
                251 => {}
                252 => {}
                253 => {}
                254 => {}
                255 => {}
                256 => {}
                257 => {}
                258 => {}
                259 => {}
                260 => {}
                261 => {}
                262 => {}
                263 => {}
                264 => {}
                265 => {}
                266 => {}
                267 => {}
                268 => {}
                269 => {}
                270 => {}
                271 => {}
                272 => {}
                273 => {}
                274 => {}
                275 => {}
                276 => {}
                277 => {}
                278 => {}
                279 => {}
                280 => {}
                _ => {
                    error = -(151 as i32);
                }
            }
            if error == 0 as i32 {
                error = exi_basetypes_decoder_nbit_uint(stream, 9 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    if eventCode != 282 as i32 as u32 {
                        error = -(230 as i32);
                    }
                }
            }
        }
    }
    return error;
}

pub unsafe extern "C" fn decode_iso20_xmldsigFragment(
    stream: &mut ExiBitstream,
    mut xmldsigFrag: *mut iso20_xmldsigFragment,
) -> i32 {
    let mut eventCode: u32 = 0;
    let mut error: i32 = stream.read_and_check()?;
    if error == 0 as i32 {
        init_iso20_xmldsigFragment(xmldsigFrag);
        error = exi_basetypes_decoder_nbit_uint(stream, 6 as i32 as usize, &mut eventCode);
        if error == 0 as i32 {
            error = -(299 as i32);
            match eventCode {
                0 => {
                    error = decode_iso20_CanonicalizationMethodType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.CanonicalizationMethod,
                    );
                    (*xmldsigFrag).set_CanonicalizationMethod_isUsed(1 as u32);
                }
                1 => {
                    error = decode_iso20_DSAKeyValueType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.DSAKeyValue,
                    );
                    (*xmldsigFrag).set_DSAKeyValue_isUsed(1 as u32);
                }
                2 => {
                    error = decode_iso20_DigestMethodType(
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
                    error = decode_iso20_KeyInfoType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.KeyInfo,
                    );
                    (*xmldsigFrag).set_KeyInfo_isUsed(1 as u32);
                }
                9 => {}
                10 => {
                    error = decode_iso20_KeyValueType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.KeyValue,
                    );
                    (*xmldsigFrag).set_KeyValue_isUsed(1 as u32);
                }
                11 => {
                    error = decode_iso20_ManifestType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.Manifest,
                    );
                    (*xmldsigFrag).set_Manifest_isUsed(1 as u32);
                }
                12 => {}
                13 => {}
                14 => {
                    error =
                        decode_iso20_ObjectType(stream, &mut (*xmldsigFrag).c2rust_unnamed.Object);
                    (*xmldsigFrag).set_Object_isUsed(1 as u32);
                }
                15 => {}
                16 => {
                    error = decode_iso20_PGPDataType(
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
                    error = decode_iso20_RSAKeyValueType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.RSAKeyValue,
                    );
                    (*xmldsigFrag).set_RSAKeyValue_isUsed(1 as u32);
                }
                22 => {
                    error = decode_iso20_ReferenceType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.Reference,
                    );
                    (*xmldsigFrag).set_Reference_isUsed(1 as u32);
                }
                23 => {
                    error = decode_iso20_RetrievalMethodType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.RetrievalMethod,
                    );
                    (*xmldsigFrag).set_RetrievalMethod_isUsed(1 as u32);
                }
                24 => {
                    error = decode_iso20_SPKIDataType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.SPKIData,
                    );
                    (*xmldsigFrag).set_SPKIData_isUsed(1 as u32);
                }
                25 => {}
                26 => {}
                27 => {
                    error = decode_iso20_SignatureType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.Signature,
                    );
                    (*xmldsigFrag).set_Signature_isUsed(1 as u32);
                }
                28 => {
                    error = decode_iso20_SignatureMethodType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.SignatureMethod,
                    );
                    (*xmldsigFrag).set_SignatureMethod_isUsed(1 as u32);
                }
                29 => {
                    error = decode_iso20_SignaturePropertiesType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.SignatureProperties,
                    );
                    (*xmldsigFrag).set_SignatureProperties_isUsed(1 as u32);
                }
                30 => {
                    error = decode_iso20_SignaturePropertyType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.SignatureProperty,
                    );
                    (*xmldsigFrag).set_SignatureProperty_isUsed(1 as u32);
                }
                31 => {
                    error = decode_iso20_SignatureValueType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.SignatureValue,
                    );
                    (*xmldsigFrag).set_SignatureValue_isUsed(1 as u32);
                }
                32 => {
                    error = decode_iso20_SignedInfoType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.SignedInfo,
                    );
                    (*xmldsigFrag).set_SignedInfo_isUsed(1 as u32);
                }
                33 => {
                    error = decode_iso20_TransformType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.Transform,
                    );
                    (*xmldsigFrag).set_Transform_isUsed(1 as u32);
                }
                34 => {
                    error = decode_iso20_TransformsType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.Transforms,
                    );
                    (*xmldsigFrag).set_Transforms_isUsed(1 as u32);
                }
                35 => {}
                36 => {}
                37 => {
                    error = decode_iso20_X509DataType(
                        stream,
                        &mut (*xmldsigFrag).c2rust_unnamed.X509Data,
                    );
                    (*xmldsigFrag).set_X509Data_isUsed(1 as u32);
                }
                38 => {}
                39 => {
                    error = decode_iso20_X509IssuerSerialType(
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
