#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

pub mod models;
pub mod schema;
mod messaging;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

use messaging::send_amqp_message;
use models::{Import, NewImport};

pub fn establish_db_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url).expect(&format!("Error connecting to {}",
                                                              database_url))
}

pub fn get_imports(conn: &MysqlConnection) -> Vec<Import> {
    use self::schema::imports::dsl::*;

    imports
        .load::<Import>(conn)
        .expect("Error loading imports")
}

pub fn get_import(conn: &MysqlConnection, import_id: &i32) -> Import {
    use self::schema::imports::dsl::*;

    imports
        .find(import_id)
        .get_result(conn)
        .expect(&format!("Error loading import with id={}", import_id))
}

pub fn create_import<'a>(conn: &MysqlConnection, name: &'a str) -> Import {
    use schema::imports;

    let new_import = NewImport { name: name };

    diesel::insert(&new_import)
        .into(imports::table)
        .execute(conn)
        .expect("Error saving new import");
    send_amqp_message();

    imports::table.order(imports::id.desc()).first(conn).unwrap()
}
