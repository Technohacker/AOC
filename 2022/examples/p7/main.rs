use aoc_2022::line_by_line;
use indextree::{Arena, NodeId};
use regex::Regex;

struct Node {
    pub name: String,
    pub size: usize,
    // Not a type enum for simplicity
    pub is_dir: bool,
}

fn get_data(fs: &Arena<Node>, id: NodeId) -> &Node {
    fs.get(id).unwrap().get()
}

fn get_data_mut(fs: &mut Arena<Node>, id: NodeId) -> &mut Node {
    fs.get_mut(id).unwrap().get_mut()
}

fn main() {
    let mut fs = Arena::<Node>::new();

    let root = fs.new_node(Node {
        name: "".to_string(),
        size: 0,
        is_dir: true,
    });

    let mut lines = line_by_line("./examples/p7/term_out.txt").peekable();

    let mut curr_folder = root;

    let cd_command = Regex::new(r"^\$ cd (.+)$").unwrap();
    let ls_command = Regex::new(r"^\$ ls$").unwrap();

    let dir_line = Regex::new(r"^dir (.+)$").unwrap();
    let file_line = Regex::new(r"^(\d+) (.+)$").unwrap();

    while let Some(line) = lines.next() {
        if let Some(groups) = cd_command.captures(&line) {
            // cd command
            let path = groups.get(1).unwrap();
            curr_folder = match path.as_str() {
                "/" => root,
                ".." => curr_folder.ancestors(&fs).nth(1).unwrap_or(root),
                folder => {
                    let mut contents = curr_folder
                        .children(&fs)
                        .map(|x| (x, &get_data(&fs, x).name));

                    contents
                        .find_map(|(id, name)| if name == folder { Some(id) } else { None })
                        .unwrap()
                }
            }
        } else if ls_command.captures(&line).is_some() {
            while let Some(peek) = lines.peek() {
                // Take until next line is a command
                if cd_command.is_match(peek) || ls_command.is_match(peek) {
                    break;
                }

                let line = lines.next().unwrap();

                if let Some(groups) = file_line.captures(&line) {
                    let size = groups.get(1).unwrap().as_str().parse::<usize>().unwrap();
                    let name = groups.get(2).unwrap().as_str().to_string();

                    let new_node = fs.new_node(Node {
                        name,
                        size,
                        is_dir: false,
                    });
                    curr_folder.append(new_node, &mut fs);
                } else if let Some(groups) = dir_line.captures(&line) {
                    let name = groups.get(1).unwrap().as_str().to_string();

                    let new_node = fs.new_node(Node {
                        name,
                        // To be calculated
                        size: 0,
                        is_dir: true,
                    });
                    curr_folder.append(new_node, &mut fs);
                }
            }
        }
    }

    for node in post_order(&fs, root) {
        let children_size: usize = node
            .children(&fs)
            .map(|x| get_data(&fs, x).size)
            .sum();

        let data = get_data_mut(&mut fs, node);
        // All directories have zero initial size, all files have zero children
        data.size += children_size;
    }

    // Part 1
    let total: usize = root.descendants(&fs)
        .map(|x| get_data(&fs, x))
        .filter(|x| x.is_dir && x.size < 100000)
        .map(|x| x.size)
        .sum();
    
    println!("{}", total);

    // Part 2
    let current_free = 70000000 - get_data(&fs, root).size;
    let smallest_remaining_free: usize = root.descendants(&fs)
        .map(|x| get_data(&fs, x))
        .filter(|x| x.is_dir)
        .map(|x| current_free + x.size)
        .filter(|x| *x > 30000000)
        .min()
        .unwrap();
    
    println!("{}", smallest_remaining_free - current_free);
}

fn post_order(fs: &Arena<Node>, root: NodeId) -> Vec<NodeId> {
    root.children(fs)
        .flat_map(|x| post_order(fs, x))
        .chain(std::iter::once(root))
        .collect()
}
