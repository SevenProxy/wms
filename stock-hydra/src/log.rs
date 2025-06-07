use std::net::IpAddr;
use tracing::{ info, warn };

use crate::error::ServerError;

pub struct Log;

impl Log {
  pub fn start_server() {
    info!("Aplicação iniciada com sucesso 🚀");
  }

  pub fn recive_request(ip_addr: Result<IpAddr, ServerError>) {
    warn!("Requisição recebida de: {:?}", ip_addr);
  }
}
