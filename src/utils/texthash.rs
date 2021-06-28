use crate::helpers::Logger;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use strum::AsStaticRef;

#[derive(AsStaticStr)]
pub enum Language {
    CHS,
    CHT,
    DE,
    EN,
    FR,
    ID,
    JA,
    KO,
    PT,
    RU,
    TH,
    VI,
}

impl Default for Language {
    fn default() -> Language {
        Language::EN
    }
}

impl FromStr for Language {
    type Err = ();

    fn from_str(input: &str) -> Result<Language, Self::Err> {
        match input.to_ascii_lowercase().as_str() {
            "chs" => Ok(Language::CHS),
            "cht" => Ok(Language::CHT),
            "de" => Ok(Language::DE),
            "en" => Ok(Language::EN),
            "fr" => Ok(Language::FR),
            "id" => Ok(Language::ID),
            "ja" => Ok(Language::JA),
            "ko" => Ok(Language::KO),
            "pt" => Ok(Language::PT),
            "ru" => Ok(Language::RU),
            "th" => Ok(Language::TH),
            "vi" => Ok(Language::VI),
            _ => Err(()),
        }
    }
}

#[derive(Default)]
pub struct TextHash {
    language: Language,
    verbose: bool,
    textmap: HashMap<usize, String>,
}

impl TextHash {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn language(mut self, language: &str) -> Self {
        let logger = Logger::new("gdp:texthash");
        match Language::from_str(language) {
            Ok(language) => self.language = language,
            Err(_) => logger.error(format!(
                "Entity type not supported: {}, Defaulting to Weapon",
                language
            )),
        }
        self
    }

    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Prepares the texthash in memory
    pub fn prepare(mut self) -> Self {
        let logger = Logger::new("gdp:texthash");

        if self.verbose {
            logger.log("Preparing texthash map")
        }

        let temp_file = fs::read_to_string(format!(
            "./data/TextMap/TextMap{}.json",
            self.language.as_static()
        ))
        .expect("Could not read text map file");
        let temp: HashMap<usize, String> =
            serde_json::from_str(&temp_file).expect("Could not deserialize text map file");

        for (key, value) in temp.into_iter() {
            if !value.is_empty() {
                self.textmap.insert(key, value);
            }
        }

        logger.log(format!("Checking len: {}", self.textmap.len()));

        self
    }

    /// Matches given hash to text
    pub fn get_match(&self, hash: &usize) -> crate::Result<String> {
        let text = match self.textmap.get(hash) {
            Some(text) => text,
            None => "Unknown TextMap",
        };

        Ok(text.to_owned())
    }
}
