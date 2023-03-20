/// Insertion Sort
///
/// Insertion sort is a simple comparison-based sorting algorithm. It works by iterating through
/// the input list and maintaining a sorted sublist in the same data structure. For each element,
/// it finds the appropriate position within the sorted sublist and inserts it, shifting the
/// remaining elements to the right.
///
/// # Example
///
/// ```
/// use algo_ds::algorithms::sort::insertion_sort;
///
/// let mut data = vec![5, 3, 1, 4, 2];
/// insertion_sort(&mut data);
/// assert_eq!(data, vec![1, 2, 3, 4, 5]);
/// ```
///
/// # Performance
///
/// - Worst-case time complexity: O(n^2)
/// - Average-case time complexity: O(n^2)
/// - Best-case time complexity: O(n)
/// - Space complexity: O(1)
///
/// Insertion sort performs well for small lists or partially sorted data but is inefficient for
/// larger data sets.
pub fn insertion_sort<T>(data: &mut [T])
where
    T: Ord + Copy,
{
    for i in 1..data.len() {
        let key = data[i];
        let mut j = i;

        while j > 0 && data[j - 1] > key {
            data[j] = data[j - 1];
            j -= 1;
        }

        data[j] = key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut data: Vec<i32> = vec![];
        insertion_sort(&mut data);
        assert_eq!(data, vec![]);
    }

    #[test]
    fn single_element() {
        let mut data = vec![1];
        insertion_sort(&mut data);
        assert_eq!(data, vec![1]);
    }

    #[test]
    fn sorted() {
        let mut data = vec![1, 2, 3, 4, 5];
        insertion_sort(&mut data);
        assert_eq!(data, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn reverse_sorted() {
        let mut data = vec![5, 4, 3, 2, 1];
        insertion_sort(&mut data);
        assert_eq!(data, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn random() {
        let mut data = vec![5, 3, 1, 4, 2];
        insertion_sort(&mut data);
        assert_eq!(data, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn duplicates() {
        let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        insertion_sort(&mut data);
        assert_eq!(data, vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
    }

    #[test]
    fn strings() {
        let mut data = vec!["orange", "apple", "banana", "grape", "kiwi"];
        insertion_sort(&mut data);
        assert_eq!(data, vec!["apple", "banana", "grape", "kiwi", "orange"]);
    }

    #[test]
    fn negative_numbers() {
        let mut data = vec![-5, 3, -1, 4, -2];
        insertion_sort(&mut data);
        assert_eq!(data, vec![-5, -2, -1, 3, 4]);
    }

    #[test]
    fn mixed_numbers() {
        let mut data = vec![0, -3, 2, -5, 4, 1];
        insertion_sort(&mut data);
        assert_eq!(data, vec![-5, -3, 0, 1, 2, 4]);
    }

    #[test]
    fn same_element() {
        let mut data = vec![42, 42, 42, 42, 42];
        insertion_sort(&mut data);
        assert_eq!(data, vec![42, 42, 42, 42, 42]);
    }

    #[test]
    fn large_input() {
        let mut data: Vec<i32> = (0..1000).rev().collect();
        insertion_sort(&mut data);
        let expected: Vec<i32> = (0..1000).collect();
        assert_eq!(data, expected);
    }
}
