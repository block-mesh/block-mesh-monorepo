#![allow(clippy::unused_unit)]

use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    pub mod configuration;
    pub mod database;
    pub mod domain;
    pub mod errors;
    pub mod middlewares;
    pub mod notification;
    pub mod routes;
    pub mod startup;
    pub mod telemetry;
    pub mod utils;
    pub mod worker;
}}
