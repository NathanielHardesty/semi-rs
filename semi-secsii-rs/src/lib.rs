//! # SEMI EQUIPMENT COMMUNICATIONS STANDARD 2 MESSAGE CONTENT (SECS-II)
//! **Based on:**
//! - **[SEMI E5]-0712**
//! 
//! Codebase will be updated to reflect more up-to-date SEMI standards if/when
//! they can be acquired for this purpose.
//! 
//! ---------------------------------------------------------------------------
//! 
//! [SECS-II] is a Presentation Protocol designed to facilitate a common
//! communications language between semiconductor equipment, particularly as
//! understood by the GEM ([SEMI E30]) Application Protocol
//! (together known as SECS/GEM). Common Session Protocols for transporting
//! [SECS-II] messages include [SECS-I] ([SEMI E4]) and [HSMS] ([SEMI E37]).
//! 
//! ---------------------------------------------------------------------------
//! 
//! [SEMI E4]:  https://store-us.semi.org/products/e00400-semi-e4-specification-for-semi-equipment-communications-standard-1-message-transfer-secs-i
//! [SEMI E5]:  https://store-us.semi.org/products/e00500-semi-e5-specification-for-semi-equipment-communications-standard-2-message-content-secs-ii
//! [SEMI E30]: https://store-us.semi.org/products/e03000-semi-e30-specification-for-the-generic-model-for-communications-and-control-of-manufacturing-equipment-gem
//! [SEMI E37]: https://store-us.semi.org/products/e03700-semi-e37-high-speed-secs-message-services-hsms-generic-services
//! 
//! [SECS-II]:  crate

#![crate_name = "secs_ii"]
#![crate_type = "lib"]

#![feature(ascii_char)]
#![feature(ascii_char_variants)]
#![allow(clippy::unusual_byte_groupings)]
#![allow(clippy::collapsible_match)]

use std::ascii::Char;

/// ## GENERIC MESSAGE
/// **Based on SEMI E5§6**
#[derive(Clone, Debug)]
pub struct Message {
  /// ### STREAM
  /// **Based on SEMI E5§6.4.2**
  /// 
  /// The message transfer protocol must be capable of identifying the
  /// [Stream] of the [Message] (0 to 127, 7 bits).
  /// 
  /// The [Stream], together with the [Function], uniquely defines a [Message].
  /// 
  /// [Message]:  Message
  /// [Stream]:   Message::stream
  /// [Function]: Message::function
  pub stream: u8,

  /// ### FUNCTION
  /// **Based on SEMI E5§6.4.2**
  /// 
  /// The message transfer protocol must be capable of identifying the
  /// [Function] of the [Message] (0 to 255, 8 bits).
  /// 
  /// The [Function], together with the [Stream], uniquely defines a [Message].
  /// 
  /// [Message]:  Message
  /// [Stream]:   Message::stream
  /// [Function]: Message::function
  pub function: u8,

  /// ### REPLY REQUESTED
  /// **Based on SEMI E5§6.4.3**
  /// 
  /// The message transfer protocol must be capable of identifying whether a
  /// reply is requested to a primary [Message].
  /// 
  /// [Message]: Message
  pub w: bool,

  /// ### MESSAGE BODY
  /// 
  /// The message's contents.
  /// 
  /// - [None] - Indicates a header-only message.
  /// - [Some] - Indicates a message with contents after the header.
  pub body: Option<Item>,
}

/// ## DATA CONVERSION ERROR
/// 
/// Represents an error in converting from a [Generic Message] to any specific
/// [Message].
/// 
/// [Message]:         messages
/// [Generic Message]: Message
pub enum Error {
  /// ### WRONG STREAM
  /// 
  /// A [Generic Message] was attempted to be converted into a specifc [Message]
  /// despite containing the wrong [Stream].
  /// 
  /// [Message]:         messages
  /// [Generic Message]: Message
  /// [Stream]:          Message::stream
  WrongStream,

  /// ### WRONG FUNCTION
  /// 
  /// A [Generic Message] was attempted to be converted into a specifc [Message]
  /// despite containing the wrong [Function].
  /// 
  /// [Message]:         messages
  /// [Generic Message]: Message
  /// [Function]:        Message::function
  WrongFunction,

  /// ### WRONG REPLY BIT
  /// 
  /// A [Generic Message] was attempted to be converted into a specifc [Message]
  /// despite containing an unacceptable [Reply Bit] value.
  /// 
  /// [Message]:         messages
  /// [Generic Message]: Message
  /// [Reply Bit]:       Message::w
  WrongReply,

  /// ### WRONG FORMAT
  /// 
  /// A [Generic Message] was attempted to be converted into a specifc [Message]
  /// despite containing an improperly formatted [Message Body].
  /// 
  /// [Message]:         messages
  /// [Generic Message]: Message
  /// [Message Body]:    Message::body
  WrongFormat,
}

/// ## GENERIC ITEM
/// **Based on SEMI E5§9**
/// 
/// An [Item] is an information packet which has a length defined by the first
/// 2, 3, or 4 bytes.
/// 
/// These first bytes are called the Item Header. The Item Header consists of
/// the Format Byte and the Length Bytes.
/// 
/// - Bits 1 to 2 of the Item Header tell how many of the following bytes
/// refer to the length of the item.
/// - The Item Length refers to the number of bytes following the Item Header,
/// called the Item Body, which is the actual data of the item.
/// - Bits 3 to 8 of the Item Header define the format of the data which
/// follows.
#[repr(u8)]
#[derive(Clone, Debug)]
pub enum Item {
  /// ### LIST
  /// **Based on SEMI E5§9.3**
  /// 
  /// - **Format Code 0o00**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// A [List] is an ordered set of elements, where elements are [Item]s.
  /// 
  /// The Item Header of a [List] is unique in that the Item Length refers to
  /// the length of the [List] in the number of [Item]s it contains, rather
  /// than the number of bytes.
  /// 
  /// [List]: self
  List(Vec<Item>) = 0b000000_00,

  /// ### BINARY
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// - **Format Code 0o10**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Single-byte quanitity where the value can be anything and does not
  /// otherwise have a strictly defined meaning.
  Binary(Vec<u8>) = 0b001000_00,

  /// ### BOOLEAN
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// - **Format Code 0o11**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Single-byte quantity where a value of 0 is equivalent to 'true' and any
  /// non-zero value is equivalent to 'false'.
  Boolean(Vec<bool>) = 0b001001_00,

  /// ### ASCII
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// - **Format Code 0o20**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// ASCII character string.
  Ascii(Vec<Char>) = 0b010000_00,

  /// ### JIS-8
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// - **Format Code 0o21**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// JIS-8 character string.
  Jis8(Vec<u8>) = 0b010001_00,

  /// ### LOCALIZED STRING
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// - **Format Code 0o22**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// 2-byte character string.
  Localized(u16, Vec< u16>) = 0b010010_00,

  /// ### 8-BYTE SIGNED INTEGER
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// - **Format Code 0o30**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// 8-byte two's compliment integer.
  Signed8(Vec<i64>) = 0b011000_00,

  /// ### 1-BYTE SIGNED INTEGER
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// - **Format Code 0o31**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// 1-byte two's compliment integer.
  Signed1(Vec<i8>) = 0b011001_00,

  /// ### 2-BYTE SIGNED INTEGER
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// - **Format Code 0o32**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// 2-byte two's compliment integer.
  Signed2(Vec<i16>) = 0b011010_00,

  /// ### 4-BYTE SIGNED INTEGER
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// - **Format Code 0o34**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// 4-byte two's compliment integer.
  Signed4(Vec<i32>) = 0b011100_00,

  /// ### 8-BYTE FLOATING POINT NUMBER
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// - **Format Code 0o40**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// 8-byte IEEE-754 floating point number.
  Float8(Vec<f64>) = 0b100000_00,

  /// ### 4-BYTE FLOATING POINT NUMBER
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// - **Format Code 0o40**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// 4-byte IEEE-754 floating point number.
  Float4(Vec<f32>) = 0b100100_00,

  /// ### 8-BYTE UNSIGNED INTEGER
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// - **Format Code 0o50**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// 8-byte integer.
  Unsigned8(Vec<u64>) = 0b101000_00,

  /// ### 1-BYTE UNSIGNED INTEGER
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// - **Format Code 0o51**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// 1-byte integer.
  Unsigned1(Vec<u8>) = 0b101001_00,

  /// ### 2-BYTE UNSIGNED INTEGER
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// - **Format Code 0o52**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// 2-byte integer.
  Unsigned2(Vec<u16>) = 0b101010_00,

  /// ### 4-BYTE UNSIGNED INTEGER
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// - **Format Code 0o54**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// 4-byte integer.
  Unsigned4(Vec<u32>) = 0b101100_00,
}

/// ## OPTIONAL LIST
/// 
/// Represents a List with either a set number of elements, or acceptably 0
/// elements in certain cases. The intent is that the type T will be a tuple
/// representing a heterogenous list of elements.
pub struct OptionList<T>(pub Option<T>);

/// ## VECTORIZED LIST
/// 
/// Represents a List with a variable number of elements of the same structure.
pub struct VecList<T>(pub Vec<T>);

/// ## ITEM -> BINARY DATA
impl From<Item> for Vec<u8> {
  fn from(item: Item) -> Self {
    let mut vec = vec![];
    match item {
      Item::List(item_vec) => {
        //Length
        let len = item_vec.len();
        if len < 256 {
          vec.push(0b000000_01);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(0b000000_10);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(0b000000_11);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Items
        for item in item_vec {
          vec.append(&mut item.into());
        }
      },
      Item::Binary(bin_vec) => {
        //Length
        let len = bin_vec.len();
        if len < 256 {
          vec.push(0b001000_01);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(0b001000_10);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(0b001000_11);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Vector
        for bin in bin_vec {
          vec.push(bin);
        }
      },
      Item::Boolean(bool_vec) => {
        //Length
        let len = bool_vec.len();
        if len < 256 {
          vec.push(0b011000_01);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(0b011000_10);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(0b011000_11);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Vector
        for bool in bool_vec {
          vec.push(bool as u8);
        }
      },
      Item::Ascii(ascii_vec) => {
        //Length
        let len = ascii_vec.len();
        if len < 256 {
          vec.push(0b010000_01);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(0b010000_10);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(0b010000_11);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Vector
        for ascii in ascii_vec {
          vec.push(ascii as u8);
        }
      },
      Item::Jis8(_jis8_vec) => {
        todo!()
      },
      Item::Localized(_widechar_format, _widechar_vec) => {
        todo!()
      },
      Item::Signed8(i8_vec) => {
        //Length
        let len = i8_vec.len() * 8;
        if len < 256 {
          vec.push(0b011000_01);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(0b011000_10);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(0b011000_11);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Vector
        for i8 in i8_vec {
          vec.extend_from_slice(&i8.to_be_bytes());
        }
      },
      Item::Signed1(i1_vec) => {
        //Length
        let len = i1_vec.len();
        if len < 256 {
          vec.push(0b011001_01);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(0b011001_10);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(0b011001_11);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Vector
        for i1 in i1_vec {
          vec.extend_from_slice(&i1.to_be_bytes());
        }
      },
      Item::Signed2(i2_vec) => {
        //Length
        let len = i2_vec.len() * 2;
        if len < 256 {
          vec.push(0b011010_01);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(0b011010_10);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(0b011010_11);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Vector
        for i2 in i2_vec {
          vec.extend_from_slice(&i2.to_be_bytes());
        }
      },
      Item::Signed4(i4_vec) => {
        //Length
        let len = i4_vec.len() * 4;
        if len < 256 {
          vec.push(0b011100_01);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(0b011100_10);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(0b011100_11);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Vector
        for i4 in i4_vec {
          vec.extend_from_slice(&i4.to_be_bytes());
        }
      },
      Item::Float8(f8_vec) => {
        //Length
        let len = f8_vec.len() * 8;
        if len < 256 {
          vec.push(0b011000_01);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(0b011000_10);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(0b011000_11);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Vector
        for f8 in f8_vec {
          vec.extend_from_slice(&f8.to_be_bytes());
        }
      },
      Item::Float4(f4_vec) => {
        //Length
        let len = f4_vec.len() * 4;
        if len < 256 {
          vec.push(0b011000_01);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(0b011000_10);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(0b011000_11);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Vector
        for f4 in f4_vec {
          vec.extend_from_slice(&f4.to_be_bytes());
        }
      },
      Item::Unsigned8(u8_vec) => {
        //Length
        let len = u8_vec.len() * 8;
        if len < 256 {
          vec.push(0b101000_01);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(0b101000_10);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(0b101000_11);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Vector
        for u8 in u8_vec {
          vec.extend_from_slice(&u8.to_be_bytes());
        }
      },
      Item::Unsigned1(u1_vec) => {
        //Length
        let len = u1_vec.len();
        if len < 256 {
          vec.push(0b101001_01);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(0b101001_10);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(0b101001_11);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Vector
        for u1 in u1_vec {
          vec.push(u1);
        }
      },
      Item::Unsigned2(u2_vec) => {
        //Length
        let len = u2_vec.len() * 2;
        if len < 256 {
          vec.push(0b101010_01);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(0b101010_10);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(0b101010_11);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Vector
        for u2 in u2_vec {
          vec.extend_from_slice(&u2.to_be_bytes());
        }
      },
      Item::Unsigned4(u4_vec) => {
        //Length
        let len = u4_vec.len() * 4;
        if len < 256 {
          vec.push(0b101100_01);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(0b101100_10);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(0b101100_11);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Vector
        for u4 in u4_vec {
          vec.extend_from_slice(&u4.to_be_bytes());
        }
      },
    }
    vec
  }
}

/// ## BINARY DATA -> ITEM
impl TryFrom<Vec<u8>> for Item {
  type Error = Error;

  fn try_from(_value: Vec<u8>) -> Result<Self, Self::Error> {
    todo!()
  }
}

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

// TODO: ITEM -> HETEROGENEOUS LIST, UP TO 15 ELEMENTS
// NOTE: To implement Stream 1, only lengths of 2 and 3 are required.

/// ## EMPTY LIST -> ITEM
impl From<()> for Item {
  fn from(_empty_list: ()) -> Self {
    Item::List(vec![])
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

// TODO: HETEROGENEOUS LIST -> ITEM, UP TO 15 ELEMENTS
// NOTE: To implement Stream 1, only lengths of 2 and 3 are required.

/// ## LOCALIZED STRING HEADER
/// **Based on SEMI E5§9.4**
#[repr(u16)]
#[derive(Clone, Copy, Debug)]
pub enum LocalizedStringHeader {
  //Universal
  Ucs2 = 1,
  Utf8 = 2,
  //Latin
  Iso646_1991 = 3, //7-bit ASCII
  Iso8859_1 = 4, //ISO Latin-1, Western Europe
  //Thai
  Iso8859_11 = 5,
  Tis620 = 6,
  //Indian
  Is13194_1991 = 7, //ISCII
  //Japanese
  ShiftJis = 8,
  EucJp = 9,
  //Korean
  EucKr = 10,
  //Simplified Chinese
  Gb = 11,
  EucCn = 12,
  //Traditional Chinese
  Big5 = 13,
  EucTw = 14,
}

/// # ITEMS
/// **Based on SEMI E5§9.6**
pub mod items {
  use crate::Item;
  use crate::Error::{self, *};
  use std::ascii::Char;
  use num_enum::{IntoPrimitive, TryFromPrimitive};

  /// ## DATA ITEM MACRO: SINGLE ACCEPTED FORMAT, VECTOR LENGTH 1
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

  /// ## DATA ITEM MACRO: SINGLE ACCEPTED FORMAT, ANY VECTOR LENGTH
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

  /// ## DATA ITEM MACRO: SINGLE ACCEPTED FORMAT, ENUMERATED VALUE
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
    }
  }

  /// ## DATA ITEM MACRO: MULTIPLE ACCCEPTED FORMATS, VECTOR LENGTH 1
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

  /// ## DATA ITEM MACRO: MULTIPLE ACCEPTED FORMATS, ANY VECTOR LENGTH
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

  /// ## ABS
  /// 
  /// Any binary string.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### Used By
  /// 
  /// - [S2F25], [S2F26]
  #[derive(Clone, Debug)]
  pub struct AnyBinaryString(Vec<u8>);
  singleformat_vec!{AnyBinaryString, Binary, 0.., u8}

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
  /// -------------------------------------------------------------------------
  /// 
  /// #### Used By
  /// 
  /// - [S5F1], [S5F6], [S5F8]
  #[derive(Clone, Copy, Debug)]
  pub struct AlarmCode(pub u8);
  singleformat!{AlarmCode, Binary}

  /// ## MDLN
  /// 
  /// Equipment Model Type, 20 bytes max.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### Used By
  /// 
  /// - [S1F2], [S1F13], [S1F14]
  /// - [S7F22], [S7F23], [S7F26], [S7F31], [S7F39], [S7F43]
  /// 
  /// [S1F2]:  crate::messages::s1::EquipmentOnLineData
  /// [S1F14]: crate::messages::s1::EquipmentCRA
  #[derive(Clone, Debug)]
  pub struct ModelName(Vec<Char>);
  singleformat_vec!{ModelName, Ascii, 0..=20, Char}

  /// ## SOFTREV
  /// 
  /// Software Revision Code, 20 bytes max.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### Used By
  /// 
  /// - [S1F2], [S1F13], [S1F14]
  /// - [S7F22], [S7F23], [S7F26], [S7F31], [S7F39], [S7F43]
  /// 
  /// [S1F2]:  crate::messages::s1::EquipmentOnLineData
  /// [S1F14]: crate::messages::s1::EquipmentCRA
  #[derive(Clone, Debug)]
  pub struct SoftwareRevision(Vec<Char>);
  singleformat_vec!{SoftwareRevision, Ascii, 0..=20, Char}

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
    Accepted = 0,
    Denied   = 1,
  }
  singleformat_enum!{CommAck, Binary}

  /// ## ERRCODE
  /// 
  /// Code identifying an error.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### Used By
  /// 
  /// - [S1F20]
  /// - [S3F18], [S3F20], [S3F22], [S3F24], [S3F26], [S3F28], [S3F30], [S3F32],
  ///   [S3F34], [S3F36]
  /// - [S4F20], [S4F22], [S4F23], [S4F31], [S4F33]
  /// - [S5F14], [S5F15], [S5F18]
  /// - [S6F25], [S6F30]
  /// - [S13F14], [S13F16]
  /// - [S14F2], [S14F4], [S14F5], [S14F6], [S14F8], [S14F10], [S14F12],
  ///   [S14F14], [S14F16], [S14F18], [S14F20], [S14F21], [S14F26], [S14F28]
  /// - [S15F4], [S15F6], [S15F8], [S15F10], [S15F12], [S15F14], [S15F16],
  ///   [S15F18], [S15F20], [S15F22], [S15F24], [S15F26], [S15F28], [S15F30],
  ///   [S15F32], [S15F34], [S15F36], [S15F38], [S15F40], [S15F42], [S15F44],
  ///   [S15F48], [S15F53]
  /// - [S16F4], [S16F6], [S16F7], [S16F12], [S16F16], [S16F18], [S16F24],
  ///   [S16F26], [S16F28]
  /// - [S17F2], [S17F4], [S17F6], [S17F8], [S17F10], [S17F12], [S17F14]
  #[repr(u32)]
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
  singleformat_enum!{OffLineAcknowledge, Binary}

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
  singleformat_enum!{OnLineAcknowledge, Binary}

  /// ## SFCD
  /// 
  /// Status form code, 1 byte.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### Used By
  /// 
  /// - [S1F5], [S1F7]
  pub struct StatusFormCode(pub u8);
  singleformat!{StatusFormCode, Binary}

  /// ## SV
  /// 
  /// Status variable value.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### Used By
  /// 
  /// - [S1F4]
  /// - [S6F1]
  /// 
  /// [S1F4]: crate::messages::s1::SelectedEquipmentStatusData
  pub enum StatusVariableValue {
    List      (Vec<Item>),
    Binary    (Vec<u8  >),
    Boolean   (Vec<bool>),
    Ascii     (Vec<Char>),
    Jis8      (Vec<u8  >),
    Signed1   (Vec<i8  >),
    Signed2   (Vec<i16 >),
    Signed4   (Vec<i32 >),
    Signed8   (Vec<i64 >),
    Unsigned1 (Vec<u8  >),
    Unsigned2 (Vec<u16 >),
    Unsigned4 (Vec<u32 >),
    Unsigned8 (Vec<u64 >),
    Float4    (Vec<f32 >),
    Float8    (Vec<f64 >),
  }
  multiformat_vec!{
    StatusVariableValue,
    List,
    Binary, Boolean,
    Ascii, Jis8,
    Signed1, Signed2, Signed4, Signed8,
    Unsigned1, Unsigned2, Unsigned4, Unsigned8,
    Float4, Float8,
  }

  /// ## SVID
  /// 
  /// Status variable ID.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### Used By
  /// 
  /// - [S1F3], [S1F11], [S1F12]
  /// - [S2F23]
  /// 
  /// [S1F3]: crate::messages::s1::SelectedEquipmentStatusRequest
  /// [S1F11]: crate::messages::s1::StatusVariableNamelistRequest
  /// [S1F12]: crate::messages::s1::StatusVariableNamelistReply
  pub enum StatusVariableID {
    Binary    (u8 ),
    Signed1   (i8 ),
    Signed2   (i16),
    Signed4   (i32),
    Signed8   (i64),
    Unsigned1 (u8 ),
    Unsigned2 (u16),
    Unsigned4 (u32),
    Unsigned8 (u64),
  }
  multiformat!{
    StatusVariableID,
    Binary,
    Signed1, Signed2, Signed4, Signed8,
    Unsigned1, Unsigned2, Unsigned4, Unsigned8,
  }

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
  pub struct StatusVariableName(pub Vec<Char>);
  singleformat_vec!{StatusVariableName, Ascii}

  /// ## TSIP
  /// 
  /// Transfer status of input port, 1 byte.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### Used By
  /// 
  /// - [S1F10]
  #[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
  #[repr(u8)]
  pub enum TransferStatusInputPort {
    Idle            = 1,
    Prep            = 2,
    TrackOn         = 3,
    StuckInReceiver = 4,
  }
  singleformat_enum!{TransferStatusInputPort, Binary}

  /// ## TSOP
  /// 
  /// Transfer status of output port, 1 byte.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### Used By
  /// 
  /// - [S1F10]
  #[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
  #[repr(u8)]
  pub enum TransferStatusOutputPort {
    Idle          = 1,
    Prep          = 2,
    TrackOn       = 3,
    StuckInSender = 4,
    Completed     = 5,
  }
  singleformat_enum!{TransferStatusOutputPort, Binary}

  /// ## UNITS
  /// 
  /// Units identifier.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### Used By
  /// 
  /// - [S1F12], [S1F22]
  /// - [S2F30], [S2F38]
  /// - [S7F22]
  pub struct Units(pub Vec<Char>);
  singleformat_vec!{Units, Ascii}
  //TODO: Implement this variable using the units module rather than a raw Vec.
}

/// # MESSAGES
/// **Based on SEMI E5§10**
pub mod messages {
  /// ## MESSAGE MACRO: HEADER ONLY
  /// 
  /// To be used with particular messages that contain only a header.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Expands into two impls:
  /// 
  /// - From<$name> for Message
  /// - TryFrom<Message> for $name
  macro_rules! message_headeronly {
    (
      $name:ident,
      $w:expr,
      $stream:expr,
      $function:expr
    ) => {
      impl From<$name> for Message {
        fn from(_value: $name) -> Self {
          Message {
            stream:   $stream,
            function: $function,
            w:        $w,
            body:     None,
          }
        }
      }
      impl TryFrom<Message> for $name {
        type Error = Error;

        fn try_from(message: Message) -> Result<Self, Self::Error> {
          if message.stream   != $stream   {return Err(WrongStream)}
          if message.function != $function {return Err(WrongFunction)}
          if message.w        != $w        {return Err(WrongReply)}
          match message.body {
            None => Ok($name),
            Some(_item) => Err(WrongFormat),
          }
        }
      }
    }
  }

  /// ## MESSAGE MACRO: DATA
  /// 
  /// To be used with particular messages that contain arbitrary data.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Expands into two impls:
  /// 
  /// - From<$name> for Message
  /// - TryFrom<Message> for $name
  macro_rules! message_data {
    (
      $name:ident,
      $w:expr,
      $stream:expr,
      $function:expr
    ) => {
      impl From<$name> for Message {
        fn from(value: $name) -> Self {
          Message {
            stream:   $stream,
            function: $function,
            w:        $w,
            body:     Some(value.0.into()),
          }
        }
      }
      impl TryFrom<Message> for $name {
        type Error = Error;

        fn try_from(message: Message) -> Result<Self, Self::Error> {
          if message.stream   != $stream   {return Err(WrongStream)}
          if message.function != $function {return Err(WrongFunction)}
          if message.w        != $w        {return Err(WrongReply)}
          match message.body {
            Some(item) => {Ok(Self(item.try_into()?))},
            None => Err(WrongFormat),
          }
        }
      }
    }
  }

  /// ## MESSAGE MACRO: ITEM
  /// 
  /// To be used with particular messages that contain just an Item.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Expands into two impls:
  /// 
  /// - From<$name> for Message
  /// - TryFrom<Message> for $name
  macro_rules! message_item {
    (
      $name:ident,
      $w:expr,
      $stream:expr,
      $function:expr
    ) => {
      impl From<$name> for Message {
        fn from(value: $name) -> Self {
          Message {
            stream:   $stream,
            function: $function,
            w:        $w,
            body:     Some(value.0.into()),
          }
        }
      }
      impl TryFrom<Message> for $name {
        type Error = Error;

        fn try_from(message: Message) -> Result<Self, Self::Error> {
          if message.stream   != $stream   {return Err(WrongStream)}
          if message.function != $function {return Err(WrongFunction)}
          if message.w        != $w        {return Err(WrongReply)}
          match message.body {
            Some(item) => {Ok(Self(item))},
            None => Err(WrongFormat),
          }
        }
      }
    }
  }

  /// # STREAM 1: EQUIPMENT STATUS
  /// **Based on SEMI E5§10.5**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Message]s which deal with exchanging information about the status of the
  /// equipment, including its current mode, depletion of various consumable
  /// items, and the status of transfer operations.
  /// 
  /// [Message]: crate::Message
  pub mod s1 {
    use crate::*;
    use crate::Error::*;
    use crate::items::*;

    /// ## S1F1
    /// 
    /// **Are You There Request (R)**
    /// 
    /// - **SINGLE-BLOCK**
    /// - **HOST <-> EQUIPMENT**
    /// - **REPLY REQUIRED**
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// Establishes if the equipment is on-line. A function 0 response to this
    /// message is equivalent to receiving a timeout on the receive timer.
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// #### Structure
    /// 
    /// Header only.
    pub struct AreYouThere;
    message_headeronly!{AreYouThere, true, 1, 1}

    /// ## S1F2
    /// 
    /// **On Line Data (D)**
    /// 
    /// - **SINGLE-BLOCK**
    /// - **HOST -> EQUIPMENT**
    /// - **REPLY NEVER**
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// Data signifying the equipment is alive.
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// #### Structure
    /// 
    /// - List - 0
    pub struct HostOnLineData(pub ());
    message_data!{HostOnLineData, false, 1, 2}

    /// ## S1F2
    /// 
    /// **On Line Data (D)**
    /// 
    /// - **SINGLE-BLOCK**
    /// - **HOST <- EQUIPMENT**
    /// - **REPLY NEVER**
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// Data signifying the equipment is alive.
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// #### Structure
    /// 
    /// - List - 2
    ///   1. [MDLN]
    ///   2. [SOFTREV]
    /// 
    /// [MDLN]:    ModelName
    /// [SOFTREV]: SoftwareRevision
    pub struct EquipmentOnLineData(pub (ModelName, SoftwareRevision));
    message_data!{EquipmentOnLineData, false, 1, 2}

    /// ## S1F3
    /// 
    /// **Selected Equipment Status Request (SSR)**
    /// 
    /// - **SINGLE-BLOCK**
    /// - **HOST -> EQUIPMENT**
    /// - **REPLY REQUIRED**
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// A request to the equipment to report selected values of its status.
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// #### Structure
    /// 
    /// - List - n
    ///   - [SVID]
    /// 
    /// A zero-length list means to report all SVIDs.
    /// 
    /// [SVID]: StatusVariableID
    pub struct SelectedEquipmentStatusRequest(pub VecList<StatusVariableID>);
    message_data!{SelectedEquipmentStatusRequest, true, 1, 3}

    /// ## S1F4
    /// 
    /// **Selected Equipment Status Data (SSD)**
    /// 
    /// - **MULTI-BLOCK**
    /// - **HOST <- EQUIPMENT**
    /// - **REPLY NEVER**
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// The equipment reports the value of each SVID requested in the order
    /// requested.
    /// 
    /// The host must remember the names of the values it requested.
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// #### Structure
    /// 
    /// - List - n
    ///   - [SV]
    /// 
    /// A zero-length item for a given [SV] means that the [SVID] does not
    /// exist.
    /// 
    /// [SV]:   StatusVariableValue
    /// [SVID]: StatusVariableID
    pub struct SelectedEquipmentStatusData(pub VecList<StatusVariableValue>);
    message_data!{SelectedEquipmentStatusData, false, 1, 4}

    /// ## S1F5
    /// 
    /// **Formatted Status Request (FSR)**
    /// 
    /// - **SINGLE-BLOCK**
    /// - **HOST -> EQUIPMENT**
    /// - **REPLY REQUIRED**
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// A request for the equipment to report the status according to a
    /// predefined fixed format.
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// #### Structure
    /// 
    /// - [SFCD]
    /// 
    /// [SFCD]: StatusFormCode
    pub struct FormattedStatusRequest(pub StatusFormCode);
    message_data!{FormattedStatusRequest, true, 1, 5}

    /// ## S1F6
    /// 
    /// **Formatted Status Data (FSD)**
    /// 
    /// - **MULTI-BLOCK**
    /// - **HOST <- EQUIPMENT**
    /// - **REPLY NEVER**
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// The value of status variables according to the [SFCD].
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// #### Structure
    /// 
    /// Depends on the structure specified by the status form.
    /// 
    /// A zero-length item means that no report can be made.
    /// 
    /// [SFCD]: StatusFormCode
    pub struct FormattedStatusData(pub Item);
    message_item!{FormattedStatusData, false, 1, 6}

    /// ## S1F7
    /// 
    /// **Fixed Form Request (FFR)**
    /// 
    /// - **SINGLE-BLOCK**
    /// - **HOST -> EQUIPMENT**
    /// - **REPLY REQUIRED**
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// A request for the form used in [S1F6].
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// #### Structure
    /// 
    /// - [SFCD]
    /// 
    /// [S1F6]: FormattedStatusData
    /// [SFCD]: StatusFormCode
    pub struct FixedFormRequest(pub StatusFormCode);
    message_data!{FixedFormRequest, true, 1, 7}

    /// ## S1F8
    /// 
    /// **Fixed Form Data (FFD)**
    /// 
    /// - **MULTI-BLOCK**
    /// - **HOST <- EQUIPMENT**
    /// - **REPLY NEVER**
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// The form is returned with the name of each value and the data format
    /// item having a zero length as a two-element list in the place of each
    /// single item to be returned in [S1F6].
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// #### Structure
    /// 
    /// Depends on the form being specified.
    /// 
    /// A zero-length item means the form is unavailable.
    /// 
    /// [S1F6]: FormattedStatusData
    pub struct FixedFormData(pub Item);
    message_item!{FixedFormData, false, 1, 8}

    /// ## S1F9
    /// 
    /// **Material Transfer Status Request (TSR)**
    /// 
    /// - **SINGLE-BLOCK**
    /// - **HOST -> EQUIPMENT**
    /// - **REPLY REQUIRED**
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// A request to report the status of all material ports.
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// #### Structure
    /// 
    /// Header only.
    pub struct MaterialTransferStatusRequest;
    message_headeronly!{MaterialTransferStatusRequest, true, 1, 9}

    /// ## S1F10
    /// 
    /// **Material Transfer Status data (TSD)**
    /// 
    /// - **MULTI-BLOCK**
    /// - **HOST <- EQUIPMENT**
    /// - **REPLY NEVER**
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// The transfer status of all material ports.
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// #### Structure
    /// 
    /// - List - 2
    ///   1. <[TSIP]...>
    ///   2. <[TSOP]...>
    /// 
    /// A zero length item means there are no such ports.
    /// A zero length list means there are no ports.
    /// 
    /// [TSIP]: TransferStatusInputPort
    /// [TSOP]: TransferStatusOutputPort
    pub struct MaterialTransferStatusData;
    //TODO: Implement this message.

    /// ## S1F11
    /// 
    /// **Status Variable Namelist Request (SVNR)**
    /// 
    /// - **SINGLE-BLOCK**
    /// - **HOST -> EQUIPMENT**
    /// - **REPLY REQUIRED**
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// A request to identify certain status variables.
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// #### Structure
    /// 
    /// - List - n
    ///   - [SVID]
    /// 
    /// A zero-length list means to report all SVIDs.
    /// 
    /// [SVID]: StatusVariableID
    pub struct StatusVariableNamelistRequest(pub VecList<StatusVariableID>);
    message_data!{StatusVariableNamelistRequest, true, 1, 11}

    /// ## S1F12
    /// 
    /// **Status Variable Namelist Reply (SVNRR)**
    /// 
    /// - **MULTI-BLOCK**
    /// - **HOST <- EQUIPMENT**
    /// - **REPLY NEVER**
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// The name and units of the requested status variables.
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// #### Structure
    /// 
    /// - List - n
    ///   - List - 3
    ///     1. [SVID]
    ///     2. [SVNAME]
    ///     3. [UNITS]
    /// 
    /// Zero length items for both SVNAME and UNITS indicates that the SVID
    /// does not exist.
    pub struct StatusVariableNamelistReply(pub VecList<(StatusVariableID, StatusVariableName, Units)>);
    message_data!{StatusVariableNamelistReply, false, 1, 12}

    /// ## S1F13
    /// 
    /// **Establish Communications Request (CR)**
    /// 
    /// - **SINGLE-BLOCK**
    /// - **HOST -> EQUIPMENT**
    /// - **REPLY REQUIRED**
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// The purpose of this message is to provide a formal means of
    /// initializing communications at a logical application level both on
    /// power-up and following a break in communications.
    /// 
    /// It should follow any period where host and equipment SECS applications
    /// are unable to communicate.
    /// 
    /// An attempt to send an Establish Communications Request ([S1F13])
    /// should be repeated at programmable intervals until an Establish
    /// Communications Acknowledge ([S1F14]) is received within the
    /// transaction timeout period with an acknowledgement code accepting the
    /// establishment.
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// #### Structure
    /// 
    /// - List - 0
    /// 
    /// [S1F13]: HostCR
    /// [S1F14]: EquipmentCRA
    pub struct HostCR(pub ());
    message_data!{HostCR, true, 1, 13}

    /// ## S1F14
    /// 
    /// **Establish Communications Request Acknowledge (CRA)**
    /// 
    /// - **SINGLE-BLOCK**
    /// - **HOST <- EQUIPMENT**
    /// - **REPLY NEVER**
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// Accept or deny Establish Communications Request ([S1F13]).
    /// 
    /// [MDLN] and [SOFTREV] are on-line data and are valid only if
    /// [COMMACK] = 0.
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// #### Structure
    /// 
    /// - List - 2
    ///   1. [COMMACK]
    ///   2. List - 2
    ///      1. [MDLN]
    ///      2. [SOFTREV]
    /// 
    /// [S1F13]:   HostCR
    /// [COMMACK]: CommAck
    /// [MDLN]:    ModelName
    /// [SOFTREV]: SoftwareRevision
    pub struct EquipmentCRA(pub (CommAck, (ModelName, SoftwareRevision)));
    message_data!{EquipmentCRA, true, 1, 14}

    /// ## S1F15
    /// 
    /// **Request OFF-LINE (ROFL)**
    /// 
    /// - **SINGLE-BLOCK**
    /// - **HOST -> EQUIPMENT**
    /// - **REPLY REQUIRED**
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// The host requirests that the equipment transition to the OFF-LINE
    /// state.
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// #### Structure
    /// 
    /// Header only.
    pub struct RequestOffLine;
    message_headeronly!{RequestOffLine, true, 1, 15}

    /// ## S1F16
    /// 
    /// **OFF-LINE Acknowledge (OFLA)**
    ///  
    /// - **SINGLE-BLOCK**
    /// - **HOST <- EQUIPMENT**
    /// - **REPLY NEVER**
    /// 
    /// ---------------------------------------------------------------------
    /// 
    /// Acknowledge or error.
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// #### Structure
    /// 
    /// [OFLACK]
    /// 
    /// [OFLACK]: OffLineAcknowledge
    pub struct OffLineAck(pub OffLineAcknowledge);
    message_data!{OffLineAck, false, 1, 16}

    /// ## S1F17
    /// 
    /// **Request ON-LINE (RONL)**
    /// 
    /// - **SINGLE-BLOCK**
    /// - **HOST -> EQUIPMENT**
    /// - **REPLY REQUIRED**
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// The host requirests that the equipment transition to the OM-LINE
    /// state.
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// #### Structure
    /// 
    /// Header only.
    pub struct RequestOnLine;
    message_headeronly!{RequestOnLine, true, 1, 17}
    
    /// ## S1F18
    /// 
    /// **ON-LINE Acknowledge (ONLA)**
    ///  
    /// - **SINGLE-BLOCK**
    /// - **HOST <- EQUIPMENT**
    /// - **REPLY NEVER**
    /// 
    /// ---------------------------------------------------------------------
    /// 
    /// Acknowledge or error.
    /// 
    /// -----------------------------------------------------------------------
    /// 
    /// #### Structure
    /// 
    /// [ONLACK]
    /// 
    /// [ONLACK]: OnLineAcknowledge
    pub struct OnLineAck(pub OnLineAcknowledge);
    message_data!{OnLineAck, false, 1, 16}
  }
  //TODO: Complete filling out stream's contents.

  /// # STREAM 2: EQUIPMENT CONTROL AND DIAGNOSTICS
  /// **Based on SEMI E5§10.6**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Message]s which deal with control of the equipment from the host.
  /// 
  /// This includes all remote operations and equipment self-diagnostics and
  /// calibration but specifically excluses:
  /// 
  /// - Control operations associated with material transfer ([Stream 4]).
  /// - Loading of executive and boot programs ([Stream 8]).
  /// - File and operating system calls ([Stream 10], [Stream 13]).
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// This functionality continues in [Stream 17].
  /// 
  /// [Message]: crate::Message
  /// [Stream 4]: crate::messages::s4
  /// [Stream 8]: crate::messages::s8
  /// [Stream 10]: crate::messages::s10
  /// [Stream 13]: crate::messages::s13
  /// [Stream 17]: crate::messages::s17
  pub mod s2 {}
  //TODO: Fill out stream's contents.

  /// # STREAM 3: MATERIAL STATUS
  /// **Based on SEMI E5§10.7**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Message]s which deal with communicating information and actions related
  /// to material, including carriers and material-in-process,
  /// time-to-completion information, and extraordinary material circumstances.
  /// 
  /// [Message]: crate::Message
  pub mod s3 {}
  //TODO: Fill out stream's contents.

  /// # STREAM 4: MATERIAL CONTROL
  /// **Based on SEMI E5§10.8**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Message]s which deal with the original material control protocol and the
  /// newer protocol which supports [SEMI E32].
  /// 
  /// [Message]: crate::Message
  pub mod s4 {}
  //TODO: Fill out stream's contents.

  /// # STREAM 5: EXCEPTION HANDLING
  /// **Based on SEMI E5§10.9**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Message]s which deal with binary and analog equipment exceptions.
  /// 
  /// Exceptions are classified into two categories: Errors and Alarms
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Message]s [S5F1] through [S5F8] provide basic alarm messages, which may
  /// be divided into the following categories:
  /// 
  /// - Personal Safety - Condition may be dangerous to people.
  /// - Equipment Safety - Condition may harm equipment.
  /// - Parameter Control Warning - Parameter variation outside of preset
  /// limits - may harm product.
  /// - Parameter Control Error - Parameter variation outside of reasonable
  /// control limits - may indicate an equipment malfunction.
  /// - Irrecoverable Error - Intervention required before normal use of
  /// equipment can resume.
  /// - Equipment Status Warning - An unexpected condition has occurred, but
  /// operation can continue.
  /// - Attention Flags - A signal from a process program indicating that a
  /// particular step has been reached.
  /// - Data Integrity - A condition which may cause loss of data; usually
  /// related to [Stream 6].
  /// 
  /// It will be the equipment's responsibility to categorize alarms.
  /// 
  /// Some alarm conditions may cause more than one type of alarm to be issued.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Message]s [S5F9] through [S5F15] provide extended capabilities for
  /// exception handling.
  /// 
  /// [Message]: crate::Message
  /// [Stream 6]: crate::messages::s6
  pub mod s5 {}
  //TODO: Fill out stream's contents.

  /// # STREAM 6: DATA COLLECTION
  /// **Based on SEMI E5§10.10**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Message]s which deal with in-process measurement and equipment
  /// monitoring.
  /// 
  /// [Message]: crate::Message
  pub mod s6 {}
  //TODO: Fill out stream's contents.

  /// # STREAM 7: PROCESS PROGRAM MANAGEMENT
  /// **Based on SEMI E5§10.11**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Message]s which deal with the management and transfer of Process Programs.
  /// 
  /// Process Programs are the equipment-specific descriptions that determine
  /// the procedure to be conducted on the material by a single piece of
  /// equipment.
  /// 
  /// Methods are provided to transfer programs as well as establish the link
  /// between the process program and the material to be processed with that
  /// program.
  /// 
  /// [Message]: crate::Message
  pub mod s7 {}
  //TODO: Fill out stream's contents.

  /// # STREAM 8: CONTROL PROGRAM TRANSFER
  /// **Based on SEMI E5§10.12**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Message]s which deal with transmitting the programs used in the equipment
  /// to perform the control function or to execute the transmitted Process
  /// Program.
  /// 
  /// [Message]: crate::Message
  pub mod s8 {}
  //TODO: Fill out stream's contents.

  /// # STREAM 9: SYSTEM ERRORS
  /// **Based on SEMI E5§10.13**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Message]s which deal with informing the host of communication errors,
  /// particularly that a message block has been received which cannot be
  /// handled or that a timeout on a transaction reception timer has occurred.
  /// 
  /// The messages indicate either a Message Fault or a Communications Fault
  /// has occurred but do not indicate a Communications Failure has occurred.
  /// 
  /// [Message]: crate::Message
  pub mod s9 {}
  //TODO: Fill out stream's contents.

  /// # STREAM 10: TERMINAL SERVICES
  /// **Based on SEMI E5§10.14**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Message]s which deal with passing textual messages between operator
  /// terminals attached to processing or testing equipment and the host.
  /// 
  /// The equipment makes no attempt to interpret the text of the message, but
  /// merely passes it from terminal keyboard to the host or from the host to
  /// the display of the terminal.
  /// 
  /// Management of human response times to information displayed on terminals
  /// is the responsibility of the host.
  /// 
  /// [Message]: crate::Message
  pub mod s10 {}
  //TODO: Fill out stream's contents.

  /// # STREAM 11: DELETED
  /// **Based on SEMI E5§10.15**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Message]s in this stream have been deprecated and no longer appear
  /// in the standard as of 1989.
  /// 
  /// [Message]: crate::Message
  pub mod s11 {}

  /// # STREAM 12: WAFER MAPPING
  /// **Based on SEMI E5§10.16**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Message]s which deal with coordinate positions and data associated with
  /// those positions.
  /// 
  /// This includes functions such as wafer mapping with the coordinates of die
  /// on wafer maps to and from the process equipment.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [S12F1] through [S12F20] address the variations required by semiconductor
  /// equipment manufactureers in transmitting wafer maps to and from the
  /// process equipment.
  /// 
  /// The functions include three basic formats:
  /// 
  /// - Row/Column - A coordinate row starting position is given with die count
  /// in the row and starting direction. The respective binning information
  /// follows each die.
  /// - Array - A matrix array captures all or part of a wafer with the
  /// associated binning information.
  /// - Coordinate - An X/Y location and bin code for die on the wafer.
  /// 
  /// [Message]: crate::Message
  pub mod s12 {}
  //TODO: Fill out more documentation here.
  //TODO: Fill out stream's contents.

  /// # STREAM 13: DATA SET TRANSFER
  /// **Based on SEMI E5§10.17**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Message]s which deal with the transfer of data sets between systems.
  /// 
  /// It is not intended to provide a general file access mechanism.
  /// 
  /// [Message]: crate::Message
  pub mod s13 {}
  //TODO: Fill out more documentation here.
  //TODO: Fill out stream's contents.

  /// # STREAM 14: OBJECT SERVICES
  /// **Based on SEMI E5§10.18**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Message]s which deal with generic functions concerning objects,
  /// including obtaining information about objects and setting values for an
  /// object.
  /// 
  /// [Message]: crate::Message
  pub mod s14 {}
  //TODO: Fill out stream's contents.

  /// # STREAM 15: RECIPE MANAGEMENT
  /// **Based on SEMI E5§10.19**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Message]s which deal with requestion information and operations
  /// concerning recipes, recipe namespaces, and recipe executors.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// A recipe is an object that is transferred in sections, where a section
  /// consists of either recipe attributes, agent-specific dataset attributes,
  /// or the body of the recipe.
  /// 
  /// An attribute is information concerning the recipe body, the recipe as a
  /// whole, or the application of the recipe, and consists of a name/value
  /// pair.
  /// 
  /// [Message]: crate::Message
  pub mod s15 {}
  //TODO: Fill out stream's contents.

  /// # STREAM 16: PROCESSING MANAGEMENT
  /// **Based on SEMI E5§10.20**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Message]s which deal with control of material processing at equipment
  /// and equipment resources.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Control is implemented by supporting two job types; the control job and
  /// the process job.
  /// 
  /// A process job is a single unit of work that ensures that the appropriate
  /// processing is applied to a particular material by a processing resource.
  /// It provides a widely applicable supervisory control capability for
  /// automated processing of material in equipment, irrespective of the
  /// particular process being used. It also creates a transient link between
  /// the three elements of the manufacturing process (material, equipment,
  /// and recipe). When a process job has been completed, it ceases to exist;
  /// its Job ID is no longer valid.
  /// 
  /// A control job is used to group a set of related process jobs. The group
  /// is logically related from the host's viewpoint. It also provides
  /// mechanisms for specifying the destination for processed material.
  /// 
  /// [Message]: crate::Message
  pub mod s16 {}
  //TODO: Fill out stream's contents.

  /// # STREAM 17: EQUIPMENT CONTROL AND DIAGNOSTICS
  /// **Based on SEMI E5§10.21**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Message]s which deal with control of the equipment from the host.
  /// 
  /// This includes all remote operations and equipment self-diagnostics and
  /// calibration but specifically excluses:
  /// 
  /// - Control operations associated with material transfer ([Stream 4]).
  /// - Loading of executive and boot programs ([Stream 8]).
  /// - File and operating system calls ([Stream 10], [Stream 13]).
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// This is a continuation of [Stream 2].
  /// 
  /// [Message]: crate::Message
  /// [Stream 2]: crate::messages::s2
  /// [Stream 4]: crate::messages::s4
  /// [Stream 8]: crate::messages::s8
  /// [Stream 10]: crate::messages::s10
  /// [Stream 13]: crate::messages::s13
  pub mod s17 {}
  //TODO: Fill out stream's contents.

  /// # STREAM 18: SUBSYSTEM CONTROL AND DATA
  /// **Based on SEMI E5§10.22**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Message]s which deal with interfacing between component subsystems and
  /// higher level controllers.
  /// 
  /// Compared to similar mesages exchanged between equipment and host,
  /// subsystem messages are less complex.
  /// 
  /// [Message]: crate::Message
  pub mod s18 {}
  //TODO: Fill out stream's contents.

  /// # STREAM 19: RECIPE AND PARAMETER MANAGEMENT
  /// **Based on SEMI E5§10.23**
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Message]s which deal with management of recipes that include:
  /// 
  /// - Self-documenting recipe component headers.
  /// - Support for multi-part recipes.
  /// - User-configured parameters.
  /// - Full assurance of byte integrity of PDE content.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Definitions:
  /// 
  /// - PDE - Process Definition Element - A component of a recipe, including
  /// an informational PDEheader and execution content PDEbody.
  /// - Recipe - Instructions or data that direct equipment behavior. A recipe
  /// is composed of one or more PDEs.
  /// - UID - Unique IDentifier - Used to identify a PDE.
  /// - GID - Group IDentifier - Used to identify PDEs that are subsitutable
  /// for one another.
  /// - InputMap, OutputMap - Data used to resolve references between PDEs in a
  /// multiple component recipe. These maps consist of a list of GID with the
  /// corresponding UID.
  /// - Resolve - Determination of all the components in a multi-part recipe.
  /// This is the process of creating an Outputmap that satisfies all the PDEs
  /// in a recipe.
  /// - TransferContainer - A group of PDEs or PDEheaders bound together as a
  /// single [Stream 13] Data Set for transfer.
  /// 
  /// [Message]: crate::Message
  /// [Stream 13]: crate::messages::s13
  pub mod s19 {}
  //TODO: Fill out stream's contents.

  /// # STREAM 20: RECIPE MANAGEMENT SYSTEM
  /// 
  /// The definition of this stream exists in a newer version of the standard
  /// as compared to SEMI E5-0712.
  pub mod s20 {}

  /// # STREAM 21: ITEM TRANSFER
  /// 
  /// The definition of this stream exists in a newer version of the standard
  /// as compared to SEMI E5-0712.
  pub mod s21 {}
}

/// # UNITS OF MEASURE
/// **Based on SEMI E5§12**
pub mod units {
  pub struct Unit {
    pub identifier: Identifier,
    pub exponent: Option<i64>,
  }

  pub enum Identifier {
    // ==== UNITLESS ==========================================================
    None                                 , //Null String
    //                                     ===== LINEAR SCALING ===============
    Percent                              , //%      | 1/100        |
    PartsPerMillion                      , //ppm    | 1/1,000,000  |
    //                                     ===== LOGARITHMIC SCALING ==========
    Bel                  (Option<Prefix>), //B      |              |
    Neper                (Option<Prefix>), //Np     | 0.1151       | dB
    PH                                   , //pH     |              |
    // ===== BASE UNITS =======================================================
    // T+1                                 ===== TIME =========================
    Second               (Option<Prefix>), //s      |              | SI
    Minute                               , //min    | 60           | s
    Hour                                 , //h      | 60           | min
    DayMeanSolar                         , //d      | 24           | h
    Month                                , //mo     |              |
    Year                                 , //yr     |              |
    //     L+1                             ===== LENGTH =======================
    Meter                (Option<Prefix>), //m      |              | SI
    Angstrom             (Option<Prefix>), //Ang    | 1e-10        | m
    Micron                               , //um     | 1e-6         | m
    MilliMicron                          , //nm     | 1e-9         | m
    NauticalMile                         , //nmi    | 1852         | m
    Inch                                 , //in     | 25.4         | mm
    Foot                                 , //ft     | 12           | in
    Mil                                  , //mil    | 1e-3         | in
    Mile                                 , //mile   | 5280         | ft
    //         M+1                         ===== MASS =========================
    Gram                 (Option<Prefix>), //g      |              | SI
    AtomicMass                           , //u      | 1.660531e-27 | kg
    Slug                                 , //slug   | 14.4939      | kg
    Pound                                , //lb     | 0.0310810    | slug
    //             I+1                     ===== ELECTRIC CURRENT =============
    Ampere               (Option<Prefix>), //A      |              | SI
    //                 H+1                 ===== TEMPERATURE ==================
    Kelvin                               , //K      |              | SI
    DegreeCelsius                        , //degC   |              |
    DegreeFarenheit                      , //degF   |              |
    //                     N+1             ===== AMOUNT OF SUBSTANCE ==========
    Mole                                 , //mol    | 6.02252e23   | SI
    //                         J+1         ===== LUMINOUS INTENSITY ===========
    Candela              (Option<Prefix>), //cd     |              | SI
    //                             P+1     ===== PLANAR ANGLE =================
    Radian               (Option<Prefix>), //rad    |              | SI
    Cycle                (Option<Prefix>), //c      | 2*pi         | rad
    Revolution                           , //r      | 1            | c
    DegreePlanar                         , //deg    | pi/180       | rad
    MinutePlanar                         , //mins   | 1/60         | deg
    SecondPlanar                         , //sec    | 1/60         | mins
    //                                 S+1 ===== SOLID ANGLE ==================
    Steradian            (Option<Prefix>), //Sr     |              | SI
    // ===== KINEMATICS =======================================================
    // T-1                         P+1     ===== FREQUENCY ====================
    Hertz                (Option<Prefix>), //Hz     | 1            | c/s
    Becquerel            (Option<Prefix>), //Bq     | 1            | Hz
    Curie                                , //Ci     | 3.7e10       | Bq
    // T-1 L+1                             ===== VELOCITY =====================
    Knot                                 , //kn     | 1            | nmi/h
    // T-2 L+1                             ===== ACCELERATION =================
    Gal                  (Option<Prefix>), //Gal    | 1            | cm/s^2
    //     L+2                             ===== AREA =========================
    Barn                 (Option<Prefix>), //barn   | 1e-28        | m^2
    Darcy                                , //D      | 0.986923     | um^2
    // T-1 L+2                             ===== KINEMATIC VISCOSITY ==========
    Stokes               (Option<Prefix>), //St     | 1            | cm^2/s
    //     L+3                             ===== VOLUME =======================
    Liter                (Option<Prefix>), //l      | 1e-3         | m^3
    Barrel                               , //bbl    | 158.99       | l
    Gallon                               , //gal    | 3.7854       | l
    GallonUK                             , //galUK  | 4.5461       | l
    PintUK                               , //ptUK   | 0.56826      | l
    PintUSDry                            , //ptUS   | 0.55061      | l
    PintUSLiquid                         , //pt     | 0.47318      | l
    QuartUK                              , //qtUK   | 1.1365       | l
    QuartUSDry                           , //qtUS   | 1.1012       | l
    QuartUSLiquid                        , //qt     | 0.94635      | l
    // T-1 L+3                             ===== FLOW =========================
    StandardCubicCentimeterPerMinute     , //sccm   | 1            | cm^3/min
    StandardLiterPerMinute               , //slpm   | 1            | l/min
    // ===== MECHANICS ========================================================
    // T-2 L+1 M+1                         ===== FORCE ========================
    Newton               (Option<Prefix>), //N      | 1            | kg*m/s^2
    Dyne                 (Option<Prefix>), //dyn    | 1e-5         | N
    GramForce            (Option<Prefix>), //gf     | 9.80665e-3   | N
    MetricTon                            , //t      | 10^3         | kgf
    PoundForce                           , //lbf    | 4.4482217    | N
    TonShort                             , //ton    | 2000         | lbf
    KiloPoundForce                       , //klbf   | 1000         | lbf
    Poundal                              , //pdl    | 0.0310810    | lbf
    OunceAvoirdupois                     , //oz     | 1/16         | lbf
    Grain                                , //gr     | 0.0022857143 | oz
    // T-2 L+2 M+1                         ===== ENERGY =======================
    Joule                (Option<Prefix>), //J      | 1            | N*m
    WattHour             (Option<Prefix>), //Wh     | 3600         | J
    BritishThermal                       , //Btu    | 1054.35      | J
    Therm                                , //thm    | 1e5          | Btu
    CalorieInternational (Option<Prefix>), //callIT | 4.1868       | J
    Calorie              (Option<Prefix>), //cal    | 4.1840       | J
    ElectronVolt         (Option<Prefix>), //eV     | 1.60209e-19  | J
    Erg                  (Option<Prefix>), //erg    | 1e-7         | J
    // T-3 L+2 M+1                         ===== POWER ========================
    Watt                 (Option<Prefix>), //W      | 1            | J/s
    Horsepower                           , //hp     | 746          | W
    Var                  (Option<Prefix>), //var    |              |
    // T-1 L-1 M+1                         ===== DYNAMIC VISCOSITY ============
    Poise                (Option<Prefix>), //P      , 36           | kg/m*s
    // T-2 L-1 M+1                         ===== PRESSURE =====================
    Pascal               (Option<Prefix>), //Pa     | 1            | N/m^2
    Bar                  (Option<Prefix>), //bar    | 100          | kPa
    AtmosphereStandard                   , //atm    | 101.325      | Pa
    AtmosphereTechnical                  , //at     | 1            | kgf/cm^2
    InchMercury                          , //inHg   | 3386.4       | Pa
    InchWater                            , //inH2O  | 249.09       | Pa
    MicronMercury                        , //umHg   | 133.32e-3    | Pa
    MilliMeterMercury                    , //mmHg   | 133.322      | Pa
    Torr                 (Option<Prefix>), //torr   | 1            | mmHg
    // ===== ELECTROMAGNETISM =================================================
    // T+1         I+1                     ===== ELECTRIC CHARGE ==============
    Coulomb              (Option<Prefix>), //C      | 1            | A*s
    // T-1         I+1                     ===== MAGNETIC FIELD STRENGTH ======
    Oersted              (Option<Prefix>), //Oe     | 79.477472    | A/m
    // T+3 L-2 M-1 I+2                     ===== CONDUCTANCE ==================
    Siemens              (Option<Prefix>), //S      | 1            | ohm^-1
    Mho                  (Option<Prefix>), //mho    | 1            | S
    // T+4 L-2 M-2 I+2                     ===== CAPACITANCE ==================
    Farad                (Option<Prefix>), //F      | 1            | A*s/V
    // T-2     M+1 I-1                     ===== MAGNETIC FLUX DENSITY ========
    Tesla                (Option<Prefix>), //T      | 1            | N/A*m
    Gauss                (Option<Prefix>), //G      | 1            | Mx/cm^2
    // T-2 L+2 M+2 I-1                     ===== MAGNETIC FLUX ================
    Weber                (Option<Prefix>), //Wb     | 1            | V*s
    Maxwell              (Option<Prefix>), //Mx     | 1e-8         | Wb
    // T-3 L+2 M+2 I-1                     ===== VOLTAGE ======================
    Volt                 (Option<Prefix>), //V      | 1            | W/A
    // T-2 L+2 M+1 I-2                     ===== INDUCTANCE ===================
    Henry                (Option<Prefix>), //H      | 1            | V*s/A
    // T-3 L+2 M+1 I-2                     ===== RESISTANCE ===================
    Ohm                  (Option<Prefix>), //ohm    | 1            | V/A
    //             I+1             P+1     ===== MAGNETOMOTIVE FORCE ==========
    AmpereTurn           (Option<Prefix>), //AT     | 1            | A*c
    Gilbert              (Option<Prefix>), //Gb     | 10/4*pi      | AT
    // ===== PHOTOMETRY =======================================================
    //                         J+1     S+1 ===== LUMINOUS FLUX ================
    Lumen                (Option<Prefix>), //lm     | 1            | cd*sr
    //     L-2                 J+1         ===== LUMINANCE ====================
    Nit                  (Option<Prefix>), //nt     | 1            | cd/m^2
    Stilb                (Option<Prefix>), //sb     | 1            | cd/cm^2
    Lambert              (Option<Prefix>), //L      | 1/pi         | cd/cm^2
    FootLambert                          , //FL     | 1/pi         | cd/ft^2
    //     L-2                 J+1     S-1 ===== ILLUMINANCE ==================
    Lux                  (Option<Prefix>), //lx     | 1            | lm/m^2
    Phot                 (Option<Prefix>), //ph     | 1            | lm/cm^2
    FootCandle                           , //Fc     | 1            | lm/ft^2
    // ===== RADIOACTIVITY ====================================================
    // T-2 L+2                             ===== ABSORBED DOSE ================
    Sievert              (Option<Prefix>), //Sv     | 1            | J/kg
    Rem                  (Option<Prefix>), //rem    | 1e-2         | Sv
    Gray                 (Option<Prefix>), //Gy     | 1            | J/kg
    Rad                  (Option<Prefix>), //rd     | 1e-2         | Gy
    // T+1     M-1 I+1                     ===== RADIATION EXPOSURE ===========
    Roentgen                             , //R      | 2.58e-4      | C/kg
    // ===== INFORMATION THEORY ===============================================
    //                                     ===== DATA =========================
    Bit                  (Option<Prefix>), //bit    |              |
    Byte                 (Option<Prefix>), //byte   | 8            | bit
    // T-1                                 ===== DATA RATE ====================
    Baud                 (Option<Prefix>), //Bd     | 1            | bit/s
    // ===== SECS SPECIAL UNITS ===============================================
    Ion                                  , //ion       | Atom that carries an electric charge as a result of losing or gaining electrons.
    Substrate                            , //substrate | Entity of material being operated on, processed, or fabricated.
    Ingot                                , //ing       | Entity of semiconductor manufacture from which wafers are made.
    Wafer                                , //wfr       | Entity of material on which semiconductor devices are fabricated.
    Die                                  , //die       | Individual integrated circuit both on a wafer and after wafer separation. Also known as bar or chip.
    Package                              , //pkg       | Individual entity both as a place for the die to reside and as a completed unit.
    Lot                                  , //lot       | Grouping of material which is undergoing the same processing operations. The amount of material represented by "1 lot" is situational.
    Boat                 (Option<Suffix>), //boat      | Holder for wafers or packages with discrete positions, whose capacity is specified by the suffix.
    Carrier              (Option<Suffix>), //carrier   | Holder for substrates, wafers, or wafer frames, whose capacity is specified by the suffix.
    Cassette             (Option<Suffix>), //css       | Holder for wafers or wafer frames, whose capacity is specified by the suffix.
    LeadFrame            (Option<Suffix>), //ldfr      | Structure for leads which is removed after packaging, whose capacity is specified by the suffix. May be a fixed length or a reel.
    Magazine             (Option<Suffix>), //mgz       | Holder for fixed length leadframes, whose capacity is specified by the suffix.
    Plate                (Option<Suffix>), //plt       | Temporary fixture used to hold die during assembly, whose capacity is specified by the suffix.
    Tube                 (Option<Suffix>), //tube      | Holder for packages arranged in a flow, whose capacity is specified by the suffix.
    WaferFrame           (Option<Suffix>), //wffr      | Temporary fixture for wafers, whose capacity is specified by the suffix.
  }

  pub enum Prefix {
    Exa,   //E  | 1e18
    Peta,  //P  | 1e15
    Tera,  //T  | 1e12
    Giga,  //G  | 1e9
    Mega,  //M  | 1e6
    Kilo,  //k  | 1e3
    Hecto, //h  | 1e2
    Deca,  //d  | 1e1
    Deci,  //da | 1e-1
    Centi, //c  | 1e-2
    Milli, //m  | 1e-3
    Micro, //u  | 1e-6
    Nano,  //n  | 1e-9
    Pico,  //p  | 1e-12
    Femto, //f  | 1e-15
    Atto,  //a  | 1e-18
  }

  pub struct Suffix(pub u64);
}
//TODO: Fully implement units.
