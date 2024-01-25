use chrono::NaiveDate;

use crate::config::GameMasterConfig;

pub struct GameMaster {
    pub name: String,
}

impl From<GameMasterConfig> for GameMaster {
    fn from(source: GameMasterConfig) -> Self {
        Self { name: source.name }
    }
}

pub struct Game {
    pub party_name: String,
    pub date: NaiveDate,
    pub game_master: GameMaster,
}
