
/// Implement impl Op<B> for &A by forwarding to impl Op<B> for A.
/// Includes &B.
macro_rules! forward_binop_first_arg_by_value {
    (impl $tr:ident<$t2:ty> for $t1:ty, $f:ident) => {
        impl $tr<$t2> for &$t1 {
            type Output = <$t1 as $tr<$t2>>::Output;

            #[inline]
            fn $f(self, rhs: $t2) -> Self::Output {
                (*self).$f(rhs)
            }
        }

        impl<'a> $tr<&'a $t2> for &$t1 {
            type Output = <$t1 as $tr<&'a $t2>>::Output;

            #[inline]
            fn $f(self, rhs: &$t2) -> Self::Output {
                (*self).$f(rhs)
            }
        }
    };
}

/// Implement impl Op<&B> for A by forwarding to impl Op<B> for A.
/// Includes &A.
macro_rules! forward_binop_second_arg_by_value {
    (impl $tr:ident<$t2:ty> for $t1:ty, $f:ident) => {
        impl $tr<&$t2> for $t1 {
            type Output = <$t1 as $tr<$t2>>::Output;