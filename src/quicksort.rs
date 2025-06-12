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

#[allow(dead_code)]
fn quicksort_partition(arr: &mut[i32]) -> usize {
    let mut low = 0;
    let pivot = arr.len() - 1;

    for i in 0..pivot {
        if arr[i] <= arr[pivot] {
            arr.swap(i, low);
            low += 1;
        }
    }

    arr.swap(pivot, low);
    low

}

#[allow(dead_code)]
fn quicksort_2(arr: &mut[i32]) {
    if arr.len() > 1 {
        let pivot = quicksort_partition(arr);
        quicksort_2(&mut arr[..pivot]);
        quicksort_2(&mut arr[pivot + 1..]);
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quicksort_works() {
        let result = quicksort(vec![3, 2, 4, 1]);
        assert_eq!(result, vec![1, 2, 3, 4])
    }

    #[test]
    fn partition() {
        let result = quicksort_partition(&mut [3, 2, 4, 1]);
        assert_eq!(result, 0)
    }
    
    #[test]
    fn quicksort_2_works() {
        let mut arr = [3, 2, 4, 1];
        quicksort_2(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4])
    }
}
