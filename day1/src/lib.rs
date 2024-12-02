pub fn quicksort<T: Ord>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }

    let pivot = partition(array);

    quicksort(&mut array[..pivot]);
    quicksort(&mut array[pivot..]);
}

fn partition<T: Ord>(array: &mut [T]) -> usize {
    let pivot = array.len() - 1;
    let mut pivot_index = 0;

    for j in 0..pivot {
        if array[j] <= array[pivot] {
            array.swap(pivot_index, j);
            pivot_index += 1;
        }
    }

    array.swap(pivot_index, pivot);

    pivot_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_array() {
        let mut arr_1 = vec![5, 4, 3, 2, 1];
        let expected = vec![1, 2, 3, 4, 5];

        quicksort(&mut arr_1);

        assert_eq!(arr_1, expected);
    }

    fn sort_array_2() {
        let mut arr_2 = vec![3, 5, 2, 4, 1];
        let expected = vec![1, 2, 3, 4, 5];

        quicksort(&mut arr_2);

        assert_eq!(arr_2, expected);
    }
}