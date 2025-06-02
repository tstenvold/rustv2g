use ::c2rust_bitfields;
use c2rust_bitfields::*;

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
pub type ExiChar = u8;
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

pub struct din_RelativeTimeIntervalType {
    pub start: u32,
    pub duration: u32,
    #[bitfield(name = "duration_isUsed", ty = "u32", bits = "0..=0")]
    pub duration_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]

pub struct din_IntervalType {
    pub _unused: i32,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct din_ConsumptionCostType {
    pub startValue: u32,
    pub Cost: din_CostType,
    #[bitfield(name = "Cost_isUsed", ty = "u32", bits = "0..=0")]
    pub Cost_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone, BitfieldStruct)]

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
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct din_TransformsType {
    pub Transform: din_TransformType,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct din_X509IssuerSerialType {
    pub X509IssuerName: C2RustUnnamed_9,
    pub X509SerialNumber: ExiSigned,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_9 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct din_DigestMethodType {
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

pub struct din_RSAKeyValueType {
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
#[derive(Copy, Clone)]

pub struct din_PMaxScheduleType {
    pub PMaxScheduleID: i16,
    pub PMaxScheduleEntry: C2RustUnnamed_14,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_14 {
    pub array: [din_PMaxScheduleEntryType; 5],
    pub arrayLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct C2RustUnnamed_15 {
    pub array: [din_SalesTariffEntryType; 5],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_16 {
    pub characters: [i8; 33],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_17 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct din_CanonicalizationMethodType {
    pub Algorithm: C2RustUnnamed_19,
    pub ANY: C2RustUnnamed_18,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_18 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_19 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct C2RustUnnamed_20 {
    pub characters: [i8; 33],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_21 {
    pub characters: [i8; 33],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct din_ServiceType {
    pub ServiceTag: din_ServiceTagType,
    pub FreeService: i32,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct din_SelectedServiceType {
    pub ServiceID: u16,
    pub ParameterSetID: i16,
    #[bitfield(name = "ParameterSetID_isUsed", ty = "u32", bits = "0..=0")]
    pub ParameterSetID_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct din_AC_EVSEStatusType {
    pub PowerSwitchClosed: i32,
    pub RCD: i32,
    pub NotificationMaxDelay: u32,
    pub EVSENotification: din_EVSENotificationType,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct C2RustUnnamed_22 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_23 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct C2RustUnnamed_24 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct din_SubCertificatesType {
    pub Certificate: C2RustUnnamed_25,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_25 {
    pub bytes: [u8; 1200],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct din_ProfileEntryType {
    pub ChargingProfileEntryStart: u32,
    pub ChargingProfileEntryMaxPower: i16,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct C2RustUnnamed_26 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_27 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_28 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_29 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct C2RustUnnamed_30 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_31 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct C2RustUnnamed_32 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_33 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_34 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_35 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_36 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct din_PGPDataType {
    pub c2rust_unnamed: C2RustUnnamed_37,
}
#[derive(Copy, Clone)]

pub union C2RustUnnamed_37 {
    pub choice_1: C2RustUnnamed_41,
    pub choice_1_isUsed: u32,
    pub choice_2: C2RustUnnamed_38,
    pub choice_2_isUsed: u32,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct C2RustUnnamed_38 {
    pub PGPKeyPacket: C2RustUnnamed_40,
    pub ANY: C2RustUnnamed_39,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_39 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_40 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct C2RustUnnamed_42 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_43 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_44 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct din_SPKIDataType {
    pub SPKISexp: C2RustUnnamed_46,
    pub ANY: C2RustUnnamed_45,
    #[bitfield(name = "ANY_isUsed", ty = "u32", bits = "0..=0")]
    pub ANY_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_45 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_46 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct C2RustUnnamed_47 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct din_SignatureValueType {
    pub Id: C2RustUnnamed_49,
    #[bitfield(name = "Id_isUsed", ty = "u32", bits = "0..=0")]
    pub Id_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub CONTENT: C2RustUnnamed_48,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_48 {
    pub bytes: [u8; 350],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_49 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct din_CertificateChainType {
    pub Certificate: C2RustUnnamed_50,
    pub SubCertificates: din_SubCertificatesType,
    #[bitfield(name = "SubCertificates_isUsed", ty = "u32", bits = "0..=0")]
    pub SubCertificates_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_50 {
    pub bytes: [u8; 1200],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct C2RustUnnamed_51 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_52 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct din_ParameterSetType {
    pub ParameterSetID: i16,
    pub Parameter: din_ParameterType,
}
#[derive(Copy, Clone)]

pub struct din_ListOfRootCertificateIDsType {
    pub RootCertificateID: C2RustUnnamed_53,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_53 {
    pub array: [C2RustUnnamed_54; 5],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_54 {
    pub characters: [i8; 41],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct din_PaymentOptionsType {
    pub PaymentOption: C2RustUnnamed_55,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_55 {
    pub array: [din_paymentOptionType; 2],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct din_SelectedServiceListType {
    pub SelectedService: C2RustUnnamed_56,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_56 {
    pub array: [din_SelectedServiceType; 16],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct din_AC_EVChargeParameterType {
    pub DepartureTime: u32,
    pub EAmount: din_PhysicalValueType,
    pub EVMaxVoltage: din_PhysicalValueType,
    pub EVMaxCurrent: din_PhysicalValueType,
    pub EVMinCurrent: din_PhysicalValueType,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct din_EVChargeParameterType {
    pub _unused: i32,
}
#[derive(Copy, Clone)]

pub struct din_ChargingProfileType {
    pub SAScheduleTupleID: i16,
    pub ProfileEntry: C2RustUnnamed_57,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_57 {
    pub array: [din_ProfileEntryType; 24],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct din_EVSEStatusType {
    pub _unused: i32,
}
#[derive(Copy, Clone, BitfieldStruct)]

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
#[derive(Copy, Clone)]

pub struct din_ServiceChargeType {
    pub ServiceTag: din_ServiceTagType,
    pub FreeService: i32,
    pub EnergyTransferType: din_EVSESupportedEnergyTransferType,
}
#[derive(Copy, Clone)]

pub struct din_ServiceParameterListType {
    pub ParameterSet: C2RustUnnamed_62,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_62 {
    pub array: [din_ParameterSetType; 5],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct din_SAScheduleListType {
    pub SAScheduleTuple: C2RustUnnamed_63,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_63 {
    pub array: [din_SAScheduleTupleType; 5],
    pub arrayLen: u16,
}
#[derive(Copy, Clone)]

pub struct din_SASchedulesType {
    pub _unused: i32,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct din_EVPowerDeliveryParameterType {
    pub _unused: i32,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct C2RustUnnamed_64 {
    pub bytes: [u8; 4],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_65 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_66 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_67 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct din_ServiceTagListType {
    pub Service: din_ServiceType,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct din_AC_EVSEChargeParameterType {
    pub AC_EVSEStatus: din_AC_EVSEStatusType,
    pub EVSEMaxVoltage: din_PhysicalValueType,
    pub EVSEMaxCurrent: din_PhysicalValueType,
    pub EVSEMinCurrent: din_PhysicalValueType,
}
#[derive(Copy, Clone)]

pub struct din_EVSEChargeParameterType {
    pub _unused: i32,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct C2RustUnnamed_68 {
    pub bytes: [u8; 32],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_69 {
    pub characters: [i8; 33],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct din_CertificateInstallationResType {
    pub Id: C2RustUnnamed_73,
    pub ResponseCode: din_responseCodeType,
    pub ContractSignatureCertChain: din_CertificateChainType,
    pub ContractSignatureEncryptedPrivateKey: C2RustUnnamed_72,
    pub DHParams: C2RustUnnamed_71,
    pub ContractID: C2RustUnnamed_70,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_70 {
    pub characters: [i8; 25],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_71 {
    pub bytes: [u8; 256],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_72 {
    pub bytes: [u8; 128],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_73 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct din_CableCheckReqType {
    pub DC_EVStatus: din_DC_EVStatusType,
}
#[derive(Copy, Clone)]

pub struct din_CableCheckResType {
    pub ResponseCode: din_responseCodeType,
    pub DC_EVSEStatus: din_DC_EVSEStatusType,
    pub EVSEProcessing: din_EVSEProcessingType,
}
#[derive(Copy, Clone)]

pub struct din_PreChargeReqType {
    pub DC_EVStatus: din_DC_EVStatusType,
    pub EVTargetVoltage: din_PhysicalValueType,
    pub EVTargetCurrent: din_PhysicalValueType,
}
#[derive(Copy, Clone)]

pub struct din_PreChargeResType {
    pub ResponseCode: din_responseCodeType,
    pub DC_EVSEStatus: din_DC_EVSEStatusType,
    pub EVSEPresentVoltage: din_PhysicalValueType,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct din_WeldingDetectionReqType {
    pub DC_EVStatus: din_DC_EVStatusType,
}
#[derive(Copy, Clone)]

pub struct din_WeldingDetectionResType {
    pub ResponseCode: din_responseCodeType,
    pub DC_EVSEStatus: din_DC_EVSEStatusType,
    pub EVSEPresentVoltage: din_PhysicalValueType,
}
#[derive(Copy, Clone)]

pub struct din_SessionSetupReqType {
    pub EVCCID: C2RustUnnamed_74,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_74 {
    pub bytes: [u8; 8],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct C2RustUnnamed_75 {
    pub bytes: [u8; 256],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_76 {
    pub bytes: [u8; 1200],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_77 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct C2RustUnnamed_78 {
    pub bytes: [u8; 32],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct C2RustUnnamed_79 {
    pub characters: [i8; 33],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct din_ServiceDetailReqType {
    pub ServiceID: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct din_ServicePaymentSelectionReqType {
    pub SelectedPaymentOption: din_paymentOptionType,
    pub SelectedServiceList: din_SelectedServiceListType,
}
#[derive(Copy, Clone)]

pub struct din_ServicePaymentSelectionResType {
    pub ResponseCode: din_responseCodeType,
}
#[derive(Copy, Clone)]

pub struct din_PaymentDetailsReqType {
    pub ContractID: C2RustUnnamed_80,
    pub ContractSignatureCertChain: din_CertificateChainType,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_80 {
    pub characters: [i8; 25],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct din_PaymentDetailsResType {
    pub ResponseCode: din_responseCodeType,
    pub GenChallenge: C2RustUnnamed_81,
    pub DateTimeNow: i64,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_81 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct C2RustUnnamed_82 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_83 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct din_ContractAuthenticationResType {
    pub ResponseCode: din_responseCodeType,
    pub EVSEProcessing: din_EVSEProcessingType,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct din_ChargingStatusReqType {
    pub _unused: i32,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct C2RustUnnamed_84 {
    pub bytes: [u8; 32],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct C2RustUnnamed_85 {
    pub bytes: [u8; 8],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_86 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct din_MeteringReceiptResType {
    pub ResponseCode: din_responseCodeType,
    pub AC_EVSEStatus: din_AC_EVSEStatusType,
}
#[derive(Copy, Clone)]

pub struct din_SessionStopType {
    pub _unused: i32,
}
#[derive(Copy, Clone)]

pub struct din_SessionStopResType {
    pub ResponseCode: din_responseCodeType,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct C2RustUnnamed_87 {
    pub bytes: [u8; 256],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_88 {
    pub characters: [i8; 25],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_89 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

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

pub struct C2RustUnnamed_90 {
    pub characters: [i8; 25],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_91 {
    pub bytes: [u8; 256],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_92 {
    pub bytes: [u8; 128],
    pub bytesLen: u16,
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_93 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone)]

pub struct din_BodyBaseType {
    pub _unused: i32,
}
#[derive(Copy, Clone, BitfieldStruct)]

pub struct din_NotificationType {
    pub FaultCode: din_faultCodeType,
    pub FaultMsg: C2RustUnnamed_94,
    #[bitfield(name = "FaultMsg_isUsed", ty = "u32", bits = "0..=0")]
    pub FaultMsg_isUsed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]

pub struct C2RustUnnamed_94 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct C2RustUnnamed_95 {
    pub characters: [i8; 65],
    pub charactersLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct C2RustUnnamed_96 {
    pub bytes: [u8; 8],
    pub bytesLen: u16,
}
#[derive(Copy, Clone, BitfieldStruct)]

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

pub struct din_V2G_Message {
    pub Header: din_MessageHeaderType,
    pub Body: din_BodyType,
}
#[derive(Copy, Clone)]

pub struct din_exiDocument {
    pub V2G_Message: din_V2G_Message,
}
unsafe extern "C" fn decode_din_CostType(
    stream: &mut ExiBitstream,
    mut CostType: *mut din_CostType,
) -> i32 {
    let mut grammar_id: i32 = 0 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_CostType(CostType);
    while done == 0 {
        match grammar_id {
            0 => {
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
                                        (*CostType).costKind = value as din_costKindType;
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
                                        grammar_id = 1 as i32;
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
            1 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(stream, &mut (*CostType).amount);
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
                                    let mut value_0: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        3 as i32 as usize,
                                        &mut value_0,
                                    );
                                    if error == 0 as i32 {
                                        (*CostType).amountMultiplier =
                                            value_0.wrapping_add(-(3 as i32) as u32) as int8_t;
                                        (*CostType).set_amountMultiplier_isUsed(1 as u32);
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
                                        grammar_id = 3 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_RelativeTimeIntervalType(
    stream: &mut ExiBitstream,
    mut RelativeTimeIntervalType: *mut din_RelativeTimeIntervalType,
) -> i32 {
    let mut grammar_id: i32 = 5 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_RelativeTimeIntervalType(RelativeTimeIntervalType);
    while done == 0 {
        match grammar_id {
            5 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*RelativeTimeIntervalType).start,
                            );
                            if error == 0 as i32 {
                                grammar_id = 6 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            6 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*RelativeTimeIntervalType).duration,
                            );
                            if error == 0 as i32 {
                                (*RelativeTimeIntervalType).set_duration_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_IntervalType(
    stream: &mut ExiBitstream,
    mut IntervalType: *mut din_IntervalType,
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
unsafe extern "C" fn decode_din_ConsumptionCostType(
    stream: &mut ExiBitstream,
    mut ConsumptionCostType: *mut din_ConsumptionCostType,
) -> i32 {
    let mut grammar_id: i32 = 7 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_ConsumptionCostType(ConsumptionCostType);
    while done == 0 {
        match grammar_id {
            7 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*ConsumptionCostType).startValue,
                            );
                            if error == 0 as i32 {
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
                            error = decode_din_CostType(stream, &mut (*ConsumptionCostType).Cost);
                            if error == 0 as i32 {
                                (*ConsumptionCostType).set_Cost_isUsed(1 as u32);
                                grammar_id = 9 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            9 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = -(110 as i32);
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_TransformType(
    stream: &mut ExiBitstream,
    mut TransformType: *mut din_TransformType,
) -> i32 {
    let mut grammar_id: i32 = 10 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_TransformType(TransformType);
    while done == 0 {
        match grammar_id {
            10 => {
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
                            grammar_id = 11 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            11 => {
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
                                        grammar_id = 3 as i32;
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
                            grammar_id = 4 as i32;
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
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_PMaxScheduleEntryType(
    stream: &mut ExiBitstream,
    mut PMaxScheduleEntryType: *mut din_PMaxScheduleEntryType,
) -> i32 {
    let mut grammar_id: i32 = 12 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_PMaxScheduleEntryType(PMaxScheduleEntryType);
    while done == 0 {
        match grammar_id {
            12 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_RelativeTimeIntervalType(
                                stream,
                                &mut (*PMaxScheduleEntryType).RelativeTimeInterval,
                            );
                            if error == 0 as i32 {
                                (*PMaxScheduleEntryType).set_RelativeTimeInterval_isUsed(1 as u32);
                                grammar_id = 13 as i32;
                            }
                        }
                        1 => {
                            error = decode_din_IntervalType(
                                stream,
                                &mut (*PMaxScheduleEntryType).TimeInterval,
                            );
                            if error == 0 as i32 {
                                (*PMaxScheduleEntryType).set_TimeInterval_isUsed(1 as u32);
                                grammar_id = 13 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            13 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*PMaxScheduleEntryType).PMax,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_SalesTariffEntryType(
    stream: &mut ExiBitstream,
    mut SalesTariffEntryType: *mut din_SalesTariffEntryType,
) -> i32 {
    let mut grammar_id: i32 = 14 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_SalesTariffEntryType(SalesTariffEntryType);
    while done == 0 {
        match grammar_id {
            14 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_RelativeTimeIntervalType(
                                stream,
                                &mut (*SalesTariffEntryType).RelativeTimeInterval,
                            );
                            if error == 0 as i32 {
                                (*SalesTariffEntryType).set_RelativeTimeInterval_isUsed(1 as u32);
                                grammar_id = 15 as i32;
                            }
                        }
                        1 => {
                            error = decode_din_IntervalType(
                                stream,
                                &mut (*SalesTariffEntryType).TimeInterval,
                            );
                            if error == 0 as i32 {
                                (*SalesTariffEntryType).set_TimeInterval_isUsed(1 as u32);
                                grammar_id = 15 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            15 => {
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
                                        (*SalesTariffEntryType).EPriceLevel = value as u8;
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
                                        grammar_id = 16 as i32;
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
            16 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_ConsumptionCostType(
                                stream,
                                &mut (*SalesTariffEntryType).ConsumptionCost,
                            );
                            if error == 0 as i32 {
                                (*SalesTariffEntryType).set_ConsumptionCost_isUsed(1 as u32);
                                grammar_id = 17 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            17 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = -(110 as i32);
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_TransformsType(
    stream: &mut ExiBitstream,
    mut TransformsType: *mut din_TransformsType,
) -> i32 {
    let mut grammar_id: i32 = 18 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_TransformsType(TransformsType);
    while done == 0 {
        match grammar_id {
            18 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error =
                                decode_din_TransformType(stream, &mut (*TransformsType).Transform);
                            if error == 0 as i32 {
                                grammar_id = 19 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            19 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = -(110 as i32);
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_DSAKeyValueType(
    stream: &mut ExiBitstream,
    mut DSAKeyValueType: *mut din_DSAKeyValueType,
) -> i32 {
    let mut grammar_id: i32 = 20 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_DSAKeyValueType(DSAKeyValueType);
    while done == 0 {
        match grammar_id {
            20 => {
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
                                grammar_id = 21 as i32;
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
                                grammar_id = 23 as i32;
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
                                grammar_id = 24 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            21 => {
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
                                grammar_id = 23 as i32;
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
                                grammar_id = 24 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            23 => {
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
                                grammar_id = 24 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            24 => {
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
                                grammar_id = 25 as i32;
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
                                grammar_id = 26 as i32;
                            }
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
                                grammar_id = 26 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            26 => {
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
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_X509IssuerSerialType(
    stream: &mut ExiBitstream,
    mut X509IssuerSerialType: *mut din_X509IssuerSerialType,
) -> i32 {
    let mut grammar_id: i32 = 27 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_X509IssuerSerialType(X509IssuerSerialType);
    while done == 0 {
        match grammar_id {
            27 => {
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
                                        grammar_id = 28 as i32;
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
            28 => {
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
                                    grammar_id = 3 as i32;
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
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_DigestMethodType(
    stream: &mut ExiBitstream,
    mut DigestMethodType: *mut din_DigestMethodType,
) -> i32 {
    let mut grammar_id: i32 = 29 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_DigestMethodType(DigestMethodType);
    while done == 0 {
        match grammar_id {
            29 => {
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
                            grammar_id = 30 as i32;
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
                            error = -(50 as i32);
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_RSAKeyValueType(
    stream: &mut ExiBitstream,
    mut RSAKeyValueType: *mut din_RSAKeyValueType,
) -> i32 {
    let mut grammar_id: i32 = 31 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_RSAKeyValueType(RSAKeyValueType);
    while done == 0 {
        match grammar_id {
            31 => {
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
                                &mut (*RSAKeyValueType).Exponent.bytesLen,
                                &mut *((*RSAKeyValueType).Exponent.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_PMaxScheduleType(
    stream: &mut ExiBitstream,
    mut PMaxScheduleType: *mut din_PMaxScheduleType,
) -> i32 {
    let mut grammar_id: i32 = 33 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_PMaxScheduleType(PMaxScheduleType);
    while done == 0 {
        match grammar_id {
            33 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*PMaxScheduleType).PMaxScheduleID,
                            );
                            if error == 0 as i32 {
                                grammar_id = 34 as i32;
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
                            if ((*PMaxScheduleType).PMaxScheduleEntry.arrayLen as i32) < 5 as i32 {
                                let fresh0 = (*PMaxScheduleType).PMaxScheduleEntry.arrayLen;
                                (*PMaxScheduleType).PMaxScheduleEntry.arrayLen =
                                    ((*PMaxScheduleType).PMaxScheduleEntry.arrayLen)
                                        .wrapping_add(1);
                                error = decode_din_PMaxScheduleEntryType(
                                    stream,
                                    &mut *((*PMaxScheduleType).PMaxScheduleEntry.array)
                                        .as_mut_ptr()
                                        .offset(fresh0 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 35 as i32;
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
                            if ((*PMaxScheduleType).PMaxScheduleEntry.arrayLen as i32) < 5 as i32 {
                                let fresh1 = (*PMaxScheduleType).PMaxScheduleEntry.arrayLen;
                                (*PMaxScheduleType).PMaxScheduleEntry.arrayLen =
                                    ((*PMaxScheduleType).PMaxScheduleEntry.arrayLen)
                                        .wrapping_add(1);
                                error = decode_din_PMaxScheduleEntryType(
                                    stream,
                                    &mut *((*PMaxScheduleType).PMaxScheduleEntry.array)
                                        .as_mut_ptr()
                                        .offset(fresh1 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 35 as i32;
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_SalesTariffType(
    stream: &mut ExiBitstream,
    mut SalesTariffType: *mut din_SalesTariffType,
) -> i32 {
    let mut grammar_id: i32 = 36 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_SalesTariffType(SalesTariffType);
    while done == 0 {
        match grammar_id {
            36 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*SalesTariffType).Id.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*SalesTariffType).Id.charactersLen as i32 >= 2 as i32 {
                                    (*SalesTariffType).Id.charactersLen =
                                        ((*SalesTariffType).Id.charactersLen as i32 - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*SalesTariffType).Id.charactersLen as usize,
                                        ((*SalesTariffType).Id.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            grammar_id = 37 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            37 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*SalesTariffType).SalesTariffID,
                            );
                            if error == 0 as i32 {
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
                                        &mut (*SalesTariffType)
                                            .SalesTariffDescription
                                            .charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*SalesTariffType).SalesTariffDescription.charactersLen
                                            as i32
                                            >= 2 as i32
                                        {
                                            (*SalesTariffType)
                                                .SalesTariffDescription
                                                .charactersLen = ((*SalesTariffType)
                                                .SalesTariffDescription
                                                .charactersLen
                                                as i32
                                                - 2 as i32)
                                                as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*SalesTariffType)
                                                    .SalesTariffDescription
                                                    .charactersLen
                                                    as usize,
                                                ((*SalesTariffType)
                                                    .SalesTariffDescription
                                                    .characters)
                                                    .as_mut_ptr(),
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
                                        (*SalesTariffType)
                                            .set_SalesTariffDescription_isUsed(1 as u32);
                                        grammar_id = 39 as i32;
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
                                        (*SalesTariffType).NumEPriceLevels = value as u8;
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
                                        grammar_id = 40 as i32;
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
            39 => {
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
                                        (*SalesTariffType).NumEPriceLevels = value_0 as u8;
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
                                        grammar_id = 40 as i32;
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
            40 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SalesTariffType).SalesTariffEntry.arrayLen as i32) < 5 as i32 {
                                let fresh2 = (*SalesTariffType).SalesTariffEntry.arrayLen;
                                (*SalesTariffType).SalesTariffEntry.arrayLen =
                                    ((*SalesTariffType).SalesTariffEntry.arrayLen).wrapping_add(1);
                                error = decode_din_SalesTariffEntryType(
                                    stream,
                                    &mut *((*SalesTariffType).SalesTariffEntry.array)
                                        .as_mut_ptr()
                                        .offset(fresh2 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 41 as i32;
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
                            if ((*SalesTariffType).SalesTariffEntry.arrayLen as i32) < 5 as i32 {
                                let fresh3 = (*SalesTariffType).SalesTariffEntry.arrayLen;
                                (*SalesTariffType).SalesTariffEntry.arrayLen =
                                    ((*SalesTariffType).SalesTariffEntry.arrayLen).wrapping_add(1);
                                error = decode_din_SalesTariffEntryType(
                                    stream,
                                    &mut *((*SalesTariffType).SalesTariffEntry.array)
                                        .as_mut_ptr()
                                        .offset(fresh3 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 41 as i32;
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_CanonicalizationMethodType(
    stream: &mut ExiBitstream,
    mut CanonicalizationMethodType: *mut din_CanonicalizationMethodType,
) -> i32 {
    let mut grammar_id: i32 = 42 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_CanonicalizationMethodType(CanonicalizationMethodType);
    while done == 0 {
        match grammar_id {
            42 => {
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
                            grammar_id = 43 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            43 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = -(50 as i32);
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_ServiceTagType(
    stream: &mut ExiBitstream,
    mut ServiceTagType: *mut din_ServiceTagType,
) -> i32 {
    let mut grammar_id: i32 = 44 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_ServiceTagType(ServiceTagType);
    while done == 0 {
        match grammar_id {
            44 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error =
                                decode_exi_type_uint16(stream, &mut (*ServiceTagType).ServiceID);
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
                                        &mut (*ServiceTagType).ServiceName.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*ServiceTagType).ServiceName.charactersLen as i32
                                            >= 2 as i32
                                        {
                                            (*ServiceTagType).ServiceName.charactersLen =
                                                ((*ServiceTagType).ServiceName.charactersLen as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*ServiceTagType).ServiceName.charactersLen
                                                    as usize,
                                                ((*ServiceTagType).ServiceName.characters)
                                                    .as_mut_ptr(),
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
                                        (*ServiceTagType).set_ServiceName_isUsed(1 as u32);
                                        grammar_id = 46 as i32;
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
                                        2 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*ServiceTagType).ServiceCategory =
                                            value as din_serviceCategoryType;
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
                                        grammar_id = 47 as i32;
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
            46 => {
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
                                        (*ServiceTagType).ServiceCategory =
                                            value_0 as din_serviceCategoryType;
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
                                        grammar_id = 47 as i32;
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
            47 => {
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
                                        &mut (*ServiceTagType).ServiceScope.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*ServiceTagType).ServiceScope.charactersLen as i32
                                            >= 2 as i32
                                        {
                                            (*ServiceTagType).ServiceScope.charactersLen =
                                                ((*ServiceTagType).ServiceScope.charactersLen
                                                    as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*ServiceTagType).ServiceScope.charactersLen
                                                    as usize,
                                                ((*ServiceTagType).ServiceScope.characters)
                                                    .as_mut_ptr(),
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
                                        (*ServiceTagType).set_ServiceScope_isUsed(1 as u32);
                                        grammar_id = 3 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_ServiceType(
    stream: &mut ExiBitstream,
    mut ServiceType: *mut din_ServiceType,
) -> i32 {
    let mut grammar_id: i32 = 48 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_ServiceType(ServiceType);
    while done == 0 {
        match grammar_id {
            48 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error =
                                decode_din_ServiceTagType(stream, &mut (*ServiceType).ServiceTag);
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
                                        grammar_id = 3 as i32;
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
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_SelectedServiceType(
    stream: &mut ExiBitstream,
    mut SelectedServiceType: *mut din_SelectedServiceType,
) -> i32 {
    let mut grammar_id: i32 = 50 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_SelectedServiceType(SelectedServiceType);
    while done == 0 {
        match grammar_id {
            50 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*SelectedServiceType).ServiceID,
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
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*SelectedServiceType).ParameterSetID,
                            );
                            if error == 0 as i32 {
                                (*SelectedServiceType).set_ParameterSetID_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_SAScheduleTupleType(
    stream: &mut ExiBitstream,
    mut SAScheduleTupleType: *mut din_SAScheduleTupleType,
) -> i32 {
    let mut grammar_id: i32 = 52 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_SAScheduleTupleType(SAScheduleTupleType);
    while done == 0 {
        match grammar_id {
            52 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*SAScheduleTupleType).SAScheduleTupleID,
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
                            error = decode_din_PMaxScheduleType(
                                stream,
                                &mut (*SAScheduleTupleType).PMaxSchedule,
                            );
                            if error == 0 as i32 {
                                grammar_id = 54 as i32;
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
                            error = decode_din_SalesTariffType(
                                stream,
                                &mut (*SAScheduleTupleType).SalesTariff,
                            );
                            if error == 0 as i32 {
                                (*SAScheduleTupleType).set_SalesTariff_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_AC_EVSEStatusType(
    stream: &mut ExiBitstream,
    mut AC_EVSEStatusType: *mut din_AC_EVSEStatusType,
) -> i32 {
    let mut grammar_id: i32 = 55 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_AC_EVSEStatusType(AC_EVSEStatusType);
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
                                        1 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*AC_EVSEStatusType).PowerSwitchClosed = value as i32;
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
                                        (*AC_EVSEStatusType).RCD = value_0 as i32;
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
                                        grammar_id = 57 as i32;
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
            57 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*AC_EVSEStatusType).NotificationMaxDelay,
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
                                        (*AC_EVSEStatusType).EVSENotification =
                                            value_1 as din_EVSENotificationType;
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
                                        grammar_id = 3 as i32;
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
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_SignatureMethodType(
    stream: &mut ExiBitstream,
    mut SignatureMethodType: *mut din_SignatureMethodType,
) -> i32 {
    let mut grammar_id: i32 = 59 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_SignatureMethodType(SignatureMethodType);
    while done == 0 {
        match grammar_id {
            59 => {
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
                            grammar_id = 60 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            60 => {
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
                                    grammar_id = 61 as i32;
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
                            grammar_id = 4 as i32;
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
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            61 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = -(50 as i32);
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_KeyValueType(
    stream: &mut ExiBitstream,
    mut KeyValueType: *mut din_KeyValueType,
) -> i32 {
    let mut grammar_id: i32 = 62 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_KeyValueType(KeyValueType);
    while done == 0 {
        match grammar_id {
            62 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_DSAKeyValueType(
                                stream,
                                &mut (*KeyValueType).DSAKeyValue,
                            );
                            if error == 0 as i32 {
                                (*KeyValueType).set_DSAKeyValue_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            error = decode_din_RSAKeyValueType(
                                stream,
                                &mut (*KeyValueType).RSAKeyValue,
                            );
                            if error == 0 as i32 {
                                (*KeyValueType).set_RSAKeyValue_isUsed(1 as u32);
                                grammar_id = 3 as i32;
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
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_SubCertificatesType(
    stream: &mut ExiBitstream,
    mut SubCertificatesType: *mut din_SubCertificatesType,
) -> i32 {
    let mut grammar_id: i32 = 63 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_SubCertificatesType(SubCertificatesType);
    while done == 0 {
        match grammar_id {
            63 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*SubCertificatesType).Certificate.bytesLen,
                                &mut *((*SubCertificatesType).Certificate.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                1200 as i32 as usize,
                            );
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
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*SubCertificatesType).Certificate.bytesLen,
                                &mut *((*SubCertificatesType).Certificate.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                1200 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_ProfileEntryType(
    stream: &mut ExiBitstream,
    mut ProfileEntryType: *mut din_ProfileEntryType,
) -> i32 {
    let mut grammar_id: i32 = 65 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_ProfileEntryType(ProfileEntryType);
    while done == 0 {
        match grammar_id {
            65 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*ProfileEntryType).ChargingProfileEntryStart,
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
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*ProfileEntryType).ChargingProfileEntryMaxPower,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_ReferenceType(
    stream: &mut ExiBitstream,
    mut ReferenceType: *mut din_ReferenceType,
) -> i32 {
    let mut grammar_id: i32 = 67 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_ReferenceType(ReferenceType);
    while done == 0 {
        match grammar_id {
            67 => {
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
                            grammar_id = 68 as i32;
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
                            grammar_id = 69 as i32;
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
                            grammar_id = 70 as i32;
                        }
                        3 => {
                            error =
                                decode_din_TransformsType(stream, &mut (*ReferenceType).Transforms);
                            if error == 0 as i32 {
                                (*ReferenceType).set_Transforms_isUsed(1 as u32);
                                grammar_id = 71 as i32;
                            }
                        }
                        4 => {
                            error = decode_din_DigestMethodType(
                                stream,
                                &mut (*ReferenceType).DigestMethod,
                            );
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
            68 => {
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
                            grammar_id = 69 as i32;
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
                            grammar_id = 70 as i32;
                        }
                        2 => {
                            error =
                                decode_din_TransformsType(stream, &mut (*ReferenceType).Transforms);
                            if error == 0 as i32 {
                                (*ReferenceType).set_Transforms_isUsed(1 as u32);
                                grammar_id = 71 as i32;
                            }
                        }
                        3 => {
                            error = decode_din_DigestMethodType(
                                stream,
                                &mut (*ReferenceType).DigestMethod,
                            );
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
            69 => {
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
                            grammar_id = 70 as i32;
                        }
                        1 => {
                            error =
                                decode_din_TransformsType(stream, &mut (*ReferenceType).Transforms);
                            if error == 0 as i32 {
                                (*ReferenceType).set_Transforms_isUsed(1 as u32);
                                grammar_id = 71 as i32;
                            }
                        }
                        2 => {
                            error = decode_din_DigestMethodType(
                                stream,
                                &mut (*ReferenceType).DigestMethod,
                            );
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
            70 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error =
                                decode_din_TransformsType(stream, &mut (*ReferenceType).Transforms);
                            if error == 0 as i32 {
                                (*ReferenceType).set_Transforms_isUsed(1 as u32);
                                grammar_id = 71 as i32;
                            }
                        }
                        1 => {
                            error = decode_din_DigestMethodType(
                                stream,
                                &mut (*ReferenceType).DigestMethod,
                            );
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
            71 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_DigestMethodType(
                                stream,
                                &mut (*ReferenceType).DigestMethod,
                            );
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
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*ReferenceType).DigestValue.bytesLen,
                                &mut *((*ReferenceType).DigestValue.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_RetrievalMethodType(
    stream: &mut ExiBitstream,
    mut RetrievalMethodType: *mut din_RetrievalMethodType,
) -> i32 {
    let mut grammar_id: i32 = 73 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_RetrievalMethodType(RetrievalMethodType);
    while done == 0 {
        match grammar_id {
            73 => {
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
                            grammar_id = 74 as i32;
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
                            grammar_id = 75 as i32;
                        }
                        2 => {
                            error = decode_din_TransformsType(
                                stream,
                                &mut (*RetrievalMethodType).Transforms,
                            );
                            if error == 0 as i32 {
                                (*RetrievalMethodType).set_Transforms_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        3 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            74 => {
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
                            grammar_id = 75 as i32;
                        }
                        1 => {
                            error = decode_din_TransformsType(
                                stream,
                                &mut (*RetrievalMethodType).Transforms,
                            );
                            if error == 0 as i32 {
                                (*RetrievalMethodType).set_Transforms_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            75 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_TransformsType(
                                stream,
                                &mut (*RetrievalMethodType).Transforms,
                            );
                            if error == 0 as i32 {
                                (*RetrievalMethodType).set_Transforms_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_X509DataType(
    stream: &mut ExiBitstream,
    mut X509DataType: *mut din_X509DataType,
) -> i32 {
    let mut grammar_id: i32 = 76 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_X509DataType(X509DataType);
    while done == 0 {
        match grammar_id {
            76 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_X509IssuerSerialType(
                                stream,
                                &mut (*X509DataType).X509IssuerSerial,
                            );
                            if error == 0 as i32 {
                                (*X509DataType).set_X509IssuerSerial_isUsed(1 as u32);
                                grammar_id = 3 as i32;
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
                                grammar_id = 3 as i32;
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
                                        grammar_id = 3 as i32;
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
                                grammar_id = 3 as i32;
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
                                grammar_id = 3 as i32;
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
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_PGPDataType(
    stream: &mut ExiBitstream,
    mut PGPDataType: *mut din_PGPDataType,
) -> i32 {
    let mut grammar_id: i32 = 77 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_PGPDataType(PGPDataType);
    while done == 0 {
        match grammar_id {
            77 => {
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
                                grammar_id = 78 as i32;
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
                                grammar_id = 79 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            78 => {
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
                                grammar_id = 79 as i32;
                            }
                        }
                        1 => {
                            error = -(50 as i32);
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
                                grammar_id = 80 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            79 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = -(50 as i32);
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
                            error = -(50 as i32);
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
                                grammar_id = 80 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_SPKIDataType(
    stream: &mut ExiBitstream,
    mut SPKIDataType: *mut din_SPKIDataType,
) -> i32 {
    let mut grammar_id: i32 = 82 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_SPKIDataType(SPKIDataType);
    while done == 0 {
        match grammar_id {
            82 => {
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
                                grammar_id = 83 as i32;
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
                            error = -(50 as i32);
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_SignedInfoType(
    stream: &mut ExiBitstream,
    mut SignedInfoType: *mut din_SignedInfoType,
) -> i32 {
    let mut grammar_id: i32 = 84 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_SignedInfoType(SignedInfoType);
    while done == 0 {
        match grammar_id {
            84 => {
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
                            grammar_id = 85 as i32;
                        }
                        1 => {
                            error = decode_din_CanonicalizationMethodType(
                                stream,
                                &mut (*SignedInfoType).CanonicalizationMethod,
                            );
                            if error == 0 as i32 {
                                grammar_id = 86 as i32;
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
                            error = decode_din_CanonicalizationMethodType(
                                stream,
                                &mut (*SignedInfoType).CanonicalizationMethod,
                            );
                            if error == 0 as i32 {
                                grammar_id = 86 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            86 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_SignatureMethodType(
                                stream,
                                &mut (*SignedInfoType).SignatureMethod,
                            );
                            if error == 0 as i32 {
                                grammar_id = 87 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            87 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error =
                                decode_din_ReferenceType(stream, &mut (*SignedInfoType).Reference);
                            if error == 0 as i32 {
                                grammar_id = 88 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            88 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = -(110 as i32);
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_DC_EVStatusType(
    stream: &mut ExiBitstream,
    mut DC_EVStatusType: *mut din_DC_EVStatusType,
) -> i32 {
    let mut grammar_id: i32 = 89 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_DC_EVStatusType(DC_EVStatusType);
    while done == 0 {
        match grammar_id {
            89 => {
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
                                        (*DC_EVStatusType).EVReady = value as i32;
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
                                        grammar_id = 90 as i32;
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
                                    let mut value_0: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_0,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_EVStatusType).EVCabinConditioning = value_0 as i32;
                                        (*DC_EVStatusType).set_EVCabinConditioning_isUsed(1 as u32);
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
                                        grammar_id = 91 as i32;
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
                                    let mut value_1: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_1,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_EVStatusType).EVRESSConditioning = value_1 as i32;
                                        (*DC_EVStatusType).set_EVRESSConditioning_isUsed(1 as u32);
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
                        2 => {
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
                                        4 as i32 as usize,
                                        &mut value_2,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_EVStatusType).EVErrorCode =
                                            value_2 as din_DC_EVErrorCodeType;
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
                                        grammar_id = 93 as i32;
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
            91 => {
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
                                        1 as i32 as usize,
                                        &mut value_3,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_EVStatusType).EVRESSConditioning = value_3 as i32;
                                        (*DC_EVStatusType).set_EVRESSConditioning_isUsed(1 as u32);
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
                        1 => {
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
                                        4 as i32 as usize,
                                        &mut value_4,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_EVStatusType).EVErrorCode =
                                            value_4 as din_DC_EVErrorCodeType;
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
                                        grammar_id = 93 as i32;
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
                                    let mut value_5: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        4 as i32 as usize,
                                        &mut value_5,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_EVStatusType).EVErrorCode =
                                            value_5 as din_DC_EVErrorCodeType;
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
                                        grammar_id = 93 as i32;
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
            93 => {
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
                                    let mut value_6: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_6,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_EVStatusType).EVRESSSOC = value_6 as int8_t;
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
                                        grammar_id = 3 as i32;
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
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_SignatureValueType(
    stream: &mut ExiBitstream,
    mut SignatureValueType: *mut din_SignatureValueType,
) -> i32 {
    let mut grammar_id: i32 = 94 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_SignatureValueType(SignatureValueType);
    while done == 0 {
        match grammar_id {
            94 => {
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
                            grammar_id = 95 as i32;
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
                                    grammar_id = 3 as i32;
                                }
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
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_CertificateChainType(
    stream: &mut ExiBitstream,
    mut CertificateChainType: *mut din_CertificateChainType,
) -> i32 {
    let mut grammar_id: i32 = 96 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_CertificateChainType(CertificateChainType);
    while done == 0 {
        match grammar_id {
            96 => {
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
                                1200 as i32 as usize,
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
                            error = decode_din_SubCertificatesType(
                                stream,
                                &mut (*CertificateChainType).SubCertificates,
                            );
                            if error == 0 as i32 {
                                (*CertificateChainType).set_SubCertificates_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_DC_EVSEStatusType(
    stream: &mut ExiBitstream,
    mut DC_EVSEStatusType: *mut din_DC_EVSEStatusType,
) -> i32 {
    let mut grammar_id: i32 = 98 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_DC_EVSEStatusType(DC_EVSEStatusType);
    while done == 0 {
        match grammar_id {
            98 => {
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
                                        2 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_EVSEStatusType).EVSEIsolationStatus =
                                            value as din_isolationLevelType;
                                        (*DC_EVSEStatusType)
                                            .set_EVSEIsolationStatus_isUsed(1 as u32);
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
                                        grammar_id = 99 as i32;
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
                                        4 as i32 as usize,
                                        &mut value_0,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_EVSEStatusType).EVSEStatusCode =
                                            value_0 as din_DC_EVSEStatusCodeType;
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
                                        grammar_id = 100 as i32;
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
                                    let mut value_1: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        4 as i32 as usize,
                                        &mut value_1,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_EVSEStatusType).EVSEStatusCode =
                                            value_1 as din_DC_EVSEStatusCodeType;
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
                                        grammar_id = 100 as i32;
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
            100 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*DC_EVSEStatusType).NotificationMaxDelay,
                            );
                            if error == 0 as i32 {
                                grammar_id = 101 as i32;
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
                                        2 as i32 as usize,
                                        &mut value_2,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_EVSEStatusType).EVSENotification =
                                            value_2 as din_EVSENotificationType;
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
                                        grammar_id = 3 as i32;
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
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_PhysicalValueType(
    stream: &mut ExiBitstream,
    mut PhysicalValueType: *mut din_PhysicalValueType,
) -> i32 {
    let mut grammar_id: i32 = 102 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_PhysicalValueType(PhysicalValueType);
    while done == 0 {
        match grammar_id {
            102 => {
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
                                        (*PhysicalValueType).Multiplier =
                                            value.wrapping_add(-(3 as i32) as u32) as int8_t;
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
                                        grammar_id = 103 as i32;
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
            103 => {
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
                                    let mut value_0: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        4 as i32 as usize,
                                        &mut value_0,
                                    );
                                    if error == 0 as i32 {
                                        (*PhysicalValueType).Unit = value_0 as din_unitSymbolType;
                                        (*PhysicalValueType).set_Unit_isUsed(1 as u32);
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
                                        grammar_id = 104 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            error =
                                decode_exi_type_integer16(stream, &mut (*PhysicalValueType).Value);
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
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
                            error =
                                decode_exi_type_integer16(stream, &mut (*PhysicalValueType).Value);
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_ParameterType(
    stream: &mut ExiBitstream,
    mut ParameterType: *mut din_ParameterType,
) -> i32 {
    let mut grammar_id: i32 = 105 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_ParameterType(ParameterType);
    while done == 0 {
        match grammar_id {
            105 => {
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
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            grammar_id = 106 as i32;
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
                            let mut value: u32 = 0;
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                3 as i32 as usize,
                                &mut value,
                            );
                            if error == 0 as i32 {
                                (*ParameterType).ValueType = value as din_valueType;
                            }
                            grammar_id = 107 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            107 => {
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
                                    let mut value_0: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_0,
                                    );
                                    if error == 0 as i32 {
                                        (*ParameterType).boolValue = value_0 as i32;
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
                                        grammar_id = 3 as i32;
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
                                    let mut value_1: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        8 as i32 as usize,
                                        &mut value_1,
                                    );
                                    if error == 0 as i32 {
                                        (*ParameterType).byteValue =
                                            value_1.wrapping_add(-(128 as i32) as u32) as int8_t;
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
                                        grammar_id = 3 as i32;
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
                                grammar_id = 3 as i32;
                            }
                        }
                        3 => {
                            error =
                                decode_exi_type_integer32(stream, &mut (*ParameterType).intValue);
                            if error == 0 as i32 {
                                (*ParameterType).set_intValue_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        4 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*ParameterType).physicalValue,
                            );
                            if error == 0 as i32 {
                                (*ParameterType).set_physicalValue_isUsed(1 as u32);
                                grammar_id = 3 as i32;
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
                                        &mut (*ParameterType).stringValue.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*ParameterType).stringValue.charactersLen as i32
                                            >= 2 as i32
                                        {
                                            (*ParameterType).stringValue.charactersLen =
                                                ((*ParameterType).stringValue.charactersLen as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*ParameterType).stringValue.charactersLen as usize,
                                                ((*ParameterType).stringValue.characters)
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
                                        (*ParameterType).set_stringValue_isUsed(1 as u32);
                                        grammar_id = 3 as i32;
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
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_ParameterSetType(
    stream: &mut ExiBitstream,
    mut ParameterSetType: *mut din_ParameterSetType,
) -> i32 {
    let mut grammar_id: i32 = 108 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_ParameterSetType(ParameterSetType);
    while done == 0 {
        match grammar_id {
            108 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*ParameterSetType).ParameterSetID,
                            );
                            if error == 0 as i32 {
                                grammar_id = 109 as i32;
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
                            error = decode_din_ParameterType(
                                stream,
                                &mut (*ParameterSetType).Parameter,
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
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = -(110 as i32);
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_ListOfRootCertificateIDsType(
    stream: &mut ExiBitstream,
    mut ListOfRootCertificateIDsType: *mut din_ListOfRootCertificateIDsType,
) -> i32 {
    let mut grammar_id: i32 = 111 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_ListOfRootCertificateIDsType(ListOfRootCertificateIDsType);
    while done == 0 {
        match grammar_id {
            111 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ListOfRootCertificateIDsType).RootCertificateID.arrayLen as i32)
                                < 5 as i32
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
                                            &mut (*((*ListOfRootCertificateIDsType)
                                                .RootCertificateID
                                                .array)
                                                .as_mut_ptr()
                                                .offset(
                                                    (*ListOfRootCertificateIDsType)
                                                        .RootCertificateID
                                                        .arrayLen
                                                        as isize,
                                                ))
                                            .charactersLen,
                                        );
                                        if error == 0 as i32 {
                                            if (*ListOfRootCertificateIDsType)
                                                .RootCertificateID
                                                .array
                                                [(*ListOfRootCertificateIDsType)
                                                    .RootCertificateID
                                                    .arrayLen
                                                    as usize]
                                                .charactersLen
                                                as i32
                                                >= 2 as i32
                                            {
                                                (*ListOfRootCertificateIDsType)
                                                    .RootCertificateID
                                                    .array
                                                    [(*ListOfRootCertificateIDsType)
                                                        .RootCertificateID
                                                        .arrayLen
                                                        as usize]
                                                    .charactersLen =
                                                    ((*ListOfRootCertificateIDsType)
                                                        .RootCertificateID
                                                        .array
                                                        [(*ListOfRootCertificateIDsType)
                                                            .RootCertificateID
                                                            .arrayLen
                                                            as usize]
                                                        .charactersLen
                                                        as i32
                                                        - 2 as i32)
                                                        as u16;
                                                error = exi_basetypes_decoder_characters(
                                                    stream,
                                                    (*ListOfRootCertificateIDsType)
                                                        .RootCertificateID
                                                        .array
                                                        [(*ListOfRootCertificateIDsType)
                                                            .RootCertificateID
                                                            .arrayLen
                                                            as usize]
                                                        .charactersLen
                                                        as usize,
                                                    ((*ListOfRootCertificateIDsType)
                                                        .RootCertificateID
                                                        .array
                                                        [(*ListOfRootCertificateIDsType)
                                                            .RootCertificateID
                                                            .arrayLen
                                                            as usize]
                                                        .characters)
                                                        .as_mut_ptr(),
                                                    (40 as i32 + 1 as i32) as usize,
                                                );
                                                if error == 0 as i32 {
                                                    (*ListOfRootCertificateIDsType)
                                                        .RootCertificateID
                                                        .arrayLen =
                                                        ((*ListOfRootCertificateIDsType)
                                                            .RootCertificateID
                                                            .arrayLen)
                                                            .wrapping_add(1);
                                                    (*ListOfRootCertificateIDsType)
                                                        .RootCertificateID
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
                                        grammar_id = 112 as i32;
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
            112 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ListOfRootCertificateIDsType).RootCertificateID.arrayLen as i32)
                                < 5 as i32
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
                                            &mut (*((*ListOfRootCertificateIDsType)
                                                .RootCertificateID
                                                .array)
                                                .as_mut_ptr()
                                                .offset(
                                                    (*ListOfRootCertificateIDsType)
                                                        .RootCertificateID
                                                        .arrayLen
                                                        as isize,
                                                ))
                                            .charactersLen,
                                        );
                                        if error == 0 as i32 {
                                            if (*ListOfRootCertificateIDsType)
                                                .RootCertificateID
                                                .array
                                                [(*ListOfRootCertificateIDsType)
                                                    .RootCertificateID
                                                    .arrayLen
                                                    as usize]
                                                .charactersLen
                                                as i32
                                                >= 2 as i32
                                            {
                                                (*ListOfRootCertificateIDsType)
                                                    .RootCertificateID
                                                    .array
                                                    [(*ListOfRootCertificateIDsType)
                                                        .RootCertificateID
                                                        .arrayLen
                                                        as usize]
                                                    .charactersLen =
                                                    ((*ListOfRootCertificateIDsType)
                                                        .RootCertificateID
                                                        .array
                                                        [(*ListOfRootCertificateIDsType)
                                                            .RootCertificateID
                                                            .arrayLen
                                                            as usize]
                                                        .charactersLen
                                                        as i32
                                                        - 2 as i32)
                                                        as u16;
                                                error = exi_basetypes_decoder_characters(
                                                    stream,
                                                    (*ListOfRootCertificateIDsType)
                                                        .RootCertificateID
                                                        .array
                                                        [(*ListOfRootCertificateIDsType)
                                                            .RootCertificateID
                                                            .arrayLen
                                                            as usize]
                                                        .charactersLen
                                                        as usize,
                                                    ((*ListOfRootCertificateIDsType)
                                                        .RootCertificateID
                                                        .array
                                                        [(*ListOfRootCertificateIDsType)
                                                            .RootCertificateID
                                                            .arrayLen
                                                            as usize]
                                                        .characters)
                                                        .as_mut_ptr(),
                                                    (40 as i32 + 1 as i32) as usize,
                                                );
                                                if error == 0 as i32 {
                                                    (*ListOfRootCertificateIDsType)
                                                        .RootCertificateID
                                                        .arrayLen =
                                                        ((*ListOfRootCertificateIDsType)
                                                            .RootCertificateID
                                                            .arrayLen)
                                                            .wrapping_add(1);
                                                    (*ListOfRootCertificateIDsType)
                                                        .RootCertificateID
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
                                        grammar_id = 112 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_PaymentOptionsType(
    stream: &mut ExiBitstream,
    mut PaymentOptionsType: *mut din_PaymentOptionsType,
) -> i32 {
    let mut grammar_id: i32 = 113 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_PaymentOptionsType(PaymentOptionsType);
    while done == 0 {
        match grammar_id {
            113 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*PaymentOptionsType).PaymentOption.arrayLen as i32) < 2 as i32 {
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
                                            (*PaymentOptionsType).PaymentOption.array
                                                [(*PaymentOptionsType).PaymentOption.arrayLen
                                                    as usize] = value as din_paymentOptionType;
                                            (*PaymentOptionsType).PaymentOption.arrayLen =
                                                ((*PaymentOptionsType).PaymentOption.arrayLen)
                                                    .wrapping_add(1);
                                            (*PaymentOptionsType).PaymentOption.arrayLen;
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
                                        grammar_id = 114 as i32;
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
            114 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*PaymentOptionsType).PaymentOption.arrayLen as i32) < 2 as i32 {
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
                                            (*PaymentOptionsType).PaymentOption.array
                                                [(*PaymentOptionsType).PaymentOption.arrayLen
                                                    as usize] = value_0 as din_paymentOptionType;
                                            (*PaymentOptionsType).PaymentOption.arrayLen =
                                                ((*PaymentOptionsType).PaymentOption.arrayLen)
                                                    .wrapping_add(1);
                                            (*PaymentOptionsType).PaymentOption.arrayLen;
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
                                        grammar_id = 3 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_SelectedServiceListType(
    stream: &mut ExiBitstream,
    mut SelectedServiceListType: *mut din_SelectedServiceListType,
) -> i32 {
    let mut grammar_id: i32 = 115 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_SelectedServiceListType(SelectedServiceListType);
    while done == 0 {
        match grammar_id {
            115 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SelectedServiceListType).SelectedService.arrayLen as i32)
                                < 16 as i32
                            {
                                let fresh4 = (*SelectedServiceListType).SelectedService.arrayLen;
                                (*SelectedServiceListType).SelectedService.arrayLen =
                                    ((*SelectedServiceListType).SelectedService.arrayLen)
                                        .wrapping_add(1);
                                error = decode_din_SelectedServiceType(
                                    stream,
                                    &mut *((*SelectedServiceListType).SelectedService.array)
                                        .as_mut_ptr()
                                        .offset(fresh4 as isize),
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
                            if ((*SelectedServiceListType).SelectedService.arrayLen as i32)
                                < 16 as i32
                            {
                                let fresh5 = (*SelectedServiceListType).SelectedService.arrayLen;
                                (*SelectedServiceListType).SelectedService.arrayLen =
                                    ((*SelectedServiceListType).SelectedService.arrayLen)
                                        .wrapping_add(1);
                                error = decode_din_SelectedServiceType(
                                    stream,
                                    &mut *((*SelectedServiceListType).SelectedService.array)
                                        .as_mut_ptr()
                                        .offset(fresh5 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 116 as i32;
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_AC_EVChargeParameterType(
    stream: &mut ExiBitstream,
    mut AC_EVChargeParameterType: *mut din_AC_EVChargeParameterType,
) -> i32 {
    let mut grammar_id: i32 = 117 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_AC_EVChargeParameterType(AC_EVChargeParameterType);
    while done == 0 {
        match grammar_id {
            117 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint32(
                                stream,
                                &mut (*AC_EVChargeParameterType).DepartureTime,
                            );
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
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*AC_EVChargeParameterType).EAmount,
                            );
                            if error == 0 as i32 {
                                grammar_id = 119 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            119 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*AC_EVChargeParameterType).EVMaxVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 120 as i32;
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
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*AC_EVChargeParameterType).EVMaxCurrent,
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
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*AC_EVChargeParameterType).EVMinCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_DC_EVChargeParameterType(
    stream: &mut ExiBitstream,
    mut DC_EVChargeParameterType: *mut din_DC_EVChargeParameterType,
) -> i32 {
    let mut grammar_id: i32 = 122 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_DC_EVChargeParameterType(DC_EVChargeParameterType);
    while done == 0 {
        match grammar_id {
            122 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_DC_EVStatusType(
                                stream,
                                &mut (*DC_EVChargeParameterType).DC_EVStatus,
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
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*DC_EVChargeParameterType).EVMaximumCurrentLimit,
                            );
                            if error == 0 as i32 {
                                grammar_id = 124 as i32;
                            }
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
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*DC_EVChargeParameterType).EVMaximumPowerLimit,
                            );
                            if error == 0 as i32 {
                                (*DC_EVChargeParameterType)
                                    .set_EVMaximumPowerLimit_isUsed(1 as u32);
                                grammar_id = 125 as i32;
                            }
                        }
                        1 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*DC_EVChargeParameterType).EVMaximumVoltageLimit,
                            );
                            if error == 0 as i32 {
                                grammar_id = 126 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            125 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*DC_EVChargeParameterType).EVMaximumVoltageLimit,
                            );
                            if error == 0 as i32 {
                                grammar_id = 126 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            126 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*DC_EVChargeParameterType).EVEnergyCapacity,
                            );
                            if error == 0 as i32 {
                                (*DC_EVChargeParameterType).set_EVEnergyCapacity_isUsed(1 as u32);
                                grammar_id = 127 as i32;
                            }
                        }
                        1 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*DC_EVChargeParameterType).EVEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*DC_EVChargeParameterType).set_EVEnergyRequest_isUsed(1 as u32);
                                grammar_id = 128 as i32;
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
                                    let mut value: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_EVChargeParameterType).FullSOC = value as int8_t;
                                        (*DC_EVChargeParameterType).set_FullSOC_isUsed(1 as u32);
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
                                        grammar_id = 129 as i32;
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
                                    let mut value_0: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_0,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_EVChargeParameterType).BulkSOC = value_0 as int8_t;
                                        (*DC_EVChargeParameterType).set_BulkSOC_isUsed(1 as u32);
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
                                        grammar_id = 3 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        4 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            127 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*DC_EVChargeParameterType).EVEnergyRequest,
                            );
                            if error == 0 as i32 {
                                (*DC_EVChargeParameterType).set_EVEnergyRequest_isUsed(1 as u32);
                                grammar_id = 128 as i32;
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
                                        7 as i32 as usize,
                                        &mut value_1,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_EVChargeParameterType).FullSOC = value_1 as int8_t;
                                        (*DC_EVChargeParameterType).set_FullSOC_isUsed(1 as u32);
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
                                        grammar_id = 129 as i32;
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
                                    let mut value_2: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_2,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_EVChargeParameterType).BulkSOC = value_2 as int8_t;
                                        (*DC_EVChargeParameterType).set_BulkSOC_isUsed(1 as u32);
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
                                        grammar_id = 3 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        3 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
                                        (*DC_EVChargeParameterType).FullSOC = value_3 as int8_t;
                                        (*DC_EVChargeParameterType).set_FullSOC_isUsed(1 as u32);
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
                                        grammar_id = 129 as i32;
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
                                    let mut value_4: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        7 as i32 as usize,
                                        &mut value_4,
                                    );
                                    if error == 0 as i32 {
                                        (*DC_EVChargeParameterType).BulkSOC = value_4 as int8_t;
                                        (*DC_EVChargeParameterType).set_BulkSOC_isUsed(1 as u32);
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
                                        grammar_id = 3 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
                                        (*DC_EVChargeParameterType).BulkSOC = value_5 as int8_t;
                                        (*DC_EVChargeParameterType).set_BulkSOC_isUsed(1 as u32);
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
                                        grammar_id = 3 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_EVChargeParameterType(
    stream: &mut ExiBitstream,
    mut EVChargeParameterType: *mut din_EVChargeParameterType,
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
unsafe extern "C" fn decode_din_ChargingProfileType(
    stream: &mut ExiBitstream,
    mut ChargingProfileType: *mut din_ChargingProfileType,
) -> i32 {
    let mut grammar_id: i32 = 130 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_ChargingProfileType(ChargingProfileType);
    while done == 0 {
        match grammar_id {
            130 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*ChargingProfileType).SAScheduleTupleID,
                            );
                            if error == 0 as i32 {
                                grammar_id = 131 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            131 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ChargingProfileType).ProfileEntry.arrayLen as i32) < 24 as i32 {
                                let fresh6 = (*ChargingProfileType).ProfileEntry.arrayLen;
                                (*ChargingProfileType).ProfileEntry.arrayLen =
                                    ((*ChargingProfileType).ProfileEntry.arrayLen).wrapping_add(1);
                                error = decode_din_ProfileEntryType(
                                    stream,
                                    &mut *((*ChargingProfileType).ProfileEntry.array)
                                        .as_mut_ptr()
                                        .offset(fresh6 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 132 as i32;
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
                            if ((*ChargingProfileType).ProfileEntry.arrayLen as i32) < 24 as i32 {
                                let fresh7 = (*ChargingProfileType).ProfileEntry.arrayLen;
                                (*ChargingProfileType).ProfileEntry.arrayLen =
                                    ((*ChargingProfileType).ProfileEntry.arrayLen).wrapping_add(1);
                                error = decode_din_ProfileEntryType(
                                    stream,
                                    &mut *((*ChargingProfileType).ProfileEntry.array)
                                        .as_mut_ptr()
                                        .offset(fresh7 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 132 as i32;
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_EVSEStatusType(
    stream: &mut ExiBitstream,
    mut EVSEStatusType: *mut din_EVSEStatusType,
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
unsafe extern "C" fn decode_din_KeyInfoType(
    stream: &mut ExiBitstream,
    mut KeyInfoType: *mut din_KeyInfoType,
) -> i32 {
    let mut grammar_id: i32 = 133 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_KeyInfoType(KeyInfoType);
    while done == 0 {
        match grammar_id {
            133 => {
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
                            grammar_id = 134 as i32;
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
                                        grammar_id = 3 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        2 => {
                            error = decode_din_KeyValueType(stream, &mut (*KeyInfoType).KeyValue);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_KeyValue_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        3 => {
                            error = decode_din_RetrievalMethodType(
                                stream,
                                &mut (*KeyInfoType).RetrievalMethod,
                            );
                            if error == 0 as i32 {
                                (*KeyInfoType).set_RetrievalMethod_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        4 => {
                            error = decode_din_X509DataType(stream, &mut (*KeyInfoType).X509Data);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_X509Data_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        5 => {
                            error = decode_din_PGPDataType(stream, &mut (*KeyInfoType).PGPData);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_PGPData_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        6 => {
                            error = decode_din_SPKIDataType(stream, &mut (*KeyInfoType).SPKIData);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_SPKIData_isUsed(1 as u32);
                                grammar_id = 3 as i32;
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
                                        grammar_id = 3 as i32;
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
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            134 => {
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
                                        grammar_id = 3 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            error = decode_din_KeyValueType(stream, &mut (*KeyInfoType).KeyValue);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_KeyValue_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            error = decode_din_RetrievalMethodType(
                                stream,
                                &mut (*KeyInfoType).RetrievalMethod,
                            );
                            if error == 0 as i32 {
                                (*KeyInfoType).set_RetrievalMethod_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        3 => {
                            error = decode_din_X509DataType(stream, &mut (*KeyInfoType).X509Data);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_X509Data_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        4 => {
                            error = decode_din_PGPDataType(stream, &mut (*KeyInfoType).PGPData);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_PGPData_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        5 => {
                            error = decode_din_SPKIDataType(stream, &mut (*KeyInfoType).SPKIData);
                            if error == 0 as i32 {
                                (*KeyInfoType).set_SPKIData_isUsed(1 as u32);
                                grammar_id = 3 as i32;
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
                                        grammar_id = 3 as i32;
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
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_ServiceChargeType(
    stream: &mut ExiBitstream,
    mut ServiceChargeType: *mut din_ServiceChargeType,
) -> i32 {
    let mut grammar_id: i32 = 135 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_ServiceChargeType(ServiceChargeType);
    while done == 0 {
        match grammar_id {
            135 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_ServiceTagType(
                                stream,
                                &mut (*ServiceChargeType).ServiceTag,
                            );
                            if error == 0 as i32 {
                                grammar_id = 136 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
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
                                        1 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*ServiceChargeType).FreeService = value as i32;
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
                                        4 as i32 as usize,
                                        &mut value_0,
                                    );
                                    if error == 0 as i32 {
                                        (*ServiceChargeType).EnergyTransferType =
                                            value_0 as din_EVSESupportedEnergyTransferType;
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
                                        grammar_id = 3 as i32;
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
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_ServiceParameterListType(
    stream: &mut ExiBitstream,
    mut ServiceParameterListType: *mut din_ServiceParameterListType,
) -> i32 {
    let mut grammar_id: i32 = 138 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_ServiceParameterListType(ServiceParameterListType);
    while done == 0 {
        match grammar_id {
            138 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*ServiceParameterListType).ParameterSet.arrayLen as i32) < 5 as i32
                            {
                                let fresh8 = (*ServiceParameterListType).ParameterSet.arrayLen;
                                (*ServiceParameterListType).ParameterSet.arrayLen =
                                    ((*ServiceParameterListType).ParameterSet.arrayLen)
                                        .wrapping_add(1);
                                error = decode_din_ParameterSetType(
                                    stream,
                                    &mut *((*ServiceParameterListType).ParameterSet.array)
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
                            if ((*ServiceParameterListType).ParameterSet.arrayLen as i32) < 5 as i32
                            {
                                let fresh9 = (*ServiceParameterListType).ParameterSet.arrayLen;
                                (*ServiceParameterListType).ParameterSet.arrayLen =
                                    ((*ServiceParameterListType).ParameterSet.arrayLen)
                                        .wrapping_add(1);
                                error = decode_din_ParameterSetType(
                                    stream,
                                    &mut *((*ServiceParameterListType).ParameterSet.array)
                                        .as_mut_ptr()
                                        .offset(fresh9 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 139 as i32;
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_SAScheduleListType(
    stream: &mut ExiBitstream,
    mut SAScheduleListType: *mut din_SAScheduleListType,
) -> i32 {
    let mut grammar_id: i32 = 140 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_SAScheduleListType(SAScheduleListType);
    while done == 0 {
        match grammar_id {
            140 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            if ((*SAScheduleListType).SAScheduleTuple.arrayLen as i32) < 5 as i32 {
                                let fresh10 = (*SAScheduleListType).SAScheduleTuple.arrayLen;
                                (*SAScheduleListType).SAScheduleTuple.arrayLen =
                                    ((*SAScheduleListType).SAScheduleTuple.arrayLen)
                                        .wrapping_add(1);
                                error = decode_din_SAScheduleTupleType(
                                    stream,
                                    &mut *((*SAScheduleListType).SAScheduleTuple.array)
                                        .as_mut_ptr()
                                        .offset(fresh10 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 141 as i32;
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
                            if ((*SAScheduleListType).SAScheduleTuple.arrayLen as i32) < 5 as i32 {
                                let fresh11 = (*SAScheduleListType).SAScheduleTuple.arrayLen;
                                (*SAScheduleListType).SAScheduleTuple.arrayLen =
                                    ((*SAScheduleListType).SAScheduleTuple.arrayLen)
                                        .wrapping_add(1);
                                error = decode_din_SAScheduleTupleType(
                                    stream,
                                    &mut *((*SAScheduleListType).SAScheduleTuple.array)
                                        .as_mut_ptr()
                                        .offset(fresh11 as isize),
                                );
                            } else {
                                error = -(110 as i32);
                            }
                            grammar_id = 141 as i32;
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_SASchedulesType(
    stream: &mut ExiBitstream,
    mut SASchedulesType: *mut din_SASchedulesType,
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
unsafe extern "C" fn decode_din_DC_EVPowerDeliveryParameterType(
    stream: &mut ExiBitstream,
    mut DC_EVPowerDeliveryParameterType: *mut din_DC_EVPowerDeliveryParameterType,
) -> i32 {
    let mut grammar_id: i32 = 142 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_DC_EVPowerDeliveryParameterType(DC_EVPowerDeliveryParameterType);
    while done == 0 {
        match grammar_id {
            142 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_DC_EVStatusType(
                                stream,
                                &mut (*DC_EVPowerDeliveryParameterType).DC_EVStatus,
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
                                        (*DC_EVPowerDeliveryParameterType).BulkChargingComplete =
                                            value as i32;
                                        (*DC_EVPowerDeliveryParameterType)
                                            .set_BulkChargingComplete_isUsed(1 as u32);
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
                                        grammar_id = 144 as i32;
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
                                        (*DC_EVPowerDeliveryParameterType).ChargingComplete =
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
                                        grammar_id = 3 as i32;
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
            144 => {
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
                                        (*DC_EVPowerDeliveryParameterType).ChargingComplete =
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
                                        grammar_id = 3 as i32;
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
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_EVPowerDeliveryParameterType(
    stream: &mut ExiBitstream,
    mut EVPowerDeliveryParameterType: *mut din_EVPowerDeliveryParameterType,
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
unsafe extern "C" fn decode_din_ObjectType(
    stream: &mut ExiBitstream,
    mut ObjectType: *mut din_ObjectType,
) -> i32 {
    let mut grammar_id: i32 = 145 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_ObjectType(ObjectType);
    while done == 0 {
        match grammar_id {
            145 => {
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
                            grammar_id = 146 as i32;
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
                            grammar_id = 147 as i32;
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
                            grammar_id = 148 as i32;
                        }
                        3 => {
                            error = -(50 as i32);
                        }
                        4 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            146 => {
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
                            grammar_id = 147 as i32;
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
                            grammar_id = 148 as i32;
                        }
                        2 => {
                            error = -(50 as i32);
                        }
                        3 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
                                grammar_id = 3 as i32;
                            }
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
                            grammar_id = 148 as i32;
                        }
                        1 => {
                            error = -(50 as i32);
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
                                grammar_id = 3 as i32;
                            }
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
                            error = -(50 as i32);
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_ServiceTagListType(
    stream: &mut ExiBitstream,
    mut ServiceTagListType: *mut din_ServiceTagListType,
) -> i32 {
    let mut grammar_id: i32 = 149 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_ServiceTagListType(ServiceTagListType);
    while done == 0 {
        match grammar_id {
            149 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error =
                                decode_din_ServiceType(stream, &mut (*ServiceTagListType).Service);
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
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = -(110 as i32);
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_DC_EVSEChargeParameterType(
    stream: &mut ExiBitstream,
    mut DC_EVSEChargeParameterType: *mut din_DC_EVSEChargeParameterType,
) -> i32 {
    let mut grammar_id: i32 = 151 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_DC_EVSEChargeParameterType(DC_EVSEChargeParameterType);
    while done == 0 {
        match grammar_id {
            151 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_DC_EVSEStatusType(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).DC_EVSEStatus,
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
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).EVSEMaximumCurrentLimit,
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
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).EVSEMaximumPowerLimit,
                            );
                            if error == 0 as i32 {
                                (*DC_EVSEChargeParameterType)
                                    .set_EVSEMaximumPowerLimit_isUsed(1 as u32);
                                grammar_id = 154 as i32;
                            }
                        }
                        1 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).EVSEMaximumVoltageLimit,
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
            154 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).EVSEMaximumVoltageLimit,
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
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).EVSEMinimumCurrentLimit,
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
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).EVSEMinimumVoltageLimit,
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
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).EVSECurrentRegulationTolerance,
                            );
                            if error == 0 as i32 {
                                (*DC_EVSEChargeParameterType)
                                    .set_EVSECurrentRegulationTolerance_isUsed(1 as u32);
                                grammar_id = 158 as i32;
                            }
                        }
                        1 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).EVSEPeakCurrentRipple,
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
            158 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).EVSEPeakCurrentRipple,
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
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*DC_EVSEChargeParameterType).EVSEEnergyToBeDelivered,
                            );
                            if error == 0 as i32 {
                                (*DC_EVSEChargeParameterType)
                                    .set_EVSEEnergyToBeDelivered_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_AC_EVSEChargeParameterType(
    stream: &mut ExiBitstream,
    mut AC_EVSEChargeParameterType: *mut din_AC_EVSEChargeParameterType,
) -> i32 {
    let mut grammar_id: i32 = 160 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_AC_EVSEChargeParameterType(AC_EVSEChargeParameterType);
    while done == 0 {
        match grammar_id {
            160 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_AC_EVSEStatusType(
                                stream,
                                &mut (*AC_EVSEChargeParameterType).AC_EVSEStatus,
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
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*AC_EVSEChargeParameterType).EVSEMaxVoltage,
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
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*AC_EVSEChargeParameterType).EVSEMaxCurrent,
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
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*AC_EVSEChargeParameterType).EVSEMinCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_EVSEChargeParameterType(
    stream: &mut ExiBitstream,
    mut EVSEChargeParameterType: *mut din_EVSEChargeParameterType,
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
unsafe extern "C" fn decode_din_MeterInfoType(
    stream: &mut ExiBitstream,
    mut MeterInfoType: *mut din_MeterInfoType,
) -> i32 {
    let mut grammar_id: i32 = 164 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_MeterInfoType(MeterInfoType);
    while done == 0 {
        match grammar_id {
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
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*MeterInfoType).MeterReading,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_MeterReading_isUsed(1 as u32);
                                grammar_id = 166 as i32;
                            }
                        }
                        1 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*MeterInfoType).SigMeterReading.bytesLen,
                                &mut *((*MeterInfoType).SigMeterReading.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                32 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_SigMeterReading_isUsed(1 as u32);
                                grammar_id = 167 as i32;
                            }
                        }
                        2 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*MeterInfoType).MeterStatus,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_MeterStatus_isUsed(1 as u32);
                                grammar_id = 168 as i32;
                            }
                        }
                        3 => {
                            error = decode_exi_type_integer64(stream, &mut (*MeterInfoType).TMeter);
                            if error == 0 as i32 {
                                (*MeterInfoType).set_TMeter_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        4 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            166 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*MeterInfoType).SigMeterReading.bytesLen,
                                &mut *((*MeterInfoType).SigMeterReading.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                32 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_SigMeterReading_isUsed(1 as u32);
                                grammar_id = 167 as i32;
                            }
                        }
                        1 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*MeterInfoType).MeterStatus,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_MeterStatus_isUsed(1 as u32);
                                grammar_id = 168 as i32;
                            }
                        }
                        2 => {
                            error = decode_exi_type_integer64(stream, &mut (*MeterInfoType).TMeter);
                            if error == 0 as i32 {
                                (*MeterInfoType).set_TMeter_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        3 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*MeterInfoType).MeterStatus,
                            );
                            if error == 0 as i32 {
                                (*MeterInfoType).set_MeterStatus_isUsed(1 as u32);
                                grammar_id = 168 as i32;
                            }
                        }
                        1 => {
                            error = decode_exi_type_integer64(stream, &mut (*MeterInfoType).TMeter);
                            if error == 0 as i32 {
                                (*MeterInfoType).set_TMeter_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            168 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_integer64(stream, &mut (*MeterInfoType).TMeter);
                            if error == 0 as i32 {
                                (*MeterInfoType).set_TMeter_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_CertificateInstallationResType(
    stream: &mut ExiBitstream,
    mut CertificateInstallationResType: *mut din_CertificateInstallationResType,
) -> i32 {
    let mut grammar_id: i32 = 169 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_CertificateInstallationResType(CertificateInstallationResType);
    while done == 0 {
        match grammar_id {
            169 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*CertificateInstallationResType).Id.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*CertificateInstallationResType).Id.charactersLen as i32
                                    >= 2 as i32
                                {
                                    (*CertificateInstallationResType).Id.charactersLen =
                                        ((*CertificateInstallationResType).Id.charactersLen as i32
                                            - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*CertificateInstallationResType).Id.charactersLen as usize,
                                        ((*CertificateInstallationResType).Id.characters)
                                            .as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            grammar_id = 170 as i32;
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
                                        5 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*CertificateInstallationResType).ResponseCode =
                                            value as din_responseCodeType;
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
                                        grammar_id = 171 as i32;
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
            171 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_CertificateChainType(
                                stream,
                                &mut (*CertificateInstallationResType).ContractSignatureCertChain,
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
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*CertificateInstallationResType)
                                    .ContractSignatureEncryptedPrivateKey
                                    .bytesLen,
                                &mut *((*CertificateInstallationResType)
                                    .ContractSignatureEncryptedPrivateKey
                                    .bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                128 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 173 as i32;
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
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*CertificateInstallationResType).DHParams.bytesLen,
                                &mut *((*CertificateInstallationResType).DHParams.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                256 as i32 as usize,
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
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    error = exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut (*CertificateInstallationResType)
                                            .ContractID
                                            .charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*CertificateInstallationResType)
                                            .ContractID
                                            .charactersLen
                                            as i32
                                            >= 2 as i32
                                        {
                                            (*CertificateInstallationResType)
                                                .ContractID
                                                .charactersLen = ((*CertificateInstallationResType)
                                                .ContractID
                                                .charactersLen
                                                as i32
                                                - 2 as i32)
                                                as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*CertificateInstallationResType)
                                                    .ContractID
                                                    .charactersLen
                                                    as usize,
                                                ((*CertificateInstallationResType)
                                                    .ContractID
                                                    .characters)
                                                    .as_mut_ptr(),
                                                (24 as i32 + 1 as i32) as usize,
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
                                        grammar_id = 3 as i32;
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
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_CableCheckReqType(
    stream: &mut ExiBitstream,
    mut CableCheckReqType: *mut din_CableCheckReqType,
) -> i32 {
    let mut grammar_id: i32 = 175 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_CableCheckReqType(CableCheckReqType);
    while done == 0 {
        match grammar_id {
            175 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_DC_EVStatusType(
                                stream,
                                &mut (*CableCheckReqType).DC_EVStatus,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_CableCheckResType(
    stream: &mut ExiBitstream,
    mut CableCheckResType: *mut din_CableCheckResType,
) -> i32 {
    let mut grammar_id: i32 = 176 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_CableCheckResType(CableCheckResType);
    while done == 0 {
        match grammar_id {
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
                                        5 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*CableCheckResType).ResponseCode =
                                            value as din_responseCodeType;
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
                            error = decode_din_DC_EVSEStatusType(
                                stream,
                                &mut (*CableCheckResType).DC_EVSEStatus,
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
                                        (*CableCheckResType).EVSEProcessing =
                                            value_0 as din_EVSEProcessingType;
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
                                        grammar_id = 3 as i32;
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
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_PreChargeReqType(
    stream: &mut ExiBitstream,
    mut PreChargeReqType: *mut din_PreChargeReqType,
) -> i32 {
    let mut grammar_id: i32 = 179 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_PreChargeReqType(PreChargeReqType);
    while done == 0 {
        match grammar_id {
            179 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_DC_EVStatusType(
                                stream,
                                &mut (*PreChargeReqType).DC_EVStatus,
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
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*PreChargeReqType).EVTargetVoltage,
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
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*PreChargeReqType).EVTargetCurrent,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_PreChargeResType(
    stream: &mut ExiBitstream,
    mut PreChargeResType: *mut din_PreChargeResType,
) -> i32 {
    let mut grammar_id: i32 = 182 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_PreChargeResType(PreChargeResType);
    while done == 0 {
        match grammar_id {
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
                                        5 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*PreChargeResType).ResponseCode =
                                            value as din_responseCodeType;
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
                            error = decode_din_DC_EVSEStatusType(
                                stream,
                                &mut (*PreChargeResType).DC_EVSEStatus,
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
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*PreChargeResType).EVSEPresentVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_CurrentDemandReqType(
    stream: &mut ExiBitstream,
    mut CurrentDemandReqType: *mut din_CurrentDemandReqType,
) -> i32 {
    let mut grammar_id: i32 = 185 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_CurrentDemandReqType(CurrentDemandReqType);
    while done == 0 {
        match grammar_id {
            185 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_DC_EVStatusType(
                                stream,
                                &mut (*CurrentDemandReqType).DC_EVStatus,
                            );
                            if error == 0 as i32 {
                                grammar_id = 186 as i32;
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
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandReqType).EVTargetCurrent,
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
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandReqType).EVMaximumVoltageLimit,
                            );
                            if error == 0 as i32 {
                                (*CurrentDemandReqType).set_EVMaximumVoltageLimit_isUsed(1 as u32);
                                grammar_id = 188 as i32;
                            }
                        }
                        1 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandReqType).EVMaximumCurrentLimit,
                            );
                            if error == 0 as i32 {
                                (*CurrentDemandReqType).set_EVMaximumCurrentLimit_isUsed(1 as u32);
                                grammar_id = 189 as i32;
                            }
                        }
                        2 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandReqType).EVMaximumPowerLimit,
                            );
                            if error == 0 as i32 {
                                (*CurrentDemandReqType).set_EVMaximumPowerLimit_isUsed(1 as u32);
                                grammar_id = 190 as i32;
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
                                        1 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*CurrentDemandReqType).BulkChargingComplete = value as i32;
                                        (*CurrentDemandReqType)
                                            .set_BulkChargingComplete_isUsed(1 as u32);
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
                        4 => {
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
                                        (*CurrentDemandReqType).ChargingComplete = value_0 as i32;
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
            188 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandReqType).EVMaximumCurrentLimit,
                            );
                            if error == 0 as i32 {
                                (*CurrentDemandReqType).set_EVMaximumCurrentLimit_isUsed(1 as u32);
                                grammar_id = 189 as i32;
                            }
                        }
                        1 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandReqType).EVMaximumPowerLimit,
                            );
                            if error == 0 as i32 {
                                (*CurrentDemandReqType).set_EVMaximumPowerLimit_isUsed(1 as u32);
                                grammar_id = 190 as i32;
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
                                        1 as i32 as usize,
                                        &mut value_1,
                                    );
                                    if error == 0 as i32 {
                                        (*CurrentDemandReqType).BulkChargingComplete =
                                            value_1 as i32;
                                        (*CurrentDemandReqType)
                                            .set_BulkChargingComplete_isUsed(1 as u32);
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
                                        1 as i32 as usize,
                                        &mut value_2,
                                    );
                                    if error == 0 as i32 {
                                        (*CurrentDemandReqType).ChargingComplete = value_2 as i32;
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
            189 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandReqType).EVMaximumPowerLimit,
                            );
                            if error == 0 as i32 {
                                (*CurrentDemandReqType).set_EVMaximumPowerLimit_isUsed(1 as u32);
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
                                    let mut value_3: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_3,
                                    );
                                    if error == 0 as i32 {
                                        (*CurrentDemandReqType).BulkChargingComplete =
                                            value_3 as i32;
                                        (*CurrentDemandReqType)
                                            .set_BulkChargingComplete_isUsed(1 as u32);
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
                        2 => {
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
                                        (*CurrentDemandReqType).ChargingComplete = value_4 as i32;
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
            190 => {
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
                                    let mut value_5: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_5,
                                    );
                                    if error == 0 as i32 {
                                        (*CurrentDemandReqType).BulkChargingComplete =
                                            value_5 as i32;
                                        (*CurrentDemandReqType)
                                            .set_BulkChargingComplete_isUsed(1 as u32);
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
                                        1 as i32 as usize,
                                        &mut value_6,
                                    );
                                    if error == 0 as i32 {
                                        (*CurrentDemandReqType).ChargingComplete = value_6 as i32;
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
                                    let mut value_7: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        &mut value_7,
                                    );
                                    if error == 0 as i32 {
                                        (*CurrentDemandReqType).ChargingComplete = value_7 as i32;
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
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandReqType).RemainingTimeToFullSoC,
                            );
                            if error == 0 as i32 {
                                (*CurrentDemandReqType).set_RemainingTimeToFullSoC_isUsed(1 as u32);
                                grammar_id = 193 as i32;
                            }
                        }
                        1 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandReqType).RemainingTimeToBulkSoC,
                            );
                            if error == 0 as i32 {
                                (*CurrentDemandReqType).set_RemainingTimeToBulkSoC_isUsed(1 as u32);
                                grammar_id = 194 as i32;
                            }
                        }
                        2 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandReqType).EVTargetVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            193 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandReqType).RemainingTimeToBulkSoC,
                            );
                            if error == 0 as i32 {
                                (*CurrentDemandReqType).set_RemainingTimeToBulkSoC_isUsed(1 as u32);
                                grammar_id = 194 as i32;
                            }
                        }
                        1 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandReqType).EVTargetVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
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
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandReqType).EVTargetVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_CurrentDemandResType(
    stream: &mut ExiBitstream,
    mut CurrentDemandResType: *mut din_CurrentDemandResType,
) -> i32 {
    let mut grammar_id: i32 = 195 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_CurrentDemandResType(CurrentDemandResType);
    while done == 0 {
        match grammar_id {
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
                                    let mut value: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*CurrentDemandResType).ResponseCode =
                                            value as din_responseCodeType;
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
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_DC_EVSEStatusType(
                                stream,
                                &mut (*CurrentDemandResType).DC_EVSEStatus,
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
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandResType).EVSEPresentVoltage,
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
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandResType).EVSEPresentCurrent,
                            );
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
                                        (*CurrentDemandResType).EVSECurrentLimitAchieved =
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
                                        grammar_id = 200 as i32;
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
            200 => {
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
                                        (*CurrentDemandResType).EVSEVoltageLimitAchieved =
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
                                        grammar_id = 201 as i32;
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
            201 => {
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
                                        (*CurrentDemandResType).EVSEPowerLimitAchieved =
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
                                        grammar_id = 202 as i32;
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
            202 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandResType).EVSEMaximumVoltageLimit,
                            );
                            if error == 0 as i32 {
                                (*CurrentDemandResType)
                                    .set_EVSEMaximumVoltageLimit_isUsed(1 as u32);
                                grammar_id = 203 as i32;
                            }
                        }
                        1 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandResType).EVSEMaximumCurrentLimit,
                            );
                            if error == 0 as i32 {
                                (*CurrentDemandResType)
                                    .set_EVSEMaximumCurrentLimit_isUsed(1 as u32);
                                grammar_id = 204 as i32;
                            }
                        }
                        2 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandResType).EVSEMaximumPowerLimit,
                            );
                            if error == 0 as i32 {
                                (*CurrentDemandResType).set_EVSEMaximumPowerLimit_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        3 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            203 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandResType).EVSEMaximumCurrentLimit,
                            );
                            if error == 0 as i32 {
                                (*CurrentDemandResType)
                                    .set_EVSEMaximumCurrentLimit_isUsed(1 as u32);
                                grammar_id = 204 as i32;
                            }
                        }
                        1 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandResType).EVSEMaximumPowerLimit,
                            );
                            if error == 0 as i32 {
                                (*CurrentDemandResType).set_EVSEMaximumPowerLimit_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*CurrentDemandResType).EVSEMaximumPowerLimit,
                            );
                            if error == 0 as i32 {
                                (*CurrentDemandResType).set_EVSEMaximumPowerLimit_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_WeldingDetectionReqType(
    stream: &mut ExiBitstream,
    mut WeldingDetectionReqType: *mut din_WeldingDetectionReqType,
) -> i32 {
    let mut grammar_id: i32 = 205 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_WeldingDetectionReqType(WeldingDetectionReqType);
    while done == 0 {
        match grammar_id {
            205 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_DC_EVStatusType(
                                stream,
                                &mut (*WeldingDetectionReqType).DC_EVStatus,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_WeldingDetectionResType(
    stream: &mut ExiBitstream,
    mut WeldingDetectionResType: *mut din_WeldingDetectionResType,
) -> i32 {
    let mut grammar_id: i32 = 206 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_WeldingDetectionResType(WeldingDetectionResType);
    while done == 0 {
        match grammar_id {
            206 => {
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
                                        5 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*WeldingDetectionResType).ResponseCode =
                                            value as din_responseCodeType;
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
                                        grammar_id = 207 as i32;
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
            207 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_DC_EVSEStatusType(
                                stream,
                                &mut (*WeldingDetectionResType).DC_EVSEStatus,
                            );
                            if error == 0 as i32 {
                                grammar_id = 208 as i32;
                            }
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
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*WeldingDetectionResType).EVSEPresentVoltage,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_SessionSetupReqType(
    stream: &mut ExiBitstream,
    mut SessionSetupReqType: *mut din_SessionSetupReqType,
) -> i32 {
    let mut grammar_id: i32 = 209 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_SessionSetupReqType(SessionSetupReqType);
    while done == 0 {
        match grammar_id {
            209 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*SessionSetupReqType).EVCCID.bytesLen,
                                &mut *((*SessionSetupReqType).EVCCID.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                8 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_CertificateInstallationReqType(
    stream: &mut ExiBitstream,
    mut CertificateInstallationReqType: *mut din_CertificateInstallationReqType,
) -> i32 {
    let mut grammar_id: i32 = 210 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_CertificateInstallationReqType(CertificateInstallationReqType);
    while done == 0 {
        match grammar_id {
            210 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*CertificateInstallationReqType).Id.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*CertificateInstallationReqType).Id.charactersLen as i32
                                    >= 2 as i32
                                {
                                    (*CertificateInstallationReqType).Id.charactersLen =
                                        ((*CertificateInstallationReqType).Id.charactersLen as i32
                                            - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*CertificateInstallationReqType).Id.charactersLen as usize,
                                        ((*CertificateInstallationReqType).Id.characters)
                                            .as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*CertificateInstallationReqType).set_Id_isUsed(1 as u32);
                            grammar_id = 211 as i32;
                        }
                        1 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*CertificateInstallationReqType)
                                    .OEMProvisioningCert
                                    .bytesLen,
                                &mut *((*CertificateInstallationReqType).OEMProvisioningCert.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                1200 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 212 as i32;
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
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*CertificateInstallationReqType)
                                    .OEMProvisioningCert
                                    .bytesLen,
                                &mut *((*CertificateInstallationReqType).OEMProvisioningCert.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                1200 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 212 as i32;
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
                            error = decode_din_ListOfRootCertificateIDsType(
                                stream,
                                &mut (*CertificateInstallationReqType).ListOfRootCertificateIDs,
                            );
                            if error == 0 as i32 {
                                grammar_id = 213 as i32;
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
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*CertificateInstallationReqType).DHParams.bytesLen,
                                &mut *((*CertificateInstallationReqType).DHParams.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                256 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_SessionSetupResType(
    stream: &mut ExiBitstream,
    mut SessionSetupResType: *mut din_SessionSetupResType,
) -> i32 {
    let mut grammar_id: i32 = 214 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_SessionSetupResType(SessionSetupResType);
    while done == 0 {
        match grammar_id {
            214 => {
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
                                        5 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*SessionSetupResType).ResponseCode =
                                            value as din_responseCodeType;
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
                                        grammar_id = 215 as i32;
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
            215 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*SessionSetupResType).EVSEID.bytesLen,
                                &mut *((*SessionSetupResType).EVSEID.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                32 as i32 as usize,
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
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_integer64(
                                stream,
                                &mut (*SessionSetupResType).DateTimeNow,
                            );
                            if error == 0 as i32 {
                                (*SessionSetupResType).set_DateTimeNow_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_ServiceDiscoveryReqType(
    stream: &mut ExiBitstream,
    mut ServiceDiscoveryReqType: *mut din_ServiceDiscoveryReqType,
) -> i32 {
    let mut grammar_id: i32 = 217 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_ServiceDiscoveryReqType(ServiceDiscoveryReqType);
    while done == 0 {
        match grammar_id {
            217 => {
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
                                        &mut (*ServiceDiscoveryReqType).ServiceScope.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*ServiceDiscoveryReqType).ServiceScope.charactersLen
                                            as i32
                                            >= 2 as i32
                                        {
                                            (*ServiceDiscoveryReqType).ServiceScope.charactersLen =
                                                ((*ServiceDiscoveryReqType)
                                                    .ServiceScope
                                                    .charactersLen
                                                    as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*ServiceDiscoveryReqType)
                                                    .ServiceScope
                                                    .charactersLen
                                                    as usize,
                                                ((*ServiceDiscoveryReqType)
                                                    .ServiceScope
                                                    .characters)
                                                    .as_mut_ptr(),
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
                                        (*ServiceDiscoveryReqType)
                                            .set_ServiceScope_isUsed(1 as u32);
                                        grammar_id = 218 as i32;
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
                                        2 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*ServiceDiscoveryReqType).ServiceCategory =
                                            value as din_serviceCategoryType;
                                        (*ServiceDiscoveryReqType)
                                            .set_ServiceCategory_isUsed(1 as u32);
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
                                        grammar_id = 3 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
                                    let mut value_0: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        2 as i32 as usize,
                                        &mut value_0,
                                    );
                                    if error == 0 as i32 {
                                        (*ServiceDiscoveryReqType).ServiceCategory =
                                            value_0 as din_serviceCategoryType;
                                        (*ServiceDiscoveryReqType)
                                            .set_ServiceCategory_isUsed(1 as u32);
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
                                        grammar_id = 3 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_ServiceDiscoveryResType(
    stream: &mut ExiBitstream,
    mut ServiceDiscoveryResType: *mut din_ServiceDiscoveryResType,
) -> i32 {
    let mut grammar_id: i32 = 219 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_ServiceDiscoveryResType(ServiceDiscoveryResType);
    while done == 0 {
        match grammar_id {
            219 => {
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
                                        5 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*ServiceDiscoveryResType).ResponseCode =
                                            value as din_responseCodeType;
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
                            error = decode_din_PaymentOptionsType(
                                stream,
                                &mut (*ServiceDiscoveryResType).PaymentOptions,
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
                            error = decode_din_ServiceChargeType(
                                stream,
                                &mut (*ServiceDiscoveryResType).ChargeService,
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
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_ServiceTagListType(
                                stream,
                                &mut (*ServiceDiscoveryResType).ServiceList,
                            );
                            if error == 0 as i32 {
                                (*ServiceDiscoveryResType).set_ServiceList_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_ServiceDetailReqType(
    stream: &mut ExiBitstream,
    mut ServiceDetailReqType: *mut din_ServiceDetailReqType,
) -> i32 {
    let mut grammar_id: i32 = 223 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_ServiceDetailReqType(ServiceDetailReqType);
    while done == 0 {
        match grammar_id {
            223 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*ServiceDetailReqType).ServiceID,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_ServiceDetailResType(
    stream: &mut ExiBitstream,
    mut ServiceDetailResType: *mut din_ServiceDetailResType,
) -> i32 {
    let mut grammar_id: i32 = 224 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_ServiceDetailResType(ServiceDetailResType);
    while done == 0 {
        match grammar_id {
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
                                    let mut value: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*ServiceDetailResType).ResponseCode =
                                            value as din_responseCodeType;
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
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_uint16(
                                stream,
                                &mut (*ServiceDetailResType).ServiceID,
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
                            error = decode_din_ServiceParameterListType(
                                stream,
                                &mut (*ServiceDetailResType).ServiceParameterList,
                            );
                            if error == 0 as i32 {
                                (*ServiceDetailResType).set_ServiceParameterList_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_ServicePaymentSelectionReqType(
    stream: &mut ExiBitstream,
    mut ServicePaymentSelectionReqType: *mut din_ServicePaymentSelectionReqType,
) -> i32 {
    let mut grammar_id: i32 = 227 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_ServicePaymentSelectionReqType(ServicePaymentSelectionReqType);
    while done == 0 {
        match grammar_id {
            227 => {
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
                                        (*ServicePaymentSelectionReqType).SelectedPaymentOption =
                                            value as din_paymentOptionType;
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
                                        grammar_id = 228 as i32;
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
            228 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_SelectedServiceListType(
                                stream,
                                &mut (*ServicePaymentSelectionReqType).SelectedServiceList,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_ServicePaymentSelectionResType(
    stream: &mut ExiBitstream,
    mut ServicePaymentSelectionResType: *mut din_ServicePaymentSelectionResType,
) -> i32 {
    let mut grammar_id: i32 = 229 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_ServicePaymentSelectionResType(ServicePaymentSelectionResType);
    while done == 0 {
        match grammar_id {
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
                                    let mut value: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*ServicePaymentSelectionResType).ResponseCode =
                                            value as din_responseCodeType;
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
                                        grammar_id = 3 as i32;
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
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_PaymentDetailsReqType(
    stream: &mut ExiBitstream,
    mut PaymentDetailsReqType: *mut din_PaymentDetailsReqType,
) -> i32 {
    let mut grammar_id: i32 = 230 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_PaymentDetailsReqType(PaymentDetailsReqType);
    while done == 0 {
        match grammar_id {
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
                                    error = exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut (*PaymentDetailsReqType).ContractID.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*PaymentDetailsReqType).ContractID.charactersLen as i32
                                            >= 2 as i32
                                        {
                                            (*PaymentDetailsReqType).ContractID.charactersLen =
                                                ((*PaymentDetailsReqType).ContractID.charactersLen
                                                    as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*PaymentDetailsReqType).ContractID.charactersLen
                                                    as usize,
                                                ((*PaymentDetailsReqType).ContractID.characters)
                                                    .as_mut_ptr(),
                                                (24 as i32 + 1 as i32) as usize,
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
                            error = decode_din_CertificateChainType(
                                stream,
                                &mut (*PaymentDetailsReqType).ContractSignatureCertChain,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_PaymentDetailsResType(
    stream: &mut ExiBitstream,
    mut PaymentDetailsResType: *mut din_PaymentDetailsResType,
) -> i32 {
    let mut grammar_id: i32 = 232 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_PaymentDetailsResType(PaymentDetailsResType);
    while done == 0 {
        match grammar_id {
            232 => {
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
                                        5 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*PaymentDetailsResType).ResponseCode =
                                            value as din_responseCodeType;
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
                                        grammar_id = 233 as i32;
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
            233 => {
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
                                        &mut (*PaymentDetailsResType).GenChallenge.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*PaymentDetailsResType).GenChallenge.charactersLen
                                            as i32
                                            >= 2 as i32
                                        {
                                            (*PaymentDetailsResType).GenChallenge.charactersLen =
                                                ((*PaymentDetailsResType).GenChallenge.charactersLen
                                                    as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*PaymentDetailsResType).GenChallenge.charactersLen
                                                    as usize,
                                                ((*PaymentDetailsResType).GenChallenge.characters)
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
                                        grammar_id = 234 as i32;
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
            234 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_integer64(
                                stream,
                                &mut (*PaymentDetailsResType).DateTimeNow,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_ContractAuthenticationReqType(
    stream: &mut ExiBitstream,
    mut ContractAuthenticationReqType: *mut din_ContractAuthenticationReqType,
) -> i32 {
    let mut grammar_id: i32 = 235 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_ContractAuthenticationReqType(ContractAuthenticationReqType);
    while done == 0 {
        match grammar_id {
            235 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*ContractAuthenticationReqType).Id.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*ContractAuthenticationReqType).Id.charactersLen as i32
                                    >= 2 as i32
                                {
                                    (*ContractAuthenticationReqType).Id.charactersLen =
                                        ((*ContractAuthenticationReqType).Id.charactersLen as i32
                                            - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*ContractAuthenticationReqType).Id.charactersLen as usize,
                                        ((*ContractAuthenticationReqType).Id.characters)
                                            .as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*ContractAuthenticationReqType).set_Id_isUsed(1 as u32);
                            grammar_id = 236 as i32;
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
                                        &mut (*ContractAuthenticationReqType)
                                            .GenChallenge
                                            .charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*ContractAuthenticationReqType)
                                            .GenChallenge
                                            .charactersLen
                                            as i32
                                            >= 2 as i32
                                        {
                                            (*ContractAuthenticationReqType)
                                                .GenChallenge
                                                .charactersLen = ((*ContractAuthenticationReqType)
                                                .GenChallenge
                                                .charactersLen
                                                as i32
                                                - 2 as i32)
                                                as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*ContractAuthenticationReqType)
                                                    .GenChallenge
                                                    .charactersLen
                                                    as usize,
                                                ((*ContractAuthenticationReqType)
                                                    .GenChallenge
                                                    .characters)
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
                                        (*ContractAuthenticationReqType)
                                            .set_GenChallenge_isUsed(1 as u32);
                                        grammar_id = 3 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    error = exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut (*ContractAuthenticationReqType)
                                            .GenChallenge
                                            .charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*ContractAuthenticationReqType)
                                            .GenChallenge
                                            .charactersLen
                                            as i32
                                            >= 2 as i32
                                        {
                                            (*ContractAuthenticationReqType)
                                                .GenChallenge
                                                .charactersLen = ((*ContractAuthenticationReqType)
                                                .GenChallenge
                                                .charactersLen
                                                as i32
                                                - 2 as i32)
                                                as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*ContractAuthenticationReqType)
                                                    .GenChallenge
                                                    .charactersLen
                                                    as usize,
                                                ((*ContractAuthenticationReqType)
                                                    .GenChallenge
                                                    .characters)
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
                                        (*ContractAuthenticationReqType)
                                            .set_GenChallenge_isUsed(1 as u32);
                                        grammar_id = 3 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_ContractAuthenticationResType(
    stream: &mut ExiBitstream,
    mut ContractAuthenticationResType: *mut din_ContractAuthenticationResType,
) -> i32 {
    let mut grammar_id: i32 = 237 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_ContractAuthenticationResType(ContractAuthenticationResType);
    while done == 0 {
        match grammar_id {
            237 => {
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
                                        5 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*ContractAuthenticationResType).ResponseCode =
                                            value as din_responseCodeType;
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
                                        grammar_id = 238 as i32;
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
                                        (*ContractAuthenticationResType).EVSEProcessing =
                                            value_0 as din_EVSEProcessingType;
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
                                        grammar_id = 3 as i32;
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
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_ChargeParameterDiscoveryReqType(
    stream: &mut ExiBitstream,
    mut ChargeParameterDiscoveryReqType: *mut din_ChargeParameterDiscoveryReqType,
) -> i32 {
    let mut grammar_id: i32 = 239 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_ChargeParameterDiscoveryReqType(ChargeParameterDiscoveryReqType);
    while done == 0 {
        match grammar_id {
            239 => {
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
                                        (*ChargeParameterDiscoveryReqType)
                                            .EVRequestedEnergyTransferType =
                                            value as din_EVRequestedEnergyTransferType;
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
                                        grammar_id = 240 as i32;
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
            240 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_AC_EVChargeParameterType(
                                stream,
                                &mut (*ChargeParameterDiscoveryReqType).AC_EVChargeParameter,
                            );
                            if error == 0 as i32 {
                                (*ChargeParameterDiscoveryReqType)
                                    .set_AC_EVChargeParameter_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            error = decode_din_DC_EVChargeParameterType(
                                stream,
                                &mut (*ChargeParameterDiscoveryReqType).DC_EVChargeParameter,
                            );
                            if error == 0 as i32 {
                                (*ChargeParameterDiscoveryReqType)
                                    .set_DC_EVChargeParameter_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            error = decode_din_EVChargeParameterType(
                                stream,
                                &mut (*ChargeParameterDiscoveryReqType).EVChargeParameter,
                            );
                            if error == 0 as i32 {
                                (*ChargeParameterDiscoveryReqType)
                                    .set_EVChargeParameter_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_ChargeParameterDiscoveryResType(
    stream: &mut ExiBitstream,
    mut ChargeParameterDiscoveryResType: *mut din_ChargeParameterDiscoveryResType,
) -> i32 {
    let mut grammar_id: i32 = 241 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_ChargeParameterDiscoveryResType(ChargeParameterDiscoveryResType);
    while done == 0 {
        match grammar_id {
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
                                    let mut value: u32 = 0;
                                    error = exi_basetypes_decoder_nbit_uint(
                                        stream,
                                        5 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*ChargeParameterDiscoveryResType).ResponseCode =
                                            value as din_responseCodeType;
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
                                        (*ChargeParameterDiscoveryResType).EVSEProcessing =
                                            value_0 as din_EVSEProcessingType;
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
                                        grammar_id = 243 as i32;
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
            243 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_SAScheduleListType(
                                stream,
                                &mut (*ChargeParameterDiscoveryResType).SAScheduleList,
                            );
                            if error == 0 as i32 {
                                (*ChargeParameterDiscoveryResType)
                                    .set_SAScheduleList_isUsed(1 as u32);
                                grammar_id = 244 as i32;
                            }
                        }
                        1 => {
                            error = decode_din_SASchedulesType(
                                stream,
                                &mut (*ChargeParameterDiscoveryResType).SASchedules,
                            );
                            if error == 0 as i32 {
                                (*ChargeParameterDiscoveryResType).set_SASchedules_isUsed(1 as u32);
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
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_AC_EVSEChargeParameterType(
                                stream,
                                &mut (*ChargeParameterDiscoveryResType).AC_EVSEChargeParameter,
                            );
                            if error == 0 as i32 {
                                (*ChargeParameterDiscoveryResType)
                                    .set_AC_EVSEChargeParameter_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            error = decode_din_DC_EVSEChargeParameterType(
                                stream,
                                &mut (*ChargeParameterDiscoveryResType).DC_EVSEChargeParameter,
                            );
                            if error == 0 as i32 {
                                (*ChargeParameterDiscoveryResType)
                                    .set_DC_EVSEChargeParameter_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            error = decode_din_EVSEChargeParameterType(
                                stream,
                                &mut (*ChargeParameterDiscoveryResType).EVSEChargeParameter,
                            );
                            if error == 0 as i32 {
                                (*ChargeParameterDiscoveryResType)
                                    .set_EVSEChargeParameter_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_PowerDeliveryReqType(
    stream: &mut ExiBitstream,
    mut PowerDeliveryReqType: *mut din_PowerDeliveryReqType,
) -> i32 {
    let mut grammar_id: i32 = 245 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_PowerDeliveryReqType(PowerDeliveryReqType);
    while done == 0 {
        match grammar_id {
            245 => {
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
                                        (*PowerDeliveryReqType).ReadyToChargeState = value as i32;
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
                                        grammar_id = 246 as i32;
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
            246 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 3 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_ChargingProfileType(
                                stream,
                                &mut (*PowerDeliveryReqType).ChargingProfile,
                            );
                            if error == 0 as i32 {
                                (*PowerDeliveryReqType).set_ChargingProfile_isUsed(1 as u32);
                                grammar_id = 247 as i32;
                            }
                        }
                        1 => {
                            error = decode_din_DC_EVPowerDeliveryParameterType(
                                stream,
                                &mut (*PowerDeliveryReqType).DC_EVPowerDeliveryParameter,
                            );
                            if error == 0 as i32 {
                                (*PowerDeliveryReqType)
                                    .set_DC_EVPowerDeliveryParameter_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            error = decode_din_EVPowerDeliveryParameterType(
                                stream,
                                &mut (*PowerDeliveryReqType).EVPowerDeliveryParameter,
                            );
                            if error == 0 as i32 {
                                (*PowerDeliveryReqType)
                                    .set_EVPowerDeliveryParameter_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        3 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
                            error = decode_din_DC_EVPowerDeliveryParameterType(
                                stream,
                                &mut (*PowerDeliveryReqType).DC_EVPowerDeliveryParameter,
                            );
                            if error == 0 as i32 {
                                (*PowerDeliveryReqType)
                                    .set_DC_EVPowerDeliveryParameter_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            error = decode_din_EVPowerDeliveryParameterType(
                                stream,
                                &mut (*PowerDeliveryReqType).EVPowerDeliveryParameter,
                            );
                            if error == 0 as i32 {
                                (*PowerDeliveryReqType)
                                    .set_EVPowerDeliveryParameter_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_PowerDeliveryResType(
    stream: &mut ExiBitstream,
    mut PowerDeliveryResType: *mut din_PowerDeliveryResType,
) -> i32 {
    let mut grammar_id: i32 = 248 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_PowerDeliveryResType(PowerDeliveryResType);
    while done == 0 {
        match grammar_id {
            248 => {
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
                                        5 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*PowerDeliveryResType).ResponseCode =
                                            value as din_responseCodeType;
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
                            error = decode_din_AC_EVSEStatusType(
                                stream,
                                &mut (*PowerDeliveryResType).AC_EVSEStatus,
                            );
                            if error == 0 as i32 {
                                (*PowerDeliveryResType).set_AC_EVSEStatus_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            error = decode_din_DC_EVSEStatusType(
                                stream,
                                &mut (*PowerDeliveryResType).DC_EVSEStatus,
                            );
                            if error == 0 as i32 {
                                (*PowerDeliveryResType).set_DC_EVSEStatus_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            error = decode_din_EVSEStatusType(
                                stream,
                                &mut (*PowerDeliveryResType).EVSEStatus,
                            );
                            if error == 0 as i32 {
                                (*PowerDeliveryResType).set_EVSEStatus_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_ChargingStatusReqType(
    stream: &mut ExiBitstream,
    mut ChargingStatusReqType: *mut din_ChargingStatusReqType,
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
unsafe extern "C" fn decode_din_ChargingStatusResType(
    stream: &mut ExiBitstream,
    mut ChargingStatusResType: *mut din_ChargingStatusResType,
) -> i32 {
    let mut grammar_id: i32 = 250 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_ChargingStatusResType(ChargingStatusResType);
    while done == 0 {
        match grammar_id {
            250 => {
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
                                        5 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*ChargingStatusResType).ResponseCode =
                                            value as din_responseCodeType;
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
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*ChargingStatusResType).EVSEID.bytesLen,
                                &mut *((*ChargingStatusResType).EVSEID.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                32 as i32 as usize,
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
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*ChargingStatusResType).SAScheduleTupleID,
                            );
                            if error == 0 as i32 {
                                grammar_id = 253 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            253 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_PhysicalValueType(
                                stream,
                                &mut (*ChargingStatusResType).EVSEMaxCurrent,
                            );
                            if error == 0 as i32 {
                                (*ChargingStatusResType).set_EVSEMaxCurrent_isUsed(1 as u32);
                                grammar_id = 254 as i32;
                            }
                        }
                        1 => {
                            error = decode_din_MeterInfoType(
                                stream,
                                &mut (*ChargingStatusResType).MeterInfo,
                            );
                            if error == 0 as i32 {
                                (*ChargingStatusResType).set_MeterInfo_isUsed(1 as u32);
                                grammar_id = 255 as i32;
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
                                        1 as i32 as usize,
                                        &mut value_0,
                                    );
                                    if error == 0 as i32 {
                                        (*ChargingStatusResType).ReceiptRequired = value_0 as i32;
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
                                        grammar_id = 256 as i32;
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
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_MeterInfoType(
                                stream,
                                &mut (*ChargingStatusResType).MeterInfo,
                            );
                            if error == 0 as i32 {
                                (*ChargingStatusResType).set_MeterInfo_isUsed(1 as u32);
                                grammar_id = 255 as i32;
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
                                        (*ChargingStatusResType).ReceiptRequired = value_1 as i32;
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
                                        grammar_id = 256 as i32;
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
                                        (*ChargingStatusResType).ReceiptRequired = value_2 as i32;
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
                                        grammar_id = 256 as i32;
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
            256 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_AC_EVSEStatusType(
                                stream,
                                &mut (*ChargingStatusResType).AC_EVSEStatus,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_MeteringReceiptReqType(
    stream: &mut ExiBitstream,
    mut MeteringReceiptReqType: *mut din_MeteringReceiptReqType,
) -> i32 {
    let mut grammar_id: i32 = 257 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_MeteringReceiptReqType(MeteringReceiptReqType);
    while done == 0 {
        match grammar_id {
            257 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*MeteringReceiptReqType).Id.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*MeteringReceiptReqType).Id.charactersLen as i32 >= 2 as i32 {
                                    (*MeteringReceiptReqType).Id.charactersLen =
                                        ((*MeteringReceiptReqType).Id.charactersLen as i32
                                            - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*MeteringReceiptReqType).Id.charactersLen as usize,
                                        ((*MeteringReceiptReqType).Id.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*MeteringReceiptReqType).set_Id_isUsed(1 as u32);
                            grammar_id = 258 as i32;
                        }
                        1 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*MeteringReceiptReqType).SessionID.bytesLen,
                                &mut *((*MeteringReceiptReqType).SessionID.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                8 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 259 as i32;
                            }
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
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*MeteringReceiptReqType).SessionID.bytesLen,
                                &mut *((*MeteringReceiptReqType).SessionID.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                8 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 259 as i32;
                            }
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
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*MeteringReceiptReqType).SAScheduleTupleID,
                            );
                            if error == 0 as i32 {
                                (*MeteringReceiptReqType).set_SAScheduleTupleID_isUsed(1 as u32);
                                grammar_id = 260 as i32;
                            }
                        }
                        1 => {
                            error = decode_din_MeterInfoType(
                                stream,
                                &mut (*MeteringReceiptReqType).MeterInfo,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            260 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_MeterInfoType(
                                stream,
                                &mut (*MeteringReceiptReqType).MeterInfo,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_MeteringReceiptResType(
    stream: &mut ExiBitstream,
    mut MeteringReceiptResType: *mut din_MeteringReceiptResType,
) -> i32 {
    let mut grammar_id: i32 = 261 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_MeteringReceiptResType(MeteringReceiptResType);
    while done == 0 {
        match grammar_id {
            261 => {
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
                                        5 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*MeteringReceiptResType).ResponseCode =
                                            value as din_responseCodeType;
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
                                        grammar_id = 262 as i32;
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
            262 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_AC_EVSEStatusType(
                                stream,
                                &mut (*MeteringReceiptResType).AC_EVSEStatus,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_SessionStopType(
    stream: &mut ExiBitstream,
    mut SessionStopType: *mut din_SessionStopType,
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
unsafe extern "C" fn decode_din_SessionStopResType(
    stream: &mut ExiBitstream,
    mut SessionStopResType: *mut din_SessionStopResType,
) -> i32 {
    let mut grammar_id: i32 = 263 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_SessionStopResType(SessionStopResType);
    while done == 0 {
        match grammar_id {
            263 => {
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
                                        5 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*SessionStopResType).ResponseCode =
                                            value as din_responseCodeType;
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
                                        grammar_id = 3 as i32;
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
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_CertificateUpdateReqType(
    stream: &mut ExiBitstream,
    mut CertificateUpdateReqType: *mut din_CertificateUpdateReqType,
) -> i32 {
    let mut grammar_id: i32 = 264 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_CertificateUpdateReqType(CertificateUpdateReqType);
    while done == 0 {
        match grammar_id {
            264 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*CertificateUpdateReqType).Id.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*CertificateUpdateReqType).Id.charactersLen as i32 >= 2 as i32 {
                                    (*CertificateUpdateReqType).Id.charactersLen =
                                        ((*CertificateUpdateReqType).Id.charactersLen as i32
                                            - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*CertificateUpdateReqType).Id.charactersLen as usize,
                                        ((*CertificateUpdateReqType).Id.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            (*CertificateUpdateReqType).set_Id_isUsed(1 as u32);
                            grammar_id = 265 as i32;
                        }
                        1 => {
                            error = decode_din_CertificateChainType(
                                stream,
                                &mut (*CertificateUpdateReqType).ContractSignatureCertChain,
                            );
                            if error == 0 as i32 {
                                grammar_id = 266 as i32;
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
                            error = decode_din_CertificateChainType(
                                stream,
                                &mut (*CertificateUpdateReqType).ContractSignatureCertChain,
                            );
                            if error == 0 as i32 {
                                grammar_id = 266 as i32;
                            }
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
                            error = exi_basetypes_decoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                &mut eventCode,
                            );
                            if error == 0 as i32 {
                                if eventCode == 0 as i32 as u32 {
                                    error = exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut (*CertificateUpdateReqType).ContractID.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*CertificateUpdateReqType).ContractID.charactersLen
                                            as i32
                                            >= 2 as i32
                                        {
                                            (*CertificateUpdateReqType).ContractID.charactersLen =
                                                ((*CertificateUpdateReqType)
                                                    .ContractID
                                                    .charactersLen
                                                    as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*CertificateUpdateReqType).ContractID.charactersLen
                                                    as usize,
                                                ((*CertificateUpdateReqType).ContractID.characters)
                                                    .as_mut_ptr(),
                                                (24 as i32 + 1 as i32) as usize,
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
                                        grammar_id = 267 as i32;
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
            267 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_ListOfRootCertificateIDsType(
                                stream,
                                &mut (*CertificateUpdateReqType).ListOfRootCertificateIDs,
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
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*CertificateUpdateReqType).DHParams.bytesLen,
                                &mut *((*CertificateUpdateReqType).DHParams.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                256 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_CertificateUpdateResType(
    stream: &mut ExiBitstream,
    mut CertificateUpdateResType: *mut din_CertificateUpdateResType,
) -> i32 {
    let mut grammar_id: i32 = 269 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_CertificateUpdateResType(CertificateUpdateResType);
    while done == 0 {
        match grammar_id {
            269 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = exi_basetypes_decoder_uint_16(
                                stream,
                                &mut (*CertificateUpdateResType).Id.charactersLen,
                            );
                            if error == 0 as i32 {
                                if (*CertificateUpdateResType).Id.charactersLen as i32 >= 2 as i32 {
                                    (*CertificateUpdateResType).Id.charactersLen =
                                        ((*CertificateUpdateResType).Id.charactersLen as i32
                                            - 2 as i32)
                                            as u16;
                                    error = exi_basetypes_decoder_characters(
                                        stream,
                                        (*CertificateUpdateResType).Id.charactersLen as usize,
                                        ((*CertificateUpdateResType).Id.characters).as_mut_ptr(),
                                        (64 as i32 + 1 as i32) as usize,
                                    );
                                } else {
                                    error = -(200 as i32);
                                }
                            }
                            grammar_id = 270 as i32;
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
                                        5 as i32 as usize,
                                        &mut value,
                                    );
                                    if error == 0 as i32 {
                                        (*CertificateUpdateResType).ResponseCode =
                                            value as din_responseCodeType;
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
                                        grammar_id = 271 as i32;
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
                            error = decode_din_CertificateChainType(
                                stream,
                                &mut (*CertificateUpdateResType).ContractSignatureCertChain,
                            );
                            if error == 0 as i32 {
                                grammar_id = 272 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            272 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*CertificateUpdateResType)
                                    .ContractSignatureEncryptedPrivateKey
                                    .bytesLen,
                                &mut *((*CertificateUpdateResType)
                                    .ContractSignatureEncryptedPrivateKey
                                    .bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                128 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 273 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            273 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_hex_binary(
                                stream,
                                &mut (*CertificateUpdateResType).DHParams.bytesLen,
                                &mut *((*CertificateUpdateResType).DHParams.bytes)
                                    .as_mut_ptr()
                                    .offset(0 as i32 as isize),
                                256 as i32 as usize,
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
                                    error = exi_basetypes_decoder_uint_16(
                                        stream,
                                        &mut (*CertificateUpdateResType).ContractID.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*CertificateUpdateResType).ContractID.charactersLen
                                            as i32
                                            >= 2 as i32
                                        {
                                            (*CertificateUpdateResType).ContractID.charactersLen =
                                                ((*CertificateUpdateResType)
                                                    .ContractID
                                                    .charactersLen
                                                    as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*CertificateUpdateResType).ContractID.charactersLen
                                                    as usize,
                                                ((*CertificateUpdateResType).ContractID.characters)
                                                    .as_mut_ptr(),
                                                (24 as i32 + 1 as i32) as usize,
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
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_exi_type_integer16(
                                stream,
                                &mut (*CertificateUpdateResType).RetryCounter,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_BodyBaseType(
    stream: &mut ExiBitstream,
    mut BodyBaseType: *mut din_BodyBaseType,
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
unsafe extern "C" fn decode_din_NotificationType(
    stream: &mut ExiBitstream,
    mut NotificationType: *mut din_NotificationType,
) -> i32 {
    let mut grammar_id: i32 = 276 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_NotificationType(NotificationType);
    while done == 0 {
        match grammar_id {
            276 => {
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
                                        (*NotificationType).FaultCode = value as din_faultCodeType;
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
                                        grammar_id = 277 as i32;
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
            277 => {
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
                                        &mut (*NotificationType).FaultMsg.charactersLen,
                                    );
                                    if error == 0 as i32 {
                                        if (*NotificationType).FaultMsg.charactersLen as i32
                                            >= 2 as i32
                                        {
                                            (*NotificationType).FaultMsg.charactersLen =
                                                ((*NotificationType).FaultMsg.charactersLen as i32
                                                    - 2 as i32)
                                                    as u16;
                                            error = exi_basetypes_decoder_characters(
                                                stream,
                                                (*NotificationType).FaultMsg.charactersLen as usize,
                                                ((*NotificationType).FaultMsg.characters)
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
                                        (*NotificationType).set_FaultMsg_isUsed(1 as u32);
                                        grammar_id = 3 as i32;
                                    } else {
                                        error = -(170 as i32);
                                    }
                                }
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_SignatureType(
    stream: &mut ExiBitstream,
    mut SignatureType: *mut din_SignatureType,
) -> i32 {
    let mut grammar_id: i32 = 278 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_SignatureType(SignatureType);
    while done == 0 {
        match grammar_id {
            278 => {
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
                            grammar_id = 279 as i32;
                        }
                        1 => {
                            error =
                                decode_din_SignedInfoType(stream, &mut (*SignatureType).SignedInfo);
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
            279 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error =
                                decode_din_SignedInfoType(stream, &mut (*SignatureType).SignedInfo);
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
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_SignatureValueType(
                                stream,
                                &mut (*SignatureType).SignatureValue,
                            );
                            if error == 0 as i32 {
                                grammar_id = 281 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            281 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_KeyInfoType(stream, &mut (*SignatureType).KeyInfo);
                            if error == 0 as i32 {
                                (*SignatureType).set_KeyInfo_isUsed(1 as u32);
                                grammar_id = 283 as i32;
                            }
                        }
                        1 => {
                            error = decode_din_ObjectType(stream, &mut (*SignatureType).Object);
                            if error == 0 as i32 {
                                (*SignatureType).set_Object_isUsed(1 as u32);
                                grammar_id = 282 as i32;
                            }
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            282 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = -(110 as i32);
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            283 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_ObjectType(stream, &mut (*SignatureType).Object);
                            if error == 0 as i32 {
                                (*SignatureType).set_Object_isUsed(1 as u32);
                                grammar_id = 284 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            284 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = -(110 as i32);
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_MessageHeaderType(
    stream: &mut ExiBitstream,
    mut MessageHeaderType: *mut din_MessageHeaderType,
) -> i32 {
    let mut grammar_id: i32 = 285 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_MessageHeaderType(MessageHeaderType);
    while done == 0 {
        match grammar_id {
            285 => {
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
                                grammar_id = 286 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            286 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_NotificationType(
                                stream,
                                &mut (*MessageHeaderType).Notification,
                            );
                            if error == 0 as i32 {
                                (*MessageHeaderType).set_Notification_isUsed(1 as u32);
                                grammar_id = 287 as i32;
                            }
                        }
                        1 => {
                            error = decode_din_SignatureType(
                                stream,
                                &mut (*MessageHeaderType).Signature,
                            );
                            if error == 0 as i32 {
                                (*MessageHeaderType).set_Signature_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            287 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 2 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_SignatureType(
                                stream,
                                &mut (*MessageHeaderType).Signature,
                            );
                            if error == 0 as i32 {
                                (*MessageHeaderType).set_Signature_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_BodyType(
    stream: &mut ExiBitstream,
    mut BodyType: *mut din_BodyType,
) -> i32 {
    let mut grammar_id: i32 = 288 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_BodyType(BodyType);
    while done == 0 {
        match grammar_id {
            288 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 6 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error = decode_din_BodyBaseType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.BodyElement,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_BodyElement_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        1 => {
                            error = decode_din_CableCheckReqType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.CableCheckReq,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_CableCheckReq_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        2 => {
                            error = decode_din_CableCheckResType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.CableCheckRes,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_CableCheckRes_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        3 => {
                            error = decode_din_CertificateInstallationReqType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.CertificateInstallationReq,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_CertificateInstallationReq_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        4 => {
                            error = decode_din_CertificateInstallationResType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.CertificateInstallationRes,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_CertificateInstallationRes_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        5 => {
                            error = decode_din_CertificateUpdateReqType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.CertificateUpdateReq,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_CertificateUpdateReq_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        6 => {
                            error = decode_din_CertificateUpdateResType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.CertificateUpdateRes,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_CertificateUpdateRes_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        7 => {
                            error = decode_din_ChargeParameterDiscoveryReqType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.ChargeParameterDiscoveryReq,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_ChargeParameterDiscoveryReq_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        8 => {
                            error = decode_din_ChargeParameterDiscoveryResType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.ChargeParameterDiscoveryRes,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_ChargeParameterDiscoveryRes_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        9 => {
                            error = decode_din_ChargingStatusReqType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.ChargingStatusReq,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_ChargingStatusReq_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        10 => {
                            error = decode_din_ChargingStatusResType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.ChargingStatusRes,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_ChargingStatusRes_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        11 => {
                            error = decode_din_ContractAuthenticationReqType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.ContractAuthenticationReq,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_ContractAuthenticationReq_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        12 => {
                            error = decode_din_ContractAuthenticationResType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.ContractAuthenticationRes,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_ContractAuthenticationRes_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        13 => {
                            error = decode_din_CurrentDemandReqType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.CurrentDemandReq,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_CurrentDemandReq_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        14 => {
                            error = decode_din_CurrentDemandResType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.CurrentDemandRes,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_CurrentDemandRes_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        15 => {
                            error = decode_din_MeteringReceiptReqType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.MeteringReceiptReq,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_MeteringReceiptReq_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        16 => {
                            error = decode_din_MeteringReceiptResType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.MeteringReceiptRes,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_MeteringReceiptRes_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        17 => {
                            error = decode_din_PaymentDetailsReqType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.PaymentDetailsReq,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_PaymentDetailsReq_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        18 => {
                            error = decode_din_PaymentDetailsResType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.PaymentDetailsRes,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_PaymentDetailsRes_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        19 => {
                            error = decode_din_PowerDeliveryReqType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.PowerDeliveryReq,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_PowerDeliveryReq_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        20 => {
                            error = decode_din_PowerDeliveryResType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.PowerDeliveryRes,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_PowerDeliveryRes_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        21 => {
                            error = decode_din_PreChargeReqType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.PreChargeReq,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_PreChargeReq_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        22 => {
                            error = decode_din_PreChargeResType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.PreChargeRes,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_PreChargeRes_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        23 => {
                            error = decode_din_ServiceDetailReqType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.ServiceDetailReq,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_ServiceDetailReq_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        24 => {
                            error = decode_din_ServiceDetailResType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.ServiceDetailRes,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_ServiceDetailRes_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        25 => {
                            error = decode_din_ServiceDiscoveryReqType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.ServiceDiscoveryReq,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_ServiceDiscoveryReq_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        26 => {
                            error = decode_din_ServiceDiscoveryResType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.ServiceDiscoveryRes,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_ServiceDiscoveryRes_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        27 => {
                            error = decode_din_ServicePaymentSelectionReqType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.ServicePaymentSelectionReq,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_ServicePaymentSelectionReq_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        28 => {
                            error = decode_din_ServicePaymentSelectionResType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.ServicePaymentSelectionRes,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_ServicePaymentSelectionRes_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        29 => {
                            error = decode_din_SessionSetupReqType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.SessionSetupReq,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_SessionSetupReq_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        30 => {
                            error = decode_din_SessionSetupResType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.SessionSetupRes,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_SessionSetupRes_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        31 => {
                            error = decode_din_SessionStopType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.SessionStopReq,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_SessionStopReq_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        32 => {
                            error = decode_din_SessionStopResType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.SessionStopRes,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_SessionStopRes_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        33 => {
                            error = decode_din_WeldingDetectionReqType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.WeldingDetectionReq,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_WeldingDetectionReq_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        34 => {
                            error = decode_din_WeldingDetectionResType(
                                stream,
                                &mut (*BodyType).c2rust_unnamed.WeldingDetectionRes,
                            );
                            if error == 0 as i32 {
                                (*BodyType).set_WeldingDetectionRes_isUsed(1 as u32);
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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
unsafe extern "C" fn decode_din_V2G_Message(
    stream: &mut ExiBitstream,
    mut V2G_Message: *mut din_V2G_Message,
) -> i32 {
    let mut grammar_id: i32 = 289 as i32;
    let mut done: i32 = 0 as i32;
    let mut eventCode: u32 = 0;
    let mut error: i32 = 0;
    init_din_V2G_Message(V2G_Message);
    while done == 0 {
        match grammar_id {
            289 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            error =
                                decode_din_MessageHeaderType(stream, &mut (*V2G_Message).Header);
                            if error == 0 as i32 {
                                grammar_id = 290 as i32;
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
                            error = decode_din_BodyType(stream, &mut (*V2G_Message).Body);
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                        _ => {
                            error = -(150 as i32);
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_decoder_nbit_uint(stream, 1 as i32 as usize, &mut eventCode);
                if error == 0 as i32 {
                    match eventCode {
                        0 => {
                            done = 1 as i32;
                            grammar_id = 4 as i32;
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

pub unsafe extern "C" fn decode_din_exiDocument(
    stream: &mut ExiBitstream,
    mut exiDoc: *mut din_exiDocument,
) -> i32 {
    let mut eventCode: u32 = 0;
    let mut error: i32 = exi_header_read_and_check(stream);
    if error == 0 as i32 {
        init_din_exiDocument(exiDoc);
        error = exi_basetypes_decoder_nbit_uint(stream, 7 as i32 as usize, &mut eventCode);
        if error == 0 as i32 {
            match eventCode {
                0 | 77 => {
                    error = decode_din_V2G_Message(stream, &mut (*exiDoc).V2G_Message);
                }
                _ => {
                    error = -(151 as i32);
                }
            }
        }
    }
    return error;
}
