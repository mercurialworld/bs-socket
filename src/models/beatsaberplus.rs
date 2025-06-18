use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(
    tag = "_type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
enum BSPlusMessage {
    Handshake { protocol_version: i32 },
    Event(Event),
}

#[derive(Debug, Deserialize)]
#[serde(
    tag = "_event",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
enum Event {
    GameState { game_state_changed: String },
    Resume { resume_time: f64 },
    Pause { pause_time: f64 },
    MapInfo { map_info_changed: Box<MapInfo> },
    Score { score_event: ScoreEvent },
}

#[derive(Debug, Deserialize, Default)]
pub struct MapInfo {
    /// The level's ID.
    pub level_id: String,
    /// The name of the song.
    pub name: String,
    /// The sub-name of the song.
    pub sub_name: String,
    /// The song's artist.
    pub artist: String,
    /// The map author.
    pub mapper: String,
    /// The difficulty characteristic (Standard, OneSaber, NoArrows, 360Degree, etc.)
    pub characteristic: String,
    /// The map's difficulty.
    pub difficulty: String,
    /// The song duration *in milliseconds*.
    pub duration: u64,
    /// The song's BPM.
    #[serde(rename = "BPM")]
    pub bpm: f64,
    /// How much *ScoreSaber* PP the map is worth.
    #[serde(rename = "PP")]
    pub pp: f64,
    /// The map's key on BeatSaver.
    #[serde(rename = "BSRKey")]
    pub bsr_key: String,
    /// The cover art of the map as a Base64 string.
    #[serde(rename = "coverRaw")]
    pub cover_raw: String,
    /// The current progress of the song *in seconds*.
    pub time: f64,
    /// Time multiplier.
    /// Usually 1.0, but can be different depending on speed modifiers/practice mode.
    pub time_multiplier: f64,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScoreEvent {
    /// The current progress of the song *in seconds*.
    pub time: f64,
    /// The current score.
    pub score: i64,
    /// The current accuracy.
    /// Is multiplied depending on modifiers.
    pub accuracy: f64,
    /// The current combo.
    pub combo: u64,
    /// The current amount of misses and bad cuts.
    pub miss_count: u64,
    /// The current amount of health.
    pub current_health: f64,
}
