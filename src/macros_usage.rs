use super::objects::bit_set::BitSet;

struct Value {
    x: i32
}

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

macro_rules! op {
    (+ $_self:ident : $self_type:ty, $other:ident $expr:expr) => {
        impl ::std::ops::Add for $self_type {
            type Output = $self_type;

            fn add($_self, $other: $self_type) -> $self_type {
                $expr
            }
        }
    };

    (- $_self:ident : $self_type:ty, $other:ident $expr:expr) => {
        impl ::std::ops::Sub for $self_type {
            type Output = $self_type;

            fn sub($_self, $other: $self_type) -> $self_type {
                $expr
            }
        }
    };
}

op!(+ self:Value, other {
    Value {
        x: self.x + other.x
    }
});

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

    #[test]
    fn test_add_pointers() {
        let p1 = Value{ x: 1 };
        let p2 = Value{ x: 3 };
        let p3 = p1 + p2;
        assert_eq!(p3.x, 4);
    }
}