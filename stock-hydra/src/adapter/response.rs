use crate::{ actix_web::{ body, Responder}, adapter::response, dto::JsonResponse, HttpRequest, HttpResponse };


pub struct Response {
  pub response: HttpResponse,
}

impl Response {
  pub fn ok(json: JsonResponse) -> Self {
    Self {
      response: HttpResponse::Ok().json(json),
    }
  }

  pub fn bad_request(msg: String) -> Self {
    Self {
      response: HttpResponse::BadRequest().json(JsonResponse {
        status: false,
        message: Some(msg),
        data: None,
      }),
    }
  }

  pub fn internal_error(msg: String) -> Self {
    Self {
      response: HttpResponse::InternalServerError().json(JsonResponse {
        status: false,
        message: Some(msg),
        data: None
      })
    }
  }

  pub fn into_inner(self) -> HttpResponse {
    self.response
  }
}

impl Responder for Response {
  type Body = body::BoxBody;

  fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
    self.response
  }
}
