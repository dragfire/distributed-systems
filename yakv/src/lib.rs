use anyhow;
use std::collections::{BTreeMap, HashMap};
use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

pub type Result<T> = anyhow::Result<T>;

/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are persisted to disk in log files. Log files are named after
/// monotonically increasing generation numbers with a `log` extension name.
/// A `BTreeMap` in memory stores the keys and the value locations for fast query.
///
/// ```rust
/// # use kvs::{KvStore, Result};
/// # fn try_main() -> Result<()> {
/// use std::env::current_dir;
/// let mut store = KvStore::open(current_dir()?)?;
/// store.set("key".to_owned(), "value".to_owned())?;
/// let val = store.get("key".to_owned())?;
/// assert_eq!(val, Some("value".to_owned()));
/// # Ok(())
/// # }
/// ```
pub struct KvStore {
    path: PathBuf,
    readers: HashMap<u64, BufReaderWithPos<File>>,
    index: BTreeMap<u64, CommandPos>,
    stale_data: u64,
}

impl KvStore {
    /// Opens a KvStore with the given path.
    pub fn open<T: Into<PathBuf>>(path: T) -> Result<Self> {
        // try to load all log files in the given path
        // if it failed then create a log file with an id suffix-ed to the file
        // e.g. key-1.log, key-2.log, key-3.log, etc
        // after loading all the logs, build the index in-memory
        let path = path.into();
        fs::create_dir_all(&path)?;

        let mut readers = HashMap::new();
        let mut index = BTreeMap::new();
        let mut stale_data = 0;

        let ids = sorted_ids(&path)?;
        for &id in &ids {
            let mut reader = BufReaderWithPos::new(File::open(log_path(&path, id))?)?;
            stale_data += load_log(id, &mut reader, &mut index)?;
            readers.insert(id, reader);
        }

        let cur_id = ids.last().unwrap_or(&0) + 1;

        Ok(KvStore {
            path,
            readers,
            index,
            stale_data,
        })
    }

    /// Sets the value of s string key to a string.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        unimplemented!();
    }

    /// Gets the string value for a given key.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        unimplemented!();
    }

    /// Removes the given key.
    pub fn remove(&mut self, key: String) -> Result<()> {
        unimplemented!();
    }
}

fn log_path<T: AsRef<Path>>(path: T, id: u64) -> PathBuf {
    path.as_ref().join(format!("{}.log", id))
}

// load a log and build index
fn load_log(
    id: u64,
    reader: &mut BufReaderWithPos<File>,
    index: &mut BTreeMap<u64, CommandPos>,
) -> Result<u64> {
    Ok(0)
}

// get all ids from the log files in a given path
//
// Returns sorted id numbers
fn sorted_ids(path: &Path) -> Result<Vec<u64>> {
    let mut ids: Vec<u64> = path
        .read_dir()?
        .flat_map(|dir_entry| -> Result<_> { Ok(dir_entry?.path()) })
        .filter(|path| path.is_file() && path.ends_with(".log"))
        .filter_map(|path| {
            path.file_name()
                .and_then(OsStr::to_str)
                .map(|s| s.trim_end_matches(".log"))
                .map(str::parse::<u64>)
        })
        .flatten()
        .collect();
    ids.sort();
    Ok(ids)
}

pub struct BufReaderWithPos<T: Read + Seek> {
    reader: BufReader<T>,
    pos: u64,
}

impl<T: Read + Seek> BufReaderWithPos<T> {
    fn new(mut file: T) -> Result<Self> {
        let pos = file.seek(SeekFrom::Current(0))?;
        Ok(BufReaderWithPos {
            reader: BufReader::new(file),
            pos,
        })
    }
}

impl<T: Read + Seek> Read for BufReaderWithPos<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = self.reader.read(buf)?;
        self.pos += len as u64;
        Ok(len)
    }
}

impl<T: Read + Seek> Seek for BufReaderWithPos<T> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.pos = self.reader.seek(pos)?;
        Ok(self.pos)
    }
}

struct BufWriterWithPos<T: Write + Seek> {
    writer: BufWriter<T>,
    pos: u64,
}

impl<T: Write + Seek> BufWriterWithPos<T> {
    fn new(mut file: T) -> Result<Self> {
        let pos = file.seek(SeekFrom::Current(0))?;
        Ok(BufWriterWithPos {
            writer: BufWriter::new(file),
            pos,
        })
    }
}

struct CommandPos {
    id: u64,
    pos: u64,
    len: u64,
}
