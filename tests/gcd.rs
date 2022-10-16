
use ibig::{
    ibig,
    ops::{Abs, UnsignedAbs},
    ubig, IBig, UBig,
};

#[test]
fn test_gcd_ubig() {
    // test cases (x, y, gcd(x,y))
    let test_cases = [
        // trivial cases
        (ubig!(0), ubig!(123), ubig!(123)),
        (ubig!(123), ubig!(0), ubig!(123)),
        (ubig!(1), ubig!(123), ubig!(1)),
        (ubig!(123), ubig!(1), ubig!(1)),
        (ubig!(123), ubig!(123), ubig!(123)),
        (
            ubig!(0),
            ubig!(_0x123456789123456789123456789123456789),
            ubig!(_0x123456789123456789123456789123456789),
        ),
        (
            ubig!(1),
            ubig!(_0x123456789123456789123456789123456789),
            ubig!(1),
        ),
        (
            ubig!(_0x123456789123456789123456789123456789),
            ubig!(_0x123456789123456789123456789123456789),
            ubig!(_0x123456789123456789123456789123456789),
        ),
        // small cases
        (ubig!(3), ubig!(7), ubig!(1)),
        (ubig!(8), ubig!(9), ubig!(1)),
        (ubig!(9), ubig!(8), ubig!(1)),
        (ubig!(42), ubig!(56), ubig!(14)),
        (ubig!(7966496), ubig!(314080416), ubig!(32)),
        // big cases
        (
            ubig!(0xffffffffffffffffffffffff1), // largest prime under 2^100
            ubig!(0x7ffffffffffffffffffffff8d), // largest prime under 2^99
            ubig!(1),
        ),
        (
            ubig!(0xffffffffffffffffffffffffffffff61), // largest prime under 2^128
            ubig!(0xffffffffffffffffffffffffffffff53), // second largest prime under 2^128
            ubig!(1),
        ),
        (
            ubig!(_0x3ffffffffffffffffffffffffffffffffffffd), // largest prime under 2^150
            ubig!(_0x1fffffffffffffffffffffffffffffffffffe1), // largest prime under 2^149
            ubig!(1),
        ),
        (
            ubig!(_0x123456789123456789123456789123456789),
            ubig!(_0x987654321),
            ubig!(_0x2d),
        ),
        (
            ubig!(_0x123456789123456789123456789123456789),
            ubig!(_0x987654321987654321987654321987654321),
            ubig!(_0x2d00000002d00000002d00000002d),
        ),
        (
            ubig!(_0x5a4653ca673768565b41f775d6947d55cf3813d1), // 3^100