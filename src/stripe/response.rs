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