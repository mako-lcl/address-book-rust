use crate::Result;
use crate::model::Contact;
use crate::model::NewContact;
use diesel::PgConnection;
use diesel::prelude::*;
use diesel::sql_types::Text;

pub fn select_contacts(conn: &mut PgConnection) -> Result<Vec<Contact>> {
    use crate::schema::contacts::dsl::*;

    let results = contacts.load(conn)?;

    Ok(results)
}

pub fn insert_contact(conn: &mut PgConnection, new_contact: NewContact) -> Result<()> {
    use crate::schema::contacts::dsl::*;

    diesel::insert_into(contacts)
        .values(&new_contact)
        .execute(conn)?;

    Ok(())
}

pub fn update_contact(conn: &mut PgConnection, contact: Contact) -> Result<()> {
    use crate::schema::contacts::dsl::*;

    diesel::update(&contact)
        .set((
            first_name.eq(&contact.first_name),
            last_name.eq(&contact.last_name),
            phone_number.eq(&contact.phone_number),
            email.eq(&contact.email),
        ))
        .execute(conn)?;

    Ok(())
}

pub fn delete_contact(conn: &mut PgConnection, contact: Contact) -> Result<()> {
    diesel::delete(&contact).execute(conn)?;

    Ok(())
}

pub fn find_contact_by_name(
    conn: &mut PgConnection,
    f_name: String,
    l_name: String,
) -> Result<Contact> {
    use crate::schema::contacts::dsl::*;

    let res = contacts
        .filter(first_name.eq(f_name))
        .filter(last_name.eq(l_name))
        .first(conn)?;

    Ok(res)
}

pub fn search_contact_by_first_name(conn: &mut PgConnection, name: String) -> Result<Vec<Contact>> {
    use crate::schema::contacts::dsl::*;

    let pattern = format!("%{}%", name);

    let res = contacts.filter(first_name.ilike(&pattern)).load(conn)?;

    Ok(res)
}

pub fn search_contact_by_last_name(conn: &mut PgConnection, name: String) -> Result<Vec<Contact>> {
    use crate::schema::contacts::dsl::*;

    let pattern = format!("%{}%", name);

    let res = contacts.filter(last_name.ilike(&pattern)).load(conn)?;

    Ok(res)
}

pub fn search_contact_by_phone(conn: &mut PgConnection, phone: String) -> Result<Vec<Contact>> {
    use crate::schema::contacts::dsl::*;

    let pattern = format!("%{}%", phone);

    let res = contacts.filter(phone_number.ilike(&pattern)).load(conn)?;

    Ok(res)
}

pub fn search_contact_by_email(
    conn: &mut PgConnection,
    email_addr: String,
) -> Result<Vec<Contact>> {
    use crate::schema::contacts::dsl::*;

    let pattern = format!("%{}%", email_addr);

    let res = contacts.filter(email.ilike(&pattern)).load(conn)?;

    Ok(res)
}

// Define a custom SQL function for PostgreSQL's CONCAT_WS.
define_sql_function! {
    fn concat_ws(separator: Text, a: Text, b: Text) -> Text;
}

pub fn search_contacts_by_term(
    conn: &mut PgConnection,
    search_term: String,
) -> Result<Vec<Contact>> {
    use crate::schema::contacts::dsl::*;

    let pattern = format!("%{}%", search_term);

    let res = contacts
        .filter(
            first_name
                .ilike(&pattern)
                .or(last_name.ilike(&pattern))
                .or(phone_number.ilike(&pattern))
                .or(email.ilike(&pattern))
                .or(concat_ws(" ", first_name, last_name).ilike(&pattern)),
        )
        .load(conn)?;

    Ok(res)
}
