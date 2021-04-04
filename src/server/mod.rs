use actix_web::{web, App, HttpRequest, Error, HttpResponse, HttpServer, Responder};
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;

mod socketresponse;

/// Define HTTP actor
pub struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            // Automatic Ping Pong
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
            },
            // Use only JSON
            Ok(ws::Message::Text(text)) => {
                socketresponse::textresponse(text, ctx);
            },
            // Ignoring binary websockets by now
            // Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}


async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    //println!("{:?}", resp);
    resp
}

pub async fn run() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/ws/", web::get().to(ws_index))
            .route("/{name}", web::get().to(greet))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}