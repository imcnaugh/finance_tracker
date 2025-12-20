# Finance Tracker

A simple CLI for tracking finances of a small business.

Features include:
 - Invoice generation and management
 - Basic account tracking


## TODO
 - Allow invoice management to affect account balances
   - <a id="modifyAccountOnInvoiceSend">Newly sent invoices should affect account balances</a>
   - <a id="acceptPayments">Incoming payments should affect account balances and mark invoices as paid</a>

## Policy
 - Client management
   - We should have the ability to add, edit, and delete clients.
   - TODO: consider breaking this into its own module. Currently, this is tightly coupled with invoice management.
 - Invoices management
   - Invoices should always be in one of the following states.
     - Draft
     - Sent
     - Paid
     - Overdue
     - Cancelled
   - New invoices should always start in the draft state.
   - Only invoices in the draft state may have their line items modified.
   - Invoices in the draft state may be sent. The sent state represents an invoice that has been sent to the customer.
   - Invoices in the sent state can be moved to the paid state.
   - If an invoice has a due date, and that date has passed, then the invoice should be moved to the overdue state.
   - At any time an invoice may be canceled.
     - TODO: Consider adding a cancel reason.
 - Double entry accounting
   - A list of Account types should be maintained. each has a name and default balance. A default list will be provided as such
     - asset: Debit
     - liability: Credit
     - equity: Credit
     - revenue: Credit
     - expense: Debit
   - A list of accounts should be maintained, each account is required to have a name and Account Type, a default balance will be provided as such.
     - cash: asset
     - accounts receivable: asset
     - accounts payable: liability
     - tax payable: liability
     - owner equity: equity
     - owner drawings: equity
     - revenues: revenue
     - operating expense: expense
     - taxes paid: expense
   - Account types may be deleted if they are not in use by any accounts.
   - TODO: consider adding a way to delete accounts.
   - TODO: Implement a check for the accounting equation to ensure that all accounts are balanced.
   - TODO: Implement a reconciliation process for end of a period.
   - Account balances are not stored directly, rather they are calculated by the summing journal transaction associated with the account.
 - [TODO: Hash this idea out more](#modifyAccountOnInvoiceSend) when an invoice is moved to the sent state, accounts should be updated to reflect the change.
 - [TODO: Hash this idea out more](#acceptPayments) Have a mechinasim to notify the system that a payment has been received.
   - If the payment is associated with an invoice, and the total amount received for the invoice is greater than or equal to the invoice total, move the invoice to the paid state.
   - Consider overpayment handling.
   - Consider payments not associated with an invoice.
   - Consider associating payments with a client and not an invoice.
     

## Details
