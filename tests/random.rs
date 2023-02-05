use ibig::{ibig, ops::DivRem, ubig, UBig};
use rand::{distributions::uniform::Uniform, prelude::*};

#[test]
fn test_uniform_ubig() {
    let mut rng = StdRng::seed_from_u64(1);

    let distr = Uniform::from(ubig!(3)..ubig!(7));
    let x = (&mut rng).sample_iter(&distr).take(1000).min().u