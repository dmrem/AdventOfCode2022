use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    name: String, // for debugging
    file_type: Type,
    size: u64,
    children: HashMap<String, Rc<RefCell<Node>>>,
    parent: Option<Weak<RefCell<Node>>>,
}

#[derive(Debug, Eq, PartialEq)]
enum Type {
    Dir,
    File,
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn write_with_indents(f: &mut Formatter<'_>, root: &Node, num_indents: usize) {
            match root.file_type {
                Type::File => {
                    writeln!(
                        f,
                        "{}- {} (File: size = {})",
                        "    ".repeat(num_indents),
                        root.name,
                        root.size
                    )
                    .unwrap();
                }
                Type::Dir => {
                    writeln!(
                        f,
                        "{}- {} (Dir: size = {})",
                        "    ".repeat(num_indents),
                        root.name,
                        root.size
                    )
                    .unwrap();
                    root.children.values().for_each(|x| {
                        write_with_indents(f, &x.as_ref().borrow(), num_indents + 1);
                    });

                    // // print files separately from dirs
                    // root.children.values().filter(|x| x.borrow().file_type == Type::File).for_each(|x| {
                    //     write_with_indents(f, &x.as_ref().borrow(), num_indents + 1);
                    // });
                    //
                    // root.children.values().filter(|x| x.borrow().file_type == Type::Dir).for_each(|x| {
                    //     write_with_indents(f, &x.as_ref().borrow(), num_indents + 1);
                    // })
                }
            }
        }

        write_with_indents(f, self, 0);
        Ok(())
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let history = input
        .split('\n')
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();

    let root = Rc::new(RefCell::new(Node {
        name: "/".to_string(),
        file_type: Type::Dir,
        size: 0,
        children: HashMap::new(),
        parent: None,
    }));

    parse_history(&history, root.clone(), root.clone());
    calculate_sizes(root.clone());

    // println!("{}", root.borrow());
    dbg!(get_sum_of_small_dirs(root.clone()));
}

fn parse_history(history: &[&str], root: Rc<RefCell<Node>>, curr: Rc<RefCell<Node>>) {
    match history {
        [command, ..] if command.starts_with("$ cd ") => match command[5..].trim() {
            "/" => parse_history(&history[1..], root.clone(), root.clone()),
            ".." => parse_history(
                &history[1..],
                root,
                curr.borrow()
                    .parent
                    .as_ref()
                    .unwrap()
                    .clone()
                    .upgrade()
                    .unwrap(),
            ),
            name => {
                let temp = Rc::new(RefCell::new(Node {
                    name: name.to_string(),
                    file_type: Type::Dir,
                    size: 0,
                    children: HashMap::new(),
                    parent: Some(Rc::downgrade(&curr.clone())),
                }));
                curr.borrow_mut()
                    .children
                    .insert(name.to_string(), temp.clone());
                parse_history(&history[1..], root, temp.clone());
            }
        },
        ["$ ls", ..] => {
            let items: Vec<_> = history
                .iter()
                .skip(1)
                .take_while(|x| !x.starts_with('$'))
                .collect();
            items
                .iter()
                .filter(|x| !x.starts_with("dir"))
                .map(|x| x.split(' ').collect::<Vec<_>>())
                .for_each(|x| {
                    curr.borrow_mut().children.insert(
                        x[1].to_string(),
                        Rc::new(RefCell::new(Node {
                            name: x[1].to_string(),
                            file_type: Type::File,
                            size: x[0].parse().unwrap(),
                            children: HashMap::new(),
                            parent: Some(Rc::downgrade(&curr.clone())),
                        })),
                    );
                });
            parse_history(&history[items.len() + 1..], root.clone(), curr.clone())
        }
        [] => {}
        _ => panic!(),
    }
}

fn calculate_sizes(root: Rc<RefCell<Node>>) -> u64 {
    let mut r = root.borrow_mut();
    match &*r {
        Node {
            file_type: Type::Dir,
            children,
            ..
        } => {
            r.size = children.values().map(|x| calculate_sizes(x.clone())).sum();
            r.size
        }
        Node {
            file_type: Type::File,
            size,
            ..
        } => *size,
    }
}

fn get_sum_of_small_dirs(root: Rc<RefCell<Node>>) -> u64 {
    match root.borrow().file_type {
        Type::Dir => {
            (if root.borrow().size <= 100000 { root.borrow().size } else { 0u64 })
                + root.borrow().children
                    .values()
                    .filter(|x| x.borrow().file_type == Type::Dir)
                    .map(|x| get_sum_of_small_dirs(x.clone()))
                    .sum::<u64>()
        }
        _ => panic!(),
    }
}
