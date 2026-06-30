use std::collections::HashMap;

/*
Arrays

Create the following arrays

unsorted
5, 8, 4, 6, 2, 9, 1, 3, 7

sorted
1, 2, 3, 4, 5, 6, 7, 8, 9
*/

pub fn array_one() -> Vec<i32> {
    vec![5, 8, 4, 6, 2, 9, 1, 3, 7]
}

pub fn sorted_array_one() -> Vec<i32> {
    vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
}

/*
Trees
*/

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

/*
Create the following BST

        7
    3       11
1     4  9      13
 2
*/

pub fn bst_one_root() -> Option<Box<TreeNode>> {
    let mut root = Box::new(TreeNode::new(7));

    let mut left = Box::new(TreeNode::new(3));
    let mut left_left = Box::new(TreeNode::new(1));
    left_left.right = Some(Box::new(TreeNode::new(2)));
    left.left = Some(left_left);
    left.right = Some(Box::new(TreeNode::new(4)));

    let mut right = Box::new(TreeNode::new(11));
    right.left = Some(Box::new(TreeNode::new(9)));
    right.right = Some(Box::new(TreeNode::new(13)));

    root.left = Some(left);
    root.right = Some(right);

    Some(root)
}

/*
Graphs

Create the following Directed Graphs

dagOne
0 -> 4, 5
1 -> 2, 5
2 -> 3
3 -> 4
4 ->
5 ->

dagTwo
0 -> 1
1 -> 2
2 -> 3
3 -> 4
4 -> 5
5 ->

dgWCycle
0 -> 1
1 -> 2
2 -> 3, 4
3 ->
4 -> 5
5 -> 2
*/

pub type Graph = HashMap<i32, Vec<i32>>;

pub fn dag_one() -> Graph {
    HashMap::from([
        (0, vec![4, 5]),
        (1, vec![2, 5]),
        (2, vec![3]),
        (3, vec![4]),
        (4, vec![]),
        (5, vec![]),
    ])
}

pub fn dag_two() -> Graph {
    HashMap::from([
        (0, vec![1]),
        (1, vec![2]),
        (2, vec![3]),
        (3, vec![4]),
        (4, vec![5]),
        (5, vec![]),
    ])
}

pub fn directed_graph_with_cycle() -> Graph {
    HashMap::from([
        (0, vec![1]),
        (1, vec![2]),
        (2, vec![3, 4]),
        (3, vec![]),
        (4, vec![5]),
        (5, vec![2]),
    ])
}

/*
Create the following Undirected Graphs

ugOne
0 -> 1
1 -> 0, 2
2 -> 1, 3
3 -> 2, 4
4 -> 3

ugWCycle
0 -> 1
1 -> 0, 2
2 -> 1, 3, 4, 5
3 -> 2
4 -> 2, 5
5 -> 2, 4
*/

pub fn undirected_graph_one() -> Graph {
    HashMap::from([
        (0, vec![1]),
        (1, vec![0, 2]),
        (2, vec![1, 3]),
        (3, vec![2, 4]),
        (4, vec![3]),
    ])
}

pub fn undirected_graph_with_cycle() -> Graph {
    HashMap::from([
        (0, vec![1]),
        (1, vec![0, 2]),
        (2, vec![1, 3, 4, 5]),
        (3, vec![2]),
        (4, vec![2, 5]),
        (5, vec![2, 4]),
    ])
}

/*
Linked Lists
*/

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingleLLNode {
    pub val: i32,
    pub next: Option<Box<SingleLLNode>>,
}

impl SingleLLNode {
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoubleLLNode {
    pub val: i32,
    pub next: Option<Box<DoubleLLNode>>,
    // A fully-owned doubly linked list in Rust is usually built with Rc<RefCell<T>>
    // or raw pointers. Keep this placeholder simple for interview practice.
}

impl DoubleLLNode {
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

/*
Create the following singly linked list

4 -> 7 -> 3 -> 9 -> 1 -> 8
*/

pub fn sll_one_head() -> Option<Box<SingleLLNode>> {
    linked_list_from_slice(&[4, 7, 3, 9, 1, 8])
}

/*
Utils
*/

pub fn linked_list_from_slice(values: &[i32]) -> Option<Box<SingleLLNode>> {
    let mut head = None;

    for &value in values.iter().rev() {
        let mut node = Box::new(SingleLLNode::new(value));
        node.next = head;
        head = Some(node);
    }

    head
}

pub fn linked_list_to_vec(head: &Option<Box<SingleLLNode>>) -> Vec<i32> {
    let mut values = Vec::new();
    let mut current = head.as_ref();

    while let Some(node) = current {
        values.push(node.val);
        current = node.next.as_ref();
    }

    values
}

pub fn print_linked_list(head: &Option<Box<SingleLLNode>>) {
    let values: Vec<String> = linked_list_to_vec(head)
        .iter()
        .map(|value| value.to_string())
        .collect();

    println!("{}", values.join(" -> "));
}
