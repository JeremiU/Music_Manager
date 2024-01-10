mod data_structures;
mod util;

use std::io::Read;

use reqwest::{self, Error, Client};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::data_structures::*;
use crate::util::*;

#[tokio::main]
async fn main() {
    let pairs = &[("LP!", "JPEGMAFIA")];

    for (album_title, album_artist) in pairs {
        let _ = print_info(album_title, album_artist).await;
    }
}

async fn print_info(album_title: &str, album_artist: &str) -> Result<(), Error>  {
    let client = Client::new();
    let client_data = client_data();
    let token = str_to_field(get_token(&client, &client_data).await?, "access_token");

    let album = album(find_album(&client, &token, album_title, album_artist).await?);
    let g = ms_to_minutes(get_info(&client, &token, &album.id).await?);
    let year = album.release_date.split("-").next().unwrap();

    let artists = album.artists.join(" & ");
    println!("{},{},{},{},{},{},{},{},{},{},{}", album.name, artists, year, "","", "", "","","","",g);
    Ok(())
}

async fn find_album(client: &Client, token: &str, search: &str, artist: &str) -> Result<String, Error> {
    let query = &mut String::new();
    if !artist.is_empty() {
        *query = format!("album%2520{}%2520artist%3A{}", search, artist);
    } else {
        *query = search.to_owned();
    }
    
    let response = client.get("https://api.spotify.com/v1/search") 
    .header("Authorization", format!("Bearer {}", token))
    .query(&[("q", query.as_str()), ("type", "album"), ("market","US"), ("limit","1"), ("offset","0")])
    .send().await?;

    Ok(response.text().await?)
}

async fn get_info(client: &Client, token: &str, id: &str) -> Result<i64, Error> {
    let response = client.get(format!("https://api.spotify.com/v1/albums/{}", id)) 
    .header("Authorization", format!("Bearer {}", token))
    .query(&[("limit","1000"), ("market","US")])
    .send()
    .await?;

    let json: Value = serde_json::from_str(&response.text().await?).unwrap();
    let duration = &mut 0;

    if let Some(val) =  json.get("tracks").and_then(|a| a.get("items")).and_then(|i| i.as_array())  {
        for track in val {
            let ms = track.get("duration_ms").unwrap().as_i64().unwrap();
            *duration = *duration + ms;
        }
    }
    Ok(duration.to_owned())
}

pub fn album(json: String) -> Album {
    let artists: &mut Vec<String> = &mut Vec::new();
    let id = &mut String::new();
    let release_date = &mut String::new();
    let media_type = &mut String::new();
    let name = &mut String::new();

    let json: Value = serde_json::from_str(&json).unwrap();
    if let Some(val) =  json.get("albums").and_then(|a| a.get("items")).and_then(|i| i.as_array())  {
        let album = val.get(0).unwrap();
        for artist in album["artists"].as_array().unwrap() {
            artists.push((*artist)["name"].as_str().unwrap().to_owned());
        }
        *id = album["id"].as_str().unwrap().to_owned();
        *name = album["name"].as_str().unwrap().to_owned();
        *release_date = album["release_date"].as_str().unwrap().to_owned();
        *media_type = album["album_type"].as_str().unwrap().to_owned();
    }
    Album {artists: artists.to_owned(), name: name.to_owned(), release_date: release_date.to_owned(), media_type: media_type.to_owned(), id: id.to_owned()}
}