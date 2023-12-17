use crate::Type::{Dir, File};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    name: String, // for debugging
    file_type: Type,
    size: u32,
    children: HashMap<String, Rc<RefCell<Node>>>,
    parent: Option<Weak<RefCell<Node>>>,
}

#[derive(Debug)]
enum Type {
    Dir,
    File,
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
        file_type: Dir,
        size: 0,
        children: HashMap::new(),
        parent: None,
    }));

    parse_history(&history, root.clone(), root.clone());
    dbg!(root);
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
                    file_type: Dir,
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
                            file_type: File,
                            size: x[0].parse().unwrap(),
                            children: HashMap::new(),
                            parent: Some(Rc::downgrade(&curr.clone())),
                        })),
                    );
                });
            parse_history(&history[items.len() + 1..], root.clone(), curr.clone())
        }
        [] => {},
        _ => panic!(),
    }
}
