use ibig::ubig;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

fn hash<T>(x: &T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    x.hash(&mut hasher);
    hasher.finish()
}

#[test]
fn test_hash() {
    let h = hash(&(ubig!(1) << 1000));
    for i in 0..=1000 {
        let h2 