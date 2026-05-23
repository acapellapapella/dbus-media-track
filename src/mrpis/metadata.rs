use std::collections::HashMap;

use zbus::zvariant::OwnedValue;

use crate::mrpis::client::{MrpisError, Player};

#[derive(Debug)]
#[allow(dead_code)]
pub struct TrackInfo {
    pub title: Option<String>,
    pub artist: Option<Vec<String>>,
    pub album: Option<String>,
    pub art_url: Option<String>,
}

pub async fn get_metadata(player: &Player) -> Result<TrackInfo, MrpisError> {
    let value = player.proxy.get(player.iface.clone(), "Metadata").await?;
    let metadata: HashMap<String, OwnedValue> = value.try_into()?;

    Ok(TrackInfo {
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
