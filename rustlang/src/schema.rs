use airtable_proxy::error::Error;
use airtable_proxy::*;

//
// FIXME: this doesn't do anything with decimals :(
pure!(
    fn money(val: u32) -> String {
        use num_format::{Locale, WriteFormatted};
        let mut buf = String::from("$");
        if buf.write_formatted(&val, &Locale::en).is_err() {
            return Err(Error::Transform {
                message: "could not format money"
            });
        }
        buf.push_str(".00");
        Ok(buf)
    }
);

gen_airtable_schema! {

    invoice_rate_unit("Invoice Units") -> InvoiceRateUnit {
        fields {
            name {
                source = "Name";
            }
        }
        module {
            pure!(get_name(unit: Mapped) -> String { unit.name });
        }
    }

    invoice_item_rate("Invoice Rates") -> InvoiceRate {
        fields {
            name {
                source = "Name";
            }
            notes -> Option<String> {
                source = "Notes";
            }
            rate -> u32 {
                source = "Rate";
            }
            unit(IDs) -> String {
                source = "Unit";
                exec = InvoiceRateUnit::fetch_and_create_first, invoice_rate_unit::get_name;
            }
        }
    }

    invoice_item("Invoice Item") -> InvoiceItem {
        fields {
            date {
                source = "Date";
            }
            description {
                source = "Description";
            }
            quantity -> u32 {
                source = "Quantity";
            }
            amount(u32) -> String {
                source = "Amount";
                exec = money;
            }
            rate(IDs) -> InvoiceRate {
                source = "Invoice Rate";
                exec = InvoiceRate::fetch_and_create_first;
            }
        }
    }

    invoice_client("Clients") -> InvoiceClient {
        fields {
            company {
                source = "Company";
            }
            contact_email {
                source = "ContactEmail";
            }
            contact_name {
                source = "ContactName";
            }
            website_url {
                source = "Website";
            }
        }
    }

    invoice_from("Me") -> InvoiceFrom {
        fields {
            name {
                source = "Name";
            }
            email {
                source = "Email";
            }
            address(String) -> Vec<String> {
                source = "Address";
                exec = split_lines;
            }
        }
    }

    invoice("Invoice") -> Invoice {
        fields {
            id -> u32 {
                source = "ID";
            }
            number {
                source = "Invoice Number";
            }
            notes -> Option<String> {
                source = "Notes";
            }
            date {
                source = "Date";
            }
            due_date {
                source = "Due Date";
            }
            was_sent(MaybeBool) -> bool {
                source = "Sent?";
                exec = force_bool;
            }
            was_paid(MaybeBool) -> bool {
                source = "Paid?";
                exec = force_bool;
            }
            total(u32) -> String {
                source = "Total Amount";
                exec = money;
            }
            from(IDs) -> InvoiceFrom {
                source = "From";
                exec = InvoiceFrom::fetch_and_create_first;
            }
            client(IDs) -> InvoiceClient {
                source = "Client";
                exec = InvoiceClient::fetch_and_create_first;
            }
            items(IDs) -> Vec<InvoiceItem> {
                source = "Invoice Item";
                exec = InvoiceItem::fetch_and_create_many;
            }
        }
        endpoints {
            query_by_invoice_id(String) -> Invoice {
                url_path { String }
                exec = param::as_id_query, one, Invoice::create_one;
            }
            find_by_record_id(String) -> Invoice {
                url_path { String }
                exec = into_vec, Invoice::fetch_and_create_first;
            }
        }
    }
}
