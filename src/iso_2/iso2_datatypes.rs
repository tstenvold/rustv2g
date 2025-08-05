use heapless::Vec;

use crate::common::exi_basetypes::ExiSigned;

pub type Iso2CostKindType = u32;
pub const ISO2_COST_KIND_TYPE_CARBON_DIOXIDE_EMISSION: Iso2CostKindType = 2;
pub const ISO2_COST_KIND_TYPE_RENEWABLE_GENERATION_PERCENTAGE: Iso2CostKindType = 1;
pub const ISO2_COST_KIND_TYPE_RELATIVE_PRICE_PERCENTAGE: Iso2CostKindType = 0;
pub const ISO2_COST_KIND_TYPE_UNKNOWN: Iso2CostKindType = 255;

pub type Iso2UnitSymbolType = u32;
pub const ISO2_UNIT_SYMBOL_TYPE_WH: Iso2UnitSymbolType = 6;
pub const ISO2_UNIT_SYMBOL_TYPE_W: Iso2UnitSymbolType = 5;
pub const ISO2_UNIT_SYMBOL_TYPE_V: Iso2UnitSymbolType = 4;
pub const ISO2_UNIT_SYMBOL_TYPE_A: Iso2UnitSymbolType = 3;
pub const ISO2_UNIT_SYMBOL_TYPE_S: Iso2UnitSymbolType = 2;
pub const ISO2_UNIT_SYMBOL_TYPE_M: Iso2UnitSymbolType = 1;
pub const ISO2_UNIT_SYMBOL_TYPE_H: Iso2UnitSymbolType = 0;
pub const ISO2_UNIT_SYMBOL_TYPE_UNKNOWN: Iso2UnitSymbolType = 255;

pub type Iso2DcEvErrorCodeType = u32;
pub const ISO2_DC_EVERROR_CODE_TYPE_NO_DATA: Iso2DcEvErrorCodeType = 11;
pub const ISO2_DC_EVERROR_CODE_TYPE_FAILED_CHARGING_SYSTEM_INCOMPATIBILITY: Iso2DcEvErrorCodeType =
    10;
pub const ISO2_DC_EVERROR_CODE_TYPE_RESERVED_C: Iso2DcEvErrorCodeType = 9;
pub const ISO2_DC_EVERROR_CODE_TYPE_RESERVED_B: Iso2DcEvErrorCodeType = 8;
pub const ISO2_DC_EVERROR_CODE_TYPE_RESERVED_A: Iso2DcEvErrorCodeType = 7;
pub const ISO2_DC_EVERROR_CODE_TYPE_FAILED_CHARGING_VOLTAGE_OUT_OF_RANGE: Iso2DcEvErrorCodeType = 6;
pub const ISO2_DC_EVERROR_CODE_TYPE_FAILED_CHARGING_CURRENTDIFFERENTIAL: Iso2DcEvErrorCodeType = 5;
pub const ISO2_DC_EVERROR_CODE_TYPE_FAILED_EVRESSMALFUNCTION: Iso2DcEvErrorCodeType = 4;
pub const ISO2_DC_EVERROR_CODE_TYPE_FAILED_CHARGER_CONNECTOR_LOCK_FAULT: Iso2DcEvErrorCodeType = 3;
pub const ISO2_DC_EVERROR_CODE_TYPE_FAILED_EVSHIFT_POSITION: Iso2DcEvErrorCodeType = 2;
pub const ISO2_DC_EVERROR_CODE_TYPE_FAILED_RESSTEMPERATURE_INHIBIT: Iso2DcEvErrorCodeType = 1;
pub const ISO2_DC_EVERROR_CODE_TYPE_NO_ERROR: Iso2DcEvErrorCodeType = 0;
pub const ISO2_DC_EVERROR_CODE_TYPE_UNKNOWN: Iso2DcEvErrorCodeType = 255;

pub type Iso2FaultCodeType = u32;
pub const ISO2_FAULT_CODE_TYPE_UNKNOWN_ERROR: Iso2FaultCodeType = 2;
pub const ISO2_FAULT_CODE_TYPE_NO_TLSROOT_CERTIFICAT_AVAILABLE: Iso2FaultCodeType = 1;
pub const ISO2_FAULT_CODE_TYPE_PARSING_ERROR: Iso2FaultCodeType = 0;
pub const ISO2_FAULT_CODE_TYPE_UNKNOWN: Iso2FaultCodeType = 255;

pub type Iso2EvseNotificationType = u32;
pub const ISO2_EVSENOTIFICATION_TYPE_RE_NEGOTIATION: Iso2EvseNotificationType = 2;
pub const ISO2_EVSENOTIFICATION_TYPE_STOP_CHARGING: Iso2EvseNotificationType = 1;
pub const ISO2_EVSENOTIFICATION_TYPE_NONE: Iso2EvseNotificationType = 0;
pub const ISO2_EVSENOTIFICATION_TYPE_UNKNOWN: Iso2EvseNotificationType = 255;

pub type Iso2IsolationLevelType = u32;
pub const ISO2_ISOLATION_LEVEL_TYPE_NO_IMD: Iso2IsolationLevelType = 4;
pub const ISO2_ISOLATION_LEVEL_TYPE_FAULT: Iso2IsolationLevelType = 3;
pub const ISO2_ISOLATION_LEVEL_TYPE_WARNING: Iso2IsolationLevelType = 2;
pub const ISO2_ISOLATION_LEVEL_TYPE_VALID: Iso2IsolationLevelType = 1;
pub const ISO2_ISOLATION_LEVEL_TYPE_INVALID: Iso2IsolationLevelType = 0;
pub const ISO2_ISOLATION_LEVEL_TYPE_UNKNOWN: Iso2IsolationLevelType = 255;

pub type Iso2ServiceCategoryType = u32;
pub const ISO2_SERVICE_CATEGORY_TYPE_OTHER_CUSTOM: Iso2ServiceCategoryType = 3;
pub const ISO2_SERVICE_CATEGORY_TYPE_CONTRACT_CERTIFICATE: Iso2ServiceCategoryType = 2;
pub const ISO2_SERVICE_CATEGORY_TYPE_INTERNET: Iso2ServiceCategoryType = 1;
pub const ISO2_SERVICE_CATEGORY_TYPE_EVCHARGING: Iso2ServiceCategoryType = 0;
pub const ISO2_SERVICE_CATEGORY_TYPE_UNKNOWN: Iso2ServiceCategoryType = 255;

pub type Iso2DcEvseStatusCodeType = u32;
pub const ISO2_DC_EVSESTATUS_CODE_TYPE_RESERVED_C: Iso2DcEvseStatusCodeType = 11;
pub const ISO2_DC_EVSESTATUS_CODE_TYPE_RESERVED_B: Iso2DcEvseStatusCodeType = 10;
pub const ISO2_DC_EVSESTATUS_CODE_TYPE_RESERVED_A: Iso2DcEvseStatusCodeType = 9;
pub const ISO2_DC_EVSESTATUS_CODE_TYPE_RESERVED_9: Iso2DcEvseStatusCodeType = 8;
pub const ISO2_DC_EVSESTATUS_CODE_TYPE_RESERVED_8: Iso2DcEvseStatusCodeType = 7;
pub const ISO2_DC_EVSESTATUS_CODE_TYPE_EVSE_MALFUNCTION: Iso2DcEvseStatusCodeType = 6;
pub const ISO2_DC_EVSESTATUS_CODE_TYPE_EVSE_EMERGENCY_SHUTDOWN: Iso2DcEvseStatusCodeType = 5;
pub const ISO2_DC_EVSESTATUS_CODE_TYPE_EVSE_ISOLATION_MONITORING_ACTIVE: Iso2DcEvseStatusCodeType =
    4;
pub const ISO2_DC_EVSESTATUS_CODE_TYPE_EVSE_UTILITY_INTERRUPT_EVENT: Iso2DcEvseStatusCodeType = 3;
pub const ISO2_DC_EVSESTATUS_CODE_TYPE_EVSE_SHUTDOWN: Iso2DcEvseStatusCodeType = 2;
pub const ISO2_DC_EVSESTATUS_CODE_TYPE_EVSE_READY: Iso2DcEvseStatusCodeType = 1;
pub const ISO2_DC_EVSESTATUS_CODE_TYPE_EVSE_NOT_READY: Iso2DcEvseStatusCodeType = 0;
pub const ISO2_DC_EVSESTATUS_CODE_TYPE_UNKNOWN: Iso2DcEvseStatusCodeType = 255;

pub type Iso2ChargeProgressType = u32;
pub const ISO2_CHARGE_PROGRESS_TYPE_RENEGOTIATE: Iso2ChargeProgressType = 2;
pub const ISO2_CHARGE_PROGRESS_TYPE_STOP: Iso2ChargeProgressType = 1;
pub const ISO2_CHARGE_PROGRESS_TYPE_START: Iso2ChargeProgressType = 0;
pub const ISO2_CHARGE_PROGRESS_TYPE_UNKNOWN: Iso2ChargeProgressType = 255;

pub type Iso2ResponseCodeType = u32;
pub const ISO2_RESPONSE_CODE_TYPE_FAILED_CERTIFICATE_REVOKED: Iso2ResponseCodeType = 25;
pub const ISO2_RESPONSE_CODE_TYPE_FAILED_CERTIFICATE_NOT_ALLOWED_AT_THIS_EVSE:
    Iso2ResponseCodeType = 24;
pub const ISO2_RESPONSE_CODE_TYPE_FAILED_CONTACTOR_ERROR: Iso2ResponseCodeType = 23;
pub const ISO2_RESPONSE_CODE_TYPE_FAILED_WRONG_ENERGY_TRANSFER_MODE: Iso2ResponseCodeType = 22;
pub const ISO2_RESPONSE_CODE_TYPE_FAILED_NO_CHARGE_SERVICE_SELECTED: Iso2ResponseCodeType = 21;
pub const ISO2_RESPONSE_CODE_TYPE_FAILED_METERING_SIGNATURE_NOT_VALID: Iso2ResponseCodeType = 20;
pub const ISO2_RESPONSE_CODE_TYPE_FAILED_CHARGING_PROFILE_INVALID: Iso2ResponseCodeType = 19;
pub const ISO2_RESPONSE_CODE_TYPE_FAILED_TARIFF_SELECTION_INVALID: Iso2ResponseCodeType = 18;
pub const ISO2_RESPONSE_CODE_TYPE_FAILED_POWER_DELIVERY_NOT_APPLIED: Iso2ResponseCodeType = 17;
pub const ISO2_RESPONSE_CODE_TYPE_FAILED_WRONG_CHARGE_PARAMETER: Iso2ResponseCodeType = 16;
pub const ISO2_RESPONSE_CODE_TYPE_FAILED_CONTRACT_CANCELED: Iso2ResponseCodeType = 15;
pub const ISO2_RESPONSE_CODE_TYPE_FAILED_CHALLENGE_INVALID: Iso2ResponseCodeType = 14;
pub const ISO2_RESPONSE_CODE_TYPE_FAILED_CERT_CHAIN_ERROR: Iso2ResponseCodeType = 13;
pub const ISO2_RESPONSE_CODE_TYPE_FAILED_NO_CERTIFICATE_AVAILABLE: Iso2ResponseCodeType = 12;
pub const ISO2_RESPONSE_CODE_TYPE_FAILED_SIGNATURE_ERROR: Iso2ResponseCodeType = 11;
pub const ISO2_RESPONSE_CODE_TYPE_FAILED_CERTIFICATE_EXPIRED: Iso2ResponseCodeType = 10;
pub const ISO2_RESPONSE_CODE_TYPE_FAILED_PAYMENT_SELECTION_INVALID: Iso2ResponseCodeType = 9;
pub const ISO2_RESPONSE_CODE_TYPE_FAILED_SERVICE_SELECTION_INVALID: Iso2ResponseCodeType = 8;
pub const ISO2_RESPONSE_CODE_TYPE_FAILED_UNKNOWN_SESSION: Iso2ResponseCodeType = 7;
pub const ISO2_RESPONSE_CODE_TYPE_FAILED_SERVICE_IDINVALID: Iso2ResponseCodeType = 6;
pub const ISO2_RESPONSE_CODE_TYPE_FAILED_SEQUENCE_ERROR: Iso2ResponseCodeType = 5;
pub const ISO2_RESPONSE_CODE_TYPE_FAILED: Iso2ResponseCodeType = 4;
pub const ISO2_RESPONSE_CODE_TYPE_OK_CERTIFICATE_EXPIRES_SOON: Iso2ResponseCodeType = 3;
pub const ISO2_RESPONSE_CODE_TYPE_OK_OLD_SESSION_JOINED: Iso2ResponseCodeType = 2;
pub const ISO2_RESPONSE_CODE_TYPE_OK_NEW_SESSION_ESTABLISHED: Iso2ResponseCodeType = 1;
pub const ISO2_RESPONSE_CODE_TYPE_OK: Iso2ResponseCodeType = 0;
pub const ISO2_RESPONSE_CODE_TYPE_UNKNOWN: Iso2ResponseCodeType = 255;

pub type Iso2PaymentOptionType = u32;
pub const ISO2_PAYMENT_OPTION_TYPE_EXTERNAL_PAYMENT: Iso2PaymentOptionType = 1;
pub const ISO2_PAYMENT_OPTION_TYPE_CONTRACT: Iso2PaymentOptionType = 0;
pub const ISO2_PAYMENT_OPTION_TYPE_UNKNOWN: Iso2PaymentOptionType = 255;

pub type Iso2ChargingSessionType = u32;
pub const ISO2_CHARGING_SESSION_TYPE_PAUSE: Iso2ChargingSessionType = 1;
pub const ISO2_CHARGING_SESSION_TYPE_TERMINATE: Iso2ChargingSessionType = 0;
pub const ISO2_CHARGING_SESSION_TYPE_UNKNOWN: Iso2ChargingSessionType = 255;

pub type Iso2EnergyTransferModeType = u32;
pub const ISO2_ENERGY_TRANSFER_MODE_TYPE_DC_UNIQUE: Iso2EnergyTransferModeType = 5;
pub const ISO2_ENERGY_TRANSFER_MODE_TYPE_DC_COMBO_CORE: Iso2EnergyTransferModeType = 4;
pub const ISO2_ENERGY_TRANSFER_MODE_TYPE_DC_EXTENDED: Iso2EnergyTransferModeType = 3;
pub const ISO2_ENERGY_TRANSFER_MODE_TYPE_DC_CORE: Iso2EnergyTransferModeType = 2;
pub const ISO2_ENERGY_TRANSFER_MODE_TYPE_AC_THREE_PHASE_CORE: Iso2EnergyTransferModeType = 1;
pub const ISO2_ENERGY_TRANSFER_MODE_TYPE_AC_SINGLE_PHASE_CORE: Iso2EnergyTransferModeType = 0;
pub const ISO2_ENERGY_TRANSFER_MODE_TYPE_UNKNOWN: Iso2EnergyTransferModeType = 255;

pub type Iso2EvseProcessingType = u32;
pub const ISO2_EVSEPROCESSING_TYPE_ONGOING_WAITING_FOR_CUSTOMER_INTERACTION:
    Iso2EvseProcessingType = 2;
pub const ISO2_EVSEPROCESSING_TYPE_ONGOING: Iso2EvseProcessingType = 1;
pub const ISO2_EVSEPROCESSING_TYPE_FINISHED: Iso2EvseProcessingType = 0;
pub const ISO2_EVSEPROCESSING_TYPE_UNKNOWN: Iso2EvseProcessingType = 255;

#[derive(Clone, Copy)]
pub struct Iso2CostType {
    pub cost_kind: Iso2CostKindType,
    pub amount: u32,
    pub amount_multiplier: Option<i8>,
}

impl Default for Iso2CostType {
    fn default() -> Self {
        Self {
            cost_kind: ISO2_COST_KIND_TYPE_UNKNOWN,
            amount: 0,
            amount_multiplier: None,
        }
    }
}

pub struct Iso2TransformType {
    pub algorithm: Iso2u8FixedBufType<65>,
    pub any: Option<Iso2u8FixedBufType<4>>,
    pub xpath: Option<Iso2u8FixedBufType<65>>,
}

impl Default for Iso2TransformType {
    fn default() -> Self {
        Self {
            algorithm: Iso2u8FixedBufType::<65>::default(),
            any: None,
            xpath: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Iso2u8FixedBufType<const N: usize> {
    pub data: [u8; N],
    pub len: usize,
}

impl<const N: usize> Default for Iso2u8FixedBufType<N> {
    fn default() -> Self {
        Self {
            data: [0; N],
            len: 0,
        }
    }
}

pub struct Iso2IntervalType {
    pub _unused: i32,
}

impl Default for Iso2IntervalType {
    fn default() -> Self {
        Self { _unused: 0 }
    }
}

pub struct Iso2TransformsType {
    pub transform: Iso2TransformType,
}

impl Default for Iso2TransformsType {
    fn default() -> Self {
        Self {
            transform: Iso2TransformType::default(),
        }
    }
}

pub struct Iso2DSAKeyValueType {
    pub p: Option<Iso2u8FixedBufType<350>>,
    pub q: Option<Iso2u8FixedBufType<350>>,
    pub g: Option<Iso2u8FixedBufType<350>>,
    pub y: Iso2u8FixedBufType<350>,
    pub j: Option<Iso2u8FixedBufType<350>>,
    pub seed: Option<Iso2u8FixedBufType<350>>,
    pub pgen_counter: Option<Iso2u8FixedBufType<350>>,
}

impl Default for Iso2DSAKeyValueType {
    fn default() -> Self {
        Self {
            p: None,
            q: None,
            g: None,
            y: Iso2u8FixedBufType::<350>::default(),
            j: None,
            seed: None,
            pgen_counter: None,
        }
    }
}

pub struct Iso2X509IssuerSerialType {
    pub x509_issuer_name: Iso2u8FixedBufType<65>,
    pub x509_serial_number: ExiSigned,
}

impl Default for Iso2X509IssuerSerialType {
    fn default() -> Self {
        Self {
            x509_issuer_name: Iso2u8FixedBufType::<65>::default(),
            x509_serial_number: ExiSigned::default(),
        }
    }
}

pub struct Iso2RelativeTimeIntervalType {
    pub start: u32,
    pub duration: Option<u32>,
}

impl Default for Iso2RelativeTimeIntervalType {
    fn default() -> Self {
        Self {
            start: 0,
            duration: None,
        }
    }
}

pub struct Iso2DigestMethodType {
    pub algorithm: Iso2u8FixedBufType<65>,
    pub any: Option<Iso2u8FixedBufType<4>>,
}

impl Default for Iso2DigestMethodType {
    fn default() -> Self {
        Self {
            algorithm: Iso2u8FixedBufType::<65>::default(),
            any: None,
        }
    }
}

pub struct Iso2RSAKeyValueType {
    pub modulus: Iso2u8FixedBufType<350>,
    pub exponent: Iso2u8FixedBufType<350>,
}

impl Default for Iso2RSAKeyValueType {
    fn default() -> Self {
        Self {
            modulus: Iso2u8FixedBufType::<350>::default(),
            exponent: Iso2u8FixedBufType::<350>::default(),
        }
    }
}

pub struct Iso2CanonicalizationMethodType {
    pub algorithm: Iso2u8FixedBufType<65>,
    pub any: Option<Iso2u8FixedBufType<4>>,
}

impl Default for Iso2CanonicalizationMethodType {
    fn default() -> Self {
        Self {
            algorithm: Iso2u8FixedBufType::<65>::default(),
            any: None,
        }
    }
}

pub struct Iso2SignatureMethodType {
    pub algorithm: Iso2u8FixedBufType<65>,
    pub hmac_output_length: Option<ExiSigned>,
    pub any: Option<Iso2u8FixedBufType<4>>,
}

impl Default for Iso2SignatureMethodType {
    fn default() -> Self {
        Self {
            algorithm: Iso2u8FixedBufType::<65>::default(),
            hmac_output_length: None,
            any: None,
        }
    }
}

pub struct Iso2KeyValueType {
    pub dsa_key_value: Option<Iso2DSAKeyValueType>,
    pub rsa_key_value: Option<Iso2RSAKeyValueType>,
    pub any: Option<Iso2u8FixedBufType<4>>,
}

impl Default for Iso2KeyValueType {
    fn default() -> Self {
        Self {
            dsa_key_value: None,
            rsa_key_value: None,
            any: None,
        }
    }
}

pub struct Iso2PhysicalValueType {
    pub multiplier: i8,
    pub unit: Iso2UnitSymbolType,
    pub value: i16,
}

impl Default for Iso2PhysicalValueType {
    fn default() -> Self {
        Self {
            multiplier: 0,
            unit: ISO2_UNIT_SYMBOL_TYPE_UNKNOWN,
            value: 0,
        }
    }
}

pub struct Iso2ConsumptionCostType {
    pub start_value: Iso2PhysicalValueType,
    pub cost: Vec<Iso2CostType, 3>,
}

impl Default for Iso2ConsumptionCostType {
    fn default() -> Self {
        Self {
            start_value: Iso2PhysicalValueType::default(),
            cost: Vec::new(),
        }
    }
}

pub struct Iso2PMaxScheduleEntryType {
    pub relative_time_interval: Option<Iso2RelativeTimeIntervalType>,
    pub time_interval: Option<Iso2IntervalType>,
    pub p_max: Iso2PhysicalValueType,
}

impl Default for Iso2PMaxScheduleEntryType {
    fn default() -> Self {
        Self {
            relative_time_interval: None,
            time_interval: None,
            p_max: Iso2PhysicalValueType::default(),
        }
    }
}

pub struct Iso2SalesTariffEntryType {
    pub relative_time_interval: Option<Iso2RelativeTimeIntervalType>,
    pub time_interval: Option<Iso2IntervalType>,
    pub e_price_level: Option<u8>,
    pub consumption_cost: Vec<Iso2ConsumptionCostType, 3>,
}

impl Default for Iso2SalesTariffEntryType {
    fn default() -> Self {
        Self {
            relative_time_interval: None,
            time_interval: None,
            e_price_level: None,
            consumption_cost: Vec::new(),
        }
    }
}

pub struct Iso2ParameterType {
    pub name: Iso2u8FixedBufType<65>,
    pub bool_value: Option<i32>,
    pub byte_value: Option<i8>,
    pub short_value: Option<i16>,
    pub int_value: Option<i32>,
    pub physical_value: Option<Iso2PhysicalValueType>,
    pub string_value: Option<Iso2u8FixedBufType<65>>,
}

impl Default for Iso2ParameterType {
    fn default() -> Self {
        Self {
            name: Iso2u8FixedBufType::<65>::default(),
            bool_value: None,
            byte_value: None,
            short_value: None,
            int_value: None,
            physical_value: None,
            string_value: None,
        }
    }
}

pub struct Iso2PMaxScheduleType {
    pub p_max_schedule_entry: Vec<Iso2PMaxScheduleEntryType, 12>,
}

impl Default for Iso2PMaxScheduleType {
    fn default() -> Self {
        Self {
            p_max_schedule_entry: Vec::new(),
        }
    }
}

pub struct Iso2ReferenceType {
    pub id: Option<Iso2u8FixedBufType<65>>,
    pub ref_type: Option<Iso2u8FixedBufType<65>>,
    pub uri: Option<Iso2u8FixedBufType<65>>,
    pub transforms: Option<Iso2TransformsType>,
    pub digest_method: Iso2DigestMethodType,
    pub digest_value: Option<Iso2u8FixedBufType<350>>,
}

impl Default for Iso2ReferenceType {
    fn default() -> Self {
        Self {
            id: None,
            ref_type: None,
            uri: None,
            transforms: None,
            digest_method: Iso2DigestMethodType::default(),
            digest_value: None,
        }
    }
}

pub struct Iso2RetrievalMethodType {
    pub ref_type: Option<Iso2u8FixedBufType<65>>,
    pub uri: Option<Iso2u8FixedBufType<65>>,
    pub transforms: Option<Iso2TransformsType>,
}

impl Default for Iso2RetrievalMethodType {
    fn default() -> Self {
        Self {
            ref_type: None,
            uri: None,
            transforms: None,
        }
    }
}

pub struct Iso2SalesTariffType {
    pub id: Option<Iso2u8FixedBufType<65>>,
    pub sales_tariff_id: u8,
    pub sales_tariff_description: Option<Iso2u8FixedBufType<33>>,
    pub num_e_price_levels: Option<u8>,
    pub sales_tariff_entry: Vec<Iso2SalesTariffEntryType, 12>,
}

impl Default for Iso2SalesTariffType {
    fn default() -> Self {
        Self {
            id: None,
            sales_tariff_id: 0,
            sales_tariff_description: None,
            num_e_price_levels: None,
            sales_tariff_entry: Vec::new(),
        }
    }
}

pub struct Iso2X509DataType {
    pub x509_issuer_serial: Option<Iso2X509IssuerSerialType>,
    pub x509_ski: Option<Iso2u8FixedBufType<350>>,
    pub x509_subject_name: Option<Iso2u8FixedBufType<65>>,
    pub x509_certificate: Option<Iso2u8FixedBufType<350>>,
    pub x509_crl: Option<Iso2u8FixedBufType<350>>,
    pub any: Option<Iso2u8FixedBufType<4>>,
}

impl Default for Iso2X509DataType {
    fn default() -> Self {
        Self {
            x509_issuer_serial: None,
            x509_ski: None,
            x509_subject_name: None,
            x509_certificate: None,
            x509_crl: None,
            any: None,
        }
    }
}

pub struct Iso2PGPDataType {
    pub pgpcomponent: Iso2PGPComponentType,
}

pub enum Iso2PGPComponentType {
    Choice1(PGPChoice1Type),
    Choice2(PGPChoice2Type),
}

pub struct PGPChoice2Type {
    pub pgpkey_packet: Iso2u8FixedBufType<350>,
    pub any: Option<Iso2u8FixedBufType<4>>,
}

impl Default for PGPChoice2Type {
    fn default() -> Self {
        Self {
            pgpkey_packet: Iso2u8FixedBufType::<350>::default(),
            any: None,
        }
    }
}

pub struct PGPChoice1Type {
    pub pgpkey_id: Iso2u8FixedBufType<350>,
    pub pgpkey_packet: Option<Iso2u8FixedBufType<350>>,
    pub any: Option<Iso2u8FixedBufType<4>>,
}

impl Default for PGPChoice1Type {
    fn default() -> Self {
        Self {
            pgpkey_id: Iso2u8FixedBufType::<350>::default(),
            pgpkey_packet: None,
            any: None,
        }
    }
}

pub struct Iso2SPKIDataType {
    pub spkisexp: Iso2u8FixedBufType<350>,
    pub any: Option<Iso2u8FixedBufType<4>>,
}

impl Default for Iso2SPKIDataType {
    fn default() -> Self {
        Self {
            spkisexp: Iso2u8FixedBufType::<350>::default(),
            any: None,
        }
    }
}

pub struct Iso2SignedInfoType {
    pub id: Option<Iso2u8FixedBufType<65>>,
    pub canonicalization_method: Iso2CanonicalizationMethodType,
    pub signature_method: Iso2SignatureMethodType,
    pub reference: [Iso2ReferenceType; 4],
    pub reference_len: usize,
}

impl Default for Iso2SignedInfoType {
    fn default() -> Self {
        Self {
            id: None,
            canonicalization_method: Iso2CanonicalizationMethodType::default(),
            signature_method: Iso2SignatureMethodType::default(),
            reference: Default::default(),
            reference_len: 0,
        }
    }
}

pub struct Iso2ProfileEntryType {
    pub charging_profile_entry_start: u32,
    pub charging_profile_entry_max_power: Iso2PhysicalValueType,
    pub charging_profile_entry_max_number_of_phases_in_use: Option<i8>,
}

impl Default for Iso2ProfileEntryType {
    fn default() -> Self {
        Self {
            charging_profile_entry_start: 0,
            charging_profile_entry_max_power: Iso2PhysicalValueType::default(),
            charging_profile_entry_max_number_of_phases_in_use: None,
        }
    }
}

pub struct Iso2DCEVStatusType {
    pub ev_ready: i32,
    pub ev_error_code: Iso2DcEvErrorCodeType,
    pub ev_res_soc: i8,
}

impl Default for Iso2DCEVStatusType {
    fn default() -> Self {
        Self {
            ev_ready: 0,
            ev_error_code: ISO2_DC_EVERROR_CODE_TYPE_UNKNOWN,
            ev_res_soc: 0,
        }
    }
}

pub struct Iso2ParameterSetType {
    pub parameter_set_id: i16,
    pub parameter: Vec<Iso2ParameterType, 16>,
}

impl Default for Iso2ParameterSetType {
    fn default() -> Self {
        Self {
            parameter_set_id: 0,
            parameter: Vec::new(),
        }
    }
}

pub struct Iso2SAScheduleTupleType {
    pub saschedule_tuple_id: u8,
    pub pmax_schedule: Iso2PMaxScheduleType,
    pub sales_tariff: Option<Iso2SalesTariffType>,
}

impl Default for Iso2SAScheduleTupleType {
    fn default() -> Self {
        Self {
            saschedule_tuple_id: 0,
            pmax_schedule: Iso2PMaxScheduleType::default(),
            sales_tariff: None,
        }
    }
}

pub struct Iso2SelectedServiceType {
    pub service_id: u16,
    pub parameter_set_id: Option<i16>,
}

impl Default for Iso2SelectedServiceType {
    fn default() -> Self {
        Self {
            service_id: 0,
            parameter_set_id: None,
        }
    }
}

pub struct Iso2ServiceType {
    pub service_id: u16,
    pub service_name: Option<Iso2u8FixedBufType<33>>,
    pub service_category: Iso2ServiceCategoryType,
    pub service_scope: Option<Iso2u8FixedBufType<65>>,
    pub free_service: i32,
}

impl Default for Iso2ServiceType {
    fn default() -> Self {
        Self {
            service_id: 0,
            service_name: None,
            service_category: ISO2_SERVICE_CATEGORY_TYPE_UNKNOWN,
            service_scope: None,
            free_service: 0,
        }
    }
}

pub struct Iso2SignatureValueType {
    pub id: Option<Iso2u8FixedBufType<65>>,
    pub content: Iso2u8FixedBufType<65>,
}

impl Default for Iso2SignatureValueType {
    fn default() -> Self {
        Self {
            id: None,
            content: Iso2u8FixedBufType::<65>::default(),
        }
    }
}

pub struct Iso2SubCertificatesType {
    pub certificate: Vec<Iso2u8FixedBufType<800>, 4>,
}

impl Default for Iso2SubCertificatesType {
    fn default() -> Self {
        Self {
            certificate: Vec::new(),
        }
    }
}

pub struct Iso2KeyInfoType {
    pub id: Option<Iso2u8FixedBufType<65>>,
    pub key_name: Option<Iso2u8FixedBufType<65>>,
    pub key_value: Option<Iso2KeyValueType>,
    pub retrieval_method: Option<Iso2RetrievalMethodType>,
    pub x509_data: Option<Iso2X509DataType>,
    pub pgp_data: Option<Iso2PGPDataType>,
    pub spki_data: Option<Iso2SPKIDataType>,
    pub mgmt_data: Option<Iso2u8FixedBufType<65>>,
    pub any: Option<Iso2u8FixedBufType<4>>,
}

impl Default for Iso2KeyInfoType {
    fn default() -> Self {
        Self {
            id: None,
            key_name: None,
            key_value: None,
            retrieval_method: None,
            x509_data: None,
            pgp_data: None,
            spki_data: None,
            mgmt_data: None,
            any: None,
        }
    }
}

pub struct Iso2ObjectType {
    pub encoding: Option<Iso2u8FixedBufType<65>>,
    pub id: Option<Iso2u8FixedBufType<65>>,
    pub mime_type: Option<Iso2u8FixedBufType<65>>,
    pub any: Option<Iso2u8FixedBufType<4>>,
}

impl Default for Iso2ObjectType {
    fn default() -> Self {
        Self {
            encoding: None,
            id: None,
            mime_type: None,
            any: None,
        }
    }
}

pub struct Iso2SupportedEnergyTransferModeType {
    pub energy_transfer_mode: Vec<Iso2EnergyTransferModeType, 6>,
}

impl Default for Iso2SupportedEnergyTransferModeType {
    fn default() -> Self {
        Self {
            energy_transfer_mode: Vec::new(),
        }
    }
}

pub struct Iso2CertificateChainType {
    pub id: Option<Iso2u8FixedBufType<65>>,
    pub certificate: Iso2u8FixedBufType<800>,
    pub sub_certificates: Option<Iso2SubCertificatesType>,
}

impl Default for Iso2CertificateChainType {
    fn default() -> Self {
        Self {
            id: None,
            certificate: Iso2u8FixedBufType::<800>::default(),
            sub_certificates: None,
        }
    }
}

pub struct Iso2BodyBaseType {
    pub _unused: i32,
}

impl Default for Iso2BodyBaseType {
    fn default() -> Self {
        Self { _unused: 0 }
    }
}

pub struct Iso2NotificationType {
    pub fault_code: Iso2FaultCodeType,
    pub fault_msg: Option<Iso2u8FixedBufType<65>>,
}

impl Default for Iso2NotificationType {
    fn default() -> Self {
        Self {
            fault_code: ISO2_FAULT_CODE_TYPE_UNKNOWN,
            fault_msg: None,
        }
    }
}

pub struct Iso2DCEVSEStatusType {
    pub notification_max_delay: u16,
    pub evse_notification: Iso2EvseNotificationType,
    pub evse_isolation_status: Option<Iso2IsolationLevelType>,
    pub evse_status_code: Iso2DcEvseStatusCodeType,
}

impl Default for Iso2DCEVSEStatusType {
    fn default() -> Self {
        Self {
            notification_max_delay: 0,
            evse_notification: ISO2_EVSENOTIFICATION_TYPE_UNKNOWN,
            evse_isolation_status: None,
            evse_status_code: ISO2_DC_EVSESTATUS_CODE_TYPE_UNKNOWN,
        }
    }
}

pub struct Iso2SelectedServiceListType {
    pub selected_service: Vec<Iso2SelectedServiceType, 16>,
}

impl Default for Iso2SelectedServiceListType {
    fn default() -> Self {
        Self {
            selected_service: Vec::new(),
        }
    }
}

pub struct Iso2PaymentOptionListType {
    pub payment_option: Vec<Iso2PaymentOptionType, 2>,
}

impl Default for Iso2PaymentOptionListType {
    fn default() -> Self {
        Self {
            payment_option: Vec::new(),
        }
    }
}

pub struct Iso2SignatureType {
    pub id: Option<Iso2u8FixedBufType<65>>,
    pub signed_info: Iso2SignedInfoType,
    pub signature_value: Iso2SignatureValueType,
    pub key_info: Option<Iso2KeyInfoType>,
    pub object: Option<Iso2ObjectType>,
}

impl Default for Iso2SignatureType {
    fn default() -> Self {
        Self {
            id: None,
            signed_info: Iso2SignedInfoType::default(),
            signature_value: Iso2SignatureValueType::default(),
            key_info: None,
            object: None,
        }
    }
}

pub struct Iso2ChargingProfileType {
    pub profile_entry: Vec<Iso2ProfileEntryType, 24>,
}

impl Default for Iso2ChargingProfileType {
    fn default() -> Self {
        Self {
            profile_entry: Vec::new(),
        }
    }
}

pub struct Iso2ServiceParameterListType {
    pub parameter_set: Vec<Iso2ParameterSetType, 5>,
}

impl Default for Iso2ServiceParameterListType {
    fn default() -> Self {
        Self {
            parameter_set: Vec::new(),
        }
    }
}

pub struct Iso2ListOfRootCertificateIDsType {
    pub root_certificate_id: Vec<Iso2X509IssuerSerialType, 5>,
}

impl Default for Iso2ListOfRootCertificateIDsType {
    fn default() -> Self {
        Self {
            root_certificate_id: Vec::new(),
        }
    }
}

pub struct Iso2ACEVChargeParameterType {
    pub departure_time: Option<u32>,
    pub e_amount: Iso2PhysicalValueType,
    pub ev_max_voltage: Iso2PhysicalValueType,
    pub ev_max_current: Iso2PhysicalValueType,
    pub ev_min_current: Iso2PhysicalValueType,
}

impl Default for Iso2ACEVChargeParameterType {
    fn default() -> Self {
        Self {
            departure_time: None,
            e_amount: Iso2PhysicalValueType::default(),
            ev_max_voltage: Iso2PhysicalValueType::default(),
            ev_max_current: Iso2PhysicalValueType::default(),
            ev_min_current: Iso2PhysicalValueType::default(),
        }
    }
}

pub struct Iso2DCEVChargeParameterType {
    pub departure_time: Option<u32>,
    pub dc_ev_status: Iso2DCEVStatusType,
    pub ev_maximum_current_limit: Iso2PhysicalValueType,
    pub ev_maximum_power_limit: Option<Iso2PhysicalValueType>,
    pub ev_maximum_voltage_limit: Iso2PhysicalValueType,
    pub ev_energy_capacity: Option<Iso2PhysicalValueType>,
    pub ev_energy_request: Option<Iso2PhysicalValueType>,
    pub full_soc: Option<i8>,
    pub bulk_soc: Option<i8>,
}

impl Default for Iso2DCEVChargeParameterType {
    fn default() -> Self {
        Self {
            departure_time: None,
            dc_ev_status: Iso2DCEVStatusType::default(),
            ev_maximum_current_limit: Iso2PhysicalValueType::default(),
            ev_maximum_power_limit: None,
            ev_maximum_voltage_limit: Iso2PhysicalValueType::default(),
            ev_energy_capacity: None,
            ev_energy_request: None,
            full_soc: None,
            bulk_soc: None,
        }
    }
}

pub struct Iso2EVChargeParameterType {
    pub departure_time: Option<u32>,
    pub ac_ev_charge_parameter: Iso2ACEVChargeParameterType,
    pub dc_ev_charge_parameter: Iso2DCEVChargeParameterType,
}

impl Default for Iso2EVChargeParameterType {
    fn default() -> Self {
        Self {
            departure_time: None,
            ac_ev_charge_parameter: Iso2ACEVChargeParameterType::default(),
            dc_ev_charge_parameter: Iso2DCEVChargeParameterType::default(),
        }
    }
}

pub struct Iso2SASchedulesType {
    pub _unused: i32,
}

pub struct Iso2SAScheduleListType {
    pub sa_schedule_tuple: Vec<Iso2SAScheduleTupleType, 3>,
}

impl Default for Iso2SAScheduleListType {
    fn default() -> Self {
        Self {
            sa_schedule_tuple: Vec::new(),
        }
    }
}

pub struct Iso2ChargeServiceType {
    pub service_id: u16,
    pub service_name: Option<Iso2u8FixedBufType<33>>,
    pub service_category: Iso2ServiceCategoryType,
    pub service_scope: Option<Iso2u8FixedBufType<65>>,
    pub free_service: i32,
    pub supported_energy_transfer_mode: Iso2SupportedEnergyTransferModeType,
}

impl Default for Iso2ChargeServiceType {
    fn default() -> Self {
        Self {
            service_id: 0,
            service_name: None,
            service_category: ISO2_SERVICE_CATEGORY_TYPE_UNKNOWN,
            service_scope: None,
            free_service: 0,
            supported_energy_transfer_mode: Iso2SupportedEnergyTransferModeType::default(),
        }
    }
}

pub struct Iso2EVPowerDeliveryParameterType {
    pub _unused: i32,
}

pub struct Iso2DCEVPowerDeliveryParameterType {
    pub dc_ev_status: Iso2DCEVStatusType,
    pub bulk_charging_complete: Option<i32>,
    pub charging_complete: i32,
}

impl Default for Iso2DCEVPowerDeliveryParameterType {
    fn default() -> Self {
        Self {
            dc_ev_status: Iso2DCEVStatusType::default(),
            bulk_charging_complete: None,
            charging_complete: 0,
        }
    }
}

pub struct Iso2ContractSignatureEncryptedPrivateKeyType {
    pub id: Iso2u8FixedBufType<65>,
    pub content: Iso2u8FixedBufType<65>,
}

impl Default for Iso2ContractSignatureEncryptedPrivateKeyType {
    fn default() -> Self {
        Self {
            id: Iso2u8FixedBufType::<65>::default(),
            content: Iso2u8FixedBufType::<65>::default(),
        }
    }
}

pub struct Iso2EVSEChargeParameterType {
    pub _unused: i32,
}

pub struct Iso2DCEVSEChargeParameterType {
    pub dc_evse_status: Iso2DCEVSEStatusType,
    pub evse_maximum_current_limit: Iso2PhysicalValueType,
    pub evse_maximum_power_limit: Iso2PhysicalValueType,
    pub evse_maximum_voltage_limit: Iso2PhysicalValueType,
    pub evse_minimum_current_limit: Iso2PhysicalValueType,
    pub evse_minimum_voltage_limit: Iso2PhysicalValueType,
    pub evse_current_regulation_tolerance: Option<Iso2PhysicalValueType>,
    pub evse_peak_current_ripple: Iso2PhysicalValueType,
    pub evse_energy_to_be_delivered: Option<Iso2PhysicalValueType>,
}

impl Default for Iso2DCEVSEChargeParameterType {
    fn default() -> Self {
        Self {
            dc_evse_status: Iso2DCEVSEStatusType::default(),
            evse_maximum_current_limit: Iso2PhysicalValueType::default(),
            evse_maximum_power_limit: Iso2PhysicalValueType::default(),
            evse_maximum_voltage_limit: Iso2PhysicalValueType::default(),
            evse_minimum_current_limit: Iso2PhysicalValueType::default(),
            evse_minimum_voltage_limit: Iso2PhysicalValueType::default(),
            evse_current_regulation_tolerance: None,
            evse_peak_current_ripple: Iso2PhysicalValueType::default(),
            evse_energy_to_be_delivered: None,
        }
    }
}

pub struct Iso2ServiceListType {
    pub service: Vec<Iso2ServiceType, 8>,
}

impl Default for Iso2ServiceListType {
    fn default() -> Self {
        Self {
            service: Vec::new(),
        }
    }
}

pub struct Iso2DiffieHellmanPublickeyType {
    pub id: Iso2u8FixedBufType<65>,
    pub content: Iso2u8FixedBufType<65>,
}

impl Default for Iso2DiffieHellmanPublickeyType {
    fn default() -> Self {
        Self {
            id: Iso2u8FixedBufType::<65>::default(),
            content: Iso2u8FixedBufType::<65>::default(),
        }
    }
}

pub struct Iso2EMAIDType {
    pub id: Iso2u8FixedBufType<65>,
    pub content: Iso2u8FixedBufType<65>,
}

impl Default for Iso2EMAIDType {
    fn default() -> Self {
        Self {
            id: Iso2u8FixedBufType::<65>::default(),
            content: Iso2u8FixedBufType::<65>::default(),
        }
    }
}

pub struct Iso2ACEVSEStatusType {
    pub notification_max_delay: u16,
    pub evse_notification: Iso2EvseNotificationType,
    pub rcd: i32,
}

impl Default for Iso2ACEVSEStatusType {
    fn default() -> Self {
        Self {
            notification_max_delay: 0,
            evse_notification: ISO2_EVSENOTIFICATION_TYPE_UNKNOWN,
            rcd: 0,
        }
    }
}

pub struct Iso2EVSEStatusType {
    pub notification_max_delay: u16,
    pub evse_notification: Iso2EvseNotificationType,
    pub ac_evse_status: Iso2ACEVSEStatusType,
    pub dc_evse_status: Iso2DCEVSEStatusType,
}

impl Default for Iso2EVSEStatusType {
    fn default() -> Self {
        Self {
            notification_max_delay: 0,
            evse_notification: ISO2_EVSENOTIFICATION_TYPE_UNKNOWN,
            ac_evse_status: Iso2ACEVSEStatusType::default(),
            dc_evse_status: Iso2DCEVSEStatusType::default(),
        }
    }
}

pub struct Iso2ACEVSEChargeParameterType {
    pub ac_evse_status: Iso2ACEVSEStatusType,
    pub evse_nominal_voltage: Iso2PhysicalValueType,
    pub evse_max_current: Iso2PhysicalValueType,
}

impl Default for Iso2ACEVSEChargeParameterType {
    fn default() -> Self {
        Self {
            ac_evse_status: Iso2ACEVSEStatusType::default(),
            evse_nominal_voltage: Iso2PhysicalValueType::default(),
            evse_max_current: Iso2PhysicalValueType::default(),
        }
    }
}

pub struct Iso2MeterInfoType {
    pub meter_id: Iso2u8FixedBufType<33>,
    pub meter_reading: Option<u64>,
    pub sig_meter_reading: Option<Iso2u8FixedBufType<64>>,
    pub meter_status: Option<i16>,
    pub t_meter: Option<i64>,
}

impl Default for Iso2MeterInfoType {
    fn default() -> Self {
        Self {
            meter_id: Iso2u8FixedBufType::<33>::default(),
            meter_reading: None,
            sig_meter_reading: None,
            meter_status: None,
            t_meter: None,
        }
    }
}

pub struct Iso2MessageHeaderType {
    pub session_id: Iso2u8FixedBufType<8>,
    pub notification: Option<Iso2NotificationType>,
    pub signature: Option<Iso2SignatureType>,
}

impl Default for Iso2MessageHeaderType {
    fn default() -> Self {
        Self {
            session_id: Iso2u8FixedBufType::<8>::default(),
            notification: None,
            signature: None,
        }
    }
}

pub struct Iso2PowerDeliveryReqType {
    pub charge_progress: Iso2ChargeProgressType,
    pub sa_schedule_tuple_id: u8,
    pub charging_profile: Option<Iso2ChargingProfileType>,
    pub dc_ev_power_delivery_parameter: Option<Iso2DCEVPowerDeliveryParameterType>,
    pub ev_power_delivery_parameter: Option<Iso2EVPowerDeliveryParameterType>,
}

impl Default for Iso2PowerDeliveryReqType {
    fn default() -> Self {
        Self {
            charge_progress: ISO2_CHARGE_PROGRESS_TYPE_UNKNOWN,
            sa_schedule_tuple_id: 0,
            charging_profile: None,
            dc_ev_power_delivery_parameter: None,
            ev_power_delivery_parameter: None,
        }
    }
}

pub struct Iso2CurrentDemandResType {
    pub response_code: Iso2ResponseCodeType,
    pub dc_evse_status: Iso2DCEVSEStatusType,
    pub evse_present_voltage: Iso2PhysicalValueType,
    pub evse_present_current: Iso2PhysicalValueType,
    pub evse_current_limit_achieved: i32,
    pub evse_voltage_limit_achieved: i32,
    pub evse_power_limit_achieved: i32,
    pub evse_maximum_voltage_limit: Option<Iso2PhysicalValueType>,
    pub evse_maximum_current_limit: Option<Iso2PhysicalValueType>,
    pub evse_maximum_power_limit: Option<Iso2PhysicalValueType>,
    pub evse_id: Iso2u8FixedBufType<38>,
    pub sa_schedule_tuple_id: u8,
    pub meter_info: Option<Iso2MeterInfoType>,
    pub receipt_required: Option<i32>,
}

impl Default for Iso2CurrentDemandResType {
    fn default() -> Self {
        Self {
            response_code: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            dc_evse_status: Iso2DCEVSEStatusType::default(),
            evse_present_voltage: Iso2PhysicalValueType::default(),
            evse_present_current: Iso2PhysicalValueType::default(),
            evse_current_limit_achieved: 0,
            evse_voltage_limit_achieved: 0,
            evse_power_limit_achieved: 0,
            evse_maximum_voltage_limit: None,
            evse_maximum_current_limit: None,
            evse_maximum_power_limit: None,
            evse_id: Iso2u8FixedBufType::<38>::default(),
            sa_schedule_tuple_id: 0,
            meter_info: None,
            receipt_required: None,
        }
    }
}

pub struct Iso2ChargingStatusResType {
    pub response_code: Iso2ResponseCodeType,
    pub evse_id: Iso2u8FixedBufType<38>,
    pub sa_schedule_tuple_id: u8,
    pub evse_max_current: Option<Iso2PhysicalValueType>,
    pub meter_info: Option<Iso2MeterInfoType>,
    pub receipt_required: Option<i32>,
    pub ac_evse_status: Iso2ACEVSEStatusType,
}

impl Default for Iso2ChargingStatusResType {
    fn default() -> Self {
        Self {
            response_code: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            evse_id: Iso2u8FixedBufType::<38>::default(),
            sa_schedule_tuple_id: 0,
            evse_max_current: None,
            meter_info: None,
            receipt_required: None,
            ac_evse_status: Iso2ACEVSEStatusType::default(),
        }
    }
}

pub struct Iso2AuthorizationReqType {
    pub id: Option<Iso2u8FixedBufType<65>>,
    pub gen_challenge: Option<Iso2u8FixedBufType<16>>,
}

impl Default for Iso2AuthorizationReqType {
    fn default() -> Self {
        Self {
            id: None,
            gen_challenge: None,
        }
    }
}

pub struct Iso2PreChargeReqType {
    pub dc_ev_status: Option<Iso2DCEVStatusType>,
    pub ev_target_voltage: Option<Iso2PhysicalValueType>,
    pub ev_target_current: Option<Iso2PhysicalValueType>,
}

impl Default for Iso2PreChargeReqType {
    fn default() -> Self {
        Self {
            dc_ev_status: None,
            ev_target_voltage: None,
            ev_target_current: None,
        }
    }
}

pub struct Iso2ServiceDetailResType {
    pub response_code: Iso2ResponseCodeType,
    pub service_id: u16,
    pub service_parameter_list: Option<Iso2ServiceParameterListType>,
}

impl Default for Iso2ServiceDetailResType {
    fn default() -> Self {
        Self {
            response_code: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            service_id: 0,
            service_parameter_list: None,
        }
    }
}

pub struct Iso2PaymentServiceSelectionResType {
    pub response_code: Iso2ResponseCodeType,
}

impl Default for Iso2PaymentServiceSelectionResType {
    fn default() -> Self {
        Self {
            response_code: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
        }
    }
}

pub struct Iso2CertificateUpdateReqType {
    pub id: Iso2u8FixedBufType<65>,
    pub contract_signature_cert_chain: Iso2CertificateChainType,
    pub e_maid: Iso2u8FixedBufType<16>,
    pub list_of_root_certificate_ids: Iso2ListOfRootCertificateIDsType,
}

impl Default for Iso2CertificateUpdateReqType {
    fn default() -> Self {
        Self {
            id: Iso2u8FixedBufType::<65>::default(),
            contract_signature_cert_chain: Iso2CertificateChainType::default(),
            e_maid: Iso2u8FixedBufType::<16>::default(),
            list_of_root_certificate_ids: Iso2ListOfRootCertificateIDsType::default(),
        }
    }
}

pub struct Iso2SessionSetupResType {
    pub response_code: Iso2ResponseCodeType,
    pub evse_id: Iso2u8FixedBufType<38>,
    pub evse_time_stamp: Option<i64>,
}

impl Default for Iso2SessionSetupResType {
    fn default() -> Self {
        Self {
            response_code: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            evse_id: Iso2u8FixedBufType::<38>::default(),
            evse_time_stamp: None,
        }
    }
}

pub struct Iso2CertificateInstallationReqType {
    pub id: Iso2u8FixedBufType<65>,
    pub oem_provisioning_cert: Iso2u8FixedBufType<800>,
    pub list_of_root_certificate_ids: Iso2ListOfRootCertificateIDsType,
}

impl Default for Iso2CertificateInstallationReqType {
    fn default() -> Self {
        Self {
            id: Iso2u8FixedBufType::<65>::default(),
            oem_provisioning_cert: Iso2u8FixedBufType::<800>::default(),
            list_of_root_certificate_ids: Iso2ListOfRootCertificateIDsType::default(),
        }
    }
}

pub struct Iso2CertificateInstallationResType {
    pub response_code: Iso2ResponseCodeType,
    pub sa_provisioning_certificate_chain: Iso2CertificateChainType,
    pub contract_signature_cert_chain: Iso2CertificateChainType,
    pub contract_signature_encrypted_private_key: Iso2ContractSignatureEncryptedPrivateKeyType,
    pub dh_public_key: Iso2DiffieHellmanPublickeyType,
    pub e_maid: Iso2EMAIDType,
}

impl Default for Iso2CertificateInstallationResType {
    fn default() -> Self {
        Self {
            response_code: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            sa_provisioning_certificate_chain: Iso2CertificateChainType::default(),
            contract_signature_cert_chain: Iso2CertificateChainType::default(),
            contract_signature_encrypted_private_key:
                Iso2ContractSignatureEncryptedPrivateKeyType::default(),
            dh_public_key: Iso2DiffieHellmanPublickeyType::default(),
            e_maid: Iso2EMAIDType::default(),
        }
    }
}

pub struct Iso2WeldingDetectionResType {
    pub response_code: Iso2ResponseCodeType,
    pub dc_evse_status: Iso2DCEVSEStatusType,
    pub evse_present_voltage: Iso2PhysicalValueType,
}

impl Default for Iso2WeldingDetectionResType {
    fn default() -> Self {
        Self {
            response_code: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            dc_evse_status: Iso2DCEVSEStatusType::default(),
            evse_present_voltage: Iso2PhysicalValueType::default(),
        }
    }
}

pub struct Iso2CurrentDemandReqType {
    pub dc_ev_status: Iso2DCEVStatusType,
    pub ev_target_current: Iso2PhysicalValueType,
    pub ev_maximum_voltage_limit: Option<Iso2PhysicalValueType>,
    pub ev_maximum_current_limit: Option<Iso2PhysicalValueType>,
    pub ev_maximum_power_limit: Option<Iso2PhysicalValueType>,
    pub bulk_charging_complete: Option<i32>,
    pub charging_complete: i32,
    pub remaining_time_to_full_soc: Option<Iso2PhysicalValueType>,
    pub remaining_time_to_bulk_soc: Option<Iso2PhysicalValueType>,
    pub ev_target_voltage: Iso2PhysicalValueType,
}

impl Default for Iso2CurrentDemandReqType {
    fn default() -> Self {
        Self {
            dc_ev_status: Iso2DCEVStatusType::default(),
            ev_target_current: Iso2PhysicalValueType::default(),
            ev_maximum_voltage_limit: None,
            ev_maximum_current_limit: None,
            ev_maximum_power_limit: None,
            bulk_charging_complete: None,
            charging_complete: 0,
            remaining_time_to_full_soc: None,
            remaining_time_to_bulk_soc: None,
            ev_target_voltage: Iso2PhysicalValueType::default(),
        }
    }
}

pub struct Iso2PreChargeResType {
    pub response_code: Iso2ResponseCodeType,
    pub dc_evse_status: Iso2DCEVSEStatusType,
    pub evse_present_voltage: Iso2PhysicalValueType,
}

impl Default for Iso2PreChargeResType {
    fn default() -> Self {
        Self {
            response_code: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            dc_evse_status: Iso2DCEVSEStatusType::default(),
            evse_present_voltage: Iso2PhysicalValueType::default(),
        }
    }
}

pub struct Iso2CertificateUpdateResType {
    pub response_code: Iso2ResponseCodeType,
    pub sa_provisioning_certificate_chain: Iso2CertificateChainType,
    pub contract_signature_cert_chain: Iso2CertificateChainType,
    pub contract_signature_encrypted_private_key: Iso2ContractSignatureEncryptedPrivateKeyType,
    pub dh_public_key: Iso2DiffieHellmanPublickeyType,
    pub e_maid: Iso2EMAIDType,
    pub retry_counter: Option<i16>,
}

impl Default for Iso2CertificateUpdateResType {
    fn default() -> Self {
        Self {
            response_code: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            sa_provisioning_certificate_chain: Iso2CertificateChainType::default(),
            contract_signature_cert_chain: Iso2CertificateChainType::default(),
            contract_signature_encrypted_private_key:
                Iso2ContractSignatureEncryptedPrivateKeyType::default(),
            dh_public_key: Iso2DiffieHellmanPublickeyType::default(),
            e_maid: Iso2EMAIDType::default(),
            retry_counter: None,
        }
    }
}

pub struct Iso2MeteringReceiptReqType {
    pub id: Option<Iso2u8FixedBufType<65>>,
    pub session_id: Iso2u8FixedBufType<8>,
    pub sa_schedule_tuple_id: Option<u8>,
    pub meter_info: Iso2MeterInfoType,
}

impl Default for Iso2MeteringReceiptReqType {
    fn default() -> Self {
        Self {
            id: None,
            session_id: Iso2u8FixedBufType::<8>::default(),
            sa_schedule_tuple_id: None,
            meter_info: Iso2MeterInfoType::default(),
        }
    }
}

pub struct Iso2ChargingStatusReqType {
    pub _unused: i32,
}

impl Default for Iso2ChargingStatusReqType {
    fn default() -> Self {
        Self { _unused: 0 }
    }
}

pub struct Iso2SessionStopResType {
    pub response_code: Iso2ResponseCodeType,
}

impl Default for Iso2SessionStopResType {
    fn default() -> Self {
        Self {
            response_code: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
        }
    }
}

pub struct Iso2ChargeParameterDiscoveryReqType {
    pub max_entries_sa_schedule_tuple: Option<u16>,
    pub requested_energy_transfer_mode: Iso2EnergyTransferModeType,
    pub ac_ev_charge_parameter: Option<Iso2ACEVChargeParameterType>,
    pub dc_ev_charge_parameter: Option<Iso2DCEVChargeParameterType>,
    pub ev_charge_parameter: Option<Iso2EVChargeParameterType>,
}

impl Default for Iso2ChargeParameterDiscoveryReqType {
    fn default() -> Self {
        Self {
            max_entries_sa_schedule_tuple: None,
            requested_energy_transfer_mode: ISO2_ENERGY_TRANSFER_MODE_TYPE_UNKNOWN,
            ac_ev_charge_parameter: None,
            dc_ev_charge_parameter: None,
            ev_charge_parameter: None,
        }
    }
}

pub struct Iso2CableCheckReqType {
    pub dc_ev_status: Iso2DCEVStatusType,
}

impl Default for Iso2CableCheckReqType {
    fn default() -> Self {
        Self {
            dc_ev_status: Iso2DCEVStatusType::default(),
        }
    }
}

pub struct Iso2WeldingDetectionReqType {
    pub dc_ev_status: Iso2DCEVStatusType,
}

impl Default for Iso2WeldingDetectionReqType {
    fn default() -> Self {
        Self {
            dc_ev_status: Iso2DCEVStatusType::default(),
        }
    }
}

pub struct Iso2PowerDeliveryResType {
    pub response_code: Iso2ResponseCodeType,
    pub ac_evse_status: Option<Iso2ACEVSEStatusType>,
    pub dc_evse_status: Option<Iso2DCEVSEStatusType>,
    pub evse_status: Option<Iso2EVSEStatusType>,
}

impl Default for Iso2PowerDeliveryResType {
    fn default() -> Self {
        Self {
            response_code: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            ac_evse_status: None,
            dc_evse_status: None,
            evse_status: None,
        }
    }
}

pub struct Iso2ChargeParameterDiscoveryResType {
    pub response_code: Iso2ResponseCodeType,
    pub evse_processing: Iso2EvseProcessingType,
    pub sa_schedule_list: Option<Iso2SAScheduleListType>,
    pub sa_schedules: Option<Iso2SASchedulesType>,
    pub ac_evse_charge_parameter: Option<Iso2ACEVSEChargeParameterType>,
    pub dc_evse_charge_parameter: Option<Iso2DCEVSEChargeParameterType>,
    pub evse_charge_parameter: Option<Iso2EVSEChargeParameterType>,
}

impl Default for Iso2ChargeParameterDiscoveryResType {
    fn default() -> Self {
        Self {
            response_code: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            evse_processing: ISO2_EVSEPROCESSING_TYPE_UNKNOWN,
            sa_schedule_list: None,
            sa_schedules: None,
            ac_evse_charge_parameter: None,
            dc_evse_charge_parameter: None,
            evse_charge_parameter: None,
        }
    }
}

pub struct Iso2PaymentServiceSelectionReqType {
    pub selected_payment_option: Iso2PaymentOptionType,
    pub selected_service_list: Iso2SelectedServiceListType,
}

impl Default for Iso2PaymentServiceSelectionReqType {
    fn default() -> Self {
        Self {
            selected_payment_option: ISO2_PAYMENT_OPTION_TYPE_UNKNOWN,
            selected_service_list: Iso2SelectedServiceListType::default(),
        }
    }
}

pub struct Iso2MeteringReceiptResType {
    pub response_code: Iso2ResponseCodeType,
    pub ac_evse_status: Option<Iso2ACEVSEStatusType>,
    pub dc_evse_status: Option<Iso2DCEVSEStatusType>,
    pub evse_status: Option<Iso2EVSEStatusType>,
}

impl Default for Iso2MeteringReceiptResType {
    fn default() -> Self {
        Self {
            response_code: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            ac_evse_status: None,
            dc_evse_status: None,
            evse_status: None,
        }
    }
}

pub struct Iso2CableCheckResType {
    pub response_code: Iso2ResponseCodeType,
    pub dc_evse_status: Option<Iso2DCEVSEStatusType>,
    pub evse_processing: Iso2EvseProcessingType,
}

impl Default for Iso2CableCheckResType {
    fn default() -> Self {
        Self {
            response_code: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            dc_evse_status: None,
            evse_processing: ISO2_EVSEPROCESSING_TYPE_UNKNOWN,
        }
    }
}

pub struct Iso2ServiceDiscoveryResType {
    pub response_code: Iso2ResponseCodeType,
    pub payment_option_list: Iso2PaymentOptionListType,
    pub charge_service: Iso2ChargeServiceType,
    pub service_list: Option<Iso2ServiceListType>,
}

impl Default for Iso2ServiceDiscoveryResType {
    fn default() -> Self {
        Self {
            response_code: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            payment_option_list: Iso2PaymentOptionListType::default(),
            charge_service: Iso2ChargeServiceType::default(),
            service_list: None,
        }
    }
}

pub struct Iso2ServiceDetailReqType {
    pub service_id: u16,
}

impl Default for Iso2ServiceDetailReqType {
    fn default() -> Self {
        Self { service_id: 0 }
    }
}

pub struct Iso2SessionSetupReqType {
    pub evcc_id: Iso2u8FixedBufType<6>,
}

impl Default for Iso2SessionSetupReqType {
    fn default() -> Self {
        Self {
            evcc_id: Iso2u8FixedBufType::<6>::default(),
        }
    }
}

pub struct Iso2SessionStopReqType {
    pub charging_session: Iso2ChargingSessionType,
}

impl Default for Iso2SessionStopReqType {
    fn default() -> Self {
        Self {
            charging_session: ISO2_CHARGING_SESSION_TYPE_UNKNOWN,
        }
    }
}

pub struct Iso2ServiceDiscoveryReqType {
    pub service_scope: Option<Iso2u8FixedBufType<65>>,
    pub service_category: Option<Iso2ServiceCategoryType>,
}

impl Default for Iso2ServiceDiscoveryReqType {
    fn default() -> Self {
        Self {
            service_scope: None,
            service_category: None,
        }
    }
}

pub struct Iso2AuthorizationResType {
    pub response_code: Iso2ResponseCodeType,
    pub evse_processing: Iso2EvseProcessingType,
}

impl Default for Iso2AuthorizationResType {
    fn default() -> Self {
        Self {
            response_code: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            evse_processing: ISO2_EVSEPROCESSING_TYPE_UNKNOWN,
        }
    }
}

pub struct Iso2PaymentDetailsReqType {
    pub e_maid: Iso2u8FixedBufType<16>,
    pub contract_signature_cert_chain: Iso2CertificateChainType,
}

impl Default for Iso2PaymentDetailsReqType {
    fn default() -> Self {
        Self {
            e_maid: Iso2u8FixedBufType::<16>::default(),
            contract_signature_cert_chain: Iso2CertificateChainType::default(),
        }
    }
}

pub struct Iso2PaymentDetailsResType {
    pub response_code: Iso2ResponseCodeType,
    pub gen_challenge: Iso2u8FixedBufType<16>,
    pub evse_time_stamp: i64,
}

impl Default for Iso2PaymentDetailsResType {
    fn default() -> Self {
        Self {
            response_code: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            gen_challenge: Iso2u8FixedBufType::<16>::default(),
            evse_time_stamp: 0,
        }
    }
}

pub struct Iso2BodyType {
    pub body_type_component: Iso2BodyTypeEnum,
}

pub enum Iso2BodyTypeEnum {
    AuthorizationReq(Iso2AuthorizationReqType),
    AuthorizationRes(Iso2AuthorizationResType),
    BodyElement(Iso2BodyBaseType),
    CableCheckReq(Iso2CableCheckReqType),
    CableCheckRes(Iso2CableCheckResType),
    CertificateInstallationReq(Iso2CertificateInstallationReqType),
    CertificateInstallationRes(Iso2CertificateInstallationResType),
    CertificateUpdateReq(Iso2CertificateUpdateReqType),
    CertificateUpdateRes(Iso2CertificateUpdateResType),
    ChargeParameterDiscoveryReq(Iso2ChargeParameterDiscoveryReqType),
    ChargeParameterDiscoveryRes(Iso2ChargeParameterDiscoveryResType),
    ChargingStatusReq(Iso2ChargingStatusReqType),
    ChargingStatusRes(Iso2ChargingStatusResType),
    CurrentDemandReq(Iso2CurrentDemandReqType),
    CurrentDemandRes(Iso2CurrentDemandResType),
    MeteringReceiptReq(Iso2MeteringReceiptReqType),
    MeteringReceiptRes(Iso2MeteringReceiptResType),
    PaymentDetailsReq(Iso2PaymentDetailsReqType),
    PaymentDetailsRes(Iso2PaymentDetailsResType),
    PaymentServiceSelectionReq(Iso2PaymentServiceSelectionReqType),
    PaymentServiceSelectionRes(Iso2PaymentServiceSelectionResType),
    PowerDeliveryReq(Iso2PowerDeliveryReqType),
    PowerDeliveryRes(Iso2PowerDeliveryResType),
    PreChargeReq(Iso2PreChargeReqType),
    PreChargeRes(Iso2PreChargeResType),
    ServiceDetailReq(Iso2ServiceDetailReqType),
    ServiceDetailRes(Iso2ServiceDetailResType),
    ServiceDiscoveryReq(Iso2ServiceDiscoveryReqType),
    ServiceDiscoveryRes(Iso2ServiceDiscoveryResType),
    SessionSetupReq(Iso2SessionSetupReqType),
    SessionSetupRes(Iso2SessionSetupResType),
    SessionStopReq(Iso2SessionStopReqType),
    SessionStopRes(Iso2SessionStopResType),
    WeldingDetectionReq(Iso2WeldingDetectionReqType),
    WeldingDetectionRes(Iso2WeldingDetectionResType),
}

pub struct Iso2v2gMessage {
    pub header: Iso2MessageHeaderType,
    pub body: Iso2BodyType,
}

pub struct Iso2ExiDocument {
    pub v2g_message: Iso2v2gMessage,
}

pub struct Iso2ExiFragment {
    pub exi_fragment_components: Iso2ExiFragmentEnum,
}

pub enum Iso2ExiFragmentEnum {
    AuthorizationReq(Iso2AuthorizationReqType),
    CertificateInstallationReq(Iso2CertificateInstallationReqType),
    CertificateUpdateReq(Iso2CertificateUpdateReqType),
    ContractSignatureCertChain(Iso2CertificateChainType),
    ContractSignatureEncryptedPrivateKey(Iso2ContractSignatureEncryptedPrivateKeyType),
    DHpublickey(Iso2DiffieHellmanPublickeyType),
    MeteringReceiptReq(Iso2MeteringReceiptReqType),
    SalesTariff(Iso2SalesTariffType),
    SignedInfo(Iso2SignedInfoType),
    EMaid(Iso2EMAIDType),
}

pub struct Iso2XmlDSigFragment {
    pub xml_dsig_components: Iso2XmlDSigEnum,
}

pub enum Iso2XmlDSigEnum {
    CanonicalizationMethod(Iso2CanonicalizationMethodType),
    DSAKeyValue(Iso2DSAKeyValueType),
    DigestMethod(Iso2DigestMethodType),
    KeyInfo(Iso2KeyInfoType),
    KeyValue(Iso2KeyValueType),
    Object(Iso2ObjectType),
    PGPData(Iso2PGPDataType),
    RSAKeyValue(Iso2RSAKeyValueType),
    Reference(Iso2ReferenceType),
    RetrievalMethod(Iso2RetrievalMethodType),
    SPKIData(Iso2SPKIDataType),
    Signature(Iso2SignatureType),
    SignatureMethod(Iso2SignatureMethodType),
    SignatureValue(Iso2SignatureValueType),
    SignedInfo(Iso2SignedInfoType),
    Transform(Iso2TransformType),
    Transforms(Iso2TransformsType),
    X509Data(Iso2X509DataType),
    X509IssuerSerial(Iso2X509IssuerSerialType),
}
