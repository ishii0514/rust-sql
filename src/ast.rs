#[derive(Debug, PartialEq)]
pub enum Statement {
    Select {
        table: String,
    },
    Insert {
        table: String,
        values: Vec<String>,
    },
    Update {
        table: String,
        set: Vec<(String, String)>,
    },
    Delete {
        table: String,
    },
}
