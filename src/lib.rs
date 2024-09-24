use std::io::BufReader;
use std::path::Path;
use zip::read::ZipArchive;

pub mod bossdata;
pub mod evtc;

pub fn open(path: impl AsRef<Path>) -> anyhow::Result<evtc::Encounter> {
    let file = std::fs::File::open(&path)?;
    let reader = BufReader::new(file);
    let mut zip = ZipArchive::new(reader)?;
    if zip.len() == 0 {
        anyhow::bail!("Empty zip file");
    }
    let z = zip.by_index(0)?;
    let mut file = BufReader::new(z);
    Ok(evtc::read_encounter(&mut file)?)
}
