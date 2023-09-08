use criterion::{
    criterion_group,
    criterion_main,
    BenchmarkId,
    Criterion,
    Throughput,
};
use poem::{
    http::StatusCode,
    test::TestClient,
    Endpoint,
};
use rand::{
    distributions::Alphanumeric,
    Rng,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
    password: String,
}

async fn create_user<E: Endpoint>(
    cli: &TestClient<E>,
    user: &User,
) {
    let resp = cli.post("/users").body_json(&user).send().await;

    // println!("{:#?}", resp.0.into_body().into_string().await);

    resp.assert_status(StatusCode::CREATED);
    resp.assert_text("").await;
}

// async fn get_users<E: Endpoint>(cli: &TestClient<E>) {
//     let resp = cli.get("/users").send().await;
//     resp.assert_status(StatusCode::OK);
// }

fn bench_create_user(c: &mut Criterion) {
    dotenv::dotenv().ok();

    let users = (0..3)
        .map(|_| {
            let name = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(8)
                .map(char::from)
                .collect::<String>();

            let email = format!("{}@example.com", name);

            User {
                name,
                email,
                password: "password123".to_string(),
            }
        })
        .collect::<Vec<_>>();

    let config = api::Config::new();
    let app = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(api::get_app(&config));
    let cli = TestClient::new(app);

    let mut group = c.benchmark_group("create_user");
    for (i, user) in users.iter().enumerate() {
        group.throughput(Throughput::Elements(1));
        group.bench_with_input(BenchmarkId::from_parameter(i), user, |b, user| {
            b.to_async(tokio::runtime::Runtime::new().unwrap())
                .iter(|| create_user(&cli, user))
        });
    }
    group.finish();
}

// fn bench_get_users(c: &mut Criterion) {
//     c.bench_function("get users", |b| {
//         b.to_async(tokio::runtime::Runtime::new().unwrap())
//             .iter(get_users)
//     });
// }

// criterion_group!(benches, bench_create_user, bench_get_users);
criterion_group!(benches, bench_create_user);
criterion_main!(benches);
