use serde::{Deserialize, Serialize};
use crate::lib;

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    max_price_to_search: String,
    min_quantity_to_search: String,
    price_to_offer: String,
    item_names: String,
}

impl Settings {
    // Getters
    pub fn max_price_to_search(&self) -> &str {
        &self.max_price_to_search
    }

    pub fn min_quantity_to_search(&self) -> &str {
        &self.min_quantity_to_search
    }

    pub fn price_to_offer(&self) -> &str {
        &self.price_to_offer
    }

    pub fn item_names(&self) -> &str {
        &self.item_names
    }

    // Setters
    pub fn set_max_price_to_search(&mut self, value: String) {
        self.max_price_to_search = value;
    }

    pub fn set_min_quantity_to_search(&mut self, value: String) {
        self.min_quantity_to_search = value;
    }

    pub fn set_price_to_offer(&mut self, value: String) {
        self.price_to_offer = value;
    }

    pub fn set_item_names(&mut self, value: String) {
        self.item_names = value;
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            max_price_to_search: lib::MAX_PRICE_TO_SEARCH.to_string(),
            min_quantity_to_search: lib::MIN_QUANTITY_TO_SEARCH.to_string(),
            price_to_offer: lib::PRICE_TO_OFFER.to_string(),
            item_names: lib::PROFITABLE_ITEM_NAMES.join("\n").to_string(),
        }
    }
}

// TODO: save presets as hashmap instead of array
#[derive(Clone, Serialize, Deserialize)]
pub struct Preset {
    pub(crate) name: String,
    settings: Settings,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SettingsManager {
    current_settings: Settings,
    pub(crate) presets: Vec<Preset>,
    pub(crate) current_preset_name: Option<String>,
}

impl SettingsManager {
    const STORAGE_KEY: &'static str = "app_settings";

    #[cfg(target_arch = "wasm32")]
    pub fn load() -> Self {
        use wasm_bindgen::JsCast;
        let window = web_sys::window().expect("no window");
        let local_storage = window
            .local_storage()
            .expect("no local storage")
            .expect("local storage undefined");

        if let Ok(Some(data)) = local_storage.get_item(Self::STORAGE_KEY) {
            if let Ok(settings) = serde_json::from_str(&data) {
                return settings;
            }
        }
        Self::default()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load() -> Self {
        const SETTINGS_FILE: &str = "settings.json";
        if let Ok(file) = std::fs::File::open(SETTINGS_FILE) {
            if let Ok(settings) = serde_json::from_reader(file) {
                return settings;
            }
        }
        Self::default()
    }

    #[cfg(target_arch = "wasm32")]
    pub fn save(&self) {
        use wasm_bindgen::JsCast;
        let window = web_sys::window().expect("no window");
        let local_storage = window
            .local_storage()
            .expect("no local storage")
            .expect("local storage undefined");

        if let Ok(data) = serde_json::to_string(self) {
            let _ = local_storage.set_item(Self::STORAGE_KEY, &data);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn save(&self) {
        const SETTINGS_FILE: &str = "settings.json";
        if let Ok(file) = std::fs::File::create(SETTINGS_FILE) {
            let _ = serde_json::to_writer_pretty(file, self);
        }
    }

    pub fn save_as_preset(&mut self, name: String) {
        let preset = Preset {
            name: name.clone(),
            settings: self.current_settings.clone(),
        };

        // Remove existing preset with the same name if it exists
        self.presets.retain(|p| p.name != name);
        self.presets.push(preset);
        self.current_preset_name = Some(name);
        self.save();
    }

    pub fn load_preset(&mut self, name: &str) -> bool {
        if let Some(preset) = self.presets.iter().find(|p| p.name == name) {
            self.current_settings = preset.settings.clone();
            self.current_preset_name = Some(name.to_string());
            true
        } else {
            false
        }
    }

    pub fn delete_preset(&mut self, name: &str) {
        self.presets.retain(|p| p.name != name);
        if self.current_preset_name.as_deref() == Some(name) {
            self.current_preset_name = None;
        }
        self.save();
    }

    pub fn reset_settings(&mut self) {
        self.current_settings = Settings::default();
        // Preserve presets and current_preset_name
    }

    pub fn delete_all_presets(&mut self) {
        self.presets.clear();
        self.current_preset_name = None;
        self.save();
    }

    pub fn get_current_settings(&self) -> &Settings {
        &self.current_settings
    }

    pub fn get_current_settings_mut(&mut self) -> &mut Settings {
        &mut self.current_settings
    }

    pub fn get_presets(&self) -> &[Preset] {
        &self.presets
    }

    pub fn get_current_preset_name(&self) -> Option<&str> {
        self.current_preset_name.as_deref()
    }
}

impl Default for SettingsManager {
    fn default() -> Self {
        Self {
            current_settings: Settings::default(),
            presets: Vec::new(),
            current_preset_name: None,
        }
    }
}