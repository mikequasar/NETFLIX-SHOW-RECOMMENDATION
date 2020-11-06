
use ibig::UBig;

// a * (a+1) * ... * (b-1)
fn product(a: u32, b: u32) -> UBig {
    if b == a + 1 {