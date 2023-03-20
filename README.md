# algo_ds
> ⚠️ Work in Progress: This crate is currently under development and some features may be incomplete or subject to change. Please use with caution and report any issues you encounter. ⚠️

`algo_ds` is a Rust crate implementing a collection of fundamental data structures and algorithms. The primary goal of this crate is to provide efficient, well-tested, and easy-to-use implementations for educational purposes and practical use in Rust projects.

## Features
- Data Structures:
    - Linked List
    - Stack
    - Queue
    - Binary Tree
    - Graph
    - Hash Map
    - Heap
- Algorithms:
    - Search
        - Linear Search
        - Binary Search
    - Sorting
        - Bubble Sort
        - Selection Sort
        - Insertion Sort
        - Merge Sort
        - Quick Sort
        - Heap Sort

## Usage
Add the following to your Cargo.toml:

```toml
Copy code
[dependencies]
algo_ds = "0.1.0"
```
Then, you can use the data structures and algorithms in your Rust code:

```rust
Copy code
use algo_ds::data_structures::linked_list::LinkedList;
use algo_ds::algorithms::sorting::insertion_sort;

fn main() {
    // Example of using LinkedList
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    println!("{:?}", list);

    // Example of using Insertion Sort
    let mut data = vec![5, 3, 1, 4, 2];
    insertion_sort(&mut data);
    println!("{:?}", data);
}
```

## Contributing
Contributions to algo_ds are welcome! Please open an issue or submit a pull request on GitHub if you would like to propose new features, report bugs, or improve the existing code.

## License
This project is licensed under the MIT License. See the LICENSE file for more information.