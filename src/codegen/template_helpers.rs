use convert_case::{Case, Casing};
use handlebars::*;

fn camel_case_helper(
  h: &Helper,
  _: &Handlebars,
  _: &Context,
  _: &mut RenderContext,
  out: &mut dyn Output,
) -> HelperResult {
  let param = h.param(0).unwrap();
  out.write(param.value().render().to_case(Case::Camel).as_ref())?;
  Ok(())
}

pub fn register_helpers(handlebars: &mut Handlebars) {
  handlebars.register_helper("camelCased", Box::new(camel_case_helper));
}
