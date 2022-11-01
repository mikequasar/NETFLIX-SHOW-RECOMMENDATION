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
        "         13670011064254