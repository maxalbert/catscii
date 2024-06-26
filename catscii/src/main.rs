use serde::Deserialize;


#[tokio::main]
async fn main() {
    let url = get_cat_image_url().await.unwrap();
    println!("The image is at {}", url);
}

async fn get_cat_image_url() -> color_eyre::Result<String> {
    let api_url = "https://api.thecatapi.com/v1/images/search";
    let res = reqwest::get(api_url).await?;
    if !res.status().is_success() {
        return Err(color_eyre::eyre::eyre!(
            "The Cat API returned HTTP {}",
            res.status()
        ));
    }

    #[derive(Deserialize)]
    struct CatImage {
        url: String,
    }
    let images: Vec<CatImage> = res.json().await?;
    // this syntax is new in Rust 1.65
    let Some(image) = images.into_iter().next() else {
        return Err(color_eyre::eyre::eyre!("The Cat API returned no images"));
    };

    Ok(image.url)
}
