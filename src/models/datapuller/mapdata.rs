use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
#[serde(rename = "PascalCase")]
pub struct Modifiers {
    pub no_fail_on_0_energy: bool,
    pub one_life: bool,
    pub four_lives: bool,
    pub no_bombs: bool,
    pub no_walls: bool,
    pub no_arrows: bool,
    pub ghost_notes: bool,
    pub disappearing_arrows: bool,
    pub small_notes: bool,
    pub pro_mode: bool,
    pub strict_angles: bool,
    pub zen_mode: bool,
    pub slower_song: bool,
    pub faster_song: bool,
    pub super_fast_song: bool,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PracticeModeModifiers {
    pub song_speed_mul: f32,
    pub start_in_advance_and_clear_notes: bool,
    pub song_start_time: f32,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct RGBAColor {
    /// Hexadecimal RGB color code, including the # symbol
    #[serde(default = "default_hex_code")]
    pub hex_code: String,
    /// The red component (0 to 255)
    pub red: u8,
    /// The green component (0 to 255)
    pub green: u8,
    /// The blue component (0 to 255)
    pub blue: u8,
    /// The alpha component (0.0 to 1.0)
    pub alpha: f32,
}

fn default_hex_code() -> String {
    "#000000".to_string()
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ColorScheme {
    /// The color of the primary (typically left) saber, and by extension the notes.
    pub saber_a_color: Option<RGBAColor>,
    /// The color of the secondary (typically right) saber, and by extension the notes.
    pub saber_b_color: Option<RGBAColor>,
    /// The color of the walls.
    pub obstacles_color: Option<RGBAColor>,
    /// The primary environment color.
    pub environment_color0: Option<RGBAColor>,
    /// The secondary environment color.
    pub environment_color1: Option<RGBAColor>,
    /// The primary environment boost color, typically set to the same as the primary environment color.
    pub environment_color0_boost: Option<RGBAColor>,
    /// The secondary environment boost color, typically set to the same as the secondary environment color.
    pub environment_color1_boost: Option<RGBAColor>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct RankedState {
    /// Is map ranked on any leaderboards
    pub ranked: bool,
    /// Is map qualified on any leaderboards
    pub qualified: bool,
    /// Is map qualified on BeatLeader
    pub beatleader_qualified: bool,
    /// Is map qualified on ScoreSaber
    pub scoresaber_qualified: bool,
    /// Is map ranked on BeatLeader
    pub beatleader_ranked: bool,
    /// Is map ranked on ScoreSaber
    pub scoresaber_ranked: bool,
    /// BeatLeader stars.
    /// 0 if the value was undetermined.
    pub beatleader_stars: f64,
    /// ScoreSaber stars.
    /// 0 if the value was undetermined.
    pub scoresaber_stars: f64,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct MapData {
    //====LEVEL====
    /// Whether the level is paused.
    /// This can remain false even if level_failed is true, when modifiers.no_fail_on_0_energy is true.
    pub level_paused: bool,
    /// Whether the level was played to the end.
    pub level_finished: bool,
    /// Whether the level was failed out.
    pub level_failed: bool,
    /// Whether the level was quit by the player.
    pub level_quit: bool,

    //====MAP====
    /// The hash ID for the current map.
    /// None if the hash could not be determined (e.g. if the map is not a custom level).
    pub hash: Option<String>,

    /// The predefined ID for the current map.
    /// None if the map is not a built-in level.
    pub level_id: Option<String>,

    /// The name of the current map.
    pub song_name: String,

    /// The sub-name of the current map.
    pub song_sub_name: String,

    /// The author of the song.
    pub song_author: String,

    /// Comma-separated names of mappers and lighters of the current map (property name kept for backwards-compatibility).
    pub mapper: String,

    /// The mappers of the current map as a list of names.
    pub mappers: Vec<String>,

    /// The lighters of the current map as a list of names.
    pub lighters: Vec<String>,

    /// The content rating of the current map.
    pub content_rating: String,

    /// The BSR key of the current map.
    /// None if the BSR key could not be obtained.
    pub bsr_key: Option<String>,

    /// The cover image of the current map.
    /// None if the cover image could not be obtained.
    pub cover_image: Option<String>,

    /// The duration of the map in seconds.
    pub duration: i64,

    //====DIFFICULTY====
    /// The type of map.
    /// i.e. Standard, 360, OneSaber, etc.
    pub map_type: String,

    /// The standard difficulty label of the map.
    /// i.e. Easy, Normal, Hard, etc.
    pub difficulty: String,

    /// The custom difficulty label set by the mapper.
    /// None if there is none.
    pub custom_difficulty_label: Option<String>,

    /// The beats per minute of the current map.
    pub bpm: i64,

    /// The note jump speed of the current map.
    pub njs: f64,

    /// The modifiers selected by the player for the current level.
    /// i.e. No fail, No arrows, Ghost notes, etc.
    pub modifiers: Modifiers,

    /// The score multiplier set by the users selection of modifiers.
    pub modifiers_multiplier: f32,

    /// Whether the current map is in practice mode.
    pub practice_mode: bool,

    /// The modifiers selected by the user that are specific to practice mode.
    pub practice_mode_modifiers: PracticeModeModifiers,

    /// The approximate amount of performance points this map is worth (legacy value for backwards-compatibility)
    /// 0 if the map is unranked or the value was undetermined.
    pub pp: f64,

    /// ScoreSaber stars (legacy value for backwards-compatibility)
    /// 0 if the value was undetermined.
    pub star: f64,

    /// Ranked state for the current map.
    pub ranked_state: RankedState,

    /// Song rating percentage on BeatSaver (0-100)
    /// 0 if the value was undetermined.
    pub rating: f32,

    /// The color scheme for the currently playing map.
    pub color_scheme: ColorScheme,

    //====MISC====
    /// The version of Beat Saber running, e.g. 1.20.0
    pub game_version: String,

    /// The version of the DataPuller plugin, e.g. 2.1.0
    pub plugin_version: String,

    /// Whether the player is currently in a multiplayer lobby.
    pub is_multiplayer: bool,

    /// The previous local record set by the player for this map specific mode and difficulty.
    /// 0 if the map variant hasn't never been played before.
    pub previous_record: i64,

    /// The BSR key for the last played map.
    /// None if there was no previous map or the previous map's BSR key was undetermined.
    /// This value won't be updated if the current map is the same as the last.
    pub previous_bsr: Option<String>,
}
