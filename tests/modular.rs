
use ibig::{ibig, modular::ModuloRing, ubig};

#[test]
fn test_modulus() {
    let ring = ModuloRing::new(&ubig!(100));
    assert_eq!(ring.modulus(), ubig!(100));

    let ring = ModuloRing::new(&ubig!(10).pow(100));
    assert_eq!(ring.modulus(), ubig!(10).pow(100));
}

#[test]
fn test_clone() {
    let ring1 = ModuloRing::new(&ubig!(100));
    let x = ring1.from(512);
    let y = x.clone();
    assert_eq!(x, y);
    let mut z = ring1.from(513);
    assert_ne!(x, z);
    z.clone_from(&x);
    assert_eq!(x, z);

    let ring2 = ModuloRing::new(&ubig!(_1000000000000000000000000000000));
    let x = ring2.from(512);
    let y = x.clone();
    assert_eq!(x, y);
    let mut z = ring2.from(513);
    assert_ne!(x, z);
    z.clone_from(&x);
    assert_eq!(x, z);

    let mut x = ring1.from(512);
    let y = ring2.from(1);
    x.clone_from(&y);
    assert_eq!(x, y);

    let ring3 = ModuloRing::new(&ubig!(10).pow(100));
    let x = ring2.from(1);
    let mut y = ring3.from(2);
    y.clone_from(&x);
    assert_eq!(x, y);
}

#[test]
fn test_convert() {
    let ring = ModuloRing::new(&ubig!(100));
    let x = ring.from(6);
    assert_eq!(x, ring.from(&ubig!(306)));
    assert_ne!(x, ring.from(&ubig!(313)));
    assert_eq!(x, ring.from(&ubig!(_18297381723918723981723981723906)));
    assert_ne!(x, ring.from(&ubig!(_18297381723918723981723981723913)));
    assert_eq!(x, ring.from(ubig!(_18297381723918723981723981723906)));
    assert_eq!(x, ring.from(ibig!(_18297381723918723981723981723906)));
    assert_eq!(x, ring.from(ibig!(-_18297381723918723981723981723994)));
    assert_eq!(x, ring.from(&ibig!(-_18297381723918723981723981723994)));
    assert_eq!(x, ring.from(106u8));
    assert_eq!(x, ring.from(106u16));
    assert_eq!(x, ring.from(1006u32));
    assert_eq!(x, ring.from(10000000006u64));
    assert_eq!(x, ring.from(1000000000000000000006u128));
    assert_eq!(x, ring.from(106usize));
    assert_eq!(x, ring.from(6i8));
    assert_eq!(x, ring.from(-94i8));
    assert_eq!(x, ring.from(-94i16));
    assert_eq!(x, ring.from(-94i32));
    assert_eq!(x, ring.from(-94i64));
    assert_eq!(x, ring.from(-94i128));
    assert_eq!(x, ring.from(-94isize));

    assert_eq!(ring.from(0), ring.from(false));
    assert_eq!(ring.from(1), ring.from(true));

    let ring = ModuloRing::new(&ubig!(
        _1000000000000000000000000000000000000000000000000000000000000
    ));
    let x = ring.from(6);
    let y = ring.from(ubig!(_333333333333333333333333333333));
    assert_eq!(
        x,
        ring.from(ubig!(
            _1000000000000000000000000000000000000000000000000000000000006
        ))
    );
    assert_eq!(
        x,
        ring.from(&ubig!(
            _1000000000000000000000000000000000000000000000000000000000006
        ))
    );
    assert_ne!(
        x,
        ring.from(ubig!(
            _1000000000000000000000000000000000000000000000000000000000007
        ))
    );
    assert_eq!(
        y,
        ring.from(ubig!(
            _7000000000000000000000000000000333333333333333333333333333333
        ))
    );
}

#[test]
fn test_negate() {
    let ring = ModuloRing::new(&ubig!(100));
    let x = ring.from(-1234);
    let y = -&x;
    assert_eq!(y.residue(), ubig!(34));
    let y = -x;
    assert_eq!(y.residue(), ubig!(34));

    let ring = ModuloRing::new(&ubig!(_1000000000000000000000000000000));
    let x = ring.from(ibig!(-_33333123456789012345678901234567890));
    let y = -&x;
    assert_eq!(y, ring.from(ubig!(_44444123456789012345678901234567890)));
    assert_eq!(y.residue(), ubig!(_123456789012345678901234567890));
    let y = -x;
    assert_eq!(y, ring.from(ubig!(_44444123456789012345678901234567890)));
}

#[test]
#[allow(clippy::eq_op)]
fn test_different_rings() {
    let ring1 = ModuloRing::new(&ubig!(100));
    let ring2 = ModuloRing::new(&ubig!(100));
    assert_eq!(ring1, ring1);
    assert_ne!(ring1, ring2);
}

#[test]
#[should_panic]
fn test_cmp_different_rings() {
    let ring1 = ModuloRing::new(&ubig!(100));
    let ring2 = ModuloRing::new(&ubig!(200));
    let x = ring1.from(5);
    let y = ring2.from(5);
    let _ = x == y;
}

#[test]
fn test_add_sub() {
    let ring1 = ModuloRing::new(&ubig!(100));
    let ring2 = ModuloRing::new(&ubig!(_1000000000000000000000000000000));
    let test_cases = [
        (ring1.from(1), ring1.from(2), ring1.from(3)),
        (ring1.from(99), ring1.from(5), ring1.from(4)),
        (ring1.from(99), ring1.from(99), ring1.from(98)),
        (
            ring2.from(ubig!(111111111111111111111111111111)),
            ring2.from(ubig!(222222222222222223333333333333)),
            ring2.from(ubig!(333333333333333334444444444444)),
        ),
        (
            ring2.from(ubig!(111111111111111111111111111111)),
            ring2.from(ubig!(888888888888888888888888888889)),
            ring2.from(ubig!(0)),
        ),
        (
            ring2.from(ubig!(999999999999999999999999999999)),
            ring2.from(ubig!(999999999999999999999999999997)),
            ring2.from(ubig!(999999999999999999999999999996)),
        ),
    ];

    let all_test_cases = test_cases
        .iter()
        .map(|(a, b, c)| (a, b, c))
        .chain(test_cases.iter().map(|(a, b, c)| (b, a, c)));

    for (a, b, c) in all_test_cases {
        assert_eq!(a + b, *c);
        assert_eq!(a.clone() + b, *c);
        assert_eq!(a + b.clone(), *c);
        assert_eq!(a.clone() + b.clone(), *c);
        let mut x = a.clone();
        x += b;
        assert_eq!(x, *c);
        let mut x = a.clone();
        x += b.clone();
        assert_eq!(x, *c);

        assert_eq!(c - a, *b);
        assert_eq!(c.clone() - a, *b);
        assert_eq!(c - a.clone(), *b);
        assert_eq!(c.clone() - a.clone(), *b);
        let mut x = c.clone();
        x -= a;
        assert_eq!(x, *b);
        let mut x = c.clone();
        x -= a.clone();
        assert_eq!(x, *b);
    }
}

#[test]
fn test_mul() {
    let ring1 = ModuloRing::new(&ubig!(100));
    let ring2 = ModuloRing::new(&ubig!(_1000000000000000000000000000000));
    let big = ubig!(10).pow(10000);
    let ring3 = ModuloRing::new(&big);
    let test_cases = [
        (ring1.from(23), ring1.from(96), ring1.from(8)),
        (
            ring2.from(ubig!(_46301564276035228370597101114)),
            ring2.from(ubig!(_170100953649249045221461413048)),
            ring2.from(ubig!(_399394418012748758198974935472)),
        ),
        (
            ring3.from(&big - ubig!(1)),
            ring3.from(&big - ubig!(1)),
            ring3.from(1),
        ),
    ];

    let all_test_cases = test_cases
        .iter()
        .map(|(a, b, c)| (a, b, c))
        .chain(test_cases.iter().map(|(a, b, c)| (b, a, c)));

    for (a, b, c) in all_test_cases {
        assert_eq!(a * b, *c);
        assert_eq!(a.clone() * b, *c);
        assert_eq!(a * b.clone(), *c);
        assert_eq!(a.clone() * b.clone(), *c);
        let mut x = a.clone();
        x *= b;
        assert_eq!(x, *c);
        let mut x = a.clone();
        x *= b.clone();
        assert_eq!(x, *c);
    }
}

#[test]
fn test_inverse() {
    let ring = ModuloRing::new(&ubig!(1));
    assert_eq!(ring.from(0).inverse(), Some(ring.from(0)));

    let ring = ModuloRing::new(&ubig!(100));
    let x = ring.from(9);
    let y = x.inverse().unwrap();
    assert_eq!(x * y, ring.from(1));

    assert!(ring.from(10).inverse().is_none());

    let ring = ModuloRing::new(&ubig!(103));
    assert_eq!(ring.from(20).inverse(), Some(ring.from(67))); // inverse is unique for prime modulus

    let ring = ModuloRing::new(&ubig!(1000000000000000000000000000000));
    let x = ring.from(ibig!(3333312345678901234567890123456789));
    let y = x.inverse().unwrap();
    assert_eq!(x * y, ring.from(1));

    assert!(ring.from(10).inverse().is_none());