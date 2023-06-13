use std::collections::HashMap;
use std::io::prelude::*;
use std::io::Result as IOResult;

pub struct GamePack {
    archiver: zip::ZipArchive<std::fs::File>,
    #[allow(dead_code)]
    filename: String,
    config: HashMap<String, String>,
}

impl GamePack {
    pub fn open(filename: &str) -> IOResult<Self> {
        let file = std::fs::File::open(filename)?;
        let mut arch = zip::ZipArchive::new(file)?;

        let mut config_data = String::new();
        arch.by_name("package.json")?
            .read_to_string(&mut config_data)?;
        let config = serde_json::from_str(&config_data)?;

        Ok(Self {
            archiver: arch,
            filename: filename.to_owned(),
            config,
        })
    }
    fn read(&mut self, name: &str) -> IOResult<Vec<u8>> {
        let size = self.archiver.by_name(name)?.size();
        let mut data = vec![0; size as usize];
        self.archiver.by_name(name)?.read_exact(&mut data)?;
        Ok(data)
    }
    pub fn get_config(&self, key: &str) -> Option<String> {
        if self.config.get(key).is_some() {
            Some(self.config.get(key).unwrap().to_owned())
        } else {
            None
        }
    }
    /** Read script in `scripts/` */
    pub fn get_script(&mut self, name: &str) -> IOResult<String> {
        let mut data = String::new();
        self.archiver
            .by_name(&format!("scripts/{name}"))?
            .read_to_string(&mut data)?;
        Ok(data)
    }
    /** Read script in `resources/` */
    pub fn get_resource(&mut self, name: &str) -> IOResult<Vec<u8>> {
        self.read(&format!("resources/{}", name))
    }
}
