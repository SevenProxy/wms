use stock_hydra::{ fmt, web::Data, App, AppState, HttpServer, Log, RoutesStock, StockController };

static PORT: u16 = 3001;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let name_server: Data<AppState> = Data::new(AppState { app_name: String::from("Hydra Stock."), });
  let stock_controller: Data<StockController> = Data::new(StockController {  });
  let routes_stocks: Data<RoutesStock> = Data::new(RoutesStock {});

  fmt()
    .with_max_level(tracing::Level::INFO)
    .with_target(false)
    .pretty()
    .init();

  Log::start_server();

  HttpServer::new(move || {
    App::new()
      .app_data(name_server.clone())
      .app_data(stock_controller.clone())
      .app_data(routes_stocks.clone())
      .service(
        routes_stocks.get(stock_controller.clone())
      )
  })
  .bind(("0.0.0.0", PORT))?
  .run()
  .await
}
