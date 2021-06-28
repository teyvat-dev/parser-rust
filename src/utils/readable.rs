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
pub struct Readable {
    language: Language,
    verbose: bool,
    readable: HashMap<String, String>,
}

impl Readable {
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

    /// Prepares the readables in memory
    pub fn prepare(mut self) -> Self {
        let logger = Logger::new("gdp:readables");

        if self.verbose {
            logger.log("Preparing readable map")
        }

        let language = self.language.as_static();

        let readables = fs::read_dir(format!("./data/Readable/{}", language)).unwrap();
        for readable in readables {
            let readable = readable.unwrap();
            let path = readable.path();
            let str_path = path.to_str().unwrap();
            if str_path.contains("Book")
                || str_path.contains("Relic")
                || str_path.contains("Weapon")
                || str_path.contains("Wings")
            {
                let key = path
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .split("_")
                    .collect::<Vec<&str>>()[0];

                // logger.log(format!("{}", key.to_str().unwrap()));
                // .to_str()
                // .unwrap();
                //.replace(format!("_{}", language), "");
                let value = fs::read_to_string(str_path);
                match value {
                    Ok(v) => {
                        self.readable.insert(key.to_owned(), v);
                    }
                    Err(_) => {}
                }
            }
        }

        logger.log(format!("Checking len: {}", self.readable.len()));

        self
    }

    /// Matches given hash to text
    pub fn get_match(&self, key: &str) -> crate::Result<String> {
        let text = match self.readable.get(key) {
            Some(text) => text,
            None => "Unknown Readable",
        };

        Ok(text.to_owned())
    }
}
