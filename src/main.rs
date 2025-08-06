use clap::Parser;
use invoice_generator::LineItem;

fn main() {
    let cli = Command::parse();
    match cli {
        Command::AddLineItem(line_item) => {
            println!(
                "Line item: {} - Quantity: {}, Price: ${:.2}, Total: ${:.2}",
                line_item.get_name(),
                line_item.get_quantity(),
                line_item.get_price(),
                line_item.get_total()
            );
        }
        Command::PrintReport => {
            println!("Printing report");
        }
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
enum Command {
    #[command(about = "Add a line item to the report")]
    AddLineItem(LineItem),
    #[command(about = "Print the report")]
    PrintReport,
}
