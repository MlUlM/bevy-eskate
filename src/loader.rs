use auto_delegate::{delegate, Delegate};
use crate::error::GameResult;
use crate::loader::json::StageJson;
use crate::loader::native::NativeStageLoader;

pub mod json;
mod native;


#[delegate]
pub trait StageLoadable {
    fn load(&self) -> GameResult<Vec<StageJson>>;


    fn save(&self, json: &StageJson) -> GameResult;
}


#[derive(Delegate, Debug, Copy, Clone)]
#[to(StageLoadable)]
pub enum StageLoader {
    Native(NativeStageLoader)
}


impl StageLoader {
    pub fn new() -> Self {
        Self::Native(NativeStageLoader)
    }
}