#![feature(ascii_char)]
#![allow(clippy::unusual_byte_groupings)]
#![allow(clippy::collapsible_match)]

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

use std::ascii::Char;

/// ## DATA CONVERSION ERROR
pub enum Error {
  WrongReply,
  WrongStream,
  WrongFunction,
  WrongFormat,
}

//9.3.1
//Table 1 Item Format Codes
//MSB Sent First
#[repr(u8)]
#[derive(Clone, Debug)]
pub enum Item {
  // 0 = List
  List     (     Vec<Item>) = 0b000000_00, //0o00
  Binary   (     Vec<  u8>) = 0b001000_00, //0o10
  Boolean  (     Vec<bool>) = 0b001001_00, //0o11
  Ascii    (     Vec<Char>) = 0b010000_00, //0o20
  Jis8     (     Vec<  u8>) = 0b010001_00, //0o21
  Localized(u16, Vec< u16>) = 0b010010_00, //0o22
  // 3() = Signed Numbers
  Signed8  (     Vec< i64>) = 0b011000_00, //0o30
  Signed1  (     Vec<  i8>) = 0b011001_00, //0o31
  Signed2  (     Vec< i16>) = 0b011010_00, //0o32
  Signed4  (     Vec< i32>) = 0b011100_00, //0o34
  // 4() = Floating Point Numbers
  Float8   (     Vec< f64>) = 0b100000_00, //0o40
  Float4   (     Vec< f32>) = 0b100100_00, //0o44
  // 5() = Unsigned Numbers
  Unsigned8(     Vec< u64>) = 0b101000_00, //0o50
  Unsigned1(     Vec<  u8>) = 0b101001_00, //0o51
  Unsigned2(     Vec< u16>) = 0b101010_00, //0o52
  Unsigned4(     Vec< u32>) = 0b101100_00, //0o54
}

//BINARY CONVERSIONS
impl From<Item> for Vec<u8> {
  fn from(val: Item) -> Self {
    let mut vec = vec![];
    match val {
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
      Item::Jis8(_) => todo!(),
      Item::Localized(_, _) => todo!(),
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

//CONVERSIONS: ITEM -> DATA
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
impl<A: TryFrom<Item, Error = Error>, B: TryFrom<Item, Error = Error>> TryFrom<Item> for (A, B) {
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
/*impl<A: TryFrom<Item, Error = Error> + Sized> TryInto<Option<A>> for Item {
  type Error = Error;

  fn try_into(self) -> Result<Option<A>, Self::Error> {
    todo!()
  }
}*/

//CONVERSIONS: DATA -> ITEM
impl From<()> for Item {
  fn from(_value: ()) -> Self {
    Item::List(vec![])
  }
}
impl<A: Into<Item>, B: Into<Item>> From<(A, B)> for Item {
  fn from(value: (A, B)) -> Self {
    Item::List(vec![value.0.into(), value.1.into()])
  }
}

//9.4.2 Localized String Header
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

#[derive(Clone, Debug)]
pub struct Message {
  pub w: bool,
  pub stream: u8,
  pub function: u8,
  pub text: Option<Item>,
}

pub mod items {
  use crate::Item;
  use crate::Error::{self, *};
  use std::ascii::Char;
  use num_enum::{IntoPrimitive, TryFromPrimitive};

  /// ## DATA ITEM MACRO: SINGLE ACCEPTED FORMAT, VECTOR LENGTH 1
  macro_rules! singleformat {
    (
      $name:ident,
      $type:ident,
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
      $type:ty,
      $format:ident,
      $range:expr
    ) => {
      impl $name {
        pub fn new(vec: Vec<$type>) -> Option<Self> {
          if $range.contains(&vec.len()) {
            Some(Self(vec))
          } else {
            None
          }
        }
      }
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
              if $range.contains(&vec.len()) {
                Ok(Self(vec))
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

  /// ## DATA ITEM MACRO: SINGLE ACCEPTED FORMAT, ENUMERATED VALUE
  macro_rules! singleformat_enum {
    (
      $name:ident,
      $type:ident,
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

  /// ## ABS
  /// 
  /// -----------------------------------------------------------------------
  /// 
  /// #### Description
  /// 
  /// Any binary string.
  /// 
  /// -----------------------------------------------------------------------
  /// 
  /// #### Used By
  /// 
  /// - [S2F25]
  /// - [S2F26]
  #[derive(Clone, Debug)]
  pub struct AnyBinaryString(Vec<u8>);
  singleformat_vec!(AnyBinaryString, u8, Binary, 0..);

  /// ## ALCD
  /// 
  /// -----------------------------------------------------------------------
  /// 
  /// #### Description
  /// 
  /// Alarm code byte.
  /// 
  /// -----------------------------------------------------------------------
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
  /// -----------------------------------------------------------------------
  /// 
  /// #### Used By
  /// 
  /// - [S5F1]
  /// - [S5F6]
  /// - [S5F8]
  #[derive(Clone, Copy, Debug)]
  pub struct AlarmCode(pub u8);
  singleformat!(AlarmCode, u8, Binary);

  /// ## MDLN
  /// 
  /// -----------------------------------------------------------------------
  /// 
  /// #### Description
  /// 
  /// Equipment Model Type, 20 bytes max.
  /// 
  /// -----------------------------------------------------------------------
  /// 
  /// #### Used By
  /// 
  /// - [S1F2]
  /// - [S1F13]
  /// - [S1F14]
  /// - [S7F22]
  /// - [S7F23]
  /// - [S7F26]
  /// - [S7F31]
  /// - [S7F39]
  /// - [S7F43]
  #[derive(Clone, Debug)]
  pub struct ModelName(Vec<Char>);
  singleformat_vec!(ModelName, Char, Ascii, 0..=20);

  /// ## SOFTREV
  /// 
  /// -----------------------------------------------------------------------
  /// 
  /// #### Description
  /// 
  /// Software Revision Code, 20 bytes max.
  /// 
  /// -----------------------------------------------------------------------
  /// 
  /// #### Used By
  /// 
  /// - [S1F2]
  /// - [S1F13]
  /// - [S1F14]
  /// - [S7F22]
  /// - [S7F23]
  /// - [S7F26]
  /// - [S7F31]
  /// - [S7F39]
  /// - [S7F43]
  #[derive(Clone, Debug)]
  pub struct SoftwareRevision(Vec<Char>);
  singleformat_vec!(SoftwareRevision, Char, Ascii, 0..=20);

  /// ## COMMACK
  #[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
  #[repr(u8)]
  pub enum CommAck {
    Accepted = 0,
    Denied   = 1,
  }
  singleformat_enum!(CommAck, u8, Binary);

  /// ## OFLACK
  #[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
  #[repr(u8)]
  pub enum OffLineAcknowledge {
    Acknowledge = 0,
  }
  singleformat_enum!(OffLineAcknowledge, u8, Binary);

  /// ## ONLACK
  #[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
  #[repr(u8)]
  pub enum OnLineAcknowledge {
    Accepted      = 0,
    NotAllowed    = 1,
    AlreadyOnLine = 2,
  }
  singleformat_enum!(OnLineAcknowledge, u8, Binary);

  #[repr(u32)]
  pub enum ErrCode {
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

}

pub mod messages {
  /// ## MESSAGE MACRO: HEADER ONLY
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
            w:        $w,
            stream:   $stream,
            function: $function,
            text:     None,
          }
        }
      }
      impl TryFrom<Message> for $name {
        type Error = Error;

        fn try_from(message: Message) -> Result<Self, Self::Error> {
          if message.stream   != $stream   {return Err(WrongStream)}
          if message.function != $function {return Err(WrongFunction)}
          if message.w        != $w        {return Err(WrongReply)}
          match message.text {
            None => Ok($name),
            Some(_item) => Err(WrongFormat),
          }
        }
      }
    }
  }

  /// ## MESSAGE MACRO: DATA
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
            w:        $w,
            stream:   $stream,
            function: $function,
            text:     Some(value.0.into()),
          }
        }
      }
      impl TryFrom<Message> for $name {
        type Error = Error;

        fn try_from(message: Message) -> Result<Self, Self::Error> {
          if message.stream   != $stream   {return Err(WrongStream)}
          if message.function != $function {return Err(WrongFunction)}
          if message.w        != $w        {return Err(WrongReply)}
          match message.text {
            Some(item) => {Ok(Self(item.try_into()?))},
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
  /// This stream provides a means for exchanging information about the status
  /// of the equipment, including its current mode, depletion of various
  /// consumable items, and the status of transfer operations.
  pub mod s1 {
    use crate::Message;
    use crate::Error::{self, *};
    use crate::items::*;

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
    /// #### Description
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
    pub struct HostCR(());
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
    /// #### Description
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
    /// [S1F13]: HostCR
    /// [COMMACK]: crate::items::CommAck
    /// [MDLN]: crate::items::ModelName
    /// [SOFTREV]: crate::items::SoftwareRevision
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
    /// #### Description
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
    /// #### Description
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
    pub struct OffLineAck(OffLineAcknowledge);
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
    /// #### Description
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
    /// #### Description
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
    pub struct OnLineAck(OnLineAcknowledge);
    message_data!{OnLineAck, false, 1, 16}
  }
}
