use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TextToSpeechLanguage {
    English,
    Spanish,
    French,
    German,
    Italian,
    Japanese,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TextToSpeechOptions {
    pub language: TextToSpeechLanguage,
    pub voice: Option<String>,
    pub volume: Option<f32>,
}

impl Default for TextToSpeechOptions {
    fn default() -> Self {
        Self {
            language: TextToSpeechLanguage::English,
            voice: None,
            volume: Some(0.8),
        }
    }
}
