use convert_case::*;
use handlebars::*;

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

pub fn register_helpers(handlebars: &mut Handlebars) {
  for case in Case::all_cases() {
    let name = format!("{:?}", case).to_case(Case::Camel) + "Case";
    handlebars.register_helper(&name, Box::new(ConvertCaseHelper(case)));
  }
}
