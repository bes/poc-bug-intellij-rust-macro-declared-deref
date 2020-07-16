#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use diesel::MysqlConnection;

#[database("poc")]
pub struct DbConn(MysqlConnection);

fn takes_mysql_connection(conn: &MysqlConnection) {
    todo!("Not implemented")
}

fn takes_db_conn(conn: &DbConn) {
    // Works, since we are giving it a &MysqlConnection
    takes_mysql_connection(&conn.0);
    // Works since there is a Deref declared by `#[database]`
    // But IntelliJ rust doesn't seem to understand...
    takes_mysql_connection(conn);
}

fn main() {
    todo!("Hello world")
}
