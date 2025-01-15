use std::fs::{self, DirEntry};
use std::io;
use std::ops::Index;
use std::path::Path;
use std::process::Output;
use std::ptr::NonNull;

// Read current dir, get all the entries
// For every entry create a node with entry name, add to the current tree nodes list
// On each entry run a check for whether it's a dir or not
// If it's a dir, pass the current node that you have created to the fn
// Recurce until no more files left

/*
    {
        val: string
        nodes: []
    }
*/

struct TreeNode {
    val: String,
    nodes: Vec<Box<TreeNode>>,
}
struct Vector2 {
    x: usize,
    y: usize,
}
fn construct_folder_tree(dir: &Path, tree: &mut TreeNode) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let entry_name = entry.file_name().to_string_lossy().into_owned();
            let path = entry.path();
            let mut curr = TreeNode {
                val: entry_name,
                nodes: vec![],
            };
            if path.is_dir() {
                construct_folder_tree(&path, &mut curr)?;
            }
            tree.nodes.push(Box::new(curr));
        }
    }
    Ok(())
}

fn print_tree_rec(tree: &TreeNode, levels: &mut Vec<usize>, x: usize, pad_left: usize) {
    let spacing = 2;
    let margin_left = x * spacing + pad_left;
    if tree.nodes.len() > 1 {
        levels.push(margin_left);
    }
    for (i, n) in tree.nodes.iter().enumerate() {
        if !levels.contains(&x) {}
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
                if levels.contains(&i) {
                    return "|".chars().nth(0).unwrap();
                }
                c
            })
            .collect();
        let entry_name = &n.val;
        println!("{}\n{}{}", pattern, pattern, &format!("__{}", entry_name));
        if n.nodes.len() > 0 {
            let mid = entry_name.len() / 2 + 1;

            if i + 1 == n.nodes.len() {
                match levels.iter().position(|&x| x == margin_left) {
                    Some(index) => {
                        levels.remove(index);
                    }
                    None => (),
                }
            }
            print_tree_rec(n, levels, x + 1, pad_left + mid)
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
