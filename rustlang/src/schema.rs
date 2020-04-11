#![allow(unused)]
use super::*;
use crate::error::Error;
use crate::network::request::*;
use crate::transform::*;

gen_airtable_schema2! {

    invoice_rate_unit("Invoice Units") -> InvoiceRateUnit {
        fields {
            name {
                source = "Name";
            }
        }
        module {
            pure_fn!(get_name(unit: Mapped) -> String { Ok(unit.name) });
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
        module {
            pure_fn!(id_query(id: String) -> Param<Mapped> {
                Ok(Param::new_query("ID".to_string(), id))
            });
        }
        endpoints {
            query_by_invoice_id(String) -> Invoice {
                url_path { String }
                exec = id_query, one, Invoice::create_one;
            }
        }
    }

}
