use clap::{Args, Parser};

fn main() {
    let cli = Command::parse();
    match cli {
        Command::AddLineItem(line_item) => {
            println!("Line item: {} - Quantity: {}, Price: ${:.2}", line_item.name, line_item.quantity, line_item.price);
        }
        Command::PrintReport => {
            println!("Printing report");
        }
    }
}

#[derive(Args)]
struct AddLineItem {
    name: String,
    quantity: f32,
    price: f32,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
enum Command {
    #[command(about = "Add a line item to the report")]
    AddLineItem(AddLineItem),
    #[command(about = "Print the report")]
    PrintReport,
}