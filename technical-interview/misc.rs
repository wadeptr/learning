mod structures;

use std::collections::{HashMap, HashSet, VecDeque};

use structures::{
    array_one, bst_one_root, dag_one, dag_two, directed_graph_with_cycle, print_linked_list,
    sorted_array_one, sll_one_head, undirected_graph_one, undirected_graph_with_cycle,
    SingleLLNode,
};

/*
Practice file for miscellaneous data structures and algorithms problems.

Run this file from technical-interview with:
  rustc misc.rs && ./misc

array_one
5, 8, 4, 6, 2, 9, 1, 3, 7

sorted_array_one
1, 2, 3, 4, 5, 6, 7, 8, 9
*/

fn reverse(mut head: Option<Box<SingleLLNode>>) -> Option<Box<SingleLLNode>> {
    let mut prev = None;

    while let Some(mut node) = head {
        let next = node.next.take();
        node.next = prev;
        prev = Some(node);
        head = next;
    }

    prev
}

fn reverse_recursive(head: Option<Box<SingleLLNode>>) -> Option<Box<SingleLLNode>> {
    fn go(
        current: Option<Box<SingleLLNode>>,
        prev: Option<Box<SingleLLNode>>,
    ) -> Option<Box<SingleLLNode>> {
        match current {
            None => prev,
            Some(mut node) => {
                let next = node.next.take();
                node.next = prev;
                go(next, Some(node))
            }
        }
    }

    go(head, None)
}

fn main() {
    let array = array_one();
    let sorted = sorted_array_one();
    println!("array_one: {:?}", array);
    println!("sorted_array_one: {:?}", sorted);

    let list = sll_one_head();
    println!("original:");
    print_linked_list(&list);

    let reversed_iterative = reverse(list.clone());
    println!("reversed iterative:");
    print_linked_list(&reversed_iterative);

    let reversed_recursive = reverse_recursive(list);
    println!("reversed recursive:");
    print_linked_list(&reversed_recursive);

    let _bst = bst_one_root();

    let _dag_one = dag_one();
    let _dag_two = dag_two();
    let _dg_with_cycle = directed_graph_with_cycle();
    let _ug_one = undirected_graph_one();
    let _ug_with_cycle = undirected_graph_with_cycle();

    // Scratch imports for exercises:
    // HashMap / HashSet: graph traversal, caches, hash tables
    // VecDeque: queues and BFS
    let _scratch_map: HashMap<i32, i32> = HashMap::new();
    let _scratch_set: HashSet<i32> = HashSet::new();
    let _scratch_queue: VecDeque<i32> = VecDeque::new();
}
