use primitive_types::U256;

pub fn pow_cache(x: U256, p_upto: U256, modulus: U256) -> Vec<U256> {
    let mut pow_cache = vec![U256::one(), x];
    let mut pow = U256::one();
    let mut x_last_pow = x;

    loop {
        pow = pow + pow;
        // If we've exceeded the p_upto, we have all we need
        if pow > p_upto {
            break;
        }
        x_last_pow = (x_last_pow * x_last_pow) % modulus;
        pow_cache.push(x_last_pow);
    }

    pow_cache
}

pub fn pow(x: u128, p: U256, modulus: u128) -> u128 {
    let x: U256 = x.into();
    let modulus: U256 = modulus.into();

    let mut x_pow_p: U256 = U256::one();
    let mut pow_bits_left = p.as_u128();
    let mut bit_pos = 0;
    let cached_pows = pow_cache(x, p, modulus);

    loop {
        if pow_bits_left == 0 {
            break;
        }

        // Set bit position
        bit_pos += 1;

        // Get the bit at position and remaining bits (quotient)
        let bit = pow_bits_left & 1;
        pow_bits_left >>= 1;

        if bit != 0 {
            x_pow_p = (x_pow_p * cached_pows[bit_pos as usize]) % modulus;
        }
    }

    x_pow_p.try_into().unwrap()
}

// Define a function  L(x) = ( x - 1 ) / n
pub fn L(x: U256, n: U256) -> U256 {
    (x - 1) / n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow_fast() {
        assert!(
            31381059609 == pow(3, 22.into(), 0x10000000000),
            "incorrect 5**4"
        );
        assert!(242546713 == pow(3, 22.into(), 0x10000000), "incorrect 5**4");
    }

    #[test]
    fn test_cache_pow() {
        let cache = pow_cache(3.into(), 25.into(), 0x100000000000000_u128.into());
        println!("{cache:?}");
        assert!(cache.len() == 6, "incorrect cache len");
        assert!(cache[0] == 1.into(), "incorrect cache 0");
        assert!(cache[1] == 3.into(), "incorrect cache 1");
        assert!(cache[2] == 9.into(), "incorrect cache 2");
        assert!(cache[3] == 81.into(), "incorrect cache 3");
        assert!(cache[4] == 6561.into(), "incorrect cache 4");
        assert!(cache[5] == 43046721.into(), "incorrect cache 5");
    }
}
