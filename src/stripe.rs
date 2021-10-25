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
