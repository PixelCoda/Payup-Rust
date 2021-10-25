extern crate payup;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().with_colors(true).init().unwrap();
    
    // Client and Secret for Stripe account
    // In a production environment...load values from environment variables.
    let client = format!("sk_test_51Jo2sKGrEH09RU9uu8d8ARKasYUKHXAHk4vUNup1JLgP5wFnQQf6t7UpKfh7woVMhI9oeuziolW2dK1uwmgAheVI00bN8ews6g");
    let secret = format!("");

    // Create the Authentication refererence
    let auth = payup::stripe::Auth::new(client, secret);
    
    // Build a customer object
    let mut cust = payup::stripe::Customer::new();
    cust.name = Some("Rust Test".to_string());
    cust.description = Some("A test customer from rust.".to_string());
    cust.phone = Some("333-333-3333".to_string());
    cust.email = Some("rust@test.com".to_string());
    cust.payment_method = None;
    
    // Post customer to stripe and update the local cust variable
    cust = cust.post(auth).unwrap();

    // Cust now has an ID from stripe.
    println!("{:?}", cust);








}
