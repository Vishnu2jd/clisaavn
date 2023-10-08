use clisaavn::{display_options, fetch_json, fetch_url, get_user_input};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let song_name = get_user_input();
    let song_data = fetch_json(&song_name)?;

    // Display options and get user selection
    let selected_option = display_options(&song_data);

    if let Some(option) = selected_option {
        // Fetch the URL with 160kbps quality based on user selection
        let url = fetch_url(&song_data, option)?;

        // Download the selected song, pass the song_data as an argument
        println!("{}", url);
    } else {
        println!("Exiting program. No valid option selected.");
    }

    Ok(())
}
