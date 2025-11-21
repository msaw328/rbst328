use rbst328::BST;

fn main() {
    let mut bst = BST::<i32>::new();
    bst.insert(10);
    bst.insert(25);
    bst.insert(4);
    bst.insert(3);
    bst.insert(15);
    bst.insert(102);
    bst.insert(15);
    bst.insert(30);
    bst.insert(2);
    bst.insert(16);

    println!("PRINTING TREE");
    bst.pretty_print();

    let test1 = 15;
    println!("BST has {} {}", test1, bst.contains(test1));

    let test2 = 100;
    println!("BST has {} {}", test2, bst.contains(test2));
}