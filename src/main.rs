use reqwest::get;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use tokio::io::Result;

#[derive(Serialize, Deserialize, Debug)]
struct DatasetData {
    title: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    data: Vec<DatasetData>,
    previous_page: Option<String>,
    next_page: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!(
        r#"
        _ __  ___ _ _ ___ _  ___ _(_)/ _|
       | '  \/ ~ \ '_/ ~ \ || \ V / |  _|
       |_|_|_\___/_| \___/\_,_|\_/|_|_|

       github.com/traumatism

        "#
    );

    let data_dir = "data";

    if let Ok(_) = fs::read_dir(data_dir) {
        println!("~ Removing {} directory...", data_dir);
        fs::remove_dir_all(data_dir)?;
    }

    println!("~ Updating datasets...");

    println!("    -- Creating {} directory...", data_dir);
    fs::create_dir(data_dir)?;

    let mut files_urls = Vec::new();
    let mut url = Some("https://www.data.gouv.fr/api/2/datasets/5de8f397634f4164071119c5/resources/?page=1&type=main&page_size=6&q=".into());

    println!("    -- Scraping files paths...");
    while let Some(next_url) = url {
        let response = get(next_url)
            .await
            .unwrap()
            .json::<Response>()
            .await
            .unwrap();

        files_urls.extend(response.data.into_iter().filter_map(|data| {
            let re = regex::Regex::new(r"^(deces\-[0-9]+\.txt)$").unwrap();
            let title = data.title;

            if !re.is_match(&title) {
                None
            } else {
                Some((data.url, title))
            }
        }));

        url = response.next_page;
    }

    println!("    -- Fetching files...");
    for (file_url, title) in files_urls {
        let response = get(file_url).await.unwrap();
        fs::write(
            format!("{}/{}", data_dir, title),
            response.text().await.unwrap(),
        )
        .unwrap();
    }

    Ok(())
}
