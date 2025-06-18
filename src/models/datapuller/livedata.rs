use serde::Deserialize;

use crate::models::common::Rank;

#[derive(Debug, Deserialize, Default)]
pub enum ColorType {
    #[default]
    None = -1,
    ColorA = 0,
    ColorB = 1,
}

#[derive(Debug, Deserialize, Default)]
pub enum NoteCutDirection {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
    UpLeft = 4,
    UpRight = 5,
    DownLeft = 6,
    DownRight = 7,
    Any = 8,
    #[default]
    None = 9,
}

#[derive(Debug, Deserialize, Default)]
pub enum LiveDataEventTrigger {
    #[default]
    Unknown = 0,
    TimerElapsed = 1,
    NoteMissed = 2,
    EnergyChange = 3,
    ScoreChange = 4,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BlockHitScore {
    /// The pre-swing score (0-70)
    pub pre_swing: u8,
    /// The post-swing score (0-30)
    pub post_swing: u8,
    /// How accurate the player was to hitting the center of the note (0-15)
    pub center_swing: u8,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct LiveData {
    //====SCORE====
    ///The current raw score.
    pub score: i64,

    ///The current score with the player selected multipliers applied.
    pub score_with_multipliers: i64,

    ///The maximum possible raw score for the current number of cut notes.
    pub max_score: i64,

    ///The maximum possible score with the player selected multipliers applied for the current number of cut notes.
    pub max_score_with_multipliers: i64,

    ///The string rank label for the current score.
    ///i.e. SS, S, A, B, etc.
    pub rank: Rank,

    /// Whether the player has a full combo at the current position in the song.
    pub full_combo: bool,

    ///The total number of notes spawned since the start position of the song until the current position in the song.
    pub notes_spawned: i64,

    ///The current note cut combo count without error.
    ///Resets back to 0 when the player: misses a note, hits a note incorrectly, takes damage or hits a bomb.
    pub combo: i64,

    ///The total number of missed and incorrectly hit notes since the start position of the song until the current position in the song.
    pub misses: i64,

    /// Player accuracy (between 0-100) at the current position in the song.
    pub accuracy: f64,

    ///The individual scores for the last hit note.
    pub block_hit_score: BlockHitScore,

    /// Player health (between 0-100) at the current position in the song.
    pub player_health: f64,

    ///The colour of note that was last hit.
    ///ColorType.None if no note was previously hit or a bomb was hit.
    pub color_type: ColorType,

    ///The note cut direction, also known as rotation.
    ///NoteCutDirection.None if no note was previously hit.
    pub cut_direction: NoteCutDirection,

    //====MISC====
    ///The total amount of time in seconds since the start of the map.
    pub time_elapsed: u64,

    ///The event that caused the update trigger to be fired.
    pub event_trigger: LiveDataEventTrigger,
}
