use std::cmp::Ordering;

fn bin_search(haystack: Vec<i32>, needle: i32) -> usize {
    let mut low = 0;
    let mut high = haystack.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let guess = haystack[mid];

        match needle.cmp(&guess) {
            Ordering::Equal => return mid,
            Ordering::Less => high = mid - 1,
            Ordering::Greater => low = mid + 1
        };
    };
    0
}
