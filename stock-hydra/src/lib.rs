mod error;
mod log;
mod dto;

mod adapter;
mod controllers;
mod routes;

mod infrastructure;
mod domain;

pub use actix_web;
pub use actix_web::{ HttpServer, App, web, HttpRequest, HttpResponse };
pub use tracing_subscriber::fmt;

pub use log::Log;

pub use controllers::StockController;
pub use routes::RoutesStock;

pub struct AppState {
  pub app_name: String,
}
