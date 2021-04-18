mod invoice;

use wasm_bindgen::prelude::*;

extern crate serde;

// Beispiel: Zugriff auf JS Funktionen
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_i32(i: i32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_f32(f: f32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_strings(a: &str, b: &str);
}

#[wasm_bindgen(module = "/www/src/simpleMaths.js")]
extern "C" {
    pub fn add_nums(a: i32, b: i32) -> i32;
}

// Beispiel: Aufruf der JS Funktionen in Rust
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn log_functions(s: &str, i: i32, f: f32, a: &str, b: &str) {
    log(s);
    log_i32(i);
    log_f32(f);
    log_strings(a, b);
}

// Beispiel: Rust Funktion, welche in JS aufgerufen werden kann
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Beispiel: Rechnungsoperationen in Rust
#[wasm_bindgen]
pub struct InvoiceResult {
    total_quantity: i32,
    total_tax_amount: f32,
}

#[wasm_bindgen]
pub fn invoice_information(invoice: &JsValue) -> InvoiceResult {
    let invoice: invoice::Invoice = invoice.into_serde().unwrap();

    log(&format!(
        "Name: {} - Invoice Number: {} - Date: {}",
        invoice.name, invoice.invoice_number, invoice.date
    ));

    log(&format!("Products: {:?}", invoice.products));

    let total_quantity = invoice::get_total_quantity(invoice.clone());
    let total_tax_amount = invoice::get_total_tax_amount(invoice.clone());

    log(&format!(
        "Total Quantity: {} - Total Tax Amount: {}",
        total_quantity, total_tax_amount
    ));

    let invoice_result: InvoiceResult = InvoiceResult {
        total_quantity: total_quantity,
        total_tax_amount: total_tax_amount,
    };

    log_i32(invoice_result.total_quantity);
    log_f32(invoice_result.total_tax_amount);

    invoice_result
}
