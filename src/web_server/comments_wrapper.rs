// extern crate serde_json;
// extern crate persistent;
// extern crate bodyparser;
//
// use iron::prelude::*;
// use iron::{Handler, status};
// use iron::method::Method;
// use iron::typemap::Key;
//
// use persistent::*;
//
// use serde_json::*;
// use std::env;
//
// #[derive(Serialize, Deserialize, Debug)]
// struct Comment {
//     author: String,
//     text:  String,
//     id: u64,
// }
//
// //Holds file and comments vector
// //loads file nad dump it into vector
// //keeps adding new comments into vector
// //every X comments saves to file
// pub struct CommentsWrapper{
//     vec_comments : Vec<Comment>,
// }
//
// impl CommentsWrapper{
//     pub fn new()->Self{
//         CommentsWrapper{vec_comments :vec![]}
//     }
//
//     // fn load(filename:String)->Json{
//     //      let mut file = File::open(filename).unwrap();
//     //      let mut data = String::new();
//     //      file.read_to_string(&mut data).unwrap();
//     //
//     //      let json = Json::from_str(&data).unwrap();
//     //      json
//     // }
//
//     fn as_string(&self)->String{
//         let ret = serde_json::to_string(&self.vec_comments).unwrap();
//         format!("{}",ret)
//     }
//
//     fn add_comment(& mut self, comment:Comment)
//     {
//         self.vec_comments.push(comment);
//     }
// }
//
// impl Key for CommentsWrapper { type Value = CommentsWrapper; }
//
// pub fn handle_comments(req: &mut Request) -> IronResult<Response> {
//      let mutex = req.get::<persistent::Write<CommentsWrapper>>().unwrap();
//      let mut dyamic_asset = mutex.lock().unwrap();
//
//      let struct_body = req.get::<bodyparser::Struct<Comment>>();
//      match struct_body {
//          Ok(Some(struct_body)) => dyamic_asset.add_comment(struct_body.clone()),
//          Ok(None) => println!("No body {:?}",req.get::<bodyparser::Raw>()),
//          Err(err) => println!("Error: {:?}", err)
//      }
//
//      Ok(Response::with((status::Ok, format!("{}",dyamic_asset.as_string()))))
// }
