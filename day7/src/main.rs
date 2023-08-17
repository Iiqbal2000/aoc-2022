use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::rc::{self, Rc, Weak};

#[derive(Debug, Clone)]
enum NodeType {
    dir(u32),
    file(u32),
}

#[derive(Debug, Clone)]
struct Node<'a> {
    name: &'a str,
    nodeType: RefCell<NodeType>,
    parent: RefCell<Weak<Node<'a>>>,
    children: RefCell<Vec<Rc<Node<'a>>>>,
}

#[derive(Debug)]
struct FileSystem<'a> {
    root_dir: Option<Rc<Node<'a>>>,
    current_dir: Option<Rc<Node<'a>>>,
}

impl<'a> FileSystem<'a> {
    fn parse_cmds(&mut self, mut l: &'a str) {
        let mut line = l.lines().peekable();

        while let Some(cmd_line) = line.next() {
            let mut cmd = cmd_line.split_whitespace();
            match (cmd.next(), cmd.next()) {
                (Some("$"), Some("cd")) => self.cd(cmd.next().unwrap()),
                (Some("$"), Some("ls")) => {
                    let mut children: Vec<&str> = vec![];

                    while let Some(list_item) = line.peek() {
                        let first_word = list_item.split_whitespace().next().unwrap();

                        if first_word.chars().all(char::is_numeric) || first_word == "dir" {
                            children.push(line.next().unwrap())
                        } else {
                            break;
                        }
                    }

                    self.ls(children);
                }
                _ => panic!("should not occur!"),
            }
        }
    }

    fn cd(&mut self, arg: &'a str) {
        if arg != ".." {
            let n = Node::new(arg, NodeType::dir(0));

            if self.root_dir.is_none() {
                self.root_dir = Some(n.clone());
                self.current_dir = Some(n.clone());
            } else {
                let mut is_target_dir_exist = false;
                let mut current_dir = self.current_dir.as_ref().borrow_mut().unwrap();

                let mut target_dir: Option<Rc<Node<'_>>> = None;

                // looking for the target dir on children of the current dir
                for c in current_dir.children.borrow().iter() {
                    if c.name == arg {
                        is_target_dir_exist = true;
                        target_dir = Some(c.clone());
                        break;
                    }
                }

                if !is_target_dir_exist {
                    Node::add_child(self.root_dir.as_ref().unwrap(), n.clone());
                    self.current_dir = Some(n.clone());
                } else {
                    self.current_dir = target_dir;
                }
            }
        } else {
            let parent_to_upgrade = if let Some(current) = &self.current_dir {
                Some(current.parent.borrow().clone())
            } else {
                None
            };

            if let Some(weak_parent) = parent_to_upgrade {
                if let Some(rc_parent) = weak_parent.upgrade() {
                    self.current_dir = Some(rc_parent);
                } else {
                    self.current_dir = None;
                }
            }
        }
    }

    fn ls(&mut self, list_item: Vec<&'a str>) {
        for item in list_item {
            let mut splitted_item = item.split_whitespace().peekable();

            if splitted_item.peek() == Some(&"dir") {
                let current_dir = self.current_dir.as_ref().unwrap();
                let node_name = splitted_item.nth(1).unwrap();
                let new_node = Node::new(node_name, NodeType::dir(0));

                Node::add_child(current_dir, new_node.clone());
            } else {
                let current_dir = self.current_dir.as_ref().unwrap();
                let node_size: u32 = splitted_item.next().unwrap().parse().unwrap();
                let node_name = splitted_item.next().unwrap();

                let new_node = Node::new(node_name, NodeType::file(node_size));

                Node::add_child(current_dir, new_node.clone());
            }
        }
    }
}

impl<'a> Node<'a> {
    fn new(name: &'a str, nodeType: NodeType) -> Rc<Node> {
        Rc::new(Node {
            name: name,
            nodeType: RefCell::new(nodeType),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }

    fn add_child(parent: &Rc<Node<'a>>, child: Rc<Node<'a>>) {
        let mut current_node = Some(parent.clone());

        while let Some(node) = current_node {
            if let NodeType::dir(ref mut size) = *node.nodeType.borrow_mut() {
                match *child.nodeType.borrow() {
                    NodeType::dir(child_size) | NodeType::file(child_size) => *size += child_size,
                }
            }

            // Move to the next parent for the next iteration
            current_node = node.parent.borrow().upgrade();
        }

        child.parent.replace(Rc::downgrade(parent));
        parent.children.borrow_mut().push(child);

        // dbg!(parent);
    }

    fn total_size(&self) -> u32 {
        match self.nodeType.clone().into_inner() {
            NodeType::file(size) => size,
            NodeType::dir(size) => size,
        }
    }

    fn filter_nodes(&self, filter: &dyn Fn(&Node<'a>) -> bool) -> Vec<Rc<Node<'a>>> {
        let mut results = Vec::new();

        if filter(self) {
            results.push(Rc::new(self.clone()));
        }

        for child in self.children.borrow().iter() {
            results.extend(child.filter_nodes(filter))
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use super::*;

    // Helper function to set up a basic filesystem for testing
    fn setup_filesystem() -> Rc<Node<'static>> {
        let root = Node::new("/", NodeType::dir(0));
        let dir1 = Node::new("dir1", NodeType::dir(0));
        let file1 = Node::new("file1", NodeType::file(500));
        let file2 = Node::new("file2", NodeType::file(2000));

        // Construct the filesystem
        Node::add_child(&root, dir1);
        Node::add_child(&root, file1);
        Node::add_child(&root, file2);

        root
    }

    #[test]
    fn add_child_nodes() {
        let root_node = Node::new("/", NodeType::dir(0));
        let dir1 = Node::new("dir1", NodeType::dir(0));

        Node::add_child(&root_node, dir1.clone());

        assert_eq!(1, root_node.children.borrow().len());
    }

    #[test]
    fn check_parent_child_relationships() {
        let root_node = Node::new("/", NodeType::dir(0));
        let dir1 = Node::new("dir1", NodeType::dir(0));
        let file1 = Node::new("file1", NodeType::file(122));

        Node::add_child(&root_node, dir1.clone());
        Node::add_child(&root_node, file1.clone());

        assert_eq!("/", dir1.parent.borrow().upgrade().unwrap().name);
        assert_eq!("/", file1.parent.borrow().upgrade().unwrap().name);
    }

    #[test]
    fn test_total_size_calculation() {
        let root_node = Node::new("/", NodeType::dir(0));
        let dir1 = Node::new("dir1", NodeType::dir(0));
        let file1 = Node::new("file1", NodeType::file(122));

        Node::add_child(&root_node, dir1.clone());
        Node::add_child(&root_node, file1.clone());

        assert_eq!(122, file1.clone().total_size());
        assert_eq!(122, root_node.total_size());
    }

    #[test]
    fn modify_parent_and_verify() {
        let dir1 = Node::new("dir1", NodeType::dir(0));
        let file1 = Node::new("file1", NodeType::file(122));

        *file1.parent.borrow_mut() = Rc::downgrade(&dir1);

        assert_eq!("dir1", file1.parent.borrow().upgrade().unwrap().name);
    }

    #[test]
    fn nested_dir_size() {
        let root = Node::new("root", NodeType::dir(0));
        let dir1 = Node::new("dir1", NodeType::dir(0));
        let file1 = Node::new("file1", NodeType::file(50));
        let dir2 = Node::new("dir2", NodeType::dir(0));
        let file2 = Node::new("file2", NodeType::file(150));

        Node::add_child(&dir1, file1);
        Node::add_child(&dir2, file2);
        Node::add_child(&root, dir1);
        Node::add_child(&root, dir2);

        assert_eq!(200, root.total_size());
    }

    #[test]
    fn filter_directories() {
        let root = setup_filesystem();

        fn is_directory(node: &Node) -> bool {
            matches!(node.nodeType.borrow().deref(), NodeType::dir(_))
        }

        let directories = root.filter_nodes(&is_directory);

        assert_eq!(directories.len(), 2); // root and dir1
        assert_eq!(root.children.borrow().len(), 3);
    }

    #[test]
    fn filter_files() {
        let root = setup_filesystem();

        fn is_file(node: &Node) -> bool {
            matches!(node.nodeType.borrow().deref(), NodeType::file(_))
        }

        let files = root.filter_nodes(&is_file);

        assert_eq!(files.len(), 2); // file1 and file2
    }

    #[test]
    fn filter_large_files() {
        let root = setup_filesystem();

        fn is_large_file(node: &Node) -> bool {
            if let NodeType::file(size) = *node.nodeType.borrow() {
                size > 1000
            } else {
                false
            }
        }

        let large_files = root.filter_nodes(&is_large_file);

        assert_eq!(large_files.len(), 1); // only file2
    }

    #[test]
    fn parse_and_get_size() {
        let mut line = "$ cd /
$ ls
dir a
5 b.txt
5 c.dat
dir d";

        let mut fs = FileSystem {
            root_dir: None,
            current_dir: None,
        };

        fs.parse_cmds(line);

        assert_eq!(10, fs.root_dir.clone().unwrap().total_size());
    }

    #[test]
    fn test_ls_output() {
        let mut line = "$ cd /
$ ls
dir a
14848514 b.txt";

        let mut fs = FileSystem {
            root_dir: None,
            current_dir: None,
        };

        fs.parse_cmds(line);
        dbg!(&fs);

        // Assuming you have a method to get the number of children in the current directory
        assert_eq!(2, fs.root_dir.clone().unwrap().children.borrow().len());
    }

    #[test]
    fn test_back_navigation() {
        let mut line = "$ cd dir1
$ cd dir2
$ cd ..";

        let mut fs = FileSystem {
            root_dir: None,
            current_dir: None,
        };

        fs.parse_cmds(line);

        dbg!(&fs);

        assert_eq!("dir1", fs.current_dir.clone().unwrap().name);
    }

    #[test]
    fn part1() {
        let mut line = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        let mut fs = FileSystem {
            root_dir: None,
            current_dir: None,
        };

        fs.parse_cmds(line);

        fn is_the_dir(node: &Node) -> bool {
            match *node.nodeType.borrow() {
                NodeType::dir(size) => size <= 100000,
                NodeType::file(_) => false,
            }
        }

        // dbg!(&fs);

        let result = fs.root_dir.unwrap().filter_nodes(&is_the_dir);
        let total_size = result
            .iter()
            .fold(0, |acc, node| match *node.nodeType.borrow() {
                NodeType::dir(size) | NodeType::file(size) => acc + size,
            });

        assert_eq!(95437, total_size);
        assert_eq!(2, result.len());
    }

    #[test]
    fn part2() {
        let mut line = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        let mut fs = FileSystem {
            root_dir: None,
            current_dir: None,
        };

        fs.parse_cmds(line);

        const total_space: u32 = 70000000;
        const unused_space_needed: u32 = 30000000;

        let current_empty_space: u32 = total_space - (&fs).root_dir.clone().unwrap().total_size();

        let is_the_dir = |node: &Node| -> bool {
            match *node.nodeType.borrow() {
                NodeType::dir(size) => (size + current_empty_space) >= unused_space_needed,
                NodeType::file(_) => false,
            }
        };

        let result = fs.root_dir.unwrap().filter_nodes(&is_the_dir);

        let unwrap_size = |node: &Node| match *node.nodeType.borrow() {
            NodeType::dir(size) | NodeType::file(size) => size,
        };

        let smallest_node = result.iter().min_by(|a, b| {
            return unwrap_size(&a).cmp(&unwrap_size(&b));
        });

        assert_eq!(24933642, unwrap_size(smallest_node.unwrap()));
    }
}

fn solve_part1(fs: FileSystem) -> u32 {
    fn is_the_dir(node: &Node) -> bool {
        if let NodeType::dir(size) = *node.nodeType.borrow() {
            size <= 100000
        } else {
            false
        }
    }

    let result = fs.root_dir.unwrap().filter_nodes(&is_the_dir);
    let total_size = result
        .iter()
        .fold(0, |acc, node| match *node.nodeType.borrow() {
            NodeType::dir(size) | NodeType::file(size) => acc + size,
        });

    return total_size;
}

fn solve_part2(fs: FileSystem) -> u32 {
    const total_space: u32 = 70000000;
    const unused_space_needed: u32 = 30000000;

    // 28390426
    let current_empty_space: u32 = total_space - (&fs).root_dir.clone().unwrap().total_size();
    // 1609574

    let is_the_dir = |node: &Node| -> bool {
        match *node.nodeType.borrow() {
            NodeType::dir(size) => (size + current_empty_space) >= unused_space_needed,
            NodeType::file(_) => false,
        }
    };

    let mut result = fs.root_dir.unwrap().filter_nodes(&is_the_dir);

    let unwrap_size = |node: &Node| match *node.nodeType.borrow() {
        NodeType::dir(size) | NodeType::file(size) => size,
    };

    let smallest_node = result.iter().min_by(|a, b| {
        return unwrap_size(&a).cmp(&unwrap_size(&b));
    });

    return unwrap_size(smallest_node.unwrap());
}

fn main() {
    let buf = fs::read_to_string("/Users/Iqbal.alasqalani/Dev/aoc-2022/day7/input.txt")
        .expect("can't read file");

    let mut filesystem_node = FileSystem {
        root_dir: None,
        current_dir: None,
    };

    filesystem_node.parse_cmds(buf.as_str());

    println!("result: {}", solve_part2(filesystem_node));
}
