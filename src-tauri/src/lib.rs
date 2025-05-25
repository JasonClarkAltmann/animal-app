use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::sync::OnceLock;

const DOG_API_URL: &str = "https://api.thedogapi.com/v1/images/search";
const CAT_API_URL: &str = "https://api.thecatapi.com/v1/images/search";

static DOG_API_KEY: OnceLock<String> = OnceLock::new();
static CAT_API_KEY: OnceLock<String> = OnceLock::new();

static LIKED_DOG_DATA: OnceLock<Mutex<Vec<AnimalData>>> = OnceLock::new();
static LIKED_CAT_DATA: OnceLock<Mutex<Vec<AnimalData>>> = OnceLock::new();

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
struct AnimalData {
    url: String,
    #[serde(default)]
    breeds: Vec<Breed>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
struct Breed {
    name: String,
    #[serde(default)]
    weight: Weight,
    #[serde(default)]
    life_span: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
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

#[tauri::command]
fn like_dog(dog: AnimalData) -> Result<(), String> {
    let liked_data_mutex = LIKED_DOG_DATA.get_or_init(|| Mutex::new(Vec::new()));
    let mut liked_data = liked_data_mutex
        .lock()
        .map_err(|e| format!("Fehler beim Sperren des Mutex: {}", e))?;

    if !liked_data.contains(&dog) {
        liked_data.push(dog);
    }
    Ok(())
}

#[tauri::command]
fn unlike_dog(dog_url: String) -> Result<(), String> {
    let liked_data_mutex = LIKED_DOG_DATA.get_or_init(|| Mutex::new(Vec::new()));
    let mut liked_data = liked_data_mutex
        .lock()
        .map_err(|e| format!("Fehler beim Sperren des Mutex: {}", e))?;

    liked_data.retain(|c| c.url != dog_url);
    Ok(())
}

#[tauri::command]
fn get_liked_dog_data() -> Result<Vec<AnimalData>, String> {
    let liked_data_mutex = LIKED_DOG_DATA.get_or_init(|| Mutex::new(Vec::new()));
    let liked_data = liked_data_mutex
        .lock()
        .map_err(|e| format!("Fehler beim Sperren des Mutex: {}", e))?;
    Ok(liked_data.clone())
}

#[tauri::command]
fn like_cat(cat: AnimalData) -> Result<(), String> {
    let liked_data_mutex = LIKED_CAT_DATA.get_or_init(|| Mutex::new(Vec::new()));
    let mut liked_data = liked_data_mutex
        .lock()
        .map_err(|e| format!("Fehler beim Sperren des Mutex: {}", e))?;

    if !liked_data.contains(&cat) {
        liked_data.push(cat);
    }
    Ok(())
}

#[tauri::command]
fn unlike_cat(cat_url: String) -> Result<(), String> {
    let liked_data_mutex = LIKED_CAT_DATA.get_or_init(|| Mutex::new(Vec::new()));
    let mut liked_data = liked_data_mutex
        .lock()
        .map_err(|e| format!("Fehler beim Sperren des Mutex: {}", e))?;

    liked_data.retain(|c| c.url != cat_url);
    Ok(())
}

#[tauri::command]
fn get_liked_cat_data() -> Result<Vec<AnimalData>, String> {
    let liked_data_mutex = LIKED_CAT_DATA.get_or_init(|| Mutex::new(Vec::new()));
    let liked_data = liked_data_mutex
        .lock()
        .map_err(|e| format!("Fehler beim Sperren des Mutex: {}", e))?;
    Ok(liked_data.clone())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv().ok();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            fetch_dog_data,
            fetch_cat_data,
            like_dog,
            unlike_dog,
            get_liked_dog_data,
            like_cat,
            unlike_cat,
            get_liked_cat_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
