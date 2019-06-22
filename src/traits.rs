use super::objects::bit_set::BitSet;

impl BitSet for u64 {
    fn clear(&mut self, index: usize) {
        *self >>= (index) as u64;
    }

    fn is_set(&self, index: usize) -> bool {
        (*self >> index as u64) & 1 == 1
    }

    fn set(&mut self, index: usize) {
        *self |= (1 << index) as u64;
    }

    fn toggle(&mut self, index: usize) {
        *self ^= (1 << index) as u64;
    }
}

pub fn run() {
    let mut num = 0;
    let test_num = 30;
    num.set(test_num);
    println!("{}", num.is_set(test_num));
    num.clear(test_num);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_set_value() {
        let mut num = 0;
        num.set(20);
        assert_eq!(num, 1048576);
        assert!(num.is_set(20));
        num.clear(10);
        assert_eq!(num, 1024);
        assert!(num.is_set(10));
    }

    #[test]
    fn test_toggle_value() {
        let mut num = 0;
        num.toggle(20);
        assert_eq!(num, 1048576);
        num.toggle(20);
        assert_eq!(num, 1);
        num.toggle(20);
        assert_eq!(num, 1048577);
    }

}