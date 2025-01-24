use crate::parser;
use colored::*;
use prettytable::{format, Cell, Row, Table};

pub async fn list_commands(commands: Vec<parser::Command>) {
    println!("{}", " Available commands:".bold());

    let mut table = Table::new();

    let format = format::FormatBuilder::new()
        .column_separator(' ')
        .padding(1, 1)
        .build();

    table.set_format(format);

    table.add_row(Row::new(vec![
        Cell::new(&"Name".bold().to_string()),
        Cell::new(&"Value".bold().to_string()),
    ]));

    for command in commands {
        table.add_row(Row::new(vec![
            Cell::new(&command.key.white().to_string()),
            Cell::new(&command.value.cyan().to_string()),
        ]));
    }

    table.printstd();
}
