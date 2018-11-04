use std::io::Write;
use std::fmt::Display;

use serde::ser::{self, Serialize};

use error::{Error, ErrorKind, Result};
use self::var::{Map, Struct};
use self::seq::Seq;
use self::tuples::Tuple;

mod var;
mod seq;
mod helpers;
mod tuples;


/// A convenience method for serializing some object to a buffer.
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate serde_derive;
/// # extern crate serde;
/// # extern crate serde_xml_rs;
/// # use serde_xml_rs::to_writer;
/// #[derive(Serialize)]
/// struct Person {
///   name: String,
///   age: u32,
/// }
///
/// # fn main() {
/// let mut buffer = Vec::new();
/// let joe = Person {name: "Joe".to_string(), age: 42};
///
/// to_writer(&mut buffer, &joe).unwrap();
///
/// let serialized = String::from_utf8(buffer).unwrap();
/// println!("{}", serialized);
/// # }
/// ```
pub fn to_writer<W: Write, S: Serialize>(writer: W, value: &S) -> Result<()> {
    let mut ser = Serializer::new(writer);
    value.serialize(&mut ser)
}


/// A convenience method for serializing some object to a string.
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate serde_derive;
/// # extern crate serde;
/// # extern crate serde_xml_rs;
/// # use serde_xml_rs::to_string;
/// #[derive(Serialize)]
/// struct Person {
///   name: String,
///   age: u32,
/// }
///
/// # fn main() {
///
/// let joe = Person {name: "Joe".to_string(), age: 42};
/// let serialized = to_string(&joe).unwrap();
/// println!("{}", serialized);
/// # }
/// ```
pub fn to_string<S: Serialize>(value: &S) -> Result<String> {
    // Create a buffer and serialize our nodes into it
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value)?;

    // We then check that the serialized string is the same as what we expect
    let string = String::from_utf8(writer)?;
    Ok(string)
}

/// An XML `Serializer`.
pub struct Serializer<W>
where
    W: Write,
{
    writer: W,
}

impl<W> Serializer<W>
where
    W: Write,
{
    pub fn new(writer: W) -> Self {
        Self { writer: writer }
    }

    fn write_primitive<P: Display>(&mut self, primitive: P) -> Result<()> {
        write!(self.writer, "{}", primitive)?;
        Ok(())
    }

    fn write_wrapped<S: Serialize>(&mut self, tag: &str, value: S) -> Result<()> {
        write!(self.writer, "<{}>", tag)?;
        value.serialize(&mut *self)?;
        write!(self.writer, "</{}>", tag)?;
        Ok(())
    }
}


#[allow(unused_variables)]
impl<'w, W> ser::Serializer for &'w mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Seq<'w, W>;
    type SerializeTuple = Tuple<'w, W>;
    type SerializeTupleStruct = Tuple<'w, W>;
    type SerializeTupleVariant = Tuple<'w, W>;
    type SerializeMap = Map<'w, W>;
    type SerializeStruct = Struct<'w, W>;
    type SerializeStructVariant = Struct<'w, W>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        if v {
            write!(self.writer, "true")?;
        } else {
            write!(self.writer, "false")?;
        }

        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.write_primitive(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.write_primitive(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        self.write_primitive(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        self.write_primitive(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.write_primitive(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        self.write_primitive(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        self.write_primitive(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        self.write_primitive(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        self.write_primitive(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        self.write_primitive(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        self.write_primitive(v)
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok> {
        self.write_primitive(value)
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok> {
        // TODO: I imagine you'd want to use base64 here.
        // Not sure how to roundtrip effectively though...
        Err(
            ErrorKind::UnsupportedOperation("serialize_bytes".to_string()).into(),
        )
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        self.serialize_none()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        self.write_wrapped(name, ())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.write_primitive(variant)
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok> {
        self.write_wrapped(name, value)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok> {
        self.write_wrapped(variant, value)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(Seq::new(self))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        Ok(Tuple::new(self))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        write!(self.writer, "<{}>", name)?;
        Ok(Tuple::new_with_name(self, name))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        write!(self.writer, "<{}>", name)?;
        Ok(Tuple::new_with_name(self, name))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(Map::new(self))
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        write!(self.writer, "<{}>", name)?;
        Ok(Struct::new(self, name))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        write!(self.writer, "<{}>", variant)?;
        Ok(Struct::new(self, variant))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serializer as SerSerializer;
    use serde::ser::{SerializeMap, SerializeStruct};
    use std::collections::BTreeMap;

    #[test]
    fn test_serialize_bool() {
        let inputs = vec![(true, "true"), (false, "false")];

        for (src, should_be) in inputs {
            let mut buffer = Vec::new();

            {
                let mut ser = Serializer::new(&mut buffer);
                ser.serialize_bool(src).unwrap();
            }

            let got = String::from_utf8(buffer).unwrap();
            assert_eq!(got, should_be);
        }
    }

    #[test]
    fn test_start_serialize_struct() {
        let mut buffer = Vec::new();

        {
            let mut ser = Serializer::new(&mut buffer);
            let _ = ser.serialize_struct("foo", 0).unwrap();
        }

        let got = String::from_utf8(buffer).unwrap();
        assert_eq!(got, "<foo>");
    }

    #[test]
    fn test_serialize_struct_field() {
        let mut buffer = Vec::new();

        {
            let mut ser = Serializer::new(&mut buffer);
            let mut struct_ser = Struct::new(&mut ser, "baz");
            struct_ser.serialize_field("foo", "bar").unwrap();
        }

        let got = String::from_utf8(buffer).unwrap();
        assert_eq!(got, "<foo>bar</foo>");
    }

    #[test]
    fn test_serialize_struct() {
        #[derive(Serialize)]
        struct Person {
            name: String,
            age: u32,
        }

        let bob = Person {
            name: "Bob".to_string(),
            age: 42,
        };
        let should_be = "<Person><name>Bob</name><age>42</age></Person>";
        let mut buffer = Vec::new();

        {
            let mut ser = Serializer::new(&mut buffer);
            bob.serialize(&mut ser).unwrap();
        }

        let got = String::from_utf8(buffer).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn test_serialize_map_entries() {
        let should_be = "<name>Bob</name><age>5</age>";
        let mut buffer = Vec::new();

        {
            let mut ser = Serializer::new(&mut buffer);
            let mut map = Map::new(&mut ser);
            map.serialize_entry("name", "Bob").unwrap();
            map.serialize_entry("age", "5").unwrap();
        }

        let got = String::from_utf8(buffer).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn test_serialize_enum() {
        #[derive(Serialize)]
        #[allow(dead_code)]
        enum Node {
            Boolean(bool),
            Number(f64),
            String(String),
        }

        let mut buffer = Vec::new();
        let should_be = "<Boolean>true</Boolean>";

        {
            let mut ser = Serializer::new(&mut buffer);
            let node = Node::Boolean(true);
            node.serialize(&mut ser).unwrap();
        }

        let got = String::from_utf8(buffer).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn serialize_a_list() {
        #[derive(Serialize)]
        struct Foo;

        let inputs = vec![Foo, Foo, Foo];
        let should_be = "<Foo></Foo><Foo></Foo><Foo></Foo>";

        let got = to_string(&inputs).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn basic_newtype_struct() {
        #[derive(Serialize)]
        struct Foo(u32);

        let f = Foo(5);
        let should_be = "<Foo>5</Foo>";

        let mut buffer = Vec::new();
        f.serialize(&mut Serializer::new(&mut buffer)).unwrap();

        let got = String::from_utf8(buffer).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn option_as_a_newtype_struct() {
        #[derive(Serialize)]
        struct Foo(Option<u32>);

        let inputs = vec![(Foo(Some(5)), "<Foo>5</Foo>"), (Foo(None), "<Foo></Foo>")];

        for (input, should_be) in inputs {
            let mut buffer = Vec::new();
            input.serialize(&mut Serializer::new(&mut buffer)).unwrap();

            let got = String::from_utf8(buffer).unwrap();
            assert_eq!(got, should_be);
        }
    }

    #[test]
    fn newtype_wrapper_around_a_map() {
        #[derive(Serialize)]
        struct Foo(BTreeMap<String, u32>);

        let pairs = vec![(String::from("a"), 5), (String::from("hello"), 42)];
        let map = Foo(pairs.into_iter().collect());

        let should_be = "<Foo><a>5</a><hello>42</hello></Foo>";
        let mut buffer = Vec::new();

        map.serialize(&mut Serializer::new(&mut buffer)).unwrap();

        let got = String::from_utf8(buffer).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn newtype_around_unit() {
        #[derive(Serialize)]
        struct Foo(());

        let should_be = "<Foo></Foo>";
        let f = Foo(());

        let mut buffer = Vec::new();
        f.serialize(&mut Serializer::new(&mut buffer)).unwrap();

        let got = String::from_utf8(buffer).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn serializing_a_list_of_primitives_is_an_error() {
        let dodgy = vec![1, 2, 3, 4, 5];
        assert!(to_string(&dodgy).is_err());
    }

    #[test]
    fn serializing_a_list_of_tuples_is_an_error() {
        let dodgy = vec![(1, 2), (3, 4)];

        assert!(to_string(&dodgy).is_err());
    }

    #[test]
    fn serialize_list_of_newtype_enums() {
        #[derive(Serialize)]
        enum Foo {
            A(u32),
            B(bool),
            C(Box<Foo>)
        }

        let f = vec![Foo::A(42), Foo::B(true), Foo::C(Box::new(Foo::B(false)))];
        let should_be = "<A>42</A><B>true</B><C><B>false</B></C>";

        let got = to_string(&f).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn serialize_tuple_containing_non_primitive_types() {
        #[derive(Serialize)]
        struct Foo;
        #[derive(Serialize)]
        struct Bar;

        let a = (Foo, Bar);
        let should_be = "<Foo></Foo><Bar></Bar>";

        let got = to_string(&a).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn serialize_tuple_of_primitives_is_error() {
        let value = (5, false);
        assert!(to_string(&value).is_err());
    }

    #[test]
    fn serialize_tuple_with_at_least_1_primitive_is_error() {
        #[derive(Serialize)]
        struct Foo;

        let value = (5, Foo);
        assert!(to_string(&value).is_err());
    }

    #[test]
    fn serialize_a_tuple_struct_containing_no_primitives() {
        #[derive(Serialize)]
        struct Foo(Bar, Baz);
        #[derive(Serialize)]
        struct Bar;
        #[derive(Serialize)]
        struct Baz(u32);

        let f = Foo(Bar, Baz(5));
        let should_be = "<Foo><Bar></Bar><Baz>5</Baz></Foo>";

        let got = to_string(&f).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn tuple_struct_cant_contain_primitives() {
        #[derive(Serialize)]
        struct Foo(u32, &'static str);

        let f = Foo(5, "bar");
        assert!(to_string(&f).is_err());
    }

    #[test]
    fn serialize_an_enum_with_a_unit_variant() {
        #[derive(Serialize)]
        enum Foo {
            A, 
        }

        let f = Foo::A;
        let should_not_be = "<A></A>";
        let should_be = "A";

        let got = to_string(&f).unwrap();
        assert_ne!(got, should_not_be);
        assert_eq!(got, should_be);

    }
    

    #[test]
    fn single_enum_ser() {
      #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
      enum EnumWithVariants {
        A,
        B(u32),
      }
      #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
      struct StructHoldingEnum {
        enum_here: EnumWithVariants,
      }
      #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
      struct StructTupleEnum(EnumWithVariants);

      let val = StructTupleEnum(EnumWithVariants::A);
      let expected = "<StructTupleEnum>A</StructTupleEnum>";
      let val_xml = to_string(&val).unwrap();
      assert_eq!(expected, val_xml);

      let val = StructHoldingEnum {
        enum_here: EnumWithVariants::A,
      };
      let expected = "<StructHoldingEnum><enum_here>A</enum_here></StructHoldingEnum>";
      let val_xml = to_string(&val).unwrap();
      assert_eq!(expected, val_xml);

      let val = StructHoldingEnum {
        enum_here: EnumWithVariants::B(10),
      };
      let expected = "<StructHoldingEnum><enum_here><B>10</B></enum_here></StructHoldingEnum>";
      let val_xml = to_string(&val).unwrap();
      assert_eq!(expected, val_xml);
    }
    
    #[test]
    fn serialize_an_enum_with_a_tuple_variant_containing_primitives_is_error() {
        #[derive(Serialize)]
        enum Foo {
            A(u32, &'static str), 
        }

        let f = Foo::A(5, "bar");
        assert!(to_string(&f).is_err());
    }

    #[test]
    fn you_can_serialize_a_tuple_variant_containing_no_primitives() {
        #[derive(Serialize)]
        enum Foo {
            A(Bar, Baz), 
        }
        #[derive(Serialize)]
        struct Bar;
        #[derive(Serialize)]
        struct Baz(u32);

        let f = Foo::A(Bar, Baz(5));
        let should_be = "<Foo><Bar></Bar><Baz>5</Baz></Foo>";

        let got = to_string(&f).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn serialize_a_struct_variant() {
        #[derive(Serialize)]
        enum Foo {
            Bar{ 
                x: u32,
                y: u32,
            }
        }

        let f = Foo::Bar {x: 5, y: 1};
        let should_be = "<Bar><x>5</x><y>1</y></Bar>";

        let got = to_string(&f).unwrap();
        assert_eq!(got, should_be);
    }
}
