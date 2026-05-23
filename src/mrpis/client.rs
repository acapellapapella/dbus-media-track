use thiserror::Error;
use zbus::{Connection, fdo::PropertiesProxy, names::InterfaceName};

#[derive(Error, Debug)]
pub enum MrpisError {
    #[error(transparent)]
    DbusError(#[from] zbus::Error),

    #[error("FDO error: {0}")]
    FdoError(#[from] zbus::fdo::Error),

    #[error(transparent)]
    VariantError(#[from] zbus::zvariant::Error),
}

pub struct Player {
    pub proxy: PropertiesProxy<'static>,
    pub iface: InterfaceName<'static>,
}

pub async fn find(player: Option<&str>, only_playing: bool) -> Result<Option<Player>, MrpisError> {
    let connection = Connection::session().await?;

    let dbus = zbus::fdo::DBusProxy::new(&connection).await?;
    let names = dbus.list_names().await?;

    let prefix = match player {
        Some(name) => format!("org.mpris.MediaPlayer2.{name}"),
        None => String::from("org.mpris.MediaPlayer2."),
    };

    let iface =
        InterfaceName::try_from("org.mpris.MediaPlayer2.Player").map_err(zbus::Error::from)?;

    for name in names {
        if !name.as_ref().starts_with(&prefix) {
            continue;
        }

        let proxy = PropertiesProxy::new(&connection, name, "/org/mpris/MediaPlayer2").await?;

        if only_playing {
            let value = proxy.get(iface.clone(), "PlaybackStatus").await?;
            let status: String = value.try_into()?;
            if status != "Playing" {
                continue;
            }
        }

        return Ok(Some(Player { proxy, iface }));
    }

    Ok(None)
}
