use comfy_table::{Cell, ContentArrangement, Row, Table, presets::UTF8_FULL};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::contacts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Contact {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: Option<String>,
    pub email: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::contacts)]
pub struct NewContact {
    pub first_name: String,
    pub last_name: String,
    pub phone_number: Option<String>,
    pub email: Option<String>,
}

pub trait FormatTable {
    fn format_to_table(&self) -> Table;
}

impl FormatTable for Vec<Contact> {
    fn format_to_table(&self) -> Table {
        let mut table = Table::new();

        table.load_preset(UTF8_FULL);
        table.set_content_arrangement(ContentArrangement::Dynamic);
        table.set_header(vec![
            Cell::new("ID"),
            Cell::new("First Name"),
            Cell::new("Last Name"),
            Cell::new("Phone Number"),
            Cell::new("Email"),
        ]);

        for contact in self {
            table.add_row(Row::from(vec![
                Cell::new(contact.id),
                Cell::new(&contact.first_name),
                Cell::new(&contact.last_name),
                Cell::new(contact.phone_number.as_deref().unwrap_or("")),
                Cell::new(contact.email.as_deref().unwrap_or("")),
            ]));
        }

        table
    }
}
