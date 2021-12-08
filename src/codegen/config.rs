use serde::Deserialize;
use serde_yaml;
use std::error::Error;
use std::fs::{canonicalize, File};
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum OutputIterator {
  Interface,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OutputConfig {
  pub per: Option<OutputIterator>,
  pub path: PathBuf,
  pub template_root: Option<PathBuf>,
  pub template: String,
  pub formatter: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
  #[serde(rename = "fragments")]
  pub fragment_paths: Vec<PathBuf>,
  pub outputs: Vec<OutputConfig>,
}

#[derive(Debug)]
pub struct ConfigResult {
  pub config: Config,
  pub root_path: PathBuf,
}

pub fn read_config(path: &PathBuf) -> Result<ConfigResult, Box<dyn Error>> {
  let canonical_path = canonicalize(&path)?;
  let file = File::open(&canonical_path)?;
  let reader = BufReader::new(file);
  let mut config: Config = serde_yaml::from_reader(reader)?;
  // Turn relative paths into absolute paths
  let root_path = canonical_path
    .parent()
    .ok_or("Failed to determine root path")?
    .to_owned();
  for path in &mut config.fragment_paths {
    *path = root_path.join(&path);
  }
  Ok(ConfigResult { config, root_path })
}
