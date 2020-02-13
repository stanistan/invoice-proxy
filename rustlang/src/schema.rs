use super::*;
use crate::error::Error;
use crate::transform::*;

gen_airtable_schema! {

    mod invoice_rate_unit("Invoice Units") as InvoiceUnit {
        "Name" => fn name(String) -> String { id },
    } {
        /// Extract the name from the rate unit.
        pub async fn get_name(_ctx: &FetchCtx, unit: Mapped) -> Result<String, Error> {
            Ok(unit.name)
        }
    }

    mod invoice_item_rate("Invoice Rates") as InvoiceRate {
        "Name" => fn name(String) -> String { id },
        "Notes" => fn notes(Option<String>) -> Option<String> { id },
        "Rate" => fn rate(u32) -> u32 { copy },
        "Unit" => fn unit(IDs) -> String {
            first,
            InvoiceUnit::fetch_one,
            InvoiceUnit::create_one,
            invoice_rate_unit::get_name
        },
    }

    mod invoice_item("Invoice Item") as InvoiceItem {
        "Date" => fn date(String) -> String { id },
        "Description" => fn description(String) -> String { id },
        "Quantity" => fn quantity(u32) -> u32 { copy },
        "Amount" => fn amount(u32) -> String { money },
        "Invoice Rate" => fn rate(IDs) -> InvoiceRate {
            first,
            InvoiceRate::fetch_one,
            InvoiceRate::create_one
        },
    }

    mod invoice_client("Clients") as InvoiceClient {
        "Company" => fn company(String) -> String { id },
        "ContactEmail" => fn contact_email(String) -> String { id },
        "ContactName" => fn contact_name(String) -> String { id },
        "Website" => fn website_url(String) -> String { id },
    }

    mod invoice_from("Me") as InvoiceFrom {
        "Name" => fn name(String) -> String { id },
        "Email" => fn email(String) -> String { id },
        "Address" => fn address(String) -> Vec<String> { split_lines },
    }

    mod invoice("Invoice") as Invoice {
        "ID" => fn id(u32) -> u32 { copy },
        "Invoice Number" => fn number(String) -> String { id },
        "Notes" => fn notes(Option<String>) -> Option<String> { id },
        "Date" => fn date(String) -> String { id },
        "Due Date" => fn due_date(String) -> String { id },
        "Sent?" => fn was_sent(MaybeBool) -> bool { force_bool },
        "Paid?" => fn was_paid(MaybeBool) -> bool { force_bool },
        "Total Amount" => fn total(u32) -> String { money },
        "From" => fn from(IDs) -> InvoiceFrom {
            first,
            InvoiceFrom::fetch_one,
            InvoiceFrom::create_one
        },
        "Client" => fn client(IDs) -> InvoiceClient {
            first,
            InvoiceClient::fetch_one,
            InvoiceClient::create_one
        },
        "Invoice Item" => fn items(IDs) -> Vec<InvoiceItem> {
            InvoiceItem::fetch_many,
            InvoiceItem::create_many
        },
    }

}
