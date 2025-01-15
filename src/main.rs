use std::fs::{self, DirEntry};
use std::io;
use std::ops::Index;
use std::path::Path;
use std::process::Output;
use std::ptr::NonNull;

struct TreeNode {
    val: String,
    nodes: Vec<Box<TreeNode>>,
}

impl PartialEq for TreeNode {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
fn construct_folder_tree(dir: &Path, tree: &mut TreeNode) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let entry_name = entry.file_name().to_string_lossy().into_owned();
            let path = entry.path();
            let mut curr = Box::new(TreeNode {
                val: entry_name,
                nodes: vec![],
            });
            if path.is_dir() {
                construct_folder_tree(&path, &mut curr)?;
            }
            tree.nodes.push(curr);
        }
    }
    Ok(())
}

fn print_tree_rec(tree: &TreeNode, depths: &mut Vec<usize>, x: usize, pad_left: usize) {
    let spacing = 2;
    let margin_left = x * spacing + pad_left;
    if tree.nodes.len() > 1 {}
    depths.push(margin_left);
    for (i, n) in tree.nodes.iter().enumerate() {
        let mut pattern = match x {
            0 => String::from("|"),
            _ => {
                format!("{:>width$}", '|', width = margin_left + 1)
            }
        };
        pattern = pattern
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if depths.contains(&i) {
                    return "|".chars().nth(0).unwrap();
                }
                c
            })
            .collect();
        let entry_name = &n.val;
        println!("{}\n{}{}", pattern, pattern, &format!("__{}", entry_name));
        if tree.nodes.len() == i + 1 {
            depths.pop();
        }
        if n.nodes.len() > 0 {
            let mid = entry_name.len() / 2 + 1;
            print_tree_rec(n, depths, x + 1, pad_left + mid)
        }
    }
}

fn main() {
    let mut tree = TreeNode {
        val: ".".to_string(),
        nodes: vec![],
    };
    construct_folder_tree(Path::new("."), &mut tree).unwrap();
    print_tree_rec(&tree, &mut vec![], 0, 0);
}
