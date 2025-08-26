use clap::Subcommand;
use invoice_manager::model::invoice_status::InvoiceStatus;
use invoice_manager::model::{InvoiceSearch, NewLineItem};

#[derive(Subcommand)]
pub enum InvoiceSubCommands {
    New {
        client_id: String,
    },

    Get {
        invoice_id: String,
    },

    #[command(alias = "ls")]
    List {
        #[command(flatten)]
        search_options: Option<InvoiceSearch>,
    },

    AddItem {
        invoice_id: String,

        #[command(flatten)]
        new_line_item: NewLineItem,
    },

    DeleteItem {
        invoice_id: String,
        line_item_id: String,
    },

    Update {
        invoice_id: String,
        status: InvoiceStatus,

        #[arg(short, long)]
        generate_pdf: bool,
    },

    GeneratePdf {
        invoice_id: String,
    },
}
