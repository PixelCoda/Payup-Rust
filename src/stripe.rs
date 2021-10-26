pub mod response;


use serde_json::json;

use serde::{Serialize, Deserialize};
use std::convert::TryInto;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auth {
    pub client: String,
    pub secret: String,
}
impl Auth {
    pub fn new(client: String, secret: String) -> Self {
        return Auth{client, secret};
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Customer {
    pub id: Option<String>,
    pub payment_method: Option<String>,
    pub description: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>
}
impl Customer {
    pub fn new() -> Self {
        return Customer{id: None, payment_method: None, description: None, email: None, name: None, phone: None};
    }
    pub fn list_all(creds: Auth) -> Vec<Self>{
        let mut objects: Vec<Self> = Vec::new();

        let mut has_more = true;
        let mut starting_after: Option<String> = None;
        while has_more{
            let json = Self::list(creds.clone(), starting_after).unwrap();
            for json_object in json.data{
                let mut object = Customer::new();
                object.id = Some(json_object.id);
                object.email = json_object.email;
                object.description = json_object.description;
                object.name = json_object.name;
                object.phone = json_object.phone;
                objects.push(object);
            }
            has_more = json.has_more;
            starting_after = objects[objects.len() - 1].id.clone();
        }
        return objects;
    }
    pub fn list(creds: Auth, starting_after: Option<String>) -> Result<crate::stripe::response::Customers, reqwest::Error> {
        let mut url = "https://api.stripe.com/v1/customers".to_string();

        if starting_after.is_some() {
            url = format!("https://api.stripe.com/v1/customers?starting_after={}", starting_after.unwrap());
        }

        let request = reqwest::blocking::Client::new().get(url)
        .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
        .send();
        match request{
            Ok(req) => {
                let json = req.json::<crate::stripe::response::Customers>().unwrap();
                return Ok(json);
            },
            Err(err) => Err(err)
        }
    }
    pub fn post(&self, creds: Auth) ->  Result<Customer, reqwest::Error> {
        let request = reqwest::blocking::Client::new().post("https://api.stripe.com/v1/customers")
        .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
        .form(&self.to_params())
        .send();

        match request{
            Ok(req) => {
                let mut cust = self.clone();
                let json = req.json::<crate::stripe::response::Customer>()?;
                cust.id = Some(json.id);



                Ok(cust)
            },
            Err(err) => Err(err)
        }
    }
    pub async fn post_async(&self, creds: Auth) ->  Result<Customer, reqwest::Error> {
        let request = reqwest::Client::new()
        .post("https://api.stripe.com/v1/customers")
        .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
        .form(&self.to_params())
        .send().await;
        match request{
            Ok(req) => {
                let mut cust = self.clone();
                let json = req.json::<crate::stripe::response::Customer>().await?;
                cust.id = Some(json.id);



                Ok(cust)
            },
            Err(err) => Err(err)
        }
    }
    fn to_params(&self) -> Vec<(&str, &str)> {
        // return Customer{client, secret};
        let mut params = vec![];
        match &self.payment_method{
            Some(payment_method) => params.push(("payment_method", payment_method.as_str())),
            None => {}
        }
        match &self.description{
            Some(description) => params.push(("description", description.as_str())),
            None => {}
        }
        match &self.email{
            Some(email) => params.push(("email", email.as_str())),
            None => {}
        }
        match &self.name{
            Some(name) => params.push(("name", name.as_str())),
            None => {}
        }
        match &self.phone{
            Some(phone) => params.push(("phone", phone.as_str())),
            None => {}
        }
        return params;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Plan {
    pub id: Option<String>,
    pub active: Option<String>,
    pub amount: Option<String>,
    pub amount_decimal: Option<String>,
    pub billing_scheme: Option<String>,
    pub created: Option<i64>,
    pub currency: Option<String>,
    pub interval: Option<String>,
    pub interval_count: Option<String>,
    pub product: Option<String>,
}
impl Plan {
    pub fn new() -> Self {
        return Plan{
            id: None, 
            active: None, 
            amount: None, 
            amount_decimal: None, 
            billing_scheme: None, 
            created: None, 
            currency: None, 
            interval: None, 
            interval_count: None,
            product: None};
    }
    pub fn list_all(creds: Auth) -> Vec<Self>{
        let mut objects: Vec<Plan> = Vec::new();

        let mut has_more = true;
        let mut starting_after: Option<String> = None;
        while has_more{
            let json = Self::list(creds.clone(), starting_after).unwrap();
            for json_object in json.data{
                let mut object = Plan::new();
                object.id = Some(json_object.id);
                object.active = Some(json_object.active.to_string());
                object.amount = Some(json_object.amount.to_string());
                object.amount_decimal = Some(json_object.amount_decimal);
                object.billing_scheme = Some(json_object.billing_scheme);
                object.created = Some(json_object.created);
                object.currency = Some(json_object.currency);
                object.interval = Some(json_object.interval);
                object.interval_count = Some(json_object.interval_count.to_string());
                object.product = Some(json_object.product);
                objects.push(object);
            }
            has_more = json.has_more;
            starting_after = objects[objects.len() - 1].id.clone();
        }


        return objects;
    }
    pub fn list(creds: Auth, starting_after: Option<String>) -> Result<crate::stripe::response::Plans, reqwest::Error> {
        let mut url = "https://api.stripe.com/v1/plans".to_string();

        if starting_after.is_some() {
            url = format!("https://api.stripe.com/v1/plans?starting_after={}", starting_after.unwrap());
        }

        let request = reqwest::blocking::Client::new().get(url)
        .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
        .send();
        match request{
            Ok(req) => {
                let json = req.json::<crate::stripe::response::Plans>().unwrap();
                return Ok(json);
            },
            Err(err) => Err(err)
        }
    }
    pub fn post(&self, creds: Auth) ->  Result<Plan, reqwest::Error> {
        let request = reqwest::blocking::Client::new().post("https://api.stripe.com/v1/plans")
        .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
        .form(&self.to_params())
        .send();

        match request{
            Ok(req) => {
                let mut plan = self.clone();
                let json = req.json::<crate::stripe::response::Plan>()?;
                plan.id = Some(json.id);
                Ok(plan)
            },
            Err(err) => Err(err)
        }
    }
    pub async fn post_async(&self, creds: Auth) ->  Result<Plan, reqwest::Error> {
        let request = reqwest::Client::new()
        .post("https://api.stripe.com/v1/plans")
        .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
        .form(&self.to_params())
        .send().await;
        match request{
            Ok(req) => {
                let mut plan = self.clone();
                let json = req.json::<crate::stripe::response::Plan>().await?;
                plan.id = Some(json.id);



                Ok(plan)
            },
            Err(err) => Err(err)
        }
    }
    fn to_params(&self) -> Vec<(&str, &str)> {
        // return Customer{client, secret};
        let mut params = vec![];
        match &self.amount{
            Some(amount) => params.push(("amount", amount.as_str())),
            None => {}
        }
        match &self.currency{
            Some(currency) => params.push(("currency", currency.as_str())),
            None => {}
        }
        match &self.interval{
            Some(interval) => params.push(("interval", interval.as_str())),
            None => {}
        }
        match &self.product{
            Some(product) => params.push(("product", product.as_str())),
            None => {}
        }
        match &self.active{
            Some(active) => params.push(("active", active.as_str())),
            None => {}
        }
        return params;
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    pub id: Option<String>,
    pub brand: Option<String>,
    pub last4: Option<String>,
    pub number: Option<String>,
    pub cvc: Option<String>,
    pub network: Option<String>,
    pub country: Option<String>,
    pub exp_month: Option<String>,
    pub exp_year: Option<String>,
    pub fingerprint: Option<String>,
}
impl Card {
    pub fn new() -> Self {
        return Card{
            id: None, 
            brand: None, 
            last4: None, 
            number: None, 
            cvc: None, 
            network: None, 
            country: None, 
            exp_month: None,
            exp_year: None,
            fingerprint: None
        };
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Address {
    pub city: Option<String>,
    pub country: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub postal_code: Option<String>,
    pub state: Option<String>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BillingDetails {
    pub address: Option<Address>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethod {
    pub id: Option<String>,
    pub method_type: Option<String>,
    pub created: Option<String>,
    pub customer: Option<String>,
    pub livemode:  Option<bool>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub billing_details: Option<BillingDetails>,
    pub card: Option<Card>,
    pub type_field: Option<String>,
}
impl PaymentMethod {
    pub fn new() -> Self {
        return PaymentMethod{
            id: None, 
            method_type: None, 
            created: None, 
            customer: None, 
            livemode: None, 
            name: None,
            phone: None,
            billing_details: None, 
            card: None,
            type_field: None
        };
    }
    pub fn attach(&self, customer: Customer, creds: Auth) ->  Result<bool, reqwest::Error>{

        match &self.id{
            Some(id) => {

                match customer.id {
                    Some(cust_id) => {
                        let url = format!("https://api.stripe.com/v1/payment_methods/{}/attach", id);
                        let params = [
                            ("customer", cust_id.as_str())
                        ];
                        let request = reqwest::blocking::Client::new().post(url)
                        .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
                        .form(&params)
                        .send()?;
                        return Ok(true);
                    },
                    None => {
                        return Ok(false);
                    }
                }
            },
            None => return Ok(false)
        }

        return Ok(false);
    }
    pub fn post(&self, creds: Auth) ->  Result<PaymentMethod, reqwest::Error> {
        let request = reqwest::blocking::Client::new().post("https://api.stripe.com/v1/payment_methods")
        .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
        .form(&self.to_params())
        .send();

        match request{
            Ok(req) => {
                let mut plan = self.clone();
                let json = req.json::<crate::stripe::response::PaymentMethod>()?;
                plan.id = Some(json.id);
                Ok(plan)
            },
            Err(err) => Err(err)
        }
    }
    fn to_params(&self) -> Vec<(&str, &str)> {
        // return Customer{client, secret};
        let mut params = vec![];
        match &self.method_type{
            Some(method_type) => params.push(("type", method_type.as_str())),
            None => {}
        }
        match &self.card{
            Some(card) => {
                match &card.number{
                    Some(number) => params.push(("card[number]", number.as_str())),
                    None => {}
                }
                match &card.exp_month{
                    Some(exp_month) => params.push(("card[exp_month]", exp_month.as_str())),
                    None => {}
                }
                match &card.exp_year{
                    Some(exp_year) => params.push(("card[exp_year]", exp_year.as_str())),
                    None => {}
                }
                match &card.cvc{
                    Some(cvc) => params.push(("card[cvc]", cvc.as_str())),
                    None => {}
                }
            },
            None => {}
        }
        return params;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subscription {
    pub id: Option<String>,
    pub billing_cycle_anchor: Option<i64>,
    pub cancel_at: Option<i64>,
    pub cancel_at_period_end: Option<bool>,
    pub canceled_at: Option<i64>,
    pub collection_method: Option<String>,
    pub created: Option<i64>,
    pub current_period_end: Option<i64>,
    pub current_period_start: Option<i64>,
    pub customer: Option<String>,
    pub days_until_due: Option<i64>,
    pub default_payment_method: Option<String>,
    pub ended_at: Option<i64>,
    pub latest_invoice: Option<String>,
    pub livemode: Option<bool>,
    pub quantity: Option<i64>,
    pub start_date: Option<i64>,
    pub status: Option<String>,
    pub price_items: Vec<String>
}
impl Subscription {
    pub fn new() -> Self {
        let price_items: Vec<String> = Vec::new();
        return Subscription{
            id: None, 
            billing_cycle_anchor: None, 
            cancel_at: None, 
            cancel_at_period_end: None, 
            canceled_at: None, 
            collection_method: None,
            created: None,
            current_period_end: None, 
            current_period_start: None,
            customer: None,
            price_items: price_items,
            days_until_due: None,
            default_payment_method: None,
            ended_at: None,
            latest_invoice: None,
            livemode: None,
            quantity: None,
            start_date: None,
            status: None
        };
    }
    pub fn post(&self, creds: Auth) ->  Result<Subscription, reqwest::Error> {
        let request = reqwest::blocking::Client::new().post("https://api.stripe.com/v1/subscriptions")
        .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
        .form(&self.to_params())
        .send();

        match request{
            Ok(req) => {
                let mut plan = self.clone();
                let json = req.json::<crate::stripe::response::Subscription>()?;
                plan.id = json.id;
                Ok(plan)
            },
            Err(err) => Err(err)
        }
    }
    fn to_params(&self) -> Vec<(&str, &str)> {
        // return Customer{client, secret};
        let mut params = vec![];
        match &self.customer{
            Some(customer) => params.push(("customer", customer.as_str())),
            None => {}
        }

        match &self.default_payment_method{
            Some(default_payment_method) => params.push(("default_payment_method", default_payment_method.as_str())),
            None => {}
        }

        let mut ii = 0;
        for (item) in &self.price_items{
            if ii < 20{
                if ii == 0{
                    params.push(("items[0][price]", item.as_str()));
                }
                ii+=1;
            }
        }
     
        return params;
    }

 
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Price {
    pub id: Option<String>,
    pub active: Option<bool>,
    pub billing_scheme: Option<String>,
    pub created: Option<i64>,
    pub currency: Option<String>,
    pub livemode: Option<bool>,
    pub product: Option<String>,
    pub tax_behavior: Option<String>,
    pub type_field: Option<String>,
    pub unit_amount: Option<String>,
    pub unit_amount_decimal: Option<String>,
}
impl Price {
    pub fn new() -> Self {
        return Price{
            id: None,
            active: None,
            billing_scheme: None,
            created: None,
            currency: None,
            livemode: None,
            product: None,
            tax_behavior: None,
            type_field: None,
            unit_amount: None,
            unit_amount_decimal: None
        };
    }
    pub fn post(&self, creds: Auth) ->  Result<Price, reqwest::Error> {
        let request = reqwest::blocking::Client::new().post("https://api.stripe.com/v1/prices")
        .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
        .form(&self.to_params())
        .send();

        match request{
            Ok(req) => {
                let mut plan = self.clone();
                let json = req.json::<crate::stripe::response::Price>()?;
                plan.id = Some(json.id);
                Ok(plan)
            },
            Err(err) => Err(err)
        }
    }
    fn to_params(&self) -> Vec<(&str, &str)> {
        // return Customer{client, secret};
        let mut params = vec![];
        match &self.currency{
            Some(currency) => params.push(("currency", currency.as_str())),
            None => {}
        }
        match &self.unit_amount{
            Some(unit_amount) => params.push(("unit_amount", unit_amount.as_str())),
            None => {}
        }

        // TODO Impliment product

        return params;
    }

 
}