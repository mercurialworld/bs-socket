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
pub enum SiraEventType {}
