use serde::ser::{Serialize, SerializeStruct, Serializer};
use webidl::ast::*;

pub struct SerializableConstructor<'a>(pub &'a Constructor);
pub struct SerializableOperation<'a>(pub &'a Operation);
pub struct SerializableArgument<'a>(pub &'a Argument);
pub struct SerializableTypeKind<'a>(pub &'a TypeKind);

pub struct SerializableNonPartialInterface<'a>(pub &'a NonPartialInterface);

pub struct SerializableNonPartialDictionary<'a>(pub &'a NonPartialDictionary);
pub struct SerializableDictionaryMember<'a>(pub &'a DictionaryMember);

// TODO: Return a custom "UnsupportedError" instead of panicing

impl Serialize for SerializableConstructor<'_> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut s = serializer.serialize_struct("Constructor", 1)?;
    let arguments = self
      .0
      .arguments
      .iter()
      .map(SerializableArgument)
      .collect::<Vec<SerializableArgument>>();
    s.serialize_field("arguments", &arguments)?;
    s.end()
  }
}

impl Serialize for SerializableOperation<'_> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut s = serializer.serialize_struct("Operation", 1)?;
    let name = match &self.0 {
      Operation::Regular(a) => a.name.as_ref(),
      Operation::Special(a) => a.name.as_ref(),
      Operation::Static(a) => a.name.as_ref(),
      _ => panic!("this part of webidl is not yet supported: {:?}", self.0),
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
        .map(SerializableArgument)
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
      TypeKind::ByteString => {
        s.serialize_field("string", &true)?;
      }
      TypeKind::Identifier(i) => {
        s.serialize_field("identifier", &true)?;
        s.serialize_field("name", &i)?;
      }
      _ => panic!("this part of webidl is not yet supported: {:?}", self.0),
    };
    s.end()
  }
}

impl Serialize for SerializableNonPartialInterface<'_> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut s = serializer.serialize_struct("NonPartialInterface", 2)?;
    s.serialize_field("name", &self.0.name)?;
    let members = &self.0.members;
    let operations = members
      .iter()
      .filter_map(|m| match m {
        InterfaceMember::Operation(o) => Some(SerializableOperation(o)),
        _ => None,
      })
      .collect::<Vec<SerializableOperation>>();
    s.serialize_field("operations", &operations)?;
    let constructor = members.iter().find_map(|m| match m {
      InterfaceMember::Constructor(c) => Some(SerializableConstructor(c)),
      _ => None,
    });
    s.serialize_field("constructor", &constructor)?;
    s.end()
  }
}

impl Serialize for SerializableNonPartialDictionary<'_> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut s = serializer.serialize_struct("NonPartialDictionary", 2)?;
    s.serialize_field("name", &self.0.name)?;
    let members = &self
      .0
      .members
      .iter()
      .map(SerializableDictionaryMember)
      .collect::<Vec<SerializableDictionaryMember>>();
    s.serialize_field("members", members)?;
    s.end()
  }
}

impl Serialize for SerializableDictionaryMember<'_> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut s = serializer.serialize_struct("DictionaryMember", 2)?;
    s.serialize_field("name", &self.0.name)?;
    s.serialize_field("required", &self.0.required)?;
    s.serialize_field("type", &SerializableTypeKind(&self.0.type_.kind))?;
    s.end()
  }
}
