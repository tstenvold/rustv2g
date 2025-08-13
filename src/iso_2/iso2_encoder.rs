use crate::{
    common::{
        exi_basetypes_encoder::{
            exi_basetypes_encoder_bool, exi_basetypes_encoder_bytes,
            exi_basetypes_encoder_characters, exi_basetypes_encoder_integer_64,
            exi_basetypes_encoder_nbit_uint, exi_basetypes_encoder_uint_16,
        },
        exi_bitstream::ExiBitstream,
        exi_error_codes::ExiError,
    },
    iso_2::iso2_datatypes::{
        Iso2BodyTypeEnum, Iso2ChargeServiceType, Iso2MessageHeaderType, Iso2NotificationType,
        Iso2PaymentOptionListType, Iso2ServiceDiscoveryReqType, Iso2ServiceDiscoveryResType,
        Iso2ServiceListType, Iso2ServiceType, Iso2SessionSetupReqType, Iso2SessionSetupResType,
        Iso2SupportedEnergyTransferModeType, Iso2v2gMessage,
    },
};

// fn encode_iso2_CostType(
//     stream: &mut ExiBitstream,
//     mut CostType: *const iso2_CostType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 0 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             0 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             2 as i32 as usize,
//                             (*CostType).costKind as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 1 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             1 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_32(stream, (*CostType).amount);
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 2 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             2 => {
//                 if (*CostType).amountMultiplier_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 3 as i32 as usize,
//                                 ((*CostType).amountMultiplier as u32)
//                                     .wrapping_sub(-(3 as i32) as u32),
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 3 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_TransformType(
//     stream: &mut ExiBitstream,
//     mut TransformType: *const iso2_TransformType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 5 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             5 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = exi_basetypes_encoder_uint_16(
//                         stream,
//                         ((*TransformType).Algorithm.charactersLen as i32 + 2 as i32) as u16,
//                     );
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_characters(
//                             stream,
//                             (*TransformType).Algorithm.charactersLen as usize,
//                             ((*TransformType).Algorithm.characters).as_ptr(),
//                             (64 as i32 + 1 as i32) as usize,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 6 as i32;
//                         }
//                     }
//                 }
//             }
//             6 => {
//                 if (*TransformType).XPath_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 ((*TransformType).XPath.charactersLen as i32 + 2 as i32) as u16,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_characters(
//                                     stream,
//                                     (*TransformType).XPath.charactersLen as usize,
//                                     ((*TransformType).XPath.characters).as_ptr(),
//                                     (64 as i32 + 1 as i32) as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 3 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else if (*TransformType).ANY_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*TransformType).ANY.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*TransformType).ANY.bytesLen as usize,
//                                     ((*TransformType).ANY.bytes).as_ptr(),
//                                     4 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 3 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_IntervalType(
//     stream: &mut ExiBitstream,
//     mut IntervalType: *const iso2_IntervalType,
// ) -> Result<(), ExiError> {
//     let mut error: i32 =
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//     return error;
// }
// fn encode_iso2_TransformsType(
//     stream: &mut ExiBitstream,
//     mut TransformsType: *const iso2_TransformsType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 7 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             7 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_TransformType(stream, &(*TransformsType).Transform);
//                     if error == 0 as i32 {
//                         grammar_id = 8 as i32;
//                     }
//                 }
//             }
//             8 => {
//                 if 1 as i32 == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_TransformType(stream, &(*TransformsType).Transform);
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_DSAKeyValueType(
//     stream: &mut ExiBitstream,
//     mut DSAKeyValueType: *const iso2_DSAKeyValueType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 9 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             9 => {
//                 if (*DSAKeyValueType).P_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*DSAKeyValueType).P.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*DSAKeyValueType).P.bytesLen as usize,
//                                     ((*DSAKeyValueType).P.bytes).as_ptr(),
//                                     350 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 10 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else if (*DSAKeyValueType).G_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*DSAKeyValueType).G.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*DSAKeyValueType).G.bytesLen as usize,
//                                     ((*DSAKeyValueType).G.bytes).as_ptr(),
//                                     350 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 12 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*DSAKeyValueType).Y.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*DSAKeyValueType).Y.bytesLen as usize,
//                                     ((*DSAKeyValueType).Y.bytes).as_ptr(),
//                                     350 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 13 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             10 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error =
//                             exi_basetypes_encoder_uint_16(stream, (*DSAKeyValueType).Q.bytesLen);
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bytes(
//                                 stream,
//                                 (*DSAKeyValueType).Q.bytesLen as usize,
//                                 ((*DSAKeyValueType).Q.bytes).as_ptr(),
//                                 350 as i32 as usize,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 11 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             11 => {
//                 if (*DSAKeyValueType).G_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*DSAKeyValueType).G.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*DSAKeyValueType).G.bytesLen as usize,
//                                     ((*DSAKeyValueType).G.bytes).as_ptr(),
//                                     350 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 12 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*DSAKeyValueType).Y.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*DSAKeyValueType).Y.bytesLen as usize,
//                                     ((*DSAKeyValueType).Y.bytes).as_ptr(),
//                                     350 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 13 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             12 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error =
//                             exi_basetypes_encoder_uint_16(stream, (*DSAKeyValueType).Y.bytesLen);
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bytes(
//                                 stream,
//                                 (*DSAKeyValueType).Y.bytesLen as usize,
//                                 ((*DSAKeyValueType).Y.bytes).as_ptr(),
//                                 350 as i32 as usize,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 13 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             13 => {
//                 if (*DSAKeyValueType).J_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*DSAKeyValueType).J.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*DSAKeyValueType).J.bytesLen as usize,
//                                     ((*DSAKeyValueType).J.bytes).as_ptr(),
//                                     350 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 14 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else if (*DSAKeyValueType).Seed_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*DSAKeyValueType).Seed.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*DSAKeyValueType).Seed.bytesLen as usize,
//                                     ((*DSAKeyValueType).Seed.bytes).as_ptr(),
//                                     350 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 15 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             14 => {
//                 if (*DSAKeyValueType).Seed_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*DSAKeyValueType).Seed.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*DSAKeyValueType).Seed.bytesLen as usize,
//                                     ((*DSAKeyValueType).Seed.bytes).as_ptr(),
//                                     350 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 15 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             15 => {
//                 if (*DSAKeyValueType).PgenCounter_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*DSAKeyValueType).PgenCounter.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*DSAKeyValueType).PgenCounter.bytesLen as usize,
//                                     ((*DSAKeyValueType).PgenCounter.bytes).as_ptr(),
//                                     350 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 3 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_X509IssuerSerialType(
//     stream: &mut ExiBitstream,
//     mut X509IssuerSerialType: *const iso2_X509IssuerSerialType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 16 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             16 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*X509IssuerSerialType).X509IssuerName.charactersLen as i32 + 2 as i32)
//                                 as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*X509IssuerSerialType).X509IssuerName.charactersLen as usize,
//                                 ((*X509IssuerSerialType).X509IssuerName.characters).as_ptr(),
//                                 (64 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 17 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             17 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_signed(
//                             stream,
//                             &(*X509IssuerSerialType).X509SerialNumber,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_RelativeTimeIntervalType(
//     stream: &mut ExiBitstream,
//     mut RelativeTimeIntervalType: *const iso2_RelativeTimeIntervalType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 18 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             18 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_32(
//                             stream,
//                             (*RelativeTimeIntervalType).start,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 19 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             19 => {
//                 if (*RelativeTimeIntervalType).duration_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_32(
//                                 stream,
//                                 (*RelativeTimeIntervalType).duration,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 3 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_DigestMethodType(
//     stream: &mut ExiBitstream,
//     mut DigestMethodType: *const iso2_DigestMethodType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 20 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             20 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = exi_basetypes_encoder_uint_16(
//                         stream,
//                         ((*DigestMethodType).Algorithm.charactersLen as i32 + 2 as i32) as u16,
//                     );
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_characters(
//                             stream,
//                             (*DigestMethodType).Algorithm.charactersLen as usize,
//                             ((*DigestMethodType).Algorithm.characters).as_ptr(),
//                             (64 as i32 + 1 as i32) as usize,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 21 as i32;
//                         }
//                     }
//                 }
//             }
//             21 => {
//                 if (*DigestMethodType).ANY_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*DigestMethodType).ANY.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*DigestMethodType).ANY.bytesLen as usize,
//                                     ((*DigestMethodType).ANY.bytes).as_ptr(),
//                                     4 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 3 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_RSAKeyValueType(
//     stream: &mut ExiBitstream,
//     mut RSAKeyValueType: *const iso2_RSAKeyValueType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 22 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             22 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             (*RSAKeyValueType).Modulus.bytesLen,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bytes(
//                                 stream,
//                                 (*RSAKeyValueType).Modulus.bytesLen as usize,
//                                 ((*RSAKeyValueType).Modulus.bytes).as_ptr(),
//                                 350 as i32 as usize,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 23 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             23 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             (*RSAKeyValueType).Exponent.bytesLen,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bytes(
//                                 stream,
//                                 (*RSAKeyValueType).Exponent.bytesLen as usize,
//                                 ((*RSAKeyValueType).Exponent.bytes).as_ptr(),
//                                 350 as i32 as usize,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 3 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_CanonicalizationMethodType(
//     stream: &mut ExiBitstream,
//     mut CanonicalizationMethodType: *const iso2_CanonicalizationMethodType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 24 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             24 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = exi_basetypes_encoder_uint_16(
//                         stream,
//                         ((*CanonicalizationMethodType).Algorithm.charactersLen as i32 + 2 as i32)
//                             as u16,
//                     );
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_characters(
//                             stream,
//                             (*CanonicalizationMethodType).Algorithm.charactersLen as usize,
//                             ((*CanonicalizationMethodType).Algorithm.characters).as_ptr(),
//                             (64 as i32 + 1 as i32) as usize,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 25 as i32;
//                         }
//                     }
//                 }
//             }
//             25 => {
//                 if (*CanonicalizationMethodType).ANY_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*CanonicalizationMethodType).ANY.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*CanonicalizationMethodType).ANY.bytesLen as usize,
//                                     ((*CanonicalizationMethodType).ANY.bytes).as_ptr(),
//                                     4 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 3 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_SignatureMethodType(
//     stream: &mut ExiBitstream,
//     mut SignatureMethodType: *const iso2_SignatureMethodType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 26 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             26 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = exi_basetypes_encoder_uint_16(
//                         stream,
//                         ((*SignatureMethodType).Algorithm.charactersLen as i32 + 2 as i32) as u16,
//                     );
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_characters(
//                             stream,
//                             (*SignatureMethodType).Algorithm.charactersLen as usize,
//                             ((*SignatureMethodType).Algorithm.characters).as_ptr(),
//                             (64 as i32 + 1 as i32) as usize,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 27 as i32;
//                         }
//                     }
//                 }
//             }
//             27 => {
//                 if (*SignatureMethodType).HMACOutputLength_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_signed(
//                                 stream,
//                                 &(*SignatureMethodType).HMACOutputLength,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 28 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else if (*SignatureMethodType).ANY_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*SignatureMethodType).ANY.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*SignatureMethodType).ANY.bytesLen as usize,
//                                     ((*SignatureMethodType).ANY.bytes).as_ptr(),
//                                     4 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 3 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             28 => {
//                 if (*SignatureMethodType).ANY_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*SignatureMethodType).ANY.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*SignatureMethodType).ANY.bytesLen as usize,
//                                     ((*SignatureMethodType).ANY.bytes).as_ptr(),
//                                     4 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 3 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_KeyValueType(
//     stream: &mut ExiBitstream,
//     mut KeyValueType: *const iso2_KeyValueType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 29 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             29 => {
//                 if (*KeyValueType).DSAKeyValue_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_DSAKeyValueType(stream, &(*KeyValueType).DSAKeyValue);
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else if (*KeyValueType).RSAKeyValue_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_RSAKeyValueType(stream, &(*KeyValueType).RSAKeyValue);
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else if (*KeyValueType).ANY_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error =
//                                 exi_basetypes_encoder_uint_16(stream, (*KeyValueType).ANY.bytesLen);
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*KeyValueType).ANY.bytesLen as usize,
//                                     ((*KeyValueType).ANY.bytes).as_ptr(),
//                                     4 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 3 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_physical_value(
//     stream: &mut ExiBitstream,
//     message: &Iso2PhysicalValueType,
// ) -> Result<(), ExiError> {

//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     exi_basetypes_encoder_nbit_uint(stream, 3, message.multiplier as u32)?;
//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     exi_basetypes_encoder_nbit_uint(stream, 3, message.unit as u32)?;
//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     exi_basetypes_encoder_integer_16(stream, message.value)?;
//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)

// }
// fn encode_iso2_ConsumptionCostType(
//     stream: &mut ExiBitstream,
//     mut ConsumptionCostType: *const iso2_ConsumptionCostType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 33 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     let mut Cost_currentIndex: u16 = 0 as i32 as u16;
//     while done == 0 {
//         match grammar_id {
//             33 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         encode_iso2_PhysicalValueType(stream, &(*ConsumptionCostType).startValue);
//                     if error == 0 as i32 {
//                         grammar_id = 34 as i32;
//                     }
//                 }
//             }
//             34 => {
//                 if (Cost_currentIndex as i32) < (*ConsumptionCostType).Cost.arrayLen as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         let fresh0 = Cost_currentIndex;
//                         Cost_currentIndex = Cost_currentIndex.wrapping_add(1);
//                         error = encode_iso2_CostType(
//                             stream,
//                             &*((*ConsumptionCostType).Cost.array)
//                                 .as_ptr()
//                                 .offset(fresh0 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 35 as i32;
//                         }
//                     }
//                 } else {
//                     error = -(150 as i32);
//                 }
//             }
//             35 => {
//                 if (Cost_currentIndex as i32) < (*ConsumptionCostType).Cost.arrayLen as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         let fresh1 = Cost_currentIndex;
//                         Cost_currentIndex = Cost_currentIndex.wrapping_add(1);
//                         error = encode_iso2_CostType(
//                             stream,
//                             &*((*ConsumptionCostType).Cost.array)
//                                 .as_ptr()
//                                 .offset(fresh1 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 35 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_PMaxScheduleEntryType(
//     stream: &mut ExiBitstream,
//     mut PMaxScheduleEntryType: *const iso2_PMaxScheduleEntryType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 36 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             36 => {
//                 if (*PMaxScheduleEntryType).RelativeTimeInterval_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_RelativeTimeIntervalType(
//                             stream,
//                             &(*PMaxScheduleEntryType).RelativeTimeInterval,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 37 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_IntervalType(
//                             stream,
//                             &(*PMaxScheduleEntryType).TimeInterval,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 37 as i32;
//                         }
//                     }
//                 }
//             }
//             37 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_PhysicalValueType(stream, &(*PMaxScheduleEntryType).PMax);
//                     if error == 0 as i32 {
//                         grammar_id = 3 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_SalesTariffEntryType(
//     stream: &mut ExiBitstream,
//     mut SalesTariffEntryType: *const iso2_SalesTariffEntryType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 38 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     let mut ConsumptionCost_currentIndex: u16 = 0 as i32 as u16;
//     while done == 0 {
//         match grammar_id {
//             38 => {
//                 if (*SalesTariffEntryType).RelativeTimeInterval_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_RelativeTimeIntervalType(
//                             stream,
//                             &(*SalesTariffEntryType).RelativeTimeInterval,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 39 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error =
//                             encode_iso2_IntervalType(stream, &(*SalesTariffEntryType).TimeInterval);
//                         if error == 0 as i32 {
//                             grammar_id = 39 as i32;
//                         }
//                     }
//                 }
//             }
//             39 => {
//                 if (*SalesTariffEntryType).EPriceLevel_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 8 as i32 as usize,
//                                 (*SalesTariffEntryType).EPriceLevel as u32,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 41 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else if (ConsumptionCost_currentIndex as i32)
//                     < (*SalesTariffEntryType).ConsumptionCost.arrayLen as i32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         let fresh2 = ConsumptionCost_currentIndex;
//                         ConsumptionCost_currentIndex = ConsumptionCost_currentIndex.wrapping_add(1);
//                         error = encode_iso2_ConsumptionCostType(
//                             stream,
//                             &*((*SalesTariffEntryType).ConsumptionCost.array)
//                                 .as_ptr()
//                                 .offset(fresh2 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 40 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             40 => {
//                 if (ConsumptionCost_currentIndex as i32)
//                     < (*SalesTariffEntryType).ConsumptionCost.arrayLen as i32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         let fresh3 = ConsumptionCost_currentIndex;
//                         ConsumptionCost_currentIndex = ConsumptionCost_currentIndex.wrapping_add(1);
//                         error = encode_iso2_ConsumptionCostType(
//                             stream,
//                             &*((*SalesTariffEntryType).ConsumptionCost.array)
//                                 .as_ptr()
//                                 .offset(fresh3 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 40 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             41 => {
//                 if (ConsumptionCost_currentIndex as i32)
//                     < (*SalesTariffEntryType).ConsumptionCost.arrayLen as i32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         let fresh4 = ConsumptionCost_currentIndex;
//                         ConsumptionCost_currentIndex = ConsumptionCost_currentIndex.wrapping_add(1);
//                         error = encode_iso2_ConsumptionCostType(
//                             stream,
//                             &*((*SalesTariffEntryType).ConsumptionCost.array)
//                                 .as_ptr()
//                                 .offset(fresh4 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 42 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             42 => {
//                 if (ConsumptionCost_currentIndex as i32)
//                     < (*SalesTariffEntryType).ConsumptionCost.arrayLen as i32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         let fresh5 = ConsumptionCost_currentIndex;
//                         ConsumptionCost_currentIndex = ConsumptionCost_currentIndex.wrapping_add(1);
//                         error = encode_iso2_ConsumptionCostType(
//                             stream,
//                             &*((*SalesTariffEntryType).ConsumptionCost.array)
//                                 .as_ptr()
//                                 .offset(fresh5 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 42 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_ParameterType(
//     stream: &mut ExiBitstream,
//     mut ParameterType: *const iso2_ParameterType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 43 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             43 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = exi_basetypes_encoder_uint_16(
//                         stream,
//                         ((*ParameterType).Name.charactersLen as i32 + 2 as i32) as u16,
//                     );
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_characters(
//                             stream,
//                             (*ParameterType).Name.charactersLen as usize,
//                             ((*ParameterType).Name.characters).as_ptr(),
//                             (64 as i32 + 1 as i32) as usize,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 44 as i32;
//                         }
//                     }
//                 }
//             }
//             44 => {
//                 if (*ParameterType).boolValue_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bool(stream, (*ParameterType).boolValue);
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 3 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else if (*ParameterType).byteValue_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 8 as i32 as usize,
//                                 ((*ParameterType).byteValue as i32 + -(128 as i32)) as u32,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 3 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else if (*ParameterType).shortValue_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_integer_16(
//                                 stream,
//                                 (*ParameterType).shortValue,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 3 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else if (*ParameterType).intValue_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error =
//                                 exi_basetypes_encoder_integer_32(stream, (*ParameterType).intValue);
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 3 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else if (*ParameterType).physicalValue_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
//                     if error == 0 as i32 {
//                         error =
//                             encode_iso2_PhysicalValueType(stream, &(*ParameterType).physicalValue);
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 ((*ParameterType).stringValue.charactersLen as i32 + 2 as i32)
//                                     as u16,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_characters(
//                                     stream,
//                                     (*ParameterType).stringValue.charactersLen as usize,
//                                     ((*ParameterType).stringValue.characters).as_ptr(),
//                                     (64 as i32 + 1 as i32) as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 3 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_PMaxScheduleType(
//     stream: &mut ExiBitstream,
//     mut PMaxScheduleType: *const iso2_PMaxScheduleType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 45 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     let mut PMaxScheduleEntry_currentIndex: u16 = 0 as i32 as u16;
//     while done == 0 {
//         match grammar_id {
//             45 => {
//                 if (PMaxScheduleEntry_currentIndex as i32)
//                     < (*PMaxScheduleType).PMaxScheduleEntry.arrayLen as i32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         let fresh6 = PMaxScheduleEntry_currentIndex;
//                         PMaxScheduleEntry_currentIndex =
//                             PMaxScheduleEntry_currentIndex.wrapping_add(1);
//                         error = encode_iso2_PMaxScheduleEntryType(
//                             stream,
//                             &*((*PMaxScheduleType).PMaxScheduleEntry.array)
//                                 .as_ptr()
//                                 .offset(fresh6 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 46 as i32;
//                         }
//                     }
//                 } else {
//                     error = -(150 as i32);
//                 }
//             }
//             46 => {
//                 if (PMaxScheduleEntry_currentIndex as i32)
//                     < (*PMaxScheduleType).PMaxScheduleEntry.arrayLen as i32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         let fresh7 = PMaxScheduleEntry_currentIndex;
//                         PMaxScheduleEntry_currentIndex =
//                             PMaxScheduleEntry_currentIndex.wrapping_add(1);
//                         error = encode_iso2_PMaxScheduleEntryType(
//                             stream,
//                             &*((*PMaxScheduleType).PMaxScheduleEntry.array)
//                                 .as_ptr()
//                                 .offset(fresh7 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 46 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_ReferenceType(
//     stream: &mut ExiBitstream,
//     mut ReferenceType: *const iso2_ReferenceType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 47 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             47 => {
//                 if (*ReferenceType).Id_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*ReferenceType).Id.charactersLen as i32 + 2 as i32) as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*ReferenceType).Id.charactersLen as usize,
//                                 ((*ReferenceType).Id.characters).as_ptr(),
//                                 (64 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 48 as i32;
//                             }
//                         }
//                     }
//                 } else if (*ReferenceType).Type_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*ReferenceType).Type.charactersLen as i32 + 2 as i32) as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*ReferenceType).Type.charactersLen as usize,
//                                 ((*ReferenceType).Type.characters).as_ptr(),
//                                 (64 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 49 as i32;
//                             }
//                         }
//                     }
//                 } else if (*ReferenceType).URI_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*ReferenceType).URI.charactersLen as i32 + 2 as i32) as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*ReferenceType).URI.charactersLen as usize,
//                                 ((*ReferenceType).URI.characters).as_ptr(),
//                                 (64 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 50 as i32;
//                             }
//                         }
//                     }
//                 } else if (*ReferenceType).Transforms_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_TransformsType(stream, &(*ReferenceType).Transforms);
//                         if error == 0 as i32 {
//                             grammar_id = 51 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
//                     if error == 0 as i32 {
//                         error =
//                             encode_iso2_DigestMethodType(stream, &(*ReferenceType).DigestMethod);
//                         if error == 0 as i32 {
//                             grammar_id = 52 as i32;
//                         }
//                     }
//                 }
//             }
//             48 => {
//                 if (*ReferenceType).Type_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*ReferenceType).Type.charactersLen as i32 + 2 as i32) as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*ReferenceType).Type.charactersLen as usize,
//                                 ((*ReferenceType).Type.characters).as_ptr(),
//                                 (64 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 49 as i32;
//                             }
//                         }
//                     }
//                 } else if (*ReferenceType).URI_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*ReferenceType).URI.charactersLen as i32 + 2 as i32) as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*ReferenceType).URI.charactersLen as usize,
//                                 ((*ReferenceType).URI.characters).as_ptr(),
//                                 (64 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 50 as i32;
//                             }
//                         }
//                     }
//                 } else if (*ReferenceType).Transforms_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_TransformsType(stream, &(*ReferenceType).Transforms);
//                         if error == 0 as i32 {
//                             grammar_id = 51 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
//                     if error == 0 as i32 {
//                         error =
//                             encode_iso2_DigestMethodType(stream, &(*ReferenceType).DigestMethod);
//                         if error == 0 as i32 {
//                             grammar_id = 52 as i32;
//                         }
//                     }
//                 }
//             }
//             49 => {
//                 if (*ReferenceType).URI_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*ReferenceType).URI.charactersLen as i32 + 2 as i32) as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*ReferenceType).URI.charactersLen as usize,
//                                 ((*ReferenceType).URI.characters).as_ptr(),
//                                 (64 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 50 as i32;
//                             }
//                         }
//                     }
//                 } else if (*ReferenceType).Transforms_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_TransformsType(stream, &(*ReferenceType).Transforms);
//                         if error == 0 as i32 {
//                             grammar_id = 51 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         error =
//                             encode_iso2_DigestMethodType(stream, &(*ReferenceType).DigestMethod);
//                         if error == 0 as i32 {
//                             grammar_id = 52 as i32;
//                         }
//                     }
//                 }
//             }
//             50 => {
//                 if (*ReferenceType).Transforms_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_TransformsType(stream, &(*ReferenceType).Transforms);
//                         if error == 0 as i32 {
//                             grammar_id = 51 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error =
//                             encode_iso2_DigestMethodType(stream, &(*ReferenceType).DigestMethod);
//                         if error == 0 as i32 {
//                             grammar_id = 52 as i32;
//                         }
//                     }
//                 }
//             }
//             51 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_DigestMethodType(stream, &(*ReferenceType).DigestMethod);
//                     if error == 0 as i32 {
//                         grammar_id = 52 as i32;
//                     }
//                 }
//             }
//             52 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             (*ReferenceType).DigestValue.bytesLen,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bytes(
//                                 stream,
//                                 (*ReferenceType).DigestValue.bytesLen as usize,
//                                 ((*ReferenceType).DigestValue.bytes).as_ptr(),
//                                 350 as i32 as usize,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 3 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_RetrievalMethodType(
//     stream: &mut ExiBitstream,
//     mut RetrievalMethodType: *const iso2_RetrievalMethodType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 53 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             53 => {
//                 if (*RetrievalMethodType).Type_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*RetrievalMethodType).Type.charactersLen as i32 + 2 as i32) as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*RetrievalMethodType).Type.charactersLen as usize,
//                                 ((*RetrievalMethodType).Type.characters).as_ptr(),
//                                 (64 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 54 as i32;
//                             }
//                         }
//                     }
//                 } else if (*RetrievalMethodType).URI_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*RetrievalMethodType).URI.charactersLen as i32 + 2 as i32) as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*RetrievalMethodType).URI.charactersLen as usize,
//                                 ((*RetrievalMethodType).URI.characters).as_ptr(),
//                                 (64 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 55 as i32;
//                             }
//                         }
//                     }
//                 } else if (*RetrievalMethodType).Transforms_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         error =
//                             encode_iso2_TransformsType(stream, &(*RetrievalMethodType).Transforms);
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             54 => {
//                 if (*RetrievalMethodType).URI_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*RetrievalMethodType).URI.charactersLen as i32 + 2 as i32) as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*RetrievalMethodType).URI.charactersLen as usize,
//                                 ((*RetrievalMethodType).URI.characters).as_ptr(),
//                                 (64 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 55 as i32;
//                             }
//                         }
//                     }
//                 } else if (*RetrievalMethodType).Transforms_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error =
//                             encode_iso2_TransformsType(stream, &(*RetrievalMethodType).Transforms);
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             55 => {
//                 if (*RetrievalMethodType).Transforms_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error =
//                             encode_iso2_TransformsType(stream, &(*RetrievalMethodType).Transforms);
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_SalesTariffType(
//     stream: &mut ExiBitstream,
//     mut SalesTariffType: *const iso2_SalesTariffType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 56 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     let mut SalesTariffEntry_currentIndex: u16 = 0 as i32 as u16;
//     while done == 0 {
//         match grammar_id {
//             56 => {
//                 if (*SalesTariffType).Id_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*SalesTariffType).Id.charactersLen as i32 + 2 as i32) as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*SalesTariffType).Id.charactersLen as usize,
//                                 ((*SalesTariffType).Id.characters).as_ptr(),
//                                 (64 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 57 as i32;
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 8 as i32 as usize,
//                                 ((*SalesTariffType).SalesTariffID as u32)
//                                     .wrapping_sub(1 as i32 as u32),
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 58 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             57 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             8 as i32 as usize,
//                             ((*SalesTariffType).SalesTariffID as u32).wrapping_sub(1 as i32 as u32),
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 58 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             58 => {
//                 if (*SalesTariffType).SalesTariffDescription_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 ((*SalesTariffType).SalesTariffDescription.charactersLen as i32
//                                     + 2 as i32) as u16,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_characters(
//                                     stream,
//                                     (*SalesTariffType).SalesTariffDescription.charactersLen
//                                         as usize,
//                                     ((*SalesTariffType).SalesTariffDescription.characters).as_ptr(),
//                                     (32 as i32 + 1 as i32) as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 60 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else if (*SalesTariffType).NumEPriceLevels_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 8 as i32 as usize,
//                                 (*SalesTariffType).NumEPriceLevels as u32,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 62 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else if (SalesTariffEntry_currentIndex as i32)
//                     < (*SalesTariffType).SalesTariffEntry.arrayLen as i32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         let fresh8 = SalesTariffEntry_currentIndex;
//                         SalesTariffEntry_currentIndex =
//                             SalesTariffEntry_currentIndex.wrapping_add(1);
//                         error = encode_iso2_SalesTariffEntryType(
//                             stream,
//                             &*((*SalesTariffType).SalesTariffEntry.array)
//                                 .as_ptr()
//                                 .offset(fresh8 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 59 as i32;
//                         }
//                     }
//                 }
//             }
//             59 => {
//                 if (SalesTariffEntry_currentIndex as i32)
//                     < (*SalesTariffType).SalesTariffEntry.arrayLen as i32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         let fresh9 = SalesTariffEntry_currentIndex;
//                         SalesTariffEntry_currentIndex =
//                             SalesTariffEntry_currentIndex.wrapping_add(1);
//                         error = encode_iso2_SalesTariffEntryType(
//                             stream,
//                             &*((*SalesTariffType).SalesTariffEntry.array)
//                                 .as_ptr()
//                                 .offset(fresh9 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 59 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             60 => {
//                 if (*SalesTariffType).NumEPriceLevels_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 8 as i32 as usize,
//                                 (*SalesTariffType).NumEPriceLevels as u32,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 62 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else if (SalesTariffEntry_currentIndex as i32)
//                     < (*SalesTariffType).SalesTariffEntry.arrayLen as i32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         let fresh10 = SalesTariffEntry_currentIndex;
//                         SalesTariffEntry_currentIndex =
//                             SalesTariffEntry_currentIndex.wrapping_add(1);
//                         error = encode_iso2_SalesTariffEntryType(
//                             stream,
//                             &*((*SalesTariffType).SalesTariffEntry.array)
//                                 .as_ptr()
//                                 .offset(fresh10 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 61 as i32;
//                         }
//                     }
//                 }
//             }
//             61 => {
//                 if (SalesTariffEntry_currentIndex as i32)
//                     < (*SalesTariffType).SalesTariffEntry.arrayLen as i32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         let fresh11 = SalesTariffEntry_currentIndex;
//                         SalesTariffEntry_currentIndex =
//                             SalesTariffEntry_currentIndex.wrapping_add(1);
//                         error = encode_iso2_SalesTariffEntryType(
//                             stream,
//                             &*((*SalesTariffType).SalesTariffEntry.array)
//                                 .as_ptr()
//                                 .offset(fresh11 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 61 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             62 => {
//                 if (SalesTariffEntry_currentIndex as i32)
//                     < (*SalesTariffType).SalesTariffEntry.arrayLen as i32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         let fresh12 = SalesTariffEntry_currentIndex;
//                         SalesTariffEntry_currentIndex =
//                             SalesTariffEntry_currentIndex.wrapping_add(1);
//                         error = encode_iso2_SalesTariffEntryType(
//                             stream,
//                             &*((*SalesTariffType).SalesTariffEntry.array)
//                                 .as_ptr()
//                                 .offset(fresh12 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 63 as i32;
//                         }
//                     }
//                 } else {
//                     error = -(150 as i32);
//                 }
//             }
//             63 => {
//                 if (SalesTariffEntry_currentIndex as i32)
//                     < (*SalesTariffType).SalesTariffEntry.arrayLen as i32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         let fresh13 = SalesTariffEntry_currentIndex;
//                         SalesTariffEntry_currentIndex =
//                             SalesTariffEntry_currentIndex.wrapping_add(1);
//                         error = encode_iso2_SalesTariffEntryType(
//                             stream,
//                             &*((*SalesTariffType).SalesTariffEntry.array)
//                                 .as_ptr()
//                                 .offset(fresh13 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 63 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_X509DataType(
//     stream: &mut ExiBitstream,
//     mut X509DataType: *const iso2_X509DataType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 64 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             64 => {
//                 if (*X509DataType).X509IssuerSerial_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_X509IssuerSerialType(
//                             stream,
//                             &(*X509DataType).X509IssuerSerial,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else if (*X509DataType).X509SKI_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*X509DataType).X509SKI.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*X509DataType).X509SKI.bytesLen as usize,
//                                     ((*X509DataType).X509SKI.bytes).as_ptr(),
//                                     350 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 3 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else if (*X509DataType).X509SubjectName_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 ((*X509DataType).X509SubjectName.charactersLen as i32 + 2 as i32)
//                                     as u16,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_characters(
//                                     stream,
//                                     (*X509DataType).X509SubjectName.charactersLen as usize,
//                                     ((*X509DataType).X509SubjectName.characters).as_ptr(),
//                                     (64 as i32 + 1 as i32) as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 3 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else if (*X509DataType).X509Certificate_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*X509DataType).X509Certificate.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*X509DataType).X509Certificate.bytesLen as usize,
//                                     ((*X509DataType).X509Certificate.bytes).as_ptr(),
//                                     350 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 3 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else if (*X509DataType).X509CRL_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*X509DataType).X509CRL.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*X509DataType).X509CRL.bytesLen as usize,
//                                     ((*X509DataType).X509CRL.bytes).as_ptr(),
//                                     350 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 3 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else if (*X509DataType).ANY_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error =
//                                 exi_basetypes_encoder_uint_16(stream, (*X509DataType).ANY.bytesLen);
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*X509DataType).ANY.bytesLen as usize,
//                                     ((*X509DataType).ANY.bytes).as_ptr(),
//                                     4 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 3 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_PGPDataType(
//     stream: &mut ExiBitstream,
//     mut PGPDataType: *const iso2_PGPDataType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 65 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             65 => {
//                 if ((*PGPDataType).c2rust_unnamed).choice_1_isUsed == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*PGPDataType).c2rust_unnamed.choice_1.PGPKeyID.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*PGPDataType).c2rust_unnamed.choice_1.PGPKeyID.bytesLen
//                                         as usize,
//                                     ((*PGPDataType).c2rust_unnamed.choice_1.PGPKeyID.bytes)
//                                         .as_ptr(),
//                                     350 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 66 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*PGPDataType).c2rust_unnamed.choice_1.PGPKeyPacket.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*PGPDataType).c2rust_unnamed.choice_1.PGPKeyPacket.bytesLen
//                                         as usize,
//                                     ((*PGPDataType).c2rust_unnamed.choice_1.PGPKeyPacket.bytes)
//                                         .as_ptr(),
//                                     350 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 67 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             66 => {
//                 if ((*PGPDataType).c2rust_unnamed).choice_1_isUsed == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*PGPDataType).c2rust_unnamed.choice_1.PGPKeyPacket.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*PGPDataType).c2rust_unnamed.choice_1.PGPKeyPacket.bytesLen
//                                         as usize,
//                                     ((*PGPDataType).c2rust_unnamed.choice_1.PGPKeyPacket.bytes)
//                                         .as_ptr(),
//                                     350 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 67 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else if ((*PGPDataType).c2rust_unnamed).choice_1_isUsed == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*PGPDataType).c2rust_unnamed.choice_1.ANY.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*PGPDataType).c2rust_unnamed.choice_1.ANY.bytesLen as usize,
//                                     ((*PGPDataType).c2rust_unnamed.choice_1.ANY.bytes).as_ptr(),
//                                     4 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 68 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             67 => {
//                 if ((*PGPDataType).c2rust_unnamed).choice_1_isUsed == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*PGPDataType).c2rust_unnamed.choice_1.ANY.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*PGPDataType).c2rust_unnamed.choice_1.ANY.bytesLen as usize,
//                                     ((*PGPDataType).c2rust_unnamed.choice_1.ANY.bytes).as_ptr(),
//                                     4 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 68 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             68 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             (*PGPDataType).c2rust_unnamed.choice_2.PGPKeyPacket.bytesLen,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bytes(
//                                 stream,
//                                 (*PGPDataType).c2rust_unnamed.choice_2.PGPKeyPacket.bytesLen
//                                     as usize,
//                                 ((*PGPDataType).c2rust_unnamed.choice_2.PGPKeyPacket.bytes)
//                                     .as_ptr(),
//                                 350 as i32 as usize,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 69 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             69 => {
//                 if ((*PGPDataType).c2rust_unnamed).choice_2_isUsed == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*PGPDataType).c2rust_unnamed.choice_2.ANY.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*PGPDataType).c2rust_unnamed.choice_2.ANY.bytesLen as usize,
//                                     ((*PGPDataType).c2rust_unnamed.choice_2.ANY.bytes).as_ptr(),
//                                     4 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 68 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_SPKIDataType(
//     stream: &mut ExiBitstream,
//     mut SPKIDataType: *const iso2_SPKIDataType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 70 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             70 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             (*SPKIDataType).SPKISexp.bytesLen,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bytes(
//                                 stream,
//                                 (*SPKIDataType).SPKISexp.bytesLen as usize,
//                                 ((*SPKIDataType).SPKISexp.bytes).as_ptr(),
//                                 350 as i32 as usize,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 71 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             71 => {
//                 if (*SPKIDataType).ANY_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error =
//                                 exi_basetypes_encoder_uint_16(stream, (*SPKIDataType).ANY.bytesLen);
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*SPKIDataType).ANY.bytesLen as usize,
//                                     ((*SPKIDataType).ANY.bytes).as_ptr(),
//                                     4 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 3 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_SignedInfoType(
//     stream: &mut ExiBitstream,
//     mut SignedInfoType: *const iso2_SignedInfoType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 72 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     let mut Reference_currentIndex: u16 = 0 as i32 as u16;
//     while done == 0 {
//         match grammar_id {
//             72 => {
//                 if (*SignedInfoType).Id_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*SignedInfoType).Id.charactersLen as i32 + 2 as i32) as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*SignedInfoType).Id.charactersLen as usize,
//                                 ((*SignedInfoType).Id.characters).as_ptr(),
//                                 (64 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 73 as i32;
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_CanonicalizationMethodType(
//                             stream,
//                             &(*SignedInfoType).CanonicalizationMethod,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 74 as i32;
//                         }
//                     }
//                 }
//             }
//             73 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_CanonicalizationMethodType(
//                         stream,
//                         &(*SignedInfoType).CanonicalizationMethod,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 74 as i32;
//                     }
//                 }
//             }
//             74 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         encode_iso2_SignatureMethodType(stream, &(*SignedInfoType).SignatureMethod);
//                     if error == 0 as i32 {
//                         grammar_id = 75 as i32;
//                     }
//                 }
//             }
//             75 => {
//                 if (Reference_currentIndex as i32) < (*SignedInfoType).Reference.arrayLen as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         let fresh14 = Reference_currentIndex;
//                         Reference_currentIndex = Reference_currentIndex.wrapping_add(1);
//                         error = encode_iso2_ReferenceType(
//                             stream,
//                             &*((*SignedInfoType).Reference.array)
//                                 .as_ptr()
//                                 .offset(fresh14 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 76 as i32;
//                         }
//                     }
//                 } else {
//                     error = -(150 as i32);
//                 }
//             }
//             76 => {
//                 if (Reference_currentIndex as i32) < (*SignedInfoType).Reference.arrayLen as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         let fresh15 = Reference_currentIndex;
//                         Reference_currentIndex = Reference_currentIndex.wrapping_add(1);
//                         error = encode_iso2_ReferenceType(
//                             stream,
//                             &*((*SignedInfoType).Reference.array)
//                                 .as_ptr()
//                                 .offset(fresh15 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 76 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_ProfileEntryType(
//     stream: &mut ExiBitstream,
//     mut ProfileEntryType: *const iso2_ProfileEntryType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 77 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             77 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_32(
//                             stream,
//                             (*ProfileEntryType).ChargingProfileEntryStart,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 78 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             78 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_PhysicalValueType(
//                         stream,
//                         &(*ProfileEntryType).ChargingProfileEntryMaxPower,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 79 as i32;
//                     }
//                 }
//             }
//             79 => {
//                 if (*ProfileEntryType).ChargingProfileEntryMaxNumberOfPhasesInUse_isUsed()
//                     == 1 as u32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 2 as i32 as usize,
//                                 ((*ProfileEntryType).ChargingProfileEntryMaxNumberOfPhasesInUse
//                                     as u32)
//                                     .wrapping_sub(1 as i32 as u32),
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 3 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_dc_ev_status(
//     stream: &mut ExiBitstream,
//     message:  &Iso2DCEVStatusType,
// ) -> Result<(), ExiError> {

//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     exi_basetypes_encoder_bool(stream, message.ev_ready)?;
//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     exi_basetypes_encoder_nbit_uint(stream, 4, message.ev_error_code as u32)?;
//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     exi_basetypes_encoder_nbit_uint(stream, 7, message.ev_res_soc as u32)?;
//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)
// }

// fn encode_iso2_ParameterSetType(
//     stream: &mut ExiBitstream,
//     mut ParameterSetType: *const iso2_ParameterSetType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 83 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     let mut Parameter_currentIndex: u16 = 0 as i32 as u16;
//     while done == 0 {
//         match grammar_id {
//             83 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_integer_16(
//                             stream,
//                             (*ParameterSetType).ParameterSetID,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 84 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             84 => {
//                 if (Parameter_currentIndex as i32) < (*ParameterSetType).Parameter.arrayLen as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         let fresh16 = Parameter_currentIndex;
//                         Parameter_currentIndex = Parameter_currentIndex.wrapping_add(1);
//                         error = encode_iso2_ParameterType(
//                             stream,
//                             &*((*ParameterSetType).Parameter.array)
//                                 .as_ptr()
//                                 .offset(fresh16 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 85 as i32;
//                         }
//                     }
//                 } else {
//                     error = -(150 as i32);
//                 }
//             }
//             85 => {
//                 if (Parameter_currentIndex as i32) < (*ParameterSetType).Parameter.arrayLen as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         let fresh17 = Parameter_currentIndex;
//                         Parameter_currentIndex = Parameter_currentIndex.wrapping_add(1);
//                         error = encode_iso2_ParameterType(
//                             stream,
//                             &*((*ParameterSetType).Parameter.array)
//                                 .as_ptr()
//                                 .offset(fresh17 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 85 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_SAScheduleTupleType(
//     stream: &mut ExiBitstream,
//     mut SAScheduleTupleType: *const iso2_SAScheduleTupleType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 86 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             86 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             8 as i32 as usize,
//                             ((*SAScheduleTupleType).SAScheduleTupleID as u32)
//                                 .wrapping_sub(1 as i32 as u32),
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 87 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             87 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         encode_iso2_PMaxScheduleType(stream, &(*SAScheduleTupleType).PMaxSchedule);
//                     if error == 0 as i32 {
//                         grammar_id = 88 as i32;
//                     }
//                 }
//             }
//             88 => {
//                 if (*SAScheduleTupleType).SalesTariff_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_SalesTariffType(
//                             stream,
//                             &(*SAScheduleTupleType).SalesTariff,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_SelectedServiceType(
//     stream: &mut ExiBitstream,
//     mut SelectedServiceType: *const iso2_SelectedServiceType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 89 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             89 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error =
//                             exi_basetypes_encoder_uint_16(stream, (*SelectedServiceType).ServiceID);
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 90 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             90 => {
//                 if (*SelectedServiceType).ParameterSetID_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_integer_16(
//                                 stream,
//                                 (*SelectedServiceType).ParameterSetID,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 3 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
fn encode_iso2_service(
    stream: &mut ExiBitstream,
    message: &Iso2ServiceType,
) -> Result<(), ExiError> {
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    exi_basetypes_encoder_uint_16(stream, message.service_id)?;
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

    if let Some(ref name) = message.service_name {
        exi_basetypes_encoder_nbit_uint(stream, 2, 0)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
        let Ok(len) = u16::try_from(name.len()) else {
            return Err(ExiError::InvalidCharactersLength);
        };
        exi_basetypes_encoder_uint_16(stream, len)?;
        exi_basetypes_encoder_characters(stream, name)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    } else {
        exi_basetypes_encoder_nbit_uint(stream, 2, 1)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
        exi_basetypes_encoder_nbit_uint(stream, 2, message.service_category as u32)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    }

    if let Some(ref scope) = message.service_scope {
        exi_basetypes_encoder_nbit_uint(stream, 2, 0)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
        let Ok(len) = u16::try_from(scope.len()) else {
            return Err(ExiError::InvalidCharactersLength);
        };
        exi_basetypes_encoder_uint_16(stream, len)?;
        exi_basetypes_encoder_characters(stream, scope)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    } else {
        exi_basetypes_encoder_nbit_uint(stream, 2, 1)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
        exi_basetypes_encoder_bool(stream, message.free_service)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    }

    Ok(())
}
// fn encode_iso2_SignatureValueType(
//     stream: &mut ExiBitstream,
//     mut SignatureValueType: *const iso2_SignatureValueType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 96 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             96 => {
//                 if (*SignatureValueType).Id_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*SignatureValueType).Id.charactersLen as i32 + 2 as i32) as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*SignatureValueType).Id.charactersLen as usize,
//                                 ((*SignatureValueType).Id.characters).as_ptr(),
//                                 (64 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 97 as i32;
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             (*SignatureValueType).CONTENT.bytesLen,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bytes(
//                                 stream,
//                                 (*SignatureValueType).CONTENT.bytesLen as usize,
//                                 ((*SignatureValueType).CONTENT.bytes).as_ptr(),
//                                 350 as i32 as usize,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             97 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = exi_basetypes_encoder_uint_16(
//                         stream,
//                         (*SignatureValueType).CONTENT.bytesLen,
//                     );
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_bytes(
//                             stream,
//                             (*SignatureValueType).CONTENT.bytesLen as usize,
//                             ((*SignatureValueType).CONTENT.bytes).as_ptr(),
//                             350 as i32 as usize,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_SubCertificatesType(
//     stream: &mut ExiBitstream,
//     mut SubCertificatesType: *const iso2_SubCertificatesType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 98 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     let mut Certificate_currentIndex: u16 = 0 as i32 as u16;
//     while done == 0 {
//         match grammar_id {
//             98 => {
//                 if (Certificate_currentIndex as i32)
//                     < (*SubCertificatesType).Certificate.arrayLen as i32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*SubCertificatesType).Certificate.array
//                                     [Certificate_currentIndex as usize]
//                                     .bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*SubCertificatesType).Certificate.array
//                                         [Certificate_currentIndex as usize]
//                                         .bytesLen as usize,
//                                     ((*SubCertificatesType).Certificate.array
//                                         [Certificate_currentIndex as usize]
//                                         .bytes)
//                                         .as_ptr(),
//                                     800 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     Certificate_currentIndex =
//                                         Certificate_currentIndex.wrapping_add(1);
//                                     Certificate_currentIndex;
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 99 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error = -(150 as i32);
//                 }
//             }
//             99 => {
//                 if (Certificate_currentIndex as i32)
//                     < (*SubCertificatesType).Certificate.arrayLen as i32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*SubCertificatesType).Certificate.array
//                                     [Certificate_currentIndex as usize]
//                                     .bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*SubCertificatesType).Certificate.array
//                                         [Certificate_currentIndex as usize]
//                                         .bytesLen as usize,
//                                     ((*SubCertificatesType).Certificate.array
//                                         [Certificate_currentIndex as usize]
//                                         .bytes)
//                                         .as_ptr(),
//                                     800 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     Certificate_currentIndex =
//                                         Certificate_currentIndex.wrapping_add(1);
//                                     Certificate_currentIndex;
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 99 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_KeyInfoType(
//     stream: &mut ExiBitstream,
//     mut KeyInfoType: *const iso2_KeyInfoType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 100 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             100 => {
//                 if (*KeyInfoType).Id_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*KeyInfoType).Id.charactersLen as i32 + 2 as i32) as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*KeyInfoType).Id.charactersLen as usize,
//                                 ((*KeyInfoType).Id.characters).as_ptr(),
//                                 (64 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 101 as i32;
//                             }
//                         }
//                     }
//                 } else if (*KeyInfoType).KeyName_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 ((*KeyInfoType).KeyName.charactersLen as i32 + 2 as i32) as u16,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_characters(
//                                     stream,
//                                     (*KeyInfoType).KeyName.charactersLen as usize,
//                                     ((*KeyInfoType).KeyName.characters).as_ptr(),
//                                     (64 as i32 + 1 as i32) as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 3 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else if (*KeyInfoType).KeyValue_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_KeyValueType(stream, &(*KeyInfoType).KeyValue);
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else if (*KeyInfoType).RetrievalMethod_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_RetrievalMethodType(
//                             stream,
//                             &(*KeyInfoType).RetrievalMethod,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else if (*KeyInfoType).X509Data_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_X509DataType(stream, &(*KeyInfoType).X509Data);
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else if (*KeyInfoType).PGPData_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_PGPDataType(stream, &(*KeyInfoType).PGPData);
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else if (*KeyInfoType).SPKIData_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_SPKIDataType(stream, &(*KeyInfoType).SPKIData);
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else if (*KeyInfoType).MgmtData_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 ((*KeyInfoType).MgmtData.charactersLen as i32 + 2 as i32) as u16,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_characters(
//                                     stream,
//                                     (*KeyInfoType).MgmtData.charactersLen as usize,
//                                     ((*KeyInfoType).MgmtData.characters).as_ptr(),
//                                     (64 as i32 + 1 as i32) as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 3 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else if (*KeyInfoType).ANY_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 8 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error =
//                                 exi_basetypes_encoder_uint_16(stream, (*KeyInfoType).ANY.bytesLen);
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*KeyInfoType).ANY.bytesLen as usize,
//                                     ((*KeyInfoType).ANY.bytes).as_ptr(),
//                                     4 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 3 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             101 => {
//                 if (*KeyInfoType).KeyName_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 ((*KeyInfoType).KeyName.charactersLen as i32 + 2 as i32) as u16,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_characters(
//                                     stream,
//                                     (*KeyInfoType).KeyName.charactersLen as usize,
//                                     ((*KeyInfoType).KeyName.characters).as_ptr(),
//                                     (64 as i32 + 1 as i32) as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 3 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else if (*KeyInfoType).KeyValue_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_KeyValueType(stream, &(*KeyInfoType).KeyValue);
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else if (*KeyInfoType).RetrievalMethod_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_RetrievalMethodType(
//                             stream,
//                             &(*KeyInfoType).RetrievalMethod,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else if (*KeyInfoType).X509Data_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_X509DataType(stream, &(*KeyInfoType).X509Data);
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else if (*KeyInfoType).PGPData_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_PGPDataType(stream, &(*KeyInfoType).PGPData);
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else if (*KeyInfoType).SPKIData_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_SPKIDataType(stream, &(*KeyInfoType).SPKIData);
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else if (*KeyInfoType).MgmtData_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 ((*KeyInfoType).MgmtData.charactersLen as i32 + 2 as i32) as u16,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_characters(
//                                     stream,
//                                     (*KeyInfoType).MgmtData.charactersLen as usize,
//                                     ((*KeyInfoType).MgmtData.characters).as_ptr(),
//                                     (64 as i32 + 1 as i32) as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 3 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else if (*KeyInfoType).ANY_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error =
//                                 exi_basetypes_encoder_uint_16(stream, (*KeyInfoType).ANY.bytesLen);
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*KeyInfoType).ANY.bytesLen as usize,
//                                     ((*KeyInfoType).ANY.bytes).as_ptr(),
//                                     4 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 3 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_ObjectType(
//     stream: &mut ExiBitstream,
//     mut ObjectType: *const iso2_ObjectType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 102 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             102 => {
//                 if (*ObjectType).Encoding_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*ObjectType).Encoding.charactersLen as i32 + 2 as i32) as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*ObjectType).Encoding.charactersLen as usize,
//                                 ((*ObjectType).Encoding.characters).as_ptr(),
//                                 (64 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 103 as i32;
//                             }
//                         }
//                     }
//                 } else if (*ObjectType).Id_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*ObjectType).Id.charactersLen as i32 + 2 as i32) as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*ObjectType).Id.charactersLen as usize,
//                                 ((*ObjectType).Id.characters).as_ptr(),
//                                 (64 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 104 as i32;
//                             }
//                         }
//                     }
//                 } else if (*ObjectType).MimeType_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*ObjectType).MimeType.charactersLen as i32 + 2 as i32) as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*ObjectType).MimeType.charactersLen as usize,
//                                 ((*ObjectType).MimeType.characters).as_ptr(),
//                                 (64 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 105 as i32;
//                             }
//                         }
//                     }
//                 } else if (*ObjectType).ANY_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error =
//                                 exi_basetypes_encoder_uint_16(stream, (*ObjectType).ANY.bytesLen);
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*ObjectType).ANY.bytesLen as usize,
//                                     ((*ObjectType).ANY.bytes).as_ptr(),
//                                     4 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 3 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             103 => {
//                 if (*ObjectType).Id_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*ObjectType).Id.charactersLen as i32 + 2 as i32) as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*ObjectType).Id.charactersLen as usize,
//                                 ((*ObjectType).Id.characters).as_ptr(),
//                                 (64 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 104 as i32;
//                             }
//                         }
//                     }
//                 } else if (*ObjectType).MimeType_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*ObjectType).MimeType.charactersLen as i32 + 2 as i32) as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*ObjectType).MimeType.charactersLen as usize,
//                                 ((*ObjectType).MimeType.characters).as_ptr(),
//                                 (64 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 105 as i32;
//                             }
//                         }
//                     }
//                 } else if (*ObjectType).ANY_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error =
//                                 exi_basetypes_encoder_uint_16(stream, (*ObjectType).ANY.bytesLen);
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*ObjectType).ANY.bytesLen as usize,
//                                     ((*ObjectType).ANY.bytes).as_ptr(),
//                                     4 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 3 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             104 => {
//                 if (*ObjectType).MimeType_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*ObjectType).MimeType.charactersLen as i32 + 2 as i32) as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*ObjectType).MimeType.charactersLen as usize,
//                                 ((*ObjectType).MimeType.characters).as_ptr(),
//                                 (64 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 105 as i32;
//                             }
//                         }
//                     }
//                 } else if (*ObjectType).ANY_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error =
//                                 exi_basetypes_encoder_uint_16(stream, (*ObjectType).ANY.bytesLen);
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*ObjectType).ANY.bytesLen as usize,
//                                     ((*ObjectType).ANY.bytes).as_ptr(),
//                                     4 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 3 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             105 => {
//                 if (*ObjectType).ANY_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error =
//                                 exi_basetypes_encoder_uint_16(stream, (*ObjectType).ANY.bytesLen);
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*ObjectType).ANY.bytesLen as usize,
//                                     ((*ObjectType).ANY.bytes).as_ptr(),
//                                     4 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 3 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
fn encode_iso2_supported_energy_transfer_mode(
    stream: &mut ExiBitstream,
    message: &Iso2SupportedEnergyTransferModeType,
) -> Result<(), ExiError> {
    if message.energy_transfer_mode.is_empty() {
        return Err(ExiError::ArrayOutOfBounds);
    }

    for (idx, mode) in message.energy_transfer_mode.iter().enumerate() {
        let bit_val = if idx == 0 { 1 } else { 2 };
        exi_basetypes_encoder_nbit_uint(stream, bit_val, 0)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
        exi_basetypes_encoder_nbit_uint(stream, 3, *mode as u32)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    }

    exi_basetypes_encoder_nbit_uint(stream, 2, 1)
}

// fn encode_iso2_CertificateChainType(
//     stream: &mut ExiBitstream,
//     mut CertificateChainType: *const iso2_CertificateChainType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 108 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             108 => {
//                 if (*CertificateChainType).Id_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*CertificateChainType).Id.charactersLen as i32 + 2 as i32) as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*CertificateChainType).Id.charactersLen as usize,
//                                 ((*CertificateChainType).Id.characters).as_ptr(),
//                                 (64 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 109 as i32;
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*CertificateChainType).Certificate.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*CertificateChainType).Certificate.bytesLen as usize,
//                                     ((*CertificateChainType).Certificate.bytes).as_ptr(),
//                                     800 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 110 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             109 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             (*CertificateChainType).Certificate.bytesLen,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bytes(
//                                 stream,
//                                 (*CertificateChainType).Certificate.bytesLen as usize,
//                                 ((*CertificateChainType).Certificate.bytes).as_ptr(),
//                                 800 as i32 as usize,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 110 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             110 => {
//                 if (*CertificateChainType).SubCertificates_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_SubCertificatesType(
//                             stream,
//                             &(*CertificateChainType).SubCertificates,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_BodyBaseType(
//     stream: &mut ExiBitstream,
//     mut BodyBaseType: *const iso2_BodyBaseType,
// ) -> Result<(), ExiError> {
//     let mut error: i32 =
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//     return error;
// }
fn encode_iso2_notification(
    stream: &mut ExiBitstream,
    message: &Iso2NotificationType,
) -> Result<(), ExiError> {
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    exi_basetypes_encoder_nbit_uint(stream, 2, message.fault_code as u32)?;
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

    if let Some(ref fault_msg) = message.fault_msg {
        exi_basetypes_encoder_nbit_uint(stream, 2, 0)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

        let Ok(len) = u16::try_from(fault_msg.len()) else {
            return Err(ExiError::InvalidCharactersLength);
        };

        exi_basetypes_encoder_uint_16(stream, len)?;
        exi_basetypes_encoder_characters(stream, fault_msg)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    } else {
        exi_basetypes_encoder_nbit_uint(stream, 2, 1)?;
    }

    exi_basetypes_encoder_nbit_uint(stream, 1, 0)
}
// fn encode_iso2_DC_EVSEStatusType(
//     stream: &mut ExiBitstream,
//     mut DC_EVSEStatusType: *const iso2_DC_EVSEStatusType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 113 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             113 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             (*DC_EVSEStatusType).NotificationMaxDelay,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 114 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             114 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             2 as i32 as usize,
//                             (*DC_EVSEStatusType).EVSENotification as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 115 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             115 => {
//                 if (*DC_EVSEStatusType).EVSEIsolationStatus_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 3 as i32 as usize,
//                                 (*DC_EVSEStatusType).EVSEIsolationStatus as u32,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 116 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 4 as i32 as usize,
//                                 (*DC_EVSEStatusType).EVSEStatusCode as u32,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 3 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             116 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             4 as i32 as usize,
//                             (*DC_EVSEStatusType).EVSEStatusCode as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_SelectedServiceListType(
//     stream: &mut ExiBitstream,
//     mut SelectedServiceListType: *const iso2_SelectedServiceListType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 117 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     let mut SelectedService_currentIndex: u16 = 0 as i32 as u16;
//     while done == 0 {
//         match grammar_id {
//             117 => {
//                 if (SelectedService_currentIndex as i32)
//                     < (*SelectedServiceListType).SelectedService.arrayLen as i32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         let fresh20 = SelectedService_currentIndex;
//                         SelectedService_currentIndex = SelectedService_currentIndex.wrapping_add(1);
//                         error = encode_iso2_SelectedServiceType(
//                             stream,
//                             &*((*SelectedServiceListType).SelectedService.array)
//                                 .as_ptr()
//                                 .offset(fresh20 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 118 as i32;
//                         }
//                     }
//                 } else {
//                     error = -(150 as i32);
//                 }
//             }
//             118 => {
//                 if (SelectedService_currentIndex as i32)
//                     < (*SelectedServiceListType).SelectedService.arrayLen as i32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         let fresh21 = SelectedService_currentIndex;
//                         SelectedService_currentIndex = SelectedService_currentIndex.wrapping_add(1);
//                         error = encode_iso2_SelectedServiceType(
//                             stream,
//                             &*((*SelectedServiceListType).SelectedService.array)
//                                 .as_ptr()
//                                 .offset(fresh21 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 118 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
fn encode_iso2_payment_option_list(
    stream: &mut ExiBitstream,
    message: &Iso2PaymentOptionListType,
) -> Result<(), ExiError> {
    if message.payment_option.len() > 0 {
        for (i, option) in message.payment_option.iter().enumerate() {
            exi_basetypes_encoder_nbit_uint(stream, i + 1, 0)?;
            exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
            exi_basetypes_encoder_nbit_uint(stream, 1, *option as u32)?;
            exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
        }

        match message.payment_option.len() {
            1 => exi_basetypes_encoder_nbit_uint(stream, 2, 1)?,
            2 => exi_basetypes_encoder_nbit_uint(stream, 1, 0)?,
            _ => return Err(ExiError::ArrayOutOfBounds),
        }
    } else {
        return Err(ExiError::ArrayOutOfBounds);
    }
    Ok(())
}
// // fn encode_iso2_signature(
//     stream: &mut ExiBitstream,
//     mut message: &mut Iso2SignatureType,
// ) -> Result<(), ExiError>{
//     let mut grammar_id: i32 = 121 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             121 => {
//                 if let Some(ref mut id) = message.id {
//                     exi_basetypes_encoder_nbit_uint(stream, 2, 0)?;
//                     let Ok(len) = u16::try_from(id.len()) else {
//                         return Err(ExiError::InvalidCharactersLength);
//                     };
//                     exi_basetypes_encoder_uint_16(stream,len)?;
//                     exi_basetypes_encoder_characters(stream, id)?;
//                     grammar_id = 122 as i32;

//                 } else {
//                     exi_basetypes_encoder_nbit_uint(stream, 2, 1)?;
//                     error = encode_iso2_SignedInfoType(stream, &(*message).signed_info)?;
//                     grammar_id = 123 as i32;
//                 }
//             }
//             122 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_SignedInfoType(stream, &(*message).SignedInfo);
//                     if error == 0 as i32 {
//                         grammar_id = 123 as i32;
//                     }
//                 }
//             }
//             123 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         encode_iso2_SignatureValueType(stream, &(*message).SignatureValue);
//                     if error == 0 as i32 {
//                         grammar_id = 124 as i32;
//                     }
//                 }
//             }
//             124 => {
//                 if (*message).KeyInfo_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_KeyInfoType(stream, &(*message).KeyInfo);
//                         if error == 0 as i32 {
//                             grammar_id = 126 as i32;
//                         }
//                     }
//                 } else if (*message).Object_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_ObjectType(stream, &(*message).Object);
//                         if error == 0 as i32 {
//                             grammar_id = 125 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             125 => {
//                 if 1 as i32 == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_ObjectType(stream, &(*message).Object);
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             126 => {
//                 if (*message).Object_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_ObjectType(stream, &(*message).Object);
//                         if error == 0 as i32 {
//                             grammar_id = 127 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             127 => {
//                 if 1 as i32 == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_ObjectType(stream, &(*message).Object);
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 break;
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_ChargingProfileType(
//     stream: &mut ExiBitstream,
//     mut ChargingProfileType: *const iso2_ChargingProfileType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 128 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     let mut ProfileEntry_currentIndex: u16 = 0 as i32 as u16;
//     while done == 0 {
//         match grammar_id {
//             128 => {
//                 if (ProfileEntry_currentIndex as i32)
//                     < (*ChargingProfileType).ProfileEntry.arrayLen as i32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         let fresh24 = ProfileEntry_currentIndex;
//                         ProfileEntry_currentIndex = ProfileEntry_currentIndex.wrapping_add(1);
//                         error = encode_iso2_ProfileEntryType(
//                             stream,
//                             &*((*ChargingProfileType).ProfileEntry.array)
//                                 .as_ptr()
//                                 .offset(fresh24 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 129 as i32;
//                         }
//                     }
//                 } else {
//                     error = -(150 as i32);
//                 }
//             }
//             129 => {
//                 if (ProfileEntry_currentIndex as i32)
//                     < (*ChargingProfileType).ProfileEntry.arrayLen as i32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         let fresh25 = ProfileEntry_currentIndex;
//                         ProfileEntry_currentIndex = ProfileEntry_currentIndex.wrapping_add(1);
//                         error = encode_iso2_ProfileEntryType(
//                             stream,
//                             &*((*ChargingProfileType).ProfileEntry.array)
//                                 .as_ptr()
//                                 .offset(fresh25 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 129 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_ServiceParameterListType(
//     stream: &mut ExiBitstream,
//     mut ServiceParameterListType: *const iso2_ServiceParameterListType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 130 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     let mut ParameterSet_currentIndex: u16 = 0 as i32 as u16;
//     while done == 0 {
//         match grammar_id {
//             130 => {
//                 if (ParameterSet_currentIndex as i32)
//                     < (*ServiceParameterListType).ParameterSet.arrayLen as i32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         let fresh26 = ParameterSet_currentIndex;
//                         ParameterSet_currentIndex = ParameterSet_currentIndex.wrapping_add(1);
//                         error = encode_iso2_ParameterSetType(
//                             stream,
//                             &*((*ServiceParameterListType).ParameterSet.array)
//                                 .as_ptr()
//                                 .offset(fresh26 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 131 as i32;
//                         }
//                     }
//                 } else {
//                     error = -(150 as i32);
//                 }
//             }
//             131 => {
//                 if (ParameterSet_currentIndex as i32)
//                     < (*ServiceParameterListType).ParameterSet.arrayLen as i32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         let fresh27 = ParameterSet_currentIndex;
//                         ParameterSet_currentIndex = ParameterSet_currentIndex.wrapping_add(1);
//                         error = encode_iso2_ParameterSetType(
//                             stream,
//                             &*((*ServiceParameterListType).ParameterSet.array)
//                                 .as_ptr()
//                                 .offset(fresh27 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 131 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_ListOfRootCertificateIDsType(
//     stream: &mut ExiBitstream,
//     mut ListOfRootCertificateIDsType: *const iso2_ListOfRootCertificateIDsType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 132 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     let mut RootCertificateID_currentIndex: u16 = 0 as i32 as u16;
//     while done == 0 {
//         match grammar_id {
//             132 => {
//                 if (RootCertificateID_currentIndex as i32)
//                     < (*ListOfRootCertificateIDsType).RootCertificateID.arrayLen as i32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         let fresh28 = RootCertificateID_currentIndex;
//                         RootCertificateID_currentIndex =
//                             RootCertificateID_currentIndex.wrapping_add(1);
//                         error = encode_iso2_X509IssuerSerialType(
//                             stream,
//                             &*((*ListOfRootCertificateIDsType).RootCertificateID.array)
//                                 .as_ptr()
//                                 .offset(fresh28 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 133 as i32;
//                         }
//                     }
//                 } else {
//                     error = -(150 as i32);
//                 }
//             }
//             133 => {
//                 if (RootCertificateID_currentIndex as i32)
//                     < (*ListOfRootCertificateIDsType).RootCertificateID.arrayLen as i32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         let fresh29 = RootCertificateID_currentIndex;
//                         RootCertificateID_currentIndex =
//                             RootCertificateID_currentIndex.wrapping_add(1);
//                         error = encode_iso2_X509IssuerSerialType(
//                             stream,
//                             &*((*ListOfRootCertificateIDsType).RootCertificateID.array)
//                                 .as_ptr()
//                                 .offset(fresh29 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 133 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_ac_evcharge_parameter(
//     stream: &mut ExiBitstream,
//     message: &Iso2ACEVChargeParameterType,
// ) -> Result<(), ExiError> {

//     if let Some(departure_time) = &message.departure_time {
//         exi_basetypes_encoder_nbit_uint(stream, 2, 0)?;
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//         exi_basetypes_encoder_uint_32(stream, *departure_time)?;
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     } else {
//         exi_basetypes_encoder_nbit_uint(stream, 2, 1)?;
//     }

//     encode_iso2_physical_value(stream, &message.e_amount)?;

//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     encode_iso2_physical_value(stream, &message.ev_max_voltage)?;

//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     encode_iso2_physical_value(stream, &message.ev_max_current)?;

//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     encode_iso2_physical_value(stream, &message.ev_min_current)?;

//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)
// }

// fn encode_iso2_dc_evcharge_parameter(
//     stream: &mut ExiBitstream,
//     message: &Iso2DCEVChargeParameterType,
// ) -> Result<(), ExiError> {

//     if let Some(departure_time) = &message.departure_time {
//         exi_basetypes_encoder_nbit_uint(stream, 2, 0)?;
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//         exi_basetypes_encoder_uint_32(stream, *departure_time)?;
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     } else {
//         exi_basetypes_encoder_nbit_uint(stream, 2, 1)?;
//     }

//     encode_iso2_dc_ev_status(stream, &message.dc_ev_status)?;

//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     encode_iso2_physical_value(stream, &message.ev_maximum_current_limit)?;

//     if let Some(ev_maximum_power_limit) = &message.ev_maximum_power_limit {
//         exi_basetypes_encoder_nbit_uint(stream, 2, 0)?;
//         encode_iso2_physical_value(stream, ev_maximum_power_limit)?;
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     } else {
//         exi_basetypes_encoder_nbit_uint(stream, 2, 1)?;
//     }

//     encode_iso2_physical_value(stream, &message.ev_maximum_voltage_limit)?;

//     if let Some(ev_energy_capacity) = &message.ev_energy_capacity {
//         exi_basetypes_encoder_nbit_uint(stream, 3, 0)?;
//         encode_iso2_physical_value(stream, ev_energy_capacity)?;
//     }
//     // ev_energy_request branch
//     else if let Some(ev_energy_request) = &message.ev_energy_request {
//         exi_basetypes_encoder_nbit_uint(stream, 3, 1)?;
//         encode_iso2_physical_value(stream, ev_energy_request)?;
//     }
//     // full_soc branch
//     else if let Some(full_soc) = &message.full_soc {
//         exi_basetypes_encoder_nbit_uint(stream, 3, 2)?;
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//         exi_basetypes_encoder_nbit_uint(stream, 7, *full_soc as u32)?;
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     }
//     // bulk_soc branch
//     else if let Some(bulk_soc) = &message.bulk_soc {
//         exi_basetypes_encoder_nbit_uint(stream, 3, 3)?;
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//         exi_basetypes_encoder_nbit_uint(stream, 7, *bulk_soc as u32)?;
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//         return Ok(()); // nothing more after bulk_soc here
//     }
//     // none present
//     else {
//         exi_basetypes_encoder_nbit_uint(stream, 3, 4)?;
//         return Ok(());
//     }

//     // Secondary level: ev_energy_request / full_soc / bulk_soc after capacity or request
//     if let Some(ev_energy_request) = &message.ev_energy_request {
//         exi_basetypes_encoder_nbit_uint(stream, 3, 0)?;
//         encode_iso2_physical_value(stream, ev_energy_request)?;
//     }
//     else if let Some(full_soc) = &message.full_soc {
//         exi_basetypes_encoder_nbit_uint(stream, 3, 1)?;
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//         exi_basetypes_encoder_nbit_uint(stream, 7, *full_soc as u32)?;
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     }
//     else if let Some(bulk_soc) = &message.bulk_soc {
//         exi_basetypes_encoder_nbit_uint(stream, 3, 2)?;
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//         exi_basetypes_encoder_nbit_uint(stream, 7, *bulk_soc as u32)?;
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//         return Ok(());
//     }
//     else {
//         exi_basetypes_encoder_nbit_uint(stream, 3, 3)?;
//         return Ok(());
//     }

//     // Final level: bulk_soc after full_soc
//     if let Some(bulk_soc) = &message.bulk_soc {
//         exi_basetypes_encoder_nbit_uint(stream, 2, 0)?;
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//         exi_basetypes_encoder_nbit_uint(stream, 7, *bulk_soc as u32)?;
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     } else {
//         exi_basetypes_encoder_nbit_uint(stream, 2, 1)?;
//     }

//     Ok(())
// }

// fn encode_iso2_evcharge_parameter(
//     stream: &mut ExiBitstream,
//     message: &Iso2EVChargeParameterType,
// ) -> Result<(), ExiError> {

//     if let Some(departure_time) = message.departure_time {
//         exi_basetypes_encoder_nbit_uint(stream, 2, 0)?;
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//         exi_basetypes_encoder_uint_32(stream, departure_time)?;
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     } else {
//         exi_basetypes_encoder_nbit_uint(stream, 2, 1)?;
//     }

//     encode_iso2_ac_evcharge_parameter(stream, &message.ac_ev_charge_parameter)?;

//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     encode_iso2_dc_evcharge_parameter(stream, &message.dc_ev_charge_parameter)?;

//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)

// }

// fn encode_iso2_SASchedulesType(
//     stream: &mut ExiBitstream,
//     mut SASchedulesType: *const iso2_SASchedulesType,
// ) -> Result<(), ExiError> {
//     let mut error: i32 =
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//     return error;
// }
// fn encode_iso2_SAScheduleListType(
//     stream: &mut ExiBitstream,
//     mut SAScheduleListType: *const iso2_SAScheduleListType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 151 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     let mut SAScheduleTuple_currentIndex: u16 = 0 as i32 as u16;
//     while done == 0 {
//         match grammar_id {
//             151 => {
//                 if (SAScheduleTuple_currentIndex as i32)
//                     < (*SAScheduleListType).SAScheduleTuple.arrayLen as i32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         let fresh30 = SAScheduleTuple_currentIndex;
//                         SAScheduleTuple_currentIndex = SAScheduleTuple_currentIndex.wrapping_add(1);
//                         error = encode_iso2_SAScheduleTupleType(
//                             stream,
//                             &*((*SAScheduleListType).SAScheduleTuple.array)
//                                 .as_ptr()
//                                 .offset(fresh30 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 152 as i32;
//                         }
//                     }
//                 } else {
//                     error = -(150 as i32);
//                 }
//             }
//             152 => {
//                 if (SAScheduleTuple_currentIndex as i32)
//                     < (*SAScheduleListType).SAScheduleTuple.arrayLen as i32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         let fresh31 = SAScheduleTuple_currentIndex;
//                         SAScheduleTuple_currentIndex = SAScheduleTuple_currentIndex.wrapping_add(1);
//                         error = encode_iso2_SAScheduleTupleType(
//                             stream,
//                             &*((*SAScheduleListType).SAScheduleTuple.array)
//                                 .as_ptr()
//                                 .offset(fresh31 as isize),
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 152 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
fn encode_iso2_charge_service(
    stream: &mut ExiBitstream,
    message: &Iso2ChargeServiceType,
) -> Result<(), ExiError> {
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    exi_basetypes_encoder_uint_16(stream, message.service_id)?;
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

    if let Some(service_name) = &message.service_name {
        exi_basetypes_encoder_nbit_uint(stream, 2, 0)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

        let len: u16 =
            u16::try_from(service_name.len()).map_err(|_| ExiError::InvalidCharactersLength)?;
        exi_basetypes_encoder_uint_16(stream, len + 2)?;
        exi_basetypes_encoder_characters(stream, service_name)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    } else {
        exi_basetypes_encoder_nbit_uint(stream, 2, 1)?;
    }
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    exi_basetypes_encoder_nbit_uint(stream, 2, message.service_category as u32)?;
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

    if let Some(service_scope) = &message.service_scope {
        exi_basetypes_encoder_nbit_uint(stream, 2, 0)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
        let len: u16 =
            u16::try_from(service_scope.len()).map_err(|_| ExiError::InvalidCharactersLength)?;
        exi_basetypes_encoder_uint_16(stream, len + 2)?;
        exi_basetypes_encoder_characters(stream, service_scope)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    } else {
        exi_basetypes_encoder_nbit_uint(stream, 2, 1)?;
    }

    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    exi_basetypes_encoder_bool(stream, message.free_service)?;
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    encode_iso2_supported_energy_transfer_mode(stream, &message.supported_energy_transfer_mode)?;
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)
}

// fn encode_iso2_EVPowerDeliveryParameterType(
//     stream: &mut ExiBitstream,
//     mut EVPowerDeliveryParameterType: *const iso2_EVPowerDeliveryParameterType,
// ) -> Result<(), ExiError> {
//     let mut error: i32 =
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//     return error;
// }
// fn encode_iso2_DC_EVPowerDeliveryParameterType(
//     stream: &mut ExiBitstream,
//     mut DC_EVPowerDeliveryParameterType: *const iso2_DC_EVPowerDeliveryParameterType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 159 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             159 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_message(
//                         stream,
//                         &(*DC_EVPowerDeliveryParameterType).DC_EVStatus,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 160 as i32;
//                     }
//                 }
//             }
//             160 => {
//                 if (*DC_EVPowerDeliveryParameterType).BulkChargingComplete_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bool(
//                                 stream,
//                                 (*DC_EVPowerDeliveryParameterType).BulkChargingComplete,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 161 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bool(
//                                 stream,
//                                 (*DC_EVPowerDeliveryParameterType).ChargingComplete,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 3 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             161 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_bool(
//                             stream,
//                             (*DC_EVPowerDeliveryParameterType).ChargingComplete,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_ContractSignatureEncryptedPrivateKeyType(
//     stream: &mut ExiBitstream,
//     mut ContractSignatureEncryptedPrivateKeyType: *const iso2_ContractSignatureEncryptedPrivateKeyType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 162 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             162 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = exi_basetypes_encoder_uint_16(
//                         stream,
//                         ((*ContractSignatureEncryptedPrivateKeyType).Id.charactersLen as i32
//                             + 2 as i32) as u16,
//                     );
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_characters(
//                             stream,
//                             (*ContractSignatureEncryptedPrivateKeyType).Id.charactersLen as usize,
//                             ((*ContractSignatureEncryptedPrivateKeyType).Id.characters).as_ptr(),
//                             (64 as i32 + 1 as i32) as usize,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 163 as i32;
//                         }
//                     }
//                 }
//             }
//             163 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = exi_basetypes_encoder_uint_16(
//                         stream,
//                         (*ContractSignatureEncryptedPrivateKeyType).CONTENT.bytesLen,
//                     );
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_bytes(
//                             stream,
//                             (*ContractSignatureEncryptedPrivateKeyType).CONTENT.bytesLen as usize,
//                             ((*ContractSignatureEncryptedPrivateKeyType).CONTENT.bytes).as_ptr(),
//                             350 as i32 as usize,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_EVSEChargeParameterType(
//     stream: &mut ExiBitstream,
//     mut EVSEChargeParameterType: *const iso2_EVSEChargeParameterType,
// ) -> Result<(), ExiError> {
//     let mut error: i32 =
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//     return error;
// }
// fn encode_iso2_DC_EVSEChargeParameterType(
//     stream: &mut ExiBitstream,
//     mut DC_EVSEChargeParameterType: *const iso2_DC_EVSEChargeParameterType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 164 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             164 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_DC_EVSEStatusType(
//                         stream,
//                         &(*DC_EVSEChargeParameterType).DC_EVSEStatus,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 165 as i32;
//                     }
//                 }
//             }
//             165 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_PhysicalValueType(
//                         stream,
//                         &(*DC_EVSEChargeParameterType).EVSEMaximumCurrentLimit,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 166 as i32;
//                     }
//                 }
//             }
//             166 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_PhysicalValueType(
//                         stream,
//                         &(*DC_EVSEChargeParameterType).EVSEMaximumPowerLimit,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 167 as i32;
//                     }
//                 }
//             }
//             167 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_PhysicalValueType(
//                         stream,
//                         &(*DC_EVSEChargeParameterType).EVSEMaximumVoltageLimit,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 168 as i32;
//                     }
//                 }
//             }
//             168 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_PhysicalValueType(
//                         stream,
//                         &(*DC_EVSEChargeParameterType).EVSEMinimumCurrentLimit,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 169 as i32;
//                     }
//                 }
//             }
//             169 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_PhysicalValueType(
//                         stream,
//                         &(*DC_EVSEChargeParameterType).EVSEMinimumVoltageLimit,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 170 as i32;
//                     }
//                 }
//             }
//             170 => {
//                 if (*DC_EVSEChargeParameterType).EVSECurrentRegulationTolerance_isUsed() == 1 as u32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_PhysicalValueType(
//                             stream,
//                             &(*DC_EVSEChargeParameterType).EVSECurrentRegulationTolerance,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 171 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_PhysicalValueType(
//                             stream,
//                             &(*DC_EVSEChargeParameterType).EVSEPeakCurrentRipple,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 172 as i32;
//                         }
//                     }
//                 }
//             }
//             171 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_PhysicalValueType(
//                         stream,
//                         &(*DC_EVSEChargeParameterType).EVSEPeakCurrentRipple,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 172 as i32;
//                     }
//                 }
//             }
//             172 => {
//                 if (*DC_EVSEChargeParameterType).EVSEEnergyToBeDelivered_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_PhysicalValueType(
//                             stream,
//                             &(*DC_EVSEChargeParameterType).EVSEEnergyToBeDelivered,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
fn encode_iso2_service_list(
    stream: &mut ExiBitstream,
    message: &Iso2ServiceListType,
) -> Result<(), ExiError> {
    if message.service.is_empty() {
        return Err(ExiError::ArrayOutOfBounds);
    }

    for (idx, service) in message.service.iter().enumerate() {
        let bit_val = if idx == 0 { 1 } else { 2 };
        exi_basetypes_encoder_nbit_uint(stream, bit_val, 0)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
        encode_iso2_service(stream, service)?;
    }

    exi_basetypes_encoder_nbit_uint(stream, 2, 1)
}

// fn encode_iso2_DiffieHellmanPublickeyType(
//     stream: &mut ExiBitstream,
//     mut DiffieHellmanPublickeyType: *const iso2_DiffieHellmanPublickeyType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 175 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             175 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = exi_basetypes_encoder_uint_16(
//                         stream,
//                         ((*DiffieHellmanPublickeyType).Id.charactersLen as i32 + 2 as i32) as u16,
//                     );
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_characters(
//                             stream,
//                             (*DiffieHellmanPublickeyType).Id.charactersLen as usize,
//                             ((*DiffieHellmanPublickeyType).Id.characters).as_ptr(),
//                             (64 as i32 + 1 as i32) as usize,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 176 as i32;
//                         }
//                     }
//                 }
//             }
//             176 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = exi_basetypes_encoder_uint_16(
//                         stream,
//                         (*DiffieHellmanPublickeyType).CONTENT.bytesLen,
//                     );
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_bytes(
//                             stream,
//                             (*DiffieHellmanPublickeyType).CONTENT.bytesLen as usize,
//                             ((*DiffieHellmanPublickeyType).CONTENT.bytes).as_ptr(),
//                             350 as i32 as usize,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_EMAIDType(
//     stream: &mut ExiBitstream,
//     mut EMAIDType: *const iso2_EMAIDType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 177 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             177 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = exi_basetypes_encoder_uint_16(
//                         stream,
//                         ((*EMAIDType).Id.charactersLen as i32 + 2 as i32) as u16,
//                     );
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_characters(
//                             stream,
//                             (*EMAIDType).Id.charactersLen as usize,
//                             ((*EMAIDType).Id.characters).as_ptr(),
//                             (64 as i32 + 1 as i32) as usize,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 178 as i32;
//                         }
//                     }
//                 }
//             }
//             178 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = exi_basetypes_encoder_uint_16(
//                         stream,
//                         ((*EMAIDType).CONTENT.charactersLen as i32 + 2 as i32) as u16,
//                     );
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_characters(
//                             stream,
//                             (*EMAIDType).CONTENT.charactersLen as usize,
//                             ((*EMAIDType).CONTENT.characters).as_ptr(),
//                             (64 as i32 + 1 as i32) as usize,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_AC_EVSEStatusType(
//     stream: &mut ExiBitstream,
//     mut AC_EVSEStatusType: *const iso2_AC_EVSEStatusType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 179 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             179 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             (*AC_EVSEStatusType).NotificationMaxDelay,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 180 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             180 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             2 as i32 as usize,
//                             (*AC_EVSEStatusType).EVSENotification as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 181 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             181 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_bool(stream, (*AC_EVSEStatusType).RCD);
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_EVSEStatusType(
//     stream: &mut ExiBitstream,
//     mut EVSEStatusType: *const iso2_EVSEStatusType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 182 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             182 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             (*EVSEStatusType).NotificationMaxDelay,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 183 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             183 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             2 as i32 as usize,
//                             (*EVSEStatusType).EVSENotification as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 184 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             184 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_AC_EVSEStatusType(stream, &(*EVSEStatusType).AC_EVSEStatus);
//                     if error == 0 as i32 {
//                         grammar_id = 185 as i32;
//                     }
//                 }
//             }
//             185 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_DC_EVSEStatusType(stream, &(*EVSEStatusType).DC_EVSEStatus);
//                     if error == 0 as i32 {
//                         grammar_id = 3 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_AC_EVSEChargeParameterType(
//     stream: &mut ExiBitstream,
//     mut AC_EVSEChargeParameterType: *const iso2_AC_EVSEChargeParameterType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 186 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             186 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_AC_EVSEStatusType(
//                         stream,
//                         &(*AC_EVSEChargeParameterType).AC_EVSEStatus,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 187 as i32;
//                     }
//                 }
//             }
//             187 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_PhysicalValueType(
//                         stream,
//                         &(*AC_EVSEChargeParameterType).EVSENominalVoltage,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 188 as i32;
//                     }
//                 }
//             }
//             188 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_PhysicalValueType(
//                         stream,
//                         &(*AC_EVSEChargeParameterType).EVSEMaxCurrent,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 3 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_MeterInfoType(
//     stream: &mut ExiBitstream,
//     mut MeterInfoType: *const iso2_MeterInfoType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 189 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             189 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*MeterInfoType).MeterID.charactersLen as i32 + 2 as i32) as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*MeterInfoType).MeterID.charactersLen as usize,
//                                 ((*MeterInfoType).MeterID.characters).as_ptr(),
//                                 (32 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 190 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             190 => {
//                 if (*MeterInfoType).MeterReading_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_64(
//                                 stream,
//                                 (*MeterInfoType).MeterReading,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 191 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else if (*MeterInfoType).SigMeterReading_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*MeterInfoType).SigMeterReading.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*MeterInfoType).SigMeterReading.bytesLen as usize,
//                                     ((*MeterInfoType).SigMeterReading.bytes).as_ptr(),
//                                     64 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 192 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else if (*MeterInfoType).MeterStatus_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_integer_16(
//                                 stream,
//                                 (*MeterInfoType).MeterStatus,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 193 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else if (*MeterInfoType).TMeter_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error =
//                                 exi_basetypes_encoder_integer_64(stream, (*MeterInfoType).TMeter);
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 3 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             191 => {
//                 if (*MeterInfoType).SigMeterReading_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*MeterInfoType).SigMeterReading.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*MeterInfoType).SigMeterReading.bytesLen as usize,
//                                     ((*MeterInfoType).SigMeterReading.bytes).as_ptr(),
//                                     64 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 192 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else if (*MeterInfoType).MeterStatus_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_integer_16(
//                                 stream,
//                                 (*MeterInfoType).MeterStatus,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 193 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else if (*MeterInfoType).TMeter_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error =
//                                 exi_basetypes_encoder_integer_64(stream, (*MeterInfoType).TMeter);
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 3 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             192 => {
//                 if (*MeterInfoType).MeterStatus_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_integer_16(
//                                 stream,
//                                 (*MeterInfoType).MeterStatus,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 193 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else if (*MeterInfoType).TMeter_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error =
//                                 exi_basetypes_encoder_integer_64(stream, (*MeterInfoType).TMeter);
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 3 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             193 => {
//                 if (*MeterInfoType).TMeter_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error =
//                                 exi_basetypes_encoder_integer_64(stream, (*MeterInfoType).TMeter);
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 3 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
fn encode_iso2_message_header(
    stream: &mut ExiBitstream,
    message: &Iso2MessageHeaderType,
) -> Result<(), ExiError> {
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

    let Ok(len) = u16::try_from((message).session_id.len()) else {
        return Err(ExiError::InvalidValue);
    };

    exi_basetypes_encoder_uint_16(stream, len)?;
    exi_basetypes_encoder_bytes(stream, &(message).session_id)?;
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

    if let Some(ref notification) = message.notification {
        exi_basetypes_encoder_nbit_uint(stream, 2, 0)?;
        encode_iso2_notification(stream, notification)?;
        if let Some(ref signature) = (message).signature {
            exi_basetypes_encoder_nbit_uint(stream, 2, 0)?;
            //TODO Fix this func when required
            //encode_iso2_signature(stream, signature)?;
            exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
        } else {
            exi_basetypes_encoder_nbit_uint(stream, 2, 1)?;
        }
        return Ok(());
    }

    exi_basetypes_encoder_nbit_uint(stream, 2, 2)
}

// fn encode_iso2_PowerDeliveryReqType(
//     stream: &mut ExiBitstream,
//     mut PowerDeliveryReqType: *const iso2_PowerDeliveryReqType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 197 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             197 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             2 as i32 as usize,
//                             (*PowerDeliveryReqType).ChargeProgress as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 198 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             198 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             8 as i32 as usize,
//                             ((*PowerDeliveryReqType).SAScheduleTupleID as u32)
//                                 .wrapping_sub(1 as i32 as u32),
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 199 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             199 => {
//                 if (*PowerDeliveryReqType).ChargingProfile_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_ChargingProfileType(
//                             stream,
//                             &(*PowerDeliveryReqType).ChargingProfile,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 200 as i32;
//                         }
//                     }
//                 } else if (*PowerDeliveryReqType).DC_EVPowerDeliveryParameter_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_DC_EVPowerDeliveryParameterType(
//                             stream,
//                             &(*PowerDeliveryReqType).DC_EVPowerDeliveryParameter,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else if (*PowerDeliveryReqType).EVPowerDeliveryParameter_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_EVPowerDeliveryParameterType(
//                             stream,
//                             &(*PowerDeliveryReqType).EVPowerDeliveryParameter,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             200 => {
//                 if (*PowerDeliveryReqType).DC_EVPowerDeliveryParameter_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_DC_EVPowerDeliveryParameterType(
//                             stream,
//                             &(*PowerDeliveryReqType).DC_EVPowerDeliveryParameter,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else if (*PowerDeliveryReqType).EVPowerDeliveryParameter_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_EVPowerDeliveryParameterType(
//                             stream,
//                             &(*PowerDeliveryReqType).EVPowerDeliveryParameter,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_CurrentDemandResType(
//     stream: &mut ExiBitstream,
//     mut CurrentDemandResType: *const iso2_CurrentDemandResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 201 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             201 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             5 as i32 as usize,
//                             (*CurrentDemandResType).ResponseCode as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 202 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             202 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_DC_EVSEStatusType(
//                         stream,
//                         &(*CurrentDemandResType).DC_EVSEStatus,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 203 as i32;
//                     }
//                 }
//             }
//             203 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_PhysicalValueType(
//                         stream,
//                         &(*CurrentDemandResType).EVSEPresentVoltage,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 204 as i32;
//                     }
//                 }
//             }
//             204 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_PhysicalValueType(
//                         stream,
//                         &(*CurrentDemandResType).EVSEPresentCurrent,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 205 as i32;
//                     }
//                 }
//             }
//             205 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_bool(
//                             stream,
//                             (*CurrentDemandResType).EVSECurrentLimitAchieved,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 206 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             206 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_bool(
//                             stream,
//                             (*CurrentDemandResType).EVSEVoltageLimitAchieved,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 207 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             207 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_bool(
//                             stream,
//                             (*CurrentDemandResType).EVSEPowerLimitAchieved,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 208 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             208 => {
//                 if (*CurrentDemandResType).EVSEMaximumVoltageLimit_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_PhysicalValueType(
//                             stream,
//                             &(*CurrentDemandResType).EVSEMaximumVoltageLimit,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 209 as i32;
//                         }
//                     }
//                 } else if (*CurrentDemandResType).EVSEMaximumCurrentLimit_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_PhysicalValueType(
//                             stream,
//                             &(*CurrentDemandResType).EVSEMaximumCurrentLimit,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 210 as i32;
//                         }
//                     }
//                 } else if (*CurrentDemandResType).EVSEMaximumPowerLimit_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_PhysicalValueType(
//                             stream,
//                             &(*CurrentDemandResType).EVSEMaximumPowerLimit,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 211 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 ((*CurrentDemandResType).EVSEID.charactersLen as i32 + 2 as i32)
//                                     as u16,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_characters(
//                                     stream,
//                                     (*CurrentDemandResType).EVSEID.charactersLen as usize,
//                                     ((*CurrentDemandResType).EVSEID.characters).as_ptr(),
//                                     (37 as i32 + 1 as i32) as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 212 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             209 => {
//                 if (*CurrentDemandResType).EVSEMaximumCurrentLimit_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_PhysicalValueType(
//                             stream,
//                             &(*CurrentDemandResType).EVSEMaximumCurrentLimit,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 210 as i32;
//                         }
//                     }
//                 } else if (*CurrentDemandResType).EVSEMaximumPowerLimit_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_PhysicalValueType(
//                             stream,
//                             &(*CurrentDemandResType).EVSEMaximumPowerLimit,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 211 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 ((*CurrentDemandResType).EVSEID.charactersLen as i32 + 2 as i32)
//                                     as u16,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_characters(
//                                     stream,
//                                     (*CurrentDemandResType).EVSEID.charactersLen as usize,
//                                     ((*CurrentDemandResType).EVSEID.characters).as_ptr(),
//                                     (37 as i32 + 1 as i32) as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 212 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             210 => {
//                 if (*CurrentDemandResType).EVSEMaximumPowerLimit_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_PhysicalValueType(
//                             stream,
//                             &(*CurrentDemandResType).EVSEMaximumPowerLimit,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 211 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 ((*CurrentDemandResType).EVSEID.charactersLen as i32 + 2 as i32)
//                                     as u16,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_characters(
//                                     stream,
//                                     (*CurrentDemandResType).EVSEID.charactersLen as usize,
//                                     ((*CurrentDemandResType).EVSEID.characters).as_ptr(),
//                                     (37 as i32 + 1 as i32) as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 212 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             211 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*CurrentDemandResType).EVSEID.charactersLen as i32 + 2 as i32) as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*CurrentDemandResType).EVSEID.charactersLen as usize,
//                                 ((*CurrentDemandResType).EVSEID.characters).as_ptr(),
//                                 (37 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 212 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             212 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             8 as i32 as usize,
//                             ((*CurrentDemandResType).SAScheduleTupleID as u32)
//                                 .wrapping_sub(1 as i32 as u32),
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 213 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             213 => {
//                 if (*CurrentDemandResType).MeterInfo_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error =
//                             encode_iso2_MeterInfoType(stream, &(*CurrentDemandResType).MeterInfo);
//                         if error == 0 as i32 {
//                             grammar_id = 214 as i32;
//                         }
//                     }
//                 } else if (*CurrentDemandResType).ReceiptRequired_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bool(
//                                 stream,
//                                 (*CurrentDemandResType).ReceiptRequired,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 3 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             214 => {
//                 if (*CurrentDemandResType).ReceiptRequired_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bool(
//                                 stream,
//                                 (*CurrentDemandResType).ReceiptRequired,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 3 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_ChargingStatusResType(
//     stream: &mut ExiBitstream,
//     mut ChargingStatusResType: *const iso2_ChargingStatusResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 215 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             215 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             5 as i32 as usize,
//                             (*ChargingStatusResType).ResponseCode as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 216 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             216 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*ChargingStatusResType).EVSEID.charactersLen as i32 + 2 as i32)
//                                 as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*ChargingStatusResType).EVSEID.charactersLen as usize,
//                                 ((*ChargingStatusResType).EVSEID.characters).as_ptr(),
//                                 (37 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 217 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             217 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             8 as i32 as usize,
//                             ((*ChargingStatusResType).SAScheduleTupleID as u32)
//                                 .wrapping_sub(1 as i32 as u32),
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 218 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             218 => {
//                 if (*ChargingStatusResType).EVSEMaxCurrent_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_PhysicalValueType(
//                             stream,
//                             &(*ChargingStatusResType).EVSEMaxCurrent,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 219 as i32;
//                         }
//                     }
//                 } else if (*ChargingStatusResType).MeterInfo_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error =
//                             encode_iso2_MeterInfoType(stream, &(*ChargingStatusResType).MeterInfo);
//                         if error == 0 as i32 {
//                             grammar_id = 220 as i32;
//                         }
//                     }
//                 } else if (*ChargingStatusResType).ReceiptRequired_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bool(
//                                 stream,
//                                 (*ChargingStatusResType).ReceiptRequired,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 221 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_AC_EVSEStatusType(
//                             stream,
//                             &(*ChargingStatusResType).AC_EVSEStatus,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 }
//             }
//             219 => {
//                 if (*ChargingStatusResType).MeterInfo_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error =
//                             encode_iso2_MeterInfoType(stream, &(*ChargingStatusResType).MeterInfo);
//                         if error == 0 as i32 {
//                             grammar_id = 220 as i32;
//                         }
//                     }
//                 } else if (*ChargingStatusResType).ReceiptRequired_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bool(
//                                 stream,
//                                 (*ChargingStatusResType).ReceiptRequired,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 221 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_AC_EVSEStatusType(
//                             stream,
//                             &(*ChargingStatusResType).AC_EVSEStatus,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 }
//             }
//             220 => {
//                 if (*ChargingStatusResType).ReceiptRequired_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bool(
//                                 stream,
//                                 (*ChargingStatusResType).ReceiptRequired,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 221 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_AC_EVSEStatusType(
//                             stream,
//                             &(*ChargingStatusResType).AC_EVSEStatus,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 }
//             }
//             221 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_AC_EVSEStatusType(
//                         stream,
//                         &(*ChargingStatusResType).AC_EVSEStatus,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 3 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_AuthorizationReqType(
//     stream: &mut ExiBitstream,
//     mut AuthorizationReqType: *const iso2_AuthorizationReqType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 222 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             222 => {
//                 if (*AuthorizationReqType).Id_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*AuthorizationReqType).Id.charactersLen as i32 + 2 as i32) as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*AuthorizationReqType).Id.charactersLen as usize,
//                                 ((*AuthorizationReqType).Id.characters).as_ptr(),
//                                 (64 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 223 as i32;
//                             }
//                         }
//                     }
//                 } else if (*AuthorizationReqType).GenChallenge_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*AuthorizationReqType).GenChallenge.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*AuthorizationReqType).GenChallenge.bytesLen as usize,
//                                     ((*AuthorizationReqType).GenChallenge.bytes).as_ptr(),
//                                     16 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 3 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             223 => {
//                 if (*AuthorizationReqType).GenChallenge_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*AuthorizationReqType).GenChallenge.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*AuthorizationReqType).GenChallenge.bytesLen as usize,
//                                     ((*AuthorizationReqType).GenChallenge.bytes).as_ptr(),
//                                     16 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 3 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_PreChargeReqType(
//     stream: &mut ExiBitstream,
//     mut PreChargeReqType: *const iso2_PreChargeReqType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 224 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             224 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_message(stream, &(*PreChargeReqType).DC_EVStatus);
//                     if error == 0 as i32 {
//                         grammar_id = 225 as i32;
//                     }
//                 }
//             }
//             225 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         encode_iso2_PhysicalValueType(stream, &(*PreChargeReqType).EVTargetVoltage);
//                     if error == 0 as i32 {
//                         grammar_id = 226 as i32;
//                     }
//                 }
//             }
//             226 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         encode_iso2_PhysicalValueType(stream, &(*PreChargeReqType).EVTargetCurrent);
//                     if error == 0 as i32 {
//                         grammar_id = 3 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_ServiceDetailResType(
//     stream: &mut ExiBitstream,
//     mut ServiceDetailResType: *const iso2_ServiceDetailResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 227 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             227 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             5 as i32 as usize,
//                             (*ServiceDetailResType).ResponseCode as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 228 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             228 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             (*ServiceDetailResType).ServiceID,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 229 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             229 => {
//                 if (*ServiceDetailResType).ServiceParameterList_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_ServiceParameterListType(
//                             stream,
//                             &(*ServiceDetailResType).ServiceParameterList,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_PaymentServiceSelectionResType(
//     stream: &mut ExiBitstream,
//     mut PaymentServiceSelectionResType: *const iso2_PaymentServiceSelectionResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 230 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             230 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             5 as i32 as usize,
//                             (*PaymentServiceSelectionResType).ResponseCode as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_CertificateUpdateReqType(
//     stream: &mut ExiBitstream,
//     mut CertificateUpdateReqType: *const iso2_CertificateUpdateReqType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 231 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             231 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = exi_basetypes_encoder_uint_16(
//                         stream,
//                         ((*CertificateUpdateReqType).Id.charactersLen as i32 + 2 as i32) as u16,
//                     );
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_characters(
//                             stream,
//                             (*CertificateUpdateReqType).Id.charactersLen as usize,
//                             ((*CertificateUpdateReqType).Id.characters).as_ptr(),
//                             (64 as i32 + 1 as i32) as usize,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 232 as i32;
//                         }
//                     }
//                 }
//             }
//             232 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_CertificateChainType(
//                         stream,
//                         &(*CertificateUpdateReqType).ContractSignatureCertChain,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 233 as i32;
//                     }
//                 }
//             }
//             233 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*CertificateUpdateReqType).eMAID.charactersLen as i32 + 2 as i32)
//                                 as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*CertificateUpdateReqType).eMAID.charactersLen as usize,
//                                 ((*CertificateUpdateReqType).eMAID.characters).as_ptr(),
//                                 (15 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 234 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             234 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_ListOfRootCertificateIDsType(
//                         stream,
//                         &(*CertificateUpdateReqType).ListOfRootCertificateIDs,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 3 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
fn encode_iso2_session_setup_res(
    stream: &mut ExiBitstream,
    message: &Iso2SessionSetupResType,
) -> Result<(), ExiError> {
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    exi_basetypes_encoder_nbit_uint(stream, 5, message.response_code as u32)?;
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    let Ok(len) = u16::try_from(message.evse_id.len()) else {
        return Err(ExiError::InvalidCharactersLength);
    };
    exi_basetypes_encoder_uint_16(stream, len + 2)?;
    exi_basetypes_encoder_characters(stream, &message.evse_id)?;
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

    if let Some(timestamp) = message.evse_time_stamp {
        exi_basetypes_encoder_nbit_uint(stream, 2, 0)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
        exi_basetypes_encoder_integer_64(stream, timestamp)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
        return Ok(());
    }

    exi_basetypes_encoder_nbit_uint(stream, 2, 1)
}

// fn encode_iso2_CertificateInstallationReqType(
//     stream: &mut ExiBitstream,
//     mut CertificateInstallationReqType: *const iso2_CertificateInstallationReqType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 238 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             238 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = exi_basetypes_encoder_uint_16(
//                         stream,
//                         ((*CertificateInstallationReqType).Id.charactersLen as i32 + 2 as i32)
//                             as u16,
//                     );
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_characters(
//                             stream,
//                             (*CertificateInstallationReqType).Id.charactersLen as usize,
//                             ((*CertificateInstallationReqType).Id.characters).as_ptr(),
//                             (64 as i32 + 1 as i32) as usize,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 239 as i32;
//                         }
//                     }
//                 }
//             }
//             239 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             (*CertificateInstallationReqType)
//                                 .OEMProvisioningCert
//                                 .bytesLen,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bytes(
//                                 stream,
//                                 (*CertificateInstallationReqType)
//                                     .OEMProvisioningCert
//                                     .bytesLen as usize,
//                                 ((*CertificateInstallationReqType).OEMProvisioningCert.bytes)
//                                     .as_ptr(),
//                                 800 as i32 as usize,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 240 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             240 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_ListOfRootCertificateIDsType(
//                         stream,
//                         &(*CertificateInstallationReqType).ListOfRootCertificateIDs,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 3 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }

// fn encode_iso2_CertificateInstallationResType(
//     stream: &mut ExiBitstream,
//     mut CertificateInstallationResType: *const iso2_CertificateInstallationResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 241 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             241 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             5 as i32 as usize,
//                             (*CertificateInstallationResType).ResponseCode as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 242 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             242 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_CertificateChainType(
//                         stream,
//                         &(*CertificateInstallationResType).SAProvisioningCertificateChain,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 243 as i32;
//                     }
//                 }
//             }
//             243 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_CertificateChainType(
//                         stream,
//                         &(*CertificateInstallationResType).ContractSignatureCertChain,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 244 as i32;
//                     }
//                 }
//             }
//             244 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_ContractSignatureEncryptedPrivateKeyType(
//                         stream,
//                         &(*CertificateInstallationResType).ContractSignatureEncryptedPrivateKey,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 245 as i32;
//                     }
//                 }
//             }
//             245 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_DiffieHellmanPublickeyType(
//                         stream,
//                         &(*CertificateInstallationResType).DHpublickey,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 246 as i32;
//                     }
//                 }
//             }
//             246 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_EMAIDType(stream, &(*CertificateInstallationResType).eMAID);
//                     if error == 0 as i32 {
//                         grammar_id = 3 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_WeldingDetectionResType(
//     stream: &mut ExiBitstream,
//     mut WeldingDetectionResType: *const iso2_WeldingDetectionResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 247 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             247 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             5 as i32 as usize,
//                             (*WeldingDetectionResType).ResponseCode as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 248 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             248 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_DC_EVSEStatusType(
//                         stream,
//                         &(*WeldingDetectionResType).DC_EVSEStatus,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 249 as i32;
//                     }
//                 }
//             }
//             249 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_PhysicalValueType(
//                         stream,
//                         &(*WeldingDetectionResType).EVSEPresentVoltage,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 3 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_CurrentDemandReqType(
//     stream: &mut ExiBitstream,
//     mut CurrentDemandReqType: *const iso2_CurrentDemandReqType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 250 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             250 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         encode_iso2_message(stream, &(*CurrentDemandReqType).DC_EVStatus);
//                     if error == 0 as i32 {
//                         grammar_id = 251 as i32;
//                     }
//                 }
//             }
//             251 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_PhysicalValueType(
//                         stream,
//                         &(*CurrentDemandReqType).EVTargetCurrent,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 252 as i32;
//                     }
//                 }
//             }
//             252 => {
//                 if (*CurrentDemandReqType).EVMaximumVoltageLimit_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_PhysicalValueType(
//                             stream,
//                             &(*CurrentDemandReqType).EVMaximumVoltageLimit,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 253 as i32;
//                         }
//                     }
//                 } else if (*CurrentDemandReqType).EVMaximumCurrentLimit_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_PhysicalValueType(
//                             stream,
//                             &(*CurrentDemandReqType).EVMaximumCurrentLimit,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 254 as i32;
//                         }
//                     }
//                 } else if (*CurrentDemandReqType).EVMaximumPowerLimit_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_PhysicalValueType(
//                             stream,
//                             &(*CurrentDemandReqType).EVMaximumPowerLimit,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 255 as i32;
//                         }
//                     }
//                 } else if (*CurrentDemandReqType).BulkChargingComplete_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bool(
//                                 stream,
//                                 (*CurrentDemandReqType).BulkChargingComplete,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 256 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bool(
//                                 stream,
//                                 (*CurrentDemandReqType).ChargingComplete,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 257 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             253 => {
//                 if (*CurrentDemandReqType).EVMaximumCurrentLimit_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_PhysicalValueType(
//                             stream,
//                             &(*CurrentDemandReqType).EVMaximumCurrentLimit,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 254 as i32;
//                         }
//                     }
//                 } else if (*CurrentDemandReqType).EVMaximumPowerLimit_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_PhysicalValueType(
//                             stream,
//                             &(*CurrentDemandReqType).EVMaximumPowerLimit,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 255 as i32;
//                         }
//                     }
//                 } else if (*CurrentDemandReqType).BulkChargingComplete_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bool(
//                                 stream,
//                                 (*CurrentDemandReqType).BulkChargingComplete,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 256 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bool(
//                                 stream,
//                                 (*CurrentDemandReqType).ChargingComplete,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 257 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             254 => {
//                 if (*CurrentDemandReqType).EVMaximumPowerLimit_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_PhysicalValueType(
//                             stream,
//                             &(*CurrentDemandReqType).EVMaximumPowerLimit,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 255 as i32;
//                         }
//                     }
//                 } else if (*CurrentDemandReqType).BulkChargingComplete_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bool(
//                                 stream,
//                                 (*CurrentDemandReqType).BulkChargingComplete,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 256 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bool(
//                                 stream,
//                                 (*CurrentDemandReqType).ChargingComplete,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 257 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             255 => {
//                 if (*CurrentDemandReqType).BulkChargingComplete_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bool(
//                                 stream,
//                                 (*CurrentDemandReqType).BulkChargingComplete,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 256 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bool(
//                                 stream,
//                                 (*CurrentDemandReqType).ChargingComplete,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 257 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             256 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_bool(
//                             stream,
//                             (*CurrentDemandReqType).ChargingComplete,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 257 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             257 => {
//                 if (*CurrentDemandReqType).RemainingTimeToFullSoC_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_PhysicalValueType(
//                             stream,
//                             &(*CurrentDemandReqType).RemainingTimeToFullSoC,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 258 as i32;
//                         }
//                     }
//                 } else if (*CurrentDemandReqType).RemainingTimeToBulkSoC_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_PhysicalValueType(
//                             stream,
//                             &(*CurrentDemandReqType).RemainingTimeToBulkSoC,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 259 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_PhysicalValueType(
//                             stream,
//                             &(*CurrentDemandReqType).EVTargetVoltage,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 }
//             }
//             258 => {
//                 if (*CurrentDemandReqType).RemainingTimeToBulkSoC_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_PhysicalValueType(
//                             stream,
//                             &(*CurrentDemandReqType).RemainingTimeToBulkSoC,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 259 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_PhysicalValueType(
//                             stream,
//                             &(*CurrentDemandReqType).EVTargetVoltage,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 }
//             }
//             259 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_PhysicalValueType(
//                         stream,
//                         &(*CurrentDemandReqType).EVTargetVoltage,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 3 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_PreChargeResType(
//     stream: &mut ExiBitstream,
//     mut PreChargeResType: *const iso2_PreChargeResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 260 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             260 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             5 as i32 as usize,
//                             (*PreChargeResType).ResponseCode as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 261 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             261 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         encode_iso2_DC_EVSEStatusType(stream, &(*PreChargeResType).DC_EVSEStatus);
//                     if error == 0 as i32 {
//                         grammar_id = 262 as i32;
//                     }
//                 }
//             }
//             262 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_PhysicalValueType(
//                         stream,
//                         &(*PreChargeResType).EVSEPresentVoltage,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 3 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_CertificateUpdateResType(
//     stream: &mut ExiBitstream,
//     mut CertificateUpdateResType: *const iso2_CertificateUpdateResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 263 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             263 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             5 as i32 as usize,
//                             (*CertificateUpdateResType).ResponseCode as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 264 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             264 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_CertificateChainType(
//                         stream,
//                         &(*CertificateUpdateResType).SAProvisioningCertificateChain,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 265 as i32;
//                     }
//                 }
//             }
//             265 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_CertificateChainType(
//                         stream,
//                         &(*CertificateUpdateResType).ContractSignatureCertChain,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 266 as i32;
//                     }
//                 }
//             }
//             266 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_ContractSignatureEncryptedPrivateKeyType(
//                         stream,
//                         &(*CertificateUpdateResType).ContractSignatureEncryptedPrivateKey,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 267 as i32;
//                     }
//                 }
//             }
//             267 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_DiffieHellmanPublickeyType(
//                         stream,
//                         &(*CertificateUpdateResType).DHpublickey,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 268 as i32;
//                     }
//                 }
//             }
//             268 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_EMAIDType(stream, &(*CertificateUpdateResType).eMAID);
//                     if error == 0 as i32 {
//                         grammar_id = 269 as i32;
//                     }
//                 }
//             }
//             269 => {
//                 if (*CertificateUpdateResType).RetryCounter_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_integer_16(
//                                 stream,
//                                 (*CertificateUpdateResType).RetryCounter,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 3 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         done = 1 as i32;
//                         grammar_id = 4 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_MeteringReceiptReqType(
//     stream: &mut ExiBitstream,
//     mut MeteringReceiptReqType: *const iso2_MeteringReceiptReqType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 270 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             270 => {
//                 if (*MeteringReceiptReqType).Id_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*MeteringReceiptReqType).Id.charactersLen as i32 + 2 as i32) as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*MeteringReceiptReqType).Id.charactersLen as usize,
//                                 ((*MeteringReceiptReqType).Id.characters).as_ptr(),
//                                 (64 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 271 as i32;
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_uint_16(
//                                 stream,
//                                 (*MeteringReceiptReqType).SessionID.bytesLen,
//                             );
//                             if error == 0 as i32 {
//                                 error = exi_basetypes_encoder_bytes(
//                                     stream,
//                                     (*MeteringReceiptReqType).SessionID.bytesLen as usize,
//                                     ((*MeteringReceiptReqType).SessionID.bytes).as_ptr(),
//                                     8 as i32 as usize,
//                                 );
//                                 if error == 0 as i32 {
//                                     exi_basetypes_encoder_nbit_uint(
//                                         stream,
//                                         1 as i32 as usize,
//                                         0 as i32 as u32,
//                                     );
//                                     if error == 0 as i32 {
//                                         grammar_id = 272 as i32;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             271 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             (*MeteringReceiptReqType).SessionID.bytesLen,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bytes(
//                                 stream,
//                                 (*MeteringReceiptReqType).SessionID.bytesLen as usize,
//                                 ((*MeteringReceiptReqType).SessionID.bytes).as_ptr(),
//                                 8 as i32 as usize,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 272 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             272 => {
//                 if (*MeteringReceiptReqType).SAScheduleTupleID_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             0 as i32 as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 8 as i32 as usize,
//                                 ((*MeteringReceiptReqType).SAScheduleTupleID as u32)
//                                     .wrapping_sub(1 as i32 as u32),
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 273 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error =
//                             encode_iso2_MeterInfoType(stream, &(*MeteringReceiptReqType).MeterInfo);
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 }
//             }
//             273 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_MeterInfoType(stream, &(*MeteringReceiptReqType).MeterInfo);
//                     if error == 0 as i32 {
//                         grammar_id = 3 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_ChargingStatusReqType(
//     stream: &mut ExiBitstream,
//     mut ChargingStatusReqType: *const iso2_ChargingStatusReqType,
// ) -> Result<(), ExiError> {
//     let mut error: i32 =
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//     return error;
// }
// fn encode_iso2_SessionStopResType(
//     stream: &mut ExiBitstream,
//     mut SessionStopResType: *const iso2_SessionStopResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 274 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             274 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             5 as i32 as usize,
//                             (*SessionStopResType).ResponseCode as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_charge_parameter_discovery_req(
//     stream: &mut ExiBitstream,
//     message: &Iso2ChargeParameterDiscoveryReqType,
// ) -> Result<(), ExiError> {
//     if let Some(max_entries) = message.max_entries_sa_schedule_tuple {
//         exi_basetypes_encoder_nbit_uint(stream, 2, 0)?;
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//         exi_basetypes_encoder_uint_16(stream, max_entries)?;
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//         exi_basetypes_encoder_nbit_uint(stream, 3, message.requested_energy_transfer_mode as u32)?;
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     } else {
//         exi_basetypes_encoder_nbit_uint(stream, 2, 1)?;
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//         exi_basetypes_encoder_nbit_uint(stream, 3, message.requested_energy_transfer_mode as u32)?;
//         exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     }

//     if let Some(ref ac_param) = message.ac_ev_charge_parameter {
//         exi_basetypes_encoder_nbit_uint(stream, 2, 0)?;
//         encode_iso2_ac_evcharge_parameter(stream, ac_param)?;
//     } else if let Some(ref dc_param) = message.dc_ev_charge_parameter {
//         exi_basetypes_encoder_nbit_uint(stream, 2, 1)?;
//         encode_iso2_dc_evcharge_parameter(stream, dc_param)?;
//     } else if let Some(ref ev_param) = message.ev_charge_parameter {
//         exi_basetypes_encoder_nbit_uint(stream, 2, 2)?;
//         encode_iso2_evcharge_parameter(stream, ev_param)?;
//     }

//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)
// }
// fn encode_iso2_CableCheckReqType(
//     stream: &mut ExiBitstream,
//     mut CableCheckReqType: *const iso2_CableCheckReqType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 278 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             278 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_message(stream, &(*CableCheckReqType).DC_EVStatus);
//                     if error == 0 as i32 {
//                         grammar_id = 3 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_WeldingDetectionReqType(
//     stream: &mut ExiBitstream,
//     mut WeldingDetectionReqType: *const iso2_WeldingDetectionReqType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 279 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             279 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_message(
//                         stream,
//                         &(*WeldingDetectionReqType).DC_EVStatus,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 3 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_PowerDeliveryResType(
//     stream: &mut ExiBitstream,
//     mut PowerDeliveryResType: *const iso2_PowerDeliveryResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 280 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             280 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             5 as i32 as usize,
//                             (*PowerDeliveryResType).ResponseCode as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 281 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             281 => {
//                 if (*PowerDeliveryResType).AC_EVSEStatus_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_AC_EVSEStatusType(
//                             stream,
//                             &(*PowerDeliveryResType).AC_EVSEStatus,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else if (*PowerDeliveryResType).DC_EVSEStatus_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_DC_EVSEStatusType(
//                             stream,
//                             &(*PowerDeliveryResType).DC_EVSEStatus,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         error =
//                             encode_iso2_EVSEStatusType(stream, &(*PowerDeliveryResType).EVSEStatus);
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_ChargeParameterDiscoveryResType(
//     stream: &mut ExiBitstream,
//     mut ChargeParameterDiscoveryResType: *const iso2_ChargeParameterDiscoveryResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 282 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             282 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             5 as i32 as usize,
//                             (*ChargeParameterDiscoveryResType).ResponseCode as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 283 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             283 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             2 as i32 as usize,
//                             (*ChargeParameterDiscoveryResType).EVSEProcessing as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 284 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             284 => {
//                 if (*ChargeParameterDiscoveryResType).SAScheduleList_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_SAScheduleListType(
//                             stream,
//                             &(*ChargeParameterDiscoveryResType).SAScheduleList,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 285 as i32;
//                         }
//                     }
//                 } else if (*ChargeParameterDiscoveryResType).SASchedules_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_SASchedulesType(
//                             stream,
//                             &(*ChargeParameterDiscoveryResType).SASchedules,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 285 as i32;
//                         }
//                     }
//                 } else if (*ChargeParameterDiscoveryResType).AC_EVSEChargeParameter_isUsed()
//                     == 1 as u32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_AC_EVSEChargeParameterType(
//                             stream,
//                             &(*ChargeParameterDiscoveryResType).AC_EVSEChargeParameter,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else if (*ChargeParameterDiscoveryResType).DC_EVSEChargeParameter_isUsed()
//                     == 1 as u32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_DC_EVSEChargeParameterType(
//                             stream,
//                             &(*ChargeParameterDiscoveryResType).DC_EVSEChargeParameter,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_EVSEChargeParameterType(
//                             stream,
//                             &(*ChargeParameterDiscoveryResType).EVSEChargeParameter,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 }
//             }
//             285 => {
//                 if (*ChargeParameterDiscoveryResType).AC_EVSEChargeParameter_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_AC_EVSEChargeParameterType(
//                             stream,
//                             &(*ChargeParameterDiscoveryResType).AC_EVSEChargeParameter,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else if (*ChargeParameterDiscoveryResType).DC_EVSEChargeParameter_isUsed()
//                     == 1 as u32
//                 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_DC_EVSEChargeParameterType(
//                             stream,
//                             &(*ChargeParameterDiscoveryResType).DC_EVSEChargeParameter,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_EVSEChargeParameterType(
//                             stream,
//                             &(*ChargeParameterDiscoveryResType).EVSEChargeParameter,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_PaymentServiceSelectionReqType(
//     stream: &mut ExiBitstream,
//     mut PaymentServiceSelectionReqType: *const iso2_PaymentServiceSelectionReqType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 286 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             286 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             (*PaymentServiceSelectionReqType).SelectedPaymentOption as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 287 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             287 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_SelectedServiceListType(
//                         stream,
//                         &(*PaymentServiceSelectionReqType).SelectedServiceList,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 3 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_MeteringReceiptResType(
//     stream: &mut ExiBitstream,
//     mut MeteringReceiptResType: *const iso2_MeteringReceiptResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 288 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             288 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             5 as i32 as usize,
//                             (*MeteringReceiptResType).ResponseCode as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 289 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             289 => {
//                 if (*MeteringReceiptResType).AC_EVSEStatus_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_AC_EVSEStatusType(
//                             stream,
//                             &(*MeteringReceiptResType).AC_EVSEStatus,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else if (*MeteringReceiptResType).DC_EVSEStatus_isUsed() == 1 as u32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_DC_EVSEStatusType(
//                             stream,
//                             &(*MeteringReceiptResType).DC_EVSEStatus,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 } else {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
//                     if error == 0 as i32 {
//                         error = encode_iso2_EVSEStatusType(
//                             stream,
//                             &(*MeteringReceiptResType).EVSEStatus,
//                         );
//                         if error == 0 as i32 {
//                             grammar_id = 3 as i32;
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_CableCheckResType(
//     stream: &mut ExiBitstream,
//     mut CableCheckResType: *const iso2_CableCheckResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 290 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             290 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             5 as i32 as usize,
//                             (*CableCheckResType).ResponseCode as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 291 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             291 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         encode_iso2_DC_EVSEStatusType(stream, &(*CableCheckResType).DC_EVSEStatus);
//                     if error == 0 as i32 {
//                         grammar_id = 292 as i32;
//                     }
//                 }
//             }
//             292 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             2 as i32 as usize,
//                             (*CableCheckResType).EVSEProcessing as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
fn encode_iso2_service_discovery_res(
    stream: &mut ExiBitstream,
    message: &Iso2ServiceDiscoveryResType,
) -> Result<(), ExiError> {
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    exi_basetypes_encoder_nbit_uint(stream, 5, message.response_code as u32)?;
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    encode_iso2_payment_option_list(stream, &message.payment_option_list)?;

    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    encode_iso2_charge_service(stream, &message.charge_service)?;

    if let Some(service_list) = &message.service_list {
        exi_basetypes_encoder_nbit_uint(stream, 2, 0)?;
        encode_iso2_service_list(stream, &service_list)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    } else {
        exi_basetypes_encoder_nbit_uint(stream, 2, 1)?;
    }

    Ok(())
}

// fn encode_iso2_service_detail_req(
//     stream: &mut ExiBitstream,
//     message: &Iso2ServiceDetailReqType,
// ) -> Result<(), ExiError> {
//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     exi_basetypes_encoder_uint_16(stream, message.service_id)?;
//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
//     exi_basetypes_encoder_nbit_uint(stream, 1, 0)
// }

fn encode_iso2_session_setup_req(
    stream: &mut ExiBitstream,
    message: &Iso2SessionSetupReqType,
) -> Result<(), ExiError> {
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;

    let Ok(len) = u16::try_from(message.evcc_id.len()) else {
        return Err(ExiError::InvalidCharactersLength);
    };

    exi_basetypes_encoder_uint_16(stream, len)?;
    exi_basetypes_encoder_bytes(stream, &message.evcc_id)?;
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)
}
// fn encode_iso2_SessionStopReqType(
//     stream: &mut ExiBitstream,
//     mut SessionStopReqType: *const iso2_SessionStopReqType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 299 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             299 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             1 as i32 as usize,
//                             (*SessionStopReqType).ChargingSession as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
fn encode_iso2_service_discovery_req(
    stream: &mut ExiBitstream,
    message: &Iso2ServiceDiscoveryReqType,
) -> Result<(), ExiError> {
    if let Some(service_scope) = &message.service_scope {
        exi_basetypes_encoder_nbit_uint(stream, 2, 0)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
        let len: u16 =
            u16::try_from(service_scope.len()).map_err(|_| ExiError::InvalidCharactersLength)?;
        exi_basetypes_encoder_uint_16(stream, len + 2)?;
        exi_basetypes_encoder_characters(stream, &service_scope)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    } else if let Some(service_category) = &message.service_category {
        exi_basetypes_encoder_nbit_uint(stream, 2, 1)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
        exi_basetypes_encoder_nbit_uint(stream, 2, *service_category as u32)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    } else {
        exi_basetypes_encoder_nbit_uint(stream, 2, 2)?;
        return Ok(());
    }

    if let Some(service_category) = &message.service_category {
        exi_basetypes_encoder_nbit_uint(stream, 2, 0)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
        exi_basetypes_encoder_nbit_uint(stream, 2, *service_category as u32)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
        exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    } else {
        exi_basetypes_encoder_nbit_uint(stream, 2, 1)?;
    }

    Ok(())
}

// fn encode_iso2_AuthorizationResType(
//     stream: &mut ExiBitstream,
//     mut AuthorizationResType: *const iso2_AuthorizationResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 302 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             302 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             5 as i32 as usize,
//                             (*AuthorizationResType).ResponseCode as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 303 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             303 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             2 as i32 as usize,
//                             (*AuthorizationResType).EVSEProcessing as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_PaymentDetailsReqType(
//     stream: &mut ExiBitstream,
//     mut PaymentDetailsReqType: *const iso2_PaymentDetailsReqType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 304 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             304 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             ((*PaymentDetailsReqType).eMAID.charactersLen as i32 + 2 as i32) as u16,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_characters(
//                                 stream,
//                                 (*PaymentDetailsReqType).eMAID.charactersLen as usize,
//                                 ((*PaymentDetailsReqType).eMAID.characters).as_ptr(),
//                                 (15 as i32 + 1 as i32) as usize,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 305 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             305 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error = encode_iso2_CertificateChainType(
//                         stream,
//                         &(*PaymentDetailsReqType).ContractSignatureCertChain,
//                     );
//                     if error == 0 as i32 {
//                         grammar_id = 3 as i32;
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
// fn encode_iso2_PaymentDetailsResType(
//     stream: &mut ExiBitstream,
//     mut PaymentDetailsResType: *const iso2_PaymentDetailsResType,
// ) -> Result<(), ExiError> {
//     let mut grammar_id: i32 = 306 as i32;
//     let mut done: i32 = 0 as i32;
//     let mut error: i32 = 0 as i32;
//     while done == 0 {
//         match grammar_id {
//             306 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         exi_basetypes_encoder_nbit_uint(
//                             stream,
//                             5 as i32 as usize,
//                             (*PaymentDetailsResType).ResponseCode as u32,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 307 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             307 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_uint_16(
//                             stream,
//                             (*PaymentDetailsResType).GenChallenge.bytesLen,
//                         );
//                         if error == 0 as i32 {
//                             error = exi_basetypes_encoder_bytes(
//                                 stream,
//                                 (*PaymentDetailsResType).GenChallenge.bytesLen as usize,
//                                 ((*PaymentDetailsResType).GenChallenge.bytes).as_ptr(),
//                                 16 as i32 as usize,
//                             );
//                             if error == 0 as i32 {
//                                 exi_basetypes_encoder_nbit_uint(
//                                     stream,
//                                     1 as i32 as usize,
//                                     0 as i32 as u32,
//                                 );
//                                 if error == 0 as i32 {
//                                     grammar_id = 308 as i32;
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             308 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     error =
//                         exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                     if error == 0 as i32 {
//                         error = exi_basetypes_encoder_integer_64(
//                             stream,
//                             (*PaymentDetailsResType).EVSETimeStamp,
//                         );
//                         if error == 0 as i32 {
//                             exi_basetypes_encoder_nbit_uint(
//                                 stream,
//                                 1 as i32 as usize,
//                                 0 as i32 as u32,
//                             );
//                             if error == 0 as i32 {
//                                 grammar_id = 3 as i32;
//                             }
//                         }
//                     }
//                 }
//             }
//             3 => {
//                 exi_basetypes_encoder_nbit_uint(stream, 1, 0);
//                 if error == 0 as i32 {
//                     done = 1 as i32;
//                     grammar_id = 4 as i32;
//                 }
//             }
//             _ => {
//                 error = -(130 as i32);
//             }
//         }
//         if error != 0 {
//             done = 1 as i32;
//         }
//     }
//     return error;
// }
fn encode_iso2_body(stream: &mut ExiBitstream, message: &Iso2v2gMessage) -> Result<(), ExiError> {
    //TODO: Implement the rest of the messages, bit count is 6, value depends on message (can get from commented out code in decoder)
    match message.body {
        Iso2BodyTypeEnum::ServiceDiscoveryReq(ref body) => {
            exi_basetypes_encoder_nbit_uint(stream, 6, 27)?;
            encode_iso2_service_discovery_req(stream, body)?;
        }
        Iso2BodyTypeEnum::ServiceDiscoveryRes(ref body) => {
            exi_basetypes_encoder_nbit_uint(stream, 6, 28)?;
            encode_iso2_service_discovery_res(stream, body)?;
        }
        Iso2BodyTypeEnum::SessionSetupReq(ref body) => {
            exi_basetypes_encoder_nbit_uint(stream, 6, 29)?;
            encode_iso2_session_setup_req(stream, body)?;
        }
        Iso2BodyTypeEnum::SessionSetupRes(ref body) => {
            exi_basetypes_encoder_nbit_uint(stream, 6, 30)?;
            encode_iso2_session_setup_res(stream, body)?;
        }
        _ => {
            return Err(ExiError::UnknownBodyTypeForEncoding);
        }
    }

    exi_basetypes_encoder_nbit_uint(stream, 1, 0)
}

fn encode_iso2_v2g_message(
    stream: &mut ExiBitstream,
    message: &Iso2v2gMessage,
) -> Result<(), ExiError> {
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    encode_iso2_message_header(stream, &message.header)?;
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    encode_iso2_body(stream, message)?;
    exi_basetypes_encoder_nbit_uint(stream, 1, 0)?;
    Ok(())
}

pub fn encode_iso2_exi(
    stream: &mut ExiBitstream,
    message: &Iso2v2gMessage,
) -> Result<(), ExiError> {
    stream.write_header()?;
    exi_basetypes_encoder_nbit_uint(stream, 7, 76)?;
    encode_iso2_v2g_message(stream, message)
}

// pub fn encode_iso2_exiFragment(
//     stream: &mut ExiBitstream,
//     mut exiFrag: *mut iso2_exiFragment,
// ) -> Result<(), ExiError> {
//     let mut error: i32 = exi_header_write(stream);
//     if error == 0 as i32 {
//         if 0 as i32 == 1 as i32 {
//             error = -(299 as i32);
//         } else if (*exiFrag).AuthorizationReq_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 8 as i32 as usize, 4 as i32 as u32);
//             if error == 0 as i32 {
//                 error = encode_iso2_AuthorizationReqType(
//                     stream,
//                     &mut (*exiFrag).c2rust_unnamed.AuthorizationReq,
//                 );
//             }
//         } else if (*exiFrag).CertificateInstallationReq_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 8 as i32 as usize, 15 as i32 as u32);
//             if error == 0 as i32 {
//                 error = encode_iso2_CertificateInstallationReqType(
//                     stream,
//                     &mut (*exiFrag).c2rust_unnamed.CertificateInstallationReq,
//                 );
//             }
//         } else if (*exiFrag).CertificateUpdateReq_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 8 as i32 as usize, 17 as i32 as u32);
//             if error == 0 as i32 {
//                 error = encode_iso2_CertificateUpdateReqType(
//                     stream,
//                     &mut (*exiFrag).c2rust_unnamed.CertificateUpdateReq,
//                 );
//             }
//         } else if (*exiFrag).ContractSignatureCertChain_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 8 as i32 as usize, 33 as i32 as u32);
//             if error == 0 as i32 {
//                 error = encode_iso2_CertificateChainType(
//                     stream,
//                     &mut (*exiFrag).c2rust_unnamed.ContractSignatureCertChain,
//                 );
//             }
//         } else if (*exiFrag).ContractSignatureEncryptedPrivateKey_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 8 as i32 as usize, 34 as i32 as u32);
//             if error == 0 as i32 {
//                 error = encode_iso2_ContractSignatureEncryptedPrivateKeyType(
//                     stream,
//                     &mut (*exiFrag)
//                         .c2rust_unnamed
//                         .ContractSignatureEncryptedPrivateKey,
//                 );
//             }
//         } else if (*exiFrag).DHpublickey_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 8 as i32 as usize, 45 as i32 as u32);
//             if error == 0 as i32 {
//                 error = encode_iso2_DiffieHellmanPublickeyType(
//                     stream,
//                     &mut (*exiFrag).c2rust_unnamed.DHpublickey,
//                 );
//             }
//         } else if (*exiFrag).MeteringReceiptReq_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 8 as i32 as usize, 121 as i32 as u32);
//             if error == 0 as i32 {
//                 error = encode_iso2_MeteringReceiptReqType(
//                     stream,
//                     &mut (*exiFrag).c2rust_unnamed.MeteringReceiptReq,
//                 );
//             }
//         } else if (*exiFrag).SalesTariff_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 8 as i32 as usize, 174 as i32 as u32);
//             if error == 0 as i32 {
//                 error =
//                     encode_iso2_SalesTariffType(stream, &mut (*exiFrag).c2rust_unnamed.SalesTariff);
//             }
//         } else if (*exiFrag).SignedInfo_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 8 as i32 as usize, 208 as i32 as u32);
//             if error == 0 as i32 {
//                 error =
//                     encode_iso2_SignedInfoType(stream, &mut (*exiFrag).c2rust_unnamed.SignedInfo);
//             }
//         } else if (*exiFrag).eMAID_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 8 as i32 as usize, 236 as i32 as u32);
//             if error == 0 as i32 {
//                 error = encode_iso2_EMAIDType(stream, &mut (*exiFrag).c2rust_unnamed.eMAID);
//             }
//         } else {
//             error = -(70 as i32);
//         }
//         if error == 0 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 8 as i32 as usize, 244 as i32 as u32);
//         }
//     }
//     return error;
// }

// pub fn encode_iso2_xmldsigFragment(
//     stream: &mut ExiBitstream,
//     mut xmldsigFrag: *mut iso2_xmldsigFragment,
// ) -> Result<(), ExiError> {
//     let mut error: i32 = exi_header_write(stream);
//     if error == 0 as i32 {
//         if (*xmldsigFrag).CanonicalizationMethod_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 0 as i32 as u32);
//             if error == 0 as i32 {
//                 error = encode_iso2_CanonicalizationMethodType(
//                     stream,
//                     &mut (*xmldsigFrag).c2rust_unnamed.CanonicalizationMethod,
//                 );
//             }
//         } else if (*xmldsigFrag).DSAKeyValue_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 1 as i32 as u32);
//             if error == 0 as i32 {
//                 error = encode_iso2_DSAKeyValueType(
//                     stream,
//                     &mut (*xmldsigFrag).c2rust_unnamed.DSAKeyValue,
//                 );
//             }
//         } else if (*xmldsigFrag).DigestMethod_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 2 as i32 as u32);
//             if error == 0 as i32 {
//                 error = encode_iso2_DigestMethodType(
//                     stream,
//                     &mut (*xmldsigFrag).c2rust_unnamed.DigestMethod,
//                 );
//             }
//         } else if (*xmldsigFrag).KeyInfo_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 8 as i32 as u32);
//             if error == 0 as i32 {
//                 error = encode_iso2_KeyInfoType(stream, &mut (*xmldsigFrag).c2rust_unnamed.KeyInfo);
//             }
//         } else if (*xmldsigFrag).KeyValue_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 10 as i32 as u32);
//             if error == 0 as i32 {
//                 error =
//                     encode_iso2_KeyValueType(stream, &mut (*xmldsigFrag).c2rust_unnamed.KeyValue);
//             }
//         } else if (*xmldsigFrag).Object_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 14 as i32 as u32);
//             if error == 0 as i32 {
//                 error = encode_iso2_ObjectType(stream, &mut (*xmldsigFrag).c2rust_unnamed.Object);
//             }
//         } else if (*xmldsigFrag).PGPData_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 16 as i32 as u32);
//             if error == 0 as i32 {
//                 error = encode_iso2_PGPDataType(stream, &mut (*xmldsigFrag).c2rust_unnamed.PGPData);
//             }
//         } else if (*xmldsigFrag).RSAKeyValue_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 21 as i32 as u32);
//             if error == 0 as i32 {
//                 error = encode_iso2_RSAKeyValueType(
//                     stream,
//                     &mut (*xmldsigFrag).c2rust_unnamed.RSAKeyValue,
//                 );
//             }
//         } else if (*xmldsigFrag).Reference_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 22 as i32 as u32);
//             if error == 0 as i32 {
//                 error =
//                     encode_iso2_ReferenceType(stream, &mut (*xmldsigFrag).c2rust_unnamed.Reference);
//             }
//         } else if (*xmldsigFrag).RetrievalMethod_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 23 as i32 as u32);
//             if error == 0 as i32 {
//                 error = encode_iso2_RetrievalMethodType(
//                     stream,
//                     &mut (*xmldsigFrag).c2rust_unnamed.RetrievalMethod,
//                 );
//             }
//         } else if (*xmldsigFrag).SPKIData_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 24 as i32 as u32);
//             if error == 0 as i32 {
//                 error =
//                     encode_iso2_SPKIDataType(stream, &mut (*xmldsigFrag).c2rust_unnamed.SPKIData);
//             }
//         } else if (*xmldsigFrag).Signature_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 27 as i32 as u32);
//             if error == 0 as i32 {
//                 error =
//                     encode_iso2_SignatureType(stream, &mut (*xmldsigFrag).c2rust_unnamed.Signature);
//             }
//         } else if (*xmldsigFrag).SignatureMethod_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 28 as i32 as u32);
//             if error == 0 as i32 {
//                 error = encode_iso2_SignatureMethodType(
//                     stream,
//                     &mut (*xmldsigFrag).c2rust_unnamed.SignatureMethod,
//                 );
//             }
//         } else if (*xmldsigFrag).SignatureValue_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 31 as i32 as u32);
//             if error == 0 as i32 {
//                 error = encode_iso2_SignatureValueType(
//                     stream,
//                     &mut (*xmldsigFrag).c2rust_unnamed.SignatureValue,
//                 );
//             }
//         } else if (*xmldsigFrag).SignedInfo_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 32 as i32 as u32);
//             if error == 0 as i32 {
//                 error = encode_iso2_SignedInfoType(
//                     stream,
//                     &mut (*xmldsigFrag).c2rust_unnamed.SignedInfo,
//                 );
//             }
//         } else if (*xmldsigFrag).Transform_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 33 as i32 as u32);
//             if error == 0 as i32 {
//                 error =
//                     encode_iso2_TransformType(stream, &mut (*xmldsigFrag).c2rust_unnamed.Transform);
//             }
//         } else if (*xmldsigFrag).Transforms_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 34 as i32 as u32);
//             if error == 0 as i32 {
//                 error = encode_iso2_TransformsType(
//                     stream,
//                     &mut (*xmldsigFrag).c2rust_unnamed.Transforms,
//                 );
//             }
//         } else if (*xmldsigFrag).X509Data_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 37 as i32 as u32);
//             if error == 0 as i32 {
//                 error =
//                     encode_iso2_X509DataType(stream, &mut (*xmldsigFrag).c2rust_unnamed.X509Data);
//             }
//         } else if (*xmldsigFrag).X509IssuerSerial_isUsed() as i32 == 1 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 39 as i32 as u32);
//             if error == 0 as i32 {
//                 error = encode_iso2_X509IssuerSerialType(
//                     stream,
//                     &mut (*xmldsigFrag).c2rust_unnamed.X509IssuerSerial,
//                 );
//             }
//         } else {
//             error = -(70 as i32);
//         }
//         if error == 0 as i32 {
//             exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 46 as i32 as u32);
//         }
//     }
//     return error;
// }
