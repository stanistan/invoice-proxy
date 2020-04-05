#![allow(unused)]
use super::*;
use crate::error::Error;
use crate::network::request::*;
use crate::transform::*;

gen_airtable_schema! {

    invoice_rate_unit("Invoice Units")
        as InvoiceUnit {
            "Name" => fn name(String) -> String { id },
        },
        mod {
            pure_fn!(get_name(unit: Mapped) -> String { Ok(unit.name) });
        }, endpoints {
            query(String) -> InvoiceItem {
                into_vec,
                InvoiceItem::fetch_and_create_first
            }
        };

    invoice_item_rate("Invoice Rates")
        as InvoiceRate {
            "Name" => fn name(String) -> String { id },
            "Notes" => fn notes(Option<String>) -> Option<String> { id },
            "Rate" => fn rate(u32) -> u32 { copy },
            "Unit" => fn unit(IDs) -> String {
                InvoiceUnit::fetch_and_create_first,
                invoice_rate_unit::get_name
            },
        }, mod {
        }, endpoints {
            query(String) -> InvoiceItem {
                into_vec,
                InvoiceItem::fetch_and_create_first
            }
        };

    invoice_item("Invoice Item")
        as InvoiceItem {
            "Date" => fn date(String) -> String { id },
            "Description" => fn description(String) -> String { id },
            "Quantity" => fn quantity(u32) -> u32 { copy },
            "Amount" => fn amount(u32) -> String { money },
            "Invoice Rate" => fn rate(IDs) -> InvoiceRate {
                InvoiceRate::fetch_and_create_first
            },
        }, mod {
        }, endpoints {
            query(String) -> InvoiceItem {
                into_vec,
                InvoiceItem::fetch_and_create_first
            }
        };

    invoice_client("Clients")
        as InvoiceClient {
            "Company" => fn company(String) -> String { id },
            "ContactEmail" => fn contact_email(String) -> String { id },
            "ContactName" => fn contact_name(String) -> String { id },
            "Website" => fn website_url(String) -> String { id },
        }, mod {
        }, endpoints {
            query(String) -> InvoiceClient {
                into_vec,
                InvoiceClient::fetch_and_create_first
            }
        };

    invoice_from("Me")
        as InvoiceFrom {
            "Name" => fn name(String) -> String { id },
            "Email" => fn email(String) -> String { id },
            "Address" => fn address(String) -> Vec<String> { split_lines },
        },
        mod { },
        endpoints {
            query(String) -> InvoiceFrom {
                into_vec,
                InvoiceFrom::fetch_and_create_first
            }
        };

    invoice("Invoice")
        as Invoice {
            "ID" => fn id(u32) -> u32 { copy },
            "Invoice Number" => fn number(String) -> String { id },
            "Notes" => fn notes(Option<String>) -> Option<String> { id },
            "Date" => fn date(String) -> String { id },
            "Due Date" => fn due_date(String) -> String { id },
            "Sent?" => fn was_sent(MaybeBool) -> bool { force_bool },
            "Paid?" => fn was_paid(MaybeBool) -> bool { force_bool },
            "Total Amount" => fn total(u32) -> String { money },
            "From" => fn from(IDs) -> InvoiceFrom {
                InvoiceFrom::fetch_and_create_first
            },
            "Client" => fn client(IDs) -> InvoiceClient {
                InvoiceClient::fetch_and_create_first
            },
            "Invoice Item" => fn items(IDs) -> Vec<InvoiceItem> {
                InvoiceItem::fetch_and_create_many
            },
        },
        mod {
            pure_fn!(id_query(id: String) -> Param<Invoice> {
                Ok(Param::new_query("ID".to_string(), id))
            });
        },
        endpoints {
            query(String) -> Invoice {
                id_query,
                one,
                Invoice::create_one
            }
        };

}
