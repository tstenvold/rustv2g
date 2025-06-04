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
    pub costKind: Iso2CostKindType,
    pub amount: u32,
    pub amountMultiplier: Option<i8>,
}

impl Default for Iso2CostType {
    fn default() -> Self {
        Iso2CostType {
            costKind: ISO2_COST_KIND_TYPE_UNKNOWN,
            amount: 0,
            amountMultiplier: None,
        }
    }
}
#[derive(Copy, Clone)]
pub struct Iso2TransformType {
    pub Algorithm: Iso2u8FixedBufType<65>,
    pub ANY: Option<Iso2u8FixedBufType<4>>,
    pub XPath: Option<Iso2u8FixedBufType<65>>,
}

impl Default for Iso2TransformType {
    fn default() -> Self {
        Iso2TransformType {
            Algorithm: Iso2u8FixedBufType::<65>::default(),
            ANY: None,
            XPath: None,
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
        Iso2u8FixedBufType {
            data: [0; N],
            len: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2IntervalType {
    pub _unused: i32,
}

impl Default for Iso2IntervalType {
    fn default() -> Self {
        Iso2IntervalType { _unused: 0 }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2TransformsType {
    pub Transform: Iso2TransformType,
}

impl Default for Iso2TransformsType {
    fn default() -> Self {
        Iso2TransformsType {
            Transform: Iso2TransformType::default(),
        }
    }
}
#[derive(Copy, Clone)]
pub struct Iso2DSAKeyValueType {
    pub P: Option<Iso2u8FixedBufType<350>>,
    pub Q: Option<Iso2u8FixedBufType<350>>,
    pub G: Option<Iso2u8FixedBufType<350>>,
    pub Y: Iso2u8FixedBufType<350>,
    pub J: Option<Iso2u8FixedBufType<350>>,
    pub Seed: Option<Iso2u8FixedBufType<350>>,
    pub PgenCounter: Option<Iso2u8FixedBufType<350>>,
}

impl Default for Iso2DSAKeyValueType {
    fn default() -> Self {
        Iso2DSAKeyValueType {
            P: None,
            Q: None,
            G: None,
            Y: Iso2u8FixedBufType::<350>::default(),
            J: None,
            Seed: None,
            PgenCounter: None,
        }
    }
}
#[derive(Copy, Clone)]
pub struct Iso2X509IssuerSerialType {
    pub X509IssuerName: Iso2u8FixedBufType<65>,
    pub X509SerialNumber: ExiSigned,
}

impl Default for Iso2X509IssuerSerialType {
    fn default() -> Self {
        Iso2X509IssuerSerialType {
            X509IssuerName: Iso2u8FixedBufType::<65>::default(),
            X509SerialNumber: ExiSigned::default(),
        }
    }
}
#[derive(Copy, Clone)]
pub struct Iso2RelativeTimeIntervalType {
    pub start: u32,
    pub duration: Option<u32>,
}

impl Default for Iso2RelativeTimeIntervalType {
    fn default() -> Self {
        Iso2RelativeTimeIntervalType {
            start: 0,
            duration: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2DigestMethodType {
    pub Algorithm: Iso2u8FixedBufType<65>,
    pub ANY: Option<Iso2u8FixedBufType<4>>,
}

impl Default for Iso2DigestMethodType {
    fn default() -> Self {
        Iso2DigestMethodType {
            Algorithm: Iso2u8FixedBufType::<65>::default(),
            ANY: None,
        }
    }
}
#[derive(Copy, Clone)]
pub struct Iso2RSAKeyValueType {
    pub Modulus: Iso2u8FixedBufType<350>,
    pub Exponent: Iso2u8FixedBufType<350>,
}

impl Default for Iso2RSAKeyValueType {
    fn default() -> Self {
        Iso2RSAKeyValueType {
            Modulus: Iso2u8FixedBufType::<350>::default(),
            Exponent: Iso2u8FixedBufType::<350>::default(),
        }
    }
}
#[derive(Copy, Clone)]
pub struct Iso2CanonicalizationMethodType {
    pub Algorithm: Iso2u8FixedBufType<65>,
    pub ANY: Option<Iso2u8FixedBufType<4>>,
}

impl Default for Iso2CanonicalizationMethodType {
    fn default() -> Self {
        Iso2CanonicalizationMethodType {
            Algorithm: Iso2u8FixedBufType::<65>::default(),
            ANY: None,
        }
    }
}
#[derive(Copy, Clone)]
pub struct Iso2SignatureMethodType {
    pub Algorithm: Iso2u8FixedBufType<65>,
    pub HMACOutputLength: Option<ExiSigned>,
    pub ANY: Option<Iso2u8FixedBufType<4>>,
}

impl Default for Iso2SignatureMethodType {
    fn default() -> Self {
        Iso2SignatureMethodType {
            Algorithm: Iso2u8FixedBufType::<65>::default(),
            HMACOutputLength: None,
            ANY: None,
        }
    }
}
#[derive(Copy, Clone)]
pub struct Iso2KeyValueType {
    pub DSAKeyValue: Option<Iso2DSAKeyValueType>,
    pub RSAKeyValue: Option<Iso2RSAKeyValueType>,
    pub ANY: Option<Iso2u8FixedBufType<4>>,
}

impl Default for Iso2KeyValueType {
    fn default() -> Self {
        Iso2KeyValueType {
            DSAKeyValue: None,
            RSAKeyValue: None,
            ANY: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2PhysicalValueType {
    pub Multiplier: i8,
    pub Unit: Iso2UnitSymbolType,
    pub Value: i16,
}

impl Default for Iso2PhysicalValueType {
    fn default() -> Self {
        Iso2PhysicalValueType {
            Multiplier: 0,
            Unit: ISO2_UNIT_SYMBOL_TYPE_UNKNOWN,
            Value: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ConsumptionCostType {
    pub startValue: Iso2PhysicalValueType,
    pub Cost: Iso2CostArrayType,
}

impl Default for Iso2ConsumptionCostType {
    fn default() -> Self {
        Iso2ConsumptionCostType {
            startValue: Iso2PhysicalValueType::default(),
            Cost: Iso2CostArrayType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2CostArrayType {
    pub array: [Iso2CostType; 3],
    pub arrayLen: usize,
}

impl Default for Iso2CostArrayType {
    fn default() -> Self {
        Iso2CostArrayType {
            array: [Iso2CostType::default(); 3],
            arrayLen: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2PMaxScheduleEntryType {
    pub RelativeTimeInterval: Option<Iso2RelativeTimeIntervalType>,
    pub TimeInterval: Option<Iso2IntervalType>,
    pub PMax: Iso2PhysicalValueType,
}

impl Default for Iso2PMaxScheduleEntryType {
    fn default() -> Self {
        Iso2PMaxScheduleEntryType {
            RelativeTimeInterval: None,
            TimeInterval: None,
            PMax: Iso2PhysicalValueType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2SalesTariffEntryType {
    pub RelativeTimeInterval: Option<Iso2RelativeTimeIntervalType>,
    pub TimeInterval: Option<Iso2IntervalType>,
    pub EPriceLevel: Option<u8>,
    pub ConsumptionCost: Iso2ConsumptionCostArrayType,
}

impl Default for Iso2SalesTariffEntryType {
    fn default() -> Self {
        Iso2SalesTariffEntryType {
            RelativeTimeInterval: None,
            TimeInterval: None,
            EPriceLevel: None,
            ConsumptionCost: Iso2ConsumptionCostArrayType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ConsumptionCostArrayType {
    pub array: [Iso2ConsumptionCostType; 3],
    pub arrayLen: usize,
}

impl Default for Iso2ConsumptionCostArrayType {
    fn default() -> Self {
        Iso2ConsumptionCostArrayType {
            array: [Iso2ConsumptionCostType::default(); 3],
            arrayLen: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ParameterType {
    pub Name: Iso2u8FixedBufType<65>,
    pub boolValue: Option<i32>,
    pub byteValue: Option<i8>,
    pub shortValue: Option<i16>,
    pub intValue: Option<i32>,
    pub physicalValue: Option<Iso2PhysicalValueType>,
    pub stringValue: Option<Iso2u8FixedBufType<65>>,
}

impl Default for Iso2ParameterType {
    fn default() -> Self {
        Iso2ParameterType {
            Name: Iso2u8FixedBufType::<65>::default(),
            boolValue: None,
            byteValue: None,
            shortValue: None,
            intValue: None,
            physicalValue: None,
            stringValue: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2PMaxScheduleType {
    pub PMaxScheduleEntry: Iso2PMaxScheduleEntryArrayType,
}

impl Default for Iso2PMaxScheduleType {
    fn default() -> Self {
        Iso2PMaxScheduleType {
            PMaxScheduleEntry: Iso2PMaxScheduleEntryArrayType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2PMaxScheduleEntryArrayType {
    pub array: [Iso2PMaxScheduleEntryType; 12],
    pub arrayLen: usize,
}

impl Default for Iso2PMaxScheduleEntryArrayType {
    fn default() -> Self {
        Iso2PMaxScheduleEntryArrayType {
            array: [Iso2PMaxScheduleEntryType::default(); 12],
            arrayLen: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ReferenceType {
    pub Id: Option<Iso2u8FixedBufType<65>>,
    pub Type: Option<Iso2u8FixedBufType<65>>,
    pub URI: Option<Iso2u8FixedBufType<65>>,
    pub Transforms: Option<Iso2TransformsType>,
    pub DigestMethod: Iso2DigestMethodType,
    pub DigestValue: Option<Iso2u8FixedBufType<350>>,
}

impl Default for Iso2ReferenceType {
    fn default() -> Self {
        Iso2ReferenceType {
            Id: None,
            Type: None,
            URI: None,
            Transforms: None,
            DigestMethod: Iso2DigestMethodType::default(),
            DigestValue: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2RetrievalMethodType {
    pub Type: Option<Iso2u8FixedBufType<65>>,
    pub URI: Option<Iso2u8FixedBufType<65>>,
    pub Transforms: Option<Iso2TransformsType>,
}

impl Default for Iso2RetrievalMethodType {
    fn default() -> Self {
        Iso2RetrievalMethodType {
            Type: None,
            URI: None,
            Transforms: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2SalesTariffType {
    pub Id: Option<Iso2u8FixedBufType<65>>,
    pub SalesTariffID: u8,
    pub SalesTariffDescription: Option<Iso2u8FixedBufType<33>>,
    pub NumEPriceLevels: Option<u8>,
    pub SalesTariffEntry: Iso2SalesTariffEntryArrayType,
}

impl Default for Iso2SalesTariffType {
    fn default() -> Self {
        Iso2SalesTariffType {
            Id: None,
            SalesTariffID: 0,
            SalesTariffDescription: None,
            NumEPriceLevels: None,
            SalesTariffEntry: Iso2SalesTariffEntryArrayType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2SalesTariffEntryArrayType {
    pub array: [Iso2SalesTariffEntryType; 12],
    pub arrayLen: usize,
}

impl Default for Iso2SalesTariffEntryArrayType {
    fn default() -> Self {
        Iso2SalesTariffEntryArrayType {
            array: [Iso2SalesTariffEntryType::default(); 12],
            arrayLen: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2X509DataType {
    pub X509IssuerSerial: Option<Iso2X509IssuerSerialType>,
    pub X509SKI: Option<Iso2u8FixedBufType<350>>,
    pub X509SubjectName: Option<Iso2u8FixedBufType<65>>,
    pub X509Certificate: Option<Iso2u8FixedBufType<350>>,
    pub X509CRL: Option<Iso2u8FixedBufType<350>>,
    pub ANY: Option<Iso2u8FixedBufType<4>>,
}

impl Default for Iso2X509DataType {
    fn default() -> Self {
        Iso2X509DataType {
            X509IssuerSerial: None,
            X509SKI: None,
            X509SubjectName: None,
            X509Certificate: None,
            X509CRL: None,
            ANY: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2PGPDataType {
    pub PGPComponent: Iso2PGPComponentType,
}

#[derive(Copy, Clone)]
pub enum Iso2PGPComponentType {
    Choice1(PGPChoice1Type),
    Choice2(PGPChoice2Type),
}

#[derive(Copy, Clone)]
pub struct PGPChoice2Type {
    pub PGPKeyPacket: Iso2u8FixedBufType<350>,
    pub ANY: Option<Iso2u8FixedBufType<4>>,
}

impl Default for PGPChoice2Type {
    fn default() -> Self {
        PGPChoice2Type {
            PGPKeyPacket: Iso2u8FixedBufType::<350>::default(),
            ANY: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct PGPChoice1Type {
    pub PGPKeyID: Iso2u8FixedBufType<350>,
    pub PGPKeyPacket: Option<Iso2u8FixedBufType<350>>,
    pub ANY: Option<Iso2u8FixedBufType<4>>,
}

impl Default for PGPChoice1Type {
    fn default() -> Self {
        PGPChoice1Type {
            PGPKeyID: Iso2u8FixedBufType::<350>::default(),
            PGPKeyPacket: None,
            ANY: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2SPKIDataType {
    pub SPKISexp: Iso2u8FixedBufType<350>,
    pub ANY: Option<Iso2u8FixedBufType<4>>,
}

impl Default for Iso2SPKIDataType {
    fn default() -> Self {
        Iso2SPKIDataType {
            SPKISexp: Iso2u8FixedBufType::<350>::default(),
            ANY: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2SignedInfoType {
    pub Id: Option<Iso2u8FixedBufType<65>>,
    pub CanonicalizationMethod: Iso2CanonicalizationMethodType,
    pub SignatureMethod: Iso2SignatureMethodType,
    pub Reference: [Iso2ReferenceType; 4],
    pub ReferenceLen: usize,
}

impl Default for Iso2SignedInfoType {
    fn default() -> Self {
        Iso2SignedInfoType {
            Id: None,
            CanonicalizationMethod: Iso2CanonicalizationMethodType::default(),
            SignatureMethod: Iso2SignatureMethodType::default(),
            Reference: Default::default(),
            ReferenceLen: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ProfileEntryType {
    pub ChargingProfileEntryStart: u32,
    pub ChargingProfileEntryMaxPower: Iso2PhysicalValueType,
    pub ChargingProfileEntryMaxNumberOfPhasesInUse: Option<i8>,
}

impl Default for Iso2ProfileEntryType {
    fn default() -> Self {
        Iso2ProfileEntryType {
            ChargingProfileEntryStart: 0,
            ChargingProfileEntryMaxPower: Iso2PhysicalValueType::default(),
            ChargingProfileEntryMaxNumberOfPhasesInUse: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2DCEVStatusType {
    pub EVReady: i32,
    pub EVErrorCode: Iso2DcEvErrorCodeType,
    pub EVRESSSOC: i8,
}

impl Default for Iso2DCEVStatusType {
    fn default() -> Self {
        Iso2DCEVStatusType {
            EVReady: 0,
            EVErrorCode: ISO2_DC_EVERROR_CODE_TYPE_UNKNOWN,
            EVRESSSOC: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ParameterSetType {
    pub ParameterSetID: i16,
    pub Parameter: Iso2ParameterArrayType,
}

impl Default for Iso2ParameterSetType {
    fn default() -> Self {
        Iso2ParameterSetType {
            ParameterSetID: 0,
            Parameter: Iso2ParameterArrayType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ParameterArrayType {
    pub array: [Iso2ParameterType; 16],
    pub arrayLen: usize,
}

impl Default for Iso2ParameterArrayType {
    fn default() -> Self {
        Iso2ParameterArrayType {
            array: [Iso2ParameterType::default(); 16],
            arrayLen: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2SAScheduleTupleType {
    pub SAScheduleTupleID: u8,
    pub PMaxSchedule: Iso2PMaxScheduleType,
    pub SalesTariff: Option<Iso2SalesTariffType>,
}

impl Default for Iso2SAScheduleTupleType {
    fn default() -> Self {
        Iso2SAScheduleTupleType {
            SAScheduleTupleID: 0,
            PMaxSchedule: Iso2PMaxScheduleType::default(),
            SalesTariff: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2SelectedServiceType {
    pub ServiceID: u16,
    pub ParameterSetID: Option<i16>,
}

impl Default for Iso2SelectedServiceType {
    fn default() -> Self {
        Iso2SelectedServiceType {
            ServiceID: 0,
            ParameterSetID: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ServiceType {
    pub ServiceID: u16,
    pub ServiceName: Option<Iso2u8FixedBufType<33>>,
    pub ServiceCategory: Iso2ServiceCategoryType,
    pub ServiceScope: Option<Iso2u8FixedBufType<65>>,
    pub FreeService: i32,
}

impl Default for Iso2ServiceType {
    fn default() -> Self {
        Iso2ServiceType {
            ServiceID: 0,
            ServiceName: None,
            ServiceCategory: ISO2_SERVICE_CATEGORY_TYPE_UNKNOWN,
            ServiceScope: None,
            FreeService: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2SignatureValueType {
    pub Id: Option<Iso2u8FixedBufType<65>>,
    pub CONTENT: Iso2u8FixedBufType<65>,
}

impl Default for Iso2SignatureValueType {
    fn default() -> Self {
        Iso2SignatureValueType {
            Id: None,
            CONTENT: Iso2u8FixedBufType::<65>::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2SubCertificatesType {
    pub Certificate: Iso2SubCertificatesArrayType,
}

impl Default for Iso2SubCertificatesType {
    fn default() -> Self {
        Iso2SubCertificatesType {
            Certificate: Iso2SubCertificatesArrayType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2SubCertificatesArrayType {
    pub array: [Iso2u8FixedBufType<800>; 4],
    pub arrayLen: usize,
}

impl Default for Iso2SubCertificatesArrayType {
    fn default() -> Self {
        Iso2SubCertificatesArrayType {
            array: [Iso2u8FixedBufType::<800>::default(); 4],
            arrayLen: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2KeyInfoType {
    pub Id: Option<Iso2u8FixedBufType<65>>,
    pub KeyName: Option<Iso2u8FixedBufType<65>>,
    pub KeyValue: Option<Iso2KeyValueType>,
    pub RetrievalMethod: Option<Iso2RetrievalMethodType>,
    pub X509Data: Option<Iso2X509DataType>,
    pub PGPData: Option<Iso2PGPDataType>,
    pub SPKIData: Option<Iso2SPKIDataType>,
    pub MgmtData: Option<Iso2u8FixedBufType<65>>,
    pub ANY: Option<Iso2u8FixedBufType<4>>,
}

impl Default for Iso2KeyInfoType {
    fn default() -> Self {
        Iso2KeyInfoType {
            Id: None,
            KeyName: None,
            KeyValue: None,
            RetrievalMethod: None,
            X509Data: None,
            PGPData: None,
            SPKIData: None,
            MgmtData: None,
            ANY: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ObjectType {
    pub Encoding: Option<Iso2u8FixedBufType<65>>,
    pub Id: Option<Iso2u8FixedBufType<65>>,
    pub MimeType: Option<Iso2u8FixedBufType<65>>,
    pub ANY: Option<Iso2u8FixedBufType<4>>,
}

impl Default for Iso2ObjectType {
    fn default() -> Self {
        Iso2ObjectType {
            Encoding: None,
            Id: None,
            MimeType: None,
            ANY: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2SupportedEnergyTransferModeType {
    pub EnergyTransferMode: Iso2EnergyTransferModeArrayType,
}

impl Default for Iso2SupportedEnergyTransferModeType {
    fn default() -> Self {
        Iso2SupportedEnergyTransferModeType {
            EnergyTransferMode: Iso2EnergyTransferModeArrayType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2EnergyTransferModeArrayType {
    pub array: [Iso2EnergyTransferModeType; 6],
    pub arrayLen: usize,
}

impl Default for Iso2EnergyTransferModeArrayType {
    fn default() -> Self {
        Iso2EnergyTransferModeArrayType {
            array: [ISO2_ENERGY_TRANSFER_MODE_TYPE_UNKNOWN; 6],
            arrayLen: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2CertificateChainType {
    pub Id: Option<Iso2u8FixedBufType<65>>,
    pub Certificate: Iso2u8FixedBufType<800>,
    pub SubCertificates: Option<Iso2SubCertificatesType>,
}

impl Default for Iso2CertificateChainType {
    fn default() -> Self {
        Iso2CertificateChainType {
            Id: None,
            Certificate: Iso2u8FixedBufType::<800>::default(),
            SubCertificates: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2BodyBaseType {
    pub _unused: i32,
}

impl Default for Iso2BodyBaseType {
    fn default() -> Self {
        Iso2BodyBaseType { _unused: 0 }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2NotificationType {
    pub FaultCode: Iso2FaultCodeType,
    pub FaultMsg: Option<Iso2u8FixedBufType<65>>,
}

impl Default for Iso2NotificationType {
    fn default() -> Self {
        Iso2NotificationType {
            FaultCode: ISO2_FAULT_CODE_TYPE_UNKNOWN,
            FaultMsg: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2DCEVSEStatusType {
    pub NotificationMaxDelay: u16,
    pub EVSENotification: Iso2EvseNotificationType,
    pub EVSEIsolationStatus: Option<Iso2IsolationLevelType>,
    pub EVSEStatusCode: Iso2DcEvseStatusCodeType,
}

impl Default for Iso2DCEVSEStatusType {
    fn default() -> Self {
        Iso2DCEVSEStatusType {
            NotificationMaxDelay: 0,
            EVSENotification: ISO2_EVSENOTIFICATION_TYPE_UNKNOWN,
            EVSEIsolationStatus: None,
            EVSEStatusCode: ISO2_DC_EVSESTATUS_CODE_TYPE_UNKNOWN,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2SelectedServiceListType {
    pub SelectedService: Iso2SelectedServiceArrayType,
}

impl Default for Iso2SelectedServiceListType {
    fn default() -> Self {
        Iso2SelectedServiceListType {
            SelectedService: Iso2SelectedServiceArrayType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2SelectedServiceArrayType {
    pub array: [Iso2SelectedServiceType; 16],
    pub arrayLen: usize,
}

impl Default for Iso2SelectedServiceArrayType {
    fn default() -> Self {
        Iso2SelectedServiceArrayType {
            array: [Iso2SelectedServiceType::default(); 16],
            arrayLen: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2PaymentOptionListType {
    pub PaymentOption: Iso2PaymentOptionArrayType,
}

impl Default for Iso2PaymentOptionListType {
    fn default() -> Self {
        Iso2PaymentOptionListType {
            PaymentOption: Iso2PaymentOptionArrayType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2PaymentOptionArrayType {
    pub array: [Iso2PaymentOptionType; 2],
    pub arrayLen: usize,
}

impl Default for Iso2PaymentOptionArrayType {
    fn default() -> Self {
        Iso2PaymentOptionArrayType {
            array: [ISO2_PAYMENT_OPTION_TYPE_UNKNOWN; 2],
            arrayLen: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2SignatureType {
    pub Id: Option<Iso2u8FixedBufType<65>>,
    pub SignedInfo: Iso2SignedInfoType,
    pub SignatureValue: Iso2SignatureValueType,
    pub KeyInfo: Option<Iso2KeyInfoType>,
    pub Object: Option<Iso2ObjectType>,
}

impl Default for Iso2SignatureType {
    fn default() -> Self {
        Iso2SignatureType {
            Id: None,
            SignedInfo: Iso2SignedInfoType::default(),
            SignatureValue: Iso2SignatureValueType::default(),
            KeyInfo: None,
            Object: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ChargingProfileType {
    pub ProfileEntry: Iso2ProfileEntryArrayType,
}

impl Default for Iso2ChargingProfileType {
    fn default() -> Self {
        Iso2ChargingProfileType {
            ProfileEntry: Iso2ProfileEntryArrayType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ProfileEntryArrayType {
    pub array: [Iso2ProfileEntryType; 24],
    pub arrayLen: usize,
}

impl Default for Iso2ProfileEntryArrayType {
    fn default() -> Self {
        Iso2ProfileEntryArrayType {
            array: [Iso2ProfileEntryType::default(); 24],
            arrayLen: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ServiceParameterListType {
    pub ParameterSet: Iso2ParameterSetArrayType,
}

impl Default for Iso2ServiceParameterListType {
    fn default() -> Self {
        Iso2ServiceParameterListType {
            ParameterSet: Iso2ParameterSetArrayType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ParameterSetArrayType {
    pub array: [Iso2ParameterSetType; 5],
    pub arrayLen: usize,
}

impl Default for Iso2ParameterSetArrayType {
    fn default() -> Self {
        Iso2ParameterSetArrayType {
            array: [Iso2ParameterSetType::default(); 5],
            arrayLen: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ListOfRootCertificateIDsType {
    pub RootCertificateID: Iso2RootCertificateIDArrayType,
}

impl Default for Iso2ListOfRootCertificateIDsType {
    fn default() -> Self {
        Iso2ListOfRootCertificateIDsType {
            RootCertificateID: Iso2RootCertificateIDArrayType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2RootCertificateIDArrayType {
    pub array: [Iso2X509IssuerSerialType; 5],
    pub arrayLen: usize,
}

impl Default for Iso2RootCertificateIDArrayType {
    fn default() -> Self {
        Iso2RootCertificateIDArrayType {
            array: [Iso2X509IssuerSerialType::default(); 5],
            arrayLen: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ACEVChargeParameterType {
    pub DepartureTime: Option<u32>,
    pub EAmount: Iso2PhysicalValueType,
    pub EVMaxVoltage: Iso2PhysicalValueType,
    pub EVMaxCurrent: Iso2PhysicalValueType,
    pub EVMinCurrent: Iso2PhysicalValueType,
}

impl Default for Iso2ACEVChargeParameterType {
    fn default() -> Self {
        Iso2ACEVChargeParameterType {
            DepartureTime: None,
            EAmount: Iso2PhysicalValueType::default(),
            EVMaxVoltage: Iso2PhysicalValueType::default(),
            EVMaxCurrent: Iso2PhysicalValueType::default(),
            EVMinCurrent: Iso2PhysicalValueType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2DCEVChargeParameterType {
    pub DepartureTime: Option<u32>,
    pub DC_EVStatus: Iso2DCEVStatusType,
    pub EVMaximumCurrentLimit: Iso2PhysicalValueType,
    pub EVMaximumPowerLimit: Option<Iso2PhysicalValueType>,
    pub EVMaximumVoltageLimit: Iso2PhysicalValueType,
    pub EVEnergyCapacity: Option<Iso2PhysicalValueType>,
    pub EVEnergyRequest: Option<Iso2PhysicalValueType>,
    pub FullSOC: Option<i8>,
    pub BulkSOC: Option<i8>,
}

impl Default for Iso2DCEVChargeParameterType {
    fn default() -> Self {
        Iso2DCEVChargeParameterType {
            DepartureTime: None,
            DC_EVStatus: Iso2DCEVStatusType::default(),
            EVMaximumCurrentLimit: Iso2PhysicalValueType::default(),
            EVMaximumPowerLimit: None,
            EVMaximumVoltageLimit: Iso2PhysicalValueType::default(),
            EVEnergyCapacity: None,
            EVEnergyRequest: None,
            FullSOC: None,
            BulkSOC: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2EVChargeParameterType {
    pub DepartureTime: Option<u32>,
    pub AC_EVChargeParameter: Iso2ACEVChargeParameterType,
    pub DC_EVChargeParameter: Iso2DCEVChargeParameterType,
}

impl Default for Iso2EVChargeParameterType {
    fn default() -> Self {
        Iso2EVChargeParameterType {
            DepartureTime: None,
            AC_EVChargeParameter: Iso2ACEVChargeParameterType::default(),
            DC_EVChargeParameter: Iso2DCEVChargeParameterType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2SASchedulesType {
    pub _unused: i32,
}

#[derive(Copy, Clone)]
pub struct Iso2SAScheduleListType {
    pub SAScheduleTuple: Iso2SAScheduleTupleArrayType,
}

impl Default for Iso2SAScheduleListType {
    fn default() -> Self {
        Iso2SAScheduleListType {
            SAScheduleTuple: Iso2SAScheduleTupleArrayType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2SAScheduleTupleArrayType {
    pub array: [Iso2SAScheduleTupleType; 3],
    pub arrayLen: usize,
}

impl Default for Iso2SAScheduleTupleArrayType {
    fn default() -> Self {
        Iso2SAScheduleTupleArrayType {
            array: [Iso2SAScheduleTupleType::default(); 3],
            arrayLen: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ChargeServiceType {
    pub ServiceID: u16,
    pub ServiceName: Option<Iso2u8FixedBufType<33>>,
    pub ServiceCategory: Iso2ServiceCategoryType,
    pub ServiceScope: Option<Iso2u8FixedBufType<65>>,
    pub FreeService: i32,
    pub SupportedEnergyTransferMode: Iso2SupportedEnergyTransferModeType,
}

impl Default for Iso2ChargeServiceType {
    fn default() -> Self {
        Iso2ChargeServiceType {
            ServiceID: 0,
            ServiceName: None,
            ServiceCategory: ISO2_SERVICE_CATEGORY_TYPE_UNKNOWN,
            ServiceScope: None,
            FreeService: 0,
            SupportedEnergyTransferMode: Iso2SupportedEnergyTransferModeType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2EVPowerDeliveryParameterType {
    pub _unused: i32,
}

#[derive(Copy, Clone)]
pub struct Iso2DCEVPowerDeliveryParameterType {
    pub DC_EVStatus: Iso2DCEVStatusType,
    pub BulkChargingComplete: Option<i32>,
    pub ChargingComplete: i32,
}

impl Default for Iso2DCEVPowerDeliveryParameterType {
    fn default() -> Self {
        Iso2DCEVPowerDeliveryParameterType {
            DC_EVStatus: Iso2DCEVStatusType::default(),
            BulkChargingComplete: None,
            ChargingComplete: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ContractSignatureEncryptedPrivateKeyType {
    pub Id: Iso2u8FixedBufType<65>,
    pub CONTENT: Iso2u8FixedBufType<65>,
}

impl Default for Iso2ContractSignatureEncryptedPrivateKeyType {
    fn default() -> Self {
        Iso2ContractSignatureEncryptedPrivateKeyType {
            Id: Iso2u8FixedBufType::<65>::default(),
            CONTENT: Iso2u8FixedBufType::<65>::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2EVSEChargeParameterType {
    pub _unused: i32,
}

#[derive(Copy, Clone)]
pub struct Iso2DCEVSEChargeParameterType {
    pub DC_EVSEStatus: Iso2DCEVSEStatusType,
    pub EVSEMaximumCurrentLimit: Iso2PhysicalValueType,
    pub EVSEMaximumPowerLimit: Iso2PhysicalValueType,
    pub EVSEMaximumVoltageLimit: Iso2PhysicalValueType,
    pub EVSEMinimumCurrentLimit: Iso2PhysicalValueType,
    pub EVSEMinimumVoltageLimit: Iso2PhysicalValueType,
    pub EVSECurrentRegulationTolerance: Option<Iso2PhysicalValueType>,
    pub EVSEPeakCurrentRipple: Iso2PhysicalValueType,
    pub EVSEEnergyToBeDelivered: Option<Iso2PhysicalValueType>,
}

impl Default for Iso2DCEVSEChargeParameterType {
    fn default() -> Self {
        Iso2DCEVSEChargeParameterType {
            DC_EVSEStatus: Iso2DCEVSEStatusType::default(),
            EVSEMaximumCurrentLimit: Iso2PhysicalValueType::default(),
            EVSEMaximumPowerLimit: Iso2PhysicalValueType::default(),
            EVSEMaximumVoltageLimit: Iso2PhysicalValueType::default(),
            EVSEMinimumCurrentLimit: Iso2PhysicalValueType::default(),
            EVSEMinimumVoltageLimit: Iso2PhysicalValueType::default(),
            EVSECurrentRegulationTolerance: None,
            EVSEPeakCurrentRipple: Iso2PhysicalValueType::default(),
            EVSEEnergyToBeDelivered: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ServiceListType {
    pub Service: Iso2ServiceArrayType,
}

impl Default for Iso2ServiceListType {
    fn default() -> Self {
        Iso2ServiceListType {
            Service: Iso2ServiceArrayType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ServiceArrayType {
    pub array: [Iso2ServiceType; 8],
    pub arrayLen: usize,
}

impl Default for Iso2ServiceArrayType {
    fn default() -> Self {
        Iso2ServiceArrayType {
            array: [Iso2ServiceType::default(); 8],
            arrayLen: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2DiffieHellmanPublickeyType {
    pub Id: Iso2u8FixedBufType<65>,
    pub CONTENT: Iso2u8FixedBufType<65>,
}

impl Default for Iso2DiffieHellmanPublickeyType {
    fn default() -> Self {
        Iso2DiffieHellmanPublickeyType {
            Id: Iso2u8FixedBufType::<65>::default(),
            CONTENT: Iso2u8FixedBufType::<65>::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2EMAIDType {
    pub Id: Iso2u8FixedBufType<65>,
    pub CONTENT: Iso2u8FixedBufType<65>,
}

impl Default for Iso2EMAIDType {
    fn default() -> Self {
        Iso2EMAIDType {
            Id: Iso2u8FixedBufType::<65>::default(),
            CONTENT: Iso2u8FixedBufType::<65>::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ACEVSEStatusType {
    pub NotificationMaxDelay: u16,
    pub EVSENotification: Iso2EvseNotificationType,
    pub RCD: i32,
}

impl Default for Iso2ACEVSEStatusType {
    fn default() -> Self {
        Iso2ACEVSEStatusType {
            NotificationMaxDelay: 0,
            EVSENotification: ISO2_EVSENOTIFICATION_TYPE_UNKNOWN,
            RCD: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2EVSEStatusType {
    pub NotificationMaxDelay: u16,
    pub EVSENotification: Iso2EvseNotificationType,
    pub AC_EVSEStatus: Iso2ACEVSEStatusType,
    pub DC_EVSEStatus: Iso2DCEVSEStatusType,
}

impl Default for Iso2EVSEStatusType {
    fn default() -> Self {
        Iso2EVSEStatusType {
            NotificationMaxDelay: 0,
            EVSENotification: ISO2_EVSENOTIFICATION_TYPE_UNKNOWN,
            AC_EVSEStatus: Iso2ACEVSEStatusType::default(),
            DC_EVSEStatus: Iso2DCEVSEStatusType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ACEVSEChargeParameterType {
    pub AC_EVSEStatus: Iso2ACEVSEStatusType,
    pub EVSENominalVoltage: Iso2PhysicalValueType,
    pub EVSEMaxCurrent: Iso2PhysicalValueType,
}

impl Default for Iso2ACEVSEChargeParameterType {
    fn default() -> Self {
        Iso2ACEVSEChargeParameterType {
            AC_EVSEStatus: Iso2ACEVSEStatusType::default(),
            EVSENominalVoltage: Iso2PhysicalValueType::default(),
            EVSEMaxCurrent: Iso2PhysicalValueType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2MeterInfoType {
    pub MeterID: Iso2u8FixedBufType<33>,
    pub MeterReading: Option<u64>,
    pub SigMeterReading: Option<Iso2u8FixedBufType<64>>,
    pub MeterStatus: Option<i16>,
    pub TMeter: Option<i64>,
}

impl Default for Iso2MeterInfoType {
    fn default() -> Self {
        Iso2MeterInfoType {
            MeterID: Iso2u8FixedBufType::<33>::default(),
            MeterReading: None,
            SigMeterReading: None,
            MeterStatus: None,
            TMeter: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2MessageHeaderType {
    pub SessionID: Iso2u8FixedBufType<8>,
    pub Notification: Option<Iso2NotificationType>,
    pub Signature: Option<Iso2SignatureType>,
}

impl Default for Iso2MessageHeaderType {
    fn default() -> Self {
        Iso2MessageHeaderType {
            SessionID: Iso2u8FixedBufType::<8>::default(),
            Notification: None,
            Signature: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2PowerDeliveryReqType {
    pub ChargeProgress: Iso2ChargeProgressType,
    pub SAScheduleTupleID: u8,
    pub ChargingProfile: Option<Iso2ChargingProfileType>,
    pub DC_EVPowerDeliveryParameter: Option<Iso2DCEVPowerDeliveryParameterType>,
    pub EVPowerDeliveryParameter: Option<Iso2EVPowerDeliveryParameterType>,
}

impl Default for Iso2PowerDeliveryReqType {
    fn default() -> Self {
        Iso2PowerDeliveryReqType {
            ChargeProgress: ISO2_CHARGE_PROGRESS_TYPE_UNKNOWN,
            SAScheduleTupleID: 0,
            ChargingProfile: None,
            DC_EVPowerDeliveryParameter: None,
            EVPowerDeliveryParameter: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2CurrentDemandResType {
    pub ResponseCode: Iso2ResponseCodeType,
    pub DC_EVSEStatus: Iso2DCEVSEStatusType,
    pub EVSEPresentVoltage: Iso2PhysicalValueType,
    pub EVSEPresentCurrent: Iso2PhysicalValueType,
    pub EVSECurrentLimitAchieved: i32,
    pub EVSEVoltageLimitAchieved: i32,
    pub EVSEPowerLimitAchieved: i32,
    pub EVSEMaximumVoltageLimit: Option<Iso2PhysicalValueType>,
    pub EVSEMaximumCurrentLimit: Option<Iso2PhysicalValueType>,
    pub EVSEMaximumPowerLimit: Option<Iso2PhysicalValueType>,
    pub EVSEID: Iso2u8FixedBufType<38>,
    pub SAScheduleTupleID: u8,
    pub MeterInfo: Option<Iso2MeterInfoType>,
    pub ReceiptRequired: Option<i32>,
}

impl Default for Iso2CurrentDemandResType {
    fn default() -> Self {
        Iso2CurrentDemandResType {
            ResponseCode: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            DC_EVSEStatus: Iso2DCEVSEStatusType::default(),
            EVSEPresentVoltage: Iso2PhysicalValueType::default(),
            EVSEPresentCurrent: Iso2PhysicalValueType::default(),
            EVSECurrentLimitAchieved: 0,
            EVSEVoltageLimitAchieved: 0,
            EVSEPowerLimitAchieved: 0,
            EVSEMaximumVoltageLimit: None,
            EVSEMaximumCurrentLimit: None,
            EVSEMaximumPowerLimit: None,
            EVSEID: Iso2u8FixedBufType::<38>::default(),
            SAScheduleTupleID: 0,
            MeterInfo: None,
            ReceiptRequired: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ChargingStatusResType {
    pub ResponseCode: Iso2ResponseCodeType,
    pub EVSEID: Iso2u8FixedBufType<38>,
    pub SAScheduleTupleID: u8,
    pub EVSEMaxCurrent: Option<Iso2PhysicalValueType>,
    pub MeterInfo: Option<Iso2MeterInfoType>,
    pub ReceiptRequired: Option<i32>,
    pub AC_EVSEStatus: Iso2ACEVSEStatusType,
}

impl Default for Iso2ChargingStatusResType {
    fn default() -> Self {
        Iso2ChargingStatusResType {
            ResponseCode: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            EVSEID: Iso2u8FixedBufType::<38>::default(),
            SAScheduleTupleID: 0,
            EVSEMaxCurrent: None,
            MeterInfo: None,
            ReceiptRequired: None,
            AC_EVSEStatus: Iso2ACEVSEStatusType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2AuthorizationReqType {
    pub Id: Option<Iso2u8FixedBufType<65>>,
    pub GenChallenge: Option<Iso2u8FixedBufType<16>>,
}

impl Default for Iso2AuthorizationReqType {
    fn default() -> Self {
        Iso2AuthorizationReqType {
            Id: None,
            GenChallenge: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2PreChargeReqType {
    pub DC_EVStatus: Option<Iso2DCEVStatusType>,
    pub EVTargetVoltage: Option<Iso2PhysicalValueType>,
    pub EVTargetCurrent: Option<Iso2PhysicalValueType>,
}

impl Default for Iso2PreChargeReqType {
    fn default() -> Self {
        Iso2PreChargeReqType {
            DC_EVStatus: None,
            EVTargetVoltage: None,
            EVTargetCurrent: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ServiceDetailResType {
    pub ResponseCode: Iso2ResponseCodeType,
    pub ServiceID: u16,
    pub ServiceParameterList: Option<Iso2ServiceParameterListType>,
}

impl Default for Iso2ServiceDetailResType {
    fn default() -> Self {
        Iso2ServiceDetailResType {
            ResponseCode: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            ServiceID: 0,
            ServiceParameterList: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2PaymentServiceSelectionResType {
    pub ResponseCode: Iso2ResponseCodeType,
}

impl Default for Iso2PaymentServiceSelectionResType {
    fn default() -> Self {
        Iso2PaymentServiceSelectionResType {
            ResponseCode: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2CertificateUpdateReqType {
    pub Id: Iso2u8FixedBufType<65>,
    pub ContractSignatureCertChain: Iso2CertificateChainType,
    pub eMaid: Iso2u8FixedBufType<16>,
    pub ListOfRootCertificateIDs: Iso2ListOfRootCertificateIDsType,
}

impl Default for Iso2CertificateUpdateReqType {
    fn default() -> Self {
        Iso2CertificateUpdateReqType {
            Id: Iso2u8FixedBufType::<65>::default(),
            ContractSignatureCertChain: Iso2CertificateChainType::default(),
            eMaid: Iso2u8FixedBufType::<16>::default(),
            ListOfRootCertificateIDs: Iso2ListOfRootCertificateIDsType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2SessionSetupResType {
    pub ResponseCode: Iso2ResponseCodeType,
    pub EVSEID: Iso2u8FixedBufType<38>,
    pub EVSETimeStamp: Option<i64>,
}

impl Default for Iso2SessionSetupResType {
    fn default() -> Self {
        Iso2SessionSetupResType {
            ResponseCode: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            EVSEID: Iso2u8FixedBufType::<38>::default(),
            EVSETimeStamp: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2CertificateInstallationReqType {
    pub Id: Iso2u8FixedBufType<65>,
    pub OEMProvisioningCert: Iso2u8FixedBufType<800>,
    pub ListOfRootCertificateIDs: Iso2ListOfRootCertificateIDsType,
}

impl Default for Iso2CertificateInstallationReqType {
    fn default() -> Self {
        Iso2CertificateInstallationReqType {
            Id: Iso2u8FixedBufType::<65>::default(),
            OEMProvisioningCert: Iso2u8FixedBufType::<800>::default(),
            ListOfRootCertificateIDs: Iso2ListOfRootCertificateIDsType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2CertificateInstallationResType {
    pub ResponseCode: Iso2ResponseCodeType,
    pub SAProvisioningCertificateChain: Iso2CertificateChainType,
    pub ContractSignatureCertChain: Iso2CertificateChainType,
    pub ContractSignatureEncryptedPrivateKey: Iso2ContractSignatureEncryptedPrivateKeyType,
    pub DHpublickey: Iso2DiffieHellmanPublickeyType,
    pub eMaid: Iso2EMAIDType,
}

impl Default for Iso2CertificateInstallationResType {
    fn default() -> Self {
        Iso2CertificateInstallationResType {
            ResponseCode: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            SAProvisioningCertificateChain: Iso2CertificateChainType::default(),
            ContractSignatureCertChain: Iso2CertificateChainType::default(),
            ContractSignatureEncryptedPrivateKey:
                Iso2ContractSignatureEncryptedPrivateKeyType::default(),
            DHpublickey: Iso2DiffieHellmanPublickeyType::default(),
            eMaid: Iso2EMAIDType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2WeldingDetectionResType {
    pub ResponseCode: Iso2ResponseCodeType,
    pub DC_EVSEStatus: Iso2DCEVSEStatusType,
    pub EVSEPresentVoltage: Iso2PhysicalValueType,
}

impl Default for Iso2WeldingDetectionResType {
    fn default() -> Self {
        Iso2WeldingDetectionResType {
            ResponseCode: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            DC_EVSEStatus: Iso2DCEVSEStatusType::default(),
            EVSEPresentVoltage: Iso2PhysicalValueType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2CurrentDemandReqType {
    pub DC_EVStatus: Iso2DCEVStatusType,
    pub EVTargetCurrent: Iso2PhysicalValueType,
    pub EVMaximumVoltageLimit: Option<Iso2PhysicalValueType>,
    pub EVMaximumCurrentLimit: Option<Iso2PhysicalValueType>,
    pub EVMaximumPowerLimit: Option<Iso2PhysicalValueType>,
    pub BulkChargingComplete: Option<i32>,
    pub ChargingComplete: i32,
    pub RemainingTimeToFullSoC: Option<Iso2PhysicalValueType>,
    pub RemainingTimeToBulkSoC: Option<Iso2PhysicalValueType>,
    pub EVTargetVoltage: Iso2PhysicalValueType,
}

impl Default for Iso2CurrentDemandReqType {
    fn default() -> Self {
        Iso2CurrentDemandReqType {
            DC_EVStatus: Iso2DCEVStatusType::default(),
            EVTargetCurrent: Iso2PhysicalValueType::default(),
            EVMaximumVoltageLimit: None,
            EVMaximumCurrentLimit: None,
            EVMaximumPowerLimit: None,
            BulkChargingComplete: None,
            ChargingComplete: 0,
            RemainingTimeToFullSoC: None,
            RemainingTimeToBulkSoC: None,
            EVTargetVoltage: Iso2PhysicalValueType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2PreChargeResType {
    pub ResponseCode: Iso2ResponseCodeType,
    pub DC_EVSEStatus: Iso2DCEVSEStatusType,
    pub EVSEPresentVoltage: Iso2PhysicalValueType,
}

impl Default for Iso2PreChargeResType {
    fn default() -> Self {
        Iso2PreChargeResType {
            ResponseCode: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            DC_EVSEStatus: Iso2DCEVSEStatusType::default(),
            EVSEPresentVoltage: Iso2PhysicalValueType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2CertificateUpdateResType {
    pub ResponseCode: Iso2ResponseCodeType,
    pub SAProvisioningCertificateChain: Iso2CertificateChainType,
    pub ContractSignatureCertChain: Iso2CertificateChainType,
    pub ContractSignatureEncryptedPrivateKey: Iso2ContractSignatureEncryptedPrivateKeyType,
    pub DHpublickey: Iso2DiffieHellmanPublickeyType,
    pub eMaid: Iso2EMAIDType,
    pub RetryCounter: Option<i16>,
}

impl Default for Iso2CertificateUpdateResType {
    fn default() -> Self {
        Iso2CertificateUpdateResType {
            ResponseCode: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            SAProvisioningCertificateChain: Iso2CertificateChainType::default(),
            ContractSignatureCertChain: Iso2CertificateChainType::default(),
            ContractSignatureEncryptedPrivateKey:
                Iso2ContractSignatureEncryptedPrivateKeyType::default(),
            DHpublickey: Iso2DiffieHellmanPublickeyType::default(),
            eMaid: Iso2EMAIDType::default(),
            RetryCounter: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2MeteringReceiptReqType {
    pub Id: Option<Iso2u8FixedBufType<65>>,
    pub SessionID: Iso2u8FixedBufType<8>,
    pub SAScheduleTupleID: Option<u8>,
    pub MeterInfo: Iso2MeterInfoType,
}

impl Default for Iso2MeteringReceiptReqType {
    fn default() -> Self {
        Iso2MeteringReceiptReqType {
            Id: None,
            SessionID: Iso2u8FixedBufType::<8>::default(),
            SAScheduleTupleID: None,
            MeterInfo: Iso2MeterInfoType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ChargingStatusReqType {
    pub _unused: i32,
}

impl Default for Iso2ChargingStatusReqType {
    fn default() -> Self {
        Iso2ChargingStatusReqType { _unused: 0 }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2SessionStopResType {
    pub ResponseCode: Iso2ResponseCodeType,
}

impl Default for Iso2SessionStopResType {
    fn default() -> Self {
        Iso2SessionStopResType {
            ResponseCode: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ChargeParameterDiscoveryReqType {
    pub MaxEntriesSAScheduleTuple: Option<u16>,
    pub RequestedEnergyTransferMode: Iso2EnergyTransferModeType,
    pub AC_EVChargeParameter: Option<Iso2ACEVChargeParameterType>,
    pub DC_EVChargeParameter: Option<Iso2DCEVChargeParameterType>,
    pub EVChargeParameter: Option<Iso2EVChargeParameterType>,
}

impl Default for Iso2ChargeParameterDiscoveryReqType {
    fn default() -> Self {
        Iso2ChargeParameterDiscoveryReqType {
            MaxEntriesSAScheduleTuple: None,
            RequestedEnergyTransferMode: ISO2_ENERGY_TRANSFER_MODE_TYPE_UNKNOWN,
            AC_EVChargeParameter: None,
            DC_EVChargeParameter: None,
            EVChargeParameter: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2CableCheckReqType {
    pub DC_EVStatus: Iso2DCEVStatusType,
}

impl Default for Iso2CableCheckReqType {
    fn default() -> Self {
        Iso2CableCheckReqType {
            DC_EVStatus: Iso2DCEVStatusType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2WeldingDetectionReqType {
    pub DC_EVStatus: Iso2DCEVStatusType,
}

impl Default for Iso2WeldingDetectionReqType {
    fn default() -> Self {
        Iso2WeldingDetectionReqType {
            DC_EVStatus: Iso2DCEVStatusType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2PowerDeliveryResType {
    pub ResponseCode: Iso2ResponseCodeType,
    pub AC_EVSEStatus: Option<Iso2ACEVSEStatusType>,
    pub DC_EVSEStatus: Option<Iso2DCEVSEStatusType>,
    pub EVSEStatus: Option<Iso2EVSEStatusType>,
}

impl Default for Iso2PowerDeliveryResType {
    fn default() -> Self {
        Iso2PowerDeliveryResType {
            ResponseCode: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            AC_EVSEStatus: None,
            DC_EVSEStatus: None,
            EVSEStatus: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ChargeParameterDiscoveryResType {
    pub ResponseCode: Iso2ResponseCodeType,
    pub EVSEProcessing: Iso2EvseProcessingType,
    pub SAScheduleList: Option<Iso2SAScheduleListType>,
    pub SASchedules: Option<Iso2SASchedulesType>,
    pub AC_EVSEChargeParameter: Option<Iso2ACEVSEChargeParameterType>,
    pub DC_EVSEChargeParameter: Option<Iso2DCEVSEChargeParameterType>,
    pub EVSEChargeParameter: Option<Iso2EVSEChargeParameterType>,
}

impl Default for Iso2ChargeParameterDiscoveryResType {
    fn default() -> Self {
        Iso2ChargeParameterDiscoveryResType {
            ResponseCode: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            EVSEProcessing: ISO2_EVSEPROCESSING_TYPE_UNKNOWN,
            SAScheduleList: None,
            SASchedules: None,
            AC_EVSEChargeParameter: None,
            DC_EVSEChargeParameter: None,
            EVSEChargeParameter: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2PaymentServiceSelectionReqType {
    pub SelectedPaymentOption: Iso2PaymentOptionType,
    pub SelectedServiceList: Iso2SelectedServiceListType,
}

impl Default for Iso2PaymentServiceSelectionReqType {
    fn default() -> Self {
        Iso2PaymentServiceSelectionReqType {
            SelectedPaymentOption: ISO2_PAYMENT_OPTION_TYPE_UNKNOWN,
            SelectedServiceList: Iso2SelectedServiceListType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2MeteringReceiptResType {
    pub ResponseCode: Iso2ResponseCodeType,
    pub AC_EVSEStatus: Option<Iso2ACEVSEStatusType>,
    pub DC_EVSEStatus: Option<Iso2DCEVSEStatusType>,
    pub EVSEStatus: Option<Iso2EVSEStatusType>,
}

impl Default for Iso2MeteringReceiptResType {
    fn default() -> Self {
        Iso2MeteringReceiptResType {
            ResponseCode: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            AC_EVSEStatus: None,
            DC_EVSEStatus: None,
            EVSEStatus: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2CableCheckResType {
    pub ResponseCode: Iso2ResponseCodeType,
    pub DC_EVSEStatus: Option<Iso2DCEVSEStatusType>,
    pub EVSEProcessing: Iso2EvseProcessingType,
}

impl Default for Iso2CableCheckResType {
    fn default() -> Self {
        Iso2CableCheckResType {
            ResponseCode: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            DC_EVSEStatus: None,
            EVSEProcessing: ISO2_EVSEPROCESSING_TYPE_UNKNOWN,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ServiceDiscoveryResType {
    pub ResponseCode: Iso2ResponseCodeType,
    pub PaymentOptionList: Iso2PaymentOptionListType,
    pub ChargeService: Iso2ChargeServiceType,
    pub ServiceList: Option<Iso2ServiceListType>,
}

impl Default for Iso2ServiceDiscoveryResType {
    fn default() -> Self {
        Iso2ServiceDiscoveryResType {
            ResponseCode: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            PaymentOptionList: Iso2PaymentOptionListType::default(),
            ChargeService: Iso2ChargeServiceType::default(),
            ServiceList: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ServiceDetailReqType {
    pub ServiceID: u16,
}

impl Default for Iso2ServiceDetailReqType {
    fn default() -> Self {
        Iso2ServiceDetailReqType { ServiceID: 0 }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2SessionSetupReqType {
    pub EVCCID: Iso2u8FixedBufType<6>,
}

impl Default for Iso2SessionSetupReqType {
    fn default() -> Self {
        Iso2SessionSetupReqType {
            EVCCID: Iso2u8FixedBufType::<6>::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2SessionStopReqType {
    pub ChargingSession: Iso2ChargingSessionType,
}

impl Default for Iso2SessionStopReqType {
    fn default() -> Self {
        Iso2SessionStopReqType {
            ChargingSession: ISO2_CHARGING_SESSION_TYPE_UNKNOWN,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2ServiceDiscoveryReqType {
    pub ServiceScope: Option<Iso2u8FixedBufType<65>>,
    pub ServiceCategory: Option<Iso2ServiceCategoryType>,
}

impl Default for Iso2ServiceDiscoveryReqType {
    fn default() -> Self {
        Iso2ServiceDiscoveryReqType {
            ServiceScope: None,
            ServiceCategory: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2AuthorizationResType {
    pub ResponseCode: Iso2ResponseCodeType,
    pub EVSEProcessing: Iso2EvseProcessingType,
}

impl Default for Iso2AuthorizationResType {
    fn default() -> Self {
        Iso2AuthorizationResType {
            ResponseCode: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            EVSEProcessing: ISO2_EVSEPROCESSING_TYPE_UNKNOWN,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2PaymentDetailsReqType {
    pub eMaid: Iso2u8FixedBufType<16>,
    pub ContractSignatureCertChain: Iso2CertificateChainType,
}

impl Default for Iso2PaymentDetailsReqType {
    fn default() -> Self {
        Iso2PaymentDetailsReqType {
            eMaid: Iso2u8FixedBufType::<16>::default(),
            ContractSignatureCertChain: Iso2CertificateChainType::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2PaymentDetailsResType {
    pub ResponseCode: Iso2ResponseCodeType,
    pub GenChallenge: Iso2u8FixedBufType<16>,
    pub EVSETimeStamp: i64,
}

impl Default for Iso2PaymentDetailsResType {
    fn default() -> Self {
        Iso2PaymentDetailsResType {
            ResponseCode: ISO2_RESPONSE_CODE_TYPE_UNKNOWN,
            GenChallenge: Iso2u8FixedBufType::<16>::default(),
            EVSETimeStamp: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Iso2BodyType {
    pub BodyTypeComponent: Iso2BodyTypeEnum,
}

#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub struct Iso2v2gMessage {
    pub Header: Iso2MessageHeaderType,
    pub Body: Iso2BodyType,
}

#[derive(Copy, Clone)]
pub struct Iso2ExiDocument {
    pub V2G_Message: Iso2v2gMessage,
}

#[derive(Copy, Clone)]
pub struct Iso2ExiFragment {
    pub ExiFragmentComponents: Iso2ExiFragmentEnum,
}

#[derive(Copy, Clone)]
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
    eMaid(Iso2EMAIDType), // Allow eMaid to not have UpperCamelCase
}

#[derive(Copy, Clone)]
pub struct Iso2XmlDSigFragment {
    pub XmlDSigComponents: Iso2XmlDSigEnum,
}

#[derive(Copy, Clone)]
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
