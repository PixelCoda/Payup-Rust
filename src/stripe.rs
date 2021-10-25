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