use anyhow;
use sqlx::postgres;

#[derive(sqlx::FromRow)]
struct Animal {
  name: Option<String>,
}

pub async fn init() -> anyhow::Result<()> {
  println!("init psql");

  let database = "postgres";
  let user = "root";
  let password = "root";
  let port = "5433";
  let host = "localhost";

  // postgresql://root:root@localhost:5433/postgres
  let url = format!(
    "postgres://{}:{}@{}:{}/{}",
    user, password, host, port, database
  );

  println!("he");
  println!("{}", url);

  let pool = postgres::PgPoolOptions::new()
    .max_connections(20)
    .connect(&url)
    .await?;

  let animals = sqlx::query_as!(Animal, "select * from animals")
    .fetch_all(&pool)
    .await?;

  println!("{:?}", animals[0].name);
  Ok(())
}
