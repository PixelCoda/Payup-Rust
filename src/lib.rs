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
        let auth = stripe::Auth::new(client, secret);
        
        // Build a customer object
        let mut cust = stripe::Customer::new();
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
}


// https://na-gateway.mastercard.com/api/documentation/integrationGuidelines/directPayment/accessPaymentGateway.html?locale=en_US


// 20211024051722
// https://na-gateway.mastercard.com/api/rest/version/1/information

// {
//     "status": "OPERATING"
//   }