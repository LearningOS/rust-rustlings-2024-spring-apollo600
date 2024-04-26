/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + std::fmt::Display,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        if let Some(root) = &mut self.root {
            root.insert(value);
        } else {
            println!("insert {}", value);
            self.root = Some(Box::new(TreeNode::new(value)));
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        let mut root: &Box<TreeNode<T>>;
        match &self.root {
            None => {
                return false;
            },
            Some(node) => {
                root = node;
            }
        }
        loop {
            if value < root.value {
                if let Some(node) = &root.left {
                    root = node;
                } else {
                    return false;
                }
            } else if value > root.value {
                if let Some(node) = &root.right {
                    root = node;
                } else {
                    return false;
                }
            } else {
                return true;
            }
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord + std::fmt::Display,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        // recursion
        if value < self.value {
            match &mut self.left {
                None => {
                    println!("insert {}", value);
                    let new_node = Box::new(TreeNode::new(value));
                    self.left = Some(new_node);
                },
                Some(left) => {
                    println!("goto left {}", self.value);
                    left.insert(value);
                }
            }
        } else if value > self.value {
            match &mut self.right {
                None => {
                    let new_node = Box::new(TreeNode::new(value));
                    self.right = Some(new_node);
                },
                Some(right) => {
                    right.insert(value);
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


