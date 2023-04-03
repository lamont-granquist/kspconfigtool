use std::fmt;

#[derive(Debug, Clone)]
pub struct Document {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Assignment(String, String),
    Node(String, Vec<Statement>),
}

impl Document {
    pub fn to_windows(&self) -> String {
        let mut builder = string_builder::Builder::default();

        for stmt in &self.statements {
            _string(&mut builder, stmt, 0, "\r\n");
        }

        builder.string().unwrap()
    }
}

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = string_builder::Builder::default();

        for stmt in &self.statements {
            _string(&mut builder, stmt, 0, "\n");
        }

        write!(f, "{}", builder.string().unwrap())
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = string_builder::Builder::default();

        _string(&mut builder, &self, 0, "\n");

        write!(f, "{}", builder.string().unwrap())
    }
}

fn _string_node(builder: &mut string_builder::Builder, identifier: &str, statements: &[Statement], depth: usize, ending: &str) {
    let indent: String = std::iter::repeat("\t").take(depth).collect();
    builder.append(format!("{}{}{}", indent, identifier, ending));
    builder.append(format!("{}{{{}", indent, ending));
    for stmt in statements {
        _string(builder, stmt, depth+1, ending);
    }
    builder.append(format!("{}}}{}", indent, ending));
}

fn _string_assignment(builder: &mut string_builder::Builder, identifier: &str, value: &str, depth: usize, ending: &str) {
    let indent: String = std::iter::repeat("\t").take(depth).collect();
    builder.append(format!("{}{} = {}{}",indent, identifier, value, ending));
}

fn _string(builder: &mut string_builder::Builder, statement: &Statement, depth: usize, ending: &str) {
    match statement {
        Statement::Assignment(identifier, value) => _string_assignment(builder, identifier, value, depth, ending),
        Statement::Node(identifier, statements) => _string_node(builder, identifier, statements, depth, ending),
    }
}
