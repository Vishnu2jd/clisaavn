use reqwest::blocking::Client;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};

#[derive(Debug, Deserialize)]
pub struct SongData {
    pub data: Data,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub results: Vec<ResultData>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct ResultData {
    pub name: String,
    pub album: AlbumData,
    pub year: u16,
    pub language: String,
    pub downloadUrl: Vec<DownloadUrl>,
}

#[derive(Debug, Deserialize)]
pub struct AlbumData {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct DownloadUrl {
    pub url: String,
    pub quality: String,
}

pub fn get_user_input() -> String {
    print!("Enter the song name: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

pub fn fetch_json(song_name: &str) -> Result<SongData, Box<dyn Error>> {
    let base_url = "http://127.0.0.1:3000/api/search/songs?query=";
    let encoded_song_name = song_name.replace(" ", "+");
    let full_url = format!("{}{}", base_url, encoded_song_name);
    println!("{}", full_url);

    let response = reqwest::blocking::get(&full_url)?;

    if response.status().is_success() {
        let body = response.text()?;
        let json_body: SongData = serde_json::from_str(&body)?;

        Ok(json_body)
    } else {
        eprintln!("Request failed with status: {}", response.status());
        Err("Failed to fetch JSON data".into())
    }
}

pub fn display_options(song_data: &SongData) -> Option<usize> {
    println!("Options:");

    for (index, data) in song_data.data.results.iter().enumerate() {
        println!(
            "{}. {} - {} ({}, {})",
            index + 1,
            data.name,
            data.album.name,
            data.year,
            data.language
        );
    }

    print!("Select an option (1-{}): ", song_data.data.results.len());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse::<usize>() {
        Ok(selected_option)
            if selected_option > 0 && selected_option <= song_data.data.results.len() =>
        {
            Some(selected_option - 1)
        }
        _ => {
            println!(
                "Invalid option. Please enter a number between 1 and {}",
                song_data.data.results.len()
            );
            None
        }
    }
}

pub fn fetch_url(song_data: &SongData, selected_option: usize) -> Result<String, Box<dyn Error>> {
    if let Some(url_160kbps) = song_data
        .data
        .results
        .get(selected_option)
        .and_then(|result_data| {
            result_data
                .downloadUrl
                .iter()
                .find(|url| url.quality == "160kbps")
        })
    {
        Ok(url_160kbps.url.clone())
    } else {
        Err("No URL found with 160kbps quality".into())
    }
}

pub fn download_song(url: &str, song_data: &ResultData) -> Result<(), Box<dyn Error>> {
    println!("Downloading song from: {}", url);

    let client = Client::new();
    let response = client.get(url).send()?;

    if response.status().is_success() {
        let filename = format!("{} - {}.m4a", song_data.name, song_data.album.name);
        let mut file = File::create(&filename)?;

        let content = response.bytes()?;
        file.write_all(&content)?;

        println!("Song downloaded successfully. Saved as: {}", filename);
    } else {
        eprintln!(
            "Failed to download song. Status code: {}",
            response.status()
        );
    }

    Ok(())
}
