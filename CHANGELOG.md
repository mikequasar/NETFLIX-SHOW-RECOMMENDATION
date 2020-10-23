
# Changelog

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.3.6 - 2022-09-18

### Features
* GCD, greatest common divisor.
* Extended GCD (GCD with Bézout coefficients).
* Modular inverse.
* Modular division.

## 0.3.5 - 2022-03-12

### Bugfixes
* Corrected too-strict lifetimes in modular exponentiation.

### Dependencies
* Removed the dependency on `const_fn_assert`.

## 0.3.4 - 2021-11-03

### Features
* Optional `serde` support for `UBig` and `IBig`.

### Toolchain
* Rust 1.49+ is now required.

### Dependencies
* Added an optional dependency on `serde`.

## 0.3.3 - 2021-10-28

### Features
* Mixed-type arithmetic with primitive integer types.

  Allows `x + 1` instead of `x + ubig!(1)`.
  
  This breaks with the convention that arithmetic operators require same type on both sides. A better alternative would be user-defined custom integer literals, so that `1` could be inferred to have type `UBig`. But Rust does not support this yet. So this is a workaround for the sake of ergonomics.

## 0.3.2 - 2021-05-02

### Toolchain
* Rust 1.47+ is now supported.

### Dependencies
* Added a dependency on `cfg-if`.

## 0.3.1 - 2021-04-03

### Features
* Maximum supported length in bits: `UBig::MAX_BIT_LEN`.

### Fixes
* Broken build for `aarch64`, `mips64` and` powerpc64` fixed.

### Dependencies
* Added a dependency on `const_fn_assert`.

## 0.3.0 - 2021-03-29