fn uppercase_a_to_h(c: char) -> char {
    match c {
        'a'..= 'h' => ((c as u8) - 32) as char,
        _ => c
    }
}

fn is_alphanumeric(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z'| '0'..='9' => true,
        _ => false
    }
}

pub fn run() {
    println!("{}", b'g');
    println!("{}", (uppercase_a_to_h('g') as char));
    println!("3 is alphanumeric = {}", is_alphanumeric('3'));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_uppercase_with_matcher() {
        let small_a = 'a';
        assert_eq!('A', uppercase_a_to_h(small_a));
        let small_g = 'g';
        assert_eq!('G', uppercase_a_to_h(small_g));
        let small_i = 'i';
        assert_eq!('i', uppercase_a_to_h(small_i));
    }
}