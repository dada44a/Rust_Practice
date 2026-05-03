mod ArrayList;
mod ListADT;
mod linked_list;

// Import the types you need
use ArrayList::ArrList;
use ListADT::{List, RandomAccessList, SequentialList};
use linked_list::{LinkedList};


fn main() {
    // Create a new ArrayList
    // let mut list = ArrList::<i32>::new();

    // // Test List trait methods
    // list.push(10);
    // list.push(20);
    // list.push(30);

    // println!("=== Testing List ADT ===");
    // println!("Length: {}", list.len());
    // println!("Is empty: {}", list.is_empty());
    // println!("Pop: {:?}", list.pop()); // Should be 30

    // // Test RandomAccessList methods
    // println!("\n=== Testing RandomAccessList ADT ===");
    // list.push(40);
    // list.push(50);

    // println!("Get index 1: {:?}", list.get(1));

    // let old = list.set(1, 99);
    // println!("Set returned: {:?}", old);
    // println!("After set, index 1: {:?}", list.get(1));

    // list.insert_at(1, 42);
    // println!("After insert, index 1: {:?}", list.get(1));

    // let removed = list.remove_at(2);
    // println!("Removed: {:?}", removed);

    // // Show final list
    // println!("\n=== Final List ===");
    // for i in 0..list.len() {
    //     println!("list[{}] = {:?}", i, list.get(i));
    // }

    let mut list = LinkedList::<i32>::new();

        // Test List trait
    list.push(10);
    list.push(20);
    list.push(30);
    list.push(40);

    println!("Initial list (cursor at 0): {:?}", list.current()); // Some(10)

        // Test cursor movement
    list.move_next();
    println!("After move_next: {:?}", list.current()); // Some(20)

    list.move_next();
    println!("After move_next: {:?}", list.current()); // Some(30)

        // Test insert at cursor
    list.insert_at_cursor(99);
    println!("After insert at cursor: {:?}", list.current()); // Some(99)

        // Test remove at cursor
    let removed = list.remove_at_cursor();
    println!("Removed: {:?}", removed); // Some(99)
    println!("Cursor after remove: {:?}", list.current()); // Some(30)

        // Test move_prev
    list.move_prev();
    println!("After move_prev: {:?}", list.current()); // Some(20)

        // Test pop
    println!("Pop: {:?}", list.pop()); // Some(10)
    println!("Cursor after pop: {:?}", list.current()); // Some(20)

        // Test clear
    list.clear();
    println!("After clear, is_empty: {:?}", list.is_empty()); // true
    println!("Cursor after clear: {:?}", list.current()); // None

}
