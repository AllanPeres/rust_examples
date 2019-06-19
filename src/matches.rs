fn uppercase(c: u8) -> u8 {
    match c {
        b'a'...b'h' => c - 32,
        _ => c
    }
}

fn is_alphanumeric(c: char) -> bool {
    match c {
        'a'...'z' | 'A'...'Z'| '0'...'9' => true,
        _ => false
    }
}

pub fn run() {
    println!("{}", b'g');
    println!("{}", (uppercase(b'g') as char));
    println!("3 is alphanumeric = {}", is_alphanumeric('3'));
}