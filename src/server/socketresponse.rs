use super::MyWs;
use json::object;

pub fn textresponse(msg: String, ctx: &mut actix_web_actors::ws::WebsocketContext<MyWs>) {
    let parsed = json::parse(msg.as_str()).unwrap_or( object!{
        action: "error",
        error: "Expected valid JSON"
    } );

    match parsed["action"].as_str().unwrap_or("no_action") {
        "test" => {
            ctx.text(json::stringify(
                object!{
                    hello: "World"
                }
            )); 
        },
        _ => {
            ctx.text(msg);       
        },
    }
}