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
        let mut buf = String::new();
        let mut f = core::fmt::Formatter::new(&mut buf);

        for stmt in &self.statements {
            _fmt(&mut f, stmt, 0).expect("a Display implementation returned an error unexpectedly");
        }

        buf
    }
}

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for stmt in &self.statements {
            _fmt(f, stmt, 0)?
        }
        Ok(())
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        _fmt(f, &self, 0)
    }
}

fn _fmt_node(f: &mut fmt::Formatter<'_>, identifier: &str, statements: &[Statement], depth: usize) -> fmt::Result {
    let indent: String = std::iter::repeat("\t").take(depth).collect();
    writeln!(f, "{}{}", indent, identifier)?;
    writeln!(f, "{}{{", indent)?;
    for stmt in statements {
        _fmt(f, stmt, depth+1)?;
    }
    writeln!(f, "{}}}", indent)
}

fn _fmt_assignment(f: &mut fmt::Formatter<'_>, identifier: &str, value: &str, depth: usize) -> fmt::Result {
    let indent: String = std::iter::repeat("\t").take(depth).collect();
    writeln!(f, "{}{} = {}", indent, identifier, value)
}

fn _fmt(f: &mut fmt::Formatter<'_>, statement: &Statement, depth: usize) -> fmt::Result {
    match statement {
        Statement::Assignment(identifier, value) => _fmt_assignment(f, identifier, value, depth),
        Statement::Node(identifier, statements) => _fmt_node(f, identifier, statements, depth),
    }
}
