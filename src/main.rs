use std::str::FromStr;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
//use tokio::{sync::Mutex, fs::File};
//use std::error::Error;
//use csv_async::AsyncWriter
use serde_with::TimestampMilliSeconds;
use serde_with::formats::Flexible;

mod entities;
use entities::request::{Entity as RequestTable, ActiveModel};
use sea_orm::{self, ActiveModelTrait, ActiveValue::NotSet, EntityTrait, Set};
use sea_orm::prelude::DateTime;
mod settings;
#[derive(Debug, Serialize, Deserialize)]
enum DocumentLifecycle {
    #[serde(rename = "prerender")]
    Prerender,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "cached")]
    Cached,
    #[serde(rename = "pending_deletion")]
    PendingDeletion,
}

#[derive(Debug, Serialize, Deserialize)]
enum FrameType {
    #[serde(rename = "outermost_frame")]
    OutermostFrame,
    #[serde(rename = "fenced_frame")]
    FencedFrame,
    #[serde(rename = "sub_frame")]
    SubFrame,
}

#[derive(Debug, Serialize, Deserialize)]
struct HttpHeaders {
    #[serde(rename = "binaryValue")]
    binary_value: Option<Vec<i8>>,
    name: Option<String>,
    value: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
enum ResourceType {
    #[serde(rename = "main_frame")]
    MainFrame,
    #[serde(rename = "sub_frame")]
    SubFrame,
    #[serde(rename = "stylesheet")]
    Stylesheet,
    #[serde(rename = "script")]
    Script,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "font")]
    Font,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "xmlhttprequest")]
    XmlHttpRequest,
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "csp_report")]
    CspReport,
    #[serde(rename = "media")]
    Media,
    #[serde(rename = "websocket")]
    Websocket,
    #[serde(rename = "other")]
    Other
}


#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
struct Request {
    data: Option<String>,
    #[serde(rename(deserialize="documentId"))]
    document_id: Option<String>,
    #[serde(rename(deserialize="documentLifecycle"))]
    document_lifecycle: Option<DocumentLifecycle>,
    #[serde(rename(deserialize="frameId"))]
    frame_id: Option<i32>,
    #[serde(rename(deserialize="frameType"))]
    frame_type: Option<FrameType>,
    #[serde(rename(deserialize="fromCache"))]  
    from_cache: Option<bool>,
    initiator: Option<String>,
    ip: Option<String>,
    method: Option<String>,
    #[serde(rename(deserialize="parentDocumentId"))]
    parent_document_id: Option<String>,
    #[serde(rename(deserialize="parentFrameId"))]
    parent_frame_id: Option<i64>,
    #[serde(rename(deserialize="requestId"))]
    request_id: Option<String>,
    #[serde(rename(deserialize="responseHeaders"))]
    response_headers: Option<HttpHeaders>,
    #[serde(rename(deserialize="statusCode"))]
    status_code: i32,
    #[serde(rename(deserialize="statusLine"))]
    status_line: Option<String>,
    #[serde(rename(deserialize="tabId"))]
    tab_id: i32,
    #[serde(rename(deserialize="timeStamp"))]
    #[serde_as(as = "Option<TimestampMilliSeconds<String, Flexible>>")]
    time_stamp: Option<DateTime>,
    #[serde(rename(deserialize="type"))]
    resoutce_type: ResourceType,
    url: String,

}

#[derive(Debug, Serialize, Deserialize)]
//#[serde(transparent)]
struct LogsBody {
    user: String,
    requests: Vec<Request>,
}

struct AppState {
    //file_handler: Mutex<csv_async::AsyncSerializer<File>>, // <- Mutex is necessary to mutate safely across threads
    database: sea_orm::DatabaseConnection
}

#[get("/")]
async fn on_ping() -> impl Responder{
    return HttpResponse::Ok();
}

#[post("/logging")]
async fn log_to_file(request_body: web::Json<LogsBody>, writer: web::Data<AppState>) -> impl Responder {//
    let db = &writer.database;
    let mut all_querries: Vec<ActiveModel> = vec![];
    for row in request_body.requests.iter() {
        let mut model = ActiveModel {
            id: NotSet,
            ..Default::default()
        };
        let v = serde_json::to_value(row).expect("Error on parse");
        model.set_from_json(v).expect("error on db parse");
        model.user = Set(Some(request_body.user.to_owned()));
        model.server_time_stamp = Set(DateTime::from_timestamp_millis(Utc::now().timestamp_millis()));
        all_querries.push(model);

    }
    RequestTable::insert_many(all_querries).exec(db).await.expect("Error on insert DB");
    return HttpResponse::Ok().body("OK");
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = settings::get_config().await.expect("Error while config parse");
    println!("Server must be up. on {}:{}", 
        config.get_string("server.url").unwrap_or("127.0.0.1".to_string()), config.get_int("server.port").unwrap_or(8080)
    );
    let db: sea_orm::DatabaseConnection = sea_orm::Database::connect(config.get_string("database.db_url").unwrap()).await.expect("Error on connect to database");
    let counter = web::Data::new(AppState {
        //file_handler: Mutex::new(wtr),
        database: db
    });
    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .service(log_to_file)
            .service(on_ping)

    })
        .max_connections(25000)
        .workers(32)
        .bind((config.get_string("server.url").unwrap_or("127.0.0.1".to_string()), config.get_int("server.port").unwrap_or(8080) as u16))?
        .run()
        .await
}