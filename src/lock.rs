use crate::dependency::Dependency;
use crate::project_config::ProjectConfig;
use log::*;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct FileLock {
    pub(crate) package: String,
    pub(crate) source: Option<PathBuf>,
    pub(crate) destination: PathBuf,
    pub(crate) digest: String,
    pub(crate) require: bool,
}

pub(crate) struct ProjectLock {
    pub(crate) name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Lock {
    pub(crate) files: Vec<FileLock>,
}

#[derive(Debug)]
pub struct LockError {}

type LockResult = Result<Lock, LockError>;

impl Lock {
    pub fn from_config(config: &ProjectConfig) -> LockResult {
        trace!("Calculating lock");
        let mut files: Vec<FileLock> = vec![];

        let lock = Lock { files };

        match validate(&lock) {
            Ok(()) => Ok(lock),
            Err(error) => Err(error),
        }
    }
}

fn validate(lock: &Lock) -> Result<(), LockError> {
    Ok(())
}
