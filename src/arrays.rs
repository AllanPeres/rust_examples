fn first<T>(slice: &[T]) -> &T {
    &slice[0]
}

fn last<T>(slice: &[T]) -> Option<&T> {
    if slice.is_empty() {
        return None;
    }
    Some(&slice[&slice.len() - 1])
}

fn find_element<T: PartialEq>(slice: &[T], target: &T) -> Option<usize> {
    for (index, element) in slice.iter().enumerate() {
        if element == target {
            return Some(index);
        }
    }
    None
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_first_in_array() {
        let array = ['a', 'b', 'c'];
        assert_eq!('a', *first(&array));
    }

    #[test]
    fn test_last_in_array() {
        let array = ['a', 'b', 'c'];
        assert_eq!('c', *last(&array).unwrap());
    }

    #[test]
    fn test_find_in_middle_of_array() {
        let array = ['a', 'b', 'c'];
        assert_eq!(1, find_element(&array, &'b').unwrap());
    }

    #[test]
    fn test_dont_find_in_array() {
        let array = ['a', 'b', 'c'];
        assert_eq!(None, find_element(&array, &'d'));
    }

    #[test]
    fn for_in_array() {
        let array = [1, 2, 3, 4, 5];
        let mut sum = 0;
        for element in &array {
            sum += *element;
        }

        assert_eq!(15, sum);
    }
}