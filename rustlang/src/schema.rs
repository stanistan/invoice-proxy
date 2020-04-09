#![allow(unused)]
use super::*;
use crate::error::Error;
use crate::network::request::*;
use crate::transform::*;

gen_airtable_schema2! {

    invoice_rate_unit("Invoice Units") -> InvoiceRateUnit {
        fields {
            name(String) -> String {
                name = "Name";
            }
        }
        module {
            pure_fn!(get_name(unit: Mapped) -> String { Ok(unit.name) });
        }
        endpoints {

        }
    }

    invoice_item_rate("Invoice Rates") -> InvoiceRate {
        fields {
            name(String) -> String {
                name = "Name";
            }
            notes(Option<String>) -> Option<String> {
                name = "Notes";
            }
            quantity(u32) -> u32 {
                name = "Quantity";
            }
            amount(u32) -> String {
                name = "Amount";
                exec = money;
            }
            unit(IDs) -> String {
                name = "Unit";
                exec = InvoiceRateUnit::fetch_and_create_first, invoice_rate_unit::get_name;
            }
        }
    }

    invoice_item("Invoice Item") -> InvoiceItem {
        fields {
            date(String) -> String {
                name = "Date";
            }
            description(String) -> String {
                name = "Description";
            }
            quantity(u32) -> u32 {
                name = "Quantity";
            }
            rate(IDs) -> InvoiceRate {
                name = "Invoice Rate";
                exec = InvoiceRate::fetch_and_create_first;
            }
        }
    }

    invoice_client("Clients") -> InvoiceClient {
        fields {
            company(String) -> String {
                name = "Company";
            }
            contact_email(String) -> String {
                name = "ContactEmail";
            }
            contact_name(String) -> String {
                name = "ContactName";
            }
            website_url(String) -> String {
                name = "Website";
            }
        }
    }

    invoice_from("Me") -> InvoiceFrom {
        fields {
            name(String) -> String {
                name = "Name";
            }
            email(String) -> String {
                name = "Email";
            }
            address(String) -> Vec<String> {
                name = "Address";
                exec = split_lines;
            }
        }
    }

    invoice("Invoice") -> Invoice {
        fields {
            id(u32) -> u32 {
                name = "ID";
            }
            number(String) -> String {
                name = "Invoice Number";
            }
            notes(String) -> String {
                name = "Notes";
            }
            date(String) -> String {
                name = "Date";
            }
            due_date(String) -> String {
                name = "Due Date";
            }
            was_sent(MaybeBool) -> bool {
                name = "Sent?";
                exec = force_bool;
            }
            was_paid(MaybeBool) -> bool {
                name = "Paid?";
                exec = force_bool;
            }
            total(u32) -> String {
                name = "Total Amount";
                exec = money;
            }
            from(IDs) -> InvoiceFrom {
                name = "From";
                exec = InvoiceFrom::fetch_and_create_first;
            }
            client(IDs) -> InvoiceClient {
                name = "Client";
                exec = InvoiceClient::fetch_and_create_first;
            }
            items(IDs) -> Vec<InvoiceItem> {
                name = "Invoice Item";
                exec = InvoiceItem::fetch_and_create_many;
            }
        }
    }

    /*
    invoice_rate_unit("Invoice Units")
        as InvoiceUnit {
            "Name" => fn name(String) -> String { id },
        },
        mod {
            pure_fn!(get_name(unit: Mapped) -> String { Ok(unit.name) });
        }, endpoints { };

    invoice_item_rate("Invoice Rates")
        as InvoiceRate {
            "Name" => fn name(String) -> String { id },
            "Notes" => fn notes(Option<String>) -> Option<String> { id },
            "Rate" => fn rate(u32) -> u32 { copy },
            "Unit" => fn unit(IDs) -> String {
                InvoiceUnit::fetch_and_create_first,
                invoice_rate_unit::get_name
            },
        }, mod { }, endpoints { };

    invoice_item("Invoice Item")
        as InvoiceItem {
            "Date" => fn date(String) -> String { id },
            "Description" => fn description(String) -> String { id },
            "Quantity" => fn quantity(u32) -> u32 { copy },
            "Amount" => fn amount(u32) -> String { money },
            "Invoice Rate" => fn rate(IDs) -> InvoiceRate {
                InvoiceRate::fetch_and_create_first
            },
        }, mod { }, endpoints { };

    invoice_client("Clients")
        as InvoiceClient {
            "Company" => fn company(String) -> String { id },
            "ContactEmail" => fn contact_email(String) -> String { id },
            "ContactName" => fn contact_name(String) -> String { id },
            "Website" => fn website_url(String) -> String { id },
        }, mod { }, endpoints { };

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

    invoice("Invoice") -> Invoice {
        fields {
            id(u32) -> u32 {
                name = "ID";
            }
            number(String) -> String {
                name = "Invoice Number";
            }
        }
        endpoints {

        }
    }

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
            query_by_invoice_id(String) -> Invoice {
                path = String;
                exec = id_query, one, Invoice::create_one;
            }
            find_by_record_id(String) -> Invoice {
                path = "record" / String;
                exec = into_vec, Invoice::fetch_and_create_first;
            }
        };

            */
}
