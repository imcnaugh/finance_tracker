use crate::model::{Client, CompanyConfiguration, Invoice};
use num_format::{Locale, ToFormattedString};
use std::fs;
use std::io::Write;
use std::process::Command;
// use tectonic::latex_to_pdf;

pub fn generate_pdf(
    invoice: &Invoice,
    client: &Client,
    company_configurations: &CompanyConfiguration,
) {
    let format_cents = |total_cents: i32| {
        let dollars = (total_cents / 100).to_formatted_string(&Locale::en);
        let cents = total_cents % 100;
        format!("{}.{:02}", dollars, cents)
    };

    let template = fs::read_to_string("invoice_manager/template/invoice.tex")
        .expect("Failed to read template file");

    let sent_date = invoice
        .get_sent_date()
        .ok()
        .flatten()
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_default();

    let line_items = invoice
        .get_line_items()
        .iter()
        .map(|li| {
            let unit_price = format_cents(li.get_unit_price_in_cents());
            let total = format_cents(li.get_total_in_cents());
            format!(
                " {} & {} & \\${} & \\${} \\\\ \n \\hline ",
                li.get_description(),
                li.get_quantity(),
                unit_price,
                total
            )
        })
        .collect::<Vec<String>>()
        .join("");

    let total = invoice
        .get_line_items()
        .iter()
        .map(|li| li.get_total_in_cents())
        .sum::<i32>();
    let total = format_cents(total);

    let client_contact = [
        client.get_address().unwrap_or_default(),
        client.get_invoice_email().unwrap_or_default(),
    ]
    .iter()
    .filter(|s| !s.is_empty())
    .cloned()
    .collect::<Vec<&str>>()
    .join("\\\\ \n")
    .replace("\n", "\\\\ \n");

    let company_address = company_configurations
        .get_address()
        .replace("\n", "\\\\ \n");

    let filled = template
        .replace("@@COMPANY_NAME@@", company_configurations.get_name())
        .replace("@@COMPANY_ADDRESS@@", &company_address)
        .replace("@@COMPANY_EMAIL@@", company_configurations.get_email())
        .replace("@@CLIENT_NAME@@", client.get_name())
        .replace("@@CLIENT_CONTACT@@", &client_contact)
        .replace("@@SENT_DATE@@", &sent_date)
        .replace("@@INVOICE_ID@@", invoice.get_id())
        .replace("@@LINE_ITEMS@@", &line_items)
        .replace("@@TOTAL@@", &total);

    // let pdf_bytes = latex_to_pdf(filled.as_bytes()).expect("LaTeX compilation failed");
    //
    // fs::write("invoice.pdf", &pdf_bytes).expect("Failed to write PDF file");

    call_cli(filled);
}

fn call_cli(filled: String) {
    let mut child = Command::new("tectonic")
        .arg("-")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(filled.as_bytes())
        .unwrap();

    child.wait().unwrap();
}
