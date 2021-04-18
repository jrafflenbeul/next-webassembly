use serde::{Deserialize, Serialize};

const NORM_VALUE: u8 = 100;
const TAX_RATE: u8 = 19;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Product {
    pub qty: i32,
    pub name: String,
    pub price: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Invoice {
    pub name: String,
    pub invoice_number: i32,
    pub date: String,
    pub products: Vec<Product>,
}

impl Invoice {
    pub fn new(name: String, invoice_number: i32, date: String, products: Vec<Product>) -> Invoice {
        Invoice {
            name: name,
            invoice_number: invoice_number,
            date: date,
            products: products,
        }
    }

    fn calculate_total_quantity(&self) -> i32 {
        let mut total_quantity: i32 = 0;
        for product in self.products.iter() {
            total_quantity += product.qty;
        }
        total_quantity
    }

    fn calculate_total_tax_amount(&self, tax_rate: u8) -> f32 {
        let mut total_gross_amount: f32 = 0.0;
        for product in self.products.iter() {
            total_gross_amount += product.price as f32;
        }

        let total_tax_amount: f32 = total_gross_amount * tax_rate as f32 / NORM_VALUE as f32;

        total_tax_amount
    }
}

pub fn get_total_quantity(invoice: Invoice) -> i32 {
    let invoice = Invoice::new(
        invoice.name,
        invoice.invoice_number,
        invoice.date,
        invoice.products,
    );

    let total_quantity = invoice.calculate_total_quantity();

    total_quantity
}

pub fn get_total_tax_amount(invoice: Invoice) -> f32 {
    let invoice = Invoice::new(
        invoice.name,
        invoice.invoice_number,
        invoice.date,
        invoice.products,
    );

    let total_tax_amount = invoice.calculate_total_tax_amount(TAX_RATE);

    total_tax_amount
}
