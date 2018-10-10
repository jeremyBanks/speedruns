// note that saves are non-atomic and can be corrutped if interrupted.

use core::{
    clone::Clone,
    fmt::{Debug, Display},
    ops::Drop,
};
use fs2::FileExt;
use log;
use serde::{
    de::{Deserialize, DeserializeOwned},
    ser::Serialize,
};
use serde_json;
use std::{
    fs::{File, OpenOptions},
    io::{Seek, SeekFrom},
};

pub struct Persistent<Data>
where
    Data: Serialize + DeserializeOwned + Clone + Default + Sized,
{
    file: File,
    data: Data,
    dirty: bool,
}

impl<Data> Persistent<Data>
where
    Data: Serialize + DeserializeOwned + Clone + Default + Sized,
{
    pub fn open(filename: &str) -> Self {
        let file;
        let data;

        let existing = OpenOptions::new().read(true).write(true).open(filename).and_then(|existing_file| {
            existing_file.try_lock_exclusive()?;
            let existing_data = serde_json::from_reader(&existing_file)?;
            Ok((existing_file, existing_data))
        });

        match existing {
            Ok(existing) => {
                file = existing.0;
                data = existing.1;
            }
            Err(error) => {
                info!("Failed to read an existing {:?}: {:?}", filename, error);
                file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(filename)
                    .unwrap();
                file.try_lock_shared().unwrap();
                data = Data::default();
            }
        }

        Self {
            file,
            data,
            dirty: false,
        }
    }

    pub fn sync(&mut self) {
        if self.dirty {
            info!("Syncing changes to disk.");
            self.file.try_lock_exclusive().unwrap();
            self.file.seek(SeekFrom::Start(0)).unwrap();
            self.file.set_len(0).unwrap();
            let mut ser = serde_json::Serializer::with_formatter(
                &self.file,
                serde_json::ser::PrettyFormatter::with_indent(b""),
            );
            self.data.serialize(&mut ser).unwrap();
            self.file.sync_all().unwrap();
            self.dirty = false;
        }
    }

    pub fn get(&self) -> &Data {
        &self.data
    }

    pub fn get_mut(&mut self) -> &mut Data {
        self.dirty = true;
        &mut self.data
    }
}

impl<Data> Drop for Persistent<Data>
where
    Data: Serialize + DeserializeOwned + Clone + Default,
{
    fn drop(&mut self) {
        self.sync();
        self.file.unlock().unwrap();
    }
}

mod speedruncom_api {
    pub mod runs {
        #[derive(Deserialize)]
        pub struct Response {
            pub data: Vec<Data>,
        }

        #[derive(Deserialize)]
        pub struct Data {
            pub id: String,
            pub weblink: String,
            pub game: String,
            pub level: Option<String>,
            pub category: Option<String>,
            pub players: Vec<Player>,
            pub date: Option<String>,
            pub submitted: Option<String>,
            pub times: Times,
        }

        #[derive(Deserialize)]
        #[serde(tag = "rel")]
        #[serde(rename_all = "kebab-case")]
        pub enum Player {
            User { id: String },
            Guest { name: String },
        }

        #[derive(Deserialize)]
        pub struct Times {
            pub primary_t: u32,
        }
    }
}
