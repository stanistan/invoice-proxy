use super::*;

gen_airtable_schema! {

    mod invoice_item("Invoice Item") {
        "Date" => fn date(String) -> &'a str { id },
        "Description" => fn description(String) -> &'a str { id },
        "Quantity" => fn quantity(u32) -> u32 { copy },
        "Amount" => fn amount(u32) -> u32 { copy },
    }

    mod invoice_client("Clients") {
        "Company" => fn company(String) -> &'a str { id },
        "ContactEmail" => fn contact_email(String) -> &'a str { id },
        "ContactName" => fn contact_name(String) -> &'a str { id },
        "Website" => fn website_url(String) -> &'a str { id },
    }

    mod invoice("Invoice") {
        "ID" => fn id(u32) -> u32 { copy },
        "Invoice Number" => fn number(String) -> &'a str { id },
        "Notes" => fn notes(Option<String>) -> &'a Option<String> { id },
        "Date" => fn date(String) -> &'a str { id },
        "Due Date" => fn due_date(String) -> &'a str { id },
        "Sent" => fn was_sent(MaybeBool) -> bool { force_bool },
        "Paid" => fn was_paid(MaybeBool) -> bool { force_bool },
        "Client" => fn client(IDs) -> &'a String { first },
        "Invoice Item" => fn items(IDs) -> &'a IDs { id },
    }

}

