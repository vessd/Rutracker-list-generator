mod error;

pub use self::error::Error;
use rpc::{TorrentClient, TorrentStatus};
use rutracker::api::{RutrackerApi, TopicData};

#[derive(Debug)]
struct Torrent {
    data: TopicData,
    status: TorrentStatus,
}

impl Torrent {
    fn new(status: TorrentStatus, data: TopicData) -> Torrent {
        Torrent { data, status }
    }
}

#[derive(Debug)]
pub struct TorrentList {
    list: Vec<Torrent>,
}

impl TorrentList {
    pub fn new<C>(client: &mut C, api: &RutrackerApi) -> Result<TorrentList, Error>
    where
        C: TorrentClient,
    {
        let mut client_list = client.list()?;
        let ids = api.get_topic_id(&client_list
            .iter()
            .map(|(hash, _)| hash.as_str())
            .collect::<Vec<&str>>())?
            .into_iter()
            .filter_map(|(_, id)| id)
            .collect::<Vec<usize>>();
        let topics_data = api.get_tor_topic_data(&ids)?
            .into_iter()
            .filter_map(|(_, t)| t)
            .collect::<Vec<TopicData>>();
        Ok(TorrentList {
            list: topics_data
                .into_iter()
                .filter(|data| data.tor_status == 2 || data.tor_status == 8)
                .map(|data| {
                    Torrent::new(client_list.remove(&data.info_hash).unwrap(), data)
                })
                .collect(),
        })
    }

    pub fn get(&self, status: TorrentStatus) -> Vec<&str> {
        self.list
            .iter()
            .filter(|t| t.status == status)
            .map(|t| t.data.info_hash.as_str())
            .collect()
    }

    pub fn remove_by_poster_id(&mut self, id: usize) {
        self.list.retain(|t| t.data.poster_id != id);
    }
}