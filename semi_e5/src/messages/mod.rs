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

//! # MESSAGES
//! **Based on SEMI E5§10**
//! 
//! ---------------------------------------------------------------------------
//! 
//! Standards compliant [Message] structures designed to express and enforce
//! the specific [Item] contents of each [Message].
//! 
//! Each such message defined herein implements:
//! - [From]\<T\> for [Message]
//! - [TryFrom]\<[Message]\> for T
//! 
//! ---------------------------------------------------------------------------
//! 
//! Groups of [Message]s are broken into separate module based on their
//! [Stream] as defined by the standard.
//! 
//! [Message]: crate::Message
//! [Stream]:  crate::Message::stream
//! [Item]:    crate::Item

/// ## MESSAGE MACRO: HEADER ONLY
/// 
/// To be used with particular messages that contain only a header.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Arguments
/// 
/// - **$name**: Name of struct.
/// - **$w**: W-bit of message.
/// - **$stream**: Stream of message.
/// - **$function**: Function of message.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Expansion
/// 
/// - From\<$name\> for Message
/// - TryFrom\<Message\> for $name
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
/// 
/// To be used with particular messages that contain arbitrary data.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Arguments
/// 
/// - **$name**: Name of struct.
/// - **$w**: W-bit of message.
/// - **$stream**: Stream of message.
/// - **$function**: Function of message.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Expansion
/// 
/// - From\<$name\> for Message
/// - TryFrom\<Message\> for $name
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

/// ## MESSAGE MACRO: ITEM
/// 
/// To be used with particular messages that contain just an Item.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Arguments
/// 
/// - **$name**: Name of struct.
/// - **$w**: W-bit of message.
/// - **$stream**: Stream of message.
/// - **$function**: Function of message.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Expansion
/// 
/// - From\<$name\> for Message
/// - TryFrom\<Message\> for $name
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
          Some(item) => {Ok(Self(item))},
          None => Err(WrongFormat),
        }
      }
    }
  }
}

pub mod s1;
pub mod s2;

/// # STREAM 3: MATERIAL STATUS
/// **Based on SEMI E5§10.7**
/// 
/// ---------------------------------------------------------------------------
/// 
/// [Message]s which deal with communicating information and actions related
/// to material, including carriers and material-in-process,
/// time-to-completion information, and extraordinary material circumstances.
/// 
/// ---------------------------------------------------------------------------
/// 
/// ## TO BE DONE
/// 
/// - Fill out stream contents
/// 
/// [Message]: crate::Message
pub mod s3 {}

/// # STREAM 4: MATERIAL CONTROL
/// **Based on SEMI E5§10.8**
/// 
/// ---------------------------------------------------------------------------
/// 
/// [Message]s which deal with the original material control protocol and the
/// newer protocol which supports [SEMI E32].
/// 
/// ---------------------------------------------------------------------------
/// 
/// ## TO BE DONE
/// 
/// - Fill out stream contents
/// 
/// [Message]: crate::Message
pub mod s4 {}

/// # STREAM 5: EXCEPTION HANDLING
/// **Based on SEMI E5§10.9**
/// 
/// ---------------------------------------------------------------------------
/// 
/// [Message]s which deal with binary and analog equipment exceptions.
/// 
/// Exceptions are classified into two categories: Errors and Alarms
/// 
/// ---------------------------------------------------------------------------
/// 
/// [Message]s S5F1 through S5F8 provide basic alarm messages, which may
/// be divided into the following categories:
/// 
/// - Personal Safety - Condition may be dangerous to people.
/// - Equipment Safety - Condition may harm equipment.
/// - Parameter Control Warning - Parameter variation outside of preset
///   limits - may harm product.
/// - Parameter Control Error - Parameter variation outside of reasonable
///   control limits - may indicate an equipment malfunction.
/// - Irrecoverable Error - Intervention required before normal use of
///   equipment can resume.
/// - Equipment Status Warning - An unexpected condition has occurred, but
///   operation can continue.
/// - Attention Flags - A signal from a process program indicating that a
///   particular step has been reached.
/// - Data Integrity - A condition which may cause loss of data; usually
///   related to [Stream 6].
/// 
/// It will be the equipment's responsibility to categorize alarms.
/// 
/// Some alarm conditions may cause more than one type of alarm to be issued.
/// 
/// ---------------------------------------------------------------------------
/// 
/// [Message]s S5F9 through S5F15 provide extended capabilities for
/// exception handling.
/// 
/// ---------------------------------------------------------------------------
/// 
/// ## TO BE DONE
/// 
/// - Fill out stream contents
/// 
/// [Message]: crate::Message
/// [Stream 6]: crate::messages::s6
pub mod s5 {}

/// # STREAM 6: DATA COLLECTION
/// **Based on SEMI E5§10.10**
/// 
/// ---------------------------------------------------------------------------
/// 
/// [Message]s which deal with in-process measurement and equipment
/// monitoring.
/// 
/// ---------------------------------------------------------------------------
/// 
/// ## TO BE DONE
/// 
/// - Fill out stream contents
/// 
/// [Message]: crate::Message
pub mod s6 {}

/// # STREAM 7: PROCESS PROGRAM MANAGEMENT
/// **Based on SEMI E5§10.11**
/// 
/// ---------------------------------------------------------------------------
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
/// ---------------------------------------------------------------------------
/// 
/// ## TO BE DONE
/// 
/// - Fill out stream contents
/// 
/// [Message]: crate::Message
pub mod s7 {}

/// # STREAM 8: CONTROL PROGRAM TRANSFER
/// **Based on SEMI E5§10.12**
/// 
/// ---------------------------------------------------------------------------
/// 
/// [Message]s which deal with transmitting the programs used in the equipment
/// to perform the control function or to execute the transmitted Process
/// Program.
/// 
/// ---------------------------------------------------------------------------
/// 
/// ## TO BE DONE
/// 
/// - Fill out stream contents
/// 
/// [Message]: crate::Message
pub mod s8 {}

/// # STREAM 9: SYSTEM ERRORS
/// **Based on SEMI E5§10.13**
/// 
/// ---------------------------------------------------------------------------
/// 
/// [Message]s which deal with informing the host of communication errors,
/// particularly that a message block has been received which cannot be
/// handled or that a timeout on a transaction reception timer has occurred.
/// 
/// The messages indicate either a Message Fault or a Communications Fault
/// has occurred but do not indicate a Communications Failure has occurred.
/// 
/// ---------------------------------------------------------------------------
/// 
/// ## TO BE DONE
/// 
/// - Fill out stream contents
/// 
/// [Message]: crate::Message
pub mod s9 {}

/// # STREAM 10: TERMINAL SERVICES
/// **Based on SEMI E5§10.14**
/// 
/// ---------------------------------------------------------------------------
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
/// ---------------------------------------------------------------------------
/// 
/// ## TO BE DONE
/// 
/// - Fill out stream contents
/// 
/// [Message]: crate::Message
pub mod s10 {}

/// # STREAM 11: DELETED
/// **Based on SEMI E5§10.15**
/// 
/// ---------------------------------------------------------------------------
/// 
/// The [Message]s in this stream have been deprecated and no longer appear
/// in the standard as of 1989.
/// 
/// ---------------------------------------------------------------------------
/// 
/// ## TO BE DONE
/// 
/// - Fill out stream contents
/// 
/// [Message]: crate::Message
pub mod s11 {}

/// # STREAM 12: WAFER MAPPING
/// **Based on SEMI E5§10.16**
/// 
/// ---------------------------------------------------------------------------
/// 
/// [Message]s which deal with coordinate positions and data associated with
/// those positions.
/// 
/// This includes functions such as wafer mapping with the coordinates of die
/// on wafer maps to and from the process equipment.
/// 
/// ---------------------------------------------------------------------------
/// 
/// S12F1 through S12F20 address the variations required by semiconductor
/// equipment manufactureers in transmitting wafer maps to and from the
/// process equipment.
/// 
/// The functions include three basic formats:
/// 
/// - Row/Column - A coordinate row starting position is given with die count
///   in the row and starting direction. The respective binning information
///   follows each die.
/// - Array - A matrix array captures all or part of a wafer with the
///   associated binning information.
/// - Coordinate - An X/Y location and bin code for die on the wafer.
/// 
/// ---------------------------------------------------------------------------
/// 
/// ## TO BE DONE
/// 
/// - Complete this documentation
/// - Fill out stream contents
/// 
/// [Message]: crate::Message
pub mod s12 {}

/// # STREAM 13: DATA SET TRANSFER
/// **Based on SEMI E5§10.17**
/// 
/// ---------------------------------------------------------------------------
/// 
/// [Message]s which deal with the transfer of data sets between systems.
/// 
/// It is not intended to provide a general file access mechanism.
/// 
/// ---------------------------------------------------------------------------
/// 
/// ## TO BE DONE
/// 
/// - Complete this documentation
/// - Fill out stream contents
/// 
/// [Message]: crate::Message
pub mod s13 {}

/// # STREAM 14: OBJECT SERVICES
/// **Based on SEMI E5§10.18**
/// 
/// ---------------------------------------------------------------------------
/// 
/// [Message]s which deal with generic functions concerning objects,
/// including obtaining information about objects and setting values for an
/// object.
/// 
/// ---------------------------------------------------------------------------
/// 
/// ## TO BE DONE
/// 
/// - Fill out stream contents
/// 
/// [Message]: crate::Message
pub mod s14 {}

/// # STREAM 15: RECIPE MANAGEMENT
/// **Based on SEMI E5§10.19**
/// 
/// ---------------------------------------------------------------------------
/// 
/// [Message]s which deal with requestion information and operations
/// concerning recipes, recipe namespaces, and recipe executors.
/// 
/// ---------------------------------------------------------------------------
/// 
/// A recipe is an object that is transferred in sections, where a section
/// consists of either recipe attributes, agent-specific dataset attributes,
/// or the body of the recipe.
/// 
/// An attribute is information concerning the recipe body, the recipe as a
/// whole, or the application of the recipe, and consists of a name/value
/// pair.
/// 
/// ---------------------------------------------------------------------------
/// 
/// ## TO BE DONE
/// 
/// - Fill out stream contents
/// 
/// [Message]: crate::Message
pub mod s15 {}

/// # STREAM 16: PROCESSING MANAGEMENT
/// **Based on SEMI E5§10.20**
/// 
/// ---------------------------------------------------------------------------
/// 
/// [Message]s which deal with control of material processing at equipment
/// and equipment resources.
/// 
/// ---------------------------------------------------------------------------
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
/// ---------------------------------------------------------------------------
/// 
/// ## TO BE DONE
/// 
/// - Fill out stream contents
/// 
/// [Message]: crate::Message
pub mod s16 {}

/// # STREAM 17: EQUIPMENT CONTROL AND DIAGNOSTICS
/// **Based on SEMI E5§10.21**
/// 
/// ---------------------------------------------------------------------------
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
/// ---------------------------------------------------------------------------
/// 
/// This is a continuation of [Stream 2].
/// 
/// ---------------------------------------------------------------------------
/// 
/// ## TO BE DONE
/// 
/// - Fill out stream contents
/// 
/// [Message]: crate::Message
/// [Stream 2]: crate::messages::s2
/// [Stream 4]: crate::messages::s4
/// [Stream 8]: crate::messages::s8
/// [Stream 10]: crate::messages::s10
/// [Stream 13]: crate::messages::s13
pub mod s17 {}

/// # STREAM 18: SUBSYSTEM CONTROL AND DATA
/// **Based on SEMI E5§10.22**
/// 
/// ---------------------------------------------------------------------------
/// 
/// [Message]s which deal with interfacing between component subsystems and
/// higher level controllers.
/// 
/// Compared to similar mesages exchanged between equipment and host,
/// subsystem messages are less complex.
/// 
/// ---------------------------------------------------------------------------
/// 
/// ## TO BE DONE
/// 
/// - Fill out stream contents
/// 
/// [Message]: crate::Message
pub mod s18 {}

/// # STREAM 19: RECIPE AND PARAMETER MANAGEMENT
/// **Based on SEMI E5§10.23**
/// 
/// ---------------------------------------------------------------------------
/// 
/// [Message]s which deal with management of recipes that include:
/// 
/// - Self-documenting recipe component headers.
/// - Support for multi-part recipes.
/// - User-configured parameters.
/// - Full assurance of byte integrity of PDE content.
/// 
/// ---------------------------------------------------------------------------
/// 
/// Definitions:
/// 
/// - PDE - Process Definition Element - A component of a recipe, including
///   an informational PDEheader and execution content PDEbody.
/// - Recipe - Instructions or data that direct equipment behavior. A recipe
///   is composed of one or more PDEs.
/// - UID - Unique IDentifier - Used to identify a PDE.
/// - GID - Group IDentifier - Used to identify PDEs that are subsitutable
///   for one another.
/// - InputMap, OutputMap - Data used to resolve references between PDEs in a
///   multiple component recipe. These maps consist of a list of GID with the
///   corresponding UID.
/// - Resolve - Determination of all the components in a multi-part recipe.
///   This is the process of creating an Outputmap that satisfies all the
///   PDEs in a recipe.
/// - TransferContainer - A group of PDEs or PDEheaders bound together as a
///   single [Stream 13] Data Set for transfer.
/// 
/// ---------------------------------------------------------------------------
/// 
/// ## TO BE DONE
/// 
/// - Fill out stream contents
/// 
/// [Message]: crate::Message
/// [Stream 13]: crate::messages::s13
pub mod s19 {}

/// # STREAM 20: RECIPE MANAGEMENT SYSTEM
/// 
/// The definition of this stream exists in a newer version of the standard
/// as compared to SEMI E5-0813.
/// 
/// ---------------------------------------------------------------------------
/// 
/// ## TO BE DONE
/// 
/// - Complete this documentation
/// - Fill out stream contents
pub mod s20 {}

/// # STREAM 21: ITEM TRANSFER
/// 
/// The definition of this stream exists in a newer version of the standard
/// as compared to SEMI E5-0813.
/// 
/// ---------------------------------------------------------------------------
/// 
/// ## TO BE DONE
/// 
/// - Complete this documentation
/// - Fill out stream contents
pub mod s21 {}
