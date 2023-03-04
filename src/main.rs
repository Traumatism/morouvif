use reqwest::get;
use serde_derive::Deserialize;
use serde_derive::Serialize;
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

    if fs::read_dir("data").is_ok() {
        print!("~ Removing data/ directory... ");
        fs::remove_dir_all("data")?;
        println!("OK");
    }

    if std::fs::read_dir("data").is_err() {
        println!("~ Updating datasets...");

        print!("    -- Creating data/ directory... ");
        fs::create_dir("data")?;
        println!("OK");

        print!("    -- Scraping files paths... ");
        let mut files_urls: Vec<(String, String)> = Vec::new();
        let mut url = Some("https://www.data.gouv.fr/api/2/datasets/5de8f397634f4164071119c5/resources/?page=1&type=main&page_size=6&q=".into());

        while url.is_some() {
            let response = get(url.unwrap())
                .await
                .unwrap()
                .json::<Response>()
                .await
                .unwrap();

            for data in response.data {
                let re = regex::Regex::new(r"^(deces\-[0-9]+\.txt)$").unwrap();
                let title = data.title;

                if !re.is_match(&title) {
                    continue;
                }

                files_urls.push((data.url, title))
            }

            let next_page = response.next_page;
            url = next_page
        }

        println!("OK ({} files)", files_urls.len());

        print!("    -- Fetching files... ");

        for (file_url, title) in files_urls {
            let response = get(file_url).await.unwrap();
            fs::write(format!("data/{title}"), response.text().await.unwrap()).unwrap();
        }
    }

    Ok(())
}
