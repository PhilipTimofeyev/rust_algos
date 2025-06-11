use std::cmp::Ordering;

#[allow(dead_code)]
fn factorial(n: i32) -> i32 {
    if n == 0 { 1 }
    else { n * factorial(n-1) }
}

#[allow(dead_code)]
fn count_list(arr: &[i32]) -> i32 {
    if arr.is_empty() { 0 }
    else {
        1 + count_list(&arr[1..])
    } 
}

#[allow(dead_code)]
fn max(arr: &[i32]) -> i32 {
    if arr.len() == 2 {
        if arr[0] > arr[1] {arr[0]}
        else {arr[1]}
    } else { 
        let sub_max = max(&arr[1..]);
        if arr[0] > sub_max {
            arr[0]
        } else { sub_max }
    }
}

#[allow(dead_code)]
fn binary_search(haystack: &[i32], needle: i32) -> usize {
    let low = 0;
    let high = haystack.len() - 1;
    let mid = (high + low) / 2;
    let guess = haystack[mid];

    match guess.cmp(&needle) {
        Ordering::Equal => mid,
        Ordering::Greater => binary_search(&haystack[low..mid - 1], needle),
        Ordering::Less => binary_search(&haystack[mid + 1..high], needle)
    } 
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn factorial_works() {
        let result = factorial(5);
        assert_eq!(result, 120)
    }
    
    #[test]
    fn count_works() {
        let result = count_list(&[1, 2, 3, 4]);
        assert_eq!(result, 4)
    }

    #[test]
    fn max_works() {
        let result = max(&[3, 2, 4, 9]);
        assert_eq!(result, 9)
    }

    #[test]
    fn binary_search_works() {
        let result = binary_search(&[1, 2, 3, 4, 5, 6, 7], 4);
        assert_eq!(result, 3)
    }
}

