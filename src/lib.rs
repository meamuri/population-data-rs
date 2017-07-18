pub mod app;
pub mod dao;
pub mod dialog;
pub mod computing;

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
