use rocksdb::{DB, Options};
use anyhow::{anyhow, Result};
use lumina_types::state::GlobalState;
use bincode;

pub struct Storage {
    pub db: DB,
}

impl Storage {
    pub fn new(path: &str) -> Result<Self> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path).map_err(|e| anyhow!("Failed to open DB: {}", e))?;
        Ok(Self { db })
    }

    pub fn save_state(&self, state: &GlobalState) -> Result<()> {
        let encoded: Vec<u8> = bincode::serialize(state).map_err(|e| anyhow!("Serialization error: {}", e))?;
        self.db.put(b"global_state", encoded).map_err(|e| anyhow!("DB write error: {}", e))?;
        Ok(())
    }

    pub fn load_state(&self) -> Result<GlobalState> {
        match self.db.get(b"global_state") {
            Ok(Some(value)) => {
                let decoded: GlobalState = bincode::deserialize(&value).map_err(|e| anyhow!("Deserialization error: {}", e))?;
                Ok(decoded)
            }
            Ok(None) => Ok(GlobalState::default()), // Return empty state if none exists
            Err(e) => Err(anyhow!("DB read error: {}", e)),
        }
    }
}
