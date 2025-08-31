use std::fs;
use std::process::Command;
use crate::model::client::Client;
use crate::model::invoice::Invoice;

pub fn generate_pdf(invoice: &Invoice, client: &Client) {
    let template = fs::read_to_string("invoice_manager/template/invoice.tex")
        .expect("Failed to read template file");

    let sent_date = invoice.get_sent_date().ok()
        .flatten()
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_default();

    let line_items = invoice.get_line_items().iter().map(|li| {
        let unit_price = li.get_unit_price_in_cents() as f64 / 100.0;
        let total = li.get_total_in_cents() as f64 / 100.0;
        format!("{} & {:.2} & \\${:.2} & \\${:.2} \\\\ \n \\hline", li.get_description(), li.get_quantity(), unit_price, total)
    }).collect::<Vec<String>>().join("");

    let total = invoice.get_line_items().iter().map(|li| li.get_total_in_cents()).sum::<i32>() as f64 / 100.0;
    let total = format!("{:.2}", total);

    let filled = template
        .replace("@@CLIENT_NAME@@", client.get_name())
        .replace("@@CLIENT_ADDRESS@@", client.get_address().unwrap_or_default())
        .replace("@@SENT_DATE@@", &sent_date)
        .replace("@@INVOICE_ID@@", &invoice.get_id().to_string())
        .replace("@@LINE_ITEMS@@", &line_items)
        .replace("@@TOTAL@@", &total);

    fs::write("template_tmp.tex", filled)
        .expect("Failed to write template file");

    Command::new("tectonic")
        .arg("template_tmp.tex")
        .status()
        .expect("Failed to run tectonic");
}
