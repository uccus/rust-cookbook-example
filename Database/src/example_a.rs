use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

pub fn test() {
    let conn = Connection::open("cats.db").unwrap();
    conn.execute(
        "create table if not exist cat_colors (id integer primary key, name text not null unique)"
        , NO_PARAMS).unwrap();
    conn.execute(
            "create table if not exists cats (
                 id integer primary key,
                 name text not null,
                 color_id integer not null references cat_colors(id)
             )",
            NO_PARAMS,
        ).unwrap();
}