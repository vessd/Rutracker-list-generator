use super::schema::{forums, keeper_torrents, local_torrents, topics, torrents};
use chrono::naive::NaiveDateTime;
use std::borrow::Cow;

#[derive(Identifiable, Insertable)]
#[primary_key(id)]
pub struct Forum {
    pub id: i16,
    pub name: String,
    pub tor_count: i32,
    pub tor_size_bytes: f64,
    pub topic_id: i32,
}

#[derive(Identifiable, Insertable)]
#[primary_key(keeper, topic_id)]
pub struct KeeperTorrent<'a> {
    pub keeper: Cow<'a, str>,
    pub topic_id: i32,
}

#[derive(Identifiable, Insertable)]
#[primary_key(hash, url)]
pub struct LocalTorrent<'a> {
    pub hash: String,
    pub status: i16,
    pub url: Cow<'a, str>,
}

#[derive(Identifiable, Insertable)]
#[primary_key(id)]
pub struct Topic {
    pub id: i32,
    pub title: String,
    pub author: String,
}

#[derive(Identifiable, Insertable)]
#[primary_key(topic_id)]
pub struct Torrent {
    pub topic_id: i32,
    pub forum_id: i16,
    pub poster_id: i32,
    pub title: String,
    pub hash: String,
    pub size: f64,
    pub reg_time: NaiveDateTime,
    pub status: i16,
    pub seeders: i16,
}
