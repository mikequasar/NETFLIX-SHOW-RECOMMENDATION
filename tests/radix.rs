use ibig::{error::ParseError, ibig, ubig, IBig, UBig};

#[test]
fn test_ubig_format() {
    assert_eq!(format!("{:b}", ubig!(0)), "0");
    assert_eq!(format!("{:b}", ubig!(100)), "1100100");
    assert_eq!(format!("{:#b}", ubig!(100)), "0b1100100");
    assert_eq!(format!("{:+b}", ubig!(100)), "+1100100");
    assert_eq!(format!("{:+#b}", ubig!(100)), "+0b1100100");
    assert_eq!(format!("{:10b}", ubig!(100)), "   1100100");
    assert_eq!(format!("{:=<10b}", ubig!(100)), "1100100===");
    assert_eq!(format!("{:=>10b}", ubig!(100)), "===1100100");
    assert_eq!(format!("{:=^10b}", ubig!(100)), "=1100100==");
    assert_eq!(format!("{:=^+10b}", ubig!(100)), "=+1100100=");
    assert_eq!(format!("{:+010b}", ubig!(100)), "+001100100");
    assert_eq!(format!("{:+#010b}", ubig!(100)), "+0b1100100");
    assert_eq!(format!("{:+#01b}", ubig!(100)), "+0b1100100");
    assert_eq!(format!("{:o}", ubig!(100)), "144");
    assert_eq!(format!("{:#o}", ubig!(100)), "0o144");
    assert_eq!(format!("{:x}", ubig!(3000)), "bb8");
    assert_eq!(format!("{:#x}", ubig!(3000)), "0xbb8");
    assert_eq!(format!("{:X}", ubig!(3000)), "BB8");
    assert_eq!(format!("{:#X}", ubig!(3000)), "0xBB8");
    assert_eq!(format!("{:#10X}", ubig!(3000)), "     0xBB8");

    assert_eq!(format!("{}", ubig!(123)), "123");
    assert_eq!(format!("{:?}", ubig!(123)), "123");
    assert_eq!(format!("{:=>5}", ubig!(123)), "==123");

    let a = UBig::from_be_bytes(&[
        0x05, 0xee, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89,
        0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67,
        0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
    ]);
    assert_eq!(
        format!("{:x}", a),
        "5ee0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    );
    assert_eq!(
        format!("{:X}", a),
        "5EE0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF"
    );
    assert_eq!(
        format!("{:^100X}", a),
        "        5EE0123456789ABCDEF\
        0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF         "
    );
    assert_eq!(
        format!("{:o}", a),
        "1367001106425474232571573600443212636115274675700221505317046\
        53633674011064254742325715736004432126361152746757"
    );
    assert_eq!(
        format!("{:>120o}", a),
        "         1367001106425474232571573600443212636115274675700221505317046\
        53633674011064254742325715736004432126361152746757"
    );
    assert_eq!(
        format!("{:>120}", a),
        "                    32424378138036567091203300829444432818122896389983\
        04588119616982843278155375835513236887964094287343"
    );
}

#[test]
fn test_ubig_in_radix() {
    assert_eq!(format!("{}", ubig!(0).in_radix(2)), "0");
    assert_eq!(format!("{}", ubig!(100).in_radix(4)), "1210");
    assert_eq!(format!("{}", ubig!(3000).in_radix(16)), "bb8");
    assert_eq!(format!("{:+010}", ubig!(3000).in_radix(16)), "+000000bb8");
    assert_eq!(format!("{:+#010}", ubig!(3000).in_radix(16)), "+000000BB8");
    assert_eq!(format!("{}", ubig!(1294).in_radix(36)), "zy");
    assert_eq!(format!("{:#010}", ubig!(1294).in_radix(36)), "00000000ZY");

    assert_eq!(
        ubig!(0xffffffff).in_radix(3).to_string(),
        "102002022201221111210"
    );
    assert_eq!(
        ubig!(0xffffffffffffffff).in_radix(3).to_string(),
        "11112220022122120101211020120210210211220"
    );

    let a = UBig::from_le_bytes(&[0xff; 50]);
    assert_eq!(
        a.in_radix(32).to_string(),
        "vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv"
    );

    assert_eq!(ubig!(123456789).to_string(), "123456789");
    assert_eq!(
        ubig!(_123456789000000000000000000000000000000000123789749847509837450987340589273405)
            .to_string(),
        "123456789000000000000000000000000000000000123789749847509837450987340589273405"
    );
    assert_eq!(ubig!(_0x83c0d7401f0188462502c2e5f7035386b1c341d307e5fbe8200756201607769a706134cfab1).to_string(),
            "1048383517376714931597372965085953822045235087388094946568022880798260489887669110556129969");
    assert_eq!(
        ubig!(_1048383517376714931597372965085953822045235087388094946568022880798260489887669110556129969).in_radix(16).to_string(),
        "83c0d7401f0188462502c2e5f7035386b1c341d307e5fbe8200756201607769a706134cfab1");
}

#[test]
fn test_ibig_format() {
    assert_eq!(format!("{:b}", ibig!(0)), "0");
    assert_eq!(format!("{:b}", ibig!(100)), "1100100");
    assert_eq!(format!("{:b}", ibig!(-100)), "-1100100");
    assert_eq!(format!("{:#b}", ibig!(100)), "0b1100100");
    assert_eq!(format!("{:#b}", ibig!(-100)), "-0b1100100");
    assert_eq!(format!("{:+b}", ibig!(100)), "+1100100");
    assert_eq!(format!("{:+b}", ibig!(-100)), "-1100100");
    assert_eq!(format!("{:+#b}", ibig!(100)), "+0b1100100");
    assert_eq!(format!("{:+#b}", ibig!(-100)), "-0b1100100");
    assert_eq!(format!("{:10b}", ibig!(100)), "   1100100");
    assert_eq!(format!("{:10b}", ibig!(-100)), "  -1100100");
    assert_eq!(format!("{:=<10b}", ibig!(100)), "1100100===");
    assert_eq!(format!("{:=<10b}", ibig!(-100)), "-1100100==");
    assert_eq!(format!("{:=>10b}", ibig!(100)), "===1100100");
    assert_eq!(format!("{:=>10b}", ibig!(-100)), "==-1100100");
    assert_eq!(format!("{:=^10b}", ibig!(100)), "=1100100==");
    assert_eq!(format!("{:=^10b}", ibig!(-100)), "=-1100100=");
    assert_eq!(format!("{:=^+10b}", ibig!(100)), "=+1100100=");
    assert_eq!(format!("{:=^+10b}", ibig!(-100)), "=-1100100=");
    assert_eq!(format!("{:+010b}", ibig!(100)), "+001100100");
    assert_eq!(format!("{:+010b}", ibig!(-100)), "-001100100");
    assert_eq!(format!("{:+#010b}", ibig!(100)), "+0b1100100");
    assert_eq!(format!("{:+#010b}", ibig!(-100)), "-0b1100100");
    assert_eq!(format!("{:+#01b}", ibig!(100)), "+0b1100100");
    assert_eq!(format!("{:+#01b}", ibig!(-100)), "-0b1100100");
    assert_eq!(format!("{:o}", ibig!(100)), "144");
    assert_eq!(format!("{:o}", ibig!(-100)), "-144");
    assert_eq!(format!("{:#o}", ibig!(100)), "0o144");
    assert_eq!(format!("{:#o}", ibig!(-100)), "-0o144");
    assert_eq!(format!("{:x}", ibig!(3000)), "bb8");
    assert_eq!(format!("{:x}", ibig!(-3000)), "-bb8");
    assert_eq!(format!("{:#x}", ibig!(3000)), "0xbb8");
    assert_eq!(format!("{:#x}", ibig!(-3000)), "-0xbb8");
    assert_eq!(format!("{:X}", ibig!(3000)), "BB8");
    assert_eq!(format!("{:X}", ibig!(-3000)), "-BB8");
    assert_eq!(format!("{:#X}", ibig!(3000)), "0xBB8");
    assert_eq!(format!("{:#X}", ibig!(-3000)), "-0xBB8");
    assert_eq!(format!("{:#10X}", ibig!(3000)), "     0xBB8");
    assert_eq!(format!("{:#10X}", ibig!(-3000)), "    -0xBB8");

    assert_eq!(format!("{}", ibig!(-123)), "-123");
    assert_eq!(format!("{:?}", ibig!(-123)), "-123");
    assert_eq!(format!("{:=>10}", ibig!(-123)), "======-123");
}

#[test]
fn test_ibig_in_radix() {
    assert_eq!(format!("{}", ibig!(0).in_radix(2)), "0");
    assert_eq!(format!("{}", ibig!(100).in_radix(4)), "1210");
    assert_eq!(format!("{}", ibig!(-100).in_radix(4)), "-1210");
    assert_eq!(format!("{}", ibig!(3000).in_radix(16)), "bb8");
    assert_eq!(format!("{}", ibig!(-3000).in_radix(16)), "-bb8");
    assert_eq!(format!("{:+010}", ibig!(3000).in_radix(16)), "+000000bb8");
    assert_eq!(format!("{:+010}", ibig!(-3000).in_radix(16)), "-000000bb8");
    assert_eq!(format!("{:#010}", ibig!(3000).in_radix(16)), "0000000BB8");
    assert_eq!(format!("{:#010}", ibig!(-3000).in_radix(16)), "-000000BB8");
    assert_eq!(format!("{:#010}", ibig!(-3000).in_radix(10)), "-000003000");

    assert_eq!(ibig!(0).in_radix(16).to_string(), "0");
    assert_eq!(ibig!(100).in_radix(4).to_string(), "1210");
    assert_eq!(ibig!(-100).in_radix(4).to_string(), "-1210");
    assert_eq!(ibig!(3000).in_radix(16).to_string(), "bb8");
    assert_eq!(ibig!(-3000).in_radix(16).to_string(), "-bb8");
    assert_eq!(ibig!(3000).in_radix(32).to_string(), "2to");
    assert_eq!(ibig!(-3000).in_radix(32).to_string(), "-2to");
    assert_eq!(ibig!(-1234).to_string(), "-1234");
}

#[test]
fn test_ubig_from_str_radix() {
    assert_eq!(
        UBig::from_str_radix("", 2).unwrap_err(),
        ParseError::NoDigits
    );
    assert_eq!(
        UBig::from_str_radix("+", 2).unwrap_err(),
        ParseError::NoDigits
    );
    assert_eq!(
        UBig::from_str_radix("012", 2).unwrap_err(),
        ParseError::InvalidDigit
    );
    assert_eq!(
        UBig::from_str_radix("ffffffffffffffffffffffffffffffffffffffffffffffg", 16).unwrap_err(),
        ParseError::InvalidDigit
    );
    assert_eq!(
        UBig::from_str_radix("-0", 2).unwrap_err(),
        ParseError::InvalidDigit
    );
    assert_eq!(UBig::from_str_radix("+0", 2).unwrap(), ubig!(0));
    assert_eq!(UBig::from_str_radix("0", 2).unwrap(), ubig!(0));
    assert_eq!(UBig::from_str_radix("0000000000000", 2).unwrap(), ubig!(0));
    assert_eq!(
        UBig::from_str_radix("1010110", 2).unwrap(),
        ubig!(0b1010110)
    );
    assert_eq!(UBig::from_str_radix("f1Ee", 16).unwrap(), ubig!(0xf1ee));
    assert_eq!(UBig::from_str_radix("Pp", 32).unwrap(), ubig!(825));

    assert_eq!(UBig::from_str_radix("12345", 10), Ok(ubig!(12345)));
    assert_eq!(UBig::from_str_radix("abzz", 36), Ok(ubig!(482111)));
    assert_eq!(
        UBig::from_str_radix(
            "1538958592398779500320098585338768070858734861441260196946465951498852935601537907018559511",