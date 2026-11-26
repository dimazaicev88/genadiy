use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub struct Diagnostic {
    path: String,
    line: usize,
    column: i32,
    message: String,
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (line {}, column {}): {}",
            self.path, self.line, self.column, self.message
        )
    }
}

pub struct Diagnostics {
    items: Vec<Diagnostic>,
}

impl Diagnostics {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn add(&mut self, diagnostic: Diagnostic) {
        self.items.push(diagnostic);
    }
}

impl fmt::Display for Diagnostics {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let messages: Vec<String> = self.items.iter().map(|item| item.to_string()).collect();
		write!(f, "{}", messages.join("\n"))
    }
}

