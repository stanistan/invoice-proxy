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
            quantity {
                source = "Quantity";
            }
            amount(u32) -> String {
                source = "Amount";
                exec = money;
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
            quantity {
                source = "Quantity";
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
            id {
                source = "ID";
            }
            number {
                source = "Invoice Number";
            }
            notes {
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
