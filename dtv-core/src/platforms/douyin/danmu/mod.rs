// This file makes the 'danmu' directory a module.

pub mod gen {
    #![allow(clippy::all, warnings)]
    include!(concat!(env!("OUT_DIR"), "/douyin.rs"));
}

pub mod message_handler;
pub mod message_parsers;
pub mod signature;
pub mod sign_worker;
pub mod web_fetcher;
pub mod websocket_connection;