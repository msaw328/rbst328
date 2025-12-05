# rbst328
Binary Search Tree implementation in Rust. For fun and to practice Rust programming.

The plan is as follows:

1) Have a working BST implementation:
    - :white_check_mark: Insertion
    - :white_check_mark: Removal
    - :white_check_mark: Search
    - Secondary operations:
        - :white_check_mark: `.clear()`
        - :white_check_mark: `.contains()`
        - :white_check_mark: `.len()`
        - :white_check_mark: `.is_empty()`
    - Iterators:
        - :white_check_mark: `.iter()`
        - :white_check_mark: `.into_iter()`
        - :x: `.iter_mut()`
2) Add Red-Black tree functionality to make it balanced
3) Add serialization and deserialization from/to bytes
4) Try sending it over a network, writing to a file or some other way of IPC/data sync
    - Perhaps a simple Redis-style key-value store with persistence to disk?
5) Optionally: make the code pretty :)
