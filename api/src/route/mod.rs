use crate::controller::{home};

pub fn home() -> Vec<rocket::Route> {
    routes![
        home::index
    ]
}