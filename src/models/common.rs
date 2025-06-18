// [TODO] add enums that are in core Beat Saber here
// ...which is Characteristic, Difficulty, and Rank
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub enum Rank {
    #[default]
    SSS,
    SS,
    A,
    B,
    C,
    D,
    E,
}
