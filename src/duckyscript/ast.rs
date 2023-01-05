#[derive(Debug, PartialEq)]
pub enum Statement {
    Command(StatementCommand),
    VariableAssignment(StatementVariableAssignment),
    VariableDeclaration(StatementVariableDeclaration),
    End(StatementEnd),
}

#[derive(Debug, PartialEq)]
pub struct StatementCommand {
    pub name: String,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct StatementVariableAssignment {
    pub name: String,
    pub assignment: String,
}

#[derive(Debug, PartialEq)]
pub struct StatementVariableDeclaration {
    pub name: String,
    pub assignment: String,
}

#[derive(Debug, PartialEq)]
pub struct StatementEnd {}
