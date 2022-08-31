//! Diesel support for PostGIS geography types and functions.

#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;

pub mod sql_types;
pub mod types;
