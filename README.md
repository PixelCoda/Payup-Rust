# Payup

## Description

A synchronous + asynchronous payment library for processing payments with rust + stripe. 

I built this library due to a lack of synchronous payment libraries. Currently im only focused on features I need for another project. 

## Current Stripe Features:

* Balance:
    * Ability to fetch the account balance for your stripe account

* BalanceTransaction:
    * Ability to retrieve a BalanceTransaction
    * Ability to list all BalanceTransactions

* Card:
    * Ability to attach a Card to a PaymentMethod

* Charge:
    * Ability to retrieve a Charge
    * Ability to list all Charges
    * Ability to update an existing Charge
    * Ability to create a new Charge
    * Ability to capture a charge

* Customer:
    * Ability to retrieve a Customer
    * Ability to list all Customers
    * Ability to update an existing Customer
    * Ability to create a new Customer
    * Ability to destroy a Customer
    * Ability to attach payment methods to Customers
    * Ability to list a customers invoices
    * Ability to list a customers payment methods

* Dispute:
    * Ability to retrieve a Dispute
    * Ability to list all Disputes
    * Ability to close a Dispute
    * Ability to update an existing Dispute

* Invoice:
    * Ability to retrieve an Invoice
    * Ability to list all Invoices

* PaymentMethod:
    * Ability to retrieve a PaymentMethod
    * Ability to create a new PaymentMethod

* Plan:
    * Ability to retrieve a Plan
    * Ability to list all Plan
    * Ability to update an existing Plan
    * Ability to create a new Plan

* Price
    * Ability to create a new Price

* Subscription
    * Ability to create a new Subscription
    * Ability to retrieve a Subscription
    * Ability to update an existing Subscription
    * Ability to cancel a subscription

## Roadmap:
* 0.1.0: Quasi-Stripe Support
* 0.2.0: Full Stripe API Support
* 0.3.0: Paypal Support
* 0.4.0: Cryptocurrency Support

## How to use library

Add the following line to your cargo.toml:
```
payup = "0.1.43"
```

Example:
```rust
extern crate payup;

fn main() {

    // Client and Secret for Stripe account
    // In a production environment...load values from environment variables.
    let client = format!("sk_test_");
    let secret = format!("");

    // Create the Authentication refererence
    let auth = payup::stripe::Auth::new(client, secret);

    let get_subscription = payup::stripe::Subscription::get(auth.clone(), "sub_1JpgYvGrEH09RU9ueB31tuQp".to_string());
    match get_subscription {
        Ok(sub) => {
            println!("SUBSCRIPTION_GET: {:?}", sub);
        },
        Err(err) => println!("{}", err),
    }


    let get_all_invoices = payup::stripe::Invoice::list(auth.clone(), None);
    match get_all_invoices {
        Ok(sub) => {
            println!("ALL_INVOICES: {:?}", sub);
        },
        Err(err) => println!("{}", err),
    }

    let get_one_invoice = payup::stripe::Invoice::get(auth.clone(), "in_1KM0TcGrEH09RU9uKzfi8E4x".to_string());
    match get_one_invoice {
        Ok(sub) => {
            println!("SINGLE_INVOICE_GET: {:?}", sub);
        },
        Err(err) => println!("{}", err),
    }


    // Build a customer object
    let mut cust = payup::stripe::Customer::new();
    cust.name = Some("Rust Test".to_string());
    cust.description = Some("A test customer from rust.".to_string());
    cust.phone = Some("333-333-3333".to_string());
    cust.email = Some("rust@test.com".to_string());
    cust.payment_method = None;
    
    // Post customer to stripe and update the local cust variable
    let cust = cust.post(auth.clone()).unwrap();

    let cust_id = cust.id.clone().unwrap();


    let get_cust = payup::stripe::Customer::get(auth.clone(), cust_id.clone());
    match get_cust {
        Ok(sub) => {
            println!("CUST_GET: {:?}", sub.clone());


  



        },
        Err(err) => println!("{}", err),
    }


    // Fetch customers from stripe account
    let customers = payup::stripe::Customer::list(auth.clone()).unwrap();
    // println!("customers: {:?}", customers);

    // Create a new plan
    let mut np = payup::stripe::Plan::new();
    np.amount = Some("200".to_string());
    np.currency = Some("usd".to_string());
    np.interval = Some("month".to_string());
    np.product = Some("prod_KSywTYVmG9jVC4".to_string());
    let new_plan = np.post(auth.clone()).unwrap();

    // Fetch plans from stripe account
    let plans = payup::stripe::Plan::list(auth.clone());
    // println!("plans: {:?}", plans);

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


    let payment_method_id = payment_method.id.clone().unwrap();


    let get_payment_method = payup::stripe::PaymentMethod::get(auth.clone(), payment_method_id.clone());
    match get_payment_method {
        Ok(sub) => {
            println!("PAYMENT_METHOD_GET: {:?}", sub);
        },
        Err(err) => println!("{}", err),
    }


    // Attach the payment method to the customer created earlier
    let attached = payment_method.attach(cust.clone(), auth.clone());
    
    // Did the attach work?
    match attached {
        Ok(is_attached) => {
            println!("{}", is_attached);
            if is_attached {
                println!("Payment Method ({}) is now attached to Customer ({})", payment_method_id.clone(), cust_id.clone());
            
            
                // Subscript the customer to the new_plan.id....
                let mut subscription = payup::stripe::Subscription::new();
                subscription.customer = Some(cust_id.clone());
                subscription.default_payment_method = payment_method.id.clone();
                subscription.price_items.push(format!("price_1Jp6siGrEH09RU9u95Xp7soZ"));
                subscription = subscription.post(auth.clone()).unwrap();
            
                println!("subscription: {:?}", subscription.clone());


                let get_subscription = payup::stripe::Subscription::get(auth.clone(), subscription.clone().id.unwrap());
                match get_subscription {
                    Ok(sub) => {
                        println!("SUBSCRIPTION_GET: {:?}", sub);
                    },
                    Err(err) => println!("{}", err),
                }



                let get_payment_methods = payup::stripe::Customer::payment_methods(auth.clone(), cust_id.clone(), format!("card"));
         
    

                let get_invoices = payup::stripe::Customer::invoices(auth.clone(), cust_id.clone());
                println!("CUSTOMER_INVOICES: {:?}", get_invoices);
           

                // Create a new card
                let mut new_card = payup::stripe::Card::new();
                new_card.number = Some(format!("4242424242424242"));
                new_card.exp_month = Some(format!("01"));
                new_card.exp_year = Some(format!("2023"));
                new_card.cvc = Some(format!("314"));


                // Change Payment Method
                let mut new_payment_method = payup::stripe::PaymentMethod::new();
                new_payment_method.method_type = Some(format!("card"));
                new_payment_method.card = Some(new_card);
                new_payment_method = new_payment_method.post(auth.clone()).unwrap();
                println!("new_payment_method: {:?}", new_payment_method.clone());
            
                let new_payment_method_id = payment_method.id.clone().unwrap();
                

                let mut new_subscription = payup::stripe::Subscription::new();
                new_subscription.default_payment_method = Some(new_payment_method_id);
                new_subscription.id = subscription.clone().id;
                let nnew_subscription = new_subscription.update(auth.clone());
                println!("new_subscription: {:?}", nnew_subscription);


                let subscription_cancel = payup::stripe::Subscription::cancel(auth.clone(), format!("sub_1JpgYvGrEH09RU9ueB31tuQp")).unwrap();
                println!("subscription_cancel: {:?}", subscription_cancel);


            }
        },
        Err(err) => println!("fail: {}", err)
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


