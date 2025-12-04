# rbst328
Binary Search Tree implementation in Rust. For fun and to practice Rust programming.

The plan is as follows:

1) Have a working BST implementation:
    - :white_check_mark: Insertion
    - :white_check_mark: Removal
    - :white_check_mark: Search
    - :white_check_mark: Secondary operations: `.clear()`, `.contains()`
    - :x: Iterator(s)
2) Add Red-Black tree functionality to make it balanced
3) Add serialization and deserialization from/to bytes
4) Try sending it over a network, writing to a file or some other way of IPC/data sync
    - Perhaps a simple Redis-style key-value store with persistence to disk?
5) Optionally: make the code pretty :)