use std::{fs::read_to_string, collections::HashMap};

use anyhow::Result;

#[derive(Debug, PartialEq)]
enum Node {
    Dir(HashMap<String, Node>),
    File(usize),
}

fn total_size(node: &Node) -> usize {
    match node {
        Node::Dir(children) => children.iter().map(|(_name, children)| total_size(children)).sum(),
        Node::File(size) => *size
    }
}

#[derive(Debug, PartialEq)]
enum Listing {
    Dir(String),
    File(String, usize),
}

#[derive(Debug, PartialEq)]
enum Command {
    Cd(String),
    Ls(Vec<Listing>),
}

fn parse_terminal_session(session: &str) -> Vec<Command> {
    let mut commands: Vec<Command> = vec![];

    let mut lines = session.lines().peekable();
    while let Some(command_line) = lines.next() {
        let (_, command_line) = command_line.split_at(2);
        if command_line.starts_with("cd") {
            let (_, dst) = command_line.split_at(3);
            commands.push(Command::Cd(dst.into()));
        }
        else if command_line.starts_with("ls") {
            let mut children: Vec<Listing> = vec![];
            while lines.peek().map(|line| !line.starts_with("$")).unwrap_or(false) {
                let (type_or_size, name) = lines.next().unwrap().split_once(" ").unwrap();
                let name = name.to_string();
                children.push(
                    match type_or_size {
                        "dir" => Listing::Dir(name),
                        size => Listing::File(name, size.parse().unwrap())
                    }
                )
            }
            commands.push(Command::Ls(children))
        }
    }

    commands
}

fn evaluate_commands(commands: &[Command]) -> Node {
    let mut root = Node::Dir(Default::default());
    let mut cwd_stack: Vec<String> = vec![];

    for command in commands {
        match command {
            Command::Cd(dst) => {
                if dst == "/" {
                    cwd_stack.clear();
                }
                else if dst == ".." {
                    cwd_stack.pop();
                }
                else {
                    cwd_stack.push(dst.clone());
                }
            },
            Command::Ls(children) => {
                let mut cwd = &mut root;
                for dir in &cwd_stack {
                    match cwd {
                        Node::Dir(children) => cwd = children.get_mut(dir).unwrap(),
                        _ => panic!()
                    }
                }
                match cwd {
                    Node::Dir(cwd_children) => {
                        for child in children {
                            match child {
                                Listing::Dir(name) => cwd_children.insert(name.clone(), Node::Dir(Default::default())),
                                Listing::File(name, size) => cwd_children.insert(name.clone(), Node::File(*size)),
                            };
                        }
                    },
                    _ => panic!()
                }
            }
        }
    }

    root
}

fn all_dir_sizes(node: &Node) -> Vec<usize> {
    match node {
        Node::File(..) => vec![],
        Node::Dir(children) => {
            let mut vec: Vec<usize> = children.values().flat_map(all_dir_sizes).collect();
            vec.push(total_size(node));
            vec
        }
    }
}

pub fn run() -> Result<(String, String)> {
    let root = evaluate_commands(&parse_terminal_session(&read_to_string("data/07.txt").unwrap()));

    let part1: usize = all_dir_sizes(&root).iter().filter(|count| **count <= 100000).sum();

    let free_space = 70000000 - total_size(&root);
    let required_space = 30000000 - free_space;
    let mut all_sizes = all_dir_sizes(&root);
    all_sizes.sort_unstable();
    let part2 = all_sizes.iter().find(|size| **size >= required_space).unwrap();

    Ok((part1.to_string(), part2.to_string()))
}

#[test]
fn test_parse_terminal_session() {
    assert_eq!(
        vec![
            Command::Cd("/".into()),                        // $ cd /
            Command::Ls(vec![                               // $ ls
                Listing::Dir("a".into()),                   // dir a
                Listing::File("b.txt".into(), 14848514),    // 14848514 b.txt
                Listing::File("c.dat".into(), 8504156),     // 8504156 c.dat
                Listing::Dir("d".into())                    // dir d
            ]),
            Command::Cd("a".into()),                        // $ cd a
            Command::Ls(vec![                               // $ ls
                Listing::Dir("e".into()),                   // dir e
                Listing::File("f".into(), 29116),           // 29116 f
                Listing::File("g".into(), 2557),            // 2557 g
                Listing::File("h.lst".into(), 62596),       // 62596 h.lst
            ]),
            Command::Cd("e".into()),                        // $ cd e
            Command::Ls(vec![                               // $ ls
                Listing::File("i".into(), 584),             // 584 i
            ]),
            Command::Cd("..".into()),                       // $ cd ..
            Command::Cd("..".into()),                       // $ cd ..
            Command::Cd("d".into()),                        // $ cd d
            Command::Ls(vec![                               // $ ls
                Listing::File("j".into(), 4060174),         // 4060174 j
                Listing::File("d.log".into(), 8033020),     // 8033020 d.log
                Listing::File("d.ext".into(), 5626152),     // 5626152 d.ext
                Listing::File("k".into(), 7214296),         // 7214296 k
            ]),
        ],
        parse_terminal_session(include_str!("../data/07_example.txt"))
    )
}

#[test]
fn test_evaluate_commands() {
    let commands = parse_terminal_session(include_str!("../data/07_example.txt"));
    assert_eq!(
        Node::Dir([
            ("a".into(), Node::Dir([
                ("e".into(), Node::Dir([
                    ("i".into(), Node::File(584)),
                ].into())),
                ("f".into(), Node::File(29116)),
                ("g".into(), Node::File(2557)),
                ("h.lst".into(), Node::File(62596)),
            ].into())),
            ("b.txt".into(), Node::File(14848514)),
            ("c.dat".into(), Node::File(8504156)),
            ("d".into(), Node::Dir([
                ("j".into(), Node::File(4060174)),
                ("d.log".into(), Node::File(8033020)),
                ("d.ext".into(), Node::File(5626152)),
                ("k".into(), Node::File(7214296)),
            ].into())),
        ].into()),
        evaluate_commands(&commands)
    );
}

#[test]
fn test_total_size() {
    let commands = parse_terminal_session(include_str!("../data/07_example.txt"));
    let root = evaluate_commands(&commands);
    assert_eq!(48381165, total_size(&root));
}
