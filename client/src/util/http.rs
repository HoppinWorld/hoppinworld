use amethyst::prelude::*;
use amethyst_extra::dirty::Dirty;
use amethyst_extra::*;
use crossbeam_channel::Sender;
use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;
use tokio::prelude::{Future, Stream};
use tokio::runtime::Runtime;

pub fn do_login(
    future_runtime: &mut Runtime,
    queue: Sender<Callback>,
    username: String,
    password: String,
) {
    let https = HttpsConnector::new(2).expect("TLS initialization failed");
    let client = Client::builder().build::<_, hyper::Body>(https);
    let request = Request::post("https://hoppinworld.net:27015/login")
        .header("Content-Type", "application/json")
        .body(Body::from(format!(
            "{{\"email\":\"{}\", \"password\":\"{}\"}}",
            username, password
        )))
        .unwrap();

    let future = client
        .request(request)
        .and_then(move |result| {
            println!("Response: {}", result.status());
            println!("Headers: {:#?}", result.headers());
            result.into_body().for_each(move |chunk| {
                match serde_json::from_slice::<Auth>(&chunk) {
                    Ok(a) => queue
                        .send(Box::new(move |world| {
                            world.write_resource::<Dirty<Auth>>().write().token = a.token.clone();
                            world
                                .write_resource::<Dirty<Auth>>()
                                .write()
                                .set_validated(true);
                        }))
                        .expect("Failed to push auth callback to future queue"),
                    Err(e) => error!("Failed to parse received data to Auth: {}", e),
                }
                Ok(())
            })
        })
        .map(move |_| {
            info!("\n\nLogin successful.");
        })
        // TODO: Show error
        .map_err(|err| {
            error!("Failed to login. Error: {}", err);
        });
    future_runtime.spawn(future);
}

// TODO remove dup from backend
#[derive(Serialize, Deserialize)]
pub struct ScoreInsertRequest {
    pub mapid: i32,
    pub segment_times: Vec<f32>,
    pub strafes: i32,
    pub jumps: i32,
    /// Seconds
    pub total_time: f32,
    pub max_speed: f32,
    pub average_speed: f32,
}

pub fn submit_score(
    future_runtime: &mut Runtime,
    auth_token: String,
    score_insert_request: ScoreInsertRequest,
) {
    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    let client = Client::builder().build::<_, hyper::Body>(https);
    let request = Request::post("https://hoppinworld.net:27015/submitscore")
        .header("Content-Type", "application/json")
        .header("X-Authorization", format!("Bearer {}", auth_token))
        .body(Body::from(json!(score_insert_request).to_string()))
        .unwrap();

    let future = client
        .request(request)
        .and_then(move |result| {
            println!("Response: {}", result.status());
            println!("Headers: {:#?}", result.headers());

            result.into_body().for_each(move |chunk| {
                info!(
                    "{}",
                    String::from_utf8(chunk.to_vec()).unwrap_or(
                        "Error converting server answer to string after score submission"
                            .to_string()
                    )
                );
                Ok(())
            })
        })
        .map(move |_| {
            info!("\n\nScore submitted with success.0");
        })
        .map_err(|err| {
            error!("Error {}", err);
        });
    future_runtime.spawn(future);
}

pub fn validate_auth_token(
    future_runtime: &mut Runtime,
    auth_token: String,
    queue: Sender<Callback>,
) {
    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    let client = Client::builder().build::<_, hyper::Body>(https);
    let request = Request::post("https://hoppinworld.net:27015/validatetoken")
        .header("Content-Type", "application/json")
        .header("X-Authorization", format!("Bearer {}", auth_token))
        .body(Body::from(json!(auth_token).to_string()))
        .unwrap();

    let future = client
        .request(request)
        .and_then(move |result| {
            println!("Response: {}", result.status());
            println!("Headers: {:#?}", result.headers());

            result.into_body().for_each(move |chunk| {
                let valid = match serde_json::from_slice::<bool>(&chunk) {
                    Ok(_a) => true,
                    Err(e) => {
                        error!("Failed to parse received data to validation bool: {}", e);
                        false
                    }
                };
                queue
                    .send(Box::new(move |world| {
                        world
                            .write_resource::<Dirty<Auth>>()
                            .write()
                            .set_validated(valid);
                    }))
                    .expect("Failed to push auth validation callback to future queue");
                Ok(())
            })
        })
        .map(move |_| {
            info!("\n\nAuth token validation request submitted with success to server.");
        })
        .map_err(|err| {
            error!("Error {}", err);
        });
    future_runtime.spawn(future);
}
