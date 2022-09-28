use std::collections::{VecDeque, LinkedList, HashMap};

const MAX_ITER: i32 = 300000;

fn main() {
    // Vectors
    vec_operations();

    // VecDeque
    vec_deque_operations();

    linked_list_operations();

    hash_map_operation();

    // Question answers
    /* 
        1 and 2.
        Both Vector and VecDeque are the fastest in inserting elements while VecDeque
        is the fastest in removing elements. In insertion, both Vector and VecDeque are
        filling up contiguous memory spaces from one end (in this case) and reallocate
        when the size reaches the capacity. However, in removal, we always remove the
        first element which is the worst case for Vector removal. All elements after
        the removed one would be shifted. Therefore, Vector perform wrost in removing
        elements. In contrast, in VecDeque, the end that closer to the removal point
        woulde be shifted. This means only minor of elements are shifted during removal.
        Therefore, it would have the best peformance in deletion. 
        3.
        If I need to remove element from the structure frequently, I would use VecDeque.
        4.
        When I need to make a list that is very large such that it is nearly impossible
        to fit all elements into the memory continuously.
        5.
        This result is similar to what I expected because vector is generally slow in
        deletion in most computer language.
    */
}

/// measure the insertion and removal
/// operations of a vector
fn vec_operations() {
    let mut vec = Vec::new();

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        vec.push(i);
    }
    let time_end = std::time::Instant::now();

    println!("==== Vector ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for _ in 0..MAX_ITER {
        vec.remove(0);
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}

/// measure the insertion and removal
/// operations of a VecDeque
fn vec_deque_operations() {
    let mut vec_deque = VecDeque::new();

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        vec_deque.push_back(i);
    }
    let time_end = std::time::Instant::now();

    println!("==== VecDeque ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for _ in 0..MAX_ITER {
        vec_deque.pop_front();
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}

fn linked_list_operations() {
    let mut linked_list = LinkedList::new();

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        linked_list.push_back(i);
    }
    let time_end = std::time::Instant::now();

    println!("==== LinkedList ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for _ in 0..MAX_ITER {
        linked_list.pop_front();
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}

fn hash_map_operation() {
    let mut hash_map:HashMap<i32, ()> = HashMap::new();
    
    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        hash_map.insert(i, ());
    }
    let time_end = std::time::Instant::now();

    println!("==== HashMap ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        hash_map.remove(&i);
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}