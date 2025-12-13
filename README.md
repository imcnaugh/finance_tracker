# Finance Tracker

A simple CLI for tracking finances of a small business.

Features include:
 - Invoice generation and management
 - Basic account tracking


## TODO
 - Allow invoice management to affect account balances
   - Newly sent invoices should effect account balances
   - Incoming payments should effect account balances and mark invoices as paid

## Things to consider
 - Removing the struts for service layer.
   - In theory there is no state for the service layer, so there should be no need for a struct.
   - Or is there in that it is dependent on a storage layer?
 - Implementing an event based system for interaction between services.
