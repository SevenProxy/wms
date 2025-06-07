use crate::{ actix_web::Scope, adapter::Request, web::{self, Data}, HttpRequest, StockController };

pub struct RoutesStock;

impl RoutesStock {
  pub fn get(&self, stock_controller: Data<StockController>) -> Scope {
    web::scope("/api/v1")
      .route("/ping", web::get().to(
        move | req: HttpRequest | {
          let request: Request = Request::new(req);
          let controller: Data<StockController> = stock_controller.clone();
          async move {
            controller.ping(request).await
          }
        }
      ))
  }
}
