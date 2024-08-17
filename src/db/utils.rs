use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::sync::OnceCell;

// This allows (I believe) a singleton Connectionp Pool that can be shared for the life-time of the applicaitonn.
// We dole out the pool in get_pool() function below.
static CONN_POOL: OnceCell<Pool<Postgres>> = OnceCell::const_new();

pub async fn init_db_conn_pool() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(get_db_connect_string().as_str())
        .await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    println!("DB ready for business: {:?}", row.0 > 0);

    let lazy_db_init = CONN_POOL.set(pool);
    if lazy_db_init.is_err() {
        panic!(
            "ALERT: DB init has error:  {0}",
            lazy_db_init.err().unwrap()
        );
    }

    Ok(())
}

// Return the DB connect string from the .env file, priting out the string with the DB password redacted.
fn get_db_connect_string() -> String {
    let password: &str =
        &std::env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set.");

    let connect_string: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let redacted_connect_string: String = connect_string.replace(password, "<password_redacted>");
    println!("DB Connect str: {0}", redacted_connect_string);

    connect_string
}

// TODO SWY: does this just hand a DB connection as an effect of the &'static" syntax?  If so, rename.
pub fn get_pool() -> &'static Pool<Postgres> {
    return CONN_POOL
        .get()
        .expect("ALERT: Don't have an availalbe pool anymore!");
}
