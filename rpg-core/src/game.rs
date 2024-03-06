use chrono::NaiveDate;
use crate::config::GameMasterConfig;

/// Game master metadata.
pub struct GameMaster {
    /// Game master display name.
    pub name: String,
}

impl From<GameMasterConfig> for GameMaster {
    fn from(source: GameMasterConfig) -> Self {
        Self { name: source.name }
    }
}

/// Game session metadata.
pub struct Game {
    /// Party display name.
    pub party_name: String,
    /// Game session date.
    pub date: NaiveDate,
    /// Game master of this session.
    pub game_master: GameMaster,
}
