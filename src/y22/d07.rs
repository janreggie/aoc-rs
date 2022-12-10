use anyhow::{bail, Context, Result};
use std::{cell::RefCell, fmt, rc::Rc};

#[derive(Debug)]
enum Node {
    Folder(Rc<RefCell<Folder>>),
    File(Rc<RefCell<File>>),
}

struct Folder {
    name: String,
    contents: Vec<Node>,
    parent: Option<Rc<RefCell<Folder>>>,
    computed_size: u32,
}

impl Folder {
    fn new(name: String, parent: Option<Rc<RefCell<Folder>>>) -> Folder {
        Folder {
            name,
            contents: vec![],
            parent,
            computed_size: 0,
        }
    }

    fn add(&mut self, node: Node) {
        self.contents.push(node);
    }
}

impl fmt::Debug for Folder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Folder")
            .field("name", &self.name)
            .field(
                "parent",
                &self
                    .parent
                    .clone()
                    .map(|pp| pp.as_ref().borrow().name.clone()),
            )
            .finish()
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
    parent: Rc<RefCell<Folder>>,
}

impl File {
    fn new(name: String, size: u32, parent: Rc<RefCell<Folder>>) -> File {
        File { name, size, parent }
    }
}

enum Line {
    Instruction(Instruction),
    LsOut(LsOut),
}

impl Line {
    fn new(line: &str) -> Result<Line> {
        if line.starts_with("$ ") {
            if line == "$ ls" {
                return Ok(Line::Instruction(Instruction::Ls));
            }
            if line.starts_with("$ cd ") {
                let cd_param = line.trim_start_matches("$ cd ");
                let cd_param = match cd_param {
                    ".." => CdParam::Parent,
                    "/" => CdParam::Root,
                    _ => CdParam::Name(cd_param.to_string()),
                };
                return Ok(Line::Instruction(Instruction::Cd(cd_param)));
            }
            bail!("invalid instruction `{}`", line)
        }

        let args = line.split_ascii_whitespace().collect::<Vec<&str>>();
        if args.len() != 2 {
            bail!("could not parse ls output line `{}`", line);
        }
        if args[0] == "dir" {
            return Ok(Line::LsOut(LsOut::Dir(args[1].to_string())));
        }

        let size = args[0]
            .parse::<u32>()
            .with_context(|| format!("could not interpret size in line `{}`", line))?;
        return Ok(Line::LsOut(LsOut::File(args[1].to_string(), size)));
    }
}

enum Instruction {
    Cd(CdParam),
    Ls,
}

enum CdParam {
    Parent,
    Root,
    Name(String),
}

enum LsOut {
    Dir(String),
    File(String, u32),
}

fn parse(input: Vec<String>) -> Result<Rc<RefCell<Folder>>> {
    let root = Rc::new(RefCell::new(Folder::new(String::from("/"), None)));
    let input = input
        .iter()
        .map(|line| Line::new(line).with_context(|| format!("could not parse line {}", line)))
        .collect::<Result<Vec<Line>>>()?;
    let mut current_folder = Rc::clone(&root);
    for line in &input {
        match line {
            Line::Instruction(instr) => {
                if let Instruction::Cd(cd_param) = instr {
                    match cd_param {
                        CdParam::Root => {
                            current_folder = Rc::clone(&root);
                        }
                        CdParam::Parent => {
                            let next_folder = current_folder
                                .as_ref()
                                .borrow()
                                .parent
                                .clone()
                                .context("could not get parent folder")?;
                            current_folder = next_folder;
                        }
                        CdParam::Name(name) => {
                            let mut next_folder = None;
                            for node in &current_folder.as_ref().borrow().contents {
                                match node {
                                    Node::Folder(folder) => {
                                        if &folder.as_ref().borrow().name == name {
                                            next_folder = Some(Rc::clone(folder));
                                            break;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            match next_folder {
                                None => bail!("cannot find folder {}", name),
                                Some(folder) => {
                                    current_folder = folder;
                                }
                            }
                        }
                    }
                }
            }
            Line::LsOut(ls_out) => {
                let next_child = match ls_out {
                    LsOut::Dir(name) => Node::Folder(Rc::new(RefCell::new(Folder::new(
                        name.to_string(),
                        Some(current_folder.clone()),
                    )))),
                    LsOut::File(name, size) => Node::File(Rc::new(RefCell::new(File::new(
                        name.to_string(),
                        *size,
                        current_folder.clone(),
                    )))),
                };
                current_folder.as_ref().borrow_mut().add(next_child);
            }
        }
    }

    Ok(root)
}

fn traverse<F, T>(root: Rc<RefCell<Folder>>, f: &F, t_0: T) -> T
where
    F: Fn(&Node, T) -> T,
{
    // First, deal with the start
    let mut t = f(&Node::Folder(Rc::clone(&root)), t_0);
    // Then the rest...
    for node in &root.as_ref().borrow().contents {
        match node {
            Node::Folder(folder) => t = traverse(folder.clone(), f, t),
            Node::File(_) => t = f(node, t),
        }
    }
    t
}

/// Computes sizes for node and all notes within
fn compute_sizes(node: &mut Node) -> u32 {
    match node {
        Node::File(file) => file.as_ref().borrow().size,
        Node::Folder(folder) => {
            let folder_size = folder
                .as_ref()
                .borrow_mut()
                .contents
                .iter_mut()
                .map(|node| compute_sizes(node))
                .sum();
            folder.as_ref().borrow_mut().computed_size = folder_size;
            folder_size
        }
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    let root_folder = parse(lines).context("could not parse input properly")?;
    compute_sizes(&mut Node::Folder(Rc::clone(&root_folder)));

    // Part 1: Get all directories whose sum may is at most 100_000.
    fn add_node_to_sum(node: &Node, sum: u32) -> u32 {
        match node {
            Node::File(_) => sum,
            Node::Folder(folder) => {
                let size = folder.as_ref().borrow().computed_size;
                if size <= 100_000 {
                    sum + size
                } else {
                    sum
                }
            }
        }
    }
    let ans1 = traverse(Rc::clone(&root_folder), &add_node_to_sum, 0).to_string();

    // Part 2: Find the smallest directory that, if deleted, will ensure root uses at most 40_000_000
    let total_size = root_folder.as_ref().borrow().computed_size;
    let delete_at_least = total_size - 40_000_000;
    let consider_node_to_delete = |node: &Node, previous_candidate: u32| -> u32 {
        if let Node::Folder(folder) = node {
            let size = folder.as_ref().borrow().computed_size;
            if size >= delete_at_least && size < previous_candidate {
                size
            } else {
                previous_candidate
            }
        } else {
            previous_candidate
        }
    };
    let ans2 = traverse(root_folder, &consider_node_to_delete, total_size).to_string();

    Ok((ans1, ans2))
}
