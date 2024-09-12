// Copyright © 2024 Nathaniel Hardesty
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the “Software”), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

//! # ITEMS
//! **Based on SEMI E5§9.6**
//! 
//! ---------------------------------------------------------------------------
//! 
//! Standards compliant [Item] structures and enums designed to express and
//! enforce the specific [Format] of each [Item].
//! 
//! Each such item defined herein implements:
//! - [From]\<T\> for [Item]
//! - [TryFrom]\<[Item]\> for T
//! 
//! ---------------------------------------------------------------------------
//! 
//! As well as the list of specific [Item]s as defined in **Table 3 - Data Item
//! Dictionary**, certain shorthands for varying usage of [List]s are provided.
//! 
//! - [Optional List]: used to represent a [List] with either a set number of
//!   elements, or acceptably zero elements in certain cases.
//! - [Vectorized List]: used to represent a [List] with a variable number of
//!   elements of homogeneous structure.
//! - Rust's Native Unit Type (): Used to represent a [List] with zero
//!   elements.
//! - Rust's Native Tuple Types (A, B, ...): Used to represent a [List] with a
//!   set number of elements of heterogeneous structure.
//!    - Currently, only Tuples of length up to 5 are supported.
//! 
//! [Optional List]:   OptionList
//! [Vectorized List]: VecList
//! [Item]:            crate::Item
//! [Format]:          crate::format
//! [List]:            crate::Item::List

use crate::Item;
use crate::Error::{self, *};
use std::ascii::Char;
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// ## OPTIONAL LIST
/// 
/// Represents a List with either a set number of elements, or acceptably zero
/// elements in certain cases. The intent is that the type T will be a tuple
/// representing a heterogenous list of elements.
pub struct OptionList<T>(pub Option<T>);

/// ## ITEM -> OPTIONAL LIST
impl<A: TryFrom<Item, Error = Error> + Sized> TryFrom<Item> for OptionList<A> {
  type Error = Error;

  fn try_from(item: Item) -> Result<Self, Self::Error> {
    match item {
      Item::List(list) => {
        if list.is_empty() {
          Ok(Self(None))
        } else {
          Ok(Self(Some(Item::List(list).try_into()?)))
        }
      },
      _ => Err(Error::WrongFormat),
    }
  }
}

/// ## OPTIONAL LIST -> ITEM
impl<A: Into<Item>> From<OptionList<A>> for Item {
  fn from(option_list: OptionList<A>) -> Self {
    match option_list.0 {
      Some(item) => item.into(),
      None => Item::List(vec![]),
    }
  }
}

/// ## VECTORIZED LIST
/// 
/// Represents a List with a variable number of elements of homogeneous
/// structure. The intent is that type T will be a specific item.
pub struct VecList<T>(pub Vec<T>);

/// ## ITEM -> VECTORIZED LIST
impl<A: TryFrom<Item, Error = Error> + Sized> TryFrom<Item> for VecList<A> {
  type Error = Error;

  fn try_from(item: Item) -> Result<Self, Self::Error> {
    match item {
      Item::List(list) => {
        let mut vec = vec![];
        for list_item in list {
          vec.push(list_item.try_into()?)
        }
        Ok(Self(vec))
      },
      _ => Err(Error::WrongFormat),
    }
  }
}

/// ## VECTORIZED LIST -> ITEM
impl<A: Into<Item>> From<VecList<A>> for Item {
  fn from(vec_list: VecList<A>) -> Self {
    let mut vec = vec![];
    for item in vec_list.0 {
      vec.push(item.into())
    }
    Item::List(vec)
  }
}

// EMPTY LIST IS IMPLEMENTED BY THE USE OF THE UNIT TYPE ()

/// ## ITEM -> EMPTY LIST
impl TryFrom<Item> for () {
  type Error = Error;

  fn try_from(item: Item) -> Result<Self, Self::Error> {
    match item {
      Item::List(list) => {
        if list.is_empty() {
          Ok(())
        } else {
          Err(Error::WrongFormat)
        }
      },
      _ => Err(Error::WrongFormat),
    }
  }
}

/// ## EMPTY LIST -> ITEM
impl From<()> for Item {
  fn from(_empty_list: ()) -> Self {
    Item::List(vec![])
  }
}

// HETEROGENEOUS LISTS ARE IMPLEMENTED BY USE OF TUPLE TYPES (...)

/// ## ITEM -> HETEROGENEOUS LIST (2 ELEMENTS)
impl <
  A: TryFrom<Item, Error = Error>,
  B: TryFrom<Item, Error = Error>,
> TryFrom<Item> for (A, B) {
  type Error = Error;

  fn try_from(item: Item) -> Result<Self, Self::Error> {
    match item {
      Item::List(list) => {
        if list.len() == 2 {
          Ok((
            list[0].clone().try_into()?,
            list[1].clone().try_into()?,
          ))
        } else {
          Err(Error::WrongFormat)
        }
      },
      _ => Err(Error::WrongFormat),
    }
  }
}

/// ## HETEROGENEOUS LIST (2 ELEMENTS) -> ITEM
impl <
  A: Into<Item>,
  B: Into<Item>,
> From<(A, B)> for Item {
  fn from(value: (A, B)) -> Self {
    Item::List(vec![
      value.0.into(),
      value.1.into(),
    ])
  }
}

/// ## ITEM -> HETEROGENEOUS LIST (3 ELEMENTS)
impl <
  A: TryFrom<Item, Error = Error>,
  B: TryFrom<Item, Error = Error>,
  C: TryFrom<Item, Error = Error>,
> TryFrom<Item> for (A, B, C) {
  type Error = Error;

  fn try_from(item: Item) -> Result<Self, Self::Error> {
    match item {
      Item::List(list) => {
        if list.len() == 3 {
          Ok((
            list[0].clone().try_into()?,
            list[1].clone().try_into()?,
            list[2].clone().try_into()?,
          ))
        } else {
          Err(Error::WrongFormat)
        }
      },
      _ => Err(Error::WrongFormat),
    }
  }
}

/// ## HETEROGENEOUS LIST (3 ELEMENTS) -> ITEM
impl <
  A: Into<Item>,
  B: Into<Item>,
  C: Into<Item>,
> From<(A, B, C)> for Item {
  fn from(value: (A, B, C)) -> Self {
    Item::List(vec![
      value.0.into(),
      value.1.into(),
      value.2.into(),
    ])
  }
}

/// ## ITEM -> HETEROGENEOUS LIST (4 ELEMENTS)
impl <
  A: TryFrom<Item, Error = Error>,
  B: TryFrom<Item, Error = Error>,
  C: TryFrom<Item, Error = Error>,
  D: TryFrom<Item, Error = Error>,
> TryFrom<Item> for (A, B, C, D) {
  type Error = Error;

  fn try_from(item: Item) -> Result<Self, Self::Error> {
    match item {
      Item::List(list) => {
        if list.len() == 4 {
          Ok((
            list[0].clone().try_into()?,
            list[1].clone().try_into()?,
            list[2].clone().try_into()?,
            list[3].clone().try_into()?,
          ))
        } else {
          Err(Error::WrongFormat)
        }
      },
      _ => Err(Error::WrongFormat),
    }
  }
}

/// ## HETEROGENEOUS LIST (4 ELEMENTS) -> ITEM
impl <
  A: Into<Item>,
  B: Into<Item>,
  C: Into<Item>,
  D: Into<Item>,
> From<(A, B, C, D)> for Item {
  fn from(value: (A, B, C, D)) -> Self {
    Item::List(vec![
      value.0.into(),
      value.1.into(),
      value.2.into(),
      value.3.into(),
    ])
  }
}

/// ## ITEM -> HETEROGENEOUS LIST (5 ELEMENTS)
impl <
  A: TryFrom<Item, Error = Error>,
  B: TryFrom<Item, Error = Error>,
  C: TryFrom<Item, Error = Error>,
  D: TryFrom<Item, Error = Error>,
  E: TryFrom<Item, Error = Error>,
> TryFrom<Item> for (A, B, C, D, E) {
  type Error = Error;

  fn try_from(item: Item) -> Result<Self, Self::Error> {
    match item {
      Item::List(list) => {
        if list.len() == 5 {
          Ok((
            list[0].clone().try_into()?,
            list[1].clone().try_into()?,
            list[2].clone().try_into()?,
            list[3].clone().try_into()?,
            list[4].clone().try_into()?,
          ))
        } else {
          Err(Error::WrongFormat)
        }
      },
      _ => Err(Error::WrongFormat),
    }
  }
}

/// ## HETEROGENEOUS LIST (5 ELEMENTS) -> ITEM
impl <
  A: Into<Item>,
  B: Into<Item>,
  C: Into<Item>,
  D: Into<Item>,
  E: Into<Item>,
> From<(A, B, C, D, E)> for Item {
  fn from(value: (A, B, C, D, E)) -> Self {
    Item::List(vec![
      value.0.into(),
      value.1.into(),
      value.2.into(),
      value.3.into(),
      value.4.into(),
    ])
  }
}

/// ## ITEM -> HETEROGENEOUS LIST (6 ELEMENTS)
impl <
  A: TryFrom<Item, Error = Error>,
  B: TryFrom<Item, Error = Error>,
  C: TryFrom<Item, Error = Error>,
  D: TryFrom<Item, Error = Error>,
  E: TryFrom<Item, Error = Error>,
  F: TryFrom<Item, Error = Error>,
> TryFrom<Item> for (A, B, C, D, E, F) {
  type Error = Error;

  fn try_from(item: Item) -> Result<Self, Self::Error> {
    match item {
      Item::List(list) => {
        if list.len() == 6 {
          Ok((
            list[0].clone().try_into()?,
            list[1].clone().try_into()?,
            list[2].clone().try_into()?,
            list[3].clone().try_into()?,
            list[4].clone().try_into()?,
            list[5].clone().try_into()?,
          ))
        } else {
          Err(Error::WrongFormat)
        }
      },
      _ => Err(Error::WrongFormat),
    }
  }
}

/// ## HETEROGENEOUS LIST (6 ELEMENTS) -> ITEM
impl <
  A: Into<Item>,
  B: Into<Item>,
  C: Into<Item>,
  D: Into<Item>,
  E: Into<Item>,
  F: Into<Item>,
> From<(A, B, C, D, E, F)> for Item {
  fn from(value: (A, B, C, D, E, F)) -> Self {
    Item::List(vec![
      value.0.into(),
      value.1.into(),
      value.2.into(),
      value.3.into(),
      value.4.into(),
      value.5.into(),
    ])
  }
}

// TODO: ITEM -> HETEROGENEOUS LIST, UP TO 15 ELEMENTS
// TODO: HETEROGENEOUS LIST -> ITEM, UP TO 15 ELEMENTS
// NOTE: To implement Stream 1, only lengths of 2 and 3 are required.

// IMPLEMENTATION MACROS

/// ## DATA ITEM MACRO: SINGLE FORMAT
/// 
/// #### Arguments:
/// 
/// - **$name**: Name of struct.
/// - **$format**: Item format.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Expansion:
/// 
/// - From\<$name\> for Item
/// - TryFrom\<Item\> for $name
macro_rules! singleformat {
  (
    $name:ident,
    $format:ident
  ) => {
    impl From<$name> for Item {
      fn from(value: $name) -> Item {
        Item::$format(vec![value.0])
      }
    }
    impl TryFrom<Item> for $name {
      type Error = Error;

      fn try_from(value: Item) -> Result<Self, Self::Error> {
        match value {
          Item::$format(vec) => {
            if vec.len() == 1 {
              Ok(Self(vec[0]))
            } else {
              Err(WrongFormat)
            }
          },
          _ => Err(WrongFormat),
        }
      }
    }
  }
}

/// ## DATA ITEM MACRO: SINGLE FORMAT, VEC
/// 
/// #### Arguments:
/// - **$name**: Name of struct.
/// - **$format**: Item format.
/// - Optional:
///    - **$range**: Range expression limiting vector length.
///    - **$type**: Type contained in vector.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Expansion:
/// 
/// - From\<$name\> for Item
/// - TryFrom\<Item\> for $name
/// - Optional:
///    - new(Vec\<$type\>) -> Option\<Self\>
///    - read(&self) -> &Vec\<$type\>
macro_rules! singleformat_vec {
  (
    $name:ident,
    $format:ident
    $(,$range:expr, $type:ty)?
  ) => {
    $(impl $name {
      pub fn new(vec: Vec<$type>) -> Option<Self> {
        if $range.contains(&vec.len()) {
          Some(Self(vec))
        } else {
          None
        }
      }
      pub fn read(&self) -> &Vec<$type> {
        &self.0
      }
    })?
    impl From<$name> for Item {
      fn from(value: $name) -> Item {
        Item::$format(value.0)
      }
    }
    impl TryFrom<Item> for $name {
      type Error = Error;

      fn try_from(value: Item) -> Result<Self, Self::Error> {
        match value {
          Item::$format(vec) => {
            $(if !$range.contains(&vec.len()) {
              return Err(WrongFormat)
            })?
            Ok(Self(vec))
          },
          _ => Err(WrongFormat),
        }
      }
    }
  }
}

/// ## DATA ITEM MACRO: SINGLE FORMAT, ENUM
/// 
/// #### Arguments
/// 
/// - **$name**: Name of enum.
/// - **$format**: Item format.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Expansion
/// 
/// - From\<$name\> for Item
/// - TryFrom\<Item\> for $name
/// - From\<Vec\<$name\>\> for Item
/// - TryFrom\<Item\> for Vec\<$name\>
macro_rules! singleformat_enum {
  (
    $name:ident,
    $format:ident
  ) => {
    impl From<$name> for Item {
      fn from(value: $name) -> Item {
        Item::$format(vec![value.into()])
      }
    }
    impl TryFrom<Item> for $name {
      type Error = Error;

      fn try_from(value: Item) -> Result<Self, Self::Error> {
        match value {
          Item::$format(vec) => {
            if vec.len() == 1 {
              $name::try_from(vec[0]).map_err(|_| -> Self::Error {WrongFormat})
            } else {
              Err(WrongFormat)
            }
          },
          _ => Err(WrongFormat),
        }
      }
    }
    impl From<Vec<$name>> for Item {
      fn from(vec: Vec<$name>) -> Item {
        let mut newvec = vec![];
        for value in vec {
          newvec.push(value.into());
        }
        Item::$format(newvec)
      }
    }
    impl TryFrom<Item> for Vec<$name> {
      type Error = Error;

      fn try_from(item: Item) -> Result<Self, Self::Error> {
        match item {
          Item::$format(vec) => {
            let mut newvec: Vec<$name> = vec![];
            for value in vec {
              newvec.push($name::try_from(value).map_err(|_| -> Self::Error {WrongFormat})?);
            }
            Ok(newvec)
          },
          _ => Err(WrongFormat),
        }
      }
    }
  }
}

/// ## DATA ITEM MACRO: MULTIFORMAT
/// 
/// #### Arguments
/// 
/// - **$name**: Name of enum.
/// - **$format**: Item format.
/// - Optional:
///    - **$formats**: Further item formats.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Expansion
/// 
/// - From\<$name\> for Item
/// - TryFrom\<Item\> for $name
macro_rules! multiformat {
  (
    $name:ident
    ,$format:ident
    $(,$formats:ident)*
    $(,)?
  ) => {
    impl From<$name> for Item {
      fn from(value: $name) -> Item {
        match value {
          $name::$format(val) => Item::$format(vec![val]),
          $(
            $name::$formats(val) => Item::$formats(vec![val]),
          )*
        }
        
      }
    }
    impl TryFrom<Item> for $name {
      type Error = Error;

      fn try_from(value: Item) -> Result<Self, Self::Error> {
        match value {
          Item::$format(vec) => {
            if vec.len() == 1 {
              Ok(Self::$format(vec[0]))
            } else {
              Err(WrongFormat)
            }
          },
          $(
            Item::$formats(vec) => {
              if vec.len() == 1 {
                Ok(Self::$formats(vec[0]))
              } else {
                Err(WrongFormat)
              }
            },
          )*
          _ => Err(WrongFormat),
        }
      }
    }
  }
}

/// ## DATA ITEM MACRO: MULTIFORMAT + ASCII
/// 
/// #### Arguments
/// 
/// - **$name**: Name of enum.
/// - **$format**: Item format.
/// - Optional:
///    - **$formats**: Further item formats.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Expansion
/// 
/// - From\<$name\> for Item
/// - TryFrom\<Item\> for $name
macro_rules! multiformat_ascii {
  (
    $name:ident
    ,$format:ident
    $(,$formats:ident)*
    $(,)?
  ) => {
    impl From<$name> for Item {
      fn from(value: $name) -> Item {
        match value {
          $name::Ascii(vec) => Item::Ascii(vec),
          $name::$format(val) => Item::$format(vec![val]),
          $($name::$formats(val) => Item::$formats(vec![val]),)*
        }
      }
    }
    impl TryFrom<Item> for $name {
      type Error = Error;

      fn try_from(item: Item) -> Result<Self, Self::Error> {
        match item {
          Item::Ascii(vec) => Ok($name::Ascii(vec)),
          Item::$format(vec) => {
            if vec.len() == 1 {
              Ok(Self::$format(vec[0]))
            } else {
              Err(WrongFormat)
            }
          },
          $(Item::$formats(vec) => {
            if vec.len() == 1 {
              Ok(Self::$formats(vec[0]))
            } else {
              Err(WrongFormat)
            }
          },)*
          _ => Err(WrongFormat),
        }
      }
    }
  }
}

/// ## DATA ITEM MACRO: MULTIFORMAT, VEC
/// 
/// #### Arguments
/// 
/// - **$name**: Name of enum.
/// - **$format**: Item format.
/// - Optional:
///    - **$formats**: Further item formats.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Expansion
/// 
/// - From\<$name\> for Item
/// - TryFrom\<Item\> for $name
macro_rules! multiformat_vec {
  (
    $name:ident
    ,$format:ident
    $(,$formats:ident)*
    $(,)?
  ) => {
    impl From<$name> for Item {
      fn from(value: $name) -> Item {
        match value {
          $name::$format(vec) => Item::$format(vec),
          $(
            $name::$formats(vec) => Item::$formats(vec),
          )*
        }
        
      }
    }
    impl TryFrom<Item> for $name {
      type Error = Error;

      fn try_from(value: Item) -> Result<Self, Self::Error> {
        match value {
          Item::$format(vec) => {
            Ok(Self::$format(vec))
          },
          $(
            Item::$formats(vec) => {
              Ok(Self::$formats(vec))
            },
          )*
          _ => Err(WrongFormat),
        }
      }
    }
  }
}

// ITEMS

/// ## ABS
/// 
/// Any binary string.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F25, S2F26
#[derive(Clone, Debug)]
pub struct AnyBinaryString(pub Vec<u8>);
singleformat_vec!{AnyBinaryString, Bin}

/// ## ACCESSMODE
/// 
/// Load Port Access Mode
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S3F21, S3F27
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum AccessMode {
  Manual = 0,
  Auto = 1,
}
singleformat_enum!{AccessMode, U1}

/// ## ACDS
/// 
/// After Command Codes
/// 
/// Vector of all command codes which the defined command must succeed
/// within the same block.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S7F22
#[derive(Clone, Debug)]
pub enum AfterCommandCodes {
  I2(Vec<i16>),
  U2(Vec<u16>),
}
multiformat_vec!{AfterCommandCodes, I2, U2}

/// ## ACKA
/// 
/// Request success, true is successful, false is unsuccessful.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S5F14, S5F15, S5F18
/// - S16F4, S16F6, S16F7, S16F12, S16F16, S16F18, S16F24, S16F26, S16F28,
///   S16F30
/// - S17F4, S17F8, S17F14
#[derive(Clone, Copy, Debug)]
pub struct AcknowledgeAny(pub bool);
singleformat!{AcknowledgeAny, Bool}

// TODO: ACKC3
// How to deal with 1-63 being reserved but the rest being open for user values?

// TODO: ACKC5
// How to deal with 1-63 being reserved but the rest being open for user values?

// TODO: ACKC6
// How to deal with 1-63 being reserved but the rest being open for user values?

// TODO: ACKC7
// How to deal with 7-63 being reserved but the rest being open for user values?

// TODO: ACKC7A
// How to deal with 6-63 being reserved but the rest being open for user values?

// TODO: ACKC10
// How to deal with 3-63 being reserved but the rest being open for user values?

// TODO: ACKC13
// How to deal with 11-127 being reserved but the rest being open for user values?

// TODO: ACKC15
// How to deal with 5-63 being reserved but the rest being open for user values?

/// ## AGENT
/// 
/// TODO: Document variable based on appearances in streams.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S15F11, S15F12, S15F21, S15F22, S15F25
#[derive(Clone, Debug)]
pub struct Agent(pub Vec<Char>);
singleformat_vec!{Agent, Ascii}

/// ## ALCD
/// 
/// Alarm code byte.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Values
/// 
/// - bit 8 = 1 - Alarm Set
/// - bit 8 = 0 - Alarm Cleared
/// - bit 7-1 - Alarm Category
///   - 0 - Not Used
///   - 1 - Personal Safety
///   - 2 - Equipment Safety
///   - 3 - Parameter Control Warning
///   - 4 - Parameter Control Error
///   - 5 - Irrecoverable Error
///   - 6 - Equipment Status Warning
///   - 7 - Attention Flags
///   - 8 - Data Integrity
///   - \>8 - Other Categories
///   - 9-63 - Reserved
/// 
/// TODO: Implement Set/Cleared and Category Manually?
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S5F1, S5F6, S5F8
#[derive(Clone, Copy, Debug)]
pub struct AlarmCode(pub u8);
singleformat!{AlarmCode, Bin}

/// ## ALED
/// 
/// Alarm Enable/Disable Code, 1 Byte.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Values
/// 
/// - Bit 8
///   - 0 = Disable Alarm
///   - 1 = Enable Alarm
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S5F3
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum AlarmEnableDisable {
  Disable = 0,
  Enable = 128,
}
singleformat_enum!{AlarmEnableDisable, Bin}

/// ## ALID
/// 
/// Alarm identification.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S5F1, S5F3, S5F5, S5F6, S5F8
#[derive(Clone, Copy, Debug)]
pub enum AlarmID {
  I1(i8),
  I2(i16),
  I4(i32),
  I8(i64),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat!{AlarmID, I1, I2, I4, I8, U1, U2, U4, U8}

/// ## ALTX
/// 
/// Alarm text, maximum 120 characters.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S5F1, S5F6, S5F8
#[derive(Clone, Debug)]
pub struct AlarmText(Vec<Char>);
singleformat_vec!{AlarmText, Ascii, 0..=120, Char}

/// ## ATTRDATA
/// 
/// Specific attribute value for a specific object.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F20]
/// - S3F17, S3F35
/// - S13F13, S13F16
/// - S14F1, S14F2, S14F3, S14F4, S14F9, S14F10, S14F11, S14F12, S14F13,
///   S14F14, S14F15, S14F16, S14F17, S14F18, S14F19
/// - S18F1, S18F3
/// 
/// [S1F20]: crate::messages::s1::AttributeData
pub enum AttributeValue {
  List(Vec<Item>),
  Bin(Vec<u8>),
  Bool(Vec<bool>),
  Ascii(Vec<Char>),
  I1(Vec<i8>),
  I2(Vec<i16>),
  I4(Vec<i32>),
  I8(Vec<i64>),
  U1(Vec<u8>),
  U2(Vec<u16>),
  U4(Vec<u32>),
  U8(Vec<u64>),
  F4(Vec<f32>),
  F8(Vec<f64>),
}
multiformat_vec!{AttributeValue, List, Bin, Bool, Ascii, I1, I2, I4, I8, U1, U2, U4, U8, F4, F8}

/// ## ATTRID
/// 
/// Identifier for an attribute for a type of object.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F19]
/// - S3F17, S3F35
/// - S13F13, S13F16
/// - S14F1, S14F2, S14F3, S14F4, S14F8, S14F9, S14F10, S14F11, S14F12,
///   S14F13, S14F14, S14F15, S14F16, S14F17, S14F18, S14F19
/// - S18F1, S18F3
/// 
/// [S1F19]: crate::messages::s1::GetAttribute
pub enum AttributeID {
  Ascii(Vec<Char>),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat_ascii!{AttributeID, U1, U2, U4, U8}

/// ## ATTRRELN
/// 
/// The relationship between a qualyfing value and the value of an attribute
/// of an object instance (i.e. value of interest).
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S14F1
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum AttributeRelation {
  /// ### EQUAL TO
  /// 
  /// The qualifying value is equal to the value of interest.
  EqualTo = 0,

  /// ### NOT EQUAL TO
  /// 
  /// The qualifying value is not equal to the value of interest.
  NotEqualTo = 1,

  /// ### LESS THAN
  /// 
  /// The qualifying value is less than the value of interest.
  LessThan = 2,

  /// ### LESS THAN OR EQUAL TO
  /// 
  /// The qualifying value is less than or equal to the value of interest.
  LessThanOrEqualTo = 3,

  /// ### GREATER THAN
  /// 
  /// The qualifying value is greater than the value of interest.
  GreaterThan = 4,

  /// ### GREATER THAN OR EQUAL TO
  /// 
  /// The qualifying value is greater than or equal to the value of interest.
  GreaterThanOrEqualTo = 5,

  /// ### PRESENT
  /// 
  /// The qualifying value is present in the set of the value of interest.
  Present = 6,

  /// ### ABSENT
  /// 
  /// The qualifying value is absent from the set of the value of interest.
  Absent = 7,
}
singleformat_enum!{AttributeRelation, U1}

/// ## BCDS
/// 
/// Before Command Codes
/// 
/// Vector of all command codes which the defined command must preceed within
/// the same block.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S7F22
#[derive(Clone, Debug)]
pub enum BeforeCommandCodes {
  I2(Vec<i16>),
  U2(Vec<u16>),
}
multiformat_vec!{BeforeCommandCodes, I2, U2}

/// ## BCEQU
/// 
/// Bin code equivalents.
/// 
/// Array of all codes that are to be processed.
/// 
/// Must be same format as [BINLT] and [NULBC].
/// 
/// Zero length indicates that all codes be sent.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S12F3, S12F4
/// 
/// [BINLT]: BinList
/// [NULBC]: NullBinCode
#[derive(Clone, Debug)]
pub enum BinCodeEquivalents {
  Ascii(Vec<Char>),
  U1(Vec<u8>),
}
multiformat_vec!{BinCodeEquivalents, Ascii, U1}

/// ## BINLT
/// 
/// The bin list.
/// 
/// Array of bin values.
/// 
/// Must be same format as [BCEQU] and [NULBC].
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S12F7, S12F9, S12F11, S12F14, S12F16, S12F18
/// 
/// [BCEQU]: BinCodeEquivalents
/// [NULBC]: NullBinCode
#[derive(Clone, Debug)]
pub enum BinList {
  Ascii(Vec<Char>),
  U1(Vec<u8>),
}
multiformat_vec!{BinList, Ascii, U1}

/// ## BLKDEF
/// 
/// Block Definition
/// 
/// Specifies whether a command being defined starts, terminates, or is
/// within the body of a block.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S7F22
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(i8)]
pub enum BlockDefinition {
  /// ### TERMINATE
  /// 
  /// Command terminates a block body.
  Terminate = -1,

  /// ### WITHIN
  /// 
  /// Command neither starts or terminates a block body.
  Within = 0,

  /// ### START
  /// 
  /// Command starts a block body.
  Start = 1,
}
singleformat_enum!{BlockDefinition, I1}

/// ## BPD
/// 
/// Boot program data.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S8F2
#[derive(Clone, Debug)]
pub struct BootProgramData(pub Vec<u8>);
singleformat_vec!{BootProgramData, Bin}

// TODO: BYTMAX
// How to deal with negative values being invalid even though you can use signed int?

// TODO: CAACK
// Usual about reserved/user enum values.

/// ## CARRIERACTION
/// 
/// Specifies the action requested for a carrier.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S3F17
#[derive(Clone, Debug)]
pub struct CarrierAction(pub Vec<Char>);
singleformat_vec!{CarrierAction, Ascii}

/// ## CARRIERID
/// 
/// The identifier of a carrier.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S3F17, S16F11, S16F15
#[derive(Clone, Debug)]
pub struct CarrierID(pub Vec<Char>);
singleformat_vec!{CarrierID, Ascii}

/// ## CARRIERSPEC
/// 
/// The object specifier for a carrier.
/// 
/// TODO: Make this conform to OBJSPEC requirements, seems related to E39.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S3F29, S3F31
#[derive(Clone, Debug)]
pub struct CarrierSpecifier(pub Vec<Char>);
singleformat_vec!{CarrierSpecifier, Ascii}

// TODO: CATTRDATA
// Seems like it should mirror ATTRDATA.

/// ## CATTRID
/// 
/// The name of a carrier attribute.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S3F17
#[derive(Clone, Debug)]
pub struct CarrierAttributeID(pub Vec<Char>);
singleformat_vec!{CarrierAttributeID, Ascii}

/// ## CCODE
/// 
/// Command code.
/// 
/// Each command code corresponds to a unique process operation the machine
/// is capable of performing.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S7F22, S7F23, S7F26, S7F31, S7F39, S7F43
#[derive(Clone, Debug)]
pub enum CommandCode {
  Ascii(Vec<Char>),
  I2(Vec<i16>),
  I4(Vec<i32>),
  U2(Vec<u16>),
  U4(Vec<u32>),
}
multiformat_vec!{CommandCode, Ascii, I2, I4, U2, U4}

/// ## CEED
/// 
/// Collection event or trace enable/disable code, 1 byte.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Values
/// 
/// - False = Disable
/// - True = Enable
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F37, S17F5
#[derive(Clone, Debug)]
pub struct CollectionEventEnableDisable(pub bool);
singleformat!{CollectionEventEnableDisable, Bool}

/// ## CEID
/// 
/// Collection event ID.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F23], [S1F24]
/// - S2F35, S2F37
/// - S6F3, S6F8, S6F9, S6F11, S6F13, S6F15, S6F16, S6F17, S6F18
/// - S17F5, S17F9, S17F10, S17F11, S17F12
/// 
/// [S1F23]: crate::messages::s1::CollectionEventNamelistRequest
/// [S1F24]: crate::messages::s1::CollectionEventNamelist
pub enum CollectionEventID {
  Ascii(Vec<Char>),
  I1(i8),
  I2(i16),
  I4(i32),
  I8(i64),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat_ascii!{CollectionEventID, I1, I2, I4, I8, U1, U2, U4, U8}

/// ## CENAME
/// 
/// Collection event name.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F24]
/// 
/// [S1F24]: crate::messages::s1::CollectionEventNamelist
#[derive(Clone, Debug)]
pub struct CollectionEventName(pub Vec<Char>);
singleformat_vec!{CollectionEventName, Ascii}

// TODO: CEPACK
// How to handle this somewhat complicated seeming list form of the variable?

// TODO: CEPVAL
// Just seems like a lot of work right now, should probably be done alongside CEPACK.

/// ## CKPNT
/// 
/// Checkpoint as defined by the sending system.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S13F3, S13F6
#[derive(Clone, Copy, Debug)]
pub struct Checkpoint(pub u32);
singleformat!{Checkpoint, U4}

/// ## CMDA
/// 
/// Command acknowledge code.
/// 
/// TODO: Implement Format 31.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F22, S2F28
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CommandAcknowledge {
  Ok = 0,
  CommandDoesNotExist = 1,
  CannotPerformNow = 2,
}
singleformat_enum!{CommandAcknowledge, U1}

// TODO: CMDMAX
// How to deal with negative values being invalid even though you can use signed int?

/// ## CNAME
/// 
/// Command name, maximum 16 characters.
/// 
/// A text string which is unique among other command names in a PCD, which
/// describes the processing done by the equipment for the corresponding
/// [CCODE].
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S7F22
/// 
/// [CCODE]: CommandCode
#[derive(Clone, Debug)]
pub struct CommandName(Vec<Char>);
singleformat_vec!{CommandName, Ascii, 0..=16, Char}

/// ## COLCT
/// 
/// Column count, in die increments.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S12F1, S12F4
#[derive(Clone, Copy, Debug)]
pub enum ColumnCount {
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat!{ColumnCount, U1, U2, U4, U8}

/// ## COLHDR
/// 
/// Text description of contents of [TBLELT], 1-20 characters.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S13F13, S13F15, S13F16
/// 
/// [TBLELT]: TableElement
#[derive(Clone, Debug)]
pub struct ColumnHeader(Vec<Char>);
singleformat_vec!{ColumnHeader, Ascii, 1..=20, Char}

/// ## COMMACK
/// 
/// Establish Communications Acknowledge Code, 1 byte.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F14]
/// 
/// [S1F14]: crate::messages::s1::EquipmentCRA
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CommAck {
  /// ### ACCEPTED
  Accepted = 0,

  /// ### DENIED
  Denied = 1,
}
singleformat_enum!{CommAck, Bin}

/// ## COMPARISONOPERATOR
/// 
/// Choice of available operators that compare the supplied value to the
/// current attribute value.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S19F1
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ComparisonOperator {
  /// ### EQ
  /// 
  /// Equals, numeric or string.
  EqualTo = 0,

  /// ### NOTEQ
  /// 
  /// Not Equal, numeric or string.
  NotEqualTo = 1,

  /// ### LT
  /// 
  /// Less Than, numeric.
  LessThan = 2,

  /// ### LE
  /// 
  /// Less than or equal to, numeric.
  LessThanOrEqualTo = 3,

  /// ### GT
  /// 
  /// Greater than, numeric.
  GreaterThan = 4,

  /// ### GE
  /// 
  /// Greater than or equal to, numeric.
  GreaterThanOrEqualTo = 5,

  /// ### LIKE
  /// 
  /// Contains the substring, string.
  Like = 6,

  /// ### NOTLIKE
  /// 
  /// Does not contain the substring, string.
  NotLike = 7,
}
singleformat_enum!{ComparisonOperator, U1}

/// ## CONDITION
/// 
/// Provides condition information for a subsystem component.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [CONDITIONLIST]
/// 
/// [CONDITIONLIST]: ConditionList
#[derive(Clone, Debug)]
pub struct Condition(pub Vec<Char>);
singleformat_vec!{Condition, Ascii}

/// ## CONDITIONLIST
/// 
/// A list of [CONDITION] data sent in a fixed order.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S18F16
/// 
/// [CONDITION]: Condition
pub type ConditionList = VecList<Condition>;

/// ## CPACK
/// 
/// Command parameter acknowledge code, 1 byte.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F42
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CommandParamaterAcknowledge {
  /// CPNAME does not exist.
  ParameterNameDoesNotExist = 1,

  /// Illegal value specified for CPVAL.
  IllegalValue = 2,

  /// Illegal format specified for CPVAL.
  IllegalFormat = 3,
}
singleformat_enum!{CommandParamaterAcknowledge, Bin}

// TODO: CPNAME
// How to combine ASCII vec and ints which are likely not vec?

// TODO: CPVAL
// Just seems like a lot of work right now, should probably be done alongside CPNAME.

/// ## CSAACK
/// 
/// Equipment acknowledge code, 1 byte.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F8
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ServiceAcknowledgeCode {
  Ok = 0,
  Busy = 1,
  InvalidSPID = 2,
  InvalidData = 3,
}
singleformat_enum!{ServiceAcknowledgeCode, Bin}

/// ## CTLJOBCMD
/// 
/// Control Job command code.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S16F27
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ControlJobCommand {
  /// ### CJStart
  Start = 1,

  /// ### CJPause
  Pause = 2,

  /// ### CJResume
  Resume = 3,

  /// ### CJCancel
  Cancel = 4,

  /// ### CJDeselect
  Deselect = 5,

  /// ### CJStop
  Stop = 6,

  /// ### CJAbort
  Abort = 7,

  /// ### CJHOQ
  HeadOfQueue = 8,
}
singleformat_enum!{ControlJobCommand, U1}

// TODO: CTLJOBID
// Something about OBJID.

/// ## DATA
/// 
/// A string of unformatted data.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S3F30, S3F31
/// - S18F6, S18F7
#[derive(Clone, Debug)]
pub struct Data(pub Vec<Char>);
singleformat_vec!{Data, Ascii}

/// ## DATAACK
/// 
/// Data acknowledge code.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S14F22
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum DataAcknowledge {
  Ok = 0,
  UnknownDataID = 1,
  InvalidParameter = 2,
}
singleformat_enum!{DataAcknowledge, Bin}

// TODO: DATAID
// How to combine ASCII vec and ints which are likely not vec?

/// ## DATALENGTH
/// 
/// Total bytes to be sent.
/// 
/// TODO: Do negative numbers need to be restricted?
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F39
/// - S3F15, S3F29, S3F31
/// - S4F25
/// - S6F5
/// - S13F11
/// - S14F23
/// - S16F1
/// - S18F5, S18F7
/// - S19F19
#[derive(Clone, Debug)]
pub enum DataLength {
  I1(i8),
  I2(i16),
  I4(i32),
  I8(i64),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat!{DataLength, I1, I2, I4, I8, U1, U2, U4, U8}

/// ## DSPER
/// 
/// Data sample period.
/// 
/// TODO: Implement format restrictions.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Values
/// 
/// Format 1:
/// - hhmmss
///    - hh = Hours
///    - mm = Minutes
///    - ss = Seconds
/// 
/// Format 2:
/// - hhmmsscc
///    - hh = Hours
///    - mm = Minutes
///    - ss = Seconds
///    - cc = CentiSeconds
/// 
/// Equipment must implement Format 1, and may optionally implement Format 2.
/// 
/// Support for Format 2 does not necessitate a trace resolution of 0.01sec.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F23
#[derive(Clone, Debug)]
pub struct DataSamplePeriod(pub Vec<Char>);
singleformat_vec!{DataSamplePeriod, Ascii}

/// ## DVVALNAME
/// 
/// Descriptive name for a data variable.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F22]
/// 
/// [S1F22]: crate::messages::s1::DataVariableNamelist
#[derive(Clone, Debug)]
pub struct DataVariableValueName(pub Vec<Char>);
singleformat_vec!{DataVariableValueName, Ascii}

/// ## EAC
/// 
/// EquipmentAcknowledgeCode, 1 byte.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F16
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum EquipmentAcknowledgeCode {
  Acknowledge = 0,
  DoesNotExist = 1,
  Busy = 2,
  OutOfRange = 3,
}
singleformat_enum!{EquipmentAcknowledgeCode, Bin}

/// ## ECDEF
/// 
/// Equipment constant default value.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F30
pub enum EquipmentConstantDefaultValue {
  Bin(Vec<u8>),
  Bool(Vec<bool>),
  Ascii(Vec<Char>),
  Jis8(String),
  I1(Vec<i8>),
  I2(Vec<i16>),
  I4(Vec<i32>),
  I8(Vec<i64>),
  U1(Vec<u8>),
  U2(Vec<u16>),
  U4(Vec<u32>),
  U8(Vec<u64>),
  F4(Vec<f32>),
  F8(Vec<f64>),
}
multiformat_vec!{EquipmentConstantDefaultValue, Bin, Bool, Ascii, Jis8, I1, I2, I4, I8, U1, U2, U4, U8, F4, F8}

/// ## ECID
/// 
/// Equipment constant ID.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F13, S2F15, S2F29, S2F30
#[derive(Clone, Debug)]
pub enum EquipmentConstantID {
  Ascii(Vec<Char>),
  I1(i8),
  I2(i16),
  I4(i32),
  I8(i64),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat_ascii!{EquipmentConstantID, I1, I2, I4, I8, U1, U2, U4, U8}

/// ## ECMAX
/// 
/// Equipment constant maximum value.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F30
pub enum EquipmentConstantMaximumValue {
  Bin(Vec<u8>),
  Bool(Vec<bool>),
  Ascii(Vec<Char>),
  Jis8(String),
  I1(Vec<i8>),
  I2(Vec<i16>),
  I4(Vec<i32>),
  I8(Vec<i64>),
  U1(Vec<u8>),
  U2(Vec<u16>),
  U4(Vec<u32>),
  U8(Vec<u64>),
  F4(Vec<f32>),
  F8(Vec<f64>),
}
multiformat_vec!{EquipmentConstantMaximumValue, Bin, Bool, Ascii, Jis8, I1, I2, I4, I8, U1, U2, U4, U8, F4, F8}

/// ## ECMIN
/// 
/// Equipment constant minimum value.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F30
pub enum EquipmentConstantMinimumValue {
  Bin(Vec<u8>),
  Bool(Vec<bool>),
  Ascii(Vec<Char>),
  Jis8(String),
  I1(Vec<i8>),
  I2(Vec<i16>),
  I4(Vec<i32>),
  I8(Vec<i64>),
  U1(Vec<u8>),
  U2(Vec<u16>),
  U4(Vec<u32>),
  U8(Vec<u64>),
  F4(Vec<f32>),
  F8(Vec<f64>),
}
multiformat_vec!{EquipmentConstantMinimumValue, Bin, Bool, Ascii, Jis8, I1, I2, I4, I8, U1, U2, U4, U8, F4, F8}

/// ## ECNAME
/// 
/// Equipment constant name.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F30
pub struct EquipmentConstantName(pub Vec<Char>);
singleformat_vec!{EquipmentConstantName, Ascii}

/// ## ECV
/// 
/// Equipment constant value.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F14, S2F15
#[derive(Clone, Debug)]
pub enum EquipmentConstantValue {
  Bin(Vec<u8>),
  Bool(Vec<bool>),
  Ascii(Vec<Char>),
  Jis8(String),
  I1(Vec<i8>),
  I2(Vec<i16>),
  I4(Vec<i32>),
  I8(Vec<i64>),
  U1(Vec<u8>),
  U2(Vec<u16>),
  U4(Vec<u32>),
  U8(Vec<u64>),
  F4(Vec<f32>),
  F8(Vec<f64>),
}
multiformat_vec!{EquipmentConstantValue, Bin, Bool, Ascii, Jis8, I1, I2, I4, I8, U1, U2, U4, U8, F4, F8}

/// ## ERRCODE
/// 
/// Code identifying an error.
/// 
/// TODO: Implement user defined errors.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F20]
/// - S3F18, S3F20, S3F22, S3F24, S3F26, S3F28, S3F30, S3F32,
///   S3F34, S3F36
/// - S4F20, S4F22, S4F23, S4F31, S4F33
/// - S5F14, S5F15, S5F18
/// - S6F25, S6F30
/// - S13F14, S13F16
/// - S14F2, S14F4, S14F5, S14F6, S14F8, S14F10, S14F12,
///   S14F14, S14F16, S14F18, S14F20, S14F21, S14F26, S14F28
/// - S15F4, S15F6, S15F8, S15F10, S15F12, S15F14, S15F16,
///   S15F18, S15F20, S15F22, S15F24, S15F26, S15F28, S15F30,
///   S15F32, S15F34, S15F36, S15F38, S15F40, S15F42, S15F44,
///   S15F48, S15F53
/// - S16F4, S16F6, S16F7, S16F12, S16F16, S16F18, S16F24,
///   S16F26, S16F28
/// - S17F2, S17F4, S17F6, S17F8, S17F10, S17F12, S17F14
/// 
/// [S1F20]: crate::messages::s1::AttributeData
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u64)]
pub enum ErrorCode {
  NoError                         = 0,
  UnknownObjectInObjectSpecifier  = 1,
  UnknownTargetObjectType         = 2,
  UnknownObjectInstance           = 3,
  UnknownAttributeName            = 4,
  ReadonlyAttributeAccessDenied   = 5,
  UnknownObjectType               = 6,
  InvalidAttributeValue           = 7,
  SyntaxError                     = 8,
  VerificationError               = 9,
  ValidationError                 = 10,
  ObjectIdentifierInUse           = 11,
  ParametersImproperlySpecified   = 12,
  InsufficientParametersSpecified = 13,
  UnsupportedOptionRequested      = 14,
  Busy                            = 15,
  NotAvailableForProcessing       = 16,
  CommandNotValidForCurrentState  = 17,
  NoMaterialAltered               = 18,
  MaterialPartiallyProcessed      = 19,
  AllMaterialProcessed            = 20,
  RecipeSpecificationError        = 21,
  FailedDuringProcessing          = 22,
  FailedWhileNotProcessing        = 23,
  FailedDueToLackOfMaterial       = 24,
  JobAborted                      = 25,
  JobStopped                      = 26,
  JobCancelled                    = 27,
  CannotChangeSelectedRecipe      = 28,
  UnknownEvent                    = 29,
  DuplicateReportID               = 30,
  UnknownDataReport               = 31,
  DataReportNotLinked             = 32,
  UnknownTraceReport              = 33,
  DuplicateTraceID                = 34,
  TooManyDataReports              = 35,
  SamplePeriodOutOfRange          = 36,
  GroupSizeTooLarge               = 37,
  RecoveryActionCurrentlyInvalid  = 38,
  BusyWithAnotherRecovery         = 39,
  NoActiveRecoveryAction          = 40,
  ExceptionRecoveryFailed         = 41,
  ExceptionRecoveryAborted        = 42,
  InvalidTableElement             = 43,
  UnknownTableElement             = 44,
  CannotDeletePredefined          = 45,
  InvalidToken                    = 46,
  InvalidParameter                = 47,
  LoadPortDoesNotExist            = 48,
  LoadPortAlreadyInUse            = 49,
  MissingCarrier                  = 50,
  //51-63: Reserved
  //64-32767: User Defined
  ActionWillBePerformed           = 32768,
  ActionCannotBePerformedNow      = 32769,
  ActionFailedDueToErrors         = 32770,
  InvalidCommand                  = 32771,
  ClientAlr                       = 32772,
  DuplicateClientID               = 32773,
  InvalidClientType               = 32774,
  IncompatibleVersions            = 32775,
  UnrecognizedClientID            = 32776,
  FailedCompletedUnsuccessfully   = 32777,
  FailedUnsafe                    = 32778,
  SensorDetectedObstacle          = 32779,
  MaterialNotSent                 = 32780,
  MaterialNotReceived             = 32781,
  MaterialLost                    = 32782,
  HardwareFailure                 = 32783,
  TransferCancelled               = 32784,
  //32785-32789: Reserved for SEMI E127
  //32793-65335: Reserved
  //65536+: User Defined
}
impl From<ErrorCode> for Item {
  fn from(value: ErrorCode) -> Self {
    let number: u64 = value.into();
    if number < 256 {
      Item::U1(vec![number as u8])
    } else if number < 65536 {
      Item::U2(vec![number as u16])
    } else {
      Item::U8(vec![number])
    }
  }
}
impl TryFrom<Item> for ErrorCode {
  type Error = Error;

  fn try_from(value: Item) -> Result<Self, Self::Error> {
    match value {
      Item::U1(vec) => {
        if vec.len() == 1 {
          ErrorCode::try_from(vec[0] as u64).map_err(|_| -> Self::Error {WrongFormat})
        } else {
          Err(WrongFormat)
        }
      },
      Item::U2(vec) => {
        if vec.len() == 1 {
          ErrorCode::try_from(vec[0] as u64).map_err(|_| -> Self::Error {WrongFormat})
        } else {
          Err(WrongFormat)
        }
      },
      Item::U4(vec) => {
        if vec.len() == 1 {
          ErrorCode::try_from(vec[0] as u64).map_err(|_| -> Self::Error {WrongFormat})
        } else {
          Err(WrongFormat)
        }
      },
      Item::U8(vec) => {
        if vec.len() == 1 {
          ErrorCode::try_from(vec[0]).map_err(|_| -> Self::Error {WrongFormat})
        } else {
          Err(WrongFormat)
        }
      },
      _ => Err(WrongFormat),
    }
  }
}

/// ## ERRTEXT
/// 
/// Text string describing the error noted in the corresponding [ERRCODE].
/// 
/// Maximum 120 characters.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F20]
/// - S3F18, S3F20, S3F22, S3F24, S3F26, S3F28, S3F30, S3F32, S3F34, S3F36
/// - S4F20, S4F22, S4F23, S4F31, S4F33
/// - S5F14, S5F15, S5F18
/// - S6F25
/// - S13F14, S13F16
/// - S14F2, S14F4, S14F6, S14F8, S14F10, S14F12, S14F14, S14F16, S14F18,
///   S14F20, S14F21, S14F26, S14F28
/// - S15F4, S15F6, S15F8, S15F10, S15F12, S15F14, S15F16, S15F18, S15F20,
///   S15F22, S15F24, S15F26, S15F28, S15F30, S15F32, S15F34, S15F36, S15F38,
///   S15F40, S15F42, S15F44, S15F48, S15F53
/// - S16F4, S16F6, S16F7, S16F12, S16F16, S16F18, S16F24, S16F26, S16F28
/// - S17F4, S17F8, S17F18
/// 
/// [ERRCODE]: ErrorCode
/// [S1F20]:   crate::messages::s1::AttributeData
#[derive(Clone, Debug)]
pub struct ErrorText(Vec<Char>);
singleformat_vec!{ErrorText, Ascii, 0..=120, Char}

/// ## GRANT
/// 
/// Grant code, 1 byte.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F2, S2F40
/// - S3F16
/// - S4F26
/// - S13F12
/// - S14F24
/// - S16F2
/// - S19F20
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Grant {
  Granted = 0,
  Busy = 1,
  NoSpaceAvailable = 2,
  DuplicateDataID = 3,
}
singleformat_enum!{Grant, Bin}

/// ## LENGTH
/// 
/// Length of the service program or process program in bytes.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F1
/// - S7F1, S7F29
#[derive(Clone, Copy, Debug)]
pub enum Length {
  I1(i8),
  I2(i16),
  I4(i32),
  I8(i64),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat!{Length, I1, I2, I4, I8, U1, U2, U4, U8}

/// ## LOC
/// 
/// Machine material location code.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// 1 byte.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F27
/// - S3F2
pub struct LocationCode(pub u8);
singleformat!{LocationCode, Bin}

/// ## MDLN
/// 
/// Equipment Model Type, 20 bytes max.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F2], [S1F13H], [S1F13E], [S1F14H], [S1F14E]
/// - S7F22, S7F23, S7F26, S7F31, S7F39, S7F43
/// 
/// [S1F2]:   crate::messages::s1::OnLineDataEquipment
/// [S1F13H]: crate::messages::s1::HostCR
/// [S1F13E]: crate::messages::s1::EquipmentCR
/// [S1F14H]: crate::messages::s1::HostCRA
/// [S1F14E]: crate::messages::s1::EquipmentCRA
#[derive(Clone, Debug)]
pub struct ModelName(Vec<Char>);
singleformat_vec!{ModelName, Ascii, 0..=20, Char}

/// ## MID
/// 
/// Material ID.
/// 
/// Maximum 80 characters.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F27
/// - S3F2, S3F4, S3F7, S3F9, S3F12, S3F13
/// - S4F1, S4F3, S4F5, S4F7, S4F9, S4F11, S4F13, S4F15, S4F17
/// - S7F7, S7F8, S7F10, S7F11, S7F13, S7F35, S7F36
/// - S12F1, S12F3, S12F4, S12F5, S12F7, S12F9, S12F11, S12F13, S12F14, S12F15
///   S12F16, S12F17, S12F18
/// - S16F3, S16F11, S16F15
/// - S18F10, S18F11, S18F16
pub struct MaterialID(Vec<Char>);
singleformat_vec!{MaterialID, Ascii, 0..=80, Char}

/// ## NULBC
/// 
/// Null bin code value.
/// 
/// Used to indicate no die at a location.
/// 
/// Must be the same format as [BCEQU] and [BINLT].
/// 
/// Zero length indicates not used.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S12F1, S12F3, S12F4
/// 
/// [BCEQU]: BinCodeEquivalents
/// [BINLT]: BinList
pub enum NullBinCode {
  Ascii(Vec<Char>),
  U1(Vec<u8>),
}
multiformat_vec!{NullBinCode, Ascii, U1}

/// ## OBJID
/// 
/// Identifier for an object.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F19]
/// - S14F1, S14F2, S14F3, S14F4
/// 
/// [S1F19]: crate::messages::s1::GetAttribute
pub enum ObjectID {
  Ascii(Vec<Char>),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat_ascii!{ObjectID, U1, U2, U4, U8}

/// ## OBJTYPE
/// 
/// An identifier for a class of objects.
/// 
/// All objects of the same type must have the same set of attributes.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F19]
/// - S14F1, S14F3, S14F6, S14F7, S14F8, S14F9, S14F25, S14F26, S14F27
/// 
/// [S1F19]: crate::messages::s1::GetAttribute
pub enum ObjectType {
  Ascii(Vec<Char>),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat_ascii!{ObjectType, U1, U2, U4, U8}

/// ## OFLACK
/// 
/// Acknowledge code for OFF-LINE request.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F16]
/// 
/// [S1F16]: crate::messages::s1::OffLineAck
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum OffLineAcknowledge {
  Acknowledge = 0,
}
singleformat_enum!{OffLineAcknowledge, Bin}

/// ## ONLACK
/// 
/// Acknowledge code for ON-LINE request.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F18]
/// 
/// [S1F18]: crate::messages::s1::OnLineAck
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum OnLineAcknowledge {
  Accepted      = 0,
  NotAllowed    = 1,
  AlreadyOnLine = 2,
}
singleformat_enum!{OnLineAcknowledge, Bin}

/// ## PPID
/// 
/// Process Program ID
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Format
/// 
/// Maximum 120 bytes.
/// 
/// Format is host dependent. For the internal use of the equipment, it can be
/// treated as a unique binary pattern. If the equipment is not prepared to
/// display the transmitted code, it should be displayed in hexadecimal.
/// 
/// TODO: Implement format 10.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F27
/// - S7F1, S7F3, S7F5, S7F6, S7F8, S7F10, S7F11, S7F13, S7F17, S7F20, S7F23,
///   S7F25, S7F26, S7F27, S7F31, S7F33, S7F34, S7F36, S7F39, S7F43
pub struct ProcessProgramID(Vec<Char>);
singleformat_vec!{ProcessProgramID, Ascii, 0..=120, Char}

/// ## RAC
/// 
/// Reset acknowledge code, 1 byte.
/// 
/// TODO: Implement Format 31.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F20
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ResetAcknowledgeCode {
  Ok = 0,
  Denied = 1,
}
singleformat_enum!{ResetAcknowledgeCode, U1}

/// ## RCMD
/// 
/// Remote command.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F21, S2F41, S2F49
#[derive(Clone, Debug)]
pub enum RemoteCommand {
  Ascii(Vec<Char>),
  I1(i8),
  U1(u8),
}
multiformat_ascii!{RemoteCommand, I1, U1}

/// ## REPGSZ
/// 
/// Reporting group size.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F23
/// - S17F5
#[derive(Clone, Debug)]
pub enum ReportingGroupSize {
  Ascii(Vec<Char>),
  I1(i8),
  I2(i16),
  I4(i32),
  I8(i64),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat_ascii!{ReportingGroupSize, I1, I2, I4, I8, U1, U2, U4, U8}

/// ## RIC
/// 
/// Reset code, 1 byte.
/// 
/// TODO: Implement Format 31.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F19
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ResetCode {
  NotUsed = 0,
  PowerUpReset = 1,
}
singleformat_enum!{ResetCode, U1}

/// ## SFCD
/// 
/// Status form code, 1 byte.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F5], [S1F7]
/// 
/// [S1F5]: crate::messages::s1::FormattedStatusRequest
/// [S1F7]: crate::messages::s1::FixedFormRequest
pub struct StatusFormCode(pub u8);
singleformat!{StatusFormCode, Bin}

/// ## SOFTREV
/// 
/// Software Revision Code, 20 bytes max.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F2E], [S1F13H], [S1F13E], [S1F14H], [S1F14E]
/// - S7F22, S7F23, S7F26, S7F31, S7F39, S7F43
/// 
/// [S1F2E]:  crate::messages::s1::OnLineDataEquipment
/// [S1F13H]: crate::messages::s1::HostCR
/// [S1F13E]: crate::messages::s1::EquipmentCR
/// [S1F14H]: crate::messages::s1::HostCRA
/// [S1F14E]: crate::messages::s1::EquipmentCRA
#[derive(Clone, Debug)]
pub struct SoftwareRevision(Vec<Char>);
singleformat_vec!{SoftwareRevision, Ascii, 0..=20, Char}

/// ## SPAACK
/// 
/// Service program acknowledge code, 1 byte.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F4
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ServiceProgramAcknowledge {
  Ok = 0,
  InvalidData = 1,
}
singleformat_enum!{ServiceProgramAcknowledge, Bin}

/// ## SPD
/// 
/// Service program data.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F3, S2F6
#[derive(Clone, Debug)]
pub struct ServiceProgramData(pub Vec<u8>);
singleformat_vec!{ServiceProgramData, Bin}

/// ## SPID
/// 
/// Service program ID, 6 characters.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F1, S2F4, S2F7, S2F9, S2F12
#[derive(Clone, Copy, Debug)]
pub struct ServiceProgramID(pub [Char; 6]);
impl From<ServiceProgramID> for Item {
  fn from(value: ServiceProgramID) -> Self {
    let mut vec = vec![];
    vec.extend_from_slice(&value.0);
    Item::Ascii(vec)
  }
}
impl TryFrom<Item> for ServiceProgramID {
  type Error = Error;
  
  fn try_from(item: Item) -> Result<Self, Self::Error> {
    match item {
      Item::Ascii(vec) => {
        if vec.len() == 6 {
          Ok(Self(vec[0..6].try_into().unwrap()))
        } else {
          Err(WrongFormat)
        }
      },
      _ => Err(WrongFormat),
    }
  }
}

/// ## SPR
/// 
/// Service program results.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F10
pub type ServiceProgramResults = Item;

/// ## SV
/// 
/// Status variable value.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F4]
/// - S6F1
/// 
/// [S1F4]: crate::messages::s1::SelectedEquipmentStatusData
#[derive(Clone, Debug)]
pub enum StatusVariableValue {
  List(Vec<Item>),
  Bin(Vec<u8>),
  Bool(Vec<bool>),
  Ascii(Vec<Char>),
  Jis8(String),
  I1(Vec<i8>),
  I2(Vec<i16>),
  I4(Vec<i32>),
  I8(Vec<i64>),
  U1(Vec<u8>),
  U2(Vec<u16>),
  U4(Vec<u32>),
  U8(Vec<u64>),
  F4(Vec<f32>),
  F8(Vec<f64>),
}
multiformat_vec!{StatusVariableValue, List, Bin, Bool, Ascii, Jis8, I1, I2, I4, I8, U1, U2, U4, U8, F4, F8}

/// ## SVID
/// 
/// Status variable ID.
/// 
/// TODO: Add ASCII.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F3], [S1F11], [S1F12]
/// - S2F23
/// 
/// [S1F3]: crate::messages::s1::SelectedEquipmentStatusRequest
/// [S1F11]: crate::messages::s1::StatusVariableNamelistRequest
/// [S1F12]: crate::messages::s1::StatusVariableNamelistReply
#[derive(Clone, Copy, Debug)]
pub enum StatusVariableID {
  I1(i8),
  I2(i16),
  I4(i32),
  I8(i64),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat!{StatusVariableID, I1, I2, I4, I8, U1, U2, U4, U8}

/// ## SVNAME
/// 
/// Status variable name.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F12]
/// 
/// [S1F12]: crate::messages::s1::StatusVariableNamelistReply
#[derive(Clone, Debug)]
pub struct StatusVariableName(pub Vec<Char>);
singleformat_vec!{StatusVariableName, Ascii}

/// ## TBLELT
/// 
/// Table element.
/// 
/// The first table element in a row is used to identify the row.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S13F13, S13F15, S13F16
pub enum TableElement {
  List(Vec<Item>),
  Bin(Vec<u8>),
  Bool(Vec<bool>),
  Ascii(Vec<Char>),
  Jis8(String),
  I1(Vec<i8>),
  I2(Vec<i16>),
  I4(Vec<i32>),
  I8(Vec<i64>),
  U1(Vec<u8>),
  U2(Vec<u16>),
  U4(Vec<u32>),
  U8(Vec<u64>),
  F4(Vec<f32>),
  F8(Vec<f64>),
}
multiformat_vec!{TableElement, List, Bin, Bool, Ascii, Jis8, I1, I2, I4, I8, U1, U2, U4, U8, F4, F8}

/// ## TIAACK
/// 
/// Equipment acknowledge code, 1 byte.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F24
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum TraceInitializeAcknowledgeCode {
  Ok = 0,
  TooManySVID = 1,
  TooManyTraces = 2,
  InvalidPeriod = 3,
  UnknownSVID = 4,
  InvalidREPGSZ = 5,
}
singleformat_enum!{TraceInitializeAcknowledgeCode, Bin}

/// ## TIME
/// 
/// Time of day.
/// 
/// TODO: Implement specific format restrictions.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Values
/// 
/// 12-byte format:
/// - YYMMDDhhmmss
///    - YY = Year,   00 to 99
///    - MM = Month,  01 to 12
///    - DD = Day,    01 to 31
///    - hh = Hour,   00 to 23
///    - mm = Minute, 00 to 59
///    - ss = Second, 00 to 59
/// 
/// 16-byte format:
/// - YYYYMMDDhhmmsscc
///    - YYYY = Year,      0000 to 9999
///    -   MM = Month,       01 to   12
///    -   DD = Day,         01 to   31
///    -   hh = Hour,        00 to   23
///    -   mm = Minute,      00 to   59
///    -   ss = Second,      00 to   59
///    -   cc = Centisecond, 00 to   99
/// 
/// Extended format (Maximum 32 Bytes)
/// - YYYY-MM-DDThh:mm:ss.sTZD
///    - YYYY = Year,     0000 to 9999
///    -   MM = Month,      01 to   12
///    -   DD = Day,        01 to   31
///    -    T = Special Separator
///    -   hh = Hour,       00 to   23
///    -   mm = Minute,     00 to   59
///    -   ss = Second,     00 to   59
///    -   .s = Fraction,  One to Six Digits
///    -  TZD = Time Zone Designator
///       - Local Time: +hh:mm or -hh:mm
///       - UTC: Z 
/// - See SEMI E148 for more information.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F18, S2F31
#[derive(Clone, Debug)]
pub struct Time(pub Vec<Char>);
singleformat_vec!{Time, Ascii}

/// ## TOTSMP
/// 
/// Total samples to be made.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F23
/// - S17F5
#[derive(Clone, Debug)]
pub enum TotalSamples {
  Ascii(Vec<Char>),
  I1(i8),
  I2(i16),
  I4(i32),
  I8(i64),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat_ascii!{TotalSamples, I1, I2, I4, I8, U1, U2, U4, U8}

/// ## TRID
/// 
/// Trace request ID.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - S2F23
/// - S6F1, S6F27, S6F28, S6F29, S6F30
/// - S17F5, S17F6, S17F7, S17F8, S17F13, S17F14
#[derive(Clone, Debug)]
pub enum TraceRequestID {
  Ascii(Vec<Char>),
  I1(i8),
  I2(i16),
  I4(i32),
  I8(i64),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat_ascii!{TraceRequestID, I1, I2, I4, I8, U1, U2, U4, U8}

/// ## TSIP
/// 
/// Transfer status of input port, 1 byte.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F10]
/// 
/// [S1F10]: crate::messages::s1::MaterialTransferStatusData
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum TransferStatusInputPort {
  Idle            = 1,
  Prep            = 2,
  TrackOn         = 3,
  StuckInReceiver = 4,
}
singleformat_enum!{TransferStatusInputPort, Bin}

/// ## TSOP
/// 
/// Transfer status of output port, 1 byte.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F10]
/// 
/// [S1F10]: crate::messages::s1::MaterialTransferStatusData
#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum TransferStatusOutputPort {
  Idle          = 1,
  Prep          = 2,
  TrackOn       = 3,
  StuckInSender = 4,
  Completed     = 5,
}
singleformat_enum!{TransferStatusOutputPort, Bin}

/// ## UNITS
/// 
/// Units identifier.
/// 
/// TODO: Implement this variable using the units module.
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F12], [S1F22]
/// - S2F30, S2F38
/// - S7F22
/// 
/// [S1F12]: crate::messages::s1::StatusVariableNamelistReply
/// [S1F22]: crate::messages::s1::DataVariableNamelist
pub struct Units(pub Vec<Char>);
singleformat_vec!{Units, Ascii}

/// ## VID
/// 
/// Variable ID
/// 
/// -------------------------------------------------------------------------
/// 
/// #### Used By
/// 
/// - [S1F21], [S1F22], [S1F24]
/// - S2F33, S2F45, S2F46, S2F47, S2F48
/// - S6F13, S6F18, S6F22
/// - S16F9
/// - S17F1
/// 
/// [S1F21]: crate::messages::s1::DataVariableNamelistRequest
/// [S1F22]: crate::messages::s1::DataVariableNamelist
/// [S1F24]: crate::messages::s1::CollectionEventNamelist
pub enum VariableID {
  Ascii(Vec<Char>),
  I1(i8),
  I2(i16),
  I4(i32),
  I8(i64),
  U1(u8),
  U2(u16),
  U4(u32),
  U8(u64),
}
multiformat_ascii!{VariableID, I1, I2, I4, I8, U1, U2, U4, U8}
