use core::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExiError {
    NoError = 0,

    // stream processing -1 to -19
    BitstreamOverflow = -1,
    BitstreamUnderflow = -2,
    NotEnoughSpace = -3,

    // stream header -20 to -29
    HeaderCookieNotSupported = -20,
    HeaderOptionsNotSupported = -21,
    HeaderIncorrect = -22,

    // stream read -30 to -39
    SupportedMaxOctetsOverrun = -30,
    OctetCountLargerThanTypeSupports = -31,

    // decoder -50 to -69
    UnknownEventForDecoding = -50,
    DecoderNotImplemented = -69,

    // encoder -70 to -89
    UnknownEventForEncoding = -70,
    EncoderNotImplemented = -89,

    // common errors -100 to -129
    BitCountLargerThanTypeSize = -100,
    ByteCountLargerThanTypeSize = -101,
    ArrayOutOfBounds = -110,
    CharacterBufferTooSmall = -111,
    ByteBufferTooSmall = -112,
    EncodedIntegerSizeLargerThanDestination = -113,
    InvalidCharactersLength = -114,
    InvalidValue = -115,

    // grammar errors -130 to -149
    UnknownGrammarId = -130,

    // event errors -150 to -169
    UnknownEventCode = -150,
    UnsupportedSubEvent = -151,
    UnsupportedEventCode = -152,

    // document errors -170 to -199
    DeviantsNotSupported = -170,

    // datatype errors -200 to -229
    StringvaluesNotSupported = -200,
    UnsupportedIntegerValueType = -210,
    UnsupportedDatetimeType = -211,
    UnsupportedCharacterValue = -212,

    // fragment errors -230 to -259
    IncorrectEndFragmentValue = -230,

    // internal errors
    NotImplementedYet = -299,
}
