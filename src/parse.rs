use crate::entities::weapons;
use crate::helpers::Logger;
use crate::utils::{output, readable::Readable, texthash::TextHash};
use std::str::FromStr;
use strum::AsStaticRef;

#[derive(AsStaticStr)]
pub enum EntityType {
    Weapon,
}

impl Default for EntityType {
    fn default() -> EntityType {
        EntityType::Weapon
    }
}

impl FromStr for EntityType {
    type Err = ();

    fn from_str(input: &str) -> Result<EntityType, Self::Err> {
        match input.to_ascii_lowercase().as_str() {
            "weapon" => Ok(EntityType::Weapon),
            _ => Err(()),
        }
    }
}

#[derive(Default)]
pub struct Parse {
    entity: EntityType,
    verbose: bool,
}

impl Parse {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn entity(mut self, entity: &str) -> Self {
        let logger = Logger::new("gdp:parse");
        match EntityType::from_str(entity) {
            Ok(entity) => self.entity = entity,
            Err(_) => logger.error(format!(
                "Entity type not supported: {}, Defaulting to Weapon",
                entity
            )),
        }
        self
    }

    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    pub fn run(self, texthash: TextHash, readable: Readable) -> crate::Result<()> {
        let logger = Logger::new("gdp:parse");

        if self.verbose {
            logger.log("Starting new parse")
        }

        logger.log(format!("Parsing: {}", self.entity.as_static()));
        let entity = match self.entity {
            EntityType::Weapon => weapons::parse(texthash, readable),
        };

        logger.log(format!("Saving: {}", self.entity.as_static()));

        match entity {
            Ok(entity) => {
                output::save(entity, self.entity.as_static());
            }
            Err(_) => {
                logger.error("Entity is not serializable");
            }
        };

        Ok(())
    }
}
