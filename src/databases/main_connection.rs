//use postgres::{Connection, TlsMode};

/*pub fn conn() -> Connection {
    let url = "PostgreSQL11://postgres:Dana0Yara@localhost:5432/postgres";
    let conn = Connection::connect( url, TlsMode::None).
    expect("Couln't connect to DB");
    conn
} */

#[database("pgdb1")]
pub struct PgDb1(postgres::Connection);

//#[database("pgdb2")]
//struct PgDb2(postgres::Connection);
