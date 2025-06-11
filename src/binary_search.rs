use std::cmp::Ordering;

pub fn binary_search(haystack: Vec<i32>, needle: i32) -> i32 {
    let mut low: i32 = 0;
    let mut high = (haystack.len() - 1) as i32;

    while low <= high {
        let mid: i32 = (low + high) / 2;
        let guess = haystack[mid as usize];
        
        match needle.cmp(&guess) {
            Ordering::Equal => return mid,
            Ordering::Greater => low = mid + 1,
            Ordering::Less => high = mid - 1
        };
    };
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smaller_value() {
        let haystack = vec![1, 2, 3, 4, 5];
        let needle = 1;

        let result = binary_search(haystack, needle);
        assert_eq!(result, 0);
    }

    #[test]
    fn larger_value() {
        let haystack = vec![1, 2, 3, 4, 5];
        let needle = 5;

        let result = binary_search(haystack, needle);
        assert_eq!(result, 4);
    }

    #[test]
    fn mid_value() {
        let haystack = vec![1, 2, 3, 4, 5];
        let needle = 3;

        let result = binary_search(haystack, needle);
        assert_eq!(result, 2);
    }    
    
    #[test]
    fn null_value() {
        let haystack = vec![1, 2, 3, 4, 5];
        let needle = -2;

        let result = binary_search(haystack, needle);
        assert_eq!(result, -1);
    }    
}
