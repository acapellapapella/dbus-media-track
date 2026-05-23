mod mrpis;

use std::{io::Write, time::Duration};

use clap::Parser;
use futures_util::StreamExt;

use crate::mrpis::{
    client::find,
    metadata::{TrackInfo, get_metadata},
};

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    player: Option<String>,

    #[arg(short, long)]
    service: bool,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    tokens: Vec<String>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if cli.service {
        run_service(&cli).await;
        return;
    }

    let player = find(cli.player.as_deref(), true)
        .await
        .unwrap()
        .expect("no playing MPRIS player");
    let metadata = get_metadata(&player).await.unwrap();
    emit(&metadata, &cli.tokens);
}

async fn run_service(cli: &Cli) {
    loop {
        let player = loop {
            match find(cli.player.as_deref(), false).await {
                Ok(Some(p)) => break p,
                _ => tokio::time::sleep(Duration::from_secs(2)).await,
            }
        };

        if let Ok(meta) = get_metadata(&player).await {
            emit(&meta, &cli.tokens);
        }

        let mut stream = match player.proxy.receive_properties_changed().await {
            Ok(s) => Box::pin(s),
            Err(_) => continue,
        };

        loop {
            tokio::select! {
                next = stream.next() => {
                    if next.is_none() { break; }
                    match get_metadata(&player).await {
                        Ok(meta) => emit(&meta, &cli.tokens),
                        Err(_) => break,
                    }
                }
                _ = tokio::time::sleep(Duration::from_secs(1)) => {
                    if get_metadata(&player).await.is_err() {
                        break;
                    }
                }
            }
        }
    }
}

fn emit(metadata: &TrackInfo, tokens: &[String]) {
    let line = if tokens.is_empty() {
        format!("{metadata:#?}")
    } else {
        let parts: Vec<String> = tokens
            .iter()
            .filter_map(|t| match t.as_str() {
                "--title" => metadata.title.clone(),
                "--artist" => metadata.artist.as_ref().map(|v| v.join(", ")),
                "--album" => metadata.album.clone(),
                "--art_url" => metadata.art_url.clone(),
                _ => Some(t.clone()),
            })
            .collect();
        parts.join(" ")
    };

    let mut stdout = std::io::stdout().lock();
    let _ = writeln!(stdout, "{line}");
    let _ = stdout.flush();
}
