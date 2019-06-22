use super::traits::BitSet;

macro_rules! int_bitset {
    ($ty: ty) => {
        impl BitSet for $ty {
            fn clear(&mut self, index: usize) {
                *self >>= (index) as $ty;
            }

            fn is_set(&self, index: usize) -> bool {
                (*self >> index as $ty) & 1 == 1
            }

            fn set(&mut self, index: usize) {
                *self |= (1 << index) as $ty;
            }
        }
    };
}

int_bitset!(i32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_value() {
        let mut num:i32 = 0;
        num.set(20);
        assert_eq!(num, 1048576);
        assert!(num.is_set(20));
        num.clear(10);
        assert_eq!(num, 1024);
        assert!(num.is_set(10));
    }
}