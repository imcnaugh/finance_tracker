# Models

```mermaid
classDiagram
    
    class NewLineItem{
        -String description
        -f64 quantity
        -f64 unit_price
        -String invoice_id
        
        +get_description()
        +get_quantity()
        +get_unit_price()
        +get_invoice_id()

        ~new(description: String, quantity: f64, price: f64, invoice_id: String)
    }
    
    class NewClient{
        -String name
        
        ~new(name: String)
        +get_name()
    }
    
    class LineItem{
        -String id
        -String description
        -i32 price_in_cents
        -f64 quantity
        -String invoice_id 
        
        ~new(id: String, description: String, unit_price_in_cents: i32, quantity: f64, invoice_id: String)
        ~from(new_line_item: NewLineItem)
        ~get_id()
        ~get_description()
        ~get_unit_price_in_cents()
        ~get_quantity()
        ~get_invoice_id()

        ~get_total_in_cents()
    }

    
    class InvoiceStatus
    <<Enumeration>> InvoiceStatus
    InvoiceStatus : DRAFT
    InvoiceStatus : SENT
    InvoiceStatus : PAID
    InvoiceStatus : OVERDUE
    InvoiceStatus : CANCELLED
    
    class Invoice {
        -String id
        -String client_id
        -InvoiceStatus status
        
        ~new(id: String, client_id: String, line_items: Vec<LineItem>)
        ~get_id()
        !get_client_id()
        ~get_status()
        ~get_line_items()
    }
    
    class Client {
        -String id
        -String name

        ~from(new_client: NewClient
        ~new(id: String, name: String)
        ~get_id()
        ~get_name())
    }
    
    Invoice "1" o-- "0..*" LineItem
```


