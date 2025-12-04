use rbst328::BSTMap;

fn main() {
    let mut bst = BSTMap::<i32, &str>::new();
    bst.insert(10, "test");
    bst.insert(-25, "test2");
    bst.insert(4, "hello");
    bst.insert(3, "hi");
    bst.insert(15, "is it working");
    bst.insert(-30, "negatives");
    bst.insert(-29, "negatives2");
    bst.insert(102, "test");
    bst.insert(15, "123");
    bst.insert(30, "strings as values");
    bst.insert(2, "heyhy");
    bst.insert(16, "utf8żółć");

    println!("PRINTING TREE");
    bst.pretty_print();

    let test1 = 15;
    println!("BST has {} {}", test1, bst.contains(test1));

    let test2 = 100;
    println!("BST has {} {}", test2, bst.contains(test2));

    bst.remove(10);
    bst.remove(4);

    println!("PRINTING TREE AFTER REMOVAL");
    bst.pretty_print();

    {
        let bst_ref = bst.get_mut(16);

        if bst_ref.is_some() {
            let bst_ref = bst_ref.unwrap();
            *bst_ref = "utf8żółćmodified";
        }
    }

    println!("PRINTING TREE AFTER get_mut()");
    bst.pretty_print();
}
