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

//! ## Description
//! 
//! A synchronous + asynchronous payment library for processing payments with rust + stripe.
//! 
//! I built this library due to a lack of synchronous payment libraries. Currently im only focused on features I need for another project. 
//! 
//! ## Current Stripe Features:
//! 
//! * Balance:
//!     * Ability to fetch the account balance for your stripe account
//! 
//! * BalanceTransaction:
//!     * Ability to fetch an individual BalanceTransaction
//!     * Ability to list all BalanceTransactions
//! 
//! * Card:
//!     * Ability to attach a Card to a PaymentMethod
//! 
//! * Charge:
//!     * Ability to list/post/get/update Charge
//!     * Ability to capture a charge
//! 
//! * Customer:
//!     * Ability to list/post/get/update/destroy Customers
//!     * Ability to attach payment methods to Customers
//!     * Ability to list a customers invoices
//!     * Ability to list a customers payment methods
//! 
//! * Invoice:
//!     * Ability to get an invoice by the id
//!     * Ability to list all invoices
//! 
//! * PaymentMethod:
//!     * Ability to get an PaymentMethod by the id
//!     * Ability to post a new PaymentMethod
//! 
//! * Plan
//!     * Ability to list/post/get/update/destroy Plans
//! 
//! * Price
//!     * Ability to post a new Price
//! 
//! * Subscription
//!     * Ability to post a new Subscription
//!     * Ability to get a Subscription by the id
//!     * Ability to post updates to an existing Subscription
//!     * Ability to cancel a subscription


/// Stripe API Implementation
pub mod stripe;