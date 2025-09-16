use clap::Subcommand;
use invoice_manager::model::{InvoiceSearch, NewLineItem};

#[derive(Subcommand)]
pub enum InvoiceSubCommands {
    /// Create a new invoice for a client
    New { client_id: String },

    /// Get an invoice by id
    #[command(visible_aliases = ["show", "print"])]
    Get { invoice_id: String },

    /// List invoices with options to search
    #[command(
        visible_alias = "ls",
        long_about = "Dates are set to start at the beginning of the day, so to search for a single day
enter 2025-11-07..2025-11-08, that will search from the start of the 7th to the
start of the 8th."
    )]
    List {
        #[command(flatten)]
        search_options: Option<InvoiceSearch>,
    },

    /// Add a new line item to an invoice
    AddItem {
        invoice_id: String,

        #[command(flatten)]
        new_line_item: NewLineItem,
    },

    /// Delete a line item from an invoice
    DeleteItem {
        invoice_id: String,
        line_item_id: String,
    },

    /// Send a draft invoice
    Send {
        invoice_id: String,
        #[arg(short, long)]
        generate_pdf: bool,
    },

    /// Mark an invoice as paid
    Paid { invoice_id: String },

    /// Cancel an invoice
    Cancel { invoice_id: String },

    /// Generate a PDF for an invoice
    #[command(visible_alias = "pdf")]
    GeneratePdf { invoice_id: String },
}
