use std::convert::TryInto;


/// Returns true iff the signed value `n` fits into `width` signed bits.
/// 
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
    // -(1 << (width - 1)) is -2^(width - 1)
    // (1 << (width - 1)) - 1 is 2^(width - 1) - 1
    // e.g., if width = 2, then -(1 << (width - 1)) = -2^(2 - 1) = -2
    // e.g., if width = 2, then (1 << (width - 1)) - 1 = 2^(2 - 1) - 1 = 1
    if n >= -(1 << (width - 1)) && n <= (1 << (width - 1)) - 1 {
        return true;
    }
    else {
        return false;
    }
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
/// 
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
    // (1 << width) - 1 is 2^width - 1
    // e.g., if width = 2, then (1 << width) - 1 = 2^2 - 1 = 3
    if n <= (1 << width) - 1 {
        return true;
    }
    else {
        return false;
    }
}


/// Retrieve a signed value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn gets(word: u64, width: u64, lsb: u64) -> i64 {
    // Apply the mask 
    let subset_mask = (1 << width) - 1 << lsb;

    // Extract the subset of bits from the word
    let mut value_bits = (word & subset_mask) as i64;

    // Check if the sign bit is set
    if (value_bits >> (lsb + width - 1)) & 1 == 1 {
        // If the value is negative, perform sign extension using negation mask
        let neg_mask = ((1 << (64 - lsb)) - 1) << lsb;
        value_bits = value_bits | neg_mask as i64;
    }

    // Return the extracted subset of bits as a signed 64-bit integer
    return value_bits;
}

/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {
    // Apply the mask
    let mask = ((1 << width) - 1) << lsb;

    // Extract subset of bits using the mask
    let value = (word & mask) >> lsb;

    // Return the extracted subset of bits as an unsigned 64-bit integer
    return value;
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the unsigned `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` unsigned bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the unsigned value to place into that bit field
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    if fitsu(value, width) {
        // Add word and shift by lsb bits
        return Some(word | (value << lsb));
    }

   return None;
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the signed `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` signed bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the signed value to place into that bit field
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    if fitss(value, width) {

        if value < 0 {
            // Apply the mask
            let mut mask: u64 = 0;
            for i in 0..(64 - width) {
                mask = mask | 1 << (64 - i - 1);
            }

            // Convert signed to unsigned value using XOR and mask
            let unsigned_bits = value as u64 ^ mask;

            let new_word: u64 = word | (unsigned_bits << lsb);

            return Some(new_word);
        }
        else {
            return newu(word, width, lsb, value as u64);
        }

    }

   return None;
}



