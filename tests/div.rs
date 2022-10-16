
use ibig::{
    ibig,
    ops::{DivEuclid, DivRem, DivRemEuclid, RemEuclid},
    ubig, IBig,
};

#[test]
fn test_div_rem_ubig() {
    let test_cases = [
        (ubig!(331), ubig!(10), ubig!(33), ubig!(1)),
        (ubig!(17),
         ubig!(_0x987987123984798abbcc213789723948792138479837492837498cc),
         ubig!(0),
         ubig!(17)
        ),
        (ubig!(_0x987987123984798abbcc213789723948792138479837492837498cc),
         ubig!(_0x1234),
         ubig!(_0x86054c502f0a4e43e2d0de91f1029d251ce67bbdb88dc3edbb40),
         ubig!(_0xfcc)
        ),
        (ubig!(_0x987987123984798abbcc213789723948792138479837492837498cc),
         ubig!(_0x1000),
         ubig!(_0x987987123984798abbcc21378972394879213847983749283749),
         ubig!(_0x8cc)
        ),
        (ubig!(_0x987987123984798abbcc213789723948792138479837492837498cc),
         ubig!(_0xf234567812345678),
         ubig!(_0xa128cfb49d0d746cc0295e163c343aafbffbfa8),
         ubig!(_0x9068d997bb10520c)
        ),
        (ubig!(_0x987987123984798abbcc213789723948792138479837492837498ce),
         ubig!(_0x987987123984798abbcc213789723948792138479837492837498cc),
         ubig!(1),
         ubig!(2),
        ),
        // Special case for division (64-bit words): top 2 / top 1 overflows.
        (
         ubig!(_0xffffffffffffffffffffffffffffffff00000000000000000000000000000000),
         ubig!(_0xffffffffffffffffffffffffffffffff0000000000000001),
         ubig!(_0xffffffffffffffff),
         ubig!(_0xfffffffffffffffffffffffffffffffe0000000000000001),
        ),
        // Random 500-bit by random 250-bit.
        (
         ubig!(_0x2b8f1bb75f1ca5bf3400549a663d503d298da7f53942cd3c5c6a1bc50598d091e8ca30896413783e9b001572e28808c4dc9598bdd17ef3ce35b40e0368b60),
         ubig!(_0x3e880309f5e48d145337aae47694a74f2860db8e49665f03978f1b11665dc80),
         ubig!(_0xb254145d6f736c22ed5fca6a41f4c883a59fc32c638710758bb50fa532b31f),
         ubig!(_0x195afda8e35e347c65ed01409c73d1c820ed78a87e83cf6cfdad1a25fb357e0),
        ),
        (
         ubig!(_0x3e880309f5e48d145337aae47694a74f2860db8e49665f03978f1b11665dc80),
         ubig!(_0x2b8f1bb75f1ca5bf3400549a663d503d298da7f53942cd3c5c6a1bc50598d091e8ca30896413783e9b001572e28808c4dc9598bdd17ef3ce35b40e0368b60),
         ubig!(0),
         ubig!(_0x3e880309f5e48d145337aae47694a74f2860db8e49665f03978f1b11665dc80),
        ),
        // 3^300 - 1 by 3^150
        (
         ubig!(_0xb39cfff485a5dbf4d6aae030b91bfb0ec6bba389cd8d7f85bba3985c19c5e24e40c543a123c6e028a873e9e3874e1b4623a44be39b34e67dc5c2670),
         ubig!(_0x359ba2b98ca11d6864a331b45ae7114c01ffbdcf60cc16e692fb63c6e219),
         ubig!(_0x359ba2b98ca11d6864a331b45ae7114c01ffbdcf60cc16e692fb63c6e218),
         ubig!(_0x359ba2b98ca11d6864a331b45ae7114c01ffbdcf60cc16e692fb63c6e218),
        ),
        // 7^70-1 by 7^35
        (
         ubig!(_0x16dc8782276b9f7addf9768f33c8007ce903866a4546c1a190),
         ubig!(_0x4c8077a58a0a8cb7c24960e57),
         ubig!(_0x4c8077a58a0a8cb7c24960e56),
         ubig!(_0x4c8077a58a0a8cb7c24960e56),
        ),
        // 2^20480-1 by 2^5120-1
        (
            (ubig!(1) << 20480) - ubig!(1),
            (ubig!(1) << 5120) - ubig!(1),
            ubig!(1) + (ubig!(1) << 5120) + (ubig!(1) << 10240) + (ubig!(1) << 15360),
            ubig!(0)
        ),
        // 2^20480-1 by 2^15360-1
        (
            (ubig!(1) << 20480) - ubig!(1),
            (ubig!(1) << 15360) - ubig!(1),
            ubig!(1) << 5120,
            (ubig!(1) << 5120) - ubig!(1)
        ),
        // 2^19000-1 by 2^5000-1
        (
            (ubig!(1) << 19000) - ubig!(1),
            (ubig!(1) << 5000) - ubig!(1),
            (ubig!(1) << 14000) + (ubig!(1) << 9000) + (ubig!(1) << 4000),
            (ubig!(1) << 4000) - ubig!(1)