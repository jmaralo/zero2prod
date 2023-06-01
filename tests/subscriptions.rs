use reqwest::Client;

mod common;

#[tokio::test]
async fn valid_subscribers_test() {
    let app = common::spawn_app().await;
    let _cleanup = common::Cleanup(app.config);

    let client = Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("http://{}/subscriptions", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn invalid_subscribers_test() {
    let app = common::spawn_app().await;
    let _cleanup = common::Cleanup(app.config);

    let client = Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (body, test_case) in test_cases {
        let response = client
            .post(format!("http://{}/subscriptions", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return 400 BAD_REQUEST on invalid payload ({})",
            test_case
        );
    }
}

#[tokio::test]
async fn malformed_subscribers_name_test() {
    let app = common::spawn_app().await;
    let _cleanup = common::Cleanup(app.config);

    let client = Client::new();
    let test_cases = vec![
        ("name=&email=ursula_le_guin%40gmail.com", "empty name"),
        ("name=Adolph Blaine CHarles David Earl afrederick Gerald Hubert Irvin John Kenneth Lloyd Martin Nero Oliver Paul Quincy Randolph Sherman Thomas Uncas Victor William Xerxes Yancy Zeus Wolfeschlegelsteinhausenbefrrgerdorffwelchevoralternwarengewissenhaftschaferswessschafewarenwohlgepflegeundsorgfaltigkeitbeschutzen&email=adolph%40gmail.com", "long name."),
        ("name=Robert'); DROP TABLE Students;--&email=robert%40gmail.com", "malicious name"),
    ];

    for (body, description) in test_cases {
        let response = client
            .post(&format!("http://{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The ;api did not return a 400 BAD_REQUEST when the payload was {}.",
            description
        )
    }
}

#[tokio::test]
async fn malformed_subscribers_emails_test() {
    let app = common::spawn_app().await;
    let _cleanup = common::Cleanup(app.config);

    let client = Client::new();
    let test_cases = vec![
        ("name=Ursula&email=", "empty email"),
        ("name=Ursula&email=this-is-not-an-email", "invalid email"),
    ];

    for (body, description) in test_cases {
        let response = client
            .post(&format!("http://{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            200,
            response.status().as_u16(),
            "The ;api did not return a 200 OK when the payload was {}.",
            description
        )
    }
}
