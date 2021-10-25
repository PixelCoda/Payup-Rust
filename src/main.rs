extern crate payup;

fn main() {

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
    cust = cust.post(auth.clone()).unwrap();

    // Fetch customers from stripe account
    let customers = payup::stripe::Customer::list_all(auth.clone());
    println!("customers: {:?}", customers);

    // Create a new plan
    let mut new_plan = payup::stripe::Plan::new();
    new_plan.amount = Some("200".to_string());
    new_plan.currency = Some("usd".to_string());
    new_plan.interval = Some("month".to_string());
    new_plan.product = Some("prod_KSywTYVmG9jVC4".to_string());
    new_plan.post(auth.clone());

    // Fetch plans from stripe account
    let plans = payup::stripe::Plan::list_all(auth.clone());
    println!("plans: {:?}", plans);


    
}
