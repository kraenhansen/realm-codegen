use convert_case::*;
use handlebars::*;
use serde_json::Value;

use super::serializable_ast::*;

#[derive(Clone, Copy)]
struct ConvertCaseHelper(Case);

impl HelperDef for ConvertCaseHelper {
  fn call<'reg: 'rc, 'rc>(
    &self,
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
  ) -> HelperResult {
    let param = h.param(0).unwrap();
    out.write(param.value().render().to_case(self.0).as_ref())?;
    Ok(())
  }
}

fn get_argument_type_names<'a>(arguments: Option<&'a Value>) -> Vec<&'a str> {
  arguments
    .and_then(|v| v.as_array())
    .and_then(|arguments| {
      Some(
        arguments
          .iter()
          .filter_map(|argument| {
            argument
              .get("type")
              .and_then(|t| t.get("name").and_then(|name| name.as_str()))
          })
          .collect::<Vec<&str>>(),
      )
    })
    .unwrap_or(Vec::<&'a str>::new())
}

fn uses_helpers_internal(interface: &Value, dictionary: &Value) -> bool {
  let dictionary_name = dictionary
    .get("name")
    .and_then(|v| v.as_str())
    .unwrap_or("");
  if let Some(constructor) = interface.get("constructor") {
    if get_argument_type_names(constructor.get("arguments")).contains(&dictionary_name) {
      return true;
    }
  }
  if let Some(operations) = interface.get("operations") {
    let operations = operations.as_array().unwrap();
    for operation in operations {
      if get_argument_type_names(operation.get("arguments")).contains(&dictionary_name) {
        return true;
      }
    }
  }
  false
}

handlebars_helper!(uses_helper: |interface: Json, dictionary: Json| {
  uses_helpers_internal(interface, dictionary)
});

pub fn register_helpers(handlebars: &mut Handlebars) {
  for case in Case::all_cases() {
    let name = format!("{:?}", case).to_case(Case::Camel) + "Case";
    handlebars.register_helper(&name, Box::new(ConvertCaseHelper(case)));
  }
  handlebars.register_helper("uses", Box::new(uses_helper));
}
