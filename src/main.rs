
#[derive(Debug)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>)
}

#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

use self::BinaryTree::*;

impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            Self::Empty => {
                *self = NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: Empty,
                    right: Empty
                }));
            }

            Self::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        }
    }
}

impl<T> BinaryTree<T> {
    fn iter(&self) -> TreeIter<T> {
        let mut iter = TreeIter { unvisted: Vec::new() };
        iter.push_left_edge(self);
        iter
    }
}

struct TreeIter<'a, T> {
    unvisted: Vec<&'a TreeNode<T>>
}

impl<'a, T: 'a> TreeIter<'a, T> {
    fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
        while let NonEmpty(ref node) = *tree {
            self.unvisted.push(node);
            tree = &node.left;
        }
    }
}

impl<'a, T: 'a> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;
    type IntoIter = TreeIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for TreeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.unvisted.pop()?;
        self.push_left_edge(&node.right);
        Some(&node.element)
    }
}

fn main() {
    let sample_buffer = ["testing1", "testing2", "testing3", "testing4", "testing5", "testing6", "testing7"];
    let mut tree = BinaryTree::Empty;
    sample_buffer.iter().for_each(|f| tree.add(*f));


    for val in &tree {
        println!("{:?}", val);
    }
}

    
