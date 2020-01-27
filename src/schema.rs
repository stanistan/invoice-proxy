use super::*;
use crate::transform::*;

gen_airtable_schema! {

    mod invoice_rate_unit("Invoice Units") {
        "Name" => fn name(String) -> String { id },
    } {
        /// Extract the name from the rate unit.
        pub async fn get_name(_ctx: &FetchCtx, unit: Mapped) -> Result<String, Error> {
            Ok(unit.name)
        }
    }

    mod invoice_item_rate("Invoice Rates") {
        "Name" => fn name(String) -> String { id },
        "Notes" => fn notes(Option<String>) -> Option<String> { id },
        "Rate" => fn rate(u32) -> u32 { copy },
        "Unit" => fn unit(IDs) -> String {
            first,
            invoice_rate_unit::one::get,
            invoice_rate_unit::one::map,
            invoice_rate_unit::get_name
        },
    }

    mod invoice_item("Invoice Item") {
        "Date" => fn date(String) -> String { id },
        "Description" => fn description(String) -> String { id },
        "Quantity" => fn quantity(u32) -> u32 { copy },
        "Amount" => fn amount(u32) -> String { money },
        "Invoice Rate" => fn rate(IDs) -> invoice_item_rate::Mapped {
            first,
            invoice_item_rate::one::get,
            invoice_item_rate::one::map
        },
    }

    mod invoice_client("Clients") {
        "Company" => fn company(String) -> String { id },
        "ContactEmail" => fn contact_email(String) -> String { id },
        "ContactName" => fn contact_name(String) -> String { id },
        "Website" => fn website_url(String) -> String { id },
    }

    mod invoice_from("Me") {
        "Name" => fn name(String) -> String { id },
        "Email" => fn email(String) -> String { id },
        "Address" => fn address(String) -> Vec<String> { split_lines },
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
        "From" => fn from(IDs) -> invoice_from::Mapped {
            first,
            invoice_from::one::get,
            invoice_from::one::map
        },
        "Client" => fn client(IDs) -> invoice_client::Mapped {
            first,
            invoice_client::one::get,
            invoice_client::one::map
        },
        "Invoice Item" => fn items(IDs) -> Vec<invoice_item::Mapped> {
            invoice_item::many::get,
            invoice_item::many::map
        },
    }

}

