use crate::{ adapter::{Request, Response}, dto::JsonResponse };

pub struct StockController {}

impl StockController {
  pub async fn ping(&self, _req: Request) -> Response {
    let message_response: JsonResponse = JsonResponse {
      status: true,
      message: Some(String::from("pong")),
      data: None,
    };
    
    Response::ok(message_response)
  }
}
