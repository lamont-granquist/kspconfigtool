use pest::Parser;
use pest_derive::Parser;
use pest::error::Error;
use pest::iterators::Pairs;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{PathBuf, Path};
extern crate diff;

mod cli;
mod document;
mod action;

use crate::document::{Document, Statement};
use action::Action;

#[derive(Parser)]
#[grammar = "confignode.pest"]
pub struct ConfigNodeParser;

fn parse_assignment(mut pairs: Pairs<Rule>) -> Statement {
    let first = pairs.next().unwrap();
    let second = pairs.next().unwrap();
    Statement::Assignment(first.as_str().to_string(), second.as_str().to_string())
}

fn parse_node(mut pairs: Pairs<Rule>) -> Statement {
    let first = pairs.next().unwrap();
    Statement::Node(first.as_str().to_string(), parse_statements(pairs))
}

fn parse_statements(pairs: Pairs<Rule>) -> Vec<Statement> {
    let mut statements = vec![];

    for pair in pairs {
        match pair.as_rule() {
            Rule::assignment => statements.push(parse_assignment(pair.into_inner())),
            Rule::node => statements.push(parse_node(pair.into_inner())),
            Rule::EOI => {},
            _ => {
                unreachable!();
            },
        };
    }

    statements
}

fn parse_string(string: &str) -> Result<Document, Error<Rule>> {
    let config = ConfigNodeParser::parse(Rule::document, string)?.next().unwrap();

    Ok(Document { statements: parse_statements(config.into_inner()) })
}

fn get_name_from_statements(node_id: &str, statements: &Vec<Statement>) -> Option<String> {
    for statement in statements {
        if let Statement::Assignment(identifier, value) = statement {
            if node_id == "PART" && identifier == "part" {
               return Some(value.clone());
            }
            if node_id == "RESOURCE" && identifier == "name" {
               return Some(value.clone());
            }
            if node_id == "MODULE" && identifier == "name" {
               return Some(value.clone());
            }
        }
    }
    None
}

fn strip_statements(original_statements: &Vec<Statement>, identifier: &str, name: &str) -> Vec<Statement> {
    let mut edited_statements = vec![];

    for statement in original_statements {
        match statement {
            Statement::Assignment(_, _) => { edited_statements.push(statement.clone()) },
            Statement::Node(node_id, node_statements) => {
                if node_id == identifier  {
                    if let Some(node_name) = get_name_from_statements(node_id, node_statements) {
                        if node_name == name {
                            continue;
                        }
                    }
                }
                edited_statements.push(Statement::Node(node_id.clone(), strip_statements(node_statements, identifier, name)));
            },
        }
    }

    edited_statements
}

fn strip_document(original: &Document, identifier: &str, name: &str) -> Document {
    let edited = strip_statements(&original.statements, identifier, name);

    Document{statements: edited}
}

fn append_to_path(path: &Path, ext: &str) -> PathBuf {
    let mut new_path = path.as_os_str().to_owned();

    new_path.push(ext);
    new_path.into()
}

fn replace_contents(string: &str, path: &PathBuf) -> PathBuf {
    let newpath = append_to_path(path, ".new");

    let mut newfile = File::create(&newpath).unwrap();

    newfile.write_all(string.as_bytes()).unwrap();

    newpath.clone()
}

fn windows_lineending(contents: &str) -> bool {
    for c in contents.chars() {
        if c == '\r'{
            return true;
        }
        if c == '\n' {
            return false;
        }
    }
    false
}

fn main() {
    let (action, files) = cli::parse_action();

    for file in files {
        let contents = fs::read_to_string(&file).expect("Cannot read file");

        let document = parse_string(&contents).unwrap_or_else(|e| panic!("{}", e));

        let edited = match action {
            Action::Remove(ref identifier, ref name) => {
                strip_document(&document, identifier, name)
            },
            Action::Clean => { document.clone() },
        };

        let edited_string = if windows_lineending(&contents) {
            edited.to_windows()
        } else {
            edited.to_string()
        };

        if contents == edited_string {
            println!("no changes for {}\n", &file.display());
            continue;
        }

        let backup_file = replace_contents(&edited_string, &file);

        println!("edited {}\n  => backed up to {}\n", file.display(), backup_file.display());

        println!("count = {}", diff::lines(&contents, &edited_string).len());

        for diff in diff::lines(&contents, &edited_string) {
            match diff {
                diff::Result::Left(l)    => println!("  - {}", l),
                diff::Result::Both(_, _) => {},
                diff::Result::Right(r)   => println!("  + {}", r),
            }
        }
    }
}
