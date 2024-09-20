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

//! # HIGH-SPEED SECS MESSAGE SERVICES (HSMS)
//! 
//! Copyright © 2024 Nathaniel Hardesty, Licensed under the MIT License
//! 
//! This software is created by a third-party and not endorsed or supported by
//! SEMI.
//! 
//! The codebase will be updated to reflect more up-to-date SEMI standards
//! if/when they can be acquired for this purpose.
//! 
//! ---------------------------------------------------------------------------
//! 
//! **Based on:**
//! - **[SEMI E37]-1109**
//! - **[SEMI E37].1-0702**
//! 
//! ---------------------------------------------------------------------------
//! 
//! HSMS is a protocol designed to facilitate the reliable transmission of
//! messages between semiconductor equipment over TCP/IP.
//! 
//! Most commonly, exchanged messages are encoded with the SECS-II ([SEMI E5])
//! protocol.
//! 
//! For ease of programming and extension, the functionality of the protocol
//! has been divided into a few subsets: the Primitive Services, which manages
//! the TCP/IP connection and the sending of messages with proper headers; the
//! Generic Services, which manages the sending of messages of particular types
//! and at particular times as allowed by the protocol; and the Single Selected
//! Session Services, which manages the restriction of the protocol to
//! scenarios involving a single host/equipment pair in communication.
//! 
//! ---------------------------------------------------------------------------
//! 
//! ## Primitive Services
//! 
//! Defines the most agnostic form in which data can be exchanged persuant to
//! the [HSMS] protocol and any subsidary protocols. This is not necessarily
//! outlined by the standard, but is an important piece of establishing and
//! maintaining proper communications.
//! 
//! To use the Primitive Services:
//! - Build [Primitive Message]s which use [Primitive Message Header]s.
//! - Create a [Primitive Client] with the [New Primitive Client] function.
//! - Manage the [Connection State] with the [Primitive Connect Procedure]
//!   and [Primitive Disconnect Procedure].
//! - Receive [Primitive Message]s with the hook provided by the
//!   [Primitive Connect Procedure].
//! - Transmit [Primitive Message]s with the [Primitive Transmit Procedure].
//! 
//! ---------------------------------------------------------------------------
//! 
//! ## Generic Services
//! 
//! Defines the full functionality of the [HSMS] protocol without modification
//! by any subsidiary standards.
//! 
//! To use the HSMS Generic Services:
//! - Build [HSMS Message]s which use an [HSMS Message ID] and
//!   [HSMS Message Contents]:
//!   - [Data Message]
//!   - [Select.req]
//!   - [Select.rsp]
//!   - [Deselect.req]
//!   - [Deselect.rsp]
//!   - [Linktest.req]
//!   - [Linktest.rsp]
//!   - [Reject.req]
//!   - [Separate.req]
//! - Create an [HSMS Client] by providing the [New HSMS Client] function
//!   with [Parameter Settings].
//! - Manage the [Connection State] with the [HSMS Connect Procedure] and
//!   [HSMS Disconnect Procedure].
//! - Manage the [Selection State] with the [HSMS Select Procedure],
//!   [HSMS Deselect Procedure], and [HSMS Separate Procedure].
//! - Receive [Data Message]s with the hook provided by the
//!   [HSMS Connect Procedure].
//! - Test connection integrity with the [HSMS Linktest Procedure].
//! - Send [Data Message]s with the [HSMS Data Procedure].
//! - Send [Reject.req] messages [HSMS Reject Procedure].
//! 
//! ---------------------------------------------------------------------------
//! 
//! ## Single Selected-Session Services
//! 
//! Not yet implemented.
//! 
//! ---------------------------------------------------------------------------
//! 
//! ## TODO
//! 
//! - [HSMS Client] - [HSMS Deselect Procedure]
//! - [HSMS Client] - [HSMS Reject Procedure]
//! - [HSMS Client] - "Simultaneous Select Procedure"
//! - [HSMS Client] - "Simultaneous Deselect Procedure"
//! - HSMS-SS
//! 
//! ---------------------------------------------------------------------------
//! 
//! ## Referenced Standards
//! 
//! - SEMI E4             - SEMI Equipment Communications Standard 1 Message Transfer (SECS-I)
//! - SEMI E5             - SEMI Equipment Communications Standard 2 Message Content (SECS-II)
//! - IETF RFC 791        - Internet Protocol (IP)
//! - IETF RFC 792        - Internet Control Message Protocol (ICMP)
//! - IETF RFC 793        - Transmission Control Protocol (TCP)
//! - IETF RFC 1120       - Requirements for Internet Hosts - Communication Layers
//! - IETF RFC 1340       - Assigned Numbers
//! - IEEE POSIX P1003.12 - Protocol Independent Interfaces (PII)
//! 
//! [SEMI E4]:  https://store-us.semi.org/products/e00400-semi-e4-specification-for-semi-equipment-communications-standard-1-message-transfer-secs-i
//! [SEMI E5]:  https://store-us.semi.org/products/e00500-semi-e5-specification-for-semi-equipment-communications-standard-2-message-content-secs-ii
//! [SEMI E30]: https://store-us.semi.org/products/e03000-semi-e30-specification-for-the-generic-model-for-communications-and-control-of-manufacturing-equipment-gem
//! [SEMI E37]: https://store-us.semi.org/products/e03700-semi-e37-high-speed-secs-message-services-hsms-generic-services
//! 
//! [HSMS]:                           crate
//! [Selection State]:                SelectionState
//! [NOT SELECTED]:                   SelectionState::NotSelected
//! [SELECTED]:                       SelectionState::Selected
//! [Parameter Settings]:             ParameterSettings
//! [Connect Mode]:                   ParameterSettings::connect_mode
//! [T3]:                             ParameterSettings::t3
//! [T5]:                             ParameterSettings::t5
//! [T6]:                             ParameterSettings::t6
//! [T7]:                             ParameterSettings::t7
//! [T8]:                             ParameterSettings::t8
//! [Primitive Message]:              PrimitiveMessage
//! [Primitive Message Text]:         PrimitiveMessage::text
//! [Primitive Message Header]:       PrimitiveMessageHeader
//! [Session ID]:                     PrimitiveMessageHeader::session_id
//! [Byte 2]:                         PrimitiveMessageHeader::byte_2
//! [Byte 3]:                         PrimitiveMessageHeader::byte_3
//! [System Bytes]:                   PrimitiveMessageHeader::system
//! [Primitive Client]:               PrimitiveClient
//! [New Primitive Client]:           PrimitiveClient::new
//! [Primitive Connect Procedure]:    PrimitiveClient::connect
//! [Primitive Disconnect Procedure]: PrimitiveClient::disconnect
//! [Primitive Transmit Procedure]:   PrimitiveClient::transmit
//! [Connection State]:               ConnectionState
//! [NOT CONNECTED]:                  ConnectionState::NotConnected
//! [CONNECTED]:                      ConnectionState::Connected
//! [Connection Mode]:                ConnectionMode
//! [PASSIVE]:                        ConnectionMode::Passive
//! [ACTIVE]:                         ConnectionMode::Active
//! [HSMS Message]:                   HsmsMessage
//! [HSMS Message ID]:                HsmsMessageID
//! [HSMS Message Contents]:          HsmsMessageContents
//! [Data Message]:                   HsmsMessageContents::DataMessage
//! [Select.req]:                     HsmsMessageContents::SelectRequest
//! [Select.rsp]:                     HsmsMessageContents::SelectResponse
//! [Deselect.req]:                   HsmsMessageContents::DeselectRequest
//! [Deselect.rsp]:                   HsmsMessageContents::DeselectResponse
//! [Linktest.req]:                   HsmsMessageContents::LinktestRequest
//! [Linktest.rsp]:                   HsmsMessageContents::LinktestResponse
//! [Reject.req]:                     HsmsMessageContents::RejectRequest
//! [Separate.req]:                   HsmsMessageContents::SeparateRequest
//! [HSMS Client]:                    HsmsClient
//! [New HSMS Client]:                HsmsClient::new
//! [HSMS Connect Procedure]:         HsmsClient::connect
//! [HSMS Disconnect Procedure]:      HsmsClient::disconnect
//! [HSMS Data Procedure]:            HsmsClient::data
//! [HSMS Select Procedure]:          HsmsClient::select
//! [HSMS Deselect Procedure]:        HsmsClient::deselect
//! [HSMS Linktest Procedure]:        HsmsClient::linktest
//! [HSMS Separate Procedure]:        HsmsClient::separate
//! [HSMS Reject Procedure]:          HsmsClient::reject
//! [Presentation Type]:              PresentationType
//! [Session Type]:                   SessionType

use std::{
  collections::HashMap,
  io::{
    Error,
    ErrorKind,
    Read,
    Write,
  },
  net::{
    Shutdown,
    TcpListener,
    TcpStream,
    ToSocketAddrs,
  },
  ops::{
    Deref,
    DerefMut,
  },
  sync::{
    mpsc::{
      channel,
      Receiver,
      Sender,
    },
    Arc,
    Mutex,
    RwLock
  },
  thread::{
    self,
    JoinHandle,
  },
  time::Duration,
};
use oneshot::Sender as SendOnce;

// PRIMITIVE SERVICES

/// ## PRIMITIVE MESSAGE
/// **Based on SEMI E37-1109§8.2**
/// 
/// Data using the [HSMS] defined structure, but not enforcing compliance
/// with the standards for how its fields are filled and what they mean.
/// 
/// Note that the Message Length field defined in the standard is not included,
/// as it is only temporarily used when a message is received or transmitted
/// by the [Primitive Client].
/// 
/// [HSMS]:             crate
/// [Primitive Client]: PrimitiveClient
#[derive(Clone, Debug)]
pub struct PrimitiveMessage {
  /// ### MESSAGE HEADER
  /// 
  /// Contains information about the [Primitive Message] using the
  /// [Primitive Message Header] format.
  /// 
  /// [Primitive Message]:        PrimitiveMessage
  /// [Primitive Message Header]: PrimitiveMessageHeader
  pub header: PrimitiveMessageHeader,

  /// ### MESSAGE TEXT
  /// 
  /// Contains the [Primitive Message]'s content, whose layout is defined by
  /// its [Presentation Type] and [Session Type].
  /// 
  /// [Primitive Message]: PrimitiveMessage
  /// [Presentation Type]: PrimitiveMessageHeader::presentation_type
  /// [Session Type]:      PrimitiveMessageHeader::session_type
  pub text: Vec<u8>,
}
impl From<&PrimitiveMessage> for Vec<u8> {
  /// ### SERIALIZE MESSAGE
  /// 
  /// Converts a [Primitive Message] into raw bytes.
  /// 
  /// [Primitive Message]: PrimitiveMessage
  fn from(val: &PrimitiveMessage) -> Self {
    let mut vec: Vec<u8> = vec![];
    let header_bytes: [u8;10] = val.header.into();
    vec.extend(header_bytes.iter());
    vec.extend(&val.text);
    vec
  }
}
impl TryFrom<Vec<u8>> for PrimitiveMessage {
  type Error = ();

  /// ### DESERIALIZE MESSAGE
  /// 
  /// Converts raw bytes into a [Primitive Message].
  /// 
  /// [Primitive Message]: PrimitiveMessage
  fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
    if bytes.len() < 10 {return Err(())}
    Ok(Self {
      header: PrimitiveMessageHeader::from(<[u8;10]>::try_from(&bytes[0..10]).map_err(|_| ())?),
      text: bytes[10..].to_vec(),
    })
  }
}

/// ## PRIMITIVE MESSAGE HEADER
/// **Based on SEMI E37-1109§8.2.5-8.2.6**
/// 
/// A 10 byte field describing the contents of a [Primitive Message].
/// 
/// [Primitive Message]: PrimitiveMessage
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PrimitiveMessageHeader {
  /// ### SESSION ID
  /// **Based on SEMI E37-1109§8.2.6.1**
  /// 
  /// Provides an association between [Primitive Message]s across multiple
  /// transactions.
  /// 
  /// [Primitive Message]: PrimitiveMessage
  pub session_id : u16,

  /// ### HEADER BYTE 2
  /// **Based on SEMI E37-1109§8.2.6.2**
  /// 
  /// Contains information specific to the [Presentation Type] and
  /// [Session Type].
  /// 
  /// [Presentation Type]: PrimitiveMessageHeader::presentation_type
  /// [Session Type]:      PrimitiveMessageHeader::session_type
  pub byte_2 : u8,

  /// ### HEADER BYTE 3
  /// **Based on SEMI E37-1109§8.2.6.3**
  /// 
  /// Contains information specific to the [Presentation Type] and
  /// [Session Type].
  /// 
  /// [Presentation Type]: PrimitiveMessageHeader::presentation_type
  /// [Session Type]:      PrimitiveMessageHeader::session_type
  pub byte_3 : u8,

  /// ### PRESENTATION TYPE
  /// **Based on SEMI E37-1109§8.2.6.4**
  /// 
  /// An enumerated value, the [Presentation Type], defining the
  /// Presentation Layer content of the [Primitive Message Text].
  /// 
  /// [Primitive Message Text]: PrimitiveMessage::text
  /// [Presentation Type]:      PresentationType
  pub presentation_type : u8,

  /// ### SESSION TYPE
  /// **Based on SEMI E37-1109§8.2.6.5-8.2.6.6**
  /// 
  /// An enumerated value, the [Session Type] defining the specific type of
  /// [HSMS Message] being represented.
  /// 
  /// [Session Type]: SessionType
  /// [HSMS Message]: HsmsMessage
  pub session_type : u8,

  /// ### SYSTEM BYTES
  /// **Based on SEMI E37-1109§8.2.6.7**
  /// 
  /// Identifies a transaction uniquely among the set of open transactions.
  pub system : u32,
}
impl From<PrimitiveMessageHeader> for [u8;10] {
  /// ### SERIALIZE MESSAGE HEADER
  /// 
  /// Converts a [Primitive Message Header] into raw bytes.
  /// 
  /// [Primitive Message Header]: PrimitiveMessageHeader
  fn from(val: PrimitiveMessageHeader) -> Self {
    let mut bytes: [u8;10] = [0;10];
    let session_id_bytes: [u8;2] = val.session_id.to_be_bytes();
    let system_bytes: [u8;4] = val.system.to_be_bytes();
    bytes[0] = session_id_bytes[0];
    bytes[1] = session_id_bytes[1];
    bytes[2] = val.byte_2;
    bytes[3] = val.byte_3;
    bytes[4] = val.presentation_type;
    bytes[5] = val.session_type;
    bytes[6] = system_bytes[0];
    bytes[7] = system_bytes[1];
    bytes[8] = system_bytes[2];
    bytes[9] = system_bytes[3];
    bytes
  }
}
impl From<[u8;10]> for PrimitiveMessageHeader {
  /// ### DESERIALIZE MESSAGE HEADER
  /// 
  /// Converts raw bytes into a [Primitive Message Header].
  /// 
  /// [Primitive Message Header]: PrimitiveMessageHeader
  fn from(bytes: [u8;10]) -> Self {
    Self {
      session_id        : u16::from_be_bytes(bytes[0..2].try_into().unwrap()),
      byte_2            : bytes[2],
      byte_3            : bytes[3],
      presentation_type : bytes[4],
      session_type      : bytes[5],
      system            : u32::from_be_bytes(bytes[6..10].try_into().unwrap()),
    }
  }
}

/// ## PRIMITIVE CLIENT
/// 
/// Encapsulates a limited set of functionality of the [HSMS] protocol referred
/// to as the Primitive Services.
/// 
/// The [Primitive Client] can be used to:
/// - Manage the [Connection State] with the [Primitive Connect Procedure]
///   and [Primitive Disconnect Procedure].
/// - Use the hooks provided by the [Primitive Connect Procedure] to transmit
///   and receive [Primitive Message]s.
/// 
/// [HSMS]:                           crate
/// [Primitive Message]:              PrimitiveMessage
/// [Primitive Client]:               PrimitiveClient
/// [Primitive Connect Procedure]:    PrimitiveClient::connect
/// [Primitive Disconnect Procedure]: PrimitiveClient::disconnect
/// [Connection State]:               ConnectionState
pub struct PrimitiveClient {
  connection_state: RwLock<ConnectionState>,
}

/// ## PRIMITIVE CLIENT: CONNECTION PROCEDURES
/// **Based on SEMI E37-1109§6.3-6.5**
/// 
/// Encapsulates the parts of the [Client]'s functionality dealing with
/// establishing and breaking a TCP/IP connection.
/// 
/// - [New Client]
/// - [Connect Procedure]
/// - [Disconnect Procedure]
/// 
/// [Client]:               PrimitiveClient
/// [New Client]:           PrimitiveClient::new
/// [Connect Procedure]:    PrimitiveClient::connect
/// [Disconnect Procedure]: PrimitiveClient::disconnect
impl PrimitiveClient {
  /// ### NEW PRIMITIVE CLIENT
  /// 
  /// Creates a [Primitive Client] in the [NOT CONNECTED] state, ready to
  /// initiate the [Primitive Connect Procedure].
  /// 
  /// [Primitive Client]:            PrimitiveClient
  /// [Primitive Connect Procedure]: PrimitiveClient::connect
  /// [NOT CONNECTED]:               ConnectionState::NotConnected
  pub fn new() -> Arc<Self> {
    Arc::new(Self {
      connection_state: Default::default(),
    })
  }

  /// ### CONNECT PROCEDURE
  /// **Based on SEMI E37-1109§6.3.4-6.3.7**
  /// 
  /// Connects the [Primitive Client] to the Remote Entity.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [NOT CONNECTED] state to use this
  /// procedure.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Primitive Connect Procedure] has two different behaviors based on
  /// the [Connection Mode] provided to it:
  /// - [PASSIVE] - The socket address of the Local Entity must be provided,
  ///   and the [Primitive Client] listens for and accepts the
  ///   [Primitive Connect Procedure] when initiated by the Remote Entity.
  /// - [ACTIVE] - The socket address of the Remote Entity must be provided,
  ///   and the [Primitive Client] initiates the [Primitive Connect Procedure]
  ///   and waits up to the time specified by [T5] for the Remote Entity to
  ///   respond.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Upon completion of the [Primitive Connect Procedure], the [T8] parameter
  /// is set as the TCP stream's read and write timeout, and the [CONNECTED]
  /// state is entered.
  /// 
  /// [Primitive Client]:            PrimitiveClient
  /// [Primitive Connect Procedure]: PrimitiveClient::connect
  /// [Connection State]:            ConnectionState
  /// [NOT CONNECTED]:               ConnectionState::NotConnected
  /// [CONNECTED]:                   ConnectionState::Connected
  /// [Connection Mode]:             ConnectionMode
  /// [PASSIVE]:                     ConnectionMode::Passive
  /// [ACTIVE]:                      ConnectionMode::Active
  /// [T5]:                          ParameterSettings::t5
  /// [T8]:                          ParameterSettings::t8
  pub fn connect(
    self: &Arc<Self>,
    entity: &str,
    connection_mode: ConnectionMode,
    t5: Duration,
    t8: Duration,
  ) -> Result<Receiver<PrimitiveMessage>, Error> {
    // TCP: CONNECT
    let stream = match self.connection_state.read().unwrap().deref() {
      // IS: NOT CONNECTED
      ConnectionState::NotConnected => {
        match connection_mode {
          // CONNECTION MODE: PASSIVE
          ConnectionMode::Passive => {
            // Create Listener and Wait
            let listener = TcpListener::bind(entity)?;
            let (stream, socket) = listener.accept()?;
            println!("PrimitiveClient::connect {:?}", socket);
            stream
          },
          // CONNECTION MODE: ACTIVE
          ConnectionMode::Active => {
            // Connect with Timeout
            let stream = TcpStream::connect_timeout(
              &entity.to_socket_addrs()?.next().ok_or(Error::new(ErrorKind::AddrNotAvailable, "INVALID ADDRESS"))?, 
              t5,
            )?;
            println!("PrimitiveClient::connect {:?}", entity);
            stream
          },
        }
      },
      // IS: CONNECTED
      _ => return Err(Error::new(ErrorKind::AlreadyExists, "ALREADY CONNECTED")),
    };
    // Set Read and Write Timeouts to T8
    stream.set_read_timeout(Some(t8))?;
    stream.set_write_timeout(Some(t8))?;
    // TO: CONNECTED
    *self.connection_state.write().unwrap().deref_mut() = ConnectionState::Connected(stream);
    // Create Channels
    let (rx_sender, rx_receiver) = channel::<PrimitiveMessage>();
    // Start RX Thread
    let rx_clone: Arc<PrimitiveClient> = self.clone();
    thread::spawn(move || {rx_clone.rx_handle(rx_sender.clone())});
    // Finish
    Ok(rx_receiver)
  }

  /// ### DISCONNECT PROCEDURE
  /// **Based on SEMI E37-1109§6.4-6.5**
  /// 
  /// Disconnects the [Primitive Client] from the Remote Entity.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state to use this
  /// procedure.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Upon completion of the [Primitive Disconnect Procedure], the
  /// [NOT CONNECTED] state is entered.
  /// 
  /// [Primitive Client]:               PrimitiveClient
  /// [Primitive Disconnect Procedure]: PrimitiveClient::disconnect
  /// [Connection State]:               ConnectionState
  /// [NOT CONNECTED]:                  ConnectionState::NotConnected
  /// [CONNECTED]:                      ConnectionState::Connected
  /// [Selection State]:                SelectionState
  /// [SELECTED]:                       SelectionState::Selected
  pub fn disconnect(
    self: &Arc<Self>
  ) -> ConnectionStateTransition {
    match self.connection_state.read().unwrap().deref() {
      // IS: CONNECTED
      ConnectionState::Connected(stream) => {
        println!("PrimitiveClient::disconnect");
        // TCP: SHUTDOWN
        // This should cause all read locks on the connection state to release.
        let _ = stream.shutdown(Shutdown::Both);
      },
      // IS: NOT CONNECTED
      _ => return ConnectionStateTransition::NotConnected,
    }
    // TO: NOT CONNECTED
    *self.connection_state.write().unwrap().deref_mut() = ConnectionState::NotConnected;
    ConnectionStateTransition::ConnectedToNotConnected
  }
}

/// ## PRIMITIVE CLIENT: MESSAGE EXCHANGE PROCEDURES
/// **Based on SEMI E37-1109§7**
/// 
/// Encapsulates the parts of the [Client]'s functionality dealing with
/// exchanging [HSMS Message]s.
/// 
/// [Client]:       PrimitiveClient
/// [HSMS Message]: HsmsMessage
impl PrimitiveClient {
  /// ### RECEPTION HANDLER
  /// 
  /// A [Primitive Client] in the [CONNECTED] state will automatically receive 
  /// [Primitive Message]s via the [Primitive Receive] function, and send them
  /// to the hook provided by the [Primitive Connect Procedure].
  /// 
  /// [Primitive Message]:           PrimitiveMessage
  /// [Primitive Receive]:           primitive_rx
  /// [Primitive Client]:            PrimitiveClient
  /// [Primitive Connect Procedure]: PrimitiveClient::connect
  /// [CONNECTED]:                   ConnectionState::Connected
  fn rx_handle(
    self: Arc<Self>,
    rx_sender: Sender<PrimitiveMessage>,
  ) {
    println!("PrimitiveClient::rx_handle start");
    while let ConnectionState::Connected(stream_immutable) = self.connection_state.read().unwrap().deref() {
      let res = 'rx: {
        let mut stream = stream_immutable;
        // Length [Bytes 0-3]
        let mut length_buffer: [u8;4] = [0;4];
        let length_bytes: usize = match stream.read(&mut length_buffer) {
          Ok(l) => l,
          Err(error) => match error.kind() {
            ErrorKind::TimedOut => {
              break 'rx Ok(None)
            },
            _ => {
              break 'rx Err(error)
            },
          }
        };
        if length_bytes != 4 {
          break 'rx Err(Error::new(ErrorKind::TimedOut, "T8"))
        }
        let length: u32 = u32::from_be_bytes(length_buffer);
        if length < 10 {
          break 'rx Err(Error::new(ErrorKind::InvalidData, "INVALID MESSAGE"))
        }
        // Header + Data [Bytes 4+]
        let mut message_buffer: Vec<u8> = vec![0; length as usize];
        let message_bytes: usize = match stream.read(&mut message_buffer) {
          Ok(message_bytes) => message_bytes,
          Err(error) => break 'rx Err(error),
        };
        if message_bytes != length as usize {
          break 'rx Err(Error::new(ErrorKind::TimedOut, "T8"))
        }
        // Diagnostic
        println!(
          "rx {: >4X} {: >3}{} {: >3} {: >2X} {: >2X} {: >8X} {:?}",
          u16::from_be_bytes(message_buffer[0..2].try_into().unwrap()),
          &message_buffer[2] & 0b0111_1111,
          if (&message_buffer[2] & 0b1000_0000) > 0 {'W'} else {' '},
          &message_buffer[3],
          &message_buffer[4],
          &message_buffer[5],
          u32::from_be_bytes(message_buffer[6..10].try_into().unwrap()),
          &message_buffer[10..],
        );
        // Finish
        match PrimitiveMessage::try_from(message_buffer) {
          Ok(message) => Ok(Some(message)),
          Err(_) => break 'rx Err(Error::new(ErrorKind::InvalidData, "INVALID MESSAGE")),
        }
      };
      match res {
        // RX: SUCCESS
        Ok(optional_rx_message) => if let Some(rx_message) = optional_rx_message {
          if rx_sender.send(rx_message).is_err() {break}
        },
        //RX Failure
        Err(_error) => break,
      }
    }
    let _ = self.disconnect();
    println!("PrimitiveClient::rx_handle end");
  }

  /// ### TRANSMIT MESSAGE
  /// **Based on SEMI E37-1109§7.2**
  /// 
  /// Serializes a [Primitive Message] and transmits it over the TCP/IP
  /// connection.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state to use this
  /// procedure.
  /// 
  /// [Primitive Message]: PrimitiveMessage
  /// [Connection State]:  ConnectionState
  /// [CONNECTED]:         ConnectionState::Connected
  pub fn transmit(
    self: &Arc<Self>,
    message: PrimitiveMessage,
  ) -> Result<(), ConnectionStateTransition> {
    match self.connection_state.read().unwrap().deref() {
      ConnectionState::Connected(stream_immutable) => 'lock: {
        let mut stream: &TcpStream = stream_immutable;
        // Header + Data [Bytes 4+]
        let message_buffer: Vec<u8> = (&message).into();
        // Length [Bytes 0-3]
        let length: u32 = message_buffer.len() as u32;
        let length_buffer: [u8; 4] = length.to_be_bytes();
        // Diagnostic
        println!(
          "tx {: >4X} {: >3}{} {: >3} {: >2X} {: >2X} {: >8X} {:?}",
          u16::from_be_bytes(message_buffer[0..2].try_into().unwrap()),
          &message_buffer[2] & 0b0111_1111,
          if (&message_buffer[2] & 0b1000_0000) > 0 {'W'} else {' '},
          &message_buffer[3],
          &message_buffer[4],
          &message_buffer[5],
          u32::from_be_bytes(message_buffer[6..10].try_into().unwrap()),
          &message_buffer[10..],
        );
        // Write
        if stream.write_all(&length_buffer).is_err() {break 'lock};
        if stream.write_all(&message_buffer).is_err() {break 'lock};
        // Finish
        return Ok(())
      },
      _ => return Err(ConnectionStateTransition::NotConnected),
    };
    Err(self.disconnect())
  }
}

/// ## CONNECTION STATE
/// **Based on SEMI E37-1109§5.4-5.5**
/// 
/// In the [HSMS] protocol, two [Connection State]s exist, [NOT CONNECTED],
/// and [CONNECTED].
/// 
/// The [Primitive Client] will move between them based on whether it has
/// established a TCP/IP connection to a Remote Entity, and the integrity of
/// that connection.
/// 
/// [HSMS]:             crate
/// [Primitive Client]: PrimitiveClient
/// [Connection State]: ConnectionState
/// [NOT CONNECTED]:    ConnectionState::NotConnected
/// [CONNECTED]:        ConnectionState::Connected
#[derive(Debug)]
pub enum ConnectionState {
  /// ### NOT CONNECTED
  /// **Based on SEMI E37-1109§5.5.1**
  /// 
  /// In this state, the [Primitive Client] is ready to initiate the
  /// [Connect Procedure] but has either not yet done so, or has terminated a
  /// previous connection.
  /// 
  /// [Primitive Client]:  PrimitiveClient
  /// [Connect Procedure]: PrimitiveClient::connect
  NotConnected,

  /// ### CONNECTED
  /// **Based on SEMI E37-1109§5.5.2**
  /// 
  /// In this state, the [Primitive Client] has successfully initiated the
  /// [Connect Procedure] and is able to send and receive data.
  /// 
  /// [Primitive Client]:  PrimitiveClient
  /// [Connect Procedure]: PrimitiveClient::connect
  Connected(TcpStream)
}
impl Default for ConnectionState {
  /// ### DEFAULT CONNECTION STATE
  /// **Based on SEMI E37-1109§5.4**
  /// 
  /// Provides the [NOT CONNECTED] state by default.
  /// 
  /// [NOT CONNECTED]: ConnectionState::NotConnected
  fn default() -> Self {
    ConnectionState::NotConnected
  }
}

/// ## CONNECTION MODE
/// **Based on SEMI E37-1109§6.3.2**
/// 
/// The [Primitive Client] must use one of two [Connection Mode]s, [PASSIVE] or
/// [ACTIVE], in order to perform the [Primitive Connect Procedure] and attain
/// a TCP/IP connection.
/// 
/// [Primitive Client]:            PrimitiveClient
/// [Primitive Connect Procedure]: PrimitiveClient::connect
/// [Connection Mode]:             ConnectionMode
/// [PASSIVE]:                     ConnectionMode::Passive
/// [ACTIVE]:                      ConnectionMode::Active
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConnectionMode {
  /// ### PASSIVE
  /// **Based on SEMI E37-1109§6.3.2**
  /// 
  /// In this mode, the [Primitive Client] listens for and accepts the
  /// [Connect Procedure] when initiated by another entity.
  /// 
  /// [Primitive Client]:  PrimitiveClient
  /// [Connect Procedure]: PrimitiveClient::connect
  Passive,

  /// ### ACTIVE
  /// **Based on SEMI E37-1109§6.3.2**
  /// 
  /// In this mode, the [Primitive Client] initiates the [Connect Procedure]
  /// and waits up to the time specified by [T5] for the other entity to
  /// respond.
  /// 
  /// [Primitive Client]:  PrimitiveClient
  /// [Connect Procedure]: PrimitiveClient::connect
  /// [T5]:                ParameterSettings::t5
  Active,
}
impl Default for ConnectionMode {
  /// ### DEFAULT CONNECTION MODE
  /// **Based on SEMI E37-1109§5.4**
  /// 
  /// Provides the [PASSIVE] mode by default.
  /// 
  /// [PASSIVE]: ConnectionMode::Passive
  fn default() -> Self {
    ConnectionMode::Passive
  }
}

// HSMS GENERIC SERVICES

/// ## HSMS MESSAGE
/// **Based on SEMI E37-1109§8.2-8.3**
/// 
/// A [Primitive Message] with a [Presentation Type] of 0, broken down into its
/// [HSMS Message ID] and [HSMS Message Contents].
/// 
/// [Primitive Message]:     PrimitiveMessage
/// [Presentation Type]:     PresentationType
/// [HSMS Message ID]:       HsmsMessageID
/// [HSMS Message Contents]: HsmsMessageContents
#[derive(Clone, Debug)]
pub struct HsmsMessage {
  pub id: HsmsMessageID,
  pub contents: HsmsMessageContents,
}
impl From<HsmsMessage> for PrimitiveMessage {
  /// ### HSMS MESSAGE INTO PRIMITIVE MESSAGE
  /// 
  /// Due to the fact that valid HSMS Messages are a subset of valid Messages,
  /// this operation is infallible.
  fn from(hsms_message: HsmsMessage) -> Self {
    match hsms_message.contents {
      HsmsMessageContents::DataMessage(message) => {
        PrimitiveMessage {
          header: PrimitiveMessageHeader {
            session_id        : hsms_message.id.session,
            byte_2            : ((message.w as u8) << 7) | message.stream,
            byte_3            : message.function,
            presentation_type : PresentationType::SecsII as u8,
            session_type      : SessionType::DataMessage as u8,
            system            : hsms_message.id.system,
          },
          text: match message.text {
            Some(item) => Vec::<u8>::from(item),
            None => vec![],
          },
        }
      },
      HsmsMessageContents::SelectRequest => {
        PrimitiveMessage {
          header: PrimitiveMessageHeader {
            session_id        : hsms_message.id.session,
            byte_2            : 0,
            byte_3            : 0,
            presentation_type : PresentationType::SecsII as u8,
            session_type      : SessionType::SelectRequest as u8,
            system            : hsms_message.id.system,
          },
          text: vec![],
        }
      },
      HsmsMessageContents::SelectResponse(select_status) => {
        PrimitiveMessage {
          header: PrimitiveMessageHeader {
            session_id        : hsms_message.id.session,
            byte_2            : 0,
            byte_3            : select_status,
            presentation_type : PresentationType::SecsII as u8,
            session_type      : SessionType::SelectResponse as u8,
            system            : hsms_message.id.system,
          },
          text: vec![],
        }
      },
      HsmsMessageContents::DeselectRequest => {
        PrimitiveMessage {
          header: PrimitiveMessageHeader {
            session_id        : hsms_message.id.session,
            byte_2            : 0,
            byte_3            : 0,
            presentation_type : PresentationType::SecsII as u8,
            session_type      : SessionType::DeselectRequest as u8,
            system            : hsms_message.id.system,
          },
          text: vec![],
        }
      },
      HsmsMessageContents::DeselectResponse(deselect_status) => {
        PrimitiveMessage {
          header: PrimitiveMessageHeader {
            session_id        : hsms_message.id.session,
            byte_2            : 0,
            byte_3            : deselect_status,
            presentation_type : PresentationType::SecsII as u8,
            session_type      : SessionType::DeselectResponse as u8,
            system            : hsms_message.id.system,
          },
          text: vec![],
        }
      },
      HsmsMessageContents::LinktestRequest => {
        PrimitiveMessage {
          header: PrimitiveMessageHeader {
            session_id        : 0xFFFF,
            byte_2            : 0,
            byte_3            : 0,
            presentation_type : PresentationType::SecsII as u8,
            session_type      : SessionType::LinktestRequest as u8,
            system            : hsms_message.id.system,
          },
          text: vec![],
        }
      },
      HsmsMessageContents::LinktestResponse => {
        PrimitiveMessage {
          header: PrimitiveMessageHeader {
            session_id        : 0xFFFF,
            byte_2            : 0,
            byte_3            : 0,
            presentation_type : PresentationType::SecsII as u8,
            session_type      : SessionType::LinktestResponse as u8,
            system            : hsms_message.id.system,
          },
          text: vec![],
        }
      },
      HsmsMessageContents::RejectRequest(message_type, reason_code) => {
        PrimitiveMessage {
          header: PrimitiveMessageHeader {
            session_id        : hsms_message.id.session,
            byte_2            : message_type,
            byte_3            : reason_code,
            presentation_type : PresentationType::SecsII as u8,
            session_type      : SessionType::RejectRequest as u8,
            system            : hsms_message.id.system,
          },
          text: vec![],
        }
      },
      HsmsMessageContents::SeparateRequest => {
        PrimitiveMessage {
          header: PrimitiveMessageHeader {
            session_id        : hsms_message.id.session,
            byte_2            : 0,
            byte_3            : 0,
            presentation_type : PresentationType::SecsII as u8,
            session_type      : SessionType::SeparateRequest as u8,
            system            : hsms_message.id.system,
          },
          text: vec![],
        }
      },
    }
  }
}
impl TryFrom<PrimitiveMessage> for HsmsMessage {
  type Error = RejectReason;

  /// ## HSMS MESSAGE FROM PRIMITIVE MESSAGE
  /// 
  /// Due to the fact that valid [HSMS Message]s are a subset of valid
  /// [Primitive Message]s, this operation is fallable when the
  /// [Primitive Message] is not an [HSMS message].
  /// 
  /// [Primitive Message]: PrimitiveMessage
  /// [HSMS Message]:      HsmsMessage
  fn try_from(message: PrimitiveMessage) -> Result<Self, Self::Error> {
    if message.header.presentation_type != 0 {return Err(RejectReason::UnsupportedPresentationType)}
    Ok(HsmsMessage {
      id: HsmsMessageID {
        session: message.header.session_id,
        system: message.header.system,
      },
      contents: match message.header.session_type {
        0 => {
          HsmsMessageContents::DataMessage(semi_e5::Message{
            stream   : message.header.byte_2 & 0b0111_1111,
            function : message.header.byte_3,
            w        : message.header.byte_2 & 0b1000_0000 > 0,
            text     : match semi_e5::Item::try_from(message.text) {
              // Valid Item
              Ok(text) => Some(text),
              // Invalid Item
              Err(error) => {
                match error {
                  // Empty Text: Considered Valid Here
                  semi_e5::Error::EmptyText => {None},
                  // Other Error: Malformed Data
                  _ => {return Err(RejectReason::MalformedData)}
                }
              },
            },
          })
        },
        1 => {
          if message.header.byte_2 != 0 {return Err(RejectReason::MalformedData)}
          if message.header.byte_3 != 0 {return Err(RejectReason::MalformedData)}
          if !message.text.is_empty()   {return Err(RejectReason::MalformedData)}
          HsmsMessageContents::SelectRequest
        },
        2 => {
          if message.header.byte_2 != 0 {return Err(RejectReason::MalformedData)}
          if !message.text.is_empty()   {return Err(RejectReason::MalformedData)}
          HsmsMessageContents::SelectResponse(message.header.byte_3)
        },
        3 => {
          if message.header.byte_2 != 0 {return Err(RejectReason::MalformedData)}
          if message.header.byte_3 != 0 {return Err(RejectReason::MalformedData)}
          if !message.text.is_empty()   {return Err(RejectReason::MalformedData)}
          HsmsMessageContents::DeselectRequest
        },
        4 => {
          if message.header.byte_2 != 0 {return Err(RejectReason::MalformedData)}
          if !message.text.is_empty()   {return Err(RejectReason::MalformedData)}
          HsmsMessageContents::DeselectResponse(message.header.byte_3)
        },
        5 => {
          if message.header.session_id != 0xFFFF {return Err(RejectReason::MalformedData)}
          if message.header.byte_2     != 0      {return Err(RejectReason::MalformedData)}
          if message.header.byte_3     != 0      {return Err(RejectReason::MalformedData)}
          if !message.text.is_empty()            {return Err(RejectReason::MalformedData)}
          HsmsMessageContents::LinktestRequest
        },
        6 => {
          if message.header.session_id != 0xFFFF {return Err(RejectReason::MalformedData)}
          if message.header.byte_2     != 0      {return Err(RejectReason::MalformedData)}
          if message.header.byte_3     != 0      {return Err(RejectReason::MalformedData)}
          if !message.text.is_empty()            {return Err(RejectReason::MalformedData)}
          HsmsMessageContents::LinktestResponse
        },
        7 => {
          if !message.text.is_empty() {return Err(RejectReason::MalformedData)}
          HsmsMessageContents::RejectRequest(message.header.byte_2, message.header.byte_3)
        },
        9 => {
          if message.header.byte_2 != 0 {return Err(RejectReason::MalformedData)}
          if message.header.byte_3 != 0 {return Err(RejectReason::MalformedData)}
          if !message.text.is_empty()   {return Err(RejectReason::MalformedData)}
          HsmsMessageContents::SeparateRequest
        },
        _ => {return Err(RejectReason::UnsupportedSessionType)}
      },
    })
  }
}

/// ## HSMS MESSAGE ID
/// **Based on SEMI E37-1109§8.2**
/// 
/// The uniquely identifying components of an [HSMS Message] in forming a valid
/// transaction, including the [Session ID] and [System Bytes].
/// 
/// [HSMS Message]: HsmsMessage
/// [Session ID]:   HsmsMessageID::session
/// [System Bytes]: HsmsMessageID::system
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HsmsMessageID {
  /// ### SESSION ID
  /// **Based on SEMI E37-1109§8.2.6.1**
  /// 
  /// Provides an association between [HSMS Message]s across multiple,
  /// transactions, particularly to link the [Select Procedure] and
  /// [Deselect Procedure] to subsequent [Data Messages].
  /// 
  /// [HSMS Message]: HsmsMessage
  pub session: u16,

  /// ### SYSTEM BYTES
  /// **Based on SEMI E37-1109§8.2.6.7**
  /// 
  /// Identifies a transaction uniquely among the set of open transactions.
  pub system: u32,
}

/// ## HSMS MESSAGE CONTENTS
/// **Based on SEMI E37-1109§8.3.1-8.3.21**
/// 
/// The contents of an [HSMS Message], broken down by their [Session Type]:
/// 
/// - SECS-II [Data Message]
/// - [Select.req]
/// - [Select.rsp]
/// - [Deselect.req]
/// - [Deselect.rsp]
/// - [Linktest.req]
/// - [Linktest.rsp]
/// - [Reject.req]
/// - [Separate.req]
/// 
/// [HSMS Message]: HsmsMessage
/// [Session Type]: SessionType
/// [Data Message]: HsmsMessageContents::DataMessage
/// [Select.req]:   HsmsMessageContents::SelectRequest
/// [Select.rsp]:   HsmsMessageContents::SelectResponse
/// [Deselect.req]: HsmsMessageContents::DeselectRequest
/// [Deselect.rsp]: HsmsMessageContents::DeselectResponse
/// [Linktest.req]: HsmsMessageContents::LinktestRequest
/// [Linktest.rsp]: HsmsMessageContents::LinktestResponse
/// [Reject.req]:   HsmsMessageContents::RejectRequest
/// [Separate.req]: HsmsMessageContents::SeparateRequest
#[repr(u8)]
#[derive(Clone, Debug)]
pub enum HsmsMessageContents {
  /// ## DATA MESSAGE
  /// **Based on SEMI E37-1109§8.3.1-8.3.3**
  /// 
  /// An [HSMS Message] with a [Session Type] of 0, used by the initiator of or
  /// responding entity in the [Data Procedure] to send data.
  /// 
  /// Contains SECS-II formatted data.
  /// 
  /// [HSMS Message]:   HsmsMessage
  /// [Session Type]:   SessionType
  /// [Data Procedure]: HsmsClient::data
  DataMessage(semi_e5::Message) = SessionType::DataMessage as u8,

  /// ## SELECT REQUEST
  /// **Based on SEMI E37-1109§8.3.4**
  /// 
  /// An [HSMS Message] with a [Session Type] of 1, used by the initiator of the
  /// [Select Procedure] for establishing communications.
  /// 
  /// [HSMS Message]:     HsmsMessage
  /// [Select Procedure]: HsmsClient::select
  /// [Session Type]:     SessionType
  SelectRequest = SessionType::SelectRequest as u8,

  /// ## SELECT RESPONSE
  /// **Based on SEMI E37-1109§8.3.5-8.3.7**
  /// 
  /// An [HSMS Message] with a [Session Type] of 2, used by the responding
  /// entity in the [Select Procedure].
  /// 
  /// Contains a [Select Status], indicating the success or failure mode of
  /// the [Select Procedure].
  /// 
  /// [HSMS Message]:     HsmsMessage
  /// [Select Procedure]: HsmsClient::select
  /// [Session Type]:     SessionType
  /// [Select Status]:    SelectStatus
  SelectResponse(u8) = SessionType::SelectResponse as u8,

  /// ## DESELECT REQUEST
  /// **Based on SEMI E37-1109§8.3.8-8.3.10**
  /// 
  /// An [HSMS Message] with a [Session Type] of 3, used by the initiator of the
  /// [Deselect Procedure] for breaking communications.
  /// 
  /// [HSMS Message]:       HsmsMessage
  /// [Deselect Procedure]: HsmsClient::deselect
  /// [Session Type]:       SessionType
  DeselectRequest = SessionType::DeselectRequest as u8,

  /// ## DESELECT RESPONSE
  /// **Based on SEMI E37-1109§8.3.11-8.3.13**
  /// 
  /// An [HSMS Message] with a [Session Type] of 4, used by the responding entity
  /// in the [Deselect Procedure].
  /// 
  /// Contains a [Deselect Status], indicating the success or failure mode of
  /// the [Deselect Procedure].
  /// 
  /// [HSMS Message]:       HsmsMessage
  /// [Deselect Procedure]: HsmsClient::deselect
  /// [Session Type]:       SessionType
  /// [Deselect Status]:    DeselectStatus
  DeselectResponse(u8) = SessionType::DeselectResponse as u8,

  /// ## LINKTEST REQUEST
  /// **Based on SEMI E37-1109§8.3.14-8.3.16**
  /// 
  /// An [HSMS Message] with a [Session Type] of 5, used by the initiator of the
  /// [Linktest Procedure] for checking communications stability.
  /// 
  /// [HSMS Message]:       HsmsMessage
  /// [Session Type]:       SessionType
  /// [Linktest Procedure]: HsmsClient::linktest
  LinktestRequest = SessionType::LinktestRequest as u8,

  /// ## LINKTEST RESPONSE
  /// **Based on SEMI E37-1109§8.3.17-8.3.19**
  /// 
  /// An [HSMS Message] with a [Session Type] of 6, used by the responding entity
  /// in the [Linktest Procedure].
  /// 
  /// [HSMS Message]:       HsmsMessage
  /// [Session Type]:       SessionType
  /// [Linktest Procedure]: HsmsClient::linktest
  LinktestResponse = SessionType::LinktestResponse as u8,

  /// ## REJECT REQUEST
  /// **Based on SEMI E37-1109§8.3.20-8.3.21**
  /// 
  /// An [HSMS Message] with a [Session Type] of 7, used by the responding entity
  /// in the [Reject Procedure].
  /// 
  /// Contains the [Presentation Type] or [Session Type] of the message being
  /// rejected, and the [Reason Code] indicating why the message was rejected.
  /// 
  /// [HSMS Message]:      HsmsMessage
  /// [Reject Procedure]:  HsmsClient::reject
  /// [Presentation Type]: PresentationType
  /// [Session Type]:      SessionType
  /// [Reason Code]:       RejectReason
  RejectRequest(u8, u8) = SessionType::RejectRequest as u8,

  /// ## SEPARATE REQUEST
  /// **Based on SEMI E37-1109§8.3.22**
  /// 
  /// An [HSMS Message] with a [Session Type] of 9, used by the initiator of the
  /// [Separate Procedure] for breaking communications.
  /// 
  /// [HSMS Message]:       HsmsMessage
  /// [Separate Procedure]: HsmsClient::separate
  /// [Session Type]:       SessionType
  SeparateRequest = SessionType::SeparateRequest as u8,
}

/// ## HSMS CLIENT
/// 
/// Encapsulates the full functionality of the [HSMS] protocol.
/// 
/// [HSMS]: crate
pub struct HsmsClient {
  parameter_settings: ParameterSettings,
  primitive_client: Arc<PrimitiveClient>,
  selection_state: RwLock<SelectionState>,
  selection_mutex: Mutex<()>,
  outbox: Mutex<HashMap<u32, (HsmsMessageID, SendOnce<Option<HsmsMessage>>)>>,
  system: Mutex<u32>,
}

/// ## HSMS CLIENT: CONNECTION PROCEDURES
/// **Based on SEMI E37-1109§6.3-6.5**
/// 
/// Encapsulates the parts of the [HSMS Client]'s functionality dealing with
/// establishing and breaking a TCP/IP connection.
/// 
/// - [New HSMS Client]
/// - [HSMS Connect Procedure]
/// - [HSMS Disconnect Procedure]
/// 
/// [HSMS Client]:               HsmsClient
/// [New HSMS Client]:           HsmsClient::new
/// [HSMS Connect Procedure]:    HsmsClient::connect
/// [HSMS Disconnect Procedure]: HsmsClient::disconnect
impl HsmsClient {
  /// ### NEW HSMS CLIENT
  /// 
  /// Creates an [HSMS Client] in the [NOT CONNECTED] state, ready to initiate
  /// the [HSMS Connect Procedure].
  /// 
  /// [NOT CONNECTED]:          ConnectionState::NotConnected
  /// [HSMS Client]:            HsmsClient
  /// [HSMS Connect Procedure]: HsmsClient::connect
  pub fn new(
    parameter_settings: ParameterSettings
  ) -> Arc<Self> {
    Arc::new(HsmsClient {
      parameter_settings,
      primitive_client: PrimitiveClient::new(),
      selection_state:  Default::default(),
      selection_mutex:  Default::default(),
      outbox:           Default::default(),
      system:           Default::default(),
    })
  }

  /// ### HSMS CONNECT PROCEDURE
  /// **Based on SEMI E37-1109§6.3.4-6.3.7**
  /// 
  /// Connects the [HSMS Client] to the Remote Entity.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [NOT CONNECTED] to use this
  /// procedure.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [HSMS Connect Procedure] has two different behaviors based on
  /// the [Connection Mode] provided to it:
  /// - [PASSIVE] - The socket address of the Local Entity must be provided,
  ///   and the [HSMS Client] listens for and accepts the
  ///   [HSMS Connect Procedure] when initiated by the Remote Entity.
  /// - [ACTIVE] - The socket address of the Remote Entity must be provided,
  ///   and the [HSMS Client] initiates the [HSMS Connect Procedure] and waits
  ///   up to the time specified by [T5] for the Remote Entity to respond.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Upon completion of the [HSMS Connect Procedure], the [T8] parameter
  /// is set as the TCP stream's read and write timeout, and the [CONNECTED]
  /// state is entered.
  /// 
  /// [Connection State]:       ConnectionState
  /// [NOT CONNECTED]:          ConnectionState::NotConnected
  /// [CONNECTED]:              ConnectionState::Connected
  /// [Connection Mode]:        ConnectionMode
  /// [PASSIVE]:                ConnectionMode::Passive
  /// [ACTIVE]:                 ConnectionMode::Active
  /// [HSMS Client]:            HsmsClient
  /// [HSMS Connect Procedure]: HsmsClient::connect
  /// [T5]:                     ParameterSettings::t5
  /// [T8]:                     ParameterSettings::t8
  pub fn connect(
    self: &Arc<Self>,
    entity: &str,
  ) -> Result<Receiver<(HsmsMessageID, semi_e5::Message)>, Error> {
    println!("HsmsClient::connect");
    // Connect Primitive Client
    let rx_receiver = self.primitive_client.connect(entity, self.parameter_settings.connect_mode, self.parameter_settings.t5, self.parameter_settings.t8)?;
    // Create Channel
    let (data_sender, data_receiver) = channel::<(HsmsMessageID, semi_e5::Message)>();
    // Start RX Thread
    let clone: Arc<HsmsClient> = self.clone();
    thread::spawn(move || {clone.rx_handle(rx_receiver, data_sender)});
    // Finish
    Ok(data_receiver)
  }

  /// ### HSMS DISCONNECT PROCEDURE
  /// **Based on SEMI E37-1109§6.4-6.5**
  /// 
  /// Disconnects the [HSMS Client] from the Remote Entity.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state to use this
  /// procedure.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Upon completion of the [HSMS Disconnect Procedure], the [NOT CONNECTED]
  /// state is entered.
  /// 
  /// [Connection State]:          ConnectionState
  /// [NOT CONNECTED]:             ConnectionState::NotConnected
  /// [CONNECTED]:                 ConnectionState::Connected
  /// [HSMS Client]:               HsmsClient
  /// [HSMS Disconnect Procedure]: HsmsClient::disconnect
  pub fn disconnect(
    self: &Arc<Self>,
  ) -> ConnectionStateTransition {
    println!("HsmsClient::disconnect");
    // Disconnect Primitive Client
    let result = self.primitive_client.disconnect();
    // Clear Outbox
    for (_, (_, sender)) in self.outbox.lock().unwrap().deref_mut().drain() {
      let _ = sender.send(None);
    }
    // Move to Not Selected State
    let _guard = self.selection_mutex.lock().unwrap();
    *self.selection_state.write().unwrap().deref_mut() = SelectionState::NotSelected;
    // Finish
    result
  }
}

/// ## HSMS CLIENT: MESSAGE EXCHANGE PROCEDURES
/// **Based on SEMI E37-1109§7**
/// 
/// Encapsulates the parts of the [HSMS Client]'s functionality dealing with
/// exchanging [HSMS Message]s.
/// 
/// - [HSMS Data Procedure] - [Data Message]s
/// - [HSMS Select Procedure] - [Select.req] and [Select.rsp]
/// - [HSMS Deselect Procedure] - [Deselect.req] and [Deselect.rsp]
/// - [HSMS Linktest Procedure] - [Linktest.req] and [Linktest.rsp]
/// - [HSMS Separate Procedure] - [Separate.req]
/// - [HSMS Reject Procedure] - [Reject.req]
/// 
/// [HSMS]:                    crate
/// [HSMS Message]:            HsmsMessage
/// [HSMS Client]:             HsmsClient
/// [HSMS Select Procedure]:   HsmsClient::select
/// [HSMS Data Procedure]:     HsmsClient::data
/// [HSMS Deselect Procedure]: HsmsClient::deselect
/// [HSMS Linktest Procedure]: HsmsClient::linktest
/// [HSMS Separate Procedure]: HsmsClient::separate
/// [HSMS Reject Procedure]:   HsmsClient::reject
/// [Data Message]:            HsmsMessageContents::DataMessage
/// [Select.req]:              HsmsMessageContents::SelectRequest
/// [Select.rsp]:              HsmsMessageContents::SelectResponse
/// [Deselect.req]:            HsmsMessageContents::DeselectRequest
/// [Deselect.rsp]:            HsmsMessageContents::DeselectResponse
/// [Linktest.req]:            HsmsMessageContents::LinktestRequest
/// [Linktest.rsp]:            HsmsMessageContents::LinktestResponse
/// [Reject.req]:              HsmsMessageContents::RejectRequest
/// [Separate.req]:            HsmsMessageContents::SeparateRequest
impl HsmsClient {
  /// ### RECEPTION HANDLER
  /// 
  /// An [HSMS Client] in the [CONNECTED] state will automatically [Receive]
  /// [Hsms Message]s and respond based on its [HSMS Message Contents] and the
  /// current [Selection State].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### [Data Message]
  /// 
  /// - [NOT SELECTED] - The [HSMS Client] will respond by transmitting a
  ///   [Reject.req] message, rejecting the [HSMS Data Procedure] and
  ///   completing the [HSMS Reject Procedure].
  /// - [SELECTED], Primary [Data Message] - The [HSMS Client] will send the
  ///   [Data Message] to the Receiver provided by the
  ///   [HSMS Connect Procedure].
  /// - [SELECTED], Response [Data Message] - The [HSMS Client] will respond by
  ///   correllating the message to a previously sent Primary [Data Message],
  ///   finishing a previously initiated [HSMS Data Procedure] if successful,
  ///   or if unsuccessful by transmitting a [Reject.req] message, rejecting
  ///   the [HSMS Data Procedure] and completing the [HSMS Reject Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### [Select.req]:
  /// 
  /// - [NOT SELECTED] - The [HSMS Client] will respond with a [Select.rsp]
  ///   accepting and completing the [HSMS Select Procedure].
  /// - [SELECT INITIATED] - The [HSMS Client] will respond with a [Select.rsp]
  ///   accepting the Remote Entity's [HSMS Select Procedure] but will not
  ///   complete the [HSMS Select Procedure] locally until a [Select.rsp] is
  ///   received.
  /// - [SELECTED] or [DESELECT INITIATED] - The [HSMS Client] will respond with
  ///   a [Select.rsp] message rejecting the [HSMS Select Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### [Select.rsp]:
  /// 
  /// - [SELECT INITIATED], Valid [Select.rsp] - The [HSMS Client] will
  ///   complete the [HSMS Select Procedure].
  /// - [NOT SELECTED], [SELECTED], [DESELECT INITIATED], or Invalid
  ///   [Select.rsp] - The [HSMS Client] will respond with a [Reject.req]
  ///   message, completing the [HSMS Reject Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### [Deselect.req]:
  /// 
  /// - Not yet implemented.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### [Deselect.rsp]:
  /// 
  /// - Not yet implemented.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### [Linktest.req]:
  /// 
  /// - The [HSMS Client] will respond with a [Linktest.rsp], completing the
  ///   [HSMS Linktest Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### [Linktest.rsp]:
  /// 
  /// - The [HSMS Client] will respond by correllating the message to a
  ///   previously sent [Linktest.req] message, finishing a previously initiated
  ///   [HSMS Linktest Procedure] if successful, or if unsuccessful by
  ///   transmitting a [Reject.req] message, completing the
  ///   [HSMS Reject Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### [Reject.req]:
  /// 
  /// - Not yet implemented.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### [Separate.req]:
  /// 
  /// - [NOT SELECTED] - The [HSMS Client] will not do anything.
  /// - [SELECTED] - The [HSMS Client] will complete the
  ///   [HSMS Separate Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### Unknown [Primitive Message]:
  /// 
  /// - The [HSMS Client] will respond by transmitting a [Reject.req] message,
  ///   completing the [HSMS Reject Procedure]. 
  /// 
  /// [Primitive Message]:       PrimitiveMessage
  /// [Connection State]:        ConnectionState
  /// [NOT CONNECTED]:           ConnectionState::NotConnected
  /// [CONNECTED]:               ConnectionState::Connected
  /// [HSMS Message]:            HsmsMessage
  /// [HSMS Message Contents]:   HsmsMessageContents
  /// [Data Message]:            HsmsMessageContents::DataMessage
  /// [Select.req]:              HsmsMessageContents::SelectRequest
  /// [Select.rsp]:              HsmsMessageContents::SelectResponse
  /// [Deselect.req]:            HsmsMessageContents::DeselectRequest
  /// [Deselect.rsp]:            HsmsMessageContents::DeselectResponse
  /// [Linktest.req]:            HsmsMessageContents::LinktestRequest
  /// [Linktest.rsp]:            HsmsMessageContents::LinktestResponse
  /// [Reject.req]:              HsmsMessageContents::RejectRequest
  /// [Separate.req]:            HsmsMessageContents::SeparateRequest
  /// [HSMS Client]:             HsmsClient
  /// [HSMS Connect Procedure]:  HsmsClient::connect
  /// [HSMS Select Procedure]:   HsmsClient::select
  /// [HSMS Data Procedure]:     HsmsClient::data
  /// [HSMS Deselect Procedure]: HsmsClient::deselect
  /// [HSMS Linktest Procedure]: HsmsClient::linktest
  /// [HSMS Separate Procedure]: HsmsClient::separate
  /// [HSMS Reject Procedure]:   HsmsClient::reject
  /// [Selection State]:         SelectionState
  /// [NOT SELECTED]:            SelectionState::NotSelected
  /// [SELECTED]:                SelectionState::Selected
  /// [SELECT INITIATED]:        SelectionState::SelectInitiated
  /// [DESELECT INITIATED]:      SelectionState::DeselectInitiated
  fn rx_handle(
    self: &Arc<Self>,
    rx_receiver: Receiver<PrimitiveMessage>,
    rx_sender: Sender<(HsmsMessageID, semi_e5::Message)>,
  ) {
    println!("HsmsClient::rx_handle start");
    for primitive_message in rx_receiver {
      let primitive_header = primitive_message.header;
      match HsmsMessage::try_from(primitive_message) {
        Ok(rx_message) => match rx_message.contents {
          // RX: Data Message
          HsmsMessageContents::DataMessage(data) => {
            match self.selection_state.read().unwrap().deref() {
              // IS: SELECTED
              SelectionState::Selected => {
                // RX: Primary Data Message
                if data.function % 2 == 1 {
                  // INBOX: New Transaction
                  if rx_sender.send((rx_message.id, data)).is_err() {break}
                }
                // RX: Response Data Message
                else {
                  // OUTBOX: Find Transaction
                  let mut outbox = self.outbox.lock().unwrap();
                  let mut optional_transaction: Option<u32> = None;
                  for (outbox_id, (message_id, _)) in outbox.deref() {
                    if *message_id == rx_message.id {
                      optional_transaction = Some(*outbox_id);
                      break;
                    }
                  }
                  // OUTBOX: Transaction Found
                  if let Some(transaction) = optional_transaction {
                    // OUTBOX: Complete Transaction
                    let (_, sender) = outbox.deref_mut().remove(&transaction).unwrap();
                    sender.send(Some(HsmsMessage{
                      id: rx_message.id,
                      contents: HsmsMessageContents::DataMessage(data),
                    })).unwrap();
                  }
                  // OUTBOX: Transaction Not Found
                  else {
                    // TX: Reject.req 
                    if self.primitive_client.transmit(HsmsMessage {
                      id: rx_message.id,
                      contents: HsmsMessageContents::RejectRequest(0, RejectReason::TransactionNotOpen as u8)
                    }.into()).is_err() {break}
                  }
                }
              },
              // IS: NOT SELECTED
              _ => {
                // TX: Reject.req
                if self.primitive_client.transmit(HsmsMessage {
                  id: rx_message.id,
                  contents: HsmsMessageContents::RejectRequest(0, RejectReason::EntityNotSelected as u8)
                }.into()).is_err() {break}
              },
            }
          },
          // RX: Select.req
          HsmsMessageContents::SelectRequest => {
            match self.selection_mutex.try_lock() {
              Ok(_guard) => {
                let selection_state = *self.selection_state.read().unwrap().deref();
                match selection_state {
                  // IS: NOT SELECTED
                  SelectionState::NotSelected => {
                    // TX: Select.rsp Success
                    if self.primitive_client.transmit(HsmsMessage {
                      id: rx_message.id,
                      contents: HsmsMessageContents::SelectResponse(SelectStatus::Success as u8),
                    }.into()).is_err() {break};
                    // TO: SELECTED
                    *self.selection_state.write().unwrap().deref_mut() = SelectionState::Selected;
                  },
                  // IS: SELECTED
                  SelectionState::Selected => {
                    // TX: Select.rsp Already Active
                    if self.primitive_client.transmit(HsmsMessage {
                      id: rx_message.id,
                      contents: HsmsMessageContents::SelectResponse(SelectStatus::AlreadyActive as u8),
                    }.into()).is_err() {break};
                  },
                  // IS: SELECT INITIATED
                  // TODO: Find way to reimplement this under the current scheme.
                  /*SelectionState::SelectInitiated(session_id) => {
                    // RX: Valid Simultaneous Select
                    if rx_message.id.session == *session_id {
                      // TX: Select.rsp Success
                      if self.primitive_client.transmit(HsmsMessage {
                        id: rx_message.id,
                        contents: HsmsMessageContents::SelectResponse(SelectStatus::Success as u8),
                      }.into()).is_err() {break};
                    }
                    // RX: Invalid Simultaneous Select
                    else {
                      // TX: Select.rsp Already Active
                      if self.primitive_client.transmit(HsmsMessage {
                        id: rx_message.id,
                        contents: HsmsMessageContents::SelectResponse(SelectStatus::AlreadyActive as u8),
                      }.into()).is_err() {break};
                    }
                  },*/
                }
              },
              Err(_) => {
                // Todo: probably appropriate to put something here, maybe to do with the simulatenous select procedure?
              },
            }
          },
          // RX: Select.rsp
          HsmsMessageContents::SelectResponse(select_status) => {
            // OUTBOX: Find Transaction
            let mut outbox = self.outbox.lock().unwrap();
            let mut optional_transaction: Option<u32> = None;
            for (outbox_id, (message_id, _)) in outbox.deref() {
              if *message_id == rx_message.id {
                optional_transaction = Some(*outbox_id);
                break;
              }
            }
            // OUTBOX: Transaction Found
            if let Some(transaction) = optional_transaction {
              // OUTBOX: Complete Transaction
              let (_, sender) = outbox.deref_mut().remove(&transaction).unwrap();
              sender.send(Some(HsmsMessage{
                id: rx_message.id,
                contents: HsmsMessageContents::SelectResponse(select_status),
              })).unwrap();
            }
            // OUTBOX: Transaction Not Found
            else {
              // TX: Reject.req
              if self.primitive_client.transmit(HsmsMessage {
                id: rx_message.id,
                contents: HsmsMessageContents::RejectRequest(0, RejectReason::TransactionNotOpen as u8)
              }.into()).is_err() {break}
            }
          },
          // RX: Deselect.req
          HsmsMessageContents::DeselectRequest => {
            todo!()
          },
          // RX: Deselect.rsp
          HsmsMessageContents::DeselectResponse(_deselect_status) => {
            todo!()
          },
          // RX: Linktest.req
          HsmsMessageContents::LinktestRequest => {
            // TX: Linktest.rsp
            if self.primitive_client.transmit(HsmsMessage{
              id: rx_message.id,
              contents: HsmsMessageContents::LinktestResponse,
            }.into()).is_err() {break};
          },
          // RX: Linktest.rsp
          HsmsMessageContents::LinktestResponse => {
            // OUTBOX: Find Transaction
            let mut outbox = self.outbox.lock().unwrap();
            let mut optional_transaction: Option<u32> = None;
            for (outbox_id, (message_id, _)) in outbox.deref() {
              if *message_id == rx_message.id {
                optional_transaction = Some(*outbox_id);
                break;
              }
            }
            // OUTBOX: Transaction Found
            if let Some(transaction) = optional_transaction {
              // OUTBOX: Complete Transaction
              let (_, sender) = outbox.deref_mut().remove(&transaction).unwrap();
              sender.send(Some(rx_message)).unwrap();
            }
            // OUTBOX: Transaction Not Found
            else {
              // TX: Reject.req
              if self.primitive_client.transmit(HsmsMessage {
                id: rx_message.id,
                contents: HsmsMessageContents::RejectRequest(SessionType::LinktestRequest as u8, RejectReason::TransactionNotOpen as u8),
              }.into()).is_err() {break}
            }
          },
          // RX: Reject.req
          HsmsMessageContents::RejectRequest(_message_type, _reason_code) => {
            // OUTBOX: Find Transaction
            let mut outbox = self.outbox.lock().unwrap();
            let mut optional_transaction: Option<u32> = None;
            for (outbox_id, (message_id, _)) in outbox.deref() {
              if *message_id == rx_message.id {
                optional_transaction = Some(*outbox_id);
                break;
              }
            }
            // OUTBOX: Transaction Found
            if let Some(transaction) = optional_transaction {
              // OUTBOX: Reject Transaction
              let (_, sender) = outbox.deref_mut().remove(&transaction).unwrap();
              sender.send(None).unwrap();
            }
          },
          // RX: Separate.req
          HsmsMessageContents::SeparateRequest => {
            let _guard: std::sync::MutexGuard<'_, ()> = self.selection_mutex.lock().unwrap();
            let selection_state = *self.selection_state.read().unwrap().deref();
            if let SelectionState::Selected = selection_state {
              *self.selection_state.write().unwrap().deref_mut() = SelectionState::NotSelected;
            }
          },
        },
        Err(reject_reason) => {
          // TX: Reject.req
          if self.primitive_client.transmit(HsmsMessage {
            id: HsmsMessageID {
              session: primitive_header.session_id,
              system: primitive_header.system,
            },
            contents: HsmsMessageContents::RejectRequest(match reject_reason {
              RejectReason::UnsupportedPresentationType => primitive_header.presentation_type,
              _ => primitive_header.session_type,
            }, reject_reason as u8),
          }.into()).is_err() {break}
        },
      }
    }
    // TO: NOT CONNECTED
    self.disconnect();
    println!("HsmsClient::rx_handle end");
  }

  /// ### TRANSMISSION HANDLER
  fn tx_handle(
    self: &Arc<Self>,
    message: HsmsMessage,
    reply_expected: bool,
    delay: Duration,
  ) -> Result<Option<HsmsMessage>, ConnectionStateTransition> {
    println!("HsmsClient::tx_handle");
    // REPLY: EXPECTED
    if reply_expected {
      'lock: {
        let (receiver, system) = {
          // OUTBOX: Lock
          let mut outbox = self.deref().outbox.lock().unwrap();
          // TX
          let message_id = message.id;
          match self.primitive_client.transmit(message.into()) {
            // TX: Success
            Ok(_) => {
              // OUTBOX: Create Transaction
              let (sender, receiver) = oneshot::channel::<Option<HsmsMessage>>();
              let system = {
                let mut system_guard = self.deref().system.lock().unwrap();
                let system_counter = system_guard.deref_mut();
                let system = *system_counter;
                *system_counter += 1;
                system
              };
              outbox.deref_mut().insert(system, (message_id, sender));
              (receiver, system)
            },
            // TX: Failure
            Err(_) => {
              // TO: NOT CONNECTED
              break 'lock;
            },
          }
        };
        // RX
        let rx_result = receiver.recv_timeout(delay);
        // OUTBOX: Remove Transaction
        let mut outbox = self.outbox.lock().unwrap();
        outbox.deref_mut().remove(&system);
        match rx_result {
          // RX: Success
          Ok(rx_message) => return Ok(rx_message),
          // RX: Failure
          Err(_e) => return Ok(None),
        }
      }
      Err(self.disconnect())
    }
    // REPLY: NOT EXPECTED
    else {
      // TX
      match self.primitive_client.transmit(message.into()) {
        Ok(_) => Ok(None),
        Err(_) => {
          // TX: Failure
          // TO: NOT CONNECTED
          Err(self.disconnect())
        },
      }
    }
  }

  /// ### HSMS DATA PROCEDURE
  /// **Based on SEMI E37-1109§7.5-7.6**
  /// 
  /// Asks the [HSMS Client] to initiate the [HSMS Data Procedure] by
  /// transmitting a Primary [Data Message] and waiting for the corresponding
  /// Response [Data Message] to be received if it is necessary to do so.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state and the
  /// [Selection State] must be in the [SELECTED] state to use this procedure.
  /// 
  /// When a Response [Data Message] is necessary, the [HSMS Client] will wait
  /// to receiver it for the amount of time specified by [T3] before it will
  /// consider it a communications failure and initiate the
  /// [HSMS Disconnect Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Although not done within this function, an [HSMS Client] in the
  /// [CONNECTED] state will automatically respond to having received a
  /// [Data Message] based on its contents and the current [Selection State]:
  /// - [NOT SELECTED] - The [HSMS Client] will respond by transmitting a
  ///   [Reject.req] message, rejecting the [HSMS Data Procedure] and
  ///   completing the [HSMS Reject Procedure].
  /// - [SELECTED], Primary [Data Message] - The [HSMS Client] will send the
  ///   [Data Message] to the receiver provided after the
  ///   [HSMS Connect Procedure] succeeded.
  /// - [SELECTED], Response [Data Message] - The [HSMS Client] will respond by
  ///   correllating the message to a previously sent Primary [Data Message],
  ///   finishing a previously initiated [HSMS Data Procedure] if successful,
  ///   or if unsuccessful by transmitting a [Reject.req] message, rejecting
  ///   the [HSMS Data Procedure] and completing the [HSMS Reject Procedure].
  /// 
  /// [Connection State]:          ConnectionState
  /// [CONNECTED]:                 ConnectionState::Connected
  /// [Selection State]:           SelectionState
  /// [NOT SELECTED]:              SelectionState::NotSelected
  /// [SELECTED]:                  SelectionState::Selected
  /// [T3]:                        ParameterSettings::t3
  /// [HSMS Client]:               HsmsClient
  /// [HSMS Connect Procedure]:    HsmsClient::connect
  /// [HSMS Disconnect Procedure]: HsmsClient::disconnect
  /// [HSMS Data Procedure]:       HsmsClient::data
  /// [HSMS Reject Procedure]:     HsmsClient::reject
  /// [Data Message]:              HsmsMessageContents::DataMessage
  /// [Reject.req]:                HsmsMessageContents::RejectRequest
  pub fn data(
    self: &Arc<Self>,
    id: HsmsMessageID,
    message: semi_e5::Message,
  ) -> JoinHandle<Result<Option<semi_e5::Message>, ConnectionStateTransition>> {
    println!("HsmsClient::data");
    let clone: Arc<HsmsClient> = self.clone();
    let reply_expected = message.function % 2 == 1 && message.w;
    thread::spawn(move || {
      'disconnect: {
        match clone.selection_state.read().unwrap().deref() {
          // IS: SELECTED
          SelectionState::Selected => {
            // TX: Data Message
            match clone.tx_handle(
              HsmsMessage {
                id,
                contents: HsmsMessageContents::DataMessage(message),
              },
              reply_expected,
              clone.parameter_settings.t3,
            )?{
              // RX: Valid
              Some(rx_message) => {
                match rx_message.contents {
                  HsmsMessageContents::DataMessage(data_message) => return Ok(Some(data_message)),
                  _ => return Err(ConnectionStateTransition::None),
                }
              },
              // RX: Invalid
              None => {
                // Reply Expected
                if reply_expected {
                  // TO: NOT CONNECTED
                  break 'disconnect;
                }
                // Reply Not Expected
                else {
                  return Ok(None);
                }
              },
            }
          },
          // IS: NOT SELECTED
          _ => return Err(ConnectionStateTransition::None),
        }
      }
      Err(clone.disconnect())
    })
  }

  /// ### HSMS SELECT PROCEDURE
  /// **Based on SEMI E37-1109§7.3-7.4**
  /// 
  /// Asks the [HSMS Client] to initiate the [HSMS Select Procedure] by
  /// transmitting a [Select.req] message and waiting for the corresponding
  /// [Select.rsp] message to be received.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state and the
  /// [Selection State] must be in the [NOT SELECTED] state to use this
  /// procedure.
  /// 
  /// The [HSMS Client] will wait to receive the [Select.rsp] for the amount
  /// of time specified by [T6] before it will consider it a communications
  /// failure and initiate the [HSMS Disconnect Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Although not done within this function, a [HSMS Client] in the
  /// [CONNECTED] state will automatically respond to having received a
  /// [Select.req] message based on its current [Selection State]:
  /// - [NOT SELECTED] - The [HSMS Client] will respond with a [Select.rsp]
  ///   accepting and completing the [Select Procedure].
  /// - [SELECTED] - The [HSMS Client] will respond with a [Select.rsp]
  ///   rejecting the [HSMS Select Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Upon completion of the [HSMS Select Procedure], the [SELECTED] state
  /// is entered.
  /// 
  /// [Connection State]:          ConnectionState
  /// [CONNECTED]:                 ConnectionState::Connected
  /// [Selection State]:           SelectionState
  /// [NOT SELECTED]:              SelectionState::NotSelected
  /// [SELECTED]:                  SelectionState::Selected
  /// [T6]:                        ParameterSettings::t6
  /// [HSMS Client]:               HsmsClient
  /// [HSMS Disconnect Procedure]: HsmsClient::disconnect
  /// [HSMS Select Procedure]:     HsmsClient::select
  /// [Select.req]:                HsmsMessageContents::SelectRequest
  /// [Select.rsp]:                HsmsMessageContents::SelectResponse
  pub fn select(
    self: &Arc<Self>,
    id: HsmsMessageID,
  ) -> JoinHandle<Result<(), ConnectionStateTransition>> {
    println!("HsmsClient::select");
    let clone: Arc<HsmsClient> = self.clone();
    thread::spawn(move || {
      let _guard = clone.selection_mutex.lock();
      let selection_state = *clone.selection_state.read().unwrap().deref();
      match selection_state {
        SelectionState::NotSelected => {
          // TX: Select.req
          match clone.tx_handle(
            HsmsMessage {
              id,
              contents: HsmsMessageContents::SelectRequest,
            },
            true,
            clone.parameter_settings.t6,
          )?{
            // RX: Valid
            Some(rx_message) => {
              match rx_message.contents {
                // RX: Select.rsp
                HsmsMessageContents::SelectResponse(select_status) => {
                  // RX: Select.rsp Success
                  if select_status == SelectStatus::Success as u8 {
                    // TO: SELECTED
                    *clone.selection_state.write().unwrap() = SelectionState::Selected;
                    Ok(())
                  }
                  // RX: Select.rsp Failure
                  else {
                    *clone.selection_state.write().unwrap() = SelectionState::NotSelected;
                    Err(ConnectionStateTransition::None)
                  }
                },
                // RX: Unknown
                _ => {
                  *clone.selection_state.write().unwrap() = SelectionState::NotSelected;
                  Err(ConnectionStateTransition::None)
                },
              }
            },
            // RX: Invalid
            None => {
              // TO: NOT CONNECTED
              *clone.selection_state.write().unwrap() = SelectionState::NotSelected;
              Err(clone.disconnect())
            },
          }
        },
        SelectionState::Selected => {
          Err(ConnectionStateTransition::None)
        },
      }
    })
  }

  /// ### HSMS DESELECT PROCEDURE (TODO)
  /// **Based on SEMI E37-1109§7.7**
  /// 
  /// Asks the [HSMS Client] to initiate the [Deselect Procedure] by
  /// transmitting a [Deselect.req] message and waiting for the corresponding
  /// [Deselect.rsp] message to be received.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state and the
  /// [Selection State] must be in the [SELECTED] state to use this procedure.
  /// 
  /// The [HSMS Client] will wait to receive the [Deselect.rsp] for the
  /// amount of time specified by [T6] before it will consider it a
  /// communications failure and initiate the [HSMS Disconnect Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Although not done within this function, an [HSMS Client] in the
  /// [CONNECTED] state will automatically respond to having received a
  /// [Deselect.req] message based on its current [Selection State]:
  /// - [NOT SELECTED] - The [HSMS Client] will respond with a [Deselect.rsp]
  ///   rejecting the [HSMS Deselect Procedure].
  /// - [SELECTED] - The [HSMS Client] will respond with a [Deselect.rsp]
  ///   accepting and completing the [HSMS Deselect Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Upon completion of the [HSMS Deselect Procedure], the [NOT SELECTED]
  /// state is entered.
  /// 
  /// [Connection State]:          ConnectionState
  /// [CONNECTED]:                 ConnectionState::Connected
  /// [Selection State]:           SelectionState
  /// [NOT SELECTED]:              SelectionState::NotSelected
  /// [SELECTED]:                  SelectionState::Selected
  /// [T6]:                        ParameterSettings::t6
  /// [HSMS Client]:               HsmsClient
  /// [HSMS Disconnect Procedure]: HsmsClient::disconnect
  /// [HSMS Deselect Procedure]:   HsmsClient::deselect
  /// [Deselect.req]:              HsmsMessageContents::DeselectRequest
  /// [Deselect.rsp]:              HsmsMessageContents::DeselectResponse
  pub fn deselect(
    self: &Arc<Self>,
  ) -> Result<(), ConnectionStateTransition> {
    println!("HsmsClient::deselect");
    todo!()
  }

  /// ### HSMS LINKTEST PROCEDURE
  /// **Based on SEMI E37-1109§7.8**
  /// 
  /// Asks the [HSMS Client] to initiate the [HSMS Linktest Procedure] by
  /// transmitting a [Linktest.req] message and waiting for the corresponding
  /// [Linktest.rsp] message to be received.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state to use this
  /// procedure.
  /// 
  /// The [HSMS Client] will wait to receive the [Linktest.rsp] for the amount
  /// of time specified by [T6] before it will consider it a communications
  /// failure and initiate the [HSMS Disconnect Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Although not done within this function, a [HSMS Client] in the
  /// [CONNECTED] state will automatically respond to having received a
  /// [Linktest.req] message:
  /// - The [HSMS Client] will respond with a [Linktest.rsp], completing the
  ///   [HSMS Linktest Procedure].
  /// 
  /// [Connection State]:          ConnectionState
  /// [CONNECTED]:                 ConnectionState::Connected
  /// [Selection State]:           SelectionState
  /// [NOT SELECTED]:              SelectionState::NotSelected
  /// [SELECTED]:                  SelectionState::Selected
  /// [T6]:                        ParameterSettings::t6
  /// [HSMS Client]:               HsmsClient
  /// [HSMS Disconnect Procedure]: HsmsClient::disconnect
  /// [HSMS Linktest Procedure]:   HsmsClient::linktest
  /// [Linktest.req]:              HsmsMessageContents::LinktestRequest
  /// [Linktest.rsp]:              HsmsMessageContents::LinktestResponse
  pub fn linktest(
    self: &Arc<Self>,
    system: u32,
  ) -> JoinHandle<Result<(), ConnectionStateTransition>> {
    println!("HsmsClient::linktest");
    let clone: Arc<HsmsClient> = self.clone();
    thread::spawn(move || {
      // TX: Linktext.req
      match clone.tx_handle(
        HsmsMessage {
          id: HsmsMessageID {
            session: 0xFFFF,
            system,
          },
          contents: HsmsMessageContents::LinktestRequest,
        },
        true,
        clone.parameter_settings.t6,
      )?{
        // RX: Valid
        Some(rx_message) => {
          match rx_message.contents {
            HsmsMessageContents::LinktestResponse => Ok(()),
            _ => Err(ConnectionStateTransition::None),
          }
        },
        // RX: Invalid
        None => {
          // TO: NOT CONNECTED
          Err(clone.disconnect())
        },
      }
    })
  }

  /// ### HSMS SEPARATE PROCEDURE
  /// **Based on SEMI E37-1109§7.9**
  /// 
  /// Asks the [HSMS Client] to initiate the [HSMS Separate Procedure] by
  /// transmitting a [Separate.req] message.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state and the
  /// [Selection State] must be in the [SELECTED] state to use this procedure.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Although not done within this function, an [HSMS Client] in the
  /// [CONNECTED] state will automatically respond to having received a
  /// [Separate.req] message based on its current [Selection State]:
  /// - [NOT SELECTED] - The [HSMS Client] will not do anything.
  /// - [SELECTED] - The [HSMS Client] will complete the
  ///   [HSMS Separate Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Upon completion of the [HSMS Separate Procedure], the [NOT SELECTED]
  /// state is entered.
  /// 
  /// [Connection State]:        ConnectionState
  /// [CONNECTED]:               ConnectionState::Connected
  /// [Selection State]:         SelectionState
  /// [NOT SELECTED]:            SelectionState::NotSelected
  /// [SELECTED]:                SelectionState::Selected
  /// [HSMS Client]:             HsmsClient
  /// [HSMS Separate Procedure]: HsmsClient::separate
  /// [Separate.req]:            HsmsMessageContents::SeparateRequest
  pub fn separate(
    self: &Arc<Self>,
    id: HsmsMessageID,
  ) -> JoinHandle<Result<(), ConnectionStateTransition>> {
    println!("HsmsClient::separate");
    let clone: Arc<HsmsClient> = self.clone();
    thread::spawn(move || {
      let _guard = clone.selection_mutex.lock().unwrap();
      let selection_state = *clone.selection_state.read().unwrap().deref();
      match selection_state {
        SelectionState::NotSelected => {
          Err(ConnectionStateTransition::None)
        },
        SelectionState::Selected => {
          // TX: Separate.req
          clone.tx_handle(
            HsmsMessage {
              id,
              contents: HsmsMessageContents::SeparateRequest,
            },
            false,
            clone.parameter_settings.t6,
          )?;
          *clone.selection_state.write().unwrap().deref_mut() = SelectionState::NotSelected;
          Ok(())
        },
      }
    })
  }

  /// ### HSMS REJECT PROCEDURE (TODO)
  /// **Based on SEMI E37-1109§7.10**
  /// 
  /// Asks the [HSMS Client] to initiate the [HSMS Reject Procedure] by
  /// transmitting a [Reject.req] message.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state to use this
  /// procedure.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Although not done within this function, an [HSMS Client] in the
  /// [CONNECTED] state will automatically respond to having received a
  /// [Reject.req]:
  /// 
  /// - Not yet implemented.
  /// 
  /// [Connection State]:      ConnectionState
  /// [CONNECTED]:             ConnectionState::Connected
  /// [Selection State]:       SelectionState
  /// [NOT SELECTED]:          SelectionState::NotSelected
  /// [SELECTED]:              SelectionState::Selected
  /// [HSMS Client]:           HsmsClient
  /// [HSMS Reject Procedure]: HsmsClient::reject
  /// [Reject.req]:            HsmsMessageContents::RejectRequest
  pub fn reject(
    self: &Arc<Self>,
    _header: PrimitiveMessageHeader,
    _reason: RejectReason,
  ) -> Result<(), ConnectionStateTransition> {
    println!("HsmsClient::reject");
    todo!()
  }
}

/// ## SELECTION STATE
/// **Based on SEMI E37-1109§5.5.2**
/// 
/// The [CONNECTED] state has two documented substates, [NOT SELECTED] and
/// [SELECTED], and two undocumented substates [SELECT INITIATED] and
/// [DESELECT INITIATED].
/// 
/// The [HSMS Client] moves between them based on whether it has established
/// an HSMS session with another entity according to the [Select Procedure],
/// [Deselect Procedure], and [Separate Procedure].
/// 
/// [NOT SELECTED]:       SelectionState::NotSelected
/// [SELECTED]:           SelectionState::Selected
/// [SELECT INITIATED]:   SelectionState::SelectInitiated
/// [DESELECT INITIATED]: SelectionState::DeselectInitiated
/// [CONNECTED]:          ConnectionState::Connected
/// [HSMS Client]:        HsmsClient
/// [Select Procedure]:   HsmsClient::select
/// [Deselect Procedure]: HsmsClient::deselect
/// [Separate Procedure]: HsmsClient::separate
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SelectionState {
  /// ### NOT SELECTED
  /// **Based on SEMI E37-1109§5.5.2.1**
  /// 
  /// In this state, the [HSMS Client] is ready to initiate the
  /// [Select Procedure] but has either not yet done so, or has terminated
  /// a previous session.
  /// 
  /// [HSMS Client]:      HsmsClient
  /// [Select Procedure]: HsmsClient::select
  NotSelected,

  /// ### SELECTED
  /// **Based on SEMI E37-1109§5.5.2.2**
  /// 
  /// In this state, the [HSMS Client] has successfully initiated the
  /// [Select Procedure] and is able to send and receive [Data Message]s.
  /// 
  /// [HSMS Client]:      HsmsClient
  /// [Select Procedure]: HsmsClient::select
  /// [Data Message]:     HsmsMessageContents::DataMessage
  Selected,
}
impl Default for SelectionState {
  /// ### DEFAULT SELECTION STATE
  /// **Based on SEMI E37-1109§5.4**
  /// 
  /// Provides the [NOT SELECTED] state by default.
  /// 
  /// [NOT SELECTED]: SelectionState::NotSelected
  fn default() -> Self {
    SelectionState::NotSelected
  }
}

/// ## PARAMETER SETTINGS
/// **Based on SEMI E37-1109§10.2**
/// 
/// The required set of paramters which an [HSMS] implementation must provide,
/// and which the [HSMS Client] will abide by.
/// 
/// [HSMS]:        crate
/// [HSMS Client]: HsmsClient
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParameterSettings {
  /// ### CONNECT MODE
  /// 
  /// Specifies the [Connection Mode] the [HSMS Client] will provide to
  /// the [Primitive Client] to use when performing the [Connect Procedure],
  /// [PASSIVE] to wait for an incoming connection, or [ACTIVE] to initiate
  /// an outgoing connection.
  /// 
  /// [Connection Mode]:   ConnectionMode
  /// [PASSIVE]:           ConnectionMode::Passive
  /// [ACTIVE]:            ConnectionMode::Active
  /// [Primitive Client]:  PrimitiveClient
  /// [HSMS Client]:       HsmsClient
  /// [Connect Procedure]: HsmsClient::connect
  pub connect_mode: ConnectionMode,

  /// ### T3: REPLY TIMEOUT
  /// 
  /// The maximum amount of time that the [HSMS Client] will wait after sending
  /// a Primary [Data Message] to receive the appropriate Response
  /// [Data Message] before it must initiate the [Disconnect Procedure].
  /// 
  /// [Data Message]:         HsmsMessageContents::DataMessage
  /// [HSMS Client]:          HsmsClient
  /// [Disconnect Procedure]: HsmsClient::disconnect
  pub t3: Duration,

  /// ### T5: CONNECTION SEPARATION TIMEOUT
  /// 
  /// The minimum amount of time that the [HSMS Client] must wait between
  /// successive attempts to initiate the [Connect Procedure] with a
  /// [Connect Mode] of [ACTIVE].
  /// 
  /// [ACTIVE]:            ConnectionMode::Active
  /// [Connect Mode]:      ParameterSettings::connect_mode
  /// [HSMS Client]:       HsmsClient
  /// [Connect Procedure]: HsmsClient::connect
  pub t5: Duration,

  /// ### T6: CONTROL TRANSACTION TIMEOUT
  /// 
  /// The maximum amount of time that the [HSMS Client] will wait after sending
  /// a [Select Request], [Deselect Request], or [Linktest Request] to receive
  /// the appropriate [Select Response], [Deselect Response], or
  /// [Linktest Response] before it must initiate the [Disconnect Procedure].
  /// 
  /// [Select Request]:       HsmsMessageContents::SelectRequest
  /// [Select Response]:      HsmsMessageContents::SelectResponse
  /// [Deselect Request]:     HsmsMessageContents::DeselectRequest
  /// [Deselect Response]:    HsmsMessageContents::DeselectResponse
  /// [Linktest Request]:     HsmsMessageContents::LinktestRequest
  /// [Linktest Response]:    HsmsMessageContents::LinktestResponse
  /// [HSMS Client]:          HsmsClient
  /// [Disconnect Procedure]: HsmsClient::disconnect
  pub t6: Duration,

  /// ### T7: NOT SELECTED TIMEOUT
  /// 
  /// The maximum amount of time that the [HSMS Client] will wait after being
  /// placed in the [NOT SELECTED] state before it must initiate the
  /// [Disconnect Procedure].
  /// 
  /// [NOT SELECTED]:         SelectionState::NotSelected
  /// [HSMS Client]:          HsmsClient
  /// [Disconnect Procedure]: HsmsClient::disconnect
  pub t7: Duration,

  /// ### T8: NETWORK INTERCHARACTER TIMEOUT
  /// 
  /// The amount of time that the [HSMS Client] will provide to the
  /// [Primitive Client] to use as the maximum amount of time it may wait while
  /// sending or receiving data between successive characters in the same
  /// [Primitive Message] before it must initiate the [Disconnect Procedure].
  /// 
  /// [Primitive Message]:    PrimitiveMessage
  /// [Primitive Client]:     PrimitiveClient
  /// [Disconnect Procedure]: PrimitiveClient::disconnect
  /// [HSMS Client]:          HsmsClient
  pub t8: Duration,
}
impl Default for ParameterSettings {
  /// ### DEFAULT PARAMETER SETTINGS
  /// **Based on SEMI E37-1109§10.2**
  /// 
  /// Provides [Parameter Settings] with these values, with timeouts as shown
  /// in the 'typical values' column in Table 10.
  /// 
  /// - [Connect Mode] of [PASSIVE]
  /// - [T3] of 45 seconds
  /// - [T5] of 10 seconds
  /// - [T6] of 5 seconds
  /// - [T7] of 10 seconds
  /// - [T8] of 5 seconds
  /// 
  /// [Connection Mode]: ConnectionMode
  /// [PASSIVE]: ConnectionMode::Passive
  /// [ACTIVE]: ConnectionMode::Active
  /// [Parameter Settings]: ParameterSettings
  /// [Connect Mode]: ParameterSettings::connect_mode
  /// [T3]: ParameterSettings::t3
  /// [T5]: ParameterSettings::t5
  /// [T6]: ParameterSettings::t6
  /// [T7]: ParameterSettings::t7
  /// [T8]: ParameterSettings::t8
  fn default() -> Self {
    Self {
      connect_mode: ConnectionMode::default(),
      t3: Duration::from_secs(45),
      t5: Duration::from_secs(10),
      t6: Duration::from_secs(5),
      t7: Duration::from_secs(10),
      t8: Duration::from_secs(5),
    }
  }
}

// OTHER

/// ## CONNECTION STATE TRANSITION
/// **Based on SEMI E37-1109§5.6**
/// 
/// State transitions, used to indicate the nature of an unexpected state
/// transition when using HSMS, for error handling purposes.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConnectionStateTransition {
  /// ### NONE
  /// **Based on SEMI E37-1109§5.6**
  /// 
  /// Used to indicate that either the [Primitive Client] or [HSMS Client] has
  /// failed to perform a certain procedure, but that this failure has not
  /// changed its [Connection State] or [Selection State].
  /// 
  /// [Primitive Client]: PrimitiveClient
  /// [HSMS Client]:      HsmsClient
  /// [Connection State]: ConnectionState
  /// [Selection State]:  SelectionState
  None = 0,

  /// ### NOT CONNECTED
  /// **Based on SEMI E37-1109§5.6**
  /// 
  /// Used to indicate that the [Primitive Client] has failed to perform a
  /// certain procedure because the [NOT CONNECTED] state is currently active.
  /// 
  /// [Primitive Client]: PrimitiveClient
  /// [NOT CONNECTED]:    ConnectionState::NotConnected
  NotConnected = 1,

  /// ### NOT CONNECTED TO NOT SELECTED
  /// **Based on SEMI E37-1109§5.6**
  /// 
  /// TCP/IP connection has been established.
  NotConnectedToNotSelected = 2,

  /// ### CONNECTED TO NOT CONNECTED
  /// **Based on SEMI E37-1109§5.6**
  /// 
  /// Used to indicate that the [Primitive Client] has undergone the
  /// [Disconnect Procedure] at an unexpected time, moving it from the
  /// [CONNECTED] to the [NOT CONNECTED] state.
  /// 
  /// [Primitive Client]:     PrimitiveClient
  /// [Disconnect Procedure]: PrimitiveClient::disconnect
  /// [NOT CONNECTED]:        ConnectionState::NotConnected
  /// [CONNECTED]:            ConnectionState::Connected
  ConnectedToNotConnected = 3,

  /// ### NOT SELECTED TO SELECTED
  /// **Based on SEMI E37-1109§5.6**
  /// 
  /// Used to indicate that the [HSMS Client] has responded to the
  /// [Select Procedure] being initated by the other entity, moving it from the
  /// [NOT SELECTED] to the [SELECTED] state.
  /// 
  /// [HSMS Client]:      HsmsClient
  /// [Select Procedure]: HsmsClient::select
  /// [NOT SELECTED]:     SelectionState::NotSelected
  /// [SELECTED]:         SelectionState::Selected
  NotSelectedToSelected = 4,

  /// ### SELECTED TO NOT SELECTED
  /// **Based on SEMI E37-1109§5.6**
  /// 
  /// Used to indicate that the [HSMS Client] has responded to the
  /// [Deselect Procedure] or the [Separate Procedure] being initiated by the
  /// other entity, moving it from the [SELECTED] to the [NOT SELECTED] state.
  /// 
  /// [HSMS Client]:        HsmsClient
  /// [Deselect Procedure]: HsmsClient::deselect
  /// [Separate Procedure]: HsmsClient::separate
  /// [NOT SELECTED]:       SelectionState::NotSelected
  /// [SELECTED]:           SelectionState::Selected
  SelectedToNotSelected = 5,

  /// ### NOT SELECTED TO NOT CONNECTED
  /// **Based on SEMI E37-1109§5.6**
  /// 
  /// Used to indicate that the [HSMS Client] remained in the [NOT SELECTED]
  /// state for longer than the amount of time specified by [T7], moving it
  /// from the [CONNECTED] state to the [NOT CONNECTED] state.
  /// 
  /// [HSMS Client]:   HsmsClient
  /// [NOT CONNECTED]: ConnectionState::NotConnected
  /// [CONNECTED]:     ConnectionState::Connected
  /// [NOT SELECTED]:  SelectionState::NotSelected
  /// [T7]:            ParameterSettings::t7
  NotSelectedToNotConnected = 6,
}

/// ## PRESENTATION TYPE
/// **Based on SEMI E37-1109§8.2.6.4**
/// 
/// Defines the Presentation Layer content of [Primitive Message Text].
/// 
/// Values 1-127 are reserved for Subsidiary Standards.
/// 
/// Values 128-255 are reserved and may not be used.
/// 
/// [Primitive Message Text]: PrimitiveMessage::text
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PresentationType {
  /// ### SECS II ENCODING
  /// 
  /// Denotes an [HSMS Message], which is often a SECS-II formatted
  /// [Data Message].
  /// 
  /// [HSMS Message]: HsmsMessage
  /// [Data Message]: HsmsMessageContents::DataMessage
  SecsII = 0,
}

/// ## SESSION TYPE
/// **Based on SEMI E37-1109§8.2.6.5-8.2.6.6**
/// 
/// Defines the type of [HSMS Message] being sent.
/// 
/// Values 11-127 are reserved for Subsidiary Standards.
/// 
/// Values 8, 10, and 128-255 are reserved and may not be used.
/// 
/// [HSMS Message]: HsmsMessage
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SessionType {
  /// ### DATA MESSAGE
  /// 
  /// Denotes a SECS-II formatted [Data Message].
  /// 
  /// [Data Message]: HsmsMessageContents::DataMessage 
  DataMessage = 0,

  /// ### SELECT REQUEST
  /// 
  /// Denotes a [Select.req] message.
  /// 
  /// [Select.req]: HsmsMessageContents::SelectRequest
  SelectRequest = 1,

  /// ### SELECT RESPONSE
  /// 
  /// Denotes a [Select.rsp] message.
  /// 
  /// [Select.rsp]: HsmsMessageContents::SelectResponse
  SelectResponse = 2,

  /// ### DESELECT REQUEST
  /// 
  /// Denotes a [Deselect.req] message.
  /// 
  /// [Deselect.req]: HsmsMessageContents::DeselectRequest
  DeselectRequest = 3,

  /// ### DESELECT RESPONSE
  /// 
  /// Denotes a [Deselect.rsp] message.
  /// 
  /// [Deselect.rsp]: HsmsMessageContents::DeselectResponse
  DeselectResponse = 4,

  /// ### LINKTEST REQUEST
  /// 
  /// Denotes a [Linktest.req] message.
  /// 
  /// [Linktest.req]: HsmsMessageContents::LinktestRequest
  LinktestRequest = 5,

  /// ### LINKTEST RESPONSE
  /// 
  /// Denotes a [Linktest.rsp] message.
  /// 
  /// [Linktest.rsp]: HsmsMessageContents::LinktestResponse
  LinktestResponse = 6,

  /// ### REJECT REQUEST
  /// 
  /// Denotes a [Reject.req] message.
  /// 
  /// [Reject.req]: HsmsMessageContents::RejectRequest
  RejectRequest = 7,

  /// ### SEPARATE REQUEST
  /// 
  /// Denotes a [Separate.req] message.
  /// 
  /// [Separate.req]: HsmsMessageContents::SeparateRequest
  SeparateRequest = 9,
}

/// ## SELECT STATUS
/// **Based on SEMI E37-1109§8.3.7.2**
/// 
/// [Byte 3] of a [Deselect.rsp] message, used as the indication of success or
/// reason for failure of the [HSMS Select Procedure].
/// 
/// Values 4-127 are reserved for Subsidiary Standards.
/// 
/// Values 128-255 are reserved for the Local Entity.
/// 
/// [Byte 3]:                PrimitiveMessageHeader::byte_3
/// [Deselect.rsp]:          HsmsMessageContents::DeselectResponse
/// [HSMS Select Procedure]: HsmsClient::select
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SelectStatus {
  Success       = 0,
  AlreadyActive = 1,
  NotReady      = 2,
  Exhausted     = 3,
}

/// ## DESELECT STATUS
/// **Based on SEMI E37-1109§8.3.13.2**
/// 
/// [Byte 3] of a [Deselect.rsp] message, used as the indication of success or
/// reason for failure of the [HSMS Deselect Procedure].
/// 
/// Values 3-127 are reserved for Subsidiary Standards.
/// 
/// Values 128-255 are reserved for the Local Entity.
/// 
/// [Byte 3]:                  PrimitiveMessageHeader::byte_3
/// [Deselect.rsp]:            HsmsMessageContents::DeselectResponse
/// [HSMS Deselect Procedure]: HsmsClient::deselect
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeselectStatus {
  Success        = 0,
  NotEstablished = 1,
  Busy           = 2,
}

/// ## REJECT REASON
/// **Based on SEMI E37-1109§8.3.21.3**
/// 
/// [Byte 3] of a [Reject.req] message, specifying the reason a message has
/// been rejected in the [HSMS Reject Procedure].
/// 
/// Values 4-127 are reserved for Subsidiary Standards.
/// 
/// Values 0, and 128-255 are reserved for the Local Entity.
/// 
/// [Byte 3]:                PrimitiveMessageHeader::byte_3
/// [Reject.req]:            HsmsMessageContents::RejectRequest
/// [HSMS Reject Procedure]: HsmsClient::reject
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RejectReason {
  /// ### MALFORMED DATA
  /// **Local Entity Specific Reason**
  /// 
  /// A message was recieved which was mostly valid but did not follow the
  /// specified restrictions outlined by the HSMS standard.
  MalformedData = 0,

  /// ### SESSION TYPE NOT SUPPORTED
  /// 
  /// A message was received whose Session Type value is not allowed.
  UnsupportedSessionType = 1,

  /// ### PRESENTATION TYPE NOT SUPPORTED
  /// 
  /// A message was received whose Presentation Type value is not allowed.
  UnsupportedPresentationType = 2,

  /// ### TRANSACTION NOT OPEN
  /// 
  /// A Response control message was recieved when there was no outstanding
  /// Request Message which corresponded to it.
  TransactionNotOpen = 3,

  /// ### ENTITY NOT SELECTED
  /// 
  /// A Data Message was recieved when not in the SELECTED state.
  EntityNotSelected = 4,
}
