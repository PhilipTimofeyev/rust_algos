

fn selection_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let mut sorted: Vec<i32> = Vec::with_capacity(arr.len());

    for _i in 0..arr.len() {
        let smallest = smallest_value(&arr[..]);
        sorted.push(arr.remove(smallest));
    }

    sorted
}

fn smallest_value(arr: &[i32]) -> usize {
    let mut smallest = (0, &arr[0]);

    for (i, element) in arr.iter().enumerate() {
        if *element < *smallest.1 {
            smallest.1 = element;
            smallest.0 = i;
        }
    }
    
    smallest.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_descending() {
        let arr = vec![6, 5, 4, 3, 2, 1];
        let result = selection_sort(arr);

        assert_eq!(result, vec![1, 2, 3, 4, 5, 6])
    }

    #[test]
    fn sorts_random() {
        let arr = vec![3,5,2,3,1,7];
        let result = selection_sort(arr);

        assert_eq!(result, vec![1, 2, 3, 3, 5, 7])
    }

    #[test]
    fn empty() {
        let arr = vec![];
        let result = selection_sort(arr);

        assert_eq!(result, vec![])
    }
    
    #[test]
    fn sorts_negative() {
        let arr = vec![4, 3, 2, 1, -1, -100];
        let result = selection_sort(arr);

        assert_eq!(result, vec![-100, -1, 1, 2, 3, 4])
    }
}
