mod utils;
use primitive_types::U256;
use utils::{pow, L};

// Generate hiding for the value from the public key
fn encrypt(m: u128, r: u128, n: u128, g: u128) -> U256 {
    let n2 = n * n;
    assert!(g < n2, "g should be < n^2");
    assert!(m < n, "m should be < n");
    assert!(n < 0x10000000000000000, "n should be < 2^64");
    assert!(r < n, "r should be < n");
    // pow(g, m, n2) * pow(r, n, n2) % n2;
    let gm: U256 = pow(g, m.into(), n2).into();
    let rn: U256 = pow(r, n.into(), n2).into();
    gm * rn % n2
}

// Reveal hidings with private key
fn decrypt(c: u128, lambda: U256, n: u128, mu: U256) -> U256 {
    let n2 = n * n;
    assert!(c < n2, "c should be < n2");
    assert!(n < 0x10000000000000000, "n should be < 2^64");
    assert!(mu < n.into(), "mu should be < n");
    // L(pow(c, lambda, n2), n) * mu % n

    let cl: U256 = pow(c, lambda, n2).into();
    let l: U256 = L(cl, n.into());

    let n256: U256 = n.into();
    l * mu % n256
}

#[cfg(test)]
mod tests {
    use primitive_types::U256;

    const N: u128 = 12672350555099587607;
    const G: u128 = 33229279471906302552176601098426510220;
    // Private key params
    const LAMBDA: u128 = 6336175273684312140;
    const MU: u128 = 7673502522022171724;

    #[test]
    fn test_enc_dec() {
        let a_ = 25634256;
        let a = super::encrypt(a_, 23432, N, G);
        assert!(U256::from(0x5f6f35d026119ebdf31f56124f32cae0_u128) == a);
        assert!(a_ == super::decrypt(a.as_u128(), LAMBDA.into(), N, MU.into()).as_u128());

        let b_ = 6876343294;
        let b = super::encrypt(b_, 43256, N, G);
        assert!(U256::from(0x2d4d1a9678ad36f9bba1bdbbbf3b7382_u128) == b);
        assert!(b_ == super::decrypt(b.as_u128(), LAMBDA.into(), N, MU.into()).as_u128());
    }
}
