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
    pub fn async_new(client: String, secret: String) -> Self {
        return Auth{client, secret};
    }
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


    /// Asynchronously lookup a stripe Customer using customer_id
    /// 
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `id` - A string representing an existing stripe customer_id
    ///
    /// # Examples
    ///
    /// ```
    /// // Client and Secret for Stripe account
    /// // In a production environment you should load values from environment variables.
    /// let client = format!("sk_test_");
    /// let secret = format!("");
    ///
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new(client, secret);
    ///
    /// // Fetch customer using id
    /// let customer = payup::stripe::Customer::get_async(auth, "cust_").await;
    /// ```
    pub async fn async_get(creds: Auth, id: String) -> Result<crate::stripe::response::Customer, reqwest::Error> {
        let mut url = format!("https://api.stripe.com/v1/customers/{}", id.clone());
        let request = reqwest::Client::new().get(url).basic_auth(creds.client.as_str(), Some(creds.secret.as_str())).send().await?;
        let json = request.json::<crate::stripe::response::Customer>().await?;
        return Ok(json);
    }

    /// Asynchronously returns all Invoices belonging to the customer_id
    /// # Examples
    ///
    /// ```
    /// // Client and Secret for Stripe account
    /// // In a production environment you should load values from environment variables.
    /// let client = format!("sk_test_");
    /// let secret = format!("");
    ///
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new(client, secret);
    ///
    /// let customers_invoices = payup::stripe::Customer::invoices(auth, format!("cust_"));     
    /// ```    
    pub async fn async_invoices(creds: Auth, customer_id: String) -> Result<Vec<crate::stripe::response::Invoice>, reqwest::Error>{
        let mut objects: Vec<crate::stripe::response::Invoice> = Vec::new();

        let mut has_more = true;
        let mut starting_after: Option<String> = None;
        while has_more{
            let json = Self::get_invoices_chunk(creds.clone(), customer_id.clone(), starting_after.clone())?;
            for json_object in json.data{
                objects.push(json_object);
            }
            has_more = json.has_more;
            starting_after = Some(objects[objects.len() - 1].id.clone());
        }
        return Ok(objects);
    }


    /// Asynchronously returns all stripe customers owned by the account.
    ///
    /// # Examples
    ///
    /// ```
    /// // Client and Secret for Stripe account
    /// // In a production environment you should load values from environment variables.
    /// let client = format!("sk_test_");
    /// let secret = format!("");
    ///
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new(client, secret);
    ///
    /// // Fetch all customers from stripe
    /// let customers = payup::stripe::Customer::list_async(auth).await;
    /// ```
    pub async fn async_list(creds: Auth) -> Result<Vec<crate::stripe::response::Customer>, reqwest::Error>{
        let mut objects: Vec<crate::stripe::response::Customer> = Vec::new();

        let mut has_more = true;
        let mut starting_after: Option<String> = None;
        while has_more{
            let json = Self::list_chunk_async(creds.clone(), starting_after).await?;
            for json_object in json.data{
                objects.push(json_object);
            }
            has_more = json.has_more;
            starting_after = Some(objects[objects.len() - 1].id.clone());
        }
        return Ok(objects);
    }

    /// Asynchronously returns an empty Customer object 
    ///
    /// # Examples
    ///
    /// ```
    /// let mut cust = payup::stripe::Customer::new();
    /// cust.name = Some("Rust Test".to_string());
    /// cust.description = Some("A test customer from rust.".to_string());
    /// cust.phone = Some("333-333-3333".to_string());
    /// cust.email = Some("rust@test.com".to_string());
    /// cust.payment_method = None;
    /// ```
    pub async fn async_new() -> Self {
        return Customer{id: None, payment_method: None, description: None, email: None, name: None, phone: None};
    }


    /// Asynchronously returns all PaymentMethods belonging to the customer_id
    /// # Examples
    ///
    /// ```
    /// // Client and Secret for Stripe account
    /// // In a production environment you should load values from environment variables.
    /// let client = format!("sk_test_");
    /// let secret = format!("");
    ///
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new(client, secret);
    ///
    /// let customers_payment_methods = payup::stripe::Customer::payment_methods_async(auth, format!("cust_"), format!("card")).await;     
    /// ```
    pub async fn async_payment_methods(creds: Auth, customer_id: String, method_type: String) -> Result<Vec<crate::stripe::response::PaymentMethod>, reqwest::Error>{
        let mut objects: Vec<crate::stripe::response::PaymentMethod> = Vec::new();

        let mut has_more = true;
        let mut starting_after: Option<String> = None;
        while has_more{
            let json = Self::get_payment_methods_chunk_async(creds.clone(), customer_id.clone(), method_type.clone(), starting_after.clone()).await?;
            for json_object in json.data{
                objects.push(json_object);
            }
            has_more = json.has_more;
            starting_after = Some(objects[objects.len() - 1].id.clone());
        }
        return Ok(objects);
    }


    /// Asynchronously POSTs a new customer to the stripe api
    /// # Examples
    ///
    /// ```
    /// // Client and Secret for Stripe account
    /// // In a production environment you should load values from environment variables.
    /// let client = format!("sk_test_");
    /// let secret = format!("");
    ///
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new(client, secret);
    ///
    /// // Build a customer object
    /// let mut cust = payup::stripe::Customer::new();
    /// cust.name = Some("Rust Test".to_string());
    /// cust.description = Some("A test customer from rust.".to_string());
    /// cust.phone = Some("333-333-3333".to_string());
    /// cust.email = Some("rust@test.com".to_string());
    /// cust.payment_method = None;
    /// 
    /// // Post customer to stripe and update the local cust variable
    /// let customer = cust.post_async(auth).await.unwrap();
    /// ```
    pub async fn async_post(&self, creds: Auth) ->  Result<crate::stripe::response::Customer, reqwest::Error> {
        let request = reqwest::Client::new()
        .post("https://api.stripe.com/v1/customers")
        .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
        .form(&self.to_params())
        .send().await;
        match request{
            Ok(req) => {
                let json = req.json::<crate::stripe::response::Customer>().await?;
            
                return Ok(json);
            },
            Err(err) => Err(err)
        }
    }


 
    /// Lookup a stripe Customer using customer_id
    /// 
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `id` - A string representing an existing stripe customer_id
    ///
    /// # Examples
    ///
    /// ```
    /// // Client and Secret for Stripe account
    /// // In a production environment you should load values from environment variables.
    /// let client = format!("sk_test_");
    /// let secret = format!("");
    ///
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new(client, secret);
    ///
    /// // Fetch customer using id
    /// let customer = payup::stripe::Customer::get(auth, "cust_");
    /// ```
    pub fn get(auth: Auth, id: String) -> Result<crate::stripe::response::Customer, reqwest::Error> {
        let mut url = format!("https://api.stripe.com/v1/customers/{}", id.clone());
        let request = reqwest::blocking::Client::new().get(url).basic_auth(auth.client.as_str(), Some(auth.secret.as_str())).send()?;
        let json = request.json::<crate::stripe::response::Customer>()?;
        return Ok(json);
    }

    /// Returns all Invoices belonging to the customer_id
    /// # Examples
    ///
    /// ```
    /// // Client and Secret for Stripe account
    /// // In a production environment you should load values from environment variables.
    /// let client = format!("sk_test_");
    /// let secret = format!("");
    ///
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new(client, secret);
    ///
    /// let customers_invoices = payup::stripe::Customer::invoices(auth, format!("cust_"));     
    /// ```    
    pub fn invoices(creds: Auth, customer_id: String) -> Result<Vec<crate::stripe::response::Invoice>, reqwest::Error>{
        let mut objects: Vec<crate::stripe::response::Invoice> = Vec::new();

        let mut has_more = true;
        let mut starting_after: Option<String> = None;
        while has_more{
            let json = Self::get_invoices_chunk(creds.clone(), customer_id.clone(), starting_after.clone())?;
            for json_object in json.data{
                objects.push(json_object);
            }
            has_more = json.has_more;
            starting_after = Some(objects[objects.len() - 1].id.clone());
        }
        return Ok(objects);
    }


    /// Returns all stripe customers
    /// 
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    ///
    /// # Examples
    ///
    /// ```
    /// // Client and Secret for Stripe account
    /// // In a production environment you should load values from environment variables.
    /// let client = format!("sk_test_");
    /// let secret = format!("");
    ///
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new(client, secret);
    ///
    /// // Fetch all customers from stripe
    /// let customers = payup::stripe::Customer::list(auth.clone());
    /// ```
    pub fn list(creds: Auth) -> Result<Vec<crate::stripe::response::Customer>, reqwest::Error>{
        let mut objects: Vec<crate::stripe::response::Customer> = Vec::new();

        let mut has_more = true;
        let mut starting_after: Option<String> = None;
        while has_more{
            let json = Self::list_chunk(creds.clone(), starting_after)?;
            for json_object in json.data{
                objects.push(json_object);
            }
            has_more = json.has_more;
            starting_after = Some(objects[objects.len() - 1].id.clone());
        }
        return Ok(objects);
    }




    /// Returns an empty Customer object
    ///
    /// # Examples
    ///
    /// ```
    /// let mut cust = payup::stripe::Customer::new();
    /// cust.name = Some("Rust Test".to_string());
    /// cust.description = Some("A test customer from rust.".to_string());
    /// cust.phone = Some("333-333-3333".to_string());
    /// cust.email = Some("rust@test.com".to_string());
    /// cust.payment_method = None;
    /// ```
    pub fn new() -> Self {
        return Customer{id: None, payment_method: None, description: None, email: None, name: None, phone: None};
    }

 

   
    /// Returns all PaymentMethods belonging to the customer_id
    /// # Examples
    ///
    /// ```
    /// // Client and Secret for Stripe account
    /// // In a production environment you should load values from environment variables.
    /// let client = format!("sk_test_");
    /// let secret = format!("");
    ///
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new(client, secret);
    ///
    /// let customers_payment_methods = payup::stripe::Customer::payment_methods(auth, format!("cust_"), format!("card"));     
    /// ```
    pub fn payment_methods(creds: Auth, customer_id: String, method_type: String) -> Result<Vec<crate::stripe::response::PaymentMethod>, reqwest::Error>{
        let mut objects: Vec<crate::stripe::response::PaymentMethod> = Vec::new();

        let mut has_more = true;
        let mut starting_after: Option<String> = None;
        while has_more{
            let json = Self::get_payment_methods_chunk(creds.clone(), customer_id.clone(), method_type.clone(), starting_after.clone())?;
            for json_object in json.data{
                objects.push(json_object);
            }
            has_more = json.has_more;
            starting_after = Some(objects[objects.len() - 1].id.clone());
        }
        return Ok(objects);
    }




 

    /// POSTs a new customer to the stripe api
    /// # Examples
    ///
    /// ```
    /// // Client and Secret for Stripe account
    /// // In a production environment you should load values from environment variables.
    /// let client = format!("sk_test_");
    /// let secret = format!("");
    ///
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new(client, secret);
    ///
    /// // Build a customer object
    /// let mut cust = payup::stripe::Customer::new();
    /// cust.name = Some("Rust Test".to_string());
    /// cust.description = Some("A test customer from rust.".to_string());
    /// cust.phone = Some("333-333-3333".to_string());
    /// cust.email = Some("rust@test.com".to_string());
    /// cust.payment_method = None;
    /// 
    /// // Post customer to stripe and update the local cust variable
    /// let customer = cust.post(auth).unwrap();
    /// ```
    pub fn post(&self, creds: Auth) ->  Result<crate::stripe::response::Customer, reqwest::Error> {
        let request = reqwest::blocking::Client::new().post("https://api.stripe.com/v1/customers")
        .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
        .form(&self.to_params())
        .send();

        match request{
            Ok(req) => {
                let json = req.json::<crate::stripe::response::Customer>()?;
                Ok(json)
            },
            Err(err) => Err(err)
        }
    }

 

   
   


    fn get_invoices_chunk(creds: Auth, customer_id: String, starting_after: Option<String>) ->  Result<crate::stripe::response::Invoices, reqwest::Error>{  
        let mut url = format!("https://api.stripe.com/v1/invoices?customer={}", customer_id);

        if starting_after.is_some() {
            url = format!("https://api.stripe.com/v1/invoices?customer={}&starting_after={}", customer_id, starting_after.unwrap())
        }

        let request = reqwest::blocking::Client::new().get(url).basic_auth(creds.client.as_str(), Some(creds.secret.as_str())).send()?;
  
        let json = request.json::<crate::stripe::response::Invoices>()?;
        return Ok(json);
    }

    fn list_chunk(creds: Auth, starting_after: Option<String>) -> Result<crate::stripe::response::Customers, reqwest::Error> {
        let mut url = "https://api.stripe.com/v1/customers".to_string();

        if starting_after.is_some() {
            url = format!("https://api.stripe.com/v1/customers?starting_after={}", starting_after.unwrap());
        }

        let request = reqwest::blocking::Client::new().get(url).basic_auth(creds.client.as_str(), Some(creds.secret.as_str())).send()?;

        let json = request.json::<crate::stripe::response::Customers>()?;
        return Ok(json);
    }

    async fn list_chunk_async(creds: Auth, starting_after: Option<String>) -> Result<crate::stripe::response::Customers, reqwest::Error> {
        let mut url = "https://api.stripe.com/v1/customers".to_string();

        if starting_after.is_some() {
            url = format!("https://api.stripe.com/v1/customers?starting_after={}", starting_after.unwrap());
        }

        let request = reqwest::Client::new().get(url).basic_auth(creds.client.as_str(), Some(creds.secret.as_str())).send().await?;
 
        let json = request.json::<crate::stripe::response::Customers>().await?;
        return Ok(json);
    }

    fn get_payment_methods_chunk(creds: Auth, customer_id: String, method_type: String, starting_after: Option<String>) ->  Result<crate::stripe::response::PaymentMethods, reqwest::Error>{
        let mut url = format!("https://api.stripe.com/v1/customers/{}/payment_methods?type={}", customer_id, method_type);

        if starting_after.is_some() {
            url = format!("https://api.stripe.com/v1/customers/{}/payment_methods?type={}&starting_after={}", customer_id, method_type, starting_after.unwrap());
        }

        let request = reqwest::blocking::Client::new().get(url).basic_auth(creds.client.as_str(), Some(creds.secret.as_str())).send()?;
   
        let json = request.json::<crate::stripe::response::PaymentMethods>()?;
        return Ok(json);
    }

    async fn get_payment_methods_chunk_async(creds: Auth, customer_id: String, method_type: String, starting_after: Option<String>) ->  Result<crate::stripe::response::PaymentMethods, reqwest::Error>{
        let mut url = format!("https://api.stripe.com/v1/customers/{}/payment_methods?type={}", customer_id, method_type);

        if starting_after.is_some() {
            url = format!("https://api.stripe.com/v1/customers/{}/payment_methods?type={}&starting_after={}", customer_id, method_type, starting_after.unwrap());
        }

        let request = reqwest::Client::new().get(url).basic_auth(creds.client.as_str(), Some(creds.secret.as_str())).send().await?;
     
        let json = request.json::<crate::stripe::response::PaymentMethods>().await?;
        return Ok(json);
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
pub struct Invoice {
    pub id: Option<String>,
    pub customer: Option<String>,
    pub auto_advance:  Option<String>
}
impl Invoice {
    // pub fn new() -> Self {
    //     return Plan{
    //         id: None, 
    //         customer: None,
    //         auto_advance: None
    //     };
    // }

    // Status draft, open, paid, uncollectible, or void
    pub fn list(creds: Auth, status: Option<String>) -> Result<crate::stripe::response::Invoices, reqwest::Error> {
        let mut url = "https://api.stripe.com/v1/invoices".to_string();

        if status.is_some() {
            url = format!("https://api.stripe.com/v1/invoices?status={}", status.unwrap());
        }

        let request = reqwest::blocking::Client::new().get(url)
        .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
        .send();
        match request{
            Ok(req) => {
                let json = req.json::<crate::stripe::response::Invoices>().unwrap();
                return Ok(json);
            },
            Err(err) => Err(err)
        }
    }
    pub fn get(creds: Auth, id: String) -> Result<crate::stripe::response::Invoice, reqwest::Error> {
        let mut url = format!("https://api.stripe.com/v1/invoices/{}", id.clone());
        
        let request = reqwest::blocking::Client::new().get(url)
        .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
        .send();
        match request{
            Ok(req) => {
                let json = req.json::<crate::stripe::response::Invoice>().unwrap();
                return Ok(json);
            },
            Err(err) => Err(err)
        }
    }
    // pub fn post(&self, creds: Auth) ->  Result<Invoice, reqwest::Error> {
    //     let request = reqwest::blocking::Client::new().post("https://api.stripe.com/v1/invoices")
    //     .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
    //     .form(&self.to_params())
    //     .send();

    //     match request{
    //         Ok(req) => {
    //             let mut plan = self.clone();
    //             let json = req.json::<crate::stripe::response::Invoice>()?;
    //             plan.id = Some(json.id);
    //             Ok(plan)
    //         },
    //         Err(err) => Err(err)
    //     }
    // }
    // pub async fn post_async(&self, creds: Auth) ->  Result<Invoice, reqwest::Error> {
    //     let request = reqwest::Client::new()
    //     .post("https://api.stripe.com/v1/invoices")
    //     .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
    //     .form(&self.to_params())
    //     .send().await;
    //     match request{
    //         Ok(req) => {
    //             let mut invoice = self.clone();
    //             let json = req.json::<crate::stripe::response::Invoice>().await?;
    //             invoice.id = Some(json.id);
    //             Ok(invoice)
    //         },
    //         Err(err) => Err(err)
    //     }
    // }
    // fn to_params(&self) -> Vec<(&str, &str)> {
    //     // return Customer{client, secret};
    //     let mut params = vec![];
    //     match &self.amount{
    //         Some(amount) => params.push(("amount", amount.as_str())),
    //         None => {}
    //     }
    //     match &self.currency{
    //         Some(currency) => params.push(("currency", currency.as_str())),
    //         None => {}
    //     }
    //     match &self.interval{
    //         Some(interval) => params.push(("interval", interval.as_str())),
    //         None => {}
    //     }
    //     match &self.product{
    //         Some(product) => params.push(("product", product.as_str())),
    //         None => {}
    //     }
    //     match &self.active{
    //         Some(active) => params.push(("active", active.as_str())),
    //         None => {}
    //     }
    //     return params;
    // }
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
pub struct PaymentMethod {
    pub id: Option<String>,
    pub method_type: Option<String>,
    pub created: Option<String>,
    pub customer: Option<String>,
    pub livemode:  Option<bool>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub billing_details: Option<crate::stripe::response::BillingDetails>,
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
    pub fn attach(&self, customer: crate::stripe::response::Customer, creds: Auth) ->  Result<bool, reqwest::Error>{

        match &self.id{
            Some(id) => {
               
                let url = format!("https://api.stripe.com/v1/payment_methods/{}/attach", id);
                let params = [
                    ("customer", customer.id.as_str())
                ];
                let request = reqwest::blocking::Client::new().post(url)
                .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
                .form(&params)
                .send()?;
                return Ok(true);
            
                
            },
            None => return Ok(false)
        }

        return Ok(false);
    }
    pub fn get(creds: Auth, id: String) -> Result<crate::stripe::response::PaymentMethod, reqwest::Error> {
        let mut url = format!("https://api.stripe.com/v1/payment_methods/{}", id.clone());
        
        let request = reqwest::blocking::Client::new().get(url)
        .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
        .send();
        match request{
            Ok(req) => {
                let json = req.json::<crate::stripe::response::PaymentMethod>().unwrap();
                return Ok(json);
            },
            Err(err) => Err(err)
        }
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
    pub fn cancel(creds: Auth, id: String) -> Result<crate::stripe::response::Subscription, reqwest::Error> {
        let mut url = format!("https://api.stripe.com/v1/subscriptions/{}", id.clone());
        
        let request = reqwest::blocking::Client::new().delete(url)
        .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
        .send();
        match request{
            Ok(req) => {
                let json = req.json::<crate::stripe::response::Subscription>().unwrap();
                return Ok(json);
            },
            Err(err) => Err(err)
        }
    }

    /// Returns a subscription
    ///
    /// # Arguments
    ///
    /// * `creds` - A struct containing the client + secret for authentication over the stripe API.
    /// * `id` - A string representing an existing stripe subscription_id
    ///
    /// # Examples
    ///
    /// ```
    /// // Client and Secret for Stripe account
    /// // In a production environment...load values from environment variables.
    /// let client = format!("sk_test_51Jo2sKGrEH09RU9uu8d8ARKasYUKHXAHk4vUNup1JLgP5wFnQQf6t7UpKfh7woVMhI9oeuziolW2dK1uwmgAheVI00bN8ews6g");
    /// let secret = format!("");
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new(client, secret);
    /// 
    /// let get_subscription = payup::stripe::Subscription::get(auth, "subscription_id");
    /// ```
    pub fn get(creds: Auth, id: String) -> Result<crate::stripe::response::Subscription, reqwest::Error> {
        let mut url = format!("https://api.stripe.com/v1/subscriptions/{}", id.clone());
        
        let request = reqwest::blocking::Client::new().get(url)
        .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
        .send();
        match request{
            Ok(req) => {
                let json = req.json::<crate::stripe::response::Subscription>().unwrap();
                return Ok(json);
            },
            Err(err) => Err(err)
        }
    }
    pub fn update(&self, creds: Auth) ->  Result<crate::stripe::response::Subscription, reqwest::Error> {
        let request = reqwest::blocking::Client::new().post(format!("https://api.stripe.com/v1/subscriptions/{}", self.clone().id.unwrap()))
        .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
        .form(&self.to_params())
        .send();

        match request{
            Ok(req) => {
                let json = req.json::<crate::stripe::response::Subscription>()?;
                Ok(json)
            },
            Err(err) => Err(err)
        }
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