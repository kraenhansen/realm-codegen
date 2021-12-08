use serde::ser::{Serialize, SerializeStruct, Serializer};
use webidl::ast::*;

pub struct SerializableNonPartialInterface<'a>(pub &'a NonPartialInterface);
pub struct SerializableInterfaceMember<'a>(pub &'a InterfaceMember);
pub struct SerializableOperation<'a>(pub &'a Operation);
pub struct SerializableArgument<'a>(pub &'a Argument);
pub struct SerializableTypeKind<'a>(pub &'a TypeKind);

impl Serialize for SerializableNonPartialInterface<'_> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut s = serializer.serialize_struct("NonPartialInterface", 2)?;
    s.serialize_field("name", &self.0.name)?;
    let members = &self
      .0
      .members
      .iter()
      .map(|member| SerializableInterfaceMember(member))
      .collect::<Vec<SerializableInterfaceMember>>();
    s.serialize_field("members", members)?;
    s.end()
  }
}

impl Serialize for SerializableInterfaceMember<'_> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match &self.0 {
      InterfaceMember::Operation(o) => SerializableOperation(o).serialize(serializer),
      _ => serializer.serialize_struct("InterfaceMember", 0)?.end(),
    }
  }
}

impl Serialize for SerializableOperation<'_> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut s = serializer.serialize_struct("Operation", 1)?;
    s.serialize_field("operation", &true)?;
    let name = match &self.0 {
      Operation::Regular(a) => a.name.as_ref(),
      Operation::Special(a) => a.name.as_ref(),
      Operation::Static(a) => a.name.as_ref(),
      _ => None,
    };
    if let Some(n) = name {
      s.serialize_field("name", n)?;
    }
    let arguments = match &self.0 {
      Operation::Regular(a) => Some(&a.arguments),
      Operation::Special(a) => Some(&a.arguments),
      Operation::Static(a) => Some(&a.arguments),
      _ => None,
    };
    if let Some(a) = arguments {
      let arguments = a
        .iter()
        .map(|argument| SerializableArgument(argument))
        .collect::<Vec<SerializableArgument>>();
      s.serialize_field("arguments", &arguments)?;
    }
    let return_type = match &self.0 {
      Operation::Regular(a) => Some(&a.return_type),
      Operation::Special(a) => Some(&a.return_type),
      Operation::Static(a) => Some(&a.return_type),
      _ => None,
    };
    if let Some(ReturnType::NonVoid(t)) = return_type {
      s.serialize_field("returnType", &SerializableTypeKind(&t.kind))?;
    }
    s.end()
  }
}

impl Serialize for SerializableArgument<'_> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut s = serializer.serialize_struct("Argument", 1)?;
    s.serialize_field("name", &self.0.name)?;
    s.serialize_field("type", &SerializableTypeKind(&self.0.type_.kind))?;
    s.end()
  }
}

impl Serialize for SerializableTypeKind<'_> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut s = serializer.serialize_struct("TypeKind", 1)?;
    match &self.0 {
      TypeKind::RestrictedDouble => {
        s.serialize_field("double", &true)?;
      }
      TypeKind::UnrestrictedDouble => {
        s.serialize_field("double", &true)?;
        s.serialize_field("unrestricted", &true)?;
      }
      TypeKind::Identifier(i) => {
        s.serialize_field("identifier", &true)?;
        s.serialize_field("name", &i)?;
      }
      _ => {
        s.serialize_field("unsupported", &true)?;
      }
    };
    s.end()
  }
}
