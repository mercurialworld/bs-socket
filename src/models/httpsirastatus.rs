use serde::Deserialize;

// [TODO] implement all of this: https://github.com/denpadokei/HttpSiraStatus/blob/master/protocol.md

#[derive(Debug, Deserialize)]
pub struct Event {
    /// UNIX timestamp in milliseconds of the moment this event happened.
    time: u64,
    /// The event in-game that fired off this event message.
    #[serde(flatten)]
    event: SiraEventType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "event", content = "status")]
pub enum SiraEventType {
    Hello,
    SongStart,
    Finished,
    SoftFailed,
    Failed,
    Menu,
    Pause,
    Resume,
    NoteSpawned,
    NoteCut,
    NoteFullyCut,
    NoteMissed,
    BombCut,
    BombMissed,
    ObstacleEnter,
    ObstacleExit,
    ScoreChanged,
    BeatmapEvent,
}
