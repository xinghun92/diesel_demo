#![recursion_limit="128"]
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate diesel_codegen;
extern crate time;
extern crate r2d2;
extern crate r2d2_diesel;

mod models;
mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use models::{Chatter, NewChatter};
use schema::chatters;
use diesel::ExecuteDsl;
use r2d2_diesel::ConnectionManager;

pub fn establish_connection() ->  SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let database_url = ::std::path::Path::new("storage").join(database_url);
    let database_url = database_url
        .to_str()
        .unwrap();
    //SqliteConnection::establish(&database_url, Some("test_db".to_string())).unwrap()
    SqliteConnection::establish(&database_url).unwrap()
}

pub fn insert_chatter(chatter_num: i64) {
    let conn = establish_connection();
    let mut chatters: Vec<NewChatter> = Vec::new();

    for i in 0..chatter_num {
        let chatter = NewChatter {
            id: i,
            name: "name",
            type_: 1,
            avatar_key: "chatter_avatar_key",
            update_time: 1,
            name_pinyin: "name",
            creator_id: Some(0),
            is_resigned: Some(true)
        };
        chatters.push(chatter);
    }

    for chatter in chatters {
        let _ = diesel::insert(&chatter).into(chatters::table).execute(&conn);
    }
 }

pub fn insert_or_replace_chatter(chatter_num: i64) {
    let conn = establish_connection();
    let mut chatters: Vec<NewChatter> = Vec::new();

    for i in 0..chatter_num {
        let chatter = NewChatter {
            id: i,
            name: "name",
            type_: 1,
            avatar_key: "chatter_avatar_key",
            update_time: 1,
            name_pinyin: "name",
            creator_id: Some(0),
            is_resigned: Some(true)
        };
        chatters.push(chatter);
    }

    for chatter in chatters {
        let _ = diesel::insert_or_replace(&chatter).into(chatters::table).execute(&conn);
    }
 }

// pub fn insert_chatters(chatter_num: i64) {
// //    let conn = establish_connection();
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set");
//     let database_url = ::std::path::Path::new("storage").join(database_url);
//     let database_url = database_url
//         .to_str()
//         .unwrap();
//     let config = r2d2::Config::default();
//     //let manager = ConnectionManager::<SqliteConnection>::new(database_url, Some("test_db".to_string()));
//     let pool = r2d2::Pool::new(config, manager).unwrap();
//     let conn = pool.get().unwrap();
//     let mut chatters: Vec<NewChatter> = Vec::new();

//     for i in 0..chatter_num {
//         let chatter = NewChatter {
//             id: i,
//             name: "name",
//             type_: 1,
//             avatar_key: "chatter_avatar_key",
//             update_time: 1,
//             name_pinyin: "name",
//             creator_id: None,
//             is_resigned: None
//         };
//         chatters.push(chatter);
//     }

//     let res = diesel::insert(&chatters).into(chatters::table).execute(&*conn);
//     println!("{:?}", res);
// }

// pub fn change_pwd(pwd: &str) {
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set");
//     let database_url = ::std::path::Path::new("storage").join(database_url);
//     let database_url = database_url
//         .to_str()
//         .unwrap();
//     let config = r2d2::Config::default();
//     let manager = ConnectionManager::<SqliteConnection>::new(database_url, Some("new_pwd".to_string()));
//     let pool = r2d2::Pool::new(config, manager).unwrap();
//     let conn = pool.get().unwrap();

//     (*conn).change_password(pwd).unwrap();
// }

pub fn delete_chatters() {
    let conn = establish_connection();

    let _ = diesel::delete(chatters::table).execute(&conn);
}

pub fn main() {
    let chatter_nums= vec![100];
//    let conn = establish_connection();
//    embed_migrations!();
//    embedded_migrations::run(&conn).unwrap();
    for chatter_num in chatter_nums {
       println!("-----------------------------------");
       println!("chatter_num is {}", chatter_num);
//        insert_chatter(chatter_num);
//        let mut chatter_ids = Vec::new();
//        for i in 0..chatter_num {
//            chatter_ids.push(i);
//        }
//        let sql = debug_sql!(diesel::update(chatters::dsl::chatters.filter(chatters::dsl::id.eq_any(&chatter_ids)))
//        .set(chatters::dsl::name.eq("changed")));
//        println!("{:?}", sql);
//        delete_chatters();
       insert_or_replace_chatter(chatter_num);
       delete_chatters();
//        insert_chatters(chatter_num);
//        delete_chatters();
        // change_pwd("test_db");
    }
}
