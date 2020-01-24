use super::*;

gen_airtable_schema! {

    mod invoice_item("Invoice Item") {
        "Date" => fn date(String) -> String { id },
        "Description" => fn description(String) -> String { id },
        "Quantity" => fn quantity(u32) -> u32 { copy },
        "Amount" => fn amount(u32) -> u32 { copy },
    }

    mod invoice_client("Clients") {
        "Company" => fn company(String) -> String { id },
        "ContactEmail" => fn contact_email(String) -> String { id },
        "ContactName" => fn contact_name(String) -> String { id },
        "Website" => fn website_url(String) -> String { id },
    }

    mod invoice("Invoice") {
        "ID" => fn id(u32) -> u32 { copy },
        "Invoice Number" => fn number(String) -> String { id },
        "Notes" => fn notes(Option<String>) -> Option<String> { id },
        "Date" => fn date(String) -> String { id },
        "Due Date" => fn due_date(String) -> String { id },
        "Sent" => fn was_sent(MaybeBool) -> bool { force_bool },
        "Paid" => fn was_paid(MaybeBool) -> bool { force_bool },
        "Total Amount" => fn total(u32) -> String { money },
        "Client" => fn client(IDs) -> invoice_client::Mapped {
            first,
            invoice_client::get_one,
            invoice_client::map
        },
        "Invoice Item" => fn items(IDs) -> IDs { id },
    }

}

