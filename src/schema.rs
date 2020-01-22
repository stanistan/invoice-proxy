use super::*;

gen_airtable_schema! {
    invoice_item
        - from: "Invoice Item",
        - fields: [
            "Date" => { date(String) -> id: &'a str },
            "Description" => { description(String) -> id: &'a str },
            "Quantity" => { quantity(u32) -> copy: u32 },
            "Amount" => { amount(u32) -> copy: u32 },
        ]
}

gen_airtable_schema! {
    invoice_client
        - from: "Clients",
        - fields: [
            "Company" => { company(String) -> id: &'a str },
            "ContactEmail" => { contact_email(String) -> id: &'a str },
            "ContactName" => { contact_name(String) -> id: &'a str },
            "Website" => { website_url(String) -> id: &'a str },
        ]
}

gen_airtable_schema! {
    invoice
        - from: "Invoice",
        - fields: [
            "ID" => { id(u32) -> copy: u32 },
            "Invoice Number" => { number(String) -> id: &'a str },
            "Notes" => { notes(Option<String>)  -> id: &'a Option<String> },
            "Due Date" => { due_date(String) -> id: &'a str },
            "Date" => { date(String) -> id: &'a str },
            "Paid" => { paid(MaybeBool) -> force_bool: bool },
            "Sent" => { sent(MaybeBool) -> force_bool: bool },
            "Client" => { client(IDs) -> first: &'a String },
            "Invoice Item" => { items(IDs) -> id: &'a IDs },
        ]
}

