use std::collections::HashMap;

use zbus::zvariant::OwnedValue;

use crate::mrpis::client::{MrpisError, Player};

#[derive(Debug, PartialEq, Eq)]
pub enum PlaybackStatus {
    Playing,
    Paused,
    Stopped,
}

impl PlaybackStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Playing => "Playing",
            Self::Paused => "Paused",
            Self::Stopped => "Stopped",
        }
    }

    fn parse(status: &str) -> Result<Self, MrpisError> {
        match status {
            "Playing" => Ok(Self::Playing),
            "Paused" => Ok(Self::Paused),
            "Stopped" => Ok(Self::Stopped),
            unknown => Err(MrpisError::UnknownStatus(unknown.to_string())),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct TrackInfo {
    pub status: PlaybackStatus,
    pub title: Option<String>,
    pub artist: Option<Vec<String>>,
    pub album: Option<String>,
    pub art_url: Option<String>,
}

pub async fn get_metadata(player: &Player) -> Result<TrackInfo, MrpisError> {
    let value = player.proxy.get(player.iface.clone(), "Metadata").await?;
    let metadata: HashMap<String, OwnedValue> = value.try_into()?;

    let status_value = player
        .proxy
        .get(player.iface.clone(), "PlaybackStatus")
        .await?;
    let status: String = status_value.try_into()?;

    Ok(TrackInfo {
        status: PlaybackStatus::parse(&status)?,
        title: extract(&metadata, "xesam:title"),
        artist: extract(&metadata, "xesam:artist"),
        album: extract(&metadata, "xesam:album"),
        art_url: extract(&metadata, "mpris:artUrl"),
    })
}

fn extract<T>(map: &HashMap<String, OwnedValue>, key: &str) -> Option<T>
where
    T: TryFrom<OwnedValue>,
{
    map.get(key)?.try_clone().ok()?.try_into().ok()
}
