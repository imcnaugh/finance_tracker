use clap::Subcommand;
use invoice_manager::model::invoice_status::InvoiceStatus;
use invoice_manager::model::{InvoiceSearch, NewLineItem};

#[derive(Subcommand)]
pub enum InvoiceSubCommands {
    /// Create a new invoice for a client
    New { client_id: String },

    /// Get an invoice by id
    Get { invoice_id: String },

    /// List invoices with options to search
    #[command(visible_alias = "ls")]
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

    /// Update an invoice's status
    #[command(visible_alias = "u")]
    UpdateStatus {
        invoice_id: String,
        status: InvoiceStatus,

        /// Generate a PDF for an invoice
        #[arg(short, long)]
        generate_pdf: bool,
    },

    /// Generate a PDF for an invoice
    #[command(visible_alias = "pdf")]
    GeneratePdf { invoice_id: String },
}
