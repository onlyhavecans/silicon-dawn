use silicon_dawn::cards::CardDeck;
use silicon_dawn::startup::run;
use std::net::TcpListener;

struct TestApp {
    address: String,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("::1:0").expect("failed to bind to random port");
    let address = listener.local_addr().unwrap().to_string();

    let deck: CardDeck = vec!["title.jpg".to_string()];

    let path = "tests/test_files";
    let server = run(listener, deck, path).expect("failed to bind address");
    let _ = tokio::spawn(server);
    TestApp { address }
}

#[tokio::test]
async fn health_check_works() {
    let test_conf = spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{}/health_check", &test_conf.address))
        .send()
        .await
        .expect("failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn robots_txt_works() {
    let test_conf = spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{}/robots.txt", &test_conf.address))
        .send()
        .await
        .expect("failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(25), response.content_length());

    let body = response.text().await.expect("could not read body");
    assert_eq!("User-agent: *\nDisallow: /", body);
}

#[tokio::test]
async fn cards_works() {
    let test_conf = spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{}/cards/", &test_conf.address))
        .send()
        .await
        .expect("failed to execute request.");

    assert_eq!(404, response.status().as_u16());

    let response = client
        .get(&format!(
            "http://{}/cards/test-back.png",
            &test_conf.address
        ))
        .send()
        .await
        .expect("failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(1953991), response.content_length());
}

#[tokio::test]
async fn index_works() {
    let test_conf = spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{}/", &test_conf.address))
        .send()
        .await
        .expect("failed to execute request.");

    assert!(response.status().is_success());

    let body = response.text().await.expect("could not read body");
    assert!(body.contains("Egypt Urnash"))
}
