extern crate rspotify;

use std::env;
use std::collections::HashMap;

use rspotify::blocking::client::Spotify;
use rspotify::blocking::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::senum::Country;
use dotenv::dotenv;

type Result<T> = std::result::Result<T,Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    dotenv().ok();

    let env_vars = env::vars().collect::<HashMap<_, _>>();

    for (key, _value) in env_vars.iter() {
        println!("{:?}", key)
    }

    let client_credential = SpotifyClientCredentials::default().client_id();
    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();

    match spotify.me() {
        Ok(user_profile) => println!("{:?}", user_profile),
        Err(error) => println!("{:?}", error),
    }

    // let birdy_uri = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
    // let tracks = spotify.artist_top_tracks(birdy_uri, Country::UnitedStates)?;

    // println!("{:?}", tracks);

    Ok(())
}