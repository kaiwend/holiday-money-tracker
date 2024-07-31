use axum::{
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use minijinja::render;
use serde::{Deserialize, Serialize};
use tokio::fs;
use tracing_subscriber;

#[derive(Debug, Serialize, Deserialize)]
struct Budget {
    current: u8,
}

impl Budget {
    pub fn new(current: u8) -> Self {
        Self { current }
    }
}

const HOME_TEMPLATE: &'static str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Money Raining Animation</title>
    <style>
        h1 {
            color: white;
            text-align: center;
            margin-top: 50px;
        }
        body {
            margin: 0;
            overflow: hidden;
            background-color: #1e1e1e;
        }

        .money {
            position: absolute;
            top: -50px;
            width: 50px;
            height: auto;
            opacity: 0;
            animation: fall linear infinite, rotate linear infinite;
        }

        @keyframes fall {
            0% {
                transform: translateY(0) rotate(0deg);
                opacity: 1;
            }
            100% {
                transform: translateY(110vh) rotate(360deg);
                opacity: 0;
            }
        }
    </style>
</head>
<body>
    <!-- Container to hold the money elements -->
    <div id="money-container"></div>
    <h1>Current Budget: {{ budget.current }} €</h1>

    <script>
        // Number of money elements
        const numberOfMoney = 30;

        const moneyContainer = document.getElementById('money-container');

        for (let i = 0; i < numberOfMoney; i++) {
            const money = document.createElement('img');
            money.src = 'https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Ftse2.mm.bing.net%2Fth%3Fid%3DOIP.mFmgsSeYBIdY3NoKWOQe8AHaDw%26pid%3DApi&f=1&ipt=ac288281d6b7bc3fcba5f0802a9470bade42e4d9c54542971e0d3090d4919933&ipo=images'; // Replace with your image URL
            money.className = 'money';
            money.style.left = `${Math.random() * 100}vw`;
            money.style.animationDuration = `${Math.random() * 3 + 2}s`;
            moneyContainer.appendChild(money);
        }
    </script>
</body>
</html>
"#;

async fn home() -> Html<String> {
    let current_budget = match fs::read_to_string("current_budget").await {
        Ok(budget) => budget,
        Err(_) => return Html("No budget found".to_string()),
    };
    let current_budget: u8 = match current_budget.parse::<u8>() {
        Ok(budget) => budget,
        Err(_) => return Html("Invalid budget".to_string()),
    };
    let r = render!(HOME_TEMPLATE, budget => Budget::new(current_budget));
    Html(r)
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(home))
        .route("/add", post(add_budget))
        .route("/subtract", post(subtract_budget));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3005").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn add_budget(Json(body): Json<Budget>) -> (StatusCode, Json<Budget>) {
    let current_budget = match fs::read_to_string("current_budget").await {
        Ok(budget) => budget,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(Budget::new(0))),
    };
    let current_budget: u8 = match current_budget.parse::<u8>() {
        Ok(budget) => budget,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(Budget::new(0))),
    };
    let new_budget = current_budget + body.current;
    fs::write("current_budget", new_budget.to_string())
        .await
        .unwrap();
    (StatusCode::CREATED, Json(Budget::new(new_budget)))
}

async fn subtract_budget(Json(body): Json<Budget>) -> (StatusCode, Json<Budget>) {
    let current_budget = match fs::read_to_string("current_budget").await {
        Ok(budget) => budget,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(Budget::new(0))),
    };
    let current_budget: u8 = match current_budget.parse::<u8>() {
        Ok(budget) => budget,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(Budget::new(0))),
    };
    let new_budget = current_budget - body.current;
    fs::write("current_budget", new_budget.to_string())
        .await
        .unwrap();
    (StatusCode::CREATED, Json(Budget::new(new_budget)))
}
