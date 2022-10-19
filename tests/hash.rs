use ibig::ubig;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

fn hash<T>(x: &T) -> u