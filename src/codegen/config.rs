use serde::Deserialize;
use serde_yaml;
use simple_error::{SimpleError, SimpleResult};
use std::error::Error;
use std::fs;
use std::fs::{canonicalize, File};
use std::io::BufReader;
use std::path::PathBuf;
use webidl::ast;

fn read_fragment(path: &PathBuf) -> Result<webidl::ast::AST, Box<dyn Error>> {
  let source = fs::read_to_string(path)?;
  match webidl::parse_string(&source) {
    Ok(v) => Ok(v),
    Err(e) => Err(Box::new(e)),
  }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum OutputIterator {
  Interface,
  Dictionary,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OutputConfig {
  pub per: Option<OutputIterator>,
  pub path: PathBuf,
  pub template: String,
  pub formatter: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TemplateRootConfig {
  pub outputs: Vec<OutputConfig>,
}

#[derive(Debug)]
pub struct EnrichedTemplateRootConfig {
  pub outputs: Vec<OutputConfig>,
  pub path: PathBuf,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
  #[serde(rename = "project")]
  pub project_name: String,
  #[serde(rename = "fragments")]
  pub fragment_paths: Vec<PathBuf>,
  #[serde(rename = "templates")]
  pub template_root_paths: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct EnrichedConfig {
  pub project_name: String,
  pub fragments: Vec<ast::AST>,
  pub root_path: PathBuf,
  pub template_roots: Vec<EnrichedTemplateRootConfig>,
}

fn read_root_config(path: &PathBuf) -> Result<Config, Box<dyn Error>> {
  let file = File::open(path)?;
  let reader = BufReader::new(file);
  Ok(serde_yaml::from_reader(reader)?)
}

fn read_template_root_config(path: &PathBuf) -> Result<TemplateRootConfig, Box<dyn Error>> {
  let config_path = resolve_config_file(path)?;
  let file = File::open(config_path)?;
  let reader = BufReader::new(file);
  Ok(serde_yaml::from_reader(reader)?)
}

fn resolve_config_file(path: &PathBuf) -> SimpleResult<PathBuf> {
  let canonical_path = canonicalize(path).or_else(|e| Err(SimpleError::from(e)))?;
  if path.is_file() {
    Ok(canonical_path.to_owned())
  } else if canonical_path.is_dir() {
    let file_path = canonical_path.join("config.yml");
    if file_path.is_file() {
      Ok(file_path)
    } else {
      Err(SimpleError::new("Failed to locate config.yml"))
    }
  } else {
    Err(SimpleError::new("Failed to locate config.yml"))
  }
}

pub fn read_config(path: &PathBuf) -> Result<EnrichedConfig, Box<dyn Error>> {
  let config_path = resolve_config_file(path)?;
  let config = read_root_config(&config_path)?;
  // Turn relative paths into absolute paths
  let root_path = config_path
    .parent()
    .ok_or("Failed to determine root path")?;
  Ok(EnrichedConfig {
    project_name: config.project_name,
    root_path: root_path.to_owned(),
    fragments: config
      .fragment_paths
      .iter()
      .map(|path| {
        let canonical_path = canonicalize(root_path.join(&path))?;
        read_fragment(&canonical_path)
      })
      .collect::<Result<Vec<webidl::ast::AST>, Box<dyn Error>>>()?,
    template_roots: config
      .template_root_paths
      .iter()
      .map(|path| {
        let canonical_path = canonicalize(root_path.join(&path))?;
        let template_root_config = read_template_root_config(&canonical_path)?;
        Ok(EnrichedTemplateRootConfig {
          path: PathBuf::from(path),
          outputs: template_root_config.outputs,
        })
      })
      .collect::<Result<Vec<EnrichedTemplateRootConfig>, Box<dyn Error>>>()?,
  })
}
