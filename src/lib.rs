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
        let mut cust = Customer::new();
        cust.email = Some("rust@test.com".to_string());
        match create_customer(cust){
            Ok(response) => println!("YES"),
            Err(err) => println!("NO")
        }
    }
}


// https://na-gateway.mastercard.com/api/documentation/integrationGuidelines/directPayment/accessPaymentGateway.html?locale=en_US


// 20211024051722
// https://na-gateway.mastercard.com/api/rest/version/1/information

// {
//     "status": "OPERATING"
//   }