#[allow(dead_code)]
fn quicksort(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        arr
    } else {
        let pivot = arr[0];
        let mut less = Vec::new();
        let mut greater = Vec::new();

        for &i in &arr[1..] {
            if i <= pivot {
                less.push(i)
            } else {
                greater.push(i)
            }
        }

        [
            quicksort(less).as_slice(),
            [pivot].as_slice(),
            quicksort(greater).as_slice(),
        ]
        .concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quicksort_words() {
        let result = quicksort(vec![3, 2, 4, 1]);
        assert_eq!(result, vec![1, 2, 3, 4])
    }
}
