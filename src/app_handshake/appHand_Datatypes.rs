#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum AppHandResponseCodeType {
    AppHandResponseCodeTypeFailedNoNegotiation = 2,
    AppHandResponseCodeTypeOkSuccessfulNegotiationWithMinorDeviation = 1,
    AppHandResponseCodeTypeOkSuccessfulNegotiation = 0,
    Unknown = 255,
}

// allow casting from u32 to AppHandResponseCodeType
impl From<u32> for AppHandResponseCodeType {
    fn from(value: u32) -> Self {
        match value {
            0 => AppHandResponseCodeType::AppHandResponseCodeTypeOkSuccessfulNegotiation,
            1 => AppHandResponseCodeType::AppHandResponseCodeTypeOkSuccessfulNegotiationWithMinorDeviation,
            2 => AppHandResponseCodeType::AppHandResponseCodeTypeFailedNoNegotiation,
            _ => AppHandResponseCodeType::Unknown,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AppHandAppProtocolType {
    pub ProtocolNamespace: AppHandProtocolNamespaceType,
    pub VersionNumberMajor: u32,
    pub VersionNumberMinor: u32,
    pub SchemaID: u8,
    pub Priority: u8,
}

impl Default for AppHandAppProtocolType {
    fn default() -> Self {
        AppHandAppProtocolType {
            ProtocolNamespace: AppHandProtocolNamespaceType {
                characters: [0; 100],
                charactersLen: 0,
            },
            VersionNumberMajor: 0,
            VersionNumberMinor: 0,
            SchemaID: 0,
            Priority: 0,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AppHandProtocolNamespaceType {
    pub characters: [u8; 100],
    pub charactersLen: u16,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AppHandSupportedAppProtocolReq {
    pub AppProtocol: AppHandSupportedAppProtocolReqArray,
}

// default impl for AppHandSupportedAppProtocolReq
impl Default for AppHandSupportedAppProtocolReq {
    fn default() -> Self {
        AppHandSupportedAppProtocolReq {
            AppProtocol: AppHandSupportedAppProtocolReqArray {
                array: [AppHandAppProtocolType {
                    ProtocolNamespace: AppHandProtocolNamespaceType {
                        characters: [0; 100],
                        charactersLen: 0,
                    },
                    VersionNumberMajor: 0,
                    VersionNumberMinor: 0,
                    SchemaID: 0,
                    Priority: 0,
                }; 5],
                arrayLen: 0,
            },
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AppHandSupportedAppProtocolReqArray {
    pub array: [AppHandAppProtocolType; 5],
    pub arrayLen: u16,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AppHandSupportedAppProtocolRes {
    pub ResponseCode: AppHandResponseCodeType,
    pub SchemaID: Option<u8>,
}

// default impl for AppHandSupportedAppProtocolRes
impl Default for AppHandSupportedAppProtocolRes {
    fn default() -> Self {
        AppHandSupportedAppProtocolRes {
            ResponseCode: AppHandResponseCodeType::Unknown,
            SchemaID: None,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AppHandExiDocument {
    pub SupportedAppProtocolReq: Option<AppHandSupportedAppProtocolReq>,
    pub SupportedAppProtocolRes: Option<AppHandSupportedAppProtocolRes>
}

impl Default for AppHandExiDocument {
    fn default() -> Self {
        AppHandExiDocument {
            SupportedAppProtocolReq: None,
            SupportedAppProtocolRes: None
        }
    }
}
