use std::fs;
use std::path::PathBuf;

use bevy_trait_query::imports::Component;

use crate::error::GameResult;
use crate::loader::json::StageJson;
use crate::loader::StageLoadable;

#[derive(Debug, Copy, Clone, Component, Hash, Eq, PartialEq)]
pub struct NativeStageLoader;


impl StageLoadable for NativeStageLoader {
    fn load(&self) -> GameResult<Vec<StageJson>> {
        let dir = eskate_dir_path()?;
        Ok(dir
            .read_dir()?
            .map(|dir| dir.unwrap())
            .filter(|entry| entry.path().extension().is_some_and(|extension| extension == "json"))
            .map(|entry| fs::read_to_string(entry.path()).unwrap())
            .map(|json| serde_json::from_str::<StageJson>(&json).unwrap())
            .collect())
    }


    fn save(&self, json: &StageJson) -> GameResult {
        let path = stage_path(&json.name)?;
        fs::write(path, serde_json::to_string(json)?)?;
        Ok(())
    }
}


fn eskate_dir_path() -> GameResult<PathBuf> {
    let base = directories::BaseDirs::new().unwrap();
    let dir = base.data_local_dir();
    let dir = dir.join("eskate");
    if fs::metadata(&dir).is_err() {
        fs::create_dir(&dir)?;
    }

    Ok(dir)
}


fn stage_path(name: &str) -> GameResult<PathBuf> {
    let file_name = format!("{}.stage.json", name);

    Ok(eskate_dir_path()?.join(file_name))
}


#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::loader::json::StageJson;
    use crate::loader::native::{eskate_dir_path, NativeStageLoader, stage_path};
    use crate::loader::StageLoadable;

    fn remove_all() {
        std::fs::read_dir(eskate_dir_path().unwrap())
            .unwrap()
            .for_each(|entry| {
                std::fs::remove_file(entry.unwrap().path()).unwrap();
            });
    }

    #[test]
    fn save() {
        remove_all();

        let json = StageJson {
            name: "stage1".to_string(),
            pages: vec![],
        };
        NativeStageLoader.save(&json).unwrap();

        let exists = File::open(stage_path("stage1").unwrap()).is_ok();
        assert!(exists)
    }


    #[test]
    fn load_once() {
        remove_all();

        let json = StageJson {
            name: "stage1".to_string(),
            pages: vec![],
        };
        NativeStageLoader.save(&json).unwrap();

        assert_eq!(NativeStageLoader.load().unwrap().len(), 1);
    }
}