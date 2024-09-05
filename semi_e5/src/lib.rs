//! # SEMI EQUIPMENT COMMUNICATIONS STANDARD 2 (SECS-II) MESSAGE CONTENT
//! **Based on:**
//! - **[SEMI E5]-0813**
//! 
//! This third-party codebase will be updated to reflect more up-to-date SEMI
//! standards if/when they can be acquired for this purpose.
//! 
//! ---------------------------------------------------------------------------
//! 
//! [SECS-II] is a [Presentation Layer] protocol designed to facilitate a
//! common communications language between semiconductor equipment,
//! particularly as understood by the GEM ([SEMI E30]) [Application Layer]
//! protocol (together known as SECS/GEM). Common [Session Layer] protocols for
//! transporting [SECS-II] messages include SECS-I ([SEMI E4]) and HSMS
//! ([SEMI E37]).
//! 
//! ---------------------------------------------------------------------------
//! 
//! ## TO BE DONE
//! 
//! - Implement "Localized" strings.
//! - Finish adding items.
//! - Add messages to Streams 2 through 21.
//! 
//! ---------------------------------------------------------------------------
//! 
//! ## REFERENCED STANDARDS
//! 
//! - SEMI E4        - SEMI Equipment Communications Standard 1 Message Transfer (SECS-I)
//! - SEMI E6        - Guide for Semiconductor Equipment Installation Documentation
//! - SEMI E37       - High-Speed SECS Message Services (HSMS) Generic Services
//! - SEMI E148      - Specification for Time Synchronization and Definition of the TS-Clock Object
//! - ANSI X3.4-1977 - Code for Information Interchange (ASCII)
//! - IEEE 754       - Standards for Binary Floating Point Arithmetic
//! - JIS-6226       - JIS 8-bit Coded Character Set for Information Exchange
//! 
//! [SEMI E4]:  https://store-us.semi.org/products/e00400-semi-e4-specification-for-semi-equipment-communications-standard-1-message-transfer-secs-i
//! [SEMI E5]:  https://store-us.semi.org/products/e00500-semi-e5-specification-for-semi-equipment-communications-standard-2-message-content-secs-ii
//! [SEMI E30]: https://store-us.semi.org/products/e03000-semi-e30-specification-for-the-generic-model-for-communications-and-control-of-manufacturing-equipment-gem
//! [SEMI E37]: https://store-us.semi.org/products/e03700-semi-e37-high-speed-secs-message-services-hsms-generic-services
//! 
//! [Application Layer]:  https://en.wikipedia.org/wiki/Application_layer
//! [Presentation Layer]: https://en.wikipedia.org/wiki/Presentation_layer
//! [Session Layer]:      https://en.wikipedia.org/wiki/Session_layer
//! 
//! [SECS-II]:  crate

#![feature(ascii_char)]
#![feature(ascii_char_variants)]
#![allow(clippy::unusual_byte_groupings)]
#![allow(clippy::collapsible_match)]

pub mod format;
pub mod items;
pub mod messages;
pub mod units;

use std::ascii::Char;
use encoding::{all::ISO_2022_JP, Encoding};

/// ## GENERIC MESSAGE
/// **Based on SEMI E5§6**
/// 
/// The set of all information required to be sent over-the-wire in any
/// particular exchange order to properly communicate according to this
/// protocol. May contain an [Item].
/// 
/// [Item]: Item
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

  /// ### TEXT
  /// 
  /// The message's contents.
  /// 
  /// - [None] - Indicates a header-only message.
  /// - [Some] - Indicates a message with contents after the header.
  pub text: Option<Item>,
}

/// ## DATA CONVERSION ERROR
/// 
/// Represents an error in converting from a [Generic Message] to any specific
/// [Message].
/// 
/// [Message]:         messages
/// [Generic Message]: Message
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
  /// ### EMPTY TEXT
  /// 
  /// Binary data was attempted to be converted into an [Item] despite being
  /// empty.
  EmptyText,

  /// ### INVALID TEXT
  /// 
  /// Binary data was attempted to be converted into a [Item] despite having an
  /// invalid format.
  InvalidText,

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
  /// [Message Body]:    Message::text
  WrongFormat,
}

/// ## GENERIC ITEM
/// **Based on SEMI E5§9**
/// 
/// A packet of information of a particular [Format], which
/// through the [List] format, is able to represent a tree-like structure
/// of information. Each item comprises a [Vector] of a particular type,
/// or sometimes a [String] of characters.
/// 
/// [Format]: format
/// [List]:   Item::List
/// [Vector]: Vec
/// [String]: String
#[repr(u8)]
#[derive(Clone, Debug)]
pub enum Item {
  /// ### LIST
  /// **Based on SEMI E5§9.3**
  /// 
  /// A [List] is an ordered set of elements, where elements are [Item]s.
  /// 
  /// The Item Header of a [List] is unique in that the Item Length refers to
  /// the length of the [List] in the number of [Item]s it contains, rather
  /// than the number of bytes.
  /// 
  /// [List]: self
  List(Vec<Item>) = format::LIST,

  /// ### ASCII
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// ASCII character string.
  Ascii(Vec<Char>) = format::ASCII,

  /// ### JIS-8
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// JIS-8 character string.
  Jis8(String) = format::JIS8,

  /// ### LOCALIZED STRING
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// Note: Used only by item 'TEXT' in S10F1, S10F3, S10F5, and S10F9
  /// 
  /// 2-byte character string.
  Local(LocalizedStringHeader, Vec<u8>) = format::LOCAL,

  /// ### BINARY
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// Single-byte quanitity where the value can be anything and does not
  /// otherwise have a strictly defined meaning.
  Bin(Vec<u8>) = format::BIN,

  /// ### BOOLEAN
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// Single-byte quantity where a value of 0 is equivalent to 'false' and any
  /// non-zero value is equivalent to 'true'.
  Bool(Vec<bool>) = format::BOOL,

  /// ### 1-BYTE SIGNED INTEGER
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// 1-byte two's compliment integer.
  I1(Vec<i8>) = format::I1,

  /// ### 2-BYTE SIGNED INTEGER
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// 2-byte two's compliment integer.
  I2(Vec<i16>) = format::I2,

  /// ### 4-BYTE SIGNED INTEGER
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// 4-byte two's compliment integer.
  I4(Vec<i32>) = format::I4,

  /// ### 8-BYTE SIGNED INTEGER
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// 8-byte two's compliment integer.
  I8(Vec<i64>) = format::I8,

  /// ### 1-BYTE UNSIGNED INTEGER
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// 1-byte integer.
  U1(Vec<u8>) = format::U1,

  /// ### 2-BYTE UNSIGNED INTEGER
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// 2-byte integer.
  U2(Vec<u16>) = format::U2,

  /// ### 4-BYTE UNSIGNED INTEGER
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// 4-byte integer.
  U4(Vec<u32>) = format::U4,

  /// ### 8-BYTE UNSIGNED INTEGER
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// 8-byte integer.
  U8(Vec<u64>) = format::U8,

  /// ### 4-BYTE FLOATING POINT NUMBER
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// 4-byte IEEE-754 floating point number.
  F4(Vec<f32>) = format::F4,

  /// ### 8-BYTE FLOATING POINT NUMBER
  /// **Based on SEMI E5§9.2.2**
  /// 
  /// 8-byte IEEE-754 floating point number.
  F8(Vec<f64>) = format::F8,
}
impl Item {
  /// ### SINGLE BINARY ITEM
  /// 
  /// Constructs a [Binary] [Item] with a single member.
  /// 
  /// Provided for convinient syntax of this common use case.
  /// 
  /// [Item]:   Item
  /// [Binary]: Item::Bin
  pub fn bin(value: u8) -> Self {
    Self::Bin(vec![value])
  }

  /// ### SINGLE BOOLEAN ITEM
  /// 
  /// Constructs a [Boolean] [Item] with a single member.
  /// 
  /// Provided for convinient syntax of this common use case.
  /// 
  /// [Item]:    Item
  /// [Boolean]: Item::Bool
  pub fn bool(value: bool) -> Self {
    Self::Bool(vec![value])
  }

  /// ### SINGLE 1-BYTE SIGNED INTEGER ITEM
  /// 
  /// Constructs a [1-byte Signed Integer] [Item] with a single member.
  /// 
  /// Provided for convinient syntax of this common use case.
  /// 
  /// [Item]:                  Item
  /// [1-byte Signed Integer]: Item::I1
  pub fn i1(value: i8) -> Self {
    Self::I1(vec![value])
  }

  /// ### SINGLE 2-BYTE SIGNED INTEGER ITEM
  /// 
  /// Constructs a [2-byte Signed Integer] [Item] with a single member.
  /// 
  /// Provided for convinient syntax of this common use case.
  /// 
  /// [Item]:                  Item
  /// [2-byte Signed Integer]: Item::I2
  pub fn i2(value: i16) -> Self {
    Self::I2(vec![value])
  }

  /// ### SINGLE 4-BYTE SIGNED INTEGER ITEM
  /// 
  /// Constructs a [4-byte Signed Integer] [Item] with a single member.
  /// 
  /// Provided for convinient syntax of this common use case.
  /// 
  /// [Item]:                  Item
  /// [4-byte Signed Integer]: Item::I4
  pub fn i4(value: i32) -> Self {
    Self::I4(vec![value])
  }

  /// ### SINGLE 8-BYTE SIGNED INTEGER ITEM
  /// 
  /// Constructs an [8-byte Signed Integer] [Item] with a single member.
  /// 
  /// Provided for convinient syntax of this common use case.
  /// 
  /// [Item]:                  Item
  /// [8-byte Signed Integer]: Item::I8
  pub fn i8(value: i64) -> Self {
    Self::I8(vec![value])
  }

  /// ### SINGLE 1-BYTE UNSIGNED INTEGER ITEM
  /// 
  /// Constructs a [1-byte Unsigned Integer] [Item] with a single member.
  /// 
  /// Provided for convinient syntax of this common use case.
  /// 
  /// [Item]:                    Item
  /// [1-byte Unsigned Integer]: Item::U1
  pub fn u1(value: u8) -> Self {
    Self::U1(vec![value])
  }

  /// ### SINGLE 2-BYTE UNSIGNED INTEGER ITEM
  /// 
  /// Constructs a [2-byte Unsigned Integer] [Item] with a single member.
  /// 
  /// Provided for convinient syntax of this common use case.
  /// 
  /// [Item]:                    Item
  /// [2-byte Unsigned Integer]: Item::U2
  pub fn u2(value: u16) -> Self {
    Self::U2(vec![value])
  }

  /// ### SINGLE 4-BYTE UNSIGNED INTEGER ITEM
  /// 
  /// Constructs a [4-byte Unsigned Integer] [Item] with a single member.
  /// 
  /// Provided for convinient syntax of this common use case.
  /// 
  /// [Item]:                    Item
  /// [4-byte Unsigned Integer]: Item::U4
  pub fn u4(value: u32) -> Self {
    Self::U4(vec![value])
  }

  /// ### SINGLE 8-BYTE UNSIGNED INTEGER ITEM
  /// 
  /// Constructs an [8-byte Unsigned Integer] [Item] with a single member.
  /// 
  /// Provided for convinient syntax of this common use case.
  /// 
  /// [Item]:                    Item
  /// [8-byte Unsigned Integer]: Item::U8
  pub fn u8(value: u64) -> Self {
    Self::U8(vec![value])
  }

  /// ### SINGLE 4-BYTE FLOATING POINT NUMBER ITEM
  /// 
  /// Constructs a [4-byte Floating Point Number] [Item] with a single member.
  /// 
  /// Provided for convinient syntax of this common use case.
  /// 
  /// [Item]:                         Item
  /// [4-byte Floating Point Number]: Item::F4
  pub fn f4(value: f32) -> Self {
    Self::F4(vec![value])
  }

  /// ### SINGLE 8-BYTE FLOATING POINT NUMBER ITEM
  /// 
  /// Constructs an [8-byte Floating Point Number] [Item] with a single member.
  /// 
  /// Provided for convinient syntax of this common use case.
  /// 
  /// [Item]:                         Item
  /// [8-byte Floating Point Number]: Item::F8
  pub fn f8(value: f64) -> Self {
    Self::F8(vec![value])
  }
}
impl From<Item> for Vec<u8> {
  /// ### ITEM -> BINARY DATA
  /// 
  /// Infallable serialization of an [Item], which can represent an entire tree
  /// of [Item]s due to [List]s, into binary data.
  /// 
  /// [Item]: Item
  /// [List]: Item::List
  fn from(item: Item) -> Self {
    let mut vec = vec![];
    match item {
      // List
      Item::List(item_vec) => {
        //Length
        let len = item_vec.len();
        if len < 256 {
          vec.push(format::LIST | 1);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(format::LIST | 2);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(format::LIST | 3);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Items
        for item in item_vec {
          vec.append(&mut item.into());
        }
      },
      // ASCII
      Item::Ascii(ascii_vec) => {
        //Length
        let len = ascii_vec.len();
        if len < 256 {
          vec.push(format::ASCII | 1);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(format::ASCII | 2);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(format::ASCII | 3);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Vector
        for ascii in ascii_vec {
          vec.push(ascii as u8);
        }
      },
      // JIS-8
      Item::Jis8(jis8_string) => {
        // Encode
        let encoded = ISO_2022_JP.encode(&jis8_string, encoding::EncoderTrap::Ignore).unwrap();
        // Item Code + Length
        let len = encoded.len();
        if len < 256 {
          vec.push(format::JIS8 | 1);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(format::JIS8 | 2);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(format::JIS8 | 3);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        // Vector
        vec.extend_from_slice(&encoded);
      },
      // Localized String (TODO)
      Item::Local(_widechar_format, _widechar_vec) => {
        // 010010_00
        todo!()
      },
      // Binary
      Item::Bin(bin_vec) => {
        //Length
        let len = bin_vec.len();
        if len < 256 {
          vec.push(format::BIN | 1);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(format::BIN | 2);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(format::BIN | 3);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Vector
        for bin in bin_vec {
          vec.push(bin);
        }
      },
      // Boolean
      Item::Bool(bool_vec) => {
        //Length
        let len = bool_vec.len();
        if len < 256 {
          vec.push(format::BOOL | 1);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(format::BOOL | 2);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(format::BOOL | 3);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Vector
        for bool in bool_vec {
          vec.push(bool as u8);
        }
      },
      // 1-Byte Signed Integer
      Item::I1(i1_vec) => {
        //Length
        let len = i1_vec.len();
        if len < 256 {
          vec.push(format::I1 | 1);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(format::I1 | 2);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(format::I1 | 3);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Vector
        for i1 in i1_vec {
          vec.extend_from_slice(&i1.to_be_bytes());
        }
      },
      // 2-Byte Signed Integer
      Item::I2(i2_vec) => {
        //Length
        let len = i2_vec.len() * 2;
        if len < 256 {
          vec.push(format::I2 | 1);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(format::I2 | 2);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(format::I2 | 3);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Vector
        for i2 in i2_vec {
          vec.extend_from_slice(&i2.to_be_bytes());
        }
      },
      // 4-Byte Signed Integer
      Item::I4(i4_vec) => {
        //Length
        let len = i4_vec.len() * 4;
        if len < 256 {
          vec.push(format::I4 | 1);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(format::I4 | 2);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(format::I4 | 3);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Vector
        for i4 in i4_vec {
          vec.extend_from_slice(&i4.to_be_bytes());
        }
      },
      // 8-Byte Signed Integer
      Item::I8(i8_vec) => {
        //Length
        let len = i8_vec.len() * 8;
        if len < 256 {
          vec.push(format::I8 | 1);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(format::I8 | 2);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(format::I8 | 3);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Vector
        for i8 in i8_vec {
          vec.extend_from_slice(&i8.to_be_bytes());
        }
      },
      // 1-Byte Unsigned Integer
      Item::U1(u1_vec) => {
        //Length
        let len = u1_vec.len();
        if len < 256 {
          vec.push(format::U1 | 1);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(format::U1 | 2);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(format::U1 | 3);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Vector
        for u1 in u1_vec {
          vec.push(u1);
        }
      },
      // 2-Byte Unsigned Integer
      Item::U2(u2_vec) => {
        //Length
        let len = u2_vec.len() * 2;
        if len < 256 {
          vec.push(format::U2 | 1);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(format::U2 | 2);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(format::U2 | 3);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Vector
        for u2 in u2_vec {
          vec.extend_from_slice(&u2.to_be_bytes());
        }
      },
      // 4-Byte Unsigned Integer
      Item::U4(u4_vec) => {
        //Length
        let len = u4_vec.len() * 4;
        if len < 256 {
          vec.push(format::U4 | 1);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(format::U4 | 2);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(format::U4 | 3);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Vector
        for u4 in u4_vec {
          vec.extend_from_slice(&u4.to_be_bytes());
        }
      },
      // 8-Byte Unsigned Integer
      Item::U8(u8_vec) => {
        //Length
        let len = u8_vec.len() * 8;
        if len < 256 {
          vec.push(format::U8 | 1);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(format::U8 | 2);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(format::U8 | 3);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Vector
        for u8 in u8_vec {
          vec.extend_from_slice(&u8.to_be_bytes());
        }
      },
      // 4-Byte Floating Point Number
      Item::F4(f4_vec) => {
        //Length
        let len = f4_vec.len() * 4;
        if len < 256 {
          vec.push(format::F4 | 1);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(format::F4 | 2);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(format::F4 | 3);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Vector
        for f4 in f4_vec {
          vec.extend_from_slice(&f4.to_be_bytes());
        }
      },
      // 8-Byte Floating Point Number
      Item::F8(f8_vec) => {
        //Length
        let len = f8_vec.len() * 8;
        if len < 256 {
          vec.push(format::F8 | 1);
          vec.push(len as u8);
        } else if len < 65536 {
          vec.push(format::F8 | 2);
          vec.extend_from_slice(&(len as u16).to_be_bytes());
        } else {
          vec.push(format::F8 | 3);
          vec.extend_from_slice(&(len as u32).to_be_bytes()[0..3]);
        };
        //Vector
        for f8 in f8_vec {
          vec.extend_from_slice(&f8.to_be_bytes());
        }
      },
    }
    vec
  }
}
impl TryFrom<Vec<u8>> for Item {
  type Error = Error;

  /// ### BINARY DATA -> ITEM
  /// 
  /// Fallable deserialization of binary data into an [Item], which can
  /// represent an entire tree of [Item]s due to [List]s.
  /// 
  /// [Item]: Item
  /// [List]: Item::List
  fn try_from(text: Vec<u8>) -> Result<Self, Self::Error> {
    /// ## INTERNAL CONVERSION FUNCTION
    /// 
    /// Converts data from an iterator into an item without final checks and
    /// using recursion in the case of List items.
    fn convert(data: &mut std::slice::Iter<u8>) -> Option<Item> {
      let format_byte = *data.next()?;
      let item = format_byte & 0b111111_00;
      let length_length = format_byte & 0b000000_11;
      if length_length == 0 {return None}
      let length: u32 = {
        let mut bytes = [0u8;4];
        for i in (4-length_length)..4 {
          bytes[i as usize] = *data.next()?;
        }
        u32::from_be_bytes(bytes)
      };
      match item {
        // List
        format::LIST => {
          let mut vec: Vec<Item> = vec![];
          // Perform Recursion
          for _ in 0..length {vec.push(convert(data)?);}
          Some(Item::List(vec))
        },
        // ASCII
        format::ASCII => {
          let mut vec: Vec<Char> = vec![];
          for _ in 0..length {vec.push(Char::from_u8(*data.next()?)?);}
          Some(Item::Ascii(vec))
        },
        // JIS-8
        format::JIS8 => {
          let mut vec: Vec<u8> = vec![];
          for _ in 0..length {vec.push(*data.next()?);}
          Some(Item::Jis8(ISO_2022_JP.decode(&vec, encoding::types::DecoderTrap::Strict).ok()?))
        },
        // Localized String (TODO)
        format::LOCAL => None,
        // Binary
        format::BIN => {
          let mut vec: Vec<u8> = vec![];
          for _ in 0..length {vec.push(*data.next()?);}
          Some(Item::Bin(vec))
        },
        // Boolean
        format::BOOL => {
          let mut vec: Vec<bool> = vec![];
          for _ in 0..length {vec.push(*data.next()? > 0);}
          Some(Item::Bool(vec))
        },
        // 1-Byte Signed Integer
        format::I1 => {
          let mut vec: Vec<i8> = vec![];
          for _ in 0..length {vec.push(*data.next()? as i8);}
          Some(Item::I1(vec))
        },
        // 2-Byte Signed Integer
        format::I2 => {
          if length % 2 != 0 {return None}
          let mut vec: Vec<i16> = vec![];
          for _ in 0..length/2 {
            let mut bytes = [0u8;2];
            for byte in &mut bytes {*byte = *data.next()?}
            vec.push(i16::from_be_bytes(bytes));
          }
          Some(Item::I2(vec))
        },
        // 4-Byte Signed Integer
        format::I4 => {
          if length % 4 != 0 {return None}
          let mut vec: Vec<i32> = vec![];
          for _ in 0..length/4 {
            let mut bytes = [0u8;4];
            for byte in &mut bytes {*byte = *data.next()?}
            vec.push(i32::from_be_bytes(bytes));
          }
          Some(Item::I4(vec))
        },
        // 8-Byte Signed Integer
        format::I8 => {
          if length % 8 != 0 {return None}
          let mut vec: Vec<i64> = vec![];
          for _ in 0..length/8 {
            let mut bytes = [0u8;8];
            for byte in &mut bytes {*byte = *data.next()?}
            vec.push(i64::from_be_bytes(bytes));
          }
          Some(Item::I8(vec))
        },
        // 1-Byte Unsigned Integer
        format::U1 => {
          let mut vec: Vec<u8> = vec![];
          for _ in 0..length {vec.push(*data.next()?);}
          Some(Item::U1(vec))
        },
        // 2-Byte Unsigned Integer
        format::U2 => {
          if length % 2 != 0 {return None}
          let mut vec: Vec<u16> = vec![];
          for _ in 0..length/2 {
            let mut bytes = [0u8;2];
            for byte in &mut bytes {*byte = *data.next()?}
            vec.push(u16::from_be_bytes(bytes));
          }
          Some(Item::U2(vec))
        },
        // 4-Byte Unsigned Integer
        format::U4 => {
          if length % 4 != 0 {return None}
          let mut vec: Vec<u32> = vec![];
          for _ in 0..length/4 {
            let mut bytes = [0u8;4];
            for byte in &mut bytes {*byte = *data.next()?}
            vec.push(u32::from_be_bytes(bytes));
          }
          Some(Item::U4(vec))
        },
        // 8-Byte Unsigned Integer
        format::U8 => {
          if length % 8 != 0 {return None}
          let mut vec: Vec<u64> = vec![];
          for _ in 0..length/8 {
            let mut bytes = [0u8;8];
            for byte in &mut bytes {*byte = *data.next()?}
            vec.push(u64::from_be_bytes(bytes));
          }
          Some(Item::U8(vec))
        },
        // 4-Byte Floating Point Number
        format::F4 => {
          if length % 4 != 0 {return None}
          let mut vec: Vec<f32> = vec![];
          for _ in 0..length/4 {
            let mut bytes = [0u8;4];
            for byte in &mut bytes {*byte = *data.next()?}
            vec.push(f32::from_be_bytes(bytes));
          }
          Some(Item::F4(vec))
        },
        // 8-Byte Floating Point Number
        format::F8 => {
          if length % 8 != 0 {return None}
          let mut vec: Vec<f64> = vec![];
          for _ in 0..length/8 {
            let mut bytes = [0u8;8];
            for byte in &mut bytes {*byte = *data.next()?}
            vec.push(f64::from_be_bytes(bytes));
          }
          Some(Item::F8(vec))
        },
        // Unrecognized
        _ => None
      }
    }
    // Empty items are their own category of error which may be acceptable elsewhere.
    if text.is_empty() {return Err(Error::EmptyText)};
    // Convert data into an item.
    let mut data: std::slice::Iter<u8> = text.iter();
    let result = convert(&mut data).ok_or(Error::InvalidText)?;
    // Check that all text has been handled.
    if data.next().is_some() {return Err(Error::InvalidText)}
    // Finish.
    Ok(result)
  }
}

/// ## LOCALIZED STRING HEADER
/// **Based on SEMI E5§9.4**
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
