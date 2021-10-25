use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Customers {
    pub object: String,
    pub url: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    pub data: Vec<Customer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    pub id: String,
    pub object: String,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub address: Value,
    // pub balance: i64,
    // pub created: i64,
    // pub currency: String,
    // #[serde(rename = "default_source")]
    // pub default_source: Value,
    // pub delinquent: bool,
    pub description: Option<String>,
    // pub discount: Value,
    pub email: Option<String>,
    // #[serde(rename = "invoice_prefix")]
    // pub invoice_prefix: String,
    // #[serde(rename = "invoice_settings")]
    // pub invoice_settings: InvoiceSettings,
    // pub livemode: bool,
    // pub metadata: Metadata,
    pub name: Option<String>,
    // #[serde(rename = "next_invoice_sequence")]
    // pub next_invoice_sequence: i64,
    pub phone: Option<String>,
    // #[serde(rename = "preferred_locales")]
    // pub preferred_locales: Vec<Value>,
    // pub shipping: Value,
    // #[serde(rename = "tax_exempt")]
    // pub tax_exempt: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceSettings {
    #[serde(rename = "custom_fields")]
    pub custom_fields: Value,
    #[serde(rename = "default_payment_method")]
    pub default_payment_method: Value,
    pub footer: Value,
}



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plans {
    pub object: String,
    pub url: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    pub data: Vec<Plan>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plan {
    pub id: String,
    pub object: String,
    pub active: bool,
    // #[serde(rename = "aggregate_usage")]
    // pub aggregate_usage: Value,
    pub amount: i64,
    #[serde(rename = "amount_decimal")]
    pub amount_decimal: String,
    #[serde(rename = "billing_scheme")]
    pub billing_scheme: String,
    pub created: i64,
    pub currency: String,
    pub interval: String,
    #[serde(rename = "interval_count")]
    pub interval_count: i64,
    pub livemode: bool,
    pub metadata: Metadata,
    // pub nickname: Value,
    pub product: String,
    // #[serde(rename = "tiers_mode")]
    // pub tiers_mode: Value,
    // #[serde(rename = "transform_usage")]
    // pub transform_usage: Value,
    // #[serde(rename = "trial_period_days")]
    // pub trial_period_days: Value,
    #[serde(rename = "usage_type")]
    pub usage_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Charges {
    pub object: String,
    pub url: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    pub data: Vec<Charge>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Charge {
    pub id: String,
    pub object: String,
    pub amount: i64,
    #[serde(rename = "amount_captured")]
    pub amount_captured: i64,
    #[serde(rename = "amount_refunded")]
    pub amount_refunded: i64,
    #[serde(rename = "balance_transaction")]
    pub balance_transaction: String,
    #[serde(rename = "billing_details")]
    pub billing_details: BillingDetails,
    pub captured: bool,
    pub created: i64,
    pub currency: String,
    pub description: String,
    pub disputed: bool,
    #[serde(rename = "fraud_details")]
    pub fraud_details: FraudDetails,
    pub livemode: bool,
    pub metadata: Metadata,
    pub paid: bool,
    #[serde(rename = "payment_method")]
    pub payment_method: String,
    #[serde(rename = "payment_method_details")]
    pub payment_method_details: PaymentMethodDetails,
    #[serde(rename = "receipt_url")]
    pub receipt_url: String,
    pub refunded: bool,
    pub refunds: Refunds,
    pub status: String,
    // #[serde(rename = "calculated_statement_descriptor")]
    // pub calculated_statement_descriptor: Value,
    // pub customer: Value,
    // pub invoice: Value,
    // #[serde(rename = "failure_code")]
    // pub failure_code: Value,
    // #[serde(rename = "failure_message")]
    // pub failure_message: Value,
    // #[serde(rename = "on_behalf_of")]
    // pub on_behalf_of: Value,
    // pub order: Value,
    // pub outcome: Value,
    // #[serde(rename = "payment_intent")]
    // pub payment_intent: Value,
    // #[serde(rename = "receipt_email")]
    // pub receipt_email: Value,
    // #[serde(rename = "receipt_number")]
    // pub receipt_number: Value,
    // pub review: Value,
    // pub shipping: Value,
    // #[serde(rename = "source_transfer")]
    // pub source_transfer: Value,
    // #[serde(rename = "statement_descriptor")]
    // pub statement_descriptor: Value,
    // #[serde(rename = "statement_descriptor_suffix")]
    // pub statement_descriptor_suffix: Value,
    // #[serde(rename = "transfer_data")]
    // pub transfer_data: Value,
    // #[serde(rename = "transfer_group")]
    // pub transfer_group: Value,
    // pub application: Value,
    // #[serde(rename = "application_fee")]
    // pub application_fee: Value,
    // #[serde(rename = "application_fee_amount")]
    // pub application_fee_amount: Value,
}











#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BillingDetails {
    pub address: Address,
    pub email: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub city: Option<String>,
    pub country: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    #[serde(rename = "postal_code")]
    pub postal_code: Option<String>,
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FraudDetails {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethodDetails {
    pub card: Card,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub brand: String,
    pub checks: Checks,
    pub country: String,
    #[serde(rename = "exp_month")]
    pub exp_month: i64,
    #[serde(rename = "exp_year")]
    pub exp_year: i64,
    pub fingerprint: String,
    pub funding: String,
    // pub installments: Value,
    pub last4: String,
    pub network: String,
    // #[serde(rename = "three_d_secure")]
    // pub three_d_secure: Value,
    // pub wallet: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Checks {
    // #[serde(rename = "address_line1_check")]
    // pub address_line1_check: Value,
    // #[serde(rename = "address_postal_code_check")]
    // pub address_postal_code_check: Value,
    #[serde(rename = "cvc_check")]
    pub cvc_check: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Refunds {
    pub object: String,
    // pub data: Vec<Value>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    pub url: String,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethod {
    pub id: String,
    pub object: String,
    #[serde(rename = "billing_details")]
    pub billing_details: BillingDetails,
    pub card: Card,
    pub created: i64,
    // pub customer: Value,
    pub livemode: bool,
    pub metadata: Metadata,
    #[serde(rename = "type")]
    pub type_field: String,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Networks {
    pub available: Vec<String>,
    // pub preferred: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreeDSecureUsage {
    pub supported: bool,
}


