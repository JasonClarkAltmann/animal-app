use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct DogData {
    url: String,
    #[serde(default)]
    breeds: Vec<DogBreed>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DogBreed {
    name: String,
    breed_group: String,
    life_span: String,
}

#[tauri::command]
async fn fetch_dog_images() -> Result<Vec<DogData>, String> {
    dotenv().ok();

    let api_key =
        std::env::var("DOG_API_KEY").map_err(|_| "DOG_API_KEY nicht gefunden".to_string())?;

    let client = reqwest::Client::new();
    let response = client
        .get("https://api.thedogapi.com/v1/images/search")
        .header("x-api-key", api_key)
        .query(&[("size", "med"), ("has_breeds", "true")])
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status().is_success() {
                let dog_images: Vec<DogData> = res
                    .json()
                    .await
                    .map_err(|e| format!("Fehler beim Parsen der JSON-Antwort: {}", e))?;
                Ok(dog_images)
            } else {
                Err(format!(
                    "API-Anfrage fehlgeschlagen mit Status: {}",
                    res.status()
                ))
            }
        }
        Err(e) => Err(format!("Fehler beim Senden der API-Anfrage: {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![fetch_dog_images])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
