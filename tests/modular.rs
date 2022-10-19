
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