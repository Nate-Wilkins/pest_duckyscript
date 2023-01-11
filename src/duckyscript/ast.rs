#[derive(Debug, PartialEq)]
pub enum Statement {
    BlockIf(StatementBlockIf),
    BlockWhile(StatementBlockWhile),
    CommandDefaultDelay(StatementCommandDefaultDelay),
    CommandDefine(StatementCommandDefine),
    CommandDelay(StatementCommandDelay),
    CommandKey(StatementCommandKey),
    CommandKeyValue(StatementCommandKeyValue),
    CommandRem(StatementCommandRem),
    CommandString(StatementCommandString),
    CommandStringln(StatementCommandStringln),
    CommandExfil(StatementCommandExfil),
    End(StatementEnd),
    SingleCommand(StatementSingleCommand),
    VariableAssignment(StatementVariableAssignment),
    VariableDeclaration(StatementVariableDeclaration),
}

#[derive(Debug, PartialEq)]
pub struct StatementBlockIf {
    pub expression: String,
    pub statements_true: Vec<Statement>,
    pub statements_false: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct StatementBlockWhile {
    pub expression: String,
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct StatementCommandDefine {
    pub name: String,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct StatementSingleCommand {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct StatementCommandRem {
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct StatementCommandDefaultDelay {
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct StatementCommandDelay {
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct StatementCommandStringln {
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct StatementCommandExfil {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct StatementCommandString {
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct StatementCommandKey {
    pub statements: Vec<Statement>,
    pub remaining_keys: String,
}

#[derive(Debug, PartialEq)]
pub struct StatementCommandKeyValue {
    pub name: String,
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
