#[derive(Debug, PartialEq)]
pub enum Statement {
    Command(StatementCommand),
    Variable(StatementVariable),
    End(StatementEnd),
}

#[derive(Debug, PartialEq)]
pub struct StatementCommand {
    pub name: String,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct StatementVariable {
    pub name: String,
    pub assignment: String,
}

#[derive(Debug, PartialEq)]
pub struct StatementEnd {}
