use handlebars::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};
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
  println!("Saving {}", absolute_path.to_string_lossy(),);
  // Create the output directory if it doesn't exist already
  fs::create_dir_all(
    &absolute_path
      .parent()
      .expect("Failed to determine parent directory"),
  )?;
  fs::write(absolute_path, formatted_result)?;
  Ok(())
}

struct TemplateContext<'a> {
  project_name: String,
  interfaces: Vec<SerializableNonPartialInterface<'a>>,
  interface: Option<SerializableNonPartialInterface<'a>>,
}

impl Serialize for TemplateContext<'_> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut s = serializer.serialize_struct("TemplateContext", 1)?;
    s.serialize_field("projectName", &self.project_name)?;
    s.serialize_field("interfaces", &self.interfaces)?;
    s.serialize_field("interface", &self.interface)?;
    s.end()
  }
}

pub fn generate(
  config: config::EnrichedConfig,
  output_path: &PathBuf,
) -> Result<(), Box<dyn Error>> {
  // Create the output directory if it doesn't exist already
  if !output_path.exists() {
    fs::create_dir_all(&output_path)?;
  }
  // Gather all interfaces
  let definitions = config.fragments.concat();
  // Gather all interfaces
  let interfaces = definitions
    .iter()
    .filter_map(|definition| match &definition {
      Definition::Interface(Interface::NonPartial(i)) => Some(i),
      _ => None,
    })
    .collect::<Vec<&NonPartialInterface>>();
  // Print the definitions
  let mut visitor = PrettyPrintVisitor::new();
  visitor.visit(&definitions);
  print!("{}", visitor.get_output());
  let mut context = TemplateContext {
    project_name: config.project_name,
    interfaces: interfaces
      .iter()
      .map(|interface| SerializableNonPartialInterface(interface))
      .collect::<Vec<SerializableNonPartialInterface>>(),
    interface: None,
  };
  for template_root in config.template_roots {
    for output in template_root.outputs {
      // Emit output files
      let mut handlebars = Handlebars::new();
      // Register some helpers
      register_helpers(&mut handlebars);
      // Register the root path as a template directory
      // TODO: I wonder if there's a simpler way to do this ...
      let absolute_template_root = config.root_path.join(&template_root.path);
      handlebars.register_templates_directory(".hbs", absolute_template_root)?;
      match &output.per {
        Some(config::OutputIterator::Interface) => {
          for interface in &interfaces {
            context.interface = Some(SerializableNonPartialInterface(interface));
            write_output(&output, &output_path, &handlebars, &context)?;
          }
        }
        None => {
          write_output(&output, &output_path, &handlebars, &context)?;
        }
      }
    }
  }
  Ok(())
}
