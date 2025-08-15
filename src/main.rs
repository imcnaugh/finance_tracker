use clap::Parser;
use invoice_generator::model::NewLineItem;
use invoice_generator::service::LineItemService;

#[tokio::main]
async fn main() {
    let cli = Command::parse();
    match cli {
        Command::AddLineItem(line_item) => {
            match LineItemService::log_new_line_item(line_item).await {
                Ok(_) => {}
                Err(_) => eprintln!("Failed to log line item"),
            }
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
    AddLineItem(NewLineItem),
    #[command(about = "Print the report")]
    PrintReport,
}
