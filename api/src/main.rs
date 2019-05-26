#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

pub mod command;
pub mod controller;
pub mod query;
pub mod route;

fn main() { rocket::ignite().mount("/", route::home()).launch(); }
