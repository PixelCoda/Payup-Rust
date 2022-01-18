# Payup

## Description

A synchronous + asynchronous payment library for processing payments with rust. Stripe is quasi-supported if you don't need the full api.

I built this library due to a lack of synchronous payment libraries. Currently im only focused on features I need for another project. 

Roadmap:
* 0.1.0: Quasi-Stripe Support
    * Ability to list/create Customers (DONE)
    * Ability to list/create Plans (DONE)
    * Ability to create PaymentMethods from a Card and attach to a Customer (DONE)
    * Ability to subscribe user to a plan (DONE)
    * Ability to cancel a subscription to a plan (DONE)
* 0.2.0: Full Stripe API Support
* 0.3.0: Paypal Support
* 0.4.0: Cryptocurrency Support

## How to use library

Add the following line to your cargo.toml:
```
payup = "0.1.2"
```

Example:
```rust

extern crate payup;

fn main() {

    // Client and Secret for Stripe account
    // In a production environment...load values from environment variables.
    let client = format!("sk_test_XXXX");
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
    // For async use: cust.post_async(auth.clone()).await?;
    cust = cust.post(auth.clone()).unwrap();

    // Fetch customers from stripe account
    let customers = payup::stripe::Customer::list_all(auth.clone());
    println!("customers: {:?}", customers);

    // Create a new plan
    let mut new_plan = payup::stripe::Plan::new();
    new_plan.amount = Some("200".to_string());
    new_plan.currency = Some("usd".to_string());
    new_plan.interval = Some("month".to_string());
    new_plan.product = Some("prod_XXX".to_string());
    new_plan = new_plan.post(auth.clone()).unwrap();

    // Fetch plans from stripe account
    let plans = payup::stripe::Plan::list_all(auth.clone());
    println!("plans: {:?}", plans);

    // Create a new card
    let mut card = payup::stripe::Card::new();
    card.number = Some(format!("4242424242424242"));
    card.exp_month = Some(format!("01"));
    card.exp_year = Some(format!("2023"));
    card.cvc = Some(format!("314"));

    // Create a payment method from the card
    let mut payment_method = payup::stripe::PaymentMethod::new();
    payment_method.method_type = Some(format!("card"));
    payment_method.card = Some(card);
    payment_method = payment_method.post(auth.clone()).unwrap();
    println!("payment_method: {:?}", payment_method.clone());

    // Attach the payment method to the customer created earlier
    let attached = payment_method.attach(cust.clone(), auth.clone());
    
    // Did the attach work?
    match attached {
        Ok(is_attached) => {
            if is_attached {
                println!("Payment Method ({}) is now attached to Customer ({})", payment_method.id.clone().unwrap(), cust.id.clone().unwrap());
            
            
                // Subscript the customer to the new_plan.id....
                let mut subscription = payup::stripe::Subscription::new();
                subscription.default_payment_method = payment_method.id.clone();
                subscription.customer = cust.id.clone();
                subscription.default_payment_method = payment_method.id.clone();
                subscription.price_items.push(new_plan.id.clone().unwrap());
                subscription = subscription.post(auth.clone()).unwrap();
                

                println!("subscription: {:?}", subscription);
            }
        },
        Err(err) => println!("{}", err)
    }




}


```
## License

Released under Apache 2.0.

# Support and follow my work by:

#### Buying my dope NTFs:
 * https://opensea.io/accounts/PixelCoda

#### Checking out my Github:
 * https://github.com/PixelCoda

#### Following my facebook page:
 * https://www.facebook.com/pixelcoda/

#### Subscribing to my Patreon:
 * https://www.patreon.com/calebsmith_pixelcoda

#### Or donating crypto:
 * ADA:    addr1vyjsx8zthl5fks8xjsf6fkrqqsxr4f5tprfwux5zsnz862glwmyr3
 * BTC:    3BCj9kYsqyENKU5YgrtHgdQh5iA7zxeJJi
 * MANA:   0x10DFc66F881226f2B91D552e0Cf7231C1e409114
 * SHIB:   0xdE897d5b511A66276E9B91A8040F2592553e6c28


