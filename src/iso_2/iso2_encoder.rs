fn encode_iso2_CostType(
    stream: &mut ExiBitstream,
    mut CostType: *const iso2_CostType,
) -> i32 {
    let mut grammar_id: i32 = 0 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            0 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*CostType).costKind as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 1 as i32;
                            }
                        }
                    }
                }
            }
            1 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_32(stream, (*CostType).amount);
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 2 as i32;
                            }
                        }
                    }
                }
            }
            2 => {
                if (*CostType).amountMultiplier_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                3 as i32 as usize,
                                ((*CostType).amountMultiplier as u32)
                                    .wrapping_sub(-(3 as i32) as u32),
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_TransformType(
    stream: &mut ExiBitstream,
    mut TransformType: *const iso2_TransformType,
) -> i32 {
    let mut grammar_id: i32 = 5 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            5 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = exi_basetypes_encoder_uint_16(
                        stream,
                        ((*TransformType).Algorithm.charactersLen as i32 + 2 as i32) as u16,
                    );
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_characters(
                            stream,
                            (*TransformType).Algorithm.charactersLen as usize,
                            ((*TransformType).Algorithm.characters).as_ptr(),
                            (64 as i32 + 1 as i32) as usize,
                        );
                        if error == 0 as i32 {
                            grammar_id = 6 as i32;
                        }
                    }
                }
            }
            6 => {
                if (*TransformType).XPath_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                ((*TransformType).XPath.charactersLen as i32 + 2 as i32) as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*TransformType).XPath.charactersLen as usize,
                                    ((*TransformType).XPath.characters).as_ptr(),
                                    (64 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*TransformType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*TransformType).ANY.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*TransformType).ANY.bytesLen as usize,
                                    ((*TransformType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_IntervalType(
    stream: &mut ExiBitstream,
    mut IntervalType: *const iso2_IntervalType,
) -> i32 {
    let mut error: i32 =
        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
    return error;
}
fn encode_iso2_TransformsType(
    stream: &mut ExiBitstream,
    mut TransformsType: *const iso2_TransformsType,
) -> i32 {
    let mut grammar_id: i32 = 7 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            7 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_TransformType(stream, &(*TransformsType).Transform);
                    if error == 0 as i32 {
                        grammar_id = 8 as i32;
                    }
                }
            }
            8 => {
                if 1 as i32 == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_TransformType(stream, &(*TransformsType).Transform);
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_DSAKeyValueType(
    stream: &mut ExiBitstream,
    mut DSAKeyValueType: *const iso2_DSAKeyValueType,
) -> i32 {
    let mut grammar_id: i32 = 9 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            9 => {
                if (*DSAKeyValueType).P_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*DSAKeyValueType).P.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*DSAKeyValueType).P.bytesLen as usize,
                                    ((*DSAKeyValueType).P.bytes).as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 10 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*DSAKeyValueType).G_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*DSAKeyValueType).G.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*DSAKeyValueType).G.bytesLen as usize,
                                    ((*DSAKeyValueType).G.bytes).as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 12 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*DSAKeyValueType).Y.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*DSAKeyValueType).Y.bytesLen as usize,
                                    ((*DSAKeyValueType).Y.bytes).as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 13 as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            10 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            exi_basetypes_encoder_uint_16(stream, (*DSAKeyValueType).Q.bytesLen);
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*DSAKeyValueType).Q.bytesLen as usize,
                                ((*DSAKeyValueType).Q.bytes).as_ptr(),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 11 as i32;
                                }
                            }
                        }
                    }
                }
            }
            11 => {
                if (*DSAKeyValueType).G_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*DSAKeyValueType).G.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*DSAKeyValueType).G.bytesLen as usize,
                                    ((*DSAKeyValueType).G.bytes).as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 12 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*DSAKeyValueType).Y.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*DSAKeyValueType).Y.bytesLen as usize,
                                    ((*DSAKeyValueType).Y.bytes).as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 13 as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            12 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            exi_basetypes_encoder_uint_16(stream, (*DSAKeyValueType).Y.bytesLen);
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*DSAKeyValueType).Y.bytesLen as usize,
                                ((*DSAKeyValueType).Y.bytes).as_ptr(),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 13 as i32;
                                }
                            }
                        }
                    }
                }
            }
            13 => {
                if (*DSAKeyValueType).J_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*DSAKeyValueType).J.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*DSAKeyValueType).J.bytesLen as usize,
                                    ((*DSAKeyValueType).J.bytes).as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 14 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*DSAKeyValueType).Seed_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*DSAKeyValueType).Seed.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*DSAKeyValueType).Seed.bytesLen as usize,
                                    ((*DSAKeyValueType).Seed.bytes).as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 15 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            14 => {
                if (*DSAKeyValueType).Seed_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*DSAKeyValueType).Seed.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*DSAKeyValueType).Seed.bytesLen as usize,
                                    ((*DSAKeyValueType).Seed.bytes).as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 15 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            15 => {
                if (*DSAKeyValueType).PgenCounter_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*DSAKeyValueType).PgenCounter.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*DSAKeyValueType).PgenCounter.bytesLen as usize,
                                    ((*DSAKeyValueType).PgenCounter.bytes).as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_X509IssuerSerialType(
    stream: &mut ExiBitstream,
    mut X509IssuerSerialType: *const iso2_X509IssuerSerialType,
) -> i32 {
    let mut grammar_id: i32 = 16 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            16 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*X509IssuerSerialType).X509IssuerName.charactersLen as i32 + 2 as i32)
                                as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*X509IssuerSerialType).X509IssuerName.charactersLen as usize,
                                ((*X509IssuerSerialType).X509IssuerName.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 17 as i32;
                                }
                            }
                        }
                    }
                }
            }
            17 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_signed(
                            stream,
                            &(*X509IssuerSerialType).X509SerialNumber,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_RelativeTimeIntervalType(
    stream: &mut ExiBitstream,
    mut RelativeTimeIntervalType: *const iso2_RelativeTimeIntervalType,
) -> i32 {
    let mut grammar_id: i32 = 18 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            18 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_32(
                            stream,
                            (*RelativeTimeIntervalType).start,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 19 as i32;
                            }
                        }
                    }
                }
            }
            19 => {
                if (*RelativeTimeIntervalType).duration_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_32(
                                stream,
                                (*RelativeTimeIntervalType).duration,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_DigestMethodType(
    stream: &mut ExiBitstream,
    mut DigestMethodType: *const iso2_DigestMethodType,
) -> i32 {
    let mut grammar_id: i32 = 20 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            20 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = exi_basetypes_encoder_uint_16(
                        stream,
                        ((*DigestMethodType).Algorithm.charactersLen as i32 + 2 as i32) as u16,
                    );
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_characters(
                            stream,
                            (*DigestMethodType).Algorithm.charactersLen as usize,
                            ((*DigestMethodType).Algorithm.characters).as_ptr(),
                            (64 as i32 + 1 as i32) as usize,
                        );
                        if error == 0 as i32 {
                            grammar_id = 21 as i32;
                        }
                    }
                }
            }
            21 => {
                if (*DigestMethodType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*DigestMethodType).ANY.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*DigestMethodType).ANY.bytesLen as usize,
                                    ((*DigestMethodType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_RSAKeyValueType(
    stream: &mut ExiBitstream,
    mut RSAKeyValueType: *const iso2_RSAKeyValueType,
) -> i32 {
    let mut grammar_id: i32 = 22 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            22 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*RSAKeyValueType).Modulus.bytesLen,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*RSAKeyValueType).Modulus.bytesLen as usize,
                                ((*RSAKeyValueType).Modulus.bytes).as_ptr(),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 23 as i32;
                                }
                            }
                        }
                    }
                }
            }
            23 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*RSAKeyValueType).Exponent.bytesLen,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*RSAKeyValueType).Exponent.bytesLen as usize,
                                ((*RSAKeyValueType).Exponent.bytes).as_ptr(),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_CanonicalizationMethodType(
    stream: &mut ExiBitstream,
    mut CanonicalizationMethodType: *const iso2_CanonicalizationMethodType,
) -> i32 {
    let mut grammar_id: i32 = 24 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            24 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = exi_basetypes_encoder_uint_16(
                        stream,
                        ((*CanonicalizationMethodType).Algorithm.charactersLen as i32 + 2 as i32)
                            as u16,
                    );
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_characters(
                            stream,
                            (*CanonicalizationMethodType).Algorithm.charactersLen as usize,
                            ((*CanonicalizationMethodType).Algorithm.characters).as_ptr(),
                            (64 as i32 + 1 as i32) as usize,
                        );
                        if error == 0 as i32 {
                            grammar_id = 25 as i32;
                        }
                    }
                }
            }
            25 => {
                if (*CanonicalizationMethodType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*CanonicalizationMethodType).ANY.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*CanonicalizationMethodType).ANY.bytesLen as usize,
                                    ((*CanonicalizationMethodType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_SignatureMethodType(
    stream: &mut ExiBitstream,
    mut SignatureMethodType: *const iso2_SignatureMethodType,
) -> i32 {
    let mut grammar_id: i32 = 26 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            26 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = exi_basetypes_encoder_uint_16(
                        stream,
                        ((*SignatureMethodType).Algorithm.charactersLen as i32 + 2 as i32) as u16,
                    );
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_characters(
                            stream,
                            (*SignatureMethodType).Algorithm.charactersLen as usize,
                            ((*SignatureMethodType).Algorithm.characters).as_ptr(),
                            (64 as i32 + 1 as i32) as usize,
                        );
                        if error == 0 as i32 {
                            grammar_id = 27 as i32;
                        }
                    }
                }
            }
            27 => {
                if (*SignatureMethodType).HMACOutputLength_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_signed(
                                stream,
                                &(*SignatureMethodType).HMACOutputLength,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 28 as i32;
                                }
                            }
                        }
                    }
                } else if (*SignatureMethodType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*SignatureMethodType).ANY.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*SignatureMethodType).ANY.bytesLen as usize,
                                    ((*SignatureMethodType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            28 => {
                if (*SignatureMethodType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*SignatureMethodType).ANY.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*SignatureMethodType).ANY.bytesLen as usize,
                                    ((*SignatureMethodType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_KeyValueType(
    stream: &mut ExiBitstream,
    mut KeyValueType: *const iso2_KeyValueType,
) -> i32 {
    let mut grammar_id: i32 = 29 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            29 => {
                if (*KeyValueType).DSAKeyValue_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_DSAKeyValueType(stream, &(*KeyValueType).DSAKeyValue);
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*KeyValueType).RSAKeyValue_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_RSAKeyValueType(stream, &(*KeyValueType).RSAKeyValue);
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*KeyValueType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error =
                                exi_basetypes_encoder_uint_16(stream, (*KeyValueType).ANY.bytesLen);
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*KeyValueType).ANY.bytesLen as usize,
                                    ((*KeyValueType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_PhysicalValueType(
    stream: &mut ExiBitstream,
    mut PhysicalValueType: *const iso2_PhysicalValueType,
) -> i32 {
    let mut grammar_id: i32 = 30 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            30 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            3 as i32 as usize,
                            ((*PhysicalValueType).Multiplier as u32)
                                .wrapping_sub(-(3 as i32) as u32),
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 31 as i32;
                            }
                        }
                    }
                }
            }
            31 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            3 as i32 as usize,
                            (*PhysicalValueType).Unit as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 32 as i32;
                            }
                        }
                    }
                }
            }
            32 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            exi_basetypes_encoder_integer_16(stream, (*PhysicalValueType).Value);
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_ConsumptionCostType(
    stream: &mut ExiBitstream,
    mut ConsumptionCostType: *const iso2_ConsumptionCostType,
) -> i32 {
    let mut grammar_id: i32 = 33 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut Cost_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            33 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso2_PhysicalValueType(stream, &(*ConsumptionCostType).startValue);
                    if error == 0 as i32 {
                        grammar_id = 34 as i32;
                    }
                }
            }
            34 => {
                if (Cost_currentIndex as i32) < (*ConsumptionCostType).Cost.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh0 = Cost_currentIndex;
                        Cost_currentIndex = Cost_currentIndex.wrapping_add(1);
                        error = encode_iso2_CostType(
                            stream,
                            &*((*ConsumptionCostType).Cost.array)
                                .as_ptr()
                                .offset(fresh0 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 35 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            35 => {
                if (Cost_currentIndex as i32) < (*ConsumptionCostType).Cost.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh1 = Cost_currentIndex;
                        Cost_currentIndex = Cost_currentIndex.wrapping_add(1);
                        error = encode_iso2_CostType(
                            stream,
                            &*((*ConsumptionCostType).Cost.array)
                                .as_ptr()
                                .offset(fresh1 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 35 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_PMaxScheduleEntryType(
    stream: &mut ExiBitstream,
    mut PMaxScheduleEntryType: *const iso2_PMaxScheduleEntryType,
) -> i32 {
    let mut grammar_id: i32 = 36 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            36 => {
                if (*PMaxScheduleEntryType).RelativeTimeInterval_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_RelativeTimeIntervalType(
                            stream,
                            &(*PMaxScheduleEntryType).RelativeTimeInterval,
                        );
                        if error == 0 as i32 {
                            grammar_id = 37 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_IntervalType(
                            stream,
                            &(*PMaxScheduleEntryType).TimeInterval,
                        );
                        if error == 0 as i32 {
                            grammar_id = 37 as i32;
                        }
                    }
                }
            }
            37 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_PhysicalValueType(stream, &(*PMaxScheduleEntryType).PMax);
                    if error == 0 as i32 {
                        grammar_id = 3 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_SalesTariffEntryType(
    stream: &mut ExiBitstream,
    mut SalesTariffEntryType: *const iso2_SalesTariffEntryType,
) -> i32 {
    let mut grammar_id: i32 = 38 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut ConsumptionCost_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            38 => {
                if (*SalesTariffEntryType).RelativeTimeInterval_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_RelativeTimeIntervalType(
                            stream,
                            &(*SalesTariffEntryType).RelativeTimeInterval,
                        );
                        if error == 0 as i32 {
                            grammar_id = 39 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso2_IntervalType(stream, &(*SalesTariffEntryType).TimeInterval);
                        if error == 0 as i32 {
                            grammar_id = 39 as i32;
                        }
                    }
                }
            }
            39 => {
                if (*SalesTariffEntryType).EPriceLevel_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                8 as i32 as usize,
                                (*SalesTariffEntryType).EPriceLevel as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 41 as i32;
                                }
                            }
                        }
                    }
                } else if (ConsumptionCost_currentIndex as i32)
                    < (*SalesTariffEntryType).ConsumptionCost.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh2 = ConsumptionCost_currentIndex;
                        ConsumptionCost_currentIndex = ConsumptionCost_currentIndex.wrapping_add(1);
                        error = encode_iso2_ConsumptionCostType(
                            stream,
                            &*((*SalesTariffEntryType).ConsumptionCost.array)
                                .as_ptr()
                                .offset(fresh2 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 40 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            40 => {
                if (ConsumptionCost_currentIndex as i32)
                    < (*SalesTariffEntryType).ConsumptionCost.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh3 = ConsumptionCost_currentIndex;
                        ConsumptionCost_currentIndex = ConsumptionCost_currentIndex.wrapping_add(1);
                        error = encode_iso2_ConsumptionCostType(
                            stream,
                            &*((*SalesTariffEntryType).ConsumptionCost.array)
                                .as_ptr()
                                .offset(fresh3 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 40 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            41 => {
                if (ConsumptionCost_currentIndex as i32)
                    < (*SalesTariffEntryType).ConsumptionCost.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh4 = ConsumptionCost_currentIndex;
                        ConsumptionCost_currentIndex = ConsumptionCost_currentIndex.wrapping_add(1);
                        error = encode_iso2_ConsumptionCostType(
                            stream,
                            &*((*SalesTariffEntryType).ConsumptionCost.array)
                                .as_ptr()
                                .offset(fresh4 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 42 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            42 => {
                if (ConsumptionCost_currentIndex as i32)
                    < (*SalesTariffEntryType).ConsumptionCost.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh5 = ConsumptionCost_currentIndex;
                        ConsumptionCost_currentIndex = ConsumptionCost_currentIndex.wrapping_add(1);
                        error = encode_iso2_ConsumptionCostType(
                            stream,
                            &*((*SalesTariffEntryType).ConsumptionCost.array)
                                .as_ptr()
                                .offset(fresh5 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 42 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_ParameterType(
    stream: &mut ExiBitstream,
    mut ParameterType: *const iso2_ParameterType,
) -> i32 {
    let mut grammar_id: i32 = 43 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            43 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = exi_basetypes_encoder_uint_16(
                        stream,
                        ((*ParameterType).Name.charactersLen as i32 + 2 as i32) as u16,
                    );
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_characters(
                            stream,
                            (*ParameterType).Name.charactersLen as usize,
                            ((*ParameterType).Name.characters).as_ptr(),
                            (64 as i32 + 1 as i32) as usize,
                        );
                        if error == 0 as i32 {
                            grammar_id = 44 as i32;
                        }
                    }
                }
            }
            44 => {
                if (*ParameterType).boolValue_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(stream, (*ParameterType).boolValue);
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                } else if (*ParameterType).byteValue_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                8 as i32 as usize,
                                ((*ParameterType).byteValue as i32 + -(128 as i32)) as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                } else if (*ParameterType).shortValue_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_integer_16(
                                stream,
                                (*ParameterType).shortValue,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                } else if (*ParameterType).intValue_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error =
                                exi_basetypes_encoder_integer_32(stream, (*ParameterType).intValue);
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                } else if (*ParameterType).physicalValue_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso2_PhysicalValueType(stream, &(*ParameterType).physicalValue);
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                ((*ParameterType).stringValue.charactersLen as i32 + 2 as i32)
                                    as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*ParameterType).stringValue.charactersLen as usize,
                                    ((*ParameterType).stringValue.characters).as_ptr(),
                                    (64 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_PMaxScheduleType(
    stream: &mut ExiBitstream,
    mut PMaxScheduleType: *const iso2_PMaxScheduleType,
) -> i32 {
    let mut grammar_id: i32 = 45 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut PMaxScheduleEntry_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            45 => {
                if (PMaxScheduleEntry_currentIndex as i32)
                    < (*PMaxScheduleType).PMaxScheduleEntry.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh6 = PMaxScheduleEntry_currentIndex;
                        PMaxScheduleEntry_currentIndex =
                            PMaxScheduleEntry_currentIndex.wrapping_add(1);
                        error = encode_iso2_PMaxScheduleEntryType(
                            stream,
                            &*((*PMaxScheduleType).PMaxScheduleEntry.array)
                                .as_ptr()
                                .offset(fresh6 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 46 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            46 => {
                if (PMaxScheduleEntry_currentIndex as i32)
                    < (*PMaxScheduleType).PMaxScheduleEntry.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh7 = PMaxScheduleEntry_currentIndex;
                        PMaxScheduleEntry_currentIndex =
                            PMaxScheduleEntry_currentIndex.wrapping_add(1);
                        error = encode_iso2_PMaxScheduleEntryType(
                            stream,
                            &*((*PMaxScheduleType).PMaxScheduleEntry.array)
                                .as_ptr()
                                .offset(fresh7 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 46 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_ReferenceType(
    stream: &mut ExiBitstream,
    mut ReferenceType: *const iso2_ReferenceType,
) -> i32 {
    let mut grammar_id: i32 = 47 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            47 => {
                if (*ReferenceType).Id_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*ReferenceType).Id.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*ReferenceType).Id.charactersLen as usize,
                                ((*ReferenceType).Id.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 48 as i32;
                            }
                        }
                    }
                } else if (*ReferenceType).Type_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*ReferenceType).Type.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*ReferenceType).Type.charactersLen as usize,
                                ((*ReferenceType).Type.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 49 as i32;
                            }
                        }
                    }
                } else if (*ReferenceType).URI_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*ReferenceType).URI.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*ReferenceType).URI.charactersLen as usize,
                                ((*ReferenceType).URI.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 50 as i32;
                            }
                        }
                    }
                } else if (*ReferenceType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_TransformsType(stream, &(*ReferenceType).Transforms);
                        if error == 0 as i32 {
                            grammar_id = 51 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso2_DigestMethodType(stream, &(*ReferenceType).DigestMethod);
                        if error == 0 as i32 {
                            grammar_id = 52 as i32;
                        }
                    }
                }
            }
            48 => {
                if (*ReferenceType).Type_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*ReferenceType).Type.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*ReferenceType).Type.charactersLen as usize,
                                ((*ReferenceType).Type.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 49 as i32;
                            }
                        }
                    }
                } else if (*ReferenceType).URI_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*ReferenceType).URI.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*ReferenceType).URI.charactersLen as usize,
                                ((*ReferenceType).URI.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 50 as i32;
                            }
                        }
                    }
                } else if (*ReferenceType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_TransformsType(stream, &(*ReferenceType).Transforms);
                        if error == 0 as i32 {
                            grammar_id = 51 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso2_DigestMethodType(stream, &(*ReferenceType).DigestMethod);
                        if error == 0 as i32 {
                            grammar_id = 52 as i32;
                        }
                    }
                }
            }
            49 => {
                if (*ReferenceType).URI_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*ReferenceType).URI.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*ReferenceType).URI.charactersLen as usize,
                                ((*ReferenceType).URI.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 50 as i32;
                            }
                        }
                    }
                } else if (*ReferenceType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_TransformsType(stream, &(*ReferenceType).Transforms);
                        if error == 0 as i32 {
                            grammar_id = 51 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso2_DigestMethodType(stream, &(*ReferenceType).DigestMethod);
                        if error == 0 as i32 {
                            grammar_id = 52 as i32;
                        }
                    }
                }
            }
            50 => {
                if (*ReferenceType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_TransformsType(stream, &(*ReferenceType).Transforms);
                        if error == 0 as i32 {
                            grammar_id = 51 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso2_DigestMethodType(stream, &(*ReferenceType).DigestMethod);
                        if error == 0 as i32 {
                            grammar_id = 52 as i32;
                        }
                    }
                }
            }
            51 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_DigestMethodType(stream, &(*ReferenceType).DigestMethod);
                    if error == 0 as i32 {
                        grammar_id = 52 as i32;
                    }
                }
            }
            52 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*ReferenceType).DigestValue.bytesLen,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*ReferenceType).DigestValue.bytesLen as usize,
                                ((*ReferenceType).DigestValue.bytes).as_ptr(),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_RetrievalMethodType(
    stream: &mut ExiBitstream,
    mut RetrievalMethodType: *const iso2_RetrievalMethodType,
) -> i32 {
    let mut grammar_id: i32 = 53 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            53 => {
                if (*RetrievalMethodType).Type_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*RetrievalMethodType).Type.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*RetrievalMethodType).Type.charactersLen as usize,
                                ((*RetrievalMethodType).Type.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 54 as i32;
                            }
                        }
                    }
                } else if (*RetrievalMethodType).URI_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*RetrievalMethodType).URI.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*RetrievalMethodType).URI.charactersLen as usize,
                                ((*RetrievalMethodType).URI.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 55 as i32;
                            }
                        }
                    }
                } else if (*RetrievalMethodType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso2_TransformsType(stream, &(*RetrievalMethodType).Transforms);
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            54 => {
                if (*RetrievalMethodType).URI_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*RetrievalMethodType).URI.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*RetrievalMethodType).URI.charactersLen as usize,
                                ((*RetrievalMethodType).URI.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 55 as i32;
                            }
                        }
                    }
                } else if (*RetrievalMethodType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso2_TransformsType(stream, &(*RetrievalMethodType).Transforms);
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            55 => {
                if (*RetrievalMethodType).Transforms_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso2_TransformsType(stream, &(*RetrievalMethodType).Transforms);
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_SalesTariffType(
    stream: &mut ExiBitstream,
    mut SalesTariffType: *const iso2_SalesTariffType,
) -> i32 {
    let mut grammar_id: i32 = 56 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut SalesTariffEntry_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            56 => {
                if (*SalesTariffType).Id_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*SalesTariffType).Id.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*SalesTariffType).Id.charactersLen as usize,
                                ((*SalesTariffType).Id.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 57 as i32;
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                8 as i32 as usize,
                                ((*SalesTariffType).SalesTariffID as u32)
                                    .wrapping_sub(1 as i32 as u32),
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 58 as i32;
                                }
                            }
                        }
                    }
                }
            }
            57 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            8 as i32 as usize,
                            ((*SalesTariffType).SalesTariffID as u32).wrapping_sub(1 as i32 as u32),
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 58 as i32;
                            }
                        }
                    }
                }
            }
            58 => {
                if (*SalesTariffType).SalesTariffDescription_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                ((*SalesTariffType).SalesTariffDescription.charactersLen as i32
                                    + 2 as i32) as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*SalesTariffType).SalesTariffDescription.charactersLen
                                        as usize,
                                    ((*SalesTariffType).SalesTariffDescription.characters).as_ptr(),
                                    (32 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 60 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*SalesTariffType).NumEPriceLevels_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                8 as i32 as usize,
                                (*SalesTariffType).NumEPriceLevels as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 62 as i32;
                                }
                            }
                        }
                    }
                } else if (SalesTariffEntry_currentIndex as i32)
                    < (*SalesTariffType).SalesTariffEntry.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh8 = SalesTariffEntry_currentIndex;
                        SalesTariffEntry_currentIndex =
                            SalesTariffEntry_currentIndex.wrapping_add(1);
                        error = encode_iso2_SalesTariffEntryType(
                            stream,
                            &*((*SalesTariffType).SalesTariffEntry.array)
                                .as_ptr()
                                .offset(fresh8 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 59 as i32;
                        }
                    }
                }
            }
            59 => {
                if (SalesTariffEntry_currentIndex as i32)
                    < (*SalesTariffType).SalesTariffEntry.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh9 = SalesTariffEntry_currentIndex;
                        SalesTariffEntry_currentIndex =
                            SalesTariffEntry_currentIndex.wrapping_add(1);
                        error = encode_iso2_SalesTariffEntryType(
                            stream,
                            &*((*SalesTariffType).SalesTariffEntry.array)
                                .as_ptr()
                                .offset(fresh9 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 59 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            60 => {
                if (*SalesTariffType).NumEPriceLevels_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                8 as i32 as usize,
                                (*SalesTariffType).NumEPriceLevels as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 62 as i32;
                                }
                            }
                        }
                    }
                } else if (SalesTariffEntry_currentIndex as i32)
                    < (*SalesTariffType).SalesTariffEntry.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh10 = SalesTariffEntry_currentIndex;
                        SalesTariffEntry_currentIndex =
                            SalesTariffEntry_currentIndex.wrapping_add(1);
                        error = encode_iso2_SalesTariffEntryType(
                            stream,
                            &*((*SalesTariffType).SalesTariffEntry.array)
                                .as_ptr()
                                .offset(fresh10 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 61 as i32;
                        }
                    }
                }
            }
            61 => {
                if (SalesTariffEntry_currentIndex as i32)
                    < (*SalesTariffType).SalesTariffEntry.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh11 = SalesTariffEntry_currentIndex;
                        SalesTariffEntry_currentIndex =
                            SalesTariffEntry_currentIndex.wrapping_add(1);
                        error = encode_iso2_SalesTariffEntryType(
                            stream,
                            &*((*SalesTariffType).SalesTariffEntry.array)
                                .as_ptr()
                                .offset(fresh11 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 61 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            62 => {
                if (SalesTariffEntry_currentIndex as i32)
                    < (*SalesTariffType).SalesTariffEntry.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh12 = SalesTariffEntry_currentIndex;
                        SalesTariffEntry_currentIndex =
                            SalesTariffEntry_currentIndex.wrapping_add(1);
                        error = encode_iso2_SalesTariffEntryType(
                            stream,
                            &*((*SalesTariffType).SalesTariffEntry.array)
                                .as_ptr()
                                .offset(fresh12 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 63 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            63 => {
                if (SalesTariffEntry_currentIndex as i32)
                    < (*SalesTariffType).SalesTariffEntry.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh13 = SalesTariffEntry_currentIndex;
                        SalesTariffEntry_currentIndex =
                            SalesTariffEntry_currentIndex.wrapping_add(1);
                        error = encode_iso2_SalesTariffEntryType(
                            stream,
                            &*((*SalesTariffType).SalesTariffEntry.array)
                                .as_ptr()
                                .offset(fresh13 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 63 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_X509DataType(
    stream: &mut ExiBitstream,
    mut X509DataType: *const iso2_X509DataType,
) -> i32 {
    let mut grammar_id: i32 = 64 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            64 => {
                if (*X509DataType).X509IssuerSerial_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_X509IssuerSerialType(
                            stream,
                            &(*X509DataType).X509IssuerSerial,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*X509DataType).X509SKI_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*X509DataType).X509SKI.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*X509DataType).X509SKI.bytesLen as usize,
                                    ((*X509DataType).X509SKI.bytes).as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*X509DataType).X509SubjectName_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                ((*X509DataType).X509SubjectName.charactersLen as i32 + 2 as i32)
                                    as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*X509DataType).X509SubjectName.charactersLen as usize,
                                    ((*X509DataType).X509SubjectName.characters).as_ptr(),
                                    (64 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*X509DataType).X509Certificate_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*X509DataType).X509Certificate.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*X509DataType).X509Certificate.bytesLen as usize,
                                    ((*X509DataType).X509Certificate.bytes).as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*X509DataType).X509CRL_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*X509DataType).X509CRL.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*X509DataType).X509CRL.bytesLen as usize,
                                    ((*X509DataType).X509CRL.bytes).as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*X509DataType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error =
                                exi_basetypes_encoder_uint_16(stream, (*X509DataType).ANY.bytesLen);
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*X509DataType).ANY.bytesLen as usize,
                                    ((*X509DataType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_PGPDataType(
    stream: &mut ExiBitstream,
    mut PGPDataType: *const iso2_PGPDataType,
) -> i32 {
    let mut grammar_id: i32 = 65 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            65 => {
                if ((*PGPDataType).c2rust_unnamed).choice_1_isUsed == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*PGPDataType).c2rust_unnamed.choice_1.PGPKeyID.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*PGPDataType).c2rust_unnamed.choice_1.PGPKeyID.bytesLen
                                        as usize,
                                    ((*PGPDataType).c2rust_unnamed.choice_1.PGPKeyID.bytes)
                                        .as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 66 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*PGPDataType).c2rust_unnamed.choice_1.PGPKeyPacket.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*PGPDataType).c2rust_unnamed.choice_1.PGPKeyPacket.bytesLen
                                        as usize,
                                    ((*PGPDataType).c2rust_unnamed.choice_1.PGPKeyPacket.bytes)
                                        .as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 67 as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            66 => {
                if ((*PGPDataType).c2rust_unnamed).choice_1_isUsed == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*PGPDataType).c2rust_unnamed.choice_1.PGPKeyPacket.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*PGPDataType).c2rust_unnamed.choice_1.PGPKeyPacket.bytesLen
                                        as usize,
                                    ((*PGPDataType).c2rust_unnamed.choice_1.PGPKeyPacket.bytes)
                                        .as_ptr(),
                                    350 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 67 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if ((*PGPDataType).c2rust_unnamed).choice_1_isUsed == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*PGPDataType).c2rust_unnamed.choice_1.ANY.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*PGPDataType).c2rust_unnamed.choice_1.ANY.bytesLen as usize,
                                    ((*PGPDataType).c2rust_unnamed.choice_1.ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 68 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            67 => {
                if ((*PGPDataType).c2rust_unnamed).choice_1_isUsed == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*PGPDataType).c2rust_unnamed.choice_1.ANY.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*PGPDataType).c2rust_unnamed.choice_1.ANY.bytesLen as usize,
                                    ((*PGPDataType).c2rust_unnamed.choice_1.ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 68 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            68 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*PGPDataType).c2rust_unnamed.choice_2.PGPKeyPacket.bytesLen,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*PGPDataType).c2rust_unnamed.choice_2.PGPKeyPacket.bytesLen
                                    as usize,
                                ((*PGPDataType).c2rust_unnamed.choice_2.PGPKeyPacket.bytes)
                                    .as_ptr(),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 69 as i32;
                                }
                            }
                        }
                    }
                }
            }
            69 => {
                if ((*PGPDataType).c2rust_unnamed).choice_2_isUsed == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*PGPDataType).c2rust_unnamed.choice_2.ANY.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*PGPDataType).c2rust_unnamed.choice_2.ANY.bytesLen as usize,
                                    ((*PGPDataType).c2rust_unnamed.choice_2.ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 68 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_SPKIDataType(
    stream: &mut ExiBitstream,
    mut SPKIDataType: *const iso2_SPKIDataType,
) -> i32 {
    let mut grammar_id: i32 = 70 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            70 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*SPKIDataType).SPKISexp.bytesLen,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*SPKIDataType).SPKISexp.bytesLen as usize,
                                ((*SPKIDataType).SPKISexp.bytes).as_ptr(),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 71 as i32;
                                }
                            }
                        }
                    }
                }
            }
            71 => {
                if (*SPKIDataType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error =
                                exi_basetypes_encoder_uint_16(stream, (*SPKIDataType).ANY.bytesLen);
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*SPKIDataType).ANY.bytesLen as usize,
                                    ((*SPKIDataType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_SignedInfoType(
    stream: &mut ExiBitstream,
    mut SignedInfoType: *const iso2_SignedInfoType,
) -> i32 {
    let mut grammar_id: i32 = 72 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut Reference_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            72 => {
                if (*SignedInfoType).Id_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*SignedInfoType).Id.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*SignedInfoType).Id.charactersLen as usize,
                                ((*SignedInfoType).Id.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 73 as i32;
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_CanonicalizationMethodType(
                            stream,
                            &(*SignedInfoType).CanonicalizationMethod,
                        );
                        if error == 0 as i32 {
                            grammar_id = 74 as i32;
                        }
                    }
                }
            }
            73 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_CanonicalizationMethodType(
                        stream,
                        &(*SignedInfoType).CanonicalizationMethod,
                    );
                    if error == 0 as i32 {
                        grammar_id = 74 as i32;
                    }
                }
            }
            74 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso2_SignatureMethodType(stream, &(*SignedInfoType).SignatureMethod);
                    if error == 0 as i32 {
                        grammar_id = 75 as i32;
                    }
                }
            }
            75 => {
                if (Reference_currentIndex as i32) < (*SignedInfoType).Reference.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh14 = Reference_currentIndex;
                        Reference_currentIndex = Reference_currentIndex.wrapping_add(1);
                        error = encode_iso2_ReferenceType(
                            stream,
                            &*((*SignedInfoType).Reference.array)
                                .as_ptr()
                                .offset(fresh14 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 76 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            76 => {
                if (Reference_currentIndex as i32) < (*SignedInfoType).Reference.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh15 = Reference_currentIndex;
                        Reference_currentIndex = Reference_currentIndex.wrapping_add(1);
                        error = encode_iso2_ReferenceType(
                            stream,
                            &*((*SignedInfoType).Reference.array)
                                .as_ptr()
                                .offset(fresh15 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 76 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_ProfileEntryType(
    stream: &mut ExiBitstream,
    mut ProfileEntryType: *const iso2_ProfileEntryType,
) -> i32 {
    let mut grammar_id: i32 = 77 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            77 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_32(
                            stream,
                            (*ProfileEntryType).ChargingProfileEntryStart,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 78 as i32;
                            }
                        }
                    }
                }
            }
            78 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_PhysicalValueType(
                        stream,
                        &(*ProfileEntryType).ChargingProfileEntryMaxPower,
                    );
                    if error == 0 as i32 {
                        grammar_id = 79 as i32;
                    }
                }
            }
            79 => {
                if (*ProfileEntryType).ChargingProfileEntryMaxNumberOfPhasesInUse_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                2 as i32 as usize,
                                ((*ProfileEntryType).ChargingProfileEntryMaxNumberOfPhasesInUse
                                    as u32)
                                    .wrapping_sub(1 as i32 as u32),
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_DC_EVStatusType(
    stream: &mut ExiBitstream,
    mut DC_EVStatusType: *const iso2_DC_EVStatusType,
) -> i32 {
    let mut grammar_id: i32 = 80 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            80 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(stream, (*DC_EVStatusType).EVReady);
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 81 as i32;
                            }
                        }
                    }
                }
            }
            81 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            4 as i32 as usize,
                            (*DC_EVStatusType).EVErrorCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 82 as i32;
                            }
                        }
                    }
                }
            }
            82 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            7 as i32 as usize,
                            (*DC_EVStatusType).EVRESSSOC as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_ParameterSetType(
    stream: &mut ExiBitstream,
    mut ParameterSetType: *const iso2_ParameterSetType,
) -> i32 {
    let mut grammar_id: i32 = 83 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut Parameter_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            83 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_integer_16(
                            stream,
                            (*ParameterSetType).ParameterSetID,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 84 as i32;
                            }
                        }
                    }
                }
            }
            84 => {
                if (Parameter_currentIndex as i32) < (*ParameterSetType).Parameter.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh16 = Parameter_currentIndex;
                        Parameter_currentIndex = Parameter_currentIndex.wrapping_add(1);
                        error = encode_iso2_ParameterType(
                            stream,
                            &*((*ParameterSetType).Parameter.array)
                                .as_ptr()
                                .offset(fresh16 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 85 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            85 => {
                if (Parameter_currentIndex as i32) < (*ParameterSetType).Parameter.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh17 = Parameter_currentIndex;
                        Parameter_currentIndex = Parameter_currentIndex.wrapping_add(1);
                        error = encode_iso2_ParameterType(
                            stream,
                            &*((*ParameterSetType).Parameter.array)
                                .as_ptr()
                                .offset(fresh17 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 85 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_SAScheduleTupleType(
    stream: &mut ExiBitstream,
    mut SAScheduleTupleType: *const iso2_SAScheduleTupleType,
) -> i32 {
    let mut grammar_id: i32 = 86 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            86 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            8 as i32 as usize,
                            ((*SAScheduleTupleType).SAScheduleTupleID as u32)
                                .wrapping_sub(1 as i32 as u32),
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 87 as i32;
                            }
                        }
                    }
                }
            }
            87 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso2_PMaxScheduleType(stream, &(*SAScheduleTupleType).PMaxSchedule);
                    if error == 0 as i32 {
                        grammar_id = 88 as i32;
                    }
                }
            }
            88 => {
                if (*SAScheduleTupleType).SalesTariff_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_SalesTariffType(
                            stream,
                            &(*SAScheduleTupleType).SalesTariff,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_SelectedServiceType(
    stream: &mut ExiBitstream,
    mut SelectedServiceType: *const iso2_SelectedServiceType,
) -> i32 {
    let mut grammar_id: i32 = 89 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            89 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            exi_basetypes_encoder_uint_16(stream, (*SelectedServiceType).ServiceID);
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 90 as i32;
                            }
                        }
                    }
                }
            }
            90 => {
                if (*SelectedServiceType).ParameterSetID_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_integer_16(
                                stream,
                                (*SelectedServiceType).ParameterSetID,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_ServiceType(
    stream: &mut ExiBitstream,
    mut ServiceType: *const iso2_ServiceType,
) -> i32 {
    let mut grammar_id: i32 = 91 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            91 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(stream, (*ServiceType).ServiceID);
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 92 as i32;
                            }
                        }
                    }
                }
            }
            92 => {
                if (*ServiceType).ServiceName_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                ((*ServiceType).ServiceName.charactersLen as i32 + 2 as i32) as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*ServiceType).ServiceName.charactersLen as usize,
                                    ((*ServiceType).ServiceName.characters).as_ptr(),
                                    (32 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 93 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                2 as i32 as usize,
                                (*ServiceType).ServiceCategory as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 94 as i32;
                                }
                            }
                        }
                    }
                }
            }
            93 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*ServiceType).ServiceCategory as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 94 as i32;
                            }
                        }
                    }
                }
            }
            94 => {
                if (*ServiceType).ServiceScope_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                ((*ServiceType).ServiceScope.charactersLen as i32 + 2 as i32)
                                    as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*ServiceType).ServiceScope.charactersLen as usize,
                                    ((*ServiceType).ServiceScope.characters).as_ptr(),
                                    (64 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 95 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(stream, (*ServiceType).FreeService);
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                }
            }
            95 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(stream, (*ServiceType).FreeService);
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_SignatureValueType(
    stream: &mut ExiBitstream,
    mut SignatureValueType: *const iso2_SignatureValueType,
) -> i32 {
    let mut grammar_id: i32 = 96 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            96 => {
                if (*SignatureValueType).Id_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*SignatureValueType).Id.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*SignatureValueType).Id.charactersLen as usize,
                                ((*SignatureValueType).Id.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 97 as i32;
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*SignatureValueType).CONTENT.bytesLen,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*SignatureValueType).CONTENT.bytesLen as usize,
                                ((*SignatureValueType).CONTENT.bytes).as_ptr(),
                                350 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                    }
                }
            }
            97 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = exi_basetypes_encoder_uint_16(
                        stream,
                        (*SignatureValueType).CONTENT.bytesLen,
                    );
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bytes(
                            stream,
                            (*SignatureValueType).CONTENT.bytesLen as usize,
                            ((*SignatureValueType).CONTENT.bytes).as_ptr(),
                            350 as i32 as usize,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_SubCertificatesType(
    stream: &mut ExiBitstream,
    mut SubCertificatesType: *const iso2_SubCertificatesType,
) -> i32 {
    let mut grammar_id: i32 = 98 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut Certificate_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            98 => {
                if (Certificate_currentIndex as i32)
                    < (*SubCertificatesType).Certificate.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*SubCertificatesType).Certificate.array
                                    [Certificate_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*SubCertificatesType).Certificate.array
                                        [Certificate_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*SubCertificatesType).Certificate.array
                                        [Certificate_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    800 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    Certificate_currentIndex =
                                        Certificate_currentIndex.wrapping_add(1);
                                    Certificate_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 99 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            99 => {
                if (Certificate_currentIndex as i32)
                    < (*SubCertificatesType).Certificate.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*SubCertificatesType).Certificate.array
                                    [Certificate_currentIndex as usize]
                                    .bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*SubCertificatesType).Certificate.array
                                        [Certificate_currentIndex as usize]
                                        .bytesLen as usize,
                                    ((*SubCertificatesType).Certificate.array
                                        [Certificate_currentIndex as usize]
                                        .bytes)
                                        .as_ptr(),
                                    800 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    Certificate_currentIndex =
                                        Certificate_currentIndex.wrapping_add(1);
                                    Certificate_currentIndex;
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 99 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_KeyInfoType(
    stream: &mut ExiBitstream,
    mut KeyInfoType: *const iso2_KeyInfoType,
) -> i32 {
    let mut grammar_id: i32 = 100 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            100 => {
                if (*KeyInfoType).Id_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*KeyInfoType).Id.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*KeyInfoType).Id.charactersLen as usize,
                                ((*KeyInfoType).Id.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 101 as i32;
                            }
                        }
                    }
                } else if (*KeyInfoType).KeyName_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                ((*KeyInfoType).KeyName.charactersLen as i32 + 2 as i32) as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*KeyInfoType).KeyName.charactersLen as usize,
                                    ((*KeyInfoType).KeyName.characters).as_ptr(),
                                    (64 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*KeyInfoType).KeyValue_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_KeyValueType(stream, &(*KeyInfoType).KeyValue);
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*KeyInfoType).RetrievalMethod_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_RetrievalMethodType(
                            stream,
                            &(*KeyInfoType).RetrievalMethod,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*KeyInfoType).X509Data_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_X509DataType(stream, &(*KeyInfoType).X509Data);
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*KeyInfoType).PGPData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PGPDataType(stream, &(*KeyInfoType).PGPData);
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*KeyInfoType).SPKIData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_SPKIDataType(stream, &(*KeyInfoType).SPKIData);
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*KeyInfoType).MgmtData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                ((*KeyInfoType).MgmtData.charactersLen as i32 + 2 as i32) as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*KeyInfoType).MgmtData.charactersLen as usize,
                                    ((*KeyInfoType).MgmtData.characters).as_ptr(),
                                    (64 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*KeyInfoType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 8 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error =
                                exi_basetypes_encoder_uint_16(stream, (*KeyInfoType).ANY.bytesLen);
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*KeyInfoType).ANY.bytesLen as usize,
                                    ((*KeyInfoType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            101 => {
                if (*KeyInfoType).KeyName_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                ((*KeyInfoType).KeyName.charactersLen as i32 + 2 as i32) as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*KeyInfoType).KeyName.charactersLen as usize,
                                    ((*KeyInfoType).KeyName.characters).as_ptr(),
                                    (64 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*KeyInfoType).KeyValue_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_KeyValueType(stream, &(*KeyInfoType).KeyValue);
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*KeyInfoType).RetrievalMethod_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_RetrievalMethodType(
                            stream,
                            &(*KeyInfoType).RetrievalMethod,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*KeyInfoType).X509Data_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_X509DataType(stream, &(*KeyInfoType).X509Data);
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*KeyInfoType).PGPData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PGPDataType(stream, &(*KeyInfoType).PGPData);
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*KeyInfoType).SPKIData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_SPKIDataType(stream, &(*KeyInfoType).SPKIData);
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*KeyInfoType).MgmtData_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                ((*KeyInfoType).MgmtData.charactersLen as i32 + 2 as i32) as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*KeyInfoType).MgmtData.charactersLen as usize,
                                    ((*KeyInfoType).MgmtData.characters).as_ptr(),
                                    (64 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*KeyInfoType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 4 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error =
                                exi_basetypes_encoder_uint_16(stream, (*KeyInfoType).ANY.bytesLen);
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*KeyInfoType).ANY.bytesLen as usize,
                                    ((*KeyInfoType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_ObjectType(
    stream: &mut ExiBitstream,
    mut ObjectType: *const iso2_ObjectType,
) -> i32 {
    let mut grammar_id: i32 = 102 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            102 => {
                if (*ObjectType).Encoding_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*ObjectType).Encoding.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*ObjectType).Encoding.charactersLen as usize,
                                ((*ObjectType).Encoding.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 103 as i32;
                            }
                        }
                    }
                } else if (*ObjectType).Id_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*ObjectType).Id.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*ObjectType).Id.charactersLen as usize,
                                ((*ObjectType).Id.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 104 as i32;
                            }
                        }
                    }
                } else if (*ObjectType).MimeType_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*ObjectType).MimeType.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*ObjectType).MimeType.charactersLen as usize,
                                ((*ObjectType).MimeType.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 105 as i32;
                            }
                        }
                    }
                } else if (*ObjectType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error =
                                exi_basetypes_encoder_uint_16(stream, (*ObjectType).ANY.bytesLen);
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*ObjectType).ANY.bytesLen as usize,
                                    ((*ObjectType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            103 => {
                if (*ObjectType).Id_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*ObjectType).Id.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*ObjectType).Id.charactersLen as usize,
                                ((*ObjectType).Id.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 104 as i32;
                            }
                        }
                    }
                } else if (*ObjectType).MimeType_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*ObjectType).MimeType.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*ObjectType).MimeType.charactersLen as usize,
                                ((*ObjectType).MimeType.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 105 as i32;
                            }
                        }
                    }
                } else if (*ObjectType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error =
                                exi_basetypes_encoder_uint_16(stream, (*ObjectType).ANY.bytesLen);
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*ObjectType).ANY.bytesLen as usize,
                                    ((*ObjectType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            104 => {
                if (*ObjectType).MimeType_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*ObjectType).MimeType.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*ObjectType).MimeType.charactersLen as usize,
                                ((*ObjectType).MimeType.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 105 as i32;
                            }
                        }
                    }
                } else if (*ObjectType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error =
                                exi_basetypes_encoder_uint_16(stream, (*ObjectType).ANY.bytesLen);
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*ObjectType).ANY.bytesLen as usize,
                                    ((*ObjectType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            105 => {
                if (*ObjectType).ANY_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error =
                                exi_basetypes_encoder_uint_16(stream, (*ObjectType).ANY.bytesLen);
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*ObjectType).ANY.bytesLen as usize,
                                    ((*ObjectType).ANY.bytes).as_ptr(),
                                    4 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_SupportedEnergyTransferModeType(
    stream: &mut ExiBitstream,
    mut SupportedEnergyTransferModeType: *const iso2_SupportedEnergyTransferModeType,
) -> i32 {
    let mut grammar_id: i32 = 106 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut EnergyTransferMode_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            106 => {
                if (EnergyTransferMode_currentIndex as i32)
                    < (*SupportedEnergyTransferModeType)
                        .EnergyTransferMode
                        .arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            let fresh18 = EnergyTransferMode_currentIndex;
                            EnergyTransferMode_currentIndex =
                                EnergyTransferMode_currentIndex.wrapping_add(1);
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                3 as i32 as usize,
                                (*SupportedEnergyTransferModeType).EnergyTransferMode.array
                                    [fresh18 as usize] as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 107 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            107 => {
                if (EnergyTransferMode_currentIndex as i32)
                    < (*SupportedEnergyTransferModeType)
                        .EnergyTransferMode
                        .arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            let fresh19 = EnergyTransferMode_currentIndex;
                            EnergyTransferMode_currentIndex =
                                EnergyTransferMode_currentIndex.wrapping_add(1);
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                3 as i32 as usize,
                                (*SupportedEnergyTransferModeType).EnergyTransferMode.array
                                    [fresh19 as usize] as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 107 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_CertificateChainType(
    stream: &mut ExiBitstream,
    mut CertificateChainType: *const iso2_CertificateChainType,
) -> i32 {
    let mut grammar_id: i32 = 108 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            108 => {
                if (*CertificateChainType).Id_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*CertificateChainType).Id.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*CertificateChainType).Id.charactersLen as usize,
                                ((*CertificateChainType).Id.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 109 as i32;
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*CertificateChainType).Certificate.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*CertificateChainType).Certificate.bytesLen as usize,
                                    ((*CertificateChainType).Certificate.bytes).as_ptr(),
                                    800 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 110 as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            109 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*CertificateChainType).Certificate.bytesLen,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*CertificateChainType).Certificate.bytesLen as usize,
                                ((*CertificateChainType).Certificate.bytes).as_ptr(),
                                800 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 110 as i32;
                                }
                            }
                        }
                    }
                }
            }
            110 => {
                if (*CertificateChainType).SubCertificates_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_SubCertificatesType(
                            stream,
                            &(*CertificateChainType).SubCertificates,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_BodyBaseType(
    stream: &mut ExiBitstream,
    mut BodyBaseType: *const iso2_BodyBaseType,
) -> i32 {
    let mut error: i32 =
        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
    return error;
}
fn encode_iso2_NotificationType(
    stream: &mut ExiBitstream,
    mut NotificationType: *const iso2_NotificationType,
) -> i32 {
    let mut grammar_id: i32 = 111 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            111 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*NotificationType).FaultCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 112 as i32;
                            }
                        }
                    }
                }
            }
            112 => {
                if (*NotificationType).FaultMsg_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                ((*NotificationType).FaultMsg.charactersLen as i32 + 2 as i32)
                                    as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*NotificationType).FaultMsg.charactersLen as usize,
                                    ((*NotificationType).FaultMsg.characters).as_ptr(),
                                    (64 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_DC_EVSEStatusType(
    stream: &mut ExiBitstream,
    mut DC_EVSEStatusType: *const iso2_DC_EVSEStatusType,
) -> i32 {
    let mut grammar_id: i32 = 113 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            113 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*DC_EVSEStatusType).NotificationMaxDelay,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 114 as i32;
                            }
                        }
                    }
                }
            }
            114 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*DC_EVSEStatusType).EVSENotification as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 115 as i32;
                            }
                        }
                    }
                }
            }
            115 => {
                if (*DC_EVSEStatusType).EVSEIsolationStatus_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                3 as i32 as usize,
                                (*DC_EVSEStatusType).EVSEIsolationStatus as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 116 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                4 as i32 as usize,
                                (*DC_EVSEStatusType).EVSEStatusCode as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                }
            }
            116 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            4 as i32 as usize,
                            (*DC_EVSEStatusType).EVSEStatusCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_SelectedServiceListType(
    stream: &mut ExiBitstream,
    mut SelectedServiceListType: *const iso2_SelectedServiceListType,
) -> i32 {
    let mut grammar_id: i32 = 117 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut SelectedService_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            117 => {
                if (SelectedService_currentIndex as i32)
                    < (*SelectedServiceListType).SelectedService.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh20 = SelectedService_currentIndex;
                        SelectedService_currentIndex = SelectedService_currentIndex.wrapping_add(1);
                        error = encode_iso2_SelectedServiceType(
                            stream,
                            &*((*SelectedServiceListType).SelectedService.array)
                                .as_ptr()
                                .offset(fresh20 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 118 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            118 => {
                if (SelectedService_currentIndex as i32)
                    < (*SelectedServiceListType).SelectedService.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh21 = SelectedService_currentIndex;
                        SelectedService_currentIndex = SelectedService_currentIndex.wrapping_add(1);
                        error = encode_iso2_SelectedServiceType(
                            stream,
                            &*((*SelectedServiceListType).SelectedService.array)
                                .as_ptr()
                                .offset(fresh21 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 118 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_PaymentOptionListType(
    stream: &mut ExiBitstream,
    mut PaymentOptionListType: *const iso2_PaymentOptionListType,
) -> i32 {
    let mut grammar_id: i32 = 119 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut PaymentOption_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            119 => {
                if (PaymentOption_currentIndex as i32)
                    < (*PaymentOptionListType).PaymentOption.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            let fresh22 = PaymentOption_currentIndex;
                            PaymentOption_currentIndex = PaymentOption_currentIndex.wrapping_add(1);
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                (*PaymentOptionListType).PaymentOption.array[fresh22 as usize]
                                    as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 120 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            120 => {
                if (PaymentOption_currentIndex as i32)
                    < (*PaymentOptionListType).PaymentOption.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            let fresh23 = PaymentOption_currentIndex;
                            PaymentOption_currentIndex = PaymentOption_currentIndex.wrapping_add(1);
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                (*PaymentOptionListType).PaymentOption.array[fresh23 as usize]
                                    as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_SignatureType(
    stream: &mut ExiBitstream,
    mut SignatureType: *const iso2_SignatureType,
) -> i32 {
    let mut grammar_id: i32 = 121 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            121 => {
                if (*SignatureType).Id_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*SignatureType).Id.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*SignatureType).Id.charactersLen as usize,
                                ((*SignatureType).Id.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 122 as i32;
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_SignedInfoType(stream, &(*SignatureType).SignedInfo);
                        if error == 0 as i32 {
                            grammar_id = 123 as i32;
                        }
                    }
                }
            }
            122 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_SignedInfoType(stream, &(*SignatureType).SignedInfo);
                    if error == 0 as i32 {
                        grammar_id = 123 as i32;
                    }
                }
            }
            123 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso2_SignatureValueType(stream, &(*SignatureType).SignatureValue);
                    if error == 0 as i32 {
                        grammar_id = 124 as i32;
                    }
                }
            }
            124 => {
                if (*SignatureType).KeyInfo_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_KeyInfoType(stream, &(*SignatureType).KeyInfo);
                        if error == 0 as i32 {
                            grammar_id = 126 as i32;
                        }
                    }
                } else if (*SignatureType).Object_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_ObjectType(stream, &(*SignatureType).Object);
                        if error == 0 as i32 {
                            grammar_id = 125 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            125 => {
                if 1 as i32 == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_ObjectType(stream, &(*SignatureType).Object);
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            126 => {
                if (*SignatureType).Object_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_ObjectType(stream, &(*SignatureType).Object);
                        if error == 0 as i32 {
                            grammar_id = 127 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            127 => {
                if 1 as i32 == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_ObjectType(stream, &(*SignatureType).Object);
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_ChargingProfileType(
    stream: &mut ExiBitstream,
    mut ChargingProfileType: *const iso2_ChargingProfileType,
) -> i32 {
    let mut grammar_id: i32 = 128 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut ProfileEntry_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            128 => {
                if (ProfileEntry_currentIndex as i32)
                    < (*ChargingProfileType).ProfileEntry.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh24 = ProfileEntry_currentIndex;
                        ProfileEntry_currentIndex = ProfileEntry_currentIndex.wrapping_add(1);
                        error = encode_iso2_ProfileEntryType(
                            stream,
                            &*((*ChargingProfileType).ProfileEntry.array)
                                .as_ptr()
                                .offset(fresh24 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 129 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            129 => {
                if (ProfileEntry_currentIndex as i32)
                    < (*ChargingProfileType).ProfileEntry.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh25 = ProfileEntry_currentIndex;
                        ProfileEntry_currentIndex = ProfileEntry_currentIndex.wrapping_add(1);
                        error = encode_iso2_ProfileEntryType(
                            stream,
                            &*((*ChargingProfileType).ProfileEntry.array)
                                .as_ptr()
                                .offset(fresh25 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 129 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_ServiceParameterListType(
    stream: &mut ExiBitstream,
    mut ServiceParameterListType: *const iso2_ServiceParameterListType,
) -> i32 {
    let mut grammar_id: i32 = 130 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut ParameterSet_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            130 => {
                if (ParameterSet_currentIndex as i32)
                    < (*ServiceParameterListType).ParameterSet.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh26 = ParameterSet_currentIndex;
                        ParameterSet_currentIndex = ParameterSet_currentIndex.wrapping_add(1);
                        error = encode_iso2_ParameterSetType(
                            stream,
                            &*((*ServiceParameterListType).ParameterSet.array)
                                .as_ptr()
                                .offset(fresh26 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 131 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            131 => {
                if (ParameterSet_currentIndex as i32)
                    < (*ServiceParameterListType).ParameterSet.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh27 = ParameterSet_currentIndex;
                        ParameterSet_currentIndex = ParameterSet_currentIndex.wrapping_add(1);
                        error = encode_iso2_ParameterSetType(
                            stream,
                            &*((*ServiceParameterListType).ParameterSet.array)
                                .as_ptr()
                                .offset(fresh27 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 131 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_ListOfRootCertificateIDsType(
    stream: &mut ExiBitstream,
    mut ListOfRootCertificateIDsType: *const iso2_ListOfRootCertificateIDsType,
) -> i32 {
    let mut grammar_id: i32 = 132 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut RootCertificateID_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            132 => {
                if (RootCertificateID_currentIndex as i32)
                    < (*ListOfRootCertificateIDsType).RootCertificateID.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh28 = RootCertificateID_currentIndex;
                        RootCertificateID_currentIndex =
                            RootCertificateID_currentIndex.wrapping_add(1);
                        error = encode_iso2_X509IssuerSerialType(
                            stream,
                            &*((*ListOfRootCertificateIDsType).RootCertificateID.array)
                                .as_ptr()
                                .offset(fresh28 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 133 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            133 => {
                if (RootCertificateID_currentIndex as i32)
                    < (*ListOfRootCertificateIDsType).RootCertificateID.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh29 = RootCertificateID_currentIndex;
                        RootCertificateID_currentIndex =
                            RootCertificateID_currentIndex.wrapping_add(1);
                        error = encode_iso2_X509IssuerSerialType(
                            stream,
                            &*((*ListOfRootCertificateIDsType).RootCertificateID.array)
                                .as_ptr()
                                .offset(fresh29 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 133 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_AC_EVChargeParameterType(
    stream: &mut ExiBitstream,
    mut AC_EVChargeParameterType: *const iso2_AC_EVChargeParameterType,
) -> i32 {
    let mut grammar_id: i32 = 134 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            134 => {
                if (*AC_EVChargeParameterType).DepartureTime_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_32(
                                stream,
                                (*AC_EVChargeParameterType).DepartureTime,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 135 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PhysicalValueType(
                            stream,
                            &(*AC_EVChargeParameterType).EAmount,
                        );
                        if error == 0 as i32 {
                            grammar_id = 136 as i32;
                        }
                    }
                }
            }
            135 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso2_PhysicalValueType(stream, &(*AC_EVChargeParameterType).EAmount);
                    if error == 0 as i32 {
                        grammar_id = 136 as i32;
                    }
                }
            }
            136 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_PhysicalValueType(
                        stream,
                        &(*AC_EVChargeParameterType).EVMaxVoltage,
                    );
                    if error == 0 as i32 {
                        grammar_id = 137 as i32;
                    }
                }
            }
            137 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_PhysicalValueType(
                        stream,
                        &(*AC_EVChargeParameterType).EVMaxCurrent,
                    );
                    if error == 0 as i32 {
                        grammar_id = 138 as i32;
                    }
                }
            }
            138 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_PhysicalValueType(
                        stream,
                        &(*AC_EVChargeParameterType).EVMinCurrent,
                    );
                    if error == 0 as i32 {
                        grammar_id = 3 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_DC_EVChargeParameterType(
    stream: &mut ExiBitstream,
    mut DC_EVChargeParameterType: *const iso2_DC_EVChargeParameterType,
) -> i32 {
    let mut grammar_id: i32 = 139 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            139 => {
                if (*DC_EVChargeParameterType).DepartureTime_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_32(
                                stream,
                                (*DC_EVChargeParameterType).DepartureTime,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 140 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_DC_EVStatusType(
                            stream,
                            &(*DC_EVChargeParameterType).DC_EVStatus,
                        );
                        if error == 0 as i32 {
                            grammar_id = 141 as i32;
                        }
                    }
                }
            }
            140 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_DC_EVStatusType(
                        stream,
                        &(*DC_EVChargeParameterType).DC_EVStatus,
                    );
                    if error == 0 as i32 {
                        grammar_id = 141 as i32;
                    }
                }
            }
            141 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_PhysicalValueType(
                        stream,
                        &(*DC_EVChargeParameterType).EVMaximumCurrentLimit,
                    );
                    if error == 0 as i32 {
                        grammar_id = 142 as i32;
                    }
                }
            }
            142 => {
                if (*DC_EVChargeParameterType).EVMaximumPowerLimit_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PhysicalValueType(
                            stream,
                            &(*DC_EVChargeParameterType).EVMaximumPowerLimit,
                        );
                        if error == 0 as i32 {
                            grammar_id = 143 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PhysicalValueType(
                            stream,
                            &(*DC_EVChargeParameterType).EVMaximumVoltageLimit,
                        );
                        if error == 0 as i32 {
                            grammar_id = 144 as i32;
                        }
                    }
                }
            }
            143 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_PhysicalValueType(
                        stream,
                        &(*DC_EVChargeParameterType).EVMaximumVoltageLimit,
                    );
                    if error == 0 as i32 {
                        grammar_id = 144 as i32;
                    }
                }
            }
            144 => {
                if (*DC_EVChargeParameterType).EVEnergyCapacity_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PhysicalValueType(
                            stream,
                            &(*DC_EVChargeParameterType).EVEnergyCapacity,
                        );
                        if error == 0 as i32 {
                            grammar_id = 145 as i32;
                        }
                    }
                } else if (*DC_EVChargeParameterType).EVEnergyRequest_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PhysicalValueType(
                            stream,
                            &(*DC_EVChargeParameterType).EVEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 146 as i32;
                        }
                    }
                } else if (*DC_EVChargeParameterType).FullSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                7 as i32 as usize,
                                (*DC_EVChargeParameterType).FullSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 147 as i32;
                                }
                            }
                        }
                    }
                } else if (*DC_EVChargeParameterType).BulkSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                7 as i32 as usize,
                                (*DC_EVChargeParameterType).BulkSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            145 => {
                if (*DC_EVChargeParameterType).EVEnergyRequest_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PhysicalValueType(
                            stream,
                            &(*DC_EVChargeParameterType).EVEnergyRequest,
                        );
                        if error == 0 as i32 {
                            grammar_id = 146 as i32;
                        }
                    }
                } else if (*DC_EVChargeParameterType).FullSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                7 as i32 as usize,
                                (*DC_EVChargeParameterType).FullSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 147 as i32;
                                }
                            }
                        }
                    }
                } else if (*DC_EVChargeParameterType).BulkSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                7 as i32 as usize,
                                (*DC_EVChargeParameterType).BulkSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            146 => {
                if (*DC_EVChargeParameterType).FullSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                7 as i32 as usize,
                                (*DC_EVChargeParameterType).FullSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 147 as i32;
                                }
                            }
                        }
                    }
                } else if (*DC_EVChargeParameterType).BulkSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                7 as i32 as usize,
                                (*DC_EVChargeParameterType).BulkSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            147 => {
                if (*DC_EVChargeParameterType).BulkSOC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                7 as i32 as usize,
                                (*DC_EVChargeParameterType).BulkSOC as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_EVChargeParameterType(
    stream: &mut ExiBitstream,
    mut EVChargeParameterType: *const iso2_EVChargeParameterType,
) -> i32 {
    let mut grammar_id: i32 = 148 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            148 => {
                if (*EVChargeParameterType).DepartureTime_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_32(
                                stream,
                                (*EVChargeParameterType).DepartureTime,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 149 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_AC_EVChargeParameterType(
                            stream,
                            &(*EVChargeParameterType).AC_EVChargeParameter,
                        );
                        if error == 0 as i32 {
                            grammar_id = 150 as i32;
                        }
                    }
                }
            }
            149 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_AC_EVChargeParameterType(
                        stream,
                        &(*EVChargeParameterType).AC_EVChargeParameter,
                    );
                    if error == 0 as i32 {
                        grammar_id = 150 as i32;
                    }
                }
            }
            150 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_DC_EVChargeParameterType(
                        stream,
                        &(*EVChargeParameterType).DC_EVChargeParameter,
                    );
                    if error == 0 as i32 {
                        grammar_id = 3 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_SASchedulesType(
    stream: &mut ExiBitstream,
    mut SASchedulesType: *const iso2_SASchedulesType,
) -> i32 {
    let mut error: i32 =
        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
    return error;
}
fn encode_iso2_SAScheduleListType(
    stream: &mut ExiBitstream,
    mut SAScheduleListType: *const iso2_SAScheduleListType,
) -> i32 {
    let mut grammar_id: i32 = 151 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut SAScheduleTuple_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            151 => {
                if (SAScheduleTuple_currentIndex as i32)
                    < (*SAScheduleListType).SAScheduleTuple.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh30 = SAScheduleTuple_currentIndex;
                        SAScheduleTuple_currentIndex = SAScheduleTuple_currentIndex.wrapping_add(1);
                        error = encode_iso2_SAScheduleTupleType(
                            stream,
                            &*((*SAScheduleListType).SAScheduleTuple.array)
                                .as_ptr()
                                .offset(fresh30 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 152 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            152 => {
                if (SAScheduleTuple_currentIndex as i32)
                    < (*SAScheduleListType).SAScheduleTuple.arrayLen as i32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh31 = SAScheduleTuple_currentIndex;
                        SAScheduleTuple_currentIndex = SAScheduleTuple_currentIndex.wrapping_add(1);
                        error = encode_iso2_SAScheduleTupleType(
                            stream,
                            &*((*SAScheduleListType).SAScheduleTuple.array)
                                .as_ptr()
                                .offset(fresh31 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 152 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_ChargeServiceType(
    stream: &mut ExiBitstream,
    mut ChargeServiceType: *const iso2_ChargeServiceType,
) -> i32 {
    let mut grammar_id: i32 = 153 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            153 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            exi_basetypes_encoder_uint_16(stream, (*ChargeServiceType).ServiceID);
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 154 as i32;
                            }
                        }
                    }
                }
            }
            154 => {
                if (*ChargeServiceType).ServiceName_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                ((*ChargeServiceType).ServiceName.charactersLen as i32 + 2 as i32)
                                    as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*ChargeServiceType).ServiceName.charactersLen as usize,
                                    ((*ChargeServiceType).ServiceName.characters).as_ptr(),
                                    (32 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 155 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                2 as i32 as usize,
                                (*ChargeServiceType).ServiceCategory as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 156 as i32;
                                }
                            }
                        }
                    }
                }
            }
            155 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*ChargeServiceType).ServiceCategory as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 156 as i32;
                            }
                        }
                    }
                }
            }
            156 => {
                if (*ChargeServiceType).ServiceScope_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                ((*ChargeServiceType).ServiceScope.charactersLen as i32 + 2 as i32)
                                    as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*ChargeServiceType).ServiceScope.charactersLen as usize,
                                    ((*ChargeServiceType).ServiceScope.characters).as_ptr(),
                                    (64 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 157 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*ChargeServiceType).FreeService,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 158 as i32;
                                }
                            }
                        }
                    }
                }
            }
            157 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            exi_basetypes_encoder_bool(stream, (*ChargeServiceType).FreeService);
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 158 as i32;
                            }
                        }
                    }
                }
            }
            158 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_SupportedEnergyTransferModeType(
                        stream,
                        &(*ChargeServiceType).SupportedEnergyTransferMode,
                    );
                    if error == 0 as i32 {
                        grammar_id = 3 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_EVPowerDeliveryParameterType(
    stream: &mut ExiBitstream,
    mut EVPowerDeliveryParameterType: *const iso2_EVPowerDeliveryParameterType,
) -> i32 {
    let mut error: i32 =
        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
    return error;
}
fn encode_iso2_DC_EVPowerDeliveryParameterType(
    stream: &mut ExiBitstream,
    mut DC_EVPowerDeliveryParameterType: *const iso2_DC_EVPowerDeliveryParameterType,
) -> i32 {
    let mut grammar_id: i32 = 159 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            159 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_DC_EVStatusType(
                        stream,
                        &(*DC_EVPowerDeliveryParameterType).DC_EVStatus,
                    );
                    if error == 0 as i32 {
                        grammar_id = 160 as i32;
                    }
                }
            }
            160 => {
                if (*DC_EVPowerDeliveryParameterType).BulkChargingComplete_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*DC_EVPowerDeliveryParameterType).BulkChargingComplete,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 161 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*DC_EVPowerDeliveryParameterType).ChargingComplete,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                }
            }
            161 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(
                            stream,
                            (*DC_EVPowerDeliveryParameterType).ChargingComplete,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_ContractSignatureEncryptedPrivateKeyType(
    stream: &mut ExiBitstream,
    mut ContractSignatureEncryptedPrivateKeyType: *const iso2_ContractSignatureEncryptedPrivateKeyType,
) -> i32 {
    let mut grammar_id: i32 = 162 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            162 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = exi_basetypes_encoder_uint_16(
                        stream,
                        ((*ContractSignatureEncryptedPrivateKeyType).Id.charactersLen as i32
                            + 2 as i32) as u16,
                    );
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_characters(
                            stream,
                            (*ContractSignatureEncryptedPrivateKeyType).Id.charactersLen as usize,
                            ((*ContractSignatureEncryptedPrivateKeyType).Id.characters).as_ptr(),
                            (64 as i32 + 1 as i32) as usize,
                        );
                        if error == 0 as i32 {
                            grammar_id = 163 as i32;
                        }
                    }
                }
            }
            163 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = exi_basetypes_encoder_uint_16(
                        stream,
                        (*ContractSignatureEncryptedPrivateKeyType).CONTENT.bytesLen,
                    );
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bytes(
                            stream,
                            (*ContractSignatureEncryptedPrivateKeyType).CONTENT.bytesLen as usize,
                            ((*ContractSignatureEncryptedPrivateKeyType).CONTENT.bytes).as_ptr(),
                            350 as i32 as usize,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_EVSEChargeParameterType(
    stream: &mut ExiBitstream,
    mut EVSEChargeParameterType: *const iso2_EVSEChargeParameterType,
) -> i32 {
    let mut error: i32 =
        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
    return error;
}
fn encode_iso2_DC_EVSEChargeParameterType(
    stream: &mut ExiBitstream,
    mut DC_EVSEChargeParameterType: *const iso2_DC_EVSEChargeParameterType,
) -> i32 {
    let mut grammar_id: i32 = 164 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            164 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_DC_EVSEStatusType(
                        stream,
                        &(*DC_EVSEChargeParameterType).DC_EVSEStatus,
                    );
                    if error == 0 as i32 {
                        grammar_id = 165 as i32;
                    }
                }
            }
            165 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_PhysicalValueType(
                        stream,
                        &(*DC_EVSEChargeParameterType).EVSEMaximumCurrentLimit,
                    );
                    if error == 0 as i32 {
                        grammar_id = 166 as i32;
                    }
                }
            }
            166 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_PhysicalValueType(
                        stream,
                        &(*DC_EVSEChargeParameterType).EVSEMaximumPowerLimit,
                    );
                    if error == 0 as i32 {
                        grammar_id = 167 as i32;
                    }
                }
            }
            167 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_PhysicalValueType(
                        stream,
                        &(*DC_EVSEChargeParameterType).EVSEMaximumVoltageLimit,
                    );
                    if error == 0 as i32 {
                        grammar_id = 168 as i32;
                    }
                }
            }
            168 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_PhysicalValueType(
                        stream,
                        &(*DC_EVSEChargeParameterType).EVSEMinimumCurrentLimit,
                    );
                    if error == 0 as i32 {
                        grammar_id = 169 as i32;
                    }
                }
            }
            169 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_PhysicalValueType(
                        stream,
                        &(*DC_EVSEChargeParameterType).EVSEMinimumVoltageLimit,
                    );
                    if error == 0 as i32 {
                        grammar_id = 170 as i32;
                    }
                }
            }
            170 => {
                if (*DC_EVSEChargeParameterType).EVSECurrentRegulationTolerance_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PhysicalValueType(
                            stream,
                            &(*DC_EVSEChargeParameterType).EVSECurrentRegulationTolerance,
                        );
                        if error == 0 as i32 {
                            grammar_id = 171 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PhysicalValueType(
                            stream,
                            &(*DC_EVSEChargeParameterType).EVSEPeakCurrentRipple,
                        );
                        if error == 0 as i32 {
                            grammar_id = 172 as i32;
                        }
                    }
                }
            }
            171 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_PhysicalValueType(
                        stream,
                        &(*DC_EVSEChargeParameterType).EVSEPeakCurrentRipple,
                    );
                    if error == 0 as i32 {
                        grammar_id = 172 as i32;
                    }
                }
            }
            172 => {
                if (*DC_EVSEChargeParameterType).EVSEEnergyToBeDelivered_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PhysicalValueType(
                            stream,
                            &(*DC_EVSEChargeParameterType).EVSEEnergyToBeDelivered,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_ServiceListType(
    stream: &mut ExiBitstream,
    mut ServiceListType: *const iso2_ServiceListType,
) -> i32 {
    let mut grammar_id: i32 = 173 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    let mut Service_currentIndex: u16 = 0 as i32 as u16;
    while done == 0 {
        match grammar_id {
            173 => {
                if (Service_currentIndex as i32) < (*ServiceListType).Service.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh32 = Service_currentIndex;
                        Service_currentIndex = Service_currentIndex.wrapping_add(1);
                        error = encode_iso2_ServiceType(
                            stream,
                            &*((*ServiceListType).Service.array)
                                .as_ptr()
                                .offset(fresh32 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 174 as i32;
                        }
                    }
                } else {
                    error = -(150 as i32);
                }
            }
            174 => {
                if (Service_currentIndex as i32) < (*ServiceListType).Service.arrayLen as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        let fresh33 = Service_currentIndex;
                        Service_currentIndex = Service_currentIndex.wrapping_add(1);
                        error = encode_iso2_ServiceType(
                            stream,
                            &*((*ServiceListType).Service.array)
                                .as_ptr()
                                .offset(fresh33 as isize),
                        );
                        if error == 0 as i32 {
                            grammar_id = 174 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_DiffieHellmanPublickeyType(
    stream: &mut ExiBitstream,
    mut DiffieHellmanPublickeyType: *const iso2_DiffieHellmanPublickeyType,
) -> i32 {
    let mut grammar_id: i32 = 175 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            175 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = exi_basetypes_encoder_uint_16(
                        stream,
                        ((*DiffieHellmanPublickeyType).Id.charactersLen as i32 + 2 as i32) as u16,
                    );
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_characters(
                            stream,
                            (*DiffieHellmanPublickeyType).Id.charactersLen as usize,
                            ((*DiffieHellmanPublickeyType).Id.characters).as_ptr(),
                            (64 as i32 + 1 as i32) as usize,
                        );
                        if error == 0 as i32 {
                            grammar_id = 176 as i32;
                        }
                    }
                }
            }
            176 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = exi_basetypes_encoder_uint_16(
                        stream,
                        (*DiffieHellmanPublickeyType).CONTENT.bytesLen,
                    );
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bytes(
                            stream,
                            (*DiffieHellmanPublickeyType).CONTENT.bytesLen as usize,
                            ((*DiffieHellmanPublickeyType).CONTENT.bytes).as_ptr(),
                            350 as i32 as usize,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_EMAIDType(
    stream: &mut ExiBitstream,
    mut EMAIDType: *const iso2_EMAIDType,
) -> i32 {
    let mut grammar_id: i32 = 177 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            177 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = exi_basetypes_encoder_uint_16(
                        stream,
                        ((*EMAIDType).Id.charactersLen as i32 + 2 as i32) as u16,
                    );
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_characters(
                            stream,
                            (*EMAIDType).Id.charactersLen as usize,
                            ((*EMAIDType).Id.characters).as_ptr(),
                            (64 as i32 + 1 as i32) as usize,
                        );
                        if error == 0 as i32 {
                            grammar_id = 178 as i32;
                        }
                    }
                }
            }
            178 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = exi_basetypes_encoder_uint_16(
                        stream,
                        ((*EMAIDType).CONTENT.charactersLen as i32 + 2 as i32) as u16,
                    );
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_characters(
                            stream,
                            (*EMAIDType).CONTENT.charactersLen as usize,
                            ((*EMAIDType).CONTENT.characters).as_ptr(),
                            (64 as i32 + 1 as i32) as usize,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_AC_EVSEStatusType(
    stream: &mut ExiBitstream,
    mut AC_EVSEStatusType: *const iso2_AC_EVSEStatusType,
) -> i32 {
    let mut grammar_id: i32 = 179 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            179 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*AC_EVSEStatusType).NotificationMaxDelay,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 180 as i32;
                            }
                        }
                    }
                }
            }
            180 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*AC_EVSEStatusType).EVSENotification as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 181 as i32;
                            }
                        }
                    }
                }
            }
            181 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(stream, (*AC_EVSEStatusType).RCD);
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_EVSEStatusType(
    stream: &mut ExiBitstream,
    mut EVSEStatusType: *const iso2_EVSEStatusType,
) -> i32 {
    let mut grammar_id: i32 = 182 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            182 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*EVSEStatusType).NotificationMaxDelay,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 183 as i32;
                            }
                        }
                    }
                }
            }
            183 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*EVSEStatusType).EVSENotification as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 184 as i32;
                            }
                        }
                    }
                }
            }
            184 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_AC_EVSEStatusType(stream, &(*EVSEStatusType).AC_EVSEStatus);
                    if error == 0 as i32 {
                        grammar_id = 185 as i32;
                    }
                }
            }
            185 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_DC_EVSEStatusType(stream, &(*EVSEStatusType).DC_EVSEStatus);
                    if error == 0 as i32 {
                        grammar_id = 3 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_AC_EVSEChargeParameterType(
    stream: &mut ExiBitstream,
    mut AC_EVSEChargeParameterType: *const iso2_AC_EVSEChargeParameterType,
) -> i32 {
    let mut grammar_id: i32 = 186 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            186 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_AC_EVSEStatusType(
                        stream,
                        &(*AC_EVSEChargeParameterType).AC_EVSEStatus,
                    );
                    if error == 0 as i32 {
                        grammar_id = 187 as i32;
                    }
                }
            }
            187 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_PhysicalValueType(
                        stream,
                        &(*AC_EVSEChargeParameterType).EVSENominalVoltage,
                    );
                    if error == 0 as i32 {
                        grammar_id = 188 as i32;
                    }
                }
            }
            188 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_PhysicalValueType(
                        stream,
                        &(*AC_EVSEChargeParameterType).EVSEMaxCurrent,
                    );
                    if error == 0 as i32 {
                        grammar_id = 3 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_MeterInfoType(
    stream: &mut ExiBitstream,
    mut MeterInfoType: *const iso2_MeterInfoType,
) -> i32 {
    let mut grammar_id: i32 = 189 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            189 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*MeterInfoType).MeterID.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*MeterInfoType).MeterID.charactersLen as usize,
                                ((*MeterInfoType).MeterID.characters).as_ptr(),
                                (32 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 190 as i32;
                                }
                            }
                        }
                    }
                }
            }
            190 => {
                if (*MeterInfoType).MeterReading_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_64(
                                stream,
                                (*MeterInfoType).MeterReading,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 191 as i32;
                                }
                            }
                        }
                    }
                } else if (*MeterInfoType).SigMeterReading_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*MeterInfoType).SigMeterReading.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*MeterInfoType).SigMeterReading.bytesLen as usize,
                                    ((*MeterInfoType).SigMeterReading.bytes).as_ptr(),
                                    64 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 192 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*MeterInfoType).MeterStatus_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_integer_16(
                                stream,
                                (*MeterInfoType).MeterStatus,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 193 as i32;
                                }
                            }
                        }
                    }
                } else if (*MeterInfoType).TMeter_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error =
                                exi_basetypes_encoder_integer_64(stream, (*MeterInfoType).TMeter);
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            191 => {
                if (*MeterInfoType).SigMeterReading_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*MeterInfoType).SigMeterReading.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*MeterInfoType).SigMeterReading.bytesLen as usize,
                                    ((*MeterInfoType).SigMeterReading.bytes).as_ptr(),
                                    64 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 192 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*MeterInfoType).MeterStatus_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_integer_16(
                                stream,
                                (*MeterInfoType).MeterStatus,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 193 as i32;
                                }
                            }
                        }
                    }
                } else if (*MeterInfoType).TMeter_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error =
                                exi_basetypes_encoder_integer_64(stream, (*MeterInfoType).TMeter);
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            192 => {
                if (*MeterInfoType).MeterStatus_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_integer_16(
                                stream,
                                (*MeterInfoType).MeterStatus,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 193 as i32;
                                }
                            }
                        }
                    }
                } else if (*MeterInfoType).TMeter_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error =
                                exi_basetypes_encoder_integer_64(stream, (*MeterInfoType).TMeter);
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            193 => {
                if (*MeterInfoType).TMeter_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error =
                                exi_basetypes_encoder_integer_64(stream, (*MeterInfoType).TMeter);
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_MessageHeaderType(
    stream: &mut ExiBitstream,
    mut MessageHeaderType: *const iso2_MessageHeaderType,
) -> i32 {
    let mut grammar_id: i32 = 194 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            194 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*MessageHeaderType).SessionID.bytesLen,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*MessageHeaderType).SessionID.bytesLen as usize,
                                ((*MessageHeaderType).SessionID.bytes).as_ptr(),
                                8 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 195 as i32;
                                }
                            }
                        }
                    }
                }
            }
            195 => {
                if (*MessageHeaderType).Notification_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_NotificationType(
                            stream,
                            &(*MessageHeaderType).Notification,
                        );
                        if error == 0 as i32 {
                            grammar_id = 196 as i32;
                        }
                    }
                } else if (*MessageHeaderType).Signature_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_SignatureType(stream, &(*MessageHeaderType).Signature);
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            196 => {
                if (*MessageHeaderType).Signature_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_SignatureType(stream, &(*MessageHeaderType).Signature);
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_PowerDeliveryReqType(
    stream: &mut ExiBitstream,
    mut PowerDeliveryReqType: *const iso2_PowerDeliveryReqType,
) -> i32 {
    let mut grammar_id: i32 = 197 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            197 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*PowerDeliveryReqType).ChargeProgress as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 198 as i32;
                            }
                        }
                    }
                }
            }
            198 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            8 as i32 as usize,
                            ((*PowerDeliveryReqType).SAScheduleTupleID as u32)
                                .wrapping_sub(1 as i32 as u32),
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 199 as i32;
                            }
                        }
                    }
                }
            }
            199 => {
                if (*PowerDeliveryReqType).ChargingProfile_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_ChargingProfileType(
                            stream,
                            &(*PowerDeliveryReqType).ChargingProfile,
                        );
                        if error == 0 as i32 {
                            grammar_id = 200 as i32;
                        }
                    }
                } else if (*PowerDeliveryReqType).DC_EVPowerDeliveryParameter_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_DC_EVPowerDeliveryParameterType(
                            stream,
                            &(*PowerDeliveryReqType).DC_EVPowerDeliveryParameter,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*PowerDeliveryReqType).EVPowerDeliveryParameter_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_EVPowerDeliveryParameterType(
                            stream,
                            &(*PowerDeliveryReqType).EVPowerDeliveryParameter,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            200 => {
                if (*PowerDeliveryReqType).DC_EVPowerDeliveryParameter_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_DC_EVPowerDeliveryParameterType(
                            stream,
                            &(*PowerDeliveryReqType).DC_EVPowerDeliveryParameter,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*PowerDeliveryReqType).EVPowerDeliveryParameter_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_EVPowerDeliveryParameterType(
                            stream,
                            &(*PowerDeliveryReqType).EVPowerDeliveryParameter,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_CurrentDemandResType(
    stream: &mut ExiBitstream,
    mut CurrentDemandResType: *const iso2_CurrentDemandResType,
) -> i32 {
    let mut grammar_id: i32 = 201 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            201 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            5 as i32 as usize,
                            (*CurrentDemandResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 202 as i32;
                            }
                        }
                    }
                }
            }
            202 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_DC_EVSEStatusType(
                        stream,
                        &(*CurrentDemandResType).DC_EVSEStatus,
                    );
                    if error == 0 as i32 {
                        grammar_id = 203 as i32;
                    }
                }
            }
            203 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_PhysicalValueType(
                        stream,
                        &(*CurrentDemandResType).EVSEPresentVoltage,
                    );
                    if error == 0 as i32 {
                        grammar_id = 204 as i32;
                    }
                }
            }
            204 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_PhysicalValueType(
                        stream,
                        &(*CurrentDemandResType).EVSEPresentCurrent,
                    );
                    if error == 0 as i32 {
                        grammar_id = 205 as i32;
                    }
                }
            }
            205 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(
                            stream,
                            (*CurrentDemandResType).EVSECurrentLimitAchieved,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 206 as i32;
                            }
                        }
                    }
                }
            }
            206 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(
                            stream,
                            (*CurrentDemandResType).EVSEVoltageLimitAchieved,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 207 as i32;
                            }
                        }
                    }
                }
            }
            207 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(
                            stream,
                            (*CurrentDemandResType).EVSEPowerLimitAchieved,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 208 as i32;
                            }
                        }
                    }
                }
            }
            208 => {
                if (*CurrentDemandResType).EVSEMaximumVoltageLimit_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PhysicalValueType(
                            stream,
                            &(*CurrentDemandResType).EVSEMaximumVoltageLimit,
                        );
                        if error == 0 as i32 {
                            grammar_id = 209 as i32;
                        }
                    }
                } else if (*CurrentDemandResType).EVSEMaximumCurrentLimit_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PhysicalValueType(
                            stream,
                            &(*CurrentDemandResType).EVSEMaximumCurrentLimit,
                        );
                        if error == 0 as i32 {
                            grammar_id = 210 as i32;
                        }
                    }
                } else if (*CurrentDemandResType).EVSEMaximumPowerLimit_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PhysicalValueType(
                            stream,
                            &(*CurrentDemandResType).EVSEMaximumPowerLimit,
                        );
                        if error == 0 as i32 {
                            grammar_id = 211 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                ((*CurrentDemandResType).EVSEID.charactersLen as i32 + 2 as i32)
                                    as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*CurrentDemandResType).EVSEID.charactersLen as usize,
                                    ((*CurrentDemandResType).EVSEID.characters).as_ptr(),
                                    (37 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 212 as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            209 => {
                if (*CurrentDemandResType).EVSEMaximumCurrentLimit_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PhysicalValueType(
                            stream,
                            &(*CurrentDemandResType).EVSEMaximumCurrentLimit,
                        );
                        if error == 0 as i32 {
                            grammar_id = 210 as i32;
                        }
                    }
                } else if (*CurrentDemandResType).EVSEMaximumPowerLimit_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PhysicalValueType(
                            stream,
                            &(*CurrentDemandResType).EVSEMaximumPowerLimit,
                        );
                        if error == 0 as i32 {
                            grammar_id = 211 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                ((*CurrentDemandResType).EVSEID.charactersLen as i32 + 2 as i32)
                                    as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*CurrentDemandResType).EVSEID.charactersLen as usize,
                                    ((*CurrentDemandResType).EVSEID.characters).as_ptr(),
                                    (37 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 212 as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            210 => {
                if (*CurrentDemandResType).EVSEMaximumPowerLimit_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PhysicalValueType(
                            stream,
                            &(*CurrentDemandResType).EVSEMaximumPowerLimit,
                        );
                        if error == 0 as i32 {
                            grammar_id = 211 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                ((*CurrentDemandResType).EVSEID.charactersLen as i32 + 2 as i32)
                                    as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*CurrentDemandResType).EVSEID.charactersLen as usize,
                                    ((*CurrentDemandResType).EVSEID.characters).as_ptr(),
                                    (37 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 212 as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            211 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*CurrentDemandResType).EVSEID.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*CurrentDemandResType).EVSEID.charactersLen as usize,
                                ((*CurrentDemandResType).EVSEID.characters).as_ptr(),
                                (37 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 212 as i32;
                                }
                            }
                        }
                    }
                }
            }
            212 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            8 as i32 as usize,
                            ((*CurrentDemandResType).SAScheduleTupleID as u32)
                                .wrapping_sub(1 as i32 as u32),
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 213 as i32;
                            }
                        }
                    }
                }
            }
            213 => {
                if (*CurrentDemandResType).MeterInfo_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso2_MeterInfoType(stream, &(*CurrentDemandResType).MeterInfo);
                        if error == 0 as i32 {
                            grammar_id = 214 as i32;
                        }
                    }
                } else if (*CurrentDemandResType).ReceiptRequired_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*CurrentDemandResType).ReceiptRequired,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            214 => {
                if (*CurrentDemandResType).ReceiptRequired_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*CurrentDemandResType).ReceiptRequired,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_ChargingStatusResType(
    stream: &mut ExiBitstream,
    mut ChargingStatusResType: *const iso2_ChargingStatusResType,
) -> i32 {
    let mut grammar_id: i32 = 215 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            215 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            5 as i32 as usize,
                            (*ChargingStatusResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 216 as i32;
                            }
                        }
                    }
                }
            }
            216 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*ChargingStatusResType).EVSEID.charactersLen as i32 + 2 as i32)
                                as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*ChargingStatusResType).EVSEID.charactersLen as usize,
                                ((*ChargingStatusResType).EVSEID.characters).as_ptr(),
                                (37 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 217 as i32;
                                }
                            }
                        }
                    }
                }
            }
            217 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            8 as i32 as usize,
                            ((*ChargingStatusResType).SAScheduleTupleID as u32)
                                .wrapping_sub(1 as i32 as u32),
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 218 as i32;
                            }
                        }
                    }
                }
            }
            218 => {
                if (*ChargingStatusResType).EVSEMaxCurrent_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PhysicalValueType(
                            stream,
                            &(*ChargingStatusResType).EVSEMaxCurrent,
                        );
                        if error == 0 as i32 {
                            grammar_id = 219 as i32;
                        }
                    }
                } else if (*ChargingStatusResType).MeterInfo_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso2_MeterInfoType(stream, &(*ChargingStatusResType).MeterInfo);
                        if error == 0 as i32 {
                            grammar_id = 220 as i32;
                        }
                    }
                } else if (*ChargingStatusResType).ReceiptRequired_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*ChargingStatusResType).ReceiptRequired,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 221 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_AC_EVSEStatusType(
                            stream,
                            &(*ChargingStatusResType).AC_EVSEStatus,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                }
            }
            219 => {
                if (*ChargingStatusResType).MeterInfo_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso2_MeterInfoType(stream, &(*ChargingStatusResType).MeterInfo);
                        if error == 0 as i32 {
                            grammar_id = 220 as i32;
                        }
                    }
                } else if (*ChargingStatusResType).ReceiptRequired_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*ChargingStatusResType).ReceiptRequired,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 221 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_AC_EVSEStatusType(
                            stream,
                            &(*ChargingStatusResType).AC_EVSEStatus,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                }
            }
            220 => {
                if (*ChargingStatusResType).ReceiptRequired_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*ChargingStatusResType).ReceiptRequired,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 221 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_AC_EVSEStatusType(
                            stream,
                            &(*ChargingStatusResType).AC_EVSEStatus,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                }
            }
            221 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_AC_EVSEStatusType(
                        stream,
                        &(*ChargingStatusResType).AC_EVSEStatus,
                    );
                    if error == 0 as i32 {
                        grammar_id = 3 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_AuthorizationReqType(
    stream: &mut ExiBitstream,
    mut AuthorizationReqType: *const iso2_AuthorizationReqType,
) -> i32 {
    let mut grammar_id: i32 = 222 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            222 => {
                if (*AuthorizationReqType).Id_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*AuthorizationReqType).Id.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*AuthorizationReqType).Id.charactersLen as usize,
                                ((*AuthorizationReqType).Id.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 223 as i32;
                            }
                        }
                    }
                } else if (*AuthorizationReqType).GenChallenge_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*AuthorizationReqType).GenChallenge.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*AuthorizationReqType).GenChallenge.bytesLen as usize,
                                    ((*AuthorizationReqType).GenChallenge.bytes).as_ptr(),
                                    16 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            223 => {
                if (*AuthorizationReqType).GenChallenge_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*AuthorizationReqType).GenChallenge.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*AuthorizationReqType).GenChallenge.bytesLen as usize,
                                    ((*AuthorizationReqType).GenChallenge.bytes).as_ptr(),
                                    16 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 3 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_PreChargeReqType(
    stream: &mut ExiBitstream,
    mut PreChargeReqType: *const iso2_PreChargeReqType,
) -> i32 {
    let mut grammar_id: i32 = 224 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            224 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_DC_EVStatusType(stream, &(*PreChargeReqType).DC_EVStatus);
                    if error == 0 as i32 {
                        grammar_id = 225 as i32;
                    }
                }
            }
            225 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso2_PhysicalValueType(stream, &(*PreChargeReqType).EVTargetVoltage);
                    if error == 0 as i32 {
                        grammar_id = 226 as i32;
                    }
                }
            }
            226 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso2_PhysicalValueType(stream, &(*PreChargeReqType).EVTargetCurrent);
                    if error == 0 as i32 {
                        grammar_id = 3 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_ServiceDetailResType(
    stream: &mut ExiBitstream,
    mut ServiceDetailResType: *const iso2_ServiceDetailResType,
) -> i32 {
    let mut grammar_id: i32 = 227 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            227 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            5 as i32 as usize,
                            (*ServiceDetailResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 228 as i32;
                            }
                        }
                    }
                }
            }
            228 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*ServiceDetailResType).ServiceID,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 229 as i32;
                            }
                        }
                    }
                }
            }
            229 => {
                if (*ServiceDetailResType).ServiceParameterList_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_ServiceParameterListType(
                            stream,
                            &(*ServiceDetailResType).ServiceParameterList,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_PaymentServiceSelectionResType(
    stream: &mut ExiBitstream,
    mut PaymentServiceSelectionResType: *const iso2_PaymentServiceSelectionResType,
) -> i32 {
    let mut grammar_id: i32 = 230 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            230 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            5 as i32 as usize,
                            (*PaymentServiceSelectionResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_CertificateUpdateReqType(
    stream: &mut ExiBitstream,
    mut CertificateUpdateReqType: *const iso2_CertificateUpdateReqType,
) -> i32 {
    let mut grammar_id: i32 = 231 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            231 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = exi_basetypes_encoder_uint_16(
                        stream,
                        ((*CertificateUpdateReqType).Id.charactersLen as i32 + 2 as i32) as u16,
                    );
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_characters(
                            stream,
                            (*CertificateUpdateReqType).Id.charactersLen as usize,
                            ((*CertificateUpdateReqType).Id.characters).as_ptr(),
                            (64 as i32 + 1 as i32) as usize,
                        );
                        if error == 0 as i32 {
                            grammar_id = 232 as i32;
                        }
                    }
                }
            }
            232 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_CertificateChainType(
                        stream,
                        &(*CertificateUpdateReqType).ContractSignatureCertChain,
                    );
                    if error == 0 as i32 {
                        grammar_id = 233 as i32;
                    }
                }
            }
            233 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*CertificateUpdateReqType).eMAID.charactersLen as i32 + 2 as i32)
                                as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*CertificateUpdateReqType).eMAID.charactersLen as usize,
                                ((*CertificateUpdateReqType).eMAID.characters).as_ptr(),
                                (15 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 234 as i32;
                                }
                            }
                        }
                    }
                }
            }
            234 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_ListOfRootCertificateIDsType(
                        stream,
                        &(*CertificateUpdateReqType).ListOfRootCertificateIDs,
                    );
                    if error == 0 as i32 {
                        grammar_id = 3 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_SessionSetupResType(
    stream: &mut ExiBitstream,
    mut SessionSetupResType: *const iso2_SessionSetupResType,
) -> i32 {
    let mut grammar_id: i32 = 235 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            235 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            5 as i32 as usize,
                            (*SessionSetupResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 236 as i32;
                            }
                        }
                    }
                }
            }
            236 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*SessionSetupResType).EVSEID.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*SessionSetupResType).EVSEID.charactersLen as usize,
                                ((*SessionSetupResType).EVSEID.characters).as_ptr(),
                                (37 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 237 as i32;
                                }
                            }
                        }
                    }
                }
            }
            237 => {
                if (*SessionSetupResType).EVSETimeStamp_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_integer_64(
                                stream,
                                (*SessionSetupResType).EVSETimeStamp,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_CertificateInstallationReqType(
    stream: &mut ExiBitstream,
    mut CertificateInstallationReqType: *const iso2_CertificateInstallationReqType,
) -> i32 {
    let mut grammar_id: i32 = 238 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            238 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = exi_basetypes_encoder_uint_16(
                        stream,
                        ((*CertificateInstallationReqType).Id.charactersLen as i32 + 2 as i32)
                            as u16,
                    );
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_characters(
                            stream,
                            (*CertificateInstallationReqType).Id.charactersLen as usize,
                            ((*CertificateInstallationReqType).Id.characters).as_ptr(),
                            (64 as i32 + 1 as i32) as usize,
                        );
                        if error == 0 as i32 {
                            grammar_id = 239 as i32;
                        }
                    }
                }
            }
            239 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*CertificateInstallationReqType)
                                .OEMProvisioningCert
                                .bytesLen,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*CertificateInstallationReqType)
                                    .OEMProvisioningCert
                                    .bytesLen as usize,
                                ((*CertificateInstallationReqType).OEMProvisioningCert.bytes)
                                    .as_ptr(),
                                800 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 240 as i32;
                                }
                            }
                        }
                    }
                }
            }
            240 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_ListOfRootCertificateIDsType(
                        stream,
                        &(*CertificateInstallationReqType).ListOfRootCertificateIDs,
                    );
                    if error == 0 as i32 {
                        grammar_id = 3 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_CertificateInstallationResType(
    stream: &mut ExiBitstream,
    mut CertificateInstallationResType: *const iso2_CertificateInstallationResType,
) -> i32 {
    let mut grammar_id: i32 = 241 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            241 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            5 as i32 as usize,
                            (*CertificateInstallationResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 242 as i32;
                            }
                        }
                    }
                }
            }
            242 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_CertificateChainType(
                        stream,
                        &(*CertificateInstallationResType).SAProvisioningCertificateChain,
                    );
                    if error == 0 as i32 {
                        grammar_id = 243 as i32;
                    }
                }
            }
            243 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_CertificateChainType(
                        stream,
                        &(*CertificateInstallationResType).ContractSignatureCertChain,
                    );
                    if error == 0 as i32 {
                        grammar_id = 244 as i32;
                    }
                }
            }
            244 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_ContractSignatureEncryptedPrivateKeyType(
                        stream,
                        &(*CertificateInstallationResType).ContractSignatureEncryptedPrivateKey,
                    );
                    if error == 0 as i32 {
                        grammar_id = 245 as i32;
                    }
                }
            }
            245 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_DiffieHellmanPublickeyType(
                        stream,
                        &(*CertificateInstallationResType).DHpublickey,
                    );
                    if error == 0 as i32 {
                        grammar_id = 246 as i32;
                    }
                }
            }
            246 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_EMAIDType(stream, &(*CertificateInstallationResType).eMAID);
                    if error == 0 as i32 {
                        grammar_id = 3 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_WeldingDetectionResType(
    stream: &mut ExiBitstream,
    mut WeldingDetectionResType: *const iso2_WeldingDetectionResType,
) -> i32 {
    let mut grammar_id: i32 = 247 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            247 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            5 as i32 as usize,
                            (*WeldingDetectionResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 248 as i32;
                            }
                        }
                    }
                }
            }
            248 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_DC_EVSEStatusType(
                        stream,
                        &(*WeldingDetectionResType).DC_EVSEStatus,
                    );
                    if error == 0 as i32 {
                        grammar_id = 249 as i32;
                    }
                }
            }
            249 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_PhysicalValueType(
                        stream,
                        &(*WeldingDetectionResType).EVSEPresentVoltage,
                    );
                    if error == 0 as i32 {
                        grammar_id = 3 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_CurrentDemandReqType(
    stream: &mut ExiBitstream,
    mut CurrentDemandReqType: *const iso2_CurrentDemandReqType,
) -> i32 {
    let mut grammar_id: i32 = 250 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            250 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso2_DC_EVStatusType(stream, &(*CurrentDemandReqType).DC_EVStatus);
                    if error == 0 as i32 {
                        grammar_id = 251 as i32;
                    }
                }
            }
            251 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_PhysicalValueType(
                        stream,
                        &(*CurrentDemandReqType).EVTargetCurrent,
                    );
                    if error == 0 as i32 {
                        grammar_id = 252 as i32;
                    }
                }
            }
            252 => {
                if (*CurrentDemandReqType).EVMaximumVoltageLimit_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PhysicalValueType(
                            stream,
                            &(*CurrentDemandReqType).EVMaximumVoltageLimit,
                        );
                        if error == 0 as i32 {
                            grammar_id = 253 as i32;
                        }
                    }
                } else if (*CurrentDemandReqType).EVMaximumCurrentLimit_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PhysicalValueType(
                            stream,
                            &(*CurrentDemandReqType).EVMaximumCurrentLimit,
                        );
                        if error == 0 as i32 {
                            grammar_id = 254 as i32;
                        }
                    }
                } else if (*CurrentDemandReqType).EVMaximumPowerLimit_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PhysicalValueType(
                            stream,
                            &(*CurrentDemandReqType).EVMaximumPowerLimit,
                        );
                        if error == 0 as i32 {
                            grammar_id = 255 as i32;
                        }
                    }
                } else if (*CurrentDemandReqType).BulkChargingComplete_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*CurrentDemandReqType).BulkChargingComplete,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 256 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*CurrentDemandReqType).ChargingComplete,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 257 as i32;
                                }
                            }
                        }
                    }
                }
            }
            253 => {
                if (*CurrentDemandReqType).EVMaximumCurrentLimit_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PhysicalValueType(
                            stream,
                            &(*CurrentDemandReqType).EVMaximumCurrentLimit,
                        );
                        if error == 0 as i32 {
                            grammar_id = 254 as i32;
                        }
                    }
                } else if (*CurrentDemandReqType).EVMaximumPowerLimit_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PhysicalValueType(
                            stream,
                            &(*CurrentDemandReqType).EVMaximumPowerLimit,
                        );
                        if error == 0 as i32 {
                            grammar_id = 255 as i32;
                        }
                    }
                } else if (*CurrentDemandReqType).BulkChargingComplete_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*CurrentDemandReqType).BulkChargingComplete,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 256 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*CurrentDemandReqType).ChargingComplete,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 257 as i32;
                                }
                            }
                        }
                    }
                }
            }
            254 => {
                if (*CurrentDemandReqType).EVMaximumPowerLimit_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PhysicalValueType(
                            stream,
                            &(*CurrentDemandReqType).EVMaximumPowerLimit,
                        );
                        if error == 0 as i32 {
                            grammar_id = 255 as i32;
                        }
                    }
                } else if (*CurrentDemandReqType).BulkChargingComplete_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*CurrentDemandReqType).BulkChargingComplete,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 256 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*CurrentDemandReqType).ChargingComplete,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 257 as i32;
                                }
                            }
                        }
                    }
                }
            }
            255 => {
                if (*CurrentDemandReqType).BulkChargingComplete_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*CurrentDemandReqType).BulkChargingComplete,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 256 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bool(
                                stream,
                                (*CurrentDemandReqType).ChargingComplete,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 257 as i32;
                                }
                            }
                        }
                    }
                }
            }
            256 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_bool(
                            stream,
                            (*CurrentDemandReqType).ChargingComplete,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 257 as i32;
                            }
                        }
                    }
                }
            }
            257 => {
                if (*CurrentDemandReqType).RemainingTimeToFullSoC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PhysicalValueType(
                            stream,
                            &(*CurrentDemandReqType).RemainingTimeToFullSoC,
                        );
                        if error == 0 as i32 {
                            grammar_id = 258 as i32;
                        }
                    }
                } else if (*CurrentDemandReqType).RemainingTimeToBulkSoC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PhysicalValueType(
                            stream,
                            &(*CurrentDemandReqType).RemainingTimeToBulkSoC,
                        );
                        if error == 0 as i32 {
                            grammar_id = 259 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PhysicalValueType(
                            stream,
                            &(*CurrentDemandReqType).EVTargetVoltage,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                }
            }
            258 => {
                if (*CurrentDemandReqType).RemainingTimeToBulkSoC_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PhysicalValueType(
                            stream,
                            &(*CurrentDemandReqType).RemainingTimeToBulkSoC,
                        );
                        if error == 0 as i32 {
                            grammar_id = 259 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_PhysicalValueType(
                            stream,
                            &(*CurrentDemandReqType).EVTargetVoltage,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                }
            }
            259 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_PhysicalValueType(
                        stream,
                        &(*CurrentDemandReqType).EVTargetVoltage,
                    );
                    if error == 0 as i32 {
                        grammar_id = 3 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_PreChargeResType(
    stream: &mut ExiBitstream,
    mut PreChargeResType: *const iso2_PreChargeResType,
) -> i32 {
    let mut grammar_id: i32 = 260 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            260 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            5 as i32 as usize,
                            (*PreChargeResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 261 as i32;
                            }
                        }
                    }
                }
            }
            261 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso2_DC_EVSEStatusType(stream, &(*PreChargeResType).DC_EVSEStatus);
                    if error == 0 as i32 {
                        grammar_id = 262 as i32;
                    }
                }
            }
            262 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_PhysicalValueType(
                        stream,
                        &(*PreChargeResType).EVSEPresentVoltage,
                    );
                    if error == 0 as i32 {
                        grammar_id = 3 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_CertificateUpdateResType(
    stream: &mut ExiBitstream,
    mut CertificateUpdateResType: *const iso2_CertificateUpdateResType,
) -> i32 {
    let mut grammar_id: i32 = 263 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            263 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            5 as i32 as usize,
                            (*CertificateUpdateResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 264 as i32;
                            }
                        }
                    }
                }
            }
            264 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_CertificateChainType(
                        stream,
                        &(*CertificateUpdateResType).SAProvisioningCertificateChain,
                    );
                    if error == 0 as i32 {
                        grammar_id = 265 as i32;
                    }
                }
            }
            265 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_CertificateChainType(
                        stream,
                        &(*CertificateUpdateResType).ContractSignatureCertChain,
                    );
                    if error == 0 as i32 {
                        grammar_id = 266 as i32;
                    }
                }
            }
            266 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_ContractSignatureEncryptedPrivateKeyType(
                        stream,
                        &(*CertificateUpdateResType).ContractSignatureEncryptedPrivateKey,
                    );
                    if error == 0 as i32 {
                        grammar_id = 267 as i32;
                    }
                }
            }
            267 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_DiffieHellmanPublickeyType(
                        stream,
                        &(*CertificateUpdateResType).DHpublickey,
                    );
                    if error == 0 as i32 {
                        grammar_id = 268 as i32;
                    }
                }
            }
            268 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_EMAIDType(stream, &(*CertificateUpdateResType).eMAID);
                    if error == 0 as i32 {
                        grammar_id = 269 as i32;
                    }
                }
            }
            269 => {
                if (*CertificateUpdateResType).RetryCounter_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_integer_16(
                                stream,
                                (*CertificateUpdateResType).RetryCounter,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_MeteringReceiptReqType(
    stream: &mut ExiBitstream,
    mut MeteringReceiptReqType: *const iso2_MeteringReceiptReqType,
) -> i32 {
    let mut grammar_id: i32 = 270 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            270 => {
                if (*MeteringReceiptReqType).Id_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*MeteringReceiptReqType).Id.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*MeteringReceiptReqType).Id.charactersLen as usize,
                                ((*MeteringReceiptReqType).Id.characters).as_ptr(),
                                (64 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                grammar_id = 271 as i32;
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*MeteringReceiptReqType).SessionID.bytesLen,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_bytes(
                                    stream,
                                    (*MeteringReceiptReqType).SessionID.bytesLen as usize,
                                    ((*MeteringReceiptReqType).SessionID.bytes).as_ptr(),
                                    8 as i32 as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 272 as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            271 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*MeteringReceiptReqType).SessionID.bytesLen,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*MeteringReceiptReqType).SessionID.bytesLen as usize,
                                ((*MeteringReceiptReqType).SessionID.bytes).as_ptr(),
                                8 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 272 as i32;
                                }
                            }
                        }
                    }
                }
            }
            272 => {
                if (*MeteringReceiptReqType).SAScheduleTupleID_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                8 as i32 as usize,
                                ((*MeteringReceiptReqType).SAScheduleTupleID as u32)
                                    .wrapping_sub(1 as i32 as u32),
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 273 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso2_MeterInfoType(stream, &(*MeteringReceiptReqType).MeterInfo);
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                }
            }
            273 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_MeterInfoType(stream, &(*MeteringReceiptReqType).MeterInfo);
                    if error == 0 as i32 {
                        grammar_id = 3 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_ChargingStatusReqType(
    stream: &mut ExiBitstream,
    mut ChargingStatusReqType: *const iso2_ChargingStatusReqType,
) -> i32 {
    let mut error: i32 =
        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
    return error;
}
fn encode_iso2_SessionStopResType(
    stream: &mut ExiBitstream,
    mut SessionStopResType: *const iso2_SessionStopResType,
) -> i32 {
    let mut grammar_id: i32 = 274 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            274 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            5 as i32 as usize,
                            (*SessionStopResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_ChargeParameterDiscoveryReqType(
    stream: &mut ExiBitstream,
    mut ChargeParameterDiscoveryReqType: *const iso2_ChargeParameterDiscoveryReqType,
) -> i32 {
    let mut grammar_id: i32 = 275 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            275 => {
                if (*ChargeParameterDiscoveryReqType).MaxEntriesSAScheduleTuple_isUsed() == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                (*ChargeParameterDiscoveryReqType).MaxEntriesSAScheduleTuple,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 276 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                3 as i32 as usize,
                                (*ChargeParameterDiscoveryReqType).RequestedEnergyTransferMode
                                    as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 277 as i32;
                                }
                            }
                        }
                    }
                }
            }
            276 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            3 as i32 as usize,
                            (*ChargeParameterDiscoveryReqType).RequestedEnergyTransferMode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 277 as i32;
                            }
                        }
                    }
                }
            }
            277 => {
                if (*ChargeParameterDiscoveryReqType).AC_EVChargeParameter_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_AC_EVChargeParameterType(
                            stream,
                            &(*ChargeParameterDiscoveryReqType).AC_EVChargeParameter,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*ChargeParameterDiscoveryReqType).DC_EVChargeParameter_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_DC_EVChargeParameterType(
                            stream,
                            &(*ChargeParameterDiscoveryReqType).DC_EVChargeParameter,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_EVChargeParameterType(
                            stream,
                            &(*ChargeParameterDiscoveryReqType).EVChargeParameter,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_CableCheckReqType(
    stream: &mut ExiBitstream,
    mut CableCheckReqType: *const iso2_CableCheckReqType,
) -> i32 {
    let mut grammar_id: i32 = 278 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            278 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_DC_EVStatusType(stream, &(*CableCheckReqType).DC_EVStatus);
                    if error == 0 as i32 {
                        grammar_id = 3 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_WeldingDetectionReqType(
    stream: &mut ExiBitstream,
    mut WeldingDetectionReqType: *const iso2_WeldingDetectionReqType,
) -> i32 {
    let mut grammar_id: i32 = 279 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            279 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_DC_EVStatusType(
                        stream,
                        &(*WeldingDetectionReqType).DC_EVStatus,
                    );
                    if error == 0 as i32 {
                        grammar_id = 3 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_PowerDeliveryResType(
    stream: &mut ExiBitstream,
    mut PowerDeliveryResType: *const iso2_PowerDeliveryResType,
) -> i32 {
    let mut grammar_id: i32 = 280 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            280 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            5 as i32 as usize,
                            (*PowerDeliveryResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 281 as i32;
                            }
                        }
                    }
                }
            }
            281 => {
                if (*PowerDeliveryResType).AC_EVSEStatus_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_AC_EVSEStatusType(
                            stream,
                            &(*PowerDeliveryResType).AC_EVSEStatus,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*PowerDeliveryResType).DC_EVSEStatus_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_DC_EVSEStatusType(
                            stream,
                            &(*PowerDeliveryResType).DC_EVSEStatus,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error =
                            encode_iso2_EVSEStatusType(stream, &(*PowerDeliveryResType).EVSEStatus);
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_ChargeParameterDiscoveryResType(
    stream: &mut ExiBitstream,
    mut ChargeParameterDiscoveryResType: *const iso2_ChargeParameterDiscoveryResType,
) -> i32 {
    let mut grammar_id: i32 = 282 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            282 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            5 as i32 as usize,
                            (*ChargeParameterDiscoveryResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 283 as i32;
                            }
                        }
                    }
                }
            }
            283 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*ChargeParameterDiscoveryResType).EVSEProcessing as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 284 as i32;
                            }
                        }
                    }
                }
            }
            284 => {
                if (*ChargeParameterDiscoveryResType).SAScheduleList_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_SAScheduleListType(
                            stream,
                            &(*ChargeParameterDiscoveryResType).SAScheduleList,
                        );
                        if error == 0 as i32 {
                            grammar_id = 285 as i32;
                        }
                    }
                } else if (*ChargeParameterDiscoveryResType).SASchedules_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_SASchedulesType(
                            stream,
                            &(*ChargeParameterDiscoveryResType).SASchedules,
                        );
                        if error == 0 as i32 {
                            grammar_id = 285 as i32;
                        }
                    }
                } else if (*ChargeParameterDiscoveryResType).AC_EVSEChargeParameter_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_AC_EVSEChargeParameterType(
                            stream,
                            &(*ChargeParameterDiscoveryResType).AC_EVSEChargeParameter,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*ChargeParameterDiscoveryResType).DC_EVSEChargeParameter_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_DC_EVSEChargeParameterType(
                            stream,
                            &(*ChargeParameterDiscoveryResType).DC_EVSEChargeParameter,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 3 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_EVSEChargeParameterType(
                            stream,
                            &(*ChargeParameterDiscoveryResType).EVSEChargeParameter,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                }
            }
            285 => {
                if (*ChargeParameterDiscoveryResType).AC_EVSEChargeParameter_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_AC_EVSEChargeParameterType(
                            stream,
                            &(*ChargeParameterDiscoveryResType).AC_EVSEChargeParameter,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*ChargeParameterDiscoveryResType).DC_EVSEChargeParameter_isUsed()
                    == 1 as u32
                {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_DC_EVSEChargeParameterType(
                            stream,
                            &(*ChargeParameterDiscoveryResType).DC_EVSEChargeParameter,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_EVSEChargeParameterType(
                            stream,
                            &(*ChargeParameterDiscoveryResType).EVSEChargeParameter,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_PaymentServiceSelectionReqType(
    stream: &mut ExiBitstream,
    mut PaymentServiceSelectionReqType: *const iso2_PaymentServiceSelectionReqType,
) -> i32 {
    let mut grammar_id: i32 = 286 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            286 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            (*PaymentServiceSelectionReqType).SelectedPaymentOption as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 287 as i32;
                            }
                        }
                    }
                }
            }
            287 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_SelectedServiceListType(
                        stream,
                        &(*PaymentServiceSelectionReqType).SelectedServiceList,
                    );
                    if error == 0 as i32 {
                        grammar_id = 3 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_MeteringReceiptResType(
    stream: &mut ExiBitstream,
    mut MeteringReceiptResType: *const iso2_MeteringReceiptResType,
) -> i32 {
    let mut grammar_id: i32 = 288 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            288 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            5 as i32 as usize,
                            (*MeteringReceiptResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 289 as i32;
                            }
                        }
                    }
                }
            }
            289 => {
                if (*MeteringReceiptResType).AC_EVSEStatus_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_AC_EVSEStatusType(
                            stream,
                            &(*MeteringReceiptResType).AC_EVSEStatus,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*MeteringReceiptResType).DC_EVSEStatus_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_DC_EVSEStatusType(
                            stream,
                            &(*MeteringReceiptResType).DC_EVSEStatus,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_EVSEStatusType(
                            stream,
                            &(*MeteringReceiptResType).EVSEStatus,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_CableCheckResType(
    stream: &mut ExiBitstream,
    mut CableCheckResType: *const iso2_CableCheckResType,
) -> i32 {
    let mut grammar_id: i32 = 290 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            290 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            5 as i32 as usize,
                            (*CableCheckResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 291 as i32;
                            }
                        }
                    }
                }
            }
            291 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        encode_iso2_DC_EVSEStatusType(stream, &(*CableCheckResType).DC_EVSEStatus);
                    if error == 0 as i32 {
                        grammar_id = 292 as i32;
                    }
                }
            }
            292 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*CableCheckResType).EVSEProcessing as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_ServiceDiscoveryResType(
    stream: &mut ExiBitstream,
    mut ServiceDiscoveryResType: *const iso2_ServiceDiscoveryResType,
) -> i32 {
    let mut grammar_id: i32 = 293 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            293 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            5 as i32 as usize,
                            (*ServiceDiscoveryResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 294 as i32;
                            }
                        }
                    }
                }
            }
            294 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_PaymentOptionListType(
                        stream,
                        &(*ServiceDiscoveryResType).PaymentOptionList,
                    );
                    if error == 0 as i32 {
                        grammar_id = 295 as i32;
                    }
                }
            }
            295 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_ChargeServiceType(
                        stream,
                        &(*ServiceDiscoveryResType).ChargeService,
                    );
                    if error == 0 as i32 {
                        grammar_id = 296 as i32;
                    }
                }
            }
            296 => {
                if (*ServiceDiscoveryResType).ServiceList_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_ServiceListType(
                            stream,
                            &(*ServiceDiscoveryResType).ServiceList,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_ServiceDetailReqType(
    stream: &mut ExiBitstream,
    mut ServiceDetailReqType: *const iso2_ServiceDetailReqType,
) -> i32 {
    let mut grammar_id: i32 = 297 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            297 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*ServiceDetailReqType).ServiceID,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_SessionSetupReqType(
    stream: &mut ExiBitstream,
    mut SessionSetupReqType: *const iso2_SessionSetupReqType,
) -> i32 {
    let mut grammar_id: i32 = 298 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            298 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*SessionSetupReqType).EVCCID.bytesLen,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*SessionSetupReqType).EVCCID.bytesLen as usize,
                                ((*SessionSetupReqType).EVCCID.bytes).as_ptr(),
                                6 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_SessionStopReqType(
    stream: &mut ExiBitstream,
    mut SessionStopReqType: *const iso2_SessionStopReqType,
) -> i32 {
    let mut grammar_id: i32 = 299 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            299 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            (*SessionStopReqType).ChargingSession as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_ServiceDiscoveryReqType(
    stream: &mut ExiBitstream,
    mut ServiceDiscoveryReqType: *const iso2_ServiceDiscoveryReqType,
) -> i32 {
    let mut grammar_id: i32 = 300 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            300 => {
                if (*ServiceDiscoveryReqType).ServiceScope_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_uint_16(
                                stream,
                                ((*ServiceDiscoveryReqType).ServiceScope.charactersLen as i32
                                    + 2 as i32) as u16,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_characters(
                                    stream,
                                    (*ServiceDiscoveryReqType).ServiceScope.charactersLen as usize,
                                    ((*ServiceDiscoveryReqType).ServiceScope.characters).as_ptr(),
                                    (64 as i32 + 1 as i32) as usize,
                                );
                                if error == 0 as i32 {
                                    error = exi_basetypes_encoder_nbit_uint(
                                        stream,
                                        1 as i32 as usize,
                                        0 as i32 as u32,
                                    );
                                    if error == 0 as i32 {
                                        grammar_id = 301 as i32;
                                    }
                                }
                            }
                        }
                    }
                } else if (*ServiceDiscoveryReqType).ServiceCategory_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                2 as i32 as usize,
                                (*ServiceDiscoveryReqType).ServiceCategory as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            301 => {
                if (*ServiceDiscoveryReqType).ServiceCategory_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            1 as i32 as usize,
                            0 as i32 as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                2 as i32 as usize,
                                (*ServiceDiscoveryReqType).ServiceCategory as u32,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 3 as i32;
                                }
                            }
                        }
                    }
                } else {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 2 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        done = 1 as i32;
                        grammar_id = 4 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_AuthorizationResType(
    stream: &mut ExiBitstream,
    mut AuthorizationResType: *const iso2_AuthorizationResType,
) -> i32 {
    let mut grammar_id: i32 = 302 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            302 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            5 as i32 as usize,
                            (*AuthorizationResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 303 as i32;
                            }
                        }
                    }
                }
            }
            303 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            2 as i32 as usize,
                            (*AuthorizationResType).EVSEProcessing as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_PaymentDetailsReqType(
    stream: &mut ExiBitstream,
    mut PaymentDetailsReqType: *const iso2_PaymentDetailsReqType,
) -> i32 {
    let mut grammar_id: i32 = 304 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            304 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            ((*PaymentDetailsReqType).eMAID.charactersLen as i32 + 2 as i32) as u16,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_characters(
                                stream,
                                (*PaymentDetailsReqType).eMAID.charactersLen as usize,
                                ((*PaymentDetailsReqType).eMAID.characters).as_ptr(),
                                (15 as i32 + 1 as i32) as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 305 as i32;
                                }
                            }
                        }
                    }
                }
            }
            305 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_CertificateChainType(
                        stream,
                        &(*PaymentDetailsReqType).ContractSignatureCertChain,
                    );
                    if error == 0 as i32 {
                        grammar_id = 3 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_PaymentDetailsResType(
    stream: &mut ExiBitstream,
    mut PaymentDetailsResType: *const iso2_PaymentDetailsResType,
) -> i32 {
    let mut grammar_id: i32 = 306 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            306 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_nbit_uint(
                            stream,
                            5 as i32 as usize,
                            (*PaymentDetailsResType).ResponseCode as u32,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 307 as i32;
                            }
                        }
                    }
                }
            }
            307 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_uint_16(
                            stream,
                            (*PaymentDetailsResType).GenChallenge.bytesLen,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_bytes(
                                stream,
                                (*PaymentDetailsResType).GenChallenge.bytesLen as usize,
                                ((*PaymentDetailsResType).GenChallenge.bytes).as_ptr(),
                                16 as i32 as usize,
                            );
                            if error == 0 as i32 {
                                error = exi_basetypes_encoder_nbit_uint(
                                    stream,
                                    1 as i32 as usize,
                                    0 as i32 as u32,
                                );
                                if error == 0 as i32 {
                                    grammar_id = 308 as i32;
                                }
                            }
                        }
                    }
                }
            }
            308 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = exi_basetypes_encoder_integer_64(
                            stream,
                            (*PaymentDetailsResType).EVSETimeStamp,
                        );
                        if error == 0 as i32 {
                            error = exi_basetypes_encoder_nbit_uint(
                                stream,
                                1 as i32 as usize,
                                0 as i32 as u32,
                            );
                            if error == 0 as i32 {
                                grammar_id = 3 as i32;
                            }
                        }
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_BodyType(
    stream: &mut ExiBitstream,
    mut BodyType: *const iso2_BodyType,
) -> i32 {
    let mut grammar_id: i32 = 309 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            309 => {
                if (*BodyType).AuthorizationReq_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 0 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_AuthorizationReqType(
                            stream,
                            &(*BodyType).c2rust_unnamed.AuthorizationReq,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).AuthorizationRes_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 1 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_AuthorizationResType(
                            stream,
                            &(*BodyType).c2rust_unnamed.AuthorizationRes,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).BodyElement_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 2 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_BodyBaseType(
                            stream,
                            &(*BodyType).c2rust_unnamed.BodyElement,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).CableCheckReq_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 3 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_CableCheckReqType(
                            stream,
                            &(*BodyType).c2rust_unnamed.CableCheckReq,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).CableCheckRes_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 4 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_CableCheckResType(
                            stream,
                            &(*BodyType).c2rust_unnamed.CableCheckRes,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).CertificateInstallationReq_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 5 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_CertificateInstallationReqType(
                            stream,
                            &(*BodyType).c2rust_unnamed.CertificateInstallationReq,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).CertificateInstallationRes_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 6 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_CertificateInstallationResType(
                            stream,
                            &(*BodyType).c2rust_unnamed.CertificateInstallationRes,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).CertificateUpdateReq_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 7 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_CertificateUpdateReqType(
                            stream,
                            &(*BodyType).c2rust_unnamed.CertificateUpdateReq,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).CertificateUpdateRes_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 8 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_CertificateUpdateResType(
                            stream,
                            &(*BodyType).c2rust_unnamed.CertificateUpdateRes,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).ChargeParameterDiscoveryReq_isUsed() == 1 as u32 {
                    error =
                        exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 9 as i32 as u32);
                    if error == 0 as i32 {
                        error = encode_iso2_ChargeParameterDiscoveryReqType(
                            stream,
                            &(*BodyType).c2rust_unnamed.ChargeParameterDiscoveryReq,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).ChargeParameterDiscoveryRes_isUsed() == 1 as u32 {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        6 as i32 as usize,
                        10 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        error = encode_iso2_ChargeParameterDiscoveryResType(
                            stream,
                            &(*BodyType).c2rust_unnamed.ChargeParameterDiscoveryRes,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).ChargingStatusReq_isUsed() == 1 as u32 {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        6 as i32 as usize,
                        11 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        error = encode_iso2_ChargingStatusReqType(
                            stream,
                            &(*BodyType).c2rust_unnamed.ChargingStatusReq,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).ChargingStatusRes_isUsed() == 1 as u32 {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        6 as i32 as usize,
                        12 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        error = encode_iso2_ChargingStatusResType(
                            stream,
                            &(*BodyType).c2rust_unnamed.ChargingStatusRes,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).CurrentDemandReq_isUsed() == 1 as u32 {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        6 as i32 as usize,
                        13 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        error = encode_iso2_CurrentDemandReqType(
                            stream,
                            &(*BodyType).c2rust_unnamed.CurrentDemandReq,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).CurrentDemandRes_isUsed() == 1 as u32 {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        6 as i32 as usize,
                        14 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        error = encode_iso2_CurrentDemandResType(
                            stream,
                            &(*BodyType).c2rust_unnamed.CurrentDemandRes,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).MeteringReceiptReq_isUsed() == 1 as u32 {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        6 as i32 as usize,
                        15 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        error = encode_iso2_MeteringReceiptReqType(
                            stream,
                            &(*BodyType).c2rust_unnamed.MeteringReceiptReq,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).MeteringReceiptRes_isUsed() == 1 as u32 {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        6 as i32 as usize,
                        16 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        error = encode_iso2_MeteringReceiptResType(
                            stream,
                            &(*BodyType).c2rust_unnamed.MeteringReceiptRes,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).PaymentDetailsReq_isUsed() == 1 as u32 {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        6 as i32 as usize,
                        17 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        error = encode_iso2_PaymentDetailsReqType(
                            stream,
                            &(*BodyType).c2rust_unnamed.PaymentDetailsReq,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).PaymentDetailsRes_isUsed() == 1 as u32 {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        6 as i32 as usize,
                        18 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        error = encode_iso2_PaymentDetailsResType(
                            stream,
                            &(*BodyType).c2rust_unnamed.PaymentDetailsRes,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).PaymentServiceSelectionReq_isUsed() == 1 as u32 {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        6 as i32 as usize,
                        19 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        error = encode_iso2_PaymentServiceSelectionReqType(
                            stream,
                            &(*BodyType).c2rust_unnamed.PaymentServiceSelectionReq,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).PaymentServiceSelectionRes_isUsed() == 1 as u32 {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        6 as i32 as usize,
                        20 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        error = encode_iso2_PaymentServiceSelectionResType(
                            stream,
                            &(*BodyType).c2rust_unnamed.PaymentServiceSelectionRes,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).PowerDeliveryReq_isUsed() == 1 as u32 {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        6 as i32 as usize,
                        21 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        error = encode_iso2_PowerDeliveryReqType(
                            stream,
                            &(*BodyType).c2rust_unnamed.PowerDeliveryReq,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).PowerDeliveryRes_isUsed() == 1 as u32 {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        6 as i32 as usize,
                        22 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        error = encode_iso2_PowerDeliveryResType(
                            stream,
                            &(*BodyType).c2rust_unnamed.PowerDeliveryRes,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).PreChargeReq_isUsed() == 1 as u32 {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        6 as i32 as usize,
                        23 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        error = encode_iso2_PreChargeReqType(
                            stream,
                            &(*BodyType).c2rust_unnamed.PreChargeReq,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).PreChargeRes_isUsed() == 1 as u32 {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        6 as i32 as usize,
                        24 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        error = encode_iso2_PreChargeResType(
                            stream,
                            &(*BodyType).c2rust_unnamed.PreChargeRes,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).ServiceDetailReq_isUsed() == 1 as u32 {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        6 as i32 as usize,
                        25 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        error = encode_iso2_ServiceDetailReqType(
                            stream,
                            &(*BodyType).c2rust_unnamed.ServiceDetailReq,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).ServiceDetailRes_isUsed() == 1 as u32 {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        6 as i32 as usize,
                        26 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        error = encode_iso2_ServiceDetailResType(
                            stream,
                            &(*BodyType).c2rust_unnamed.ServiceDetailRes,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).ServiceDiscoveryReq_isUsed() == 1 as u32 {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        6 as i32 as usize,
                        27 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        error = encode_iso2_ServiceDiscoveryReqType(
                            stream,
                            &(*BodyType).c2rust_unnamed.ServiceDiscoveryReq,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).ServiceDiscoveryRes_isUsed() == 1 as u32 {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        6 as i32 as usize,
                        28 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        error = encode_iso2_ServiceDiscoveryResType(
                            stream,
                            &(*BodyType).c2rust_unnamed.ServiceDiscoveryRes,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).SessionSetupReq_isUsed() == 1 as u32 {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        6 as i32 as usize,
                        29 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        error = encode_iso2_SessionSetupReqType(
                            stream,
                            &(*BodyType).c2rust_unnamed.SessionSetupReq,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).SessionSetupRes_isUsed() == 1 as u32 {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        6 as i32 as usize,
                        30 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        error = encode_iso2_SessionSetupResType(
                            stream,
                            &(*BodyType).c2rust_unnamed.SessionSetupRes,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).SessionStopReq_isUsed() == 1 as u32 {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        6 as i32 as usize,
                        31 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        error = encode_iso2_SessionStopReqType(
                            stream,
                            &(*BodyType).c2rust_unnamed.SessionStopReq,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).SessionStopRes_isUsed() == 1 as u32 {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        6 as i32 as usize,
                        32 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        error = encode_iso2_SessionStopResType(
                            stream,
                            &(*BodyType).c2rust_unnamed.SessionStopRes,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).WeldingDetectionReq_isUsed() == 1 as u32 {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        6 as i32 as usize,
                        33 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        error = encode_iso2_WeldingDetectionReqType(
                            stream,
                            &(*BodyType).c2rust_unnamed.WeldingDetectionReq,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else if (*BodyType).WeldingDetectionRes_isUsed() == 1 as u32 {
                    error = exi_basetypes_encoder_nbit_uint(
                        stream,
                        6 as i32 as usize,
                        34 as i32 as u32,
                    );
                    if error == 0 as i32 {
                        error = encode_iso2_WeldingDetectionResType(
                            stream,
                            &(*BodyType).c2rust_unnamed.WeldingDetectionRes,
                        );
                        if error == 0 as i32 {
                            grammar_id = 3 as i32;
                        }
                    }
                } else {
                    error = -(70 as i32);
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}
fn encode_iso2_V2G_Message(
    stream: &mut ExiBitstream,
    mut V2G_Message: *const iso2_V2G_Message,
) -> i32 {
    let mut grammar_id: i32 = 310 as i32;
    let mut done: i32 = 0 as i32;
    let mut error: i32 = 0 as i32;
    while done == 0 {
        match grammar_id {
            310 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_MessageHeaderType(stream, &(*V2G_Message).Header);
                    if error == 0 as i32 {
                        grammar_id = 311 as i32;
                    }
                }
            }
            311 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    error = encode_iso2_BodyType(stream, &(*V2G_Message).Body);
                    if error == 0 as i32 {
                        grammar_id = 3 as i32;
                    }
                }
            }
            3 => {
                error = exi_basetypes_encoder_nbit_uint(stream, 1 as i32 as usize, 0 as i32 as u32);
                if error == 0 as i32 {
                    done = 1 as i32;
                    grammar_id = 4 as i32;
                }
            }
            _ => {
                error = -(130 as i32);
            }
        }
        if error != 0 {
            done = 1 as i32;
        }
    }
    return error;
}

pub fn encode_iso2_exiDocument(
    stream: &mut ExiBitstream,
    mut exiDoc: *mut iso2_exiDocument,
) -> i32 {
    let mut error: i32 = exi_header_write(stream);
    if error == 0 as i32 {
        error = exi_basetypes_encoder_nbit_uint(stream, 7 as i32 as usize, 76 as i32 as u32);
        if error == 0 as i32 {
            error = encode_iso2_V2G_Message(stream, &mut (*exiDoc).V2G_Message);
        }
    }
    return error;
}

pub fn encode_iso2_exiFragment(
    stream: &mut ExiBitstream,
    mut exiFrag: *mut iso2_exiFragment,
) -> i32 {
    let mut error: i32 = exi_header_write(stream);
    if error == 0 as i32 {
        if 0 as i32 == 1 as i32 {
            error = -(299 as i32);
        } else if (*exiFrag).AuthorizationReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 8 as i32 as usize, 4 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso2_AuthorizationReqType(
                    stream,
                    &mut (*exiFrag).c2rust_unnamed.AuthorizationReq,
                );
            }
        } else if (*exiFrag).CertificateInstallationReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 8 as i32 as usize, 15 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso2_CertificateInstallationReqType(
                    stream,
                    &mut (*exiFrag).c2rust_unnamed.CertificateInstallationReq,
                );
            }
        } else if (*exiFrag).CertificateUpdateReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 8 as i32 as usize, 17 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso2_CertificateUpdateReqType(
                    stream,
                    &mut (*exiFrag).c2rust_unnamed.CertificateUpdateReq,
                );
            }
        } else if (*exiFrag).ContractSignatureCertChain_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 8 as i32 as usize, 33 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso2_CertificateChainType(
                    stream,
                    &mut (*exiFrag).c2rust_unnamed.ContractSignatureCertChain,
                );
            }
        } else if (*exiFrag).ContractSignatureEncryptedPrivateKey_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 8 as i32 as usize, 34 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso2_ContractSignatureEncryptedPrivateKeyType(
                    stream,
                    &mut (*exiFrag)
                        .c2rust_unnamed
                        .ContractSignatureEncryptedPrivateKey,
                );
            }
        } else if (*exiFrag).DHpublickey_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 8 as i32 as usize, 45 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso2_DiffieHellmanPublickeyType(
                    stream,
                    &mut (*exiFrag).c2rust_unnamed.DHpublickey,
                );
            }
        } else if (*exiFrag).MeteringReceiptReq_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 8 as i32 as usize, 121 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso2_MeteringReceiptReqType(
                    stream,
                    &mut (*exiFrag).c2rust_unnamed.MeteringReceiptReq,
                );
            }
        } else if (*exiFrag).SalesTariff_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 8 as i32 as usize, 174 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso2_SalesTariffType(stream, &mut (*exiFrag).c2rust_unnamed.SalesTariff);
            }
        } else if (*exiFrag).SignedInfo_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 8 as i32 as usize, 208 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso2_SignedInfoType(stream, &mut (*exiFrag).c2rust_unnamed.SignedInfo);
            }
        } else if (*exiFrag).eMAID_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 8 as i32 as usize, 236 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso2_EMAIDType(stream, &mut (*exiFrag).c2rust_unnamed.eMAID);
            }
        } else {
            error = -(70 as i32);
        }
        if error == 0 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 8 as i32 as usize, 244 as i32 as u32);
        }
    }
    return error;
}

pub fn encode_iso2_xmldsigFragment(
    stream: &mut ExiBitstream,
    mut xmldsigFrag: *mut iso2_xmldsigFragment,
) -> i32 {
    let mut error: i32 = exi_header_write(stream);
    if error == 0 as i32 {
        if (*xmldsigFrag).CanonicalizationMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 0 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso2_CanonicalizationMethodType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.CanonicalizationMethod,
                );
            }
        } else if (*xmldsigFrag).DSAKeyValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 1 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso2_DSAKeyValueType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.DSAKeyValue,
                );
            }
        } else if (*xmldsigFrag).DigestMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 2 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso2_DigestMethodType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.DigestMethod,
                );
            }
        } else if (*xmldsigFrag).KeyInfo_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 8 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso2_KeyInfoType(stream, &mut (*xmldsigFrag).c2rust_unnamed.KeyInfo);
            }
        } else if (*xmldsigFrag).KeyValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 10 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso2_KeyValueType(stream, &mut (*xmldsigFrag).c2rust_unnamed.KeyValue);
            }
        } else if (*xmldsigFrag).Object_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 14 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso2_ObjectType(stream, &mut (*xmldsigFrag).c2rust_unnamed.Object);
            }
        } else if (*xmldsigFrag).PGPData_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 16 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso2_PGPDataType(stream, &mut (*xmldsigFrag).c2rust_unnamed.PGPData);
            }
        } else if (*xmldsigFrag).RSAKeyValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 21 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso2_RSAKeyValueType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.RSAKeyValue,
                );
            }
        } else if (*xmldsigFrag).Reference_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 22 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso2_ReferenceType(stream, &mut (*xmldsigFrag).c2rust_unnamed.Reference);
            }
        } else if (*xmldsigFrag).RetrievalMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 23 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso2_RetrievalMethodType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.RetrievalMethod,
                );
            }
        } else if (*xmldsigFrag).SPKIData_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 24 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso2_SPKIDataType(stream, &mut (*xmldsigFrag).c2rust_unnamed.SPKIData);
            }
        } else if (*xmldsigFrag).Signature_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 27 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso2_SignatureType(stream, &mut (*xmldsigFrag).c2rust_unnamed.Signature);
            }
        } else if (*xmldsigFrag).SignatureMethod_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 28 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso2_SignatureMethodType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.SignatureMethod,
                );
            }
        } else if (*xmldsigFrag).SignatureValue_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 31 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso2_SignatureValueType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.SignatureValue,
                );
            }
        } else if (*xmldsigFrag).SignedInfo_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 32 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso2_SignedInfoType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.SignedInfo,
                );
            }
        } else if (*xmldsigFrag).Transform_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 33 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso2_TransformType(stream, &mut (*xmldsigFrag).c2rust_unnamed.Transform);
            }
        } else if (*xmldsigFrag).Transforms_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 34 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso2_TransformsType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.Transforms,
                );
            }
        } else if (*xmldsigFrag).X509Data_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 37 as i32 as u32);
            if error == 0 as i32 {
                error =
                    encode_iso2_X509DataType(stream, &mut (*xmldsigFrag).c2rust_unnamed.X509Data);
            }
        } else if (*xmldsigFrag).X509IssuerSerial_isUsed() as i32 == 1 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 39 as i32 as u32);
            if error == 0 as i32 {
                error = encode_iso2_X509IssuerSerialType(
                    stream,
                    &mut (*xmldsigFrag).c2rust_unnamed.X509IssuerSerial,
                );
            }
        } else {
            error = -(70 as i32);
        }
        if error == 0 as i32 {
            error = exi_basetypes_encoder_nbit_uint(stream, 6 as i32 as usize, 46 as i32 as u32);
        }
    }
    return error;
}
