use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

const DOG_API_URL: &str = "https://api.thedogapi.com/v1/images/search";
const CAT_API_URL: &str = "https://api.thecatapi.com/v1/images/search";

static DOG_API_KEY: OnceLock<String> = OnceLock::new();
static CAT_API_KEY: OnceLock<String> = OnceLock::new();

#[derive(Debug, Serialize, Deserialize)]
struct AnimalData {
    url: String,
    #[serde(default)]
    breeds: Vec<Breed>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Breed {
    name: String,
    #[serde(default)]
    weight: Weight,
    #[serde(default)]
    life_span: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Weight {
    #[serde(default)]
    imperial: String,
    #[serde(default)]
    metric: String,
}

async fn fetch_base<T>(url: &str, api_key: &str) -> Result<Vec<T>, String>
where
    T: serde::de::DeserializeOwned,
{
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("x-api-key", api_key)
        .query(&[("size", "med"), ("has_breeds", "true")])
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status().is_success() {
                let data: Vec<T> = res
                    .json()
                    .await
                    .map_err(|e| format!("Fehler beim Parsen der JSON-Antwort: {}", e))?;
                Ok(data)
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

#[tauri::command]
async fn fetch_dog_data() -> Result<Vec<AnimalData>, String> {
    let dog_api_key = DOG_API_KEY.get_or_init(|| {
        std::env::var("DOG_API_KEY").expect(
            "DOG_API_KEY muss in der .env-Datei gesetzt sein oder als Umgebungsvariable existieren",
        )
    });

    fetch_base::<AnimalData>(DOG_API_URL, dog_api_key).await
}

#[tauri::command]
async fn fetch_cat_data() -> Result<Vec<AnimalData>, String> {
    let cat_api_key = CAT_API_KEY.get_or_init(|| {
        std::env::var("CAT_API_KEY").expect(
            "CAT_API_KEY muss in der .env-Datei gesetzt sein oder als Umgebungsvariable existieren",
        )
    });

    fetch_base::<AnimalData>(CAT_API_URL, cat_api_key).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv().ok();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![fetch_dog_data, fetch_cat_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
