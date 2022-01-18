// Copyright 2021 Caleb Mitchell Smith-Woolrich (PixelCoda)
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod stripe;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {

    // Client and Secret for Stripe account
    // In a production environment...load values from environment variables.
    let client = format!("sk_test_51Jo2sKGrEH09RU9uu8d8ARKasYUKHXAHk4vUNup1JLgP5wFnQQf6t7UpKfh7woVMhI9oeuziolW2dK1uwmgAheVI00bN8ews6g");
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

    // Build a customer object
    let mut cust = payup::stripe::Customer::new();
    cust.name = Some("Rust Test".to_string());
    cust.description = Some("A test customer from rust.".to_string());
    cust.phone = Some("333-333-3333".to_string());
    cust.email = Some("rust@test.com".to_string());
    cust.payment_method = None;
    
    // Post customer to stripe and update the local cust variable
    // cust = cust.post(auth.clone()).unwrap();

    // Fetch customers from stripe account
    let customers = payup::stripe::Customer::list_all(auth.clone());
    // println!("customers: {:?}", customers);

    // Create a new plan
    let mut new_plan = payup::stripe::Plan::new();
    new_plan.amount = Some("200".to_string());
    new_plan.currency = Some("usd".to_string());
    new_plan.interval = Some("month".to_string());
    new_plan.product = Some("prod_KSywTYVmG9jVC4".to_string());
    // new_plan = new_plan.post(auth.clone()).unwrap();

    // Fetch plans from stripe account
    let plans = payup::stripe::Plan::list_all(auth.clone());
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


                let subscription_cancel = subscription.cancel(auth.clone()).unwrap();

                println!("subscription_cancel: {:?}", subscription_cancel);


            }
        },
        Err(err) => println!("fail: {}", err)
    }


    }
}


// https://na-gateway.mastercard.com/api/documentation/integrationGuidelines/directPayment/accessPaymentGateway.html?locale=en_US


// 20211024051722
// https://na-gateway.mastercard.com/api/rest/version/1/information

// {
//     "status": "OPERATING"
//   }