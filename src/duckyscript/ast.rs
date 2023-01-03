#[derive(Debug)]
pub enum Statement {
    Command(StatementCommand),
    Variable(StatementVariable),
    End(StatementEnd),
}

#[derive(Debug)]
pub struct StatementCommand {
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
pub struct StatementVariable {
    pub name: String,
    pub assignment: String,
}

#[derive(Debug)]
pub struct StatementEnd {}
