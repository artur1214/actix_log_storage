use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
//use tokio::{sync::Mutex, fs::File};
//use std::error::Error;
//use csv_async::AsyncWriter;
mod entities;
use entities::request::{Entity as RequestTable, ActiveModel};
use sea_orm::{self, ActiveModelTrait, ActiveValue::NotSet, EntityTrait, Set};
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
    time_stamp: f64,
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
        model.set_from_json(serde_json::to_value(row).unwrap()).unwrap();
        model.user = Set(Some(request_body.user.to_owned()));
        println!("{:?}", &model);
        all_querries.push(model);
        
        //model.insert(db).await.unwrap();
        //println!("{}", serde_json::to_string(row).unwrap());
        

        //wri.serialize(row).await.expect("Error on serializing");
        //println!("el {:?}", &wri);
        //wri.flush().await.expect("Error on flushing");
        //println!("el {:?}", &wri);
    }
    RequestTable::insert_many(all_querries).exec(db).await.unwrap();
    return HttpResponse::Ok().body("body");
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let mut wtr = csv_async::AsyncWriterBuilder::new()
    //     .has_headers(true)
    //     //.create_writer(File::create("file_out.csv").await?)
    //     .create_serializer(File::create("file_out.csv").await?);
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