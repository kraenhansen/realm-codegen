use handlebars::*;
use serde::Serialize;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use webidl::ast::*;
use webidl::visitor::*;

use super::config;
use super::serializable_ast::*;
use super::template_helpers::*;

fn read_fragment(path: &PathBuf) -> Result<webidl::ast::AST, Box<dyn Error>> {
  let source = fs::read_to_string(path)?;
  match webidl::parse_string(&source) {
    Ok(v) => Ok(v),
    Err(e) => Err(Box::new(e)),
  }
}

fn apply_formatter(formatter: &Option<String>, source: String) -> Result<String, Box<dyn Error>> {
  if let Some(formatter) = &formatter {
    if let Some((command, arguments)) = formatter.split(" ").collect::<Vec<&str>>().split_first() {
      let mut child = Command::new(command)
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed running formatter");
      let mut stdin = child.stdin.take().expect("Failed to take stdin");
      std::thread::spawn(move || {
        stdin
          .write_all(source.as_bytes())
          .expect("Failed to write to stdin");
      });
      let output = child.wait_with_output()?;
      Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
      Ok(source)
    }
  } else {
    Ok(source)
  }
}

pub fn write_output<T>(
  config: &config::OutputConfig,
  output_path: &PathBuf,
  handlebars: &Handlebars,
  data: &T,
) -> Result<(), Box<dyn Error>>
where
  T: Serialize,
{
  // Determine the output path
  let path_string = config.path.to_str().expect("Expected a path");
  let rendered_filename = handlebars.render_template(path_string, &data)?;
  let absolute_path = output_path.join(rendered_filename);
  let result: String = handlebars.render(&config.template, &data)?;
  let formatted_result = apply_formatter(&config.formatter, result)?;
  println!(
    "Saving {}:\n{}",
    absolute_path.to_string_lossy(),
    formatted_result
  );
  fs::write(absolute_path, formatted_result)?;
  Ok(())
}

pub fn generate(config: config::ConfigResult, output_path: &PathBuf) -> Result<(), Box<dyn Error>> {
  // Create the output directory if it doesn't exist already
  if !output_path.exists() {
    fs::create_dir_all(&output_path)?;
  }
  let fragments = config
    .config
    .fragment_paths
    .iter()
    .map(|path| read_fragment(&path))
    .collect::<Result<Vec<webidl::ast::AST>, Box<dyn Error>>>()?;
  // Gather all interfaces
  let definitions = fragments.concat();
  // Print the definitions
  let mut visitor = PrettyPrintVisitor::new();
  visitor.visit(&definitions);
  print!("{}", visitor.get_output());
  // Gather all interfaces
  let interfaces = definitions
    .iter()
    .filter_map(|definition| match &definition {
      Definition::Interface(Interface::NonPartial(i)) => Some(i),
      _ => None,
    })
    .collect::<Vec<&NonPartialInterface>>();
  for output in &config.config.outputs {
    // Emit output files
    let mut handlebars = Handlebars::new();
    // Register some helpers
    register_helpers(&mut handlebars);
    // Register the root path as a template directory
    // TODO: I wonder if there's a simpler way to do this ...
    let absolute_template_root = if let Some(template_root) = &output.template_root {
      config.root_path.join(template_root)
    } else {
      config.root_path.clone()
    };
    handlebars.register_templates_directory(".hbs", absolute_template_root)?;
    match &output.per {
      Some(config::OutputIterator::Interface) => {
        for interface in &interfaces {
          write_output(
            &output,
            &output_path,
            &handlebars,
            &SerializableNonPartialInterface(interface),
          )?;
        }
      }
      None => {
        let mut data = BTreeMap::new();
        data.insert(
          "interfaces",
          interfaces
            .iter()
            .map(|interface| SerializableNonPartialInterface(interface))
            .collect::<Vec<SerializableNonPartialInterface>>(),
        );
        write_output(&output, &output_path, &handlebars, &data)?;
      }
    }
  }
  Ok(())
}
