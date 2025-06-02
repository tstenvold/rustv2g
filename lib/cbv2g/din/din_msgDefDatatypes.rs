use ::c2rust_bitfields;
use c2rust_bitfields::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExiUnsigned {
    pub octets: [u8; 29],
    pub octets_count: usize,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct ExiSigned {
    pub data: ExiUnsigned,
    #[bitfield(name = "is_negative", ty = "u8", bits = "0..=0")]
    pub is_negative: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type din_costKindType = u32;
pub const din_costKindType_CarbonDioxideEmission: din_costKindType = 2;
pub const din_costKindType_RenewableGenerationPercentage: din_costKindType = 1;
pub const din_costKindType_relativePricePercentage: din_costKindType = 0;
pub type din_isolationLevelType = u32;
pub const din_isolationLevelType_Fault: din_isolationLevelType = 3;
pub const din_isolationLevelType_Warning: din_isolationLevelType = 2;
pub const din_isolationLevelType_Valid: din_isolationLevelType = 1;
pub const din_isolationLevelType_Invalid: din_isolationLevelType = 0;
pub type din_paymentOptionType = u32;
pub const din_paymentOptionType_ExternalPayment: din_paymentOptionType = 1;
pub const din_paymentOptionType_Contract: din_paymentOptionType = 0;
pub type din_DC_EVSEStatusCodeType = u32;
pub const din_DC_EVSEStatusCodeType_Reserved_C: din_DC_EVSEStatusCodeType = 11;
pub const din_DC_EVSEStatusCodeType_Reserved_B: din_DC_EVSEStatusCodeType = 10;
pub const din_DC_EVSEStatusCodeType_Reserved_A: din_DC_EVSEStatusCodeType = 9;
pub const din_DC_EVSEStatusCodeType_Reserved_9: din_DC_EVSEStatusCodeType = 8;
pub const din_DC_EVSEStatusCodeType_Reserved_8: din_DC_EVSEStatusCodeType = 7;
pub const din_DC_EVSEStatusCodeType_EVSE_Malfunction: din_DC_EVSEStatusCodeType = 6;
pub const din_DC_EVSEStatusCodeType_EVSE_EmergencyShutdown: din_DC_EVSEStatusCodeType = 5;
pub const din_DC_EVSEStatusCodeType_EVSE_IsolationMonitoringActive: din_DC_EVSEStatusCodeType = 4;
pub const din_DC_EVSEStatusCodeType_EVSE_UtilityInterruptEvent: din_DC_EVSEStatusCodeType = 3;
pub const din_DC_EVSEStatusCodeType_EVSE_Shutdown: din_DC_EVSEStatusCodeType = 2;
pub const din_DC_EVSEStatusCodeType_EVSE_Ready: din_DC_EVSEStatusCodeType = 1;
pub const din_DC_EVSEStatusCodeType_EVSE_NotReady: din_DC_EVSEStatusCodeType = 0;
pub type din_unitSymbolType = u32;
pub const din_unitSymbolType_Wh: din_unitSymbolType = 9;
pub const din_unitSymbolType_W_s: din_unitSymbolType = 8;
pub const din_unitSymbolType_W: din_unitSymbolType = 7;
pub const din_unitSymbolType_VA: din_unitSymbolType = 6;
pub const din_unitSymbolType_V: din_unitSymbolType = 5;
pub const din_unitSymbolType_Ah: din_unitSymbolType = 4;
pub const din_unitSymbolType_A: din_unitSymbolType = 3;
pub const din_unitSymbolType_s: din_unitSymbolType = 2;
pub const din_unitSymbolType_m: din_unitSymbolType = 1;
pub const din_unitSymbolType_h: din_unitSymbolType = 0;
pub type din_EVSESupportedEnergyTransferType = u32;
pub const din_EVSESupportedEnergyTransferType_AC_core3p_DC_extended:
    din_EVSESupportedEnergyTransferType = 9;
pub const din_EVSESupportedEnergyTransferType_AC_single_phase_three_phase_core_DC_extended:
    din_EVSESupportedEnergyTransferType = 8;
pub const din_EVSESupportedEnergyTransferType_AC_single_DC_core:
    din_EVSESupportedEnergyTransferType = 7;
pub const din_EVSESupportedEnergyTransferType_AC_core1p_DC_extended:
    din_EVSESupportedEnergyTransferType = 6;
pub const din_EVSESupportedEnergyTransferType_DC_dual: din_EVSESupportedEnergyTransferType = 5;
pub const din_EVSESupportedEnergyTransferType_DC_combo_core: din_EVSESupportedEnergyTransferType =
    4;
pub const din_EVSESupportedEnergyTransferType_DC_extended: din_EVSESupportedEnergyTransferType = 3;
pub const din_EVSESupportedEnergyTransferType_DC_core: din_EVSESupportedEnergyTransferType = 2;
pub const din_EVSESupportedEnergyTransferType_AC_three_phase_core:
    din_EVSESupportedEnergyTransferType = 1;
pub const din_EVSESupportedEnergyTransferType_AC_single_phase_core:
    din_EVSESupportedEnergyTransferType = 0;
pub type din_DC_EVErrorCodeType = u32;
pub const din_DC_EVErrorCodeType_NoData: din_DC_EVErrorCodeType = 11;
pub const din_DC_EVErrorCodeType_FAILED_ChargingSystemIncompatibility: din_DC_EVErrorCodeType = 10;
pub const din_DC_EVErrorCodeType_Reserved_C: din_DC_EVErrorCodeType = 9;
pub const din_DC_EVErrorCodeType_Reserved_B: din_DC_EVErrorCodeType = 8;
pub const din_DC_EVErrorCodeType_Reserved_A: din_DC_EVErrorCodeType = 7;
pub const din_DC_EVErrorCodeType_FAILED_ChargingVoltageOutOfRange: din_DC_EVErrorCodeType = 6;
pub const din_DC_EVErrorCodeType_FAILED_ChargingCurrentdifferential: din_DC_EVErrorCodeType = 5;
pub const din_DC_EVErrorCodeType_FAILED_EVRESSMalfunction: din_DC_EVErrorCodeType = 4;
pub const din_DC_EVErrorCodeType_FAILED_ChargerConnectorLockFault: din_DC_EVErrorCodeType = 3;
pub const din_DC_EVErrorCodeType_FAILED_EVShiftPosition: din_DC_EVErrorCodeType = 2;
pub const din_DC_EVErrorCodeType_FAILED_RESSTemperatureInhibit: din_DC_EVErrorCodeType = 1;
pub const din_DC_EVErrorCodeType_NO_ERROR: din_DC_EVErrorCodeType = 0;
pub type din_EVSENotificationType = u32;
pub const din_EVSENotificationType_ReNegotiation: din_EVSENotificationType = 2;
pub const din_EVSENotificationType_StopCharging: din_EVSENotificationType = 1;
pub const din_EVSENotificationType_None: din_EVSENotificationType = 0;
pub type din_faultCodeType = u32;
pub const din_faultCodeType_UnknownError: din_faultCodeType = 2;
pub const din_faultCodeType_NoTLSRootCertificatAvailable: din_faultCodeType = 1;
pub const din_faultCodeType_ParsingError: din_faultCodeType = 0;
pub type din_responseCodeType = u32;
pub const din_responseCodeType_FAILED_WrongEnergyTransferType: din_responseCodeType = 22;
pub const din_responseCodeType_FAILED_MeteringSignatureNotValid: din_responseCodeType = 21;
pub const din_responseCodeType_FAILED_EVSEPresentVoltageToLow: din_responseCodeType = 20;
pub const din_responseCodeType_FAILED_ChargingProfileInvalid: din_responseCodeType = 19;
pub const din_responseCodeType_FAILED_TariffSelectionInvalid: din_responseCodeType = 18;
pub const din_responseCodeType_FAILED_PowerDeliveryNotApplied: din_responseCodeType = 17;
pub const din_responseCodeType_FAILED_WrongChargeParameter: din_responseCodeType = 16;
pub const din_responseCodeType_FAILED_ContractCanceled: din_responseCodeType = 15;
pub const din_responseCodeType_FAILED_ChallengeInvalid: din_responseCodeType = 14;
pub const din_responseCodeType_FAILED_CertChainError: din_responseCodeType = 13;
pub const din_responseCodeType_FAILED_NoCertificateAvailable: din_responseCodeType = 12;
pub const din_responseCodeType_FAILED_SignatureError: din_responseCodeType = 11;
pub const din_responseCodeType_FAILED_CertificateExpired: din_responseCodeType = 10;
pub const din_responseCodeType_FAILED_PaymentSelectionInvalid: din_responseCodeType = 9;
pub const din_responseCodeType_FAILED_ServiceSelectionInvalid: din_responseCodeType = 8;
pub const din_responseCodeType_FAILED_UnknownSession: din_responseCodeType = 7;
pub const din_responseCodeType_FAILED_ServiceIDInvalid: din_responseCodeType = 6;
pub const din_responseCodeType_FAILED_SequenceError: din_responseCodeType = 5;
pub const din_responseCodeType_FAILED: din_responseCodeType = 4;
pub const din_responseCodeType_OK_CertificateExpiresSoon: din_responseCodeType = 3;
pub const din_responseCodeType_OK_OldSessionJoined: din_responseCodeType = 2;
pub const din_responseCodeType_OK_NewSessionEstablished: din_responseCodeType = 1;
pub const din_responseCodeType_OK: din_responseCodeType = 0;
pub type din_EVRequestedEnergyTransferType = u32;
pub const din_EVRequestedEnergyTransferType_DC_unique: din_EVRequestedEnergyTransferType = 5;
pub const din_EVRequestedEnergyTransferType_DC_combo_core: din_EVRequestedEnergyTransferType = 4;
pub const din_EVRequestedEnergyTransferType_DC_extended: din_EVRequestedEnergyTransferType = 3;
pub const din_EVRequestedEnergyTransferType_DC_core: din_EVRequestedEnergyTransferType = 2;
pub const din_EVRequestedEnergyTransferType_AC_three_phase_core: din_EVRequestedEnergyTransferType =
    1;
pub const din_EVRequestedEnergyTransferType_AC_single_phase_core:
    din_EVRequestedEnergyTransferType = 0;
pub type din_serviceCategoryType = u32;
pub const din_serviceCategoryType_OtherCustom: din_serviceCategoryType = 3;
pub const din_serviceCategoryType_ContractCertificate: din_serviceCategoryType = 2;
pub const din_serviceCategoryType_Internet: din_serviceCategoryType = 1;
pub const din_serviceCategoryType_EVCharging: din_serviceCategoryType = 0;
pub type din_EVSEProcessingType = u32;
pub const din_EVSEProcessingType_Ongoing: din_EVSEProcessingType = 1;
pub const din_EVSEProcessingType_Finished: din_EVSEProcessingType = 0;
pub type din_valueType = u32;
pub const din_valueType_string: din_valueType = 5;
pub const din_valueType_physicalValue: din_valueType = 4;
pub const din_valueType_int: din_valueType = 3;
pub const din_valueType_short: din_valueType = 2;
pub const din_valueType_byte: din_valueType = 1;
pub const din_valueType_bool: din_valueType = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_CostType {
    pub costKind: din_costKindType,
    pub amount: u32,
    pub amountMultiplier: int8_t,
    #[bitfield(name = "amountMultiplier_isUsed", ty = "u32", bits = "0..=0")]
    pub amountMultiplier_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_RelativeTimeIntervalType {
    pub start: u32,
    pub duration: u32,
    #[bitfield(name = "duration_isUsed", ty = "u32", bits = "0..=0")]
    pub duration_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_IntervalType {
    pub _unused: i32,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_ConsumptionCostType {
    pub startValue: u32,
    pub Cost: din_CostType,
    #[bitfield(name = "Cost_isUsed", ty = "u32", bits = "0..=0")]
    pub Cost_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_TransformType {
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
#[repr(C)]
pub struct C2RustUnnamed {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_PMaxScheduleEntryType {
    pub RelativeTimeInterval: din_RelativeTimeIntervalType,
    #[bitfield(name = "RelativeTimeInterval_isUsed", ty = "u32", bits = "0..=0")]
    pub RelativeTimeInterval_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub TimeInterval: din_IntervalType,
    #[bitfield(name = "TimeInterval_isUsed", ty = "u32", bits = "0..=0")]
    pub TimeInterval_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub PMax: i16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_SalesTariffEntryType {
    pub RelativeTimeInterval: din_RelativeTimeIntervalType,
    #[bitfield(name = "RelativeTimeInterval_isUsed", ty = "u32", bits = "0..=0")]
    pub RelativeTimeInterval_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub TimeInterval: din_IntervalType,
    #[bitfield(name = "TimeInterval_isUsed", ty = "u32", bits = "0..=0")]
    pub TimeInterval_isUsed: [u8; 1],
    pub EPriceLevel: u8,
    pub ConsumptionCost: din_ConsumptionCostType,
    #[bitfield(name = "ConsumptionCost_isUsed", ty = "u32", bits = "0..=0")]
    pub ConsumptionCost_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_TransformsType {
    pub Transform: din_TransformType,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_DSAKeyValueType {
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
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_X509IssuerSerialType {
    pub X509IssuerName: C2RustUnnamed_9,
    pub X509SerialNumber: ExiSigned,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_DigestMethodType {
    pub Algorithm: C2RustUnnamed_11,
    pub ANY: C2RustUnnamed_10,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_RSAKeyValueType {
    pub Modulus: C2RustUnnamed_13,
    pub Exponent: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_PMaxScheduleType {
    pub PMaxScheduleID: i16,
    pub PMaxScheduleEntry: C2RustUnnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub array: [din_PMaxScheduleEntryType; 5],
    pub arrayLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_SalesTariffType {
    pub Id: C2RustUnnamed_17,
    pub SalesTariffID: i16,
    pub SalesTariffDescription: C2RustUnnamed_16,
    #[bitfield(name = "SalesTariffDescription_isUsed", ty = "u32", bits = "0..=0")]
    pub SalesTariffDescription_isUsed: [u8; 1],
    pub NumEPriceLevels: u8,
    pub SalesTariffEntry: C2RustUnnamed_15,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub array: [din_SalesTariffEntryType; 5],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub characters: [i8; 33],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_CanonicalizationMethodType {
    pub Algorithm: C2RustUnnamed_19,
    pub ANY: C2RustUnnamed_18,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_ServiceTagType {
    pub ServiceID: u16,
    pub ServiceName: C2RustUnnamed_21,
    #[bitfield(name = "ServiceName_isUsed", ty = "u32", bits = "0..=0")]
    pub ServiceName_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub ServiceCategory: din_serviceCategoryType,
    pub ServiceScope: C2RustUnnamed_20,
    #[bitfield(name = "ServiceScope_isUsed", ty = "u32", bits = "0..=0")]
    pub ServiceScope_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub characters: [i8; 33],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub characters: [i8; 33],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_ServiceType {
    pub ServiceTag: din_ServiceTagType,
    pub FreeService: i32,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_SelectedServiceType {
    pub ServiceID: u16,
    pub ParameterSetID: i16,
    #[bitfield(name = "ParameterSetID_isUsed", ty = "u32", bits = "0..=0")]
    pub ParameterSetID_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_SAScheduleTupleType {
    pub SAScheduleTupleID: i16,
    pub PMaxSchedule: din_PMaxScheduleType,
    pub SalesTariff: din_SalesTariffType,
    #[bitfield(name = "SalesTariff_isUsed", ty = "u32", bits = "0..=0")]
    pub SalesTariff_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_AC_EVSEStatusType {
    pub PowerSwitchClosed: i32,
    pub RCD: i32,
    pub NotificationMaxDelay: u32,
    pub EVSENotification: din_EVSENotificationType,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_SignatureMethodType {
    pub Algorithm: C2RustUnnamed_23,
    pub HMACOutputLength: ExiSigned,
    #[bitfield(name = "HMACOutputLength_isUsed", ty = "u32", bits = "0..=0")]
    pub HMACOutputLength_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub ANY: C2RustUnnamed_22,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_KeyValueType {
    pub DSAKeyValue: din_DSAKeyValueType,
    #[bitfield(name = "DSAKeyValue_isUsed", ty = "u32", bits = "0..=0")]
    pub DSAKeyValue_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub RSAKeyValue: din_RSAKeyValueType,
    #[bitfield(name = "RSAKeyValue_isUsed", ty = "u32", bits = "0..=0")]
    pub RSAKeyValue_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub ANY: C2RustUnnamed_24,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_SubCertificatesType {
    pub Certificate: C2RustUnnamed_25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub bytes: [u8; 1200],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_ProfileEntryType {
    pub ChargingProfileEntryStart: u32,
    pub ChargingProfileEntryMaxPower: i16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_ReferenceType {
    pub Id: C2RustUnnamed_29,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub Type: C2RustUnnamed_28,
    #[bitfield(name = "Type_isUsed", ty = "u32", bits = "0..=0")]
    pub Type_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub URI: C2RustUnnamed_27,
    #[bitfield(name = "URI_isUsed", ty = "u32", bits = "0..=0")]
    pub URI_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 3],
    pub Transforms: din_TransformsType,
    #[bitfield(name = "Transforms_isUsed", ty = "u32", bits = "0..=0")]
    pub Transforms_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 3],
    pub DigestMethod: din_DigestMethodType,
    pub DigestValue: C2RustUnnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_RetrievalMethodType {
    pub Type: C2RustUnnamed_31,
    #[bitfield(name = "Type_isUsed", ty = "u32", bits = "0..=0")]
    pub Type_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub URI: C2RustUnnamed_30,
    #[bitfield(name = "URI_isUsed", ty = "u32", bits = "0..=0")]
    pub URI_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub Transforms: din_TransformsType,
    #[bitfield(name = "Transforms_isUsed", ty = "u32", bits = "0..=0")]
    pub Transforms_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_X509DataType {
    pub X509IssuerSerial: din_X509IssuerSerialType,
    #[bitfield(name = "X509IssuerSerial_isUsed", ty = "u32", bits = "0..=0")]
    pub X509IssuerSerial_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub X509SKI: C2RustUnnamed_36,
    #[bitfield(name = "X509SKI_isUsed", ty = "u32", bits = "0..=0")]
    pub X509SKI_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub X509SubjectName: C2RustUnnamed_35,
    #[bitfield(name = "X509SubjectName_isUsed", ty = "u32", bits = "0..=0")]
    pub X509SubjectName_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub X509Certificate: C2RustUnnamed_34,
    #[bitfield(name = "X509Certificate_isUsed", ty = "u32", bits = "0..=0")]
    pub X509Certificate_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub X509CRL: C2RustUnnamed_33,
    #[bitfield(name = "X509CRL_isUsed", ty = "u32", bits = "0..=0")]
    pub X509CRL_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 1],
    pub ANY: C2RustUnnamed_32,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_32 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_33 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_36 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_PGPDataType {
    pub c2rust_unnamed: C2RustUnnamed_37,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_37 {
    pub choice_1: C2RustUnnamed_41,
    pub choice_1_isUsed: u32,
    pub choice_2: C2RustUnnamed_38,
    pub choice_2_isUsed: u32,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_38 {
    pub PGPKeyPacket: C2RustUnnamed_40,
    pub ANY: C2RustUnnamed_39,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_39 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_40 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_41 {
    pub PGPKeyID: C2RustUnnamed_44,
    pub PGPKeyPacket: C2RustUnnamed_43,
    #[bitfield(name = "PGPKeyPacket_isUsed", ty = "u32", bits = "0..=0")]
    pub PGPKeyPacket_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub ANY: C2RustUnnamed_42,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_42 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_43 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_44 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_SPKIDataType {
    pub SPKISexp: C2RustUnnamed_46,
    pub ANY: C2RustUnnamed_45,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_45 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_46 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_SignedInfoType {
    pub Id: C2RustUnnamed_47,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub CanonicalizationMethod: din_CanonicalizationMethodType,
    pub SignatureMethod: din_SignatureMethodType,
    pub Reference: din_ReferenceType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_47 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_DC_EVStatusType {
    pub EVReady: i32,
    pub EVCabinConditioning: i32,
    #[bitfield(name = "EVCabinConditioning_isUsed", ty = "u32", bits = "0..=0")]
    pub EVCabinConditioning_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub EVRESSConditioning: i32,
    #[bitfield(name = "EVRESSConditioning_isUsed", ty = "u32", bits = "0..=0")]
    pub EVRESSConditioning_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
    pub EVErrorCode: din_DC_EVErrorCodeType,
    pub EVRESSSOC: int8_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_SignatureValueType {
    pub Id: C2RustUnnamed_49,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub CONTENT: C2RustUnnamed_48,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_48 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_49 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_CertificateChainType {
    pub Certificate: C2RustUnnamed_50,
    pub SubCertificates: din_SubCertificatesType,
    #[bitfield(name = "SubCertificates_isUsed", ty = "u32", bits = "0..=0")]
    pub SubCertificates_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_50 {
    pub bytes: [u8; 1200],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_DC_EVSEStatusType {
    pub EVSEIsolationStatus: din_isolationLevelType,
    #[bitfield(name = "EVSEIsolationStatus_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEIsolationStatus_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub EVSEStatusCode: din_DC_EVSEStatusCodeType,
    pub NotificationMaxDelay: u32,
    pub EVSENotification: din_EVSENotificationType,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_PhysicalValueType {
    pub Multiplier: int8_t,
    pub Unit: din_unitSymbolType,
    #[bitfield(name = "Unit_isUsed", ty = "u32", bits = "0..=0")]
    pub Unit_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub Value: i16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_ParameterType {
    pub Name: C2RustUnnamed_52,
    pub ValueType: din_valueType,
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
    pub c2rust_padding_1: [u8; 3],
    pub physicalValue: din_PhysicalValueType,
    #[bitfield(name = "physicalValue_isUsed", ty = "u32", bits = "0..=0")]
    pub physicalValue_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 1],
    pub stringValue: C2RustUnnamed_51,
    #[bitfield(name = "stringValue_isUsed", ty = "u32", bits = "0..=0")]
    pub stringValue_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_51 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_52 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_ParameterSetType {
    pub ParameterSetID: i16,
    pub Parameter: din_ParameterType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_ListOfRootCertificateIDsType {
    pub RootCertificateID: C2RustUnnamed_53,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_53 {
    pub array: [C2RustUnnamed_54; 5],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_54 {
    pub characters: [i8; 41],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_PaymentOptionsType {
    pub PaymentOption: C2RustUnnamed_55,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_55 {
    pub array: [din_paymentOptionType; 2],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_SelectedServiceListType {
    pub SelectedService: C2RustUnnamed_56,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_56 {
    pub array: [din_SelectedServiceType; 16],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_AC_EVChargeParameterType {
    pub DepartureTime: u32,
    pub EAmount: din_PhysicalValueType,
    pub EVMaxVoltage: din_PhysicalValueType,
    pub EVMaxCurrent: din_PhysicalValueType,
    pub EVMinCurrent: din_PhysicalValueType,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_DC_EVChargeParameterType {
    pub DC_EVStatus: din_DC_EVStatusType,
    pub EVMaximumCurrentLimit: din_PhysicalValueType,
    pub EVMaximumPowerLimit: din_PhysicalValueType,
    #[bitfield(name = "EVMaximumPowerLimit_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumPowerLimit_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub EVMaximumVoltageLimit: din_PhysicalValueType,
    pub EVEnergyCapacity: din_PhysicalValueType,
    #[bitfield(name = "EVEnergyCapacity_isUsed", ty = "u32", bits = "0..=0")]
    pub EVEnergyCapacity_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
    pub EVEnergyRequest: din_PhysicalValueType,
    #[bitfield(name = "EVEnergyRequest_isUsed", ty = "u32", bits = "0..=0")]
    pub EVEnergyRequest_isUsed: [u8; 1],
    pub FullSOC: int8_t,
    #[bitfield(name = "FullSOC_isUsed", ty = "u32", bits = "0..=0")]
    pub FullSOC_isUsed: [u8; 1],
    pub BulkSOC: int8_t,
    #[bitfield(name = "BulkSOC_isUsed", ty = "u32", bits = "0..=0")]
    pub BulkSOC_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_EVChargeParameterType {
    pub _unused: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_ChargingProfileType {
    pub SAScheduleTupleID: i16,
    pub ProfileEntry: C2RustUnnamed_57,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_57 {
    pub array: [din_ProfileEntryType; 24],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_EVSEStatusType {
    pub _unused: i32,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_KeyInfoType {
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
    pub KeyValue: din_KeyValueType,
    #[bitfield(name = "KeyValue_isUsed", ty = "u32", bits = "0..=0")]
    pub KeyValue_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 3],
    pub RetrievalMethod: din_RetrievalMethodType,
    #[bitfield(name = "RetrievalMethod_isUsed", ty = "u32", bits = "0..=0")]
    pub RetrievalMethod_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 3],
    pub X509Data: din_X509DataType,
    #[bitfield(name = "X509Data_isUsed", ty = "u32", bits = "0..=0")]
    pub X509Data_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 3],
    pub PGPData: din_PGPDataType,
    #[bitfield(name = "PGPData_isUsed", ty = "u32", bits = "0..=0")]
    pub PGPData_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 3],
    pub SPKIData: din_SPKIDataType,
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
#[repr(C)]
pub struct C2RustUnnamed_58 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_59 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_60 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_61 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_ServiceChargeType {
    pub ServiceTag: din_ServiceTagType,
    pub FreeService: i32,
    pub EnergyTransferType: din_EVSESupportedEnergyTransferType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_ServiceParameterListType {
    pub ParameterSet: C2RustUnnamed_62,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_62 {
    pub array: [din_ParameterSetType; 5],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_SAScheduleListType {
    pub SAScheduleTuple: C2RustUnnamed_63,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_63 {
    pub array: [din_SAScheduleTupleType; 5],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_SASchedulesType {
    pub _unused: i32,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_DC_EVPowerDeliveryParameterType {
    pub DC_EVStatus: din_DC_EVStatusType,
    pub BulkChargingComplete: i32,
    #[bitfield(name = "BulkChargingComplete_isUsed", ty = "u32", bits = "0..=0")]
    pub BulkChargingComplete_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub ChargingComplete: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_EVPowerDeliveryParameterType {
    pub _unused: i32,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_ObjectType {
    pub Encoding: C2RustUnnamed_67,
    #[bitfield(name = "Encoding_isUsed", ty = "u32", bits = "0..=0")]
    pub Encoding_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub Id: C2RustUnnamed_66,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub MimeType: C2RustUnnamed_65,
    #[bitfield(name = "MimeType_isUsed", ty = "u32", bits = "0..=0")]
    pub MimeType_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 1],
    pub ANY: C2RustUnnamed_64,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_64 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_65 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_66 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_67 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_ServiceTagListType {
    pub Service: din_ServiceType,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_DC_EVSEChargeParameterType {
    pub DC_EVSEStatus: din_DC_EVSEStatusType,
    pub EVSEMaximumCurrentLimit: din_PhysicalValueType,
    pub EVSEMaximumPowerLimit: din_PhysicalValueType,
    #[bitfield(name = "EVSEMaximumPowerLimit_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEMaximumPowerLimit_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub EVSEMaximumVoltageLimit: din_PhysicalValueType,
    pub EVSEMinimumCurrentLimit: din_PhysicalValueType,
    pub EVSEMinimumVoltageLimit: din_PhysicalValueType,
    pub EVSECurrentRegulationTolerance: din_PhysicalValueType,
    #[bitfield(
        name = "EVSECurrentRegulationTolerance_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub EVSECurrentRegulationTolerance_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
    pub EVSEPeakCurrentRipple: din_PhysicalValueType,
    pub EVSEEnergyToBeDelivered: din_PhysicalValueType,
    #[bitfield(name = "EVSEEnergyToBeDelivered_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEEnergyToBeDelivered_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_AC_EVSEChargeParameterType {
    pub AC_EVSEStatus: din_AC_EVSEStatusType,
    pub EVSEMaxVoltage: din_PhysicalValueType,
    pub EVSEMaxCurrent: din_PhysicalValueType,
    pub EVSEMinCurrent: din_PhysicalValueType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_EVSEChargeParameterType {
    pub _unused: i32,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_MeterInfoType {
    pub MeterID: C2RustUnnamed_69,
    pub MeterReading: din_PhysicalValueType,
    #[bitfield(name = "MeterReading_isUsed", ty = "u32", bits = "0..=0")]
    pub MeterReading_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub SigMeterReading: C2RustUnnamed_68,
    #[bitfield(name = "SigMeterReading_isUsed", ty = "u32", bits = "0..=0")]
    pub SigMeterReading_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
    pub MeterStatus: i16,
    #[bitfield(name = "MeterStatus_isUsed", ty = "u32", bits = "0..=0")]
    pub MeterStatus_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 7],
    pub TMeter: i64,
    #[bitfield(name = "TMeter_isUsed", ty = "u32", bits = "0..=0")]
    pub TMeter_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_68 {
    pub bytes: [u8; 32],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_69 {
    pub characters: [i8; 33],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_CertificateInstallationResType {
    pub Id: C2RustUnnamed_73,
    pub ResponseCode: din_responseCodeType,
    pub ContractSignatureCertChain: din_CertificateChainType,
    pub ContractSignatureEncryptedPrivateKey: C2RustUnnamed_72,
    pub DHParams: C2RustUnnamed_71,
    pub ContractID: C2RustUnnamed_70,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_70 {
    pub characters: [i8; 25],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_71 {
    pub bytes: [u8; 256],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_72 {
    pub bytes: [u8; 128],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_73 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_CableCheckReqType {
    pub DC_EVStatus: din_DC_EVStatusType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_CableCheckResType {
    pub ResponseCode: din_responseCodeType,
    pub DC_EVSEStatus: din_DC_EVSEStatusType,
    pub EVSEProcessing: din_EVSEProcessingType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_PreChargeReqType {
    pub DC_EVStatus: din_DC_EVStatusType,
    pub EVTargetVoltage: din_PhysicalValueType,
    pub EVTargetCurrent: din_PhysicalValueType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_PreChargeResType {
    pub ResponseCode: din_responseCodeType,
    pub DC_EVSEStatus: din_DC_EVSEStatusType,
    pub EVSEPresentVoltage: din_PhysicalValueType,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_CurrentDemandReqType {
    pub DC_EVStatus: din_DC_EVStatusType,
    pub EVTargetCurrent: din_PhysicalValueType,
    pub EVMaximumVoltageLimit: din_PhysicalValueType,
    #[bitfield(name = "EVMaximumVoltageLimit_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumVoltageLimit_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub EVMaximumCurrentLimit: din_PhysicalValueType,
    #[bitfield(name = "EVMaximumCurrentLimit_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumCurrentLimit_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
    pub EVMaximumPowerLimit: din_PhysicalValueType,
    #[bitfield(name = "EVMaximumPowerLimit_isUsed", ty = "u32", bits = "0..=0")]
    pub EVMaximumPowerLimit_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 3],
    pub BulkChargingComplete: i32,
    #[bitfield(name = "BulkChargingComplete_isUsed", ty = "u32", bits = "0..=0")]
    pub BulkChargingComplete_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 3],
    pub ChargingComplete: i32,
    pub RemainingTimeToFullSoC: din_PhysicalValueType,
    #[bitfield(name = "RemainingTimeToFullSoC_isUsed", ty = "u32", bits = "0..=0")]
    pub RemainingTimeToFullSoC_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 3],
    pub RemainingTimeToBulkSoC: din_PhysicalValueType,
    #[bitfield(name = "RemainingTimeToBulkSoC_isUsed", ty = "u32", bits = "0..=0")]
    pub RemainingTimeToBulkSoC_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 3],
    pub EVTargetVoltage: din_PhysicalValueType,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_CurrentDemandResType {
    pub ResponseCode: din_responseCodeType,
    pub DC_EVSEStatus: din_DC_EVSEStatusType,
    pub EVSEPresentVoltage: din_PhysicalValueType,
    pub EVSEPresentCurrent: din_PhysicalValueType,
    pub EVSECurrentLimitAchieved: i32,
    pub EVSEVoltageLimitAchieved: i32,
    pub EVSEPowerLimitAchieved: i32,
    pub EVSEMaximumVoltageLimit: din_PhysicalValueType,
    #[bitfield(name = "EVSEMaximumVoltageLimit_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEMaximumVoltageLimit_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub EVSEMaximumCurrentLimit: din_PhysicalValueType,
    #[bitfield(name = "EVSEMaximumCurrentLimit_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEMaximumCurrentLimit_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
    pub EVSEMaximumPowerLimit: din_PhysicalValueType,
    #[bitfield(name = "EVSEMaximumPowerLimit_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEMaximumPowerLimit_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_WeldingDetectionReqType {
    pub DC_EVStatus: din_DC_EVStatusType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_WeldingDetectionResType {
    pub ResponseCode: din_responseCodeType,
    pub DC_EVSEStatus: din_DC_EVSEStatusType,
    pub EVSEPresentVoltage: din_PhysicalValueType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_SessionSetupReqType {
    pub EVCCID: C2RustUnnamed_74,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_74 {
    pub bytes: [u8; 8],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_CertificateInstallationReqType {
    pub Id: C2RustUnnamed_77,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub OEMProvisioningCert: C2RustUnnamed_76,
    pub ListOfRootCertificateIDs: din_ListOfRootCertificateIDsType,
    pub DHParams: C2RustUnnamed_75,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_75 {
    pub bytes: [u8; 256],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_76 {
    pub bytes: [u8; 1200],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_77 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_SessionSetupResType {
    pub ResponseCode: din_responseCodeType,
    pub EVSEID: C2RustUnnamed_78,
    pub DateTimeNow: i64,
    #[bitfield(name = "DateTimeNow_isUsed", ty = "u32", bits = "0..=0")]
    pub DateTimeNow_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_78 {
    pub bytes: [u8; 32],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_ServiceDiscoveryReqType {
    pub ServiceScope: C2RustUnnamed_79,
    #[bitfield(name = "ServiceScope_isUsed", ty = "u32", bits = "0..=0")]
    pub ServiceScope_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub ServiceCategory: din_serviceCategoryType,
    #[bitfield(name = "ServiceCategory_isUsed", ty = "u32", bits = "0..=0")]
    pub ServiceCategory_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_79 {
    pub characters: [i8; 33],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_ServiceDiscoveryResType {
    pub ResponseCode: din_responseCodeType,
    pub PaymentOptions: din_PaymentOptionsType,
    pub ChargeService: din_ServiceChargeType,
    pub ServiceList: din_ServiceTagListType,
    #[bitfield(name = "ServiceList_isUsed", ty = "u32", bits = "0..=0")]
    pub ServiceList_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_ServiceDetailReqType {
    pub ServiceID: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_ServiceDetailResType {
    pub ResponseCode: din_responseCodeType,
    pub ServiceID: u16,
    pub ServiceParameterList: din_ServiceParameterListType,
    #[bitfield(name = "ServiceParameterList_isUsed", ty = "u32", bits = "0..=0")]
    pub ServiceParameterList_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_ServicePaymentSelectionReqType {
    pub SelectedPaymentOption: din_paymentOptionType,
    pub SelectedServiceList: din_SelectedServiceListType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_ServicePaymentSelectionResType {
    pub ResponseCode: din_responseCodeType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_PaymentDetailsReqType {
    pub ContractID: C2RustUnnamed_80,
    pub ContractSignatureCertChain: din_CertificateChainType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_80 {
    pub characters: [i8; 25],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_PaymentDetailsResType {
    pub ResponseCode: din_responseCodeType,
    pub GenChallenge: C2RustUnnamed_81,
    pub DateTimeNow: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_81 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_ContractAuthenticationReqType {
    pub Id: C2RustUnnamed_83,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub GenChallenge: C2RustUnnamed_82,
    #[bitfield(name = "GenChallenge_isUsed", ty = "u32", bits = "0..=0")]
    pub GenChallenge_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_82 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_83 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_ContractAuthenticationResType {
    pub ResponseCode: din_responseCodeType,
    pub EVSEProcessing: din_EVSEProcessingType,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_ChargeParameterDiscoveryReqType {
    pub EVRequestedEnergyTransferType: din_EVRequestedEnergyTransferType,
    pub AC_EVChargeParameter: din_AC_EVChargeParameterType,
    #[bitfield(name = "AC_EVChargeParameter_isUsed", ty = "u32", bits = "0..=0")]
    pub AC_EVChargeParameter_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub DC_EVChargeParameter: din_DC_EVChargeParameterType,
    #[bitfield(name = "DC_EVChargeParameter_isUsed", ty = "u32", bits = "0..=0")]
    pub DC_EVChargeParameter_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
    pub EVChargeParameter: din_EVChargeParameterType,
    #[bitfield(name = "EVChargeParameter_isUsed", ty = "u32", bits = "0..=0")]
    pub EVChargeParameter_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 3],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_ChargeParameterDiscoveryResType {
    pub ResponseCode: din_responseCodeType,
    pub EVSEProcessing: din_EVSEProcessingType,
    pub SAScheduleList: din_SAScheduleListType,
    #[bitfield(name = "SAScheduleList_isUsed", ty = "u32", bits = "0..=0")]
    pub SAScheduleList_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub SASchedules: din_SASchedulesType,
    #[bitfield(name = "SASchedules_isUsed", ty = "u32", bits = "0..=0")]
    pub SASchedules_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
    pub AC_EVSEChargeParameter: din_AC_EVSEChargeParameterType,
    #[bitfield(name = "AC_EVSEChargeParameter_isUsed", ty = "u32", bits = "0..=0")]
    pub AC_EVSEChargeParameter_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 3],
    pub DC_EVSEChargeParameter: din_DC_EVSEChargeParameterType,
    #[bitfield(name = "DC_EVSEChargeParameter_isUsed", ty = "u32", bits = "0..=0")]
    pub DC_EVSEChargeParameter_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 3],
    pub EVSEChargeParameter: din_EVSEChargeParameterType,
    #[bitfield(name = "EVSEChargeParameter_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEChargeParameter_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 3],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_PowerDeliveryReqType {
    pub ReadyToChargeState: i32,
    pub ChargingProfile: din_ChargingProfileType,
    #[bitfield(name = "ChargingProfile_isUsed", ty = "u32", bits = "0..=0")]
    pub ChargingProfile_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub DC_EVPowerDeliveryParameter: din_DC_EVPowerDeliveryParameterType,
    #[bitfield(
        name = "DC_EVPowerDeliveryParameter_isUsed",
        ty = "u32",
        bits = "0..=0"
    )]
    pub DC_EVPowerDeliveryParameter_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
    pub EVPowerDeliveryParameter: din_EVPowerDeliveryParameterType,
    #[bitfield(name = "EVPowerDeliveryParameter_isUsed", ty = "u32", bits = "0..=0")]
    pub EVPowerDeliveryParameter_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 3],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_PowerDeliveryResType {
    pub ResponseCode: din_responseCodeType,
    pub AC_EVSEStatus: din_AC_EVSEStatusType,
    #[bitfield(name = "AC_EVSEStatus_isUsed", ty = "u32", bits = "0..=0")]
    pub AC_EVSEStatus_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub DC_EVSEStatus: din_DC_EVSEStatusType,
    #[bitfield(name = "DC_EVSEStatus_isUsed", ty = "u32", bits = "0..=0")]
    pub DC_EVSEStatus_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
    pub EVSEStatus: din_EVSEStatusType,
    #[bitfield(name = "EVSEStatus_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEStatus_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_ChargingStatusReqType {
    pub _unused: i32,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_ChargingStatusResType {
    pub ResponseCode: din_responseCodeType,
    pub EVSEID: C2RustUnnamed_84,
    pub SAScheduleTupleID: i16,
    pub EVSEMaxCurrent: din_PhysicalValueType,
    #[bitfield(name = "EVSEMaxCurrent_isUsed", ty = "u32", bits = "0..=0")]
    pub EVSEMaxCurrent_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub MeterInfo: din_MeterInfoType,
    #[bitfield(name = "MeterInfo_isUsed", ty = "u32", bits = "0..=0")]
    pub MeterInfo_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
    pub ReceiptRequired: i32,
    pub AC_EVSEStatus: din_AC_EVSEStatusType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_84 {
    pub bytes: [u8; 32],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_MeteringReceiptReqType {
    pub Id: C2RustUnnamed_86,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub SessionID: C2RustUnnamed_85,
    pub SAScheduleTupleID: i16,
    #[bitfield(name = "SAScheduleTupleID_isUsed", ty = "u32", bits = "0..=0")]
    pub SAScheduleTupleID_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 5],
    pub MeterInfo: din_MeterInfoType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_85 {
    pub bytes: [u8; 8],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_86 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_MeteringReceiptResType {
    pub ResponseCode: din_responseCodeType,
    pub AC_EVSEStatus: din_AC_EVSEStatusType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_SessionStopType {
    pub _unused: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_SessionStopResType {
    pub ResponseCode: din_responseCodeType,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_CertificateUpdateReqType {
    pub Id: C2RustUnnamed_89,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub ContractSignatureCertChain: din_CertificateChainType,
    pub ContractID: C2RustUnnamed_88,
    pub ListOfRootCertificateIDs: din_ListOfRootCertificateIDsType,
    pub DHParams: C2RustUnnamed_87,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_87 {
    pub bytes: [u8; 256],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_88 {
    pub characters: [i8; 25],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_89 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_CertificateUpdateResType {
    pub Id: C2RustUnnamed_93,
    pub ResponseCode: din_responseCodeType,
    pub ContractSignatureCertChain: din_CertificateChainType,
    pub ContractSignatureEncryptedPrivateKey: C2RustUnnamed_92,
    pub DHParams: C2RustUnnamed_91,
    pub ContractID: C2RustUnnamed_90,
    pub RetryCounter: i16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_90 {
    pub characters: [i8; 25],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_91 {
    pub bytes: [u8; 256],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_92 {
    pub bytes: [u8; 128],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_93 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_BodyBaseType {
    pub _unused: i32,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_NotificationType {
    pub FaultCode: din_faultCodeType,
    pub FaultMsg: C2RustUnnamed_94,
    #[bitfield(name = "FaultMsg_isUsed", ty = "u32", bits = "0..=0")]
    pub FaultMsg_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_94 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_SignatureType {
    pub Id: C2RustUnnamed_95,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub SignedInfo: din_SignedInfoType,
    pub SignatureValue: din_SignatureValueType,
    pub KeyInfo: din_KeyInfoType,
    #[bitfield(name = "KeyInfo_isUsed", ty = "u32", bits = "0..=0")]
    pub KeyInfo_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
    pub Object: din_ObjectType,
    #[bitfield(name = "Object_isUsed", ty = "u32", bits = "0..=0")]
    pub Object_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_95 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_MessageHeaderType {
    pub SessionID: C2RustUnnamed_96,
    pub Notification: din_NotificationType,
    #[bitfield(name = "Notification_isUsed", ty = "u32", bits = "0..=0")]
    pub Notification_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub Signature: din_SignatureType,
    #[bitfield(name = "Signature_isUsed", ty = "u32", bits = "0..=0")]
    pub Signature_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_96 {
    pub bytes: [u8; 8],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct din_BodyType {
    pub c2rust_unnamed: C2RustUnnamed_97,
    #[bitfield(name = "BodyElement_isUsed", ty = "u32", bits = "0..=0")]
    #[bitfield(name = "CableCheckReq_isUsed", ty = "u32", bits = "1..=1")]
    #[bitfield(name = "CableCheckRes_isUsed", ty = "u32", bits = "2..=2")]
    #[bitfield(name = "CertificateInstallationReq_isUsed", ty = "u32", bits = "3..=3")]
    #[bitfield(name = "CertificateInstallationRes_isUsed", ty = "u32", bits = "4..=4")]
    #[bitfield(name = "CertificateUpdateReq_isUsed", ty = "u32", bits = "5..=5")]
    #[bitfield(name = "CertificateUpdateRes_isUsed", ty = "u32", bits = "6..=6")]
    #[bitfield(
        name = "ChargeParameterDiscoveryReq_isUsed",
        ty = "u32",
        bits = "7..=7"
    )]
    #[bitfield(
        name = "ChargeParameterDiscoveryRes_isUsed",
        ty = "u32",
        bits = "8..=8"
    )]
    #[bitfield(name = "ChargingStatusReq_isUsed", ty = "u32", bits = "9..=9")]
    #[bitfield(name = "ChargingStatusRes_isUsed", ty = "u32", bits = "10..=10")]
    #[bitfield(
        name = "ContractAuthenticationReq_isUsed",
        ty = "u32",
        bits = "11..=11"
    )]
    #[bitfield(
        name = "ContractAuthenticationRes_isUsed",
        ty = "u32",
        bits = "12..=12"
    )]
    #[bitfield(name = "CurrentDemandReq_isUsed", ty = "u32", bits = "13..=13")]
    #[bitfield(name = "CurrentDemandRes_isUsed", ty = "u32", bits = "14..=14")]
    #[bitfield(name = "MeteringReceiptReq_isUsed", ty = "u32", bits = "15..=15")]
    #[bitfield(name = "MeteringReceiptRes_isUsed", ty = "u32", bits = "16..=16")]
    #[bitfield(name = "PaymentDetailsReq_isUsed", ty = "u32", bits = "17..=17")]
    #[bitfield(name = "PaymentDetailsRes_isUsed", ty = "u32", bits = "18..=18")]
    #[bitfield(name = "PowerDeliveryReq_isUsed", ty = "u32", bits = "19..=19")]
    #[bitfield(name = "PowerDeliveryRes_isUsed", ty = "u32", bits = "20..=20")]
    #[bitfield(name = "PreChargeReq_isUsed", ty = "u32", bits = "21..=21")]
    #[bitfield(name = "PreChargeRes_isUsed", ty = "u32", bits = "22..=22")]
    #[bitfield(name = "ServiceDetailReq_isUsed", ty = "u32", bits = "23..=23")]
    #[bitfield(name = "ServiceDetailRes_isUsed", ty = "u32", bits = "24..=24")]
    #[bitfield(name = "ServiceDiscoveryReq_isUsed", ty = "u32", bits = "25..=25")]
    #[bitfield(name = "ServiceDiscoveryRes_isUsed", ty = "u32", bits = "26..=26")]
    #[bitfield(
        name = "ServicePaymentSelectionReq_isUsed",
        ty = "u32",
        bits = "27..=27"
    )]
    #[bitfield(
        name = "ServicePaymentSelectionRes_isUsed",
        ty = "u32",
        bits = "28..=28"
    )]
    #[bitfield(name = "SessionSetupReq_isUsed", ty = "u32", bits = "29..=29")]
    #[bitfield(name = "SessionSetupRes_isUsed", ty = "u32", bits = "30..=30")]
    #[bitfield(name = "SessionStopReq_isUsed", ty = "u32", bits = "31..=31")]
    #[bitfield(name = "SessionStopRes_isUsed", ty = "u32", bits = "32..=32")]
    #[bitfield(name = "WeldingDetectionReq_isUsed", ty = "u32", bits = "33..=33")]
    #[bitfield(name = "WeldingDetectionRes_isUsed", ty = "u32", bits = "34..=34")]
    pub BodyElement_isUsed_CableCheckReq_isUsed_CableCheckRes_isUsed_CertificateInstallationReq_isUsed_CertificateInstallationRes_isUsed_CertificateUpdateReq_isUsed_CertificateUpdateRes_isUsed_ChargeParameterDiscoveryReq_isUsed_ChargeParameterDiscoveryRes_isUsed_ChargingStatusReq_isUsed_ChargingStatusRes_isUsed_ContractAuthenticationReq_isUsed_ContractAuthenticationRes_isUsed_CurrentDemandReq_isUsed_CurrentDemandRes_isUsed_MeteringReceiptReq_isUsed_MeteringReceiptRes_isUsed_PaymentDetailsReq_isUsed_PaymentDetailsRes_isUsed_PowerDeliveryReq_isUsed_PowerDeliveryRes_isUsed_PreChargeReq_isUsed_PreChargeRes_isUsed_ServiceDetailReq_isUsed_ServiceDetailRes_isUsed_ServiceDiscoveryReq_isUsed_ServiceDiscoveryRes_isUsed_ServicePaymentSelectionReq_isUsed_ServicePaymentSelectionRes_isUsed_SessionSetupReq_isUsed_SessionSetupRes_isUsed_SessionStopReq_isUsed_SessionStopRes_isUsed_WeldingDetectionReq_isUsed_WeldingDetectionRes_isUsed:
        [u8; 5],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_97 {
    pub BodyElement: din_BodyBaseType,
    pub CableCheckReq: din_CableCheckReqType,
    pub CableCheckRes: din_CableCheckResType,
    pub CertificateInstallationReq: din_CertificateInstallationReqType,
    pub CertificateInstallationRes: din_CertificateInstallationResType,
    pub CertificateUpdateReq: din_CertificateUpdateReqType,
    pub CertificateUpdateRes: din_CertificateUpdateResType,
    pub ChargeParameterDiscoveryReq: din_ChargeParameterDiscoveryReqType,
    pub ChargeParameterDiscoveryRes: din_ChargeParameterDiscoveryResType,
    pub ChargingStatusReq: din_ChargingStatusReqType,
    pub ChargingStatusRes: din_ChargingStatusResType,
    pub ContractAuthenticationReq: din_ContractAuthenticationReqType,
    pub ContractAuthenticationRes: din_ContractAuthenticationResType,
    pub CurrentDemandReq: din_CurrentDemandReqType,
    pub CurrentDemandRes: din_CurrentDemandResType,
    pub MeteringReceiptReq: din_MeteringReceiptReqType,
    pub MeteringReceiptRes: din_MeteringReceiptResType,
    pub PaymentDetailsReq: din_PaymentDetailsReqType,
    pub PaymentDetailsRes: din_PaymentDetailsResType,
    pub PowerDeliveryReq: din_PowerDeliveryReqType,
    pub PowerDeliveryRes: din_PowerDeliveryResType,
    pub PreChargeReq: din_PreChargeReqType,
    pub PreChargeRes: din_PreChargeResType,
    pub ServiceDetailReq: din_ServiceDetailReqType,
    pub ServiceDetailRes: din_ServiceDetailResType,
    pub ServiceDiscoveryReq: din_ServiceDiscoveryReqType,
    pub ServiceDiscoveryRes: din_ServiceDiscoveryResType,
    pub ServicePaymentSelectionReq: din_ServicePaymentSelectionReqType,
    pub ServicePaymentSelectionRes: din_ServicePaymentSelectionResType,
    pub SessionSetupReq: din_SessionSetupReqType,
    pub SessionSetupRes: din_SessionSetupResType,
    pub SessionStopReq: din_SessionStopType,
    pub SessionStopRes: din_SessionStopResType,
    pub WeldingDetectionReq: din_WeldingDetectionReqType,
    pub WeldingDetectionRes: din_WeldingDetectionResType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_V2G_Message {
    pub Header: din_MessageHeaderType,
    pub Body: din_BodyType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct din_exiDocument {
    pub V2G_Message: din_V2G_Message,
}
#[no_mangle]
pub unsafe extern "C" fn init_din_exiDocument(mut exiDoc: *mut din_exiDocument) {}
#[no_mangle]
pub unsafe extern "C" fn init_din_CostType(mut CostType: *mut din_CostType) {
    (*CostType).set_amountMultiplier_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_RelativeTimeIntervalType(
    mut RelativeTimeIntervalType: *mut din_RelativeTimeIntervalType,
) {
    (*RelativeTimeIntervalType).set_duration_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_IntervalType(mut IntervalType: *mut din_IntervalType) {}
#[no_mangle]
pub unsafe extern "C" fn init_din_ConsumptionCostType(
    mut ConsumptionCostType: *mut din_ConsumptionCostType,
) {
    (*ConsumptionCostType).set_Cost_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_TransformType(mut TransformType: *mut din_TransformType) {
    (*TransformType).set_ANY_isUsed(0 as u32);
    (*TransformType).set_XPath_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_PMaxScheduleEntryType(
    mut PMaxScheduleEntryType: *mut din_PMaxScheduleEntryType,
) {
    (*PMaxScheduleEntryType).set_RelativeTimeInterval_isUsed(0 as u32);
    (*PMaxScheduleEntryType).set_TimeInterval_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_SalesTariffEntryType(
    mut SalesTariffEntryType: *mut din_SalesTariffEntryType,
) {
    (*SalesTariffEntryType).set_RelativeTimeInterval_isUsed(0 as u32);
    (*SalesTariffEntryType).set_TimeInterval_isUsed(0 as u32);
    (*SalesTariffEntryType).set_ConsumptionCost_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_TransformsType(mut TransformsType: *mut din_TransformsType) {}
#[no_mangle]
pub unsafe extern "C" fn init_din_DSAKeyValueType(mut DSAKeyValueType: *mut din_DSAKeyValueType) {
    (*DSAKeyValueType).set_P_isUsed(0 as u32);
    (*DSAKeyValueType).set_Q_isUsed(0 as u32);
    (*DSAKeyValueType).set_G_isUsed(0 as u32);
    (*DSAKeyValueType).set_J_isUsed(0 as u32);
    (*DSAKeyValueType).set_Seed_isUsed(0 as u32);
    (*DSAKeyValueType).set_PgenCounter_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_X509IssuerSerialType(
    mut X509IssuerSerialType: *mut din_X509IssuerSerialType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_DigestMethodType(
    mut DigestMethodType: *mut din_DigestMethodType,
) {
    (*DigestMethodType).set_ANY_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_RSAKeyValueType(mut RSAKeyValueType: *mut din_RSAKeyValueType) {}
#[no_mangle]
pub unsafe extern "C" fn init_din_ParameterType(mut ParameterType: *mut din_ParameterType) {
    (*ParameterType).set_boolValue_isUsed(0 as u32);
    (*ParameterType).set_byteValue_isUsed(0 as u32);
    (*ParameterType).set_shortValue_isUsed(0 as u32);
    (*ParameterType).set_intValue_isUsed(0 as u32);
    (*ParameterType).set_physicalValue_isUsed(0 as u32);
    (*ParameterType).set_stringValue_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_PMaxScheduleType(
    mut PMaxScheduleType: *mut din_PMaxScheduleType,
) {
    (*PMaxScheduleType).PMaxScheduleEntry.arrayLen = 0 as u32 as u16;
}
#[no_mangle]
pub unsafe extern "C" fn init_din_SalesTariffType(mut SalesTariffType: *mut din_SalesTariffType) {
    (*SalesTariffType).SalesTariffEntry.arrayLen = 0 as u32 as u16;
    (*SalesTariffType).set_SalesTariffDescription_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_CanonicalizationMethodType(
    mut CanonicalizationMethodType: *mut din_CanonicalizationMethodType,
) {
    (*CanonicalizationMethodType).set_ANY_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_ServiceTagType(mut ServiceTagType: *mut din_ServiceTagType) {
    (*ServiceTagType).set_ServiceName_isUsed(0 as u32);
    (*ServiceTagType).set_ServiceScope_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_ServiceType(mut ServiceType: *mut din_ServiceType) {}
#[no_mangle]
pub unsafe extern "C" fn init_din_ParameterSetType(
    mut ParameterSetType: *mut din_ParameterSetType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_SelectedServiceType(
    mut SelectedServiceType: *mut din_SelectedServiceType,
) {
    (*SelectedServiceType).set_ParameterSetID_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_SAScheduleTupleType(
    mut SAScheduleTupleType: *mut din_SAScheduleTupleType,
) {
    (*SAScheduleTupleType).set_SalesTariff_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_AC_EVSEStatusType(
    mut AC_EVSEStatusType: *mut din_AC_EVSEStatusType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_SignatureMethodType(
    mut SignatureMethodType: *mut din_SignatureMethodType,
) {
    (*SignatureMethodType).set_HMACOutputLength_isUsed(0 as u32);
    (*SignatureMethodType).set_ANY_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_KeyValueType(mut KeyValueType: *mut din_KeyValueType) {
    (*KeyValueType).set_DSAKeyValue_isUsed(0 as u32);
    (*KeyValueType).set_RSAKeyValue_isUsed(0 as u32);
    (*KeyValueType).set_ANY_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_SubCertificatesType(
    mut SubCertificatesType: *mut din_SubCertificatesType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_ProfileEntryType(
    mut ProfileEntryType: *mut din_ProfileEntryType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_ReferenceType(mut ReferenceType: *mut din_ReferenceType) {
    (*ReferenceType).set_Id_isUsed(0 as u32);
    (*ReferenceType).set_Type_isUsed(0 as u32);
    (*ReferenceType).set_URI_isUsed(0 as u32);
    (*ReferenceType).set_Transforms_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_RetrievalMethodType(
    mut RetrievalMethodType: *mut din_RetrievalMethodType,
) {
    (*RetrievalMethodType).set_Type_isUsed(0 as u32);
    (*RetrievalMethodType).set_URI_isUsed(0 as u32);
    (*RetrievalMethodType).set_Transforms_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_X509DataType(mut X509DataType: *mut din_X509DataType) {
    (*X509DataType).set_X509IssuerSerial_isUsed(0 as u32);
    (*X509DataType).set_X509SKI_isUsed(0 as u32);
    (*X509DataType).set_X509SubjectName_isUsed(0 as u32);
    (*X509DataType).set_X509Certificate_isUsed(0 as u32);
    (*X509DataType).set_X509CRL_isUsed(0 as u32);
    (*X509DataType).set_ANY_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_PGPDataType(mut PGPDataType: *mut din_PGPDataType) {
    ((*PGPDataType).c2rust_unnamed).choice_1_isUsed = 0 as u32;
    ((*PGPDataType).c2rust_unnamed).choice_2_isUsed = 0 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn init_din_SPKIDataType(mut SPKIDataType: *mut din_SPKIDataType) {
    (*SPKIDataType).set_ANY_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_SignedInfoType(mut SignedInfoType: *mut din_SignedInfoType) {
    (*SignedInfoType).set_Id_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_DC_EVStatusType(mut DC_EVStatusType: *mut din_DC_EVStatusType) {
    (*DC_EVStatusType).set_EVCabinConditioning_isUsed(0 as u32);
    (*DC_EVStatusType).set_EVRESSConditioning_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_SignatureValueType(
    mut SignatureValueType: *mut din_SignatureValueType,
) {
    (*SignatureValueType).set_Id_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_CertificateChainType(
    mut CertificateChainType: *mut din_CertificateChainType,
) {
    (*CertificateChainType).set_SubCertificates_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_DC_EVSEStatusType(
    mut DC_EVSEStatusType: *mut din_DC_EVSEStatusType,
) {
    (*DC_EVSEStatusType).set_EVSEIsolationStatus_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_PhysicalValueType(
    mut PhysicalValueType: *mut din_PhysicalValueType,
) {
    (*PhysicalValueType).set_Unit_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_ListOfRootCertificateIDsType(
    mut ListOfRootCertificateIDsType: *mut din_ListOfRootCertificateIDsType,
) {
    (*ListOfRootCertificateIDsType).RootCertificateID.arrayLen = 0 as u32 as u16;
}
#[no_mangle]
pub unsafe extern "C" fn init_din_PaymentOptionsType(
    mut PaymentOptionsType: *mut din_PaymentOptionsType,
) {
    (*PaymentOptionsType).PaymentOption.arrayLen = 0 as u32 as u16;
}
#[no_mangle]
pub unsafe extern "C" fn init_din_SelectedServiceListType(
    mut SelectedServiceListType: *mut din_SelectedServiceListType,
) {
    (*SelectedServiceListType).SelectedService.arrayLen = 0 as u32 as u16;
}
#[no_mangle]
pub unsafe extern "C" fn init_din_AC_EVChargeParameterType(
    mut AC_EVChargeParameterType: *mut din_AC_EVChargeParameterType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_DC_EVChargeParameterType(
    mut DC_EVChargeParameterType: *mut din_DC_EVChargeParameterType,
) {
    (*DC_EVChargeParameterType).set_EVMaximumPowerLimit_isUsed(0 as u32);
    (*DC_EVChargeParameterType).set_EVEnergyCapacity_isUsed(0 as u32);
    (*DC_EVChargeParameterType).set_EVEnergyRequest_isUsed(0 as u32);
    (*DC_EVChargeParameterType).set_FullSOC_isUsed(0 as u32);
    (*DC_EVChargeParameterType).set_BulkSOC_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_EVChargeParameterType(
    mut EVChargeParameterType: *mut din_EVChargeParameterType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_ChargingProfileType(
    mut ChargingProfileType: *mut din_ChargingProfileType,
) {
    (*ChargingProfileType).ProfileEntry.arrayLen = 0 as u32 as u16;
}
#[no_mangle]
pub unsafe extern "C" fn init_din_EVSEStatusType(mut EVSEStatusType: *mut din_EVSEStatusType) {}
#[no_mangle]
pub unsafe extern "C" fn init_din_KeyInfoType(mut KeyInfoType: *mut din_KeyInfoType) {
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
#[no_mangle]
pub unsafe extern "C" fn init_din_ServiceChargeType(
    mut ServiceChargeType: *mut din_ServiceChargeType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_ServiceParameterListType(
    mut ServiceParameterListType: *mut din_ServiceParameterListType,
) {
    (*ServiceParameterListType).ParameterSet.arrayLen = 0 as u32 as u16;
}
#[no_mangle]
pub unsafe extern "C" fn init_din_SAScheduleListType(
    mut SAScheduleListType: *mut din_SAScheduleListType,
) {
    (*SAScheduleListType).SAScheduleTuple.arrayLen = 0 as u32 as u16;
}
#[no_mangle]
pub unsafe extern "C" fn init_din_SASchedulesType(mut SASchedulesType: *mut din_SASchedulesType) {}
#[no_mangle]
pub unsafe extern "C" fn init_din_DC_EVPowerDeliveryParameterType(
    mut DC_EVPowerDeliveryParameterType: *mut din_DC_EVPowerDeliveryParameterType,
) {
    (*DC_EVPowerDeliveryParameterType).set_BulkChargingComplete_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_EVPowerDeliveryParameterType(
    mut EVPowerDeliveryParameterType: *mut din_EVPowerDeliveryParameterType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_ObjectType(mut ObjectType: *mut din_ObjectType) {
    (*ObjectType).set_Encoding_isUsed(0 as u32);
    (*ObjectType).set_Id_isUsed(0 as u32);
    (*ObjectType).set_MimeType_isUsed(0 as u32);
    (*ObjectType).set_ANY_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_ServiceTagListType(
    mut ServiceTagListType: *mut din_ServiceTagListType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_DC_EVSEChargeParameterType(
    mut DC_EVSEChargeParameterType: *mut din_DC_EVSEChargeParameterType,
) {
    (*DC_EVSEChargeParameterType).set_EVSEMaximumPowerLimit_isUsed(0 as u32);
    (*DC_EVSEChargeParameterType).set_EVSECurrentRegulationTolerance_isUsed(0 as u32);
    (*DC_EVSEChargeParameterType).set_EVSEEnergyToBeDelivered_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_AC_EVSEChargeParameterType(
    mut AC_EVSEChargeParameterType: *mut din_AC_EVSEChargeParameterType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_EVSEChargeParameterType(
    mut EVSEChargeParameterType: *mut din_EVSEChargeParameterType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_MeterInfoType(mut MeterInfoType: *mut din_MeterInfoType) {
    (*MeterInfoType).set_MeterReading_isUsed(0 as u32);
    (*MeterInfoType).set_SigMeterReading_isUsed(0 as u32);
    (*MeterInfoType).set_MeterStatus_isUsed(0 as u32);
    (*MeterInfoType).set_TMeter_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_CertificateInstallationResType(
    mut CertificateInstallationResType: *mut din_CertificateInstallationResType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_CableCheckReqType(
    mut CableCheckReqType: *mut din_CableCheckReqType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_CableCheckResType(
    mut CableCheckResType: *mut din_CableCheckResType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_PreChargeReqType(
    mut PreChargeReqType: *mut din_PreChargeReqType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_PreChargeResType(
    mut PreChargeResType: *mut din_PreChargeResType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_CurrentDemandReqType(
    mut CurrentDemandReqType: *mut din_CurrentDemandReqType,
) {
    (*CurrentDemandReqType).set_EVMaximumVoltageLimit_isUsed(0 as u32);
    (*CurrentDemandReqType).set_EVMaximumCurrentLimit_isUsed(0 as u32);
    (*CurrentDemandReqType).set_EVMaximumPowerLimit_isUsed(0 as u32);
    (*CurrentDemandReqType).set_BulkChargingComplete_isUsed(0 as u32);
    (*CurrentDemandReqType).set_RemainingTimeToFullSoC_isUsed(0 as u32);
    (*CurrentDemandReqType).set_RemainingTimeToBulkSoC_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_CurrentDemandResType(
    mut CurrentDemandResType: *mut din_CurrentDemandResType,
) {
    (*CurrentDemandResType).set_EVSEMaximumVoltageLimit_isUsed(0 as u32);
    (*CurrentDemandResType).set_EVSEMaximumCurrentLimit_isUsed(0 as u32);
    (*CurrentDemandResType).set_EVSEMaximumPowerLimit_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_WeldingDetectionReqType(
    mut WeldingDetectionReqType: *mut din_WeldingDetectionReqType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_WeldingDetectionResType(
    mut WeldingDetectionResType: *mut din_WeldingDetectionResType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_SessionSetupReqType(
    mut SessionSetupReqType: *mut din_SessionSetupReqType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_CertificateInstallationReqType(
    mut CertificateInstallationReqType: *mut din_CertificateInstallationReqType,
) {
    (*CertificateInstallationReqType).set_Id_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_SessionSetupResType(
    mut SessionSetupResType: *mut din_SessionSetupResType,
) {
    (*SessionSetupResType).set_DateTimeNow_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_ServiceDiscoveryReqType(
    mut ServiceDiscoveryReqType: *mut din_ServiceDiscoveryReqType,
) {
    (*ServiceDiscoveryReqType).set_ServiceScope_isUsed(0 as u32);
    (*ServiceDiscoveryReqType).set_ServiceCategory_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_ServiceDiscoveryResType(
    mut ServiceDiscoveryResType: *mut din_ServiceDiscoveryResType,
) {
    (*ServiceDiscoveryResType).set_ServiceList_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_ServiceDetailReqType(
    mut ServiceDetailReqType: *mut din_ServiceDetailReqType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_ServiceDetailResType(
    mut ServiceDetailResType: *mut din_ServiceDetailResType,
) {
    (*ServiceDetailResType).set_ServiceParameterList_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_ServicePaymentSelectionReqType(
    mut ServicePaymentSelectionReqType: *mut din_ServicePaymentSelectionReqType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_ServicePaymentSelectionResType(
    mut ServicePaymentSelectionResType: *mut din_ServicePaymentSelectionResType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_PaymentDetailsReqType(
    mut PaymentDetailsReqType: *mut din_PaymentDetailsReqType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_PaymentDetailsResType(
    mut PaymentDetailsResType: *mut din_PaymentDetailsResType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_ContractAuthenticationReqType(
    mut ContractAuthenticationReqType: *mut din_ContractAuthenticationReqType,
) {
    (*ContractAuthenticationReqType).set_Id_isUsed(0 as u32);
    (*ContractAuthenticationReqType).set_GenChallenge_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_ContractAuthenticationResType(
    mut ContractAuthenticationResType: *mut din_ContractAuthenticationResType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_ChargeParameterDiscoveryReqType(
    mut ChargeParameterDiscoveryReqType: *mut din_ChargeParameterDiscoveryReqType,
) {
    (*ChargeParameterDiscoveryReqType).set_AC_EVChargeParameter_isUsed(0 as u32);
    (*ChargeParameterDiscoveryReqType).set_DC_EVChargeParameter_isUsed(0 as u32);
    (*ChargeParameterDiscoveryReqType).set_EVChargeParameter_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_ChargeParameterDiscoveryResType(
    mut ChargeParameterDiscoveryResType: *mut din_ChargeParameterDiscoveryResType,
) {
    (*ChargeParameterDiscoveryResType).set_SAScheduleList_isUsed(0 as u32);
    (*ChargeParameterDiscoveryResType).set_SASchedules_isUsed(0 as u32);
    (*ChargeParameterDiscoveryResType).set_AC_EVSEChargeParameter_isUsed(0 as u32);
    (*ChargeParameterDiscoveryResType).set_DC_EVSEChargeParameter_isUsed(0 as u32);
    (*ChargeParameterDiscoveryResType).set_EVSEChargeParameter_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_PowerDeliveryReqType(
    mut PowerDeliveryReqType: *mut din_PowerDeliveryReqType,
) {
    (*PowerDeliveryReqType).set_ChargingProfile_isUsed(0 as u32);
    (*PowerDeliveryReqType).set_DC_EVPowerDeliveryParameter_isUsed(0 as u32);
    (*PowerDeliveryReqType).set_EVPowerDeliveryParameter_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_PowerDeliveryResType(
    mut PowerDeliveryResType: *mut din_PowerDeliveryResType,
) {
    (*PowerDeliveryResType).set_AC_EVSEStatus_isUsed(0 as u32);
    (*PowerDeliveryResType).set_DC_EVSEStatus_isUsed(0 as u32);
    (*PowerDeliveryResType).set_EVSEStatus_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_ChargingStatusReqType(
    mut ChargingStatusReqType: *mut din_ChargingStatusReqType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_ChargingStatusResType(
    mut ChargingStatusResType: *mut din_ChargingStatusResType,
) {
    (*ChargingStatusResType).set_EVSEMaxCurrent_isUsed(0 as u32);
    (*ChargingStatusResType).set_MeterInfo_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_MeteringReceiptReqType(
    mut MeteringReceiptReqType: *mut din_MeteringReceiptReqType,
) {
    (*MeteringReceiptReqType).set_Id_isUsed(0 as u32);
    (*MeteringReceiptReqType).set_SAScheduleTupleID_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_MeteringReceiptResType(
    mut MeteringReceiptResType: *mut din_MeteringReceiptResType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_SessionStopType(mut SessionStopType: *mut din_SessionStopType) {}
#[no_mangle]
pub unsafe extern "C" fn init_din_SessionStopResType(
    mut SessionStopResType: *mut din_SessionStopResType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_CertificateUpdateReqType(
    mut CertificateUpdateReqType: *mut din_CertificateUpdateReqType,
) {
    (*CertificateUpdateReqType).set_Id_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_CertificateUpdateResType(
    mut CertificateUpdateResType: *mut din_CertificateUpdateResType,
) {
}
#[no_mangle]
pub unsafe extern "C" fn init_din_BodyBaseType(mut BodyBaseType: *mut din_BodyBaseType) {}
#[no_mangle]
pub unsafe extern "C" fn init_din_NotificationType(
    mut NotificationType: *mut din_NotificationType,
) {
    (*NotificationType).set_FaultMsg_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_SignatureType(mut SignatureType: *mut din_SignatureType) {
    (*SignatureType).set_Id_isUsed(0 as u32);
    (*SignatureType).set_KeyInfo_isUsed(0 as u32);
    (*SignatureType).set_Object_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_MessageHeaderType(
    mut MessageHeaderType: *mut din_MessageHeaderType,
) {
    (*MessageHeaderType).set_Notification_isUsed(0 as u32);
    (*MessageHeaderType).set_Signature_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_BodyType(mut BodyType: *mut din_BodyType) {
    (*BodyType).set_BodyElement_isUsed(0 as u32);
    (*BodyType).set_CableCheckReq_isUsed(0 as u32);
    (*BodyType).set_CableCheckRes_isUsed(0 as u32);
    (*BodyType).set_CertificateInstallationReq_isUsed(0 as u32);
    (*BodyType).set_CertificateInstallationRes_isUsed(0 as u32);
    (*BodyType).set_CertificateUpdateReq_isUsed(0 as u32);
    (*BodyType).set_CertificateUpdateRes_isUsed(0 as u32);
    (*BodyType).set_ChargeParameterDiscoveryReq_isUsed(0 as u32);
    (*BodyType).set_ChargeParameterDiscoveryRes_isUsed(0 as u32);
    (*BodyType).set_ChargingStatusReq_isUsed(0 as u32);
    (*BodyType).set_ChargingStatusRes_isUsed(0 as u32);
    (*BodyType).set_ContractAuthenticationReq_isUsed(0 as u32);
    (*BodyType).set_ContractAuthenticationRes_isUsed(0 as u32);
    (*BodyType).set_CurrentDemandReq_isUsed(0 as u32);
    (*BodyType).set_CurrentDemandRes_isUsed(0 as u32);
    (*BodyType).set_MeteringReceiptReq_isUsed(0 as u32);
    (*BodyType).set_MeteringReceiptRes_isUsed(0 as u32);
    (*BodyType).set_PaymentDetailsReq_isUsed(0 as u32);
    (*BodyType).set_PaymentDetailsRes_isUsed(0 as u32);
    (*BodyType).set_PowerDeliveryReq_isUsed(0 as u32);
    (*BodyType).set_PowerDeliveryRes_isUsed(0 as u32);
    (*BodyType).set_PreChargeReq_isUsed(0 as u32);
    (*BodyType).set_PreChargeRes_isUsed(0 as u32);
    (*BodyType).set_ServiceDetailReq_isUsed(0 as u32);
    (*BodyType).set_ServiceDetailRes_isUsed(0 as u32);
    (*BodyType).set_ServiceDiscoveryReq_isUsed(0 as u32);
    (*BodyType).set_ServiceDiscoveryRes_isUsed(0 as u32);
    (*BodyType).set_ServicePaymentSelectionReq_isUsed(0 as u32);
    (*BodyType).set_ServicePaymentSelectionRes_isUsed(0 as u32);
    (*BodyType).set_SessionSetupReq_isUsed(0 as u32);
    (*BodyType).set_SessionSetupRes_isUsed(0 as u32);
    (*BodyType).set_SessionStopReq_isUsed(0 as u32);
    (*BodyType).set_SessionStopRes_isUsed(0 as u32);
    (*BodyType).set_WeldingDetectionReq_isUsed(0 as u32);
    (*BodyType).set_WeldingDetectionRes_isUsed(0 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn init_din_V2G_Message(mut V2G_Message: *mut din_V2G_Message) {}
