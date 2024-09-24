//! # PRIMITIVE SERVICES
//! 
//! Defines the most agnostic form in which data can be exchanged persuant to
//! the [HSMS] protocol and any subsidary protocols. This involves managing the
//! creation and breaking of the TCP/IP connection, and the sending of messages
//! with properly formatted headers. This is not necessarily outlined by the
//! standard, but is an important piece of establishing and maintaining proper
//! communications.
//! 
//! ---------------------------------------------------------------------------
//! 
//! To use the [Primitive Services]:
//! 
//! - Build [Message]s which use [Message Header]s.
//! - Create a [Client] with the [New Client] function.
//! - Manage the [Connection State] with the [Connect Procedure] and
//!   [Disconnect Procedure].
//! - Receive [Message]s with the hook provided by the [Connect Procedure].
//! - Transmit [Message]s with the [Transmit Procedure].
//! 
//! [HSMS]:                 crate
//! [Primitive Services]:   crate::primitive
//! [Client]:               Client
//! [New Client]:           Client::new
//! [Connect Procedure]:    Client::connect
//! [Disconnect Procedure]: Client::disconnect
//! [Transmit Procedure]:   Client::transmit
//! [Message]:              Message
//! [Message Header]:       MessageHeader
//! [Connection State]:     ConnectionState

use std::{
  io::{
    Error,
    ErrorKind,
    Read,
    Write,
  },
  net::{
    Shutdown,
    SocketAddr,
    TcpListener,
    TcpStream,
    ToSocketAddrs,
  },
  ops::{
    Deref,
    DerefMut,
  },
  sync::{
    Arc,
    mpsc::{
      channel,
      Receiver,
      Sender,
    },
    RwLock,
  },
  thread,
  time::Duration,
};

/// ## CLIENT
/// 
/// Encapsulates a limited set of functionality of the [HSMS] protocol referred
/// to as the [Primitive Services].
/// 
/// This [Client] can be used to:
/// - Manage the [Connection State] with the [Connect Procedure] and
///   [Disconnect Procedure].
/// - Receive [Message]s with the hook provided by the [Connect Procedure].
/// - Transmit [Message]s with the [Transmit Procedure].
/// 
/// [HSMS]:                 crate
/// [Primitive Services]:   crate::primitive
/// [Message]:              Message
/// [Client]:               Client
/// [Connect Procedure]:    Client::connect
/// [Disconnect Procedure]: Client::disconnect
/// [Transmit Procedure]:   Client::transmit
/// [Connection State]:     ConnectionState
pub struct Client {
  connection_state: RwLock<ConnectionState>,
}

/// ## CONNECTION PROCEDURES
/// **Based on SEMI E37-1109§6.3-6.5**
/// 
/// Encapsulates the parts of the [Client]'s functionality dealing with
/// establishing and breaking a TCP/IP connection.
/// 
/// - [New Client]
/// - [Connect Procedure]
/// - [Disconnect Procedure]
/// 
/// [Client]:               Client
/// [New Client]:           Client::new
/// [Connect Procedure]:    Client::connect
/// [Disconnect Procedure]: Client::disconnect
impl Client {
  /// ### NEW CLIENT
  /// 
  /// Creates a [Client] in the [NOT CONNECTED] state, ready to initiate the
  /// [Connect Procedure].
  /// 
  /// [Client]:            Client
  /// [Connect Procedure]: Client::connect
  /// [NOT CONNECTED]:     ConnectionState::NotConnected
  pub fn new() -> Arc<Self> {
    Arc::new(Self {
      connection_state: Default::default(),
    })
  }

  /// ### CONNECT PROCEDURE
  /// **Based on SEMI E37-1109§6.3.4-6.3.7**
  /// 
  /// Connects the [Client] to the Remote Entity.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [NOT CONNECTED] state to use this
  /// procedure.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connect Procedure] has two different behaviors based on the
  /// [Connection Mode] provided to it:
  /// - [PASSIVE] - The socket address of the Local Entity must be provided,
  ///   and the [Client] listens for and accepts the [Connect Procedure] when
  ///   initiated by the Remote Entity.
  /// - [ACTIVE] - The socket address of the Remote Entity must be provided,
  ///   and the [Client] initiates the [Connect Procedure] and waits up to the
  ///   time specified by [T5] for the Remote Entity to respond.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Upon completion of the [Connect Procedure], the [T8] parameter is set as
  /// the TCP stream's read and write timeout, and the [CONNECTED] state is
  /// entered.
  /// 
  /// [Client]:            Client
  /// [Connect Procedure]: Client::connect
  /// [Connection State]:  ConnectionState
  /// [NOT CONNECTED]:     ConnectionState::NotConnected
  /// [CONNECTED]:         ConnectionState::Connected
  /// [Connection Mode]:   ConnectionMode
  /// [PASSIVE]:           ConnectionMode::Passive
  /// [ACTIVE]:            ConnectionMode::Active
  /// [T5]:                crate::generic::ParameterSettings::t5
  /// [T8]:                crate::generic::ParameterSettings::t8
  pub fn connect(
    self: &Arc<Self>,
    entity: &str,
    connection_mode: ConnectionMode,
    t5: Duration,
    t8: Duration,
  ) -> Result<(SocketAddr, Receiver<Message>), Error> {
    // TCP: CONNECT
    let (stream, socket) = match self.connection_state.read().unwrap().deref() {
      // IS: NOT CONNECTED
      ConnectionState::NotConnected => {
        match connection_mode {
          // CONNECTION MODE: PASSIVE
          ConnectionMode::Passive => {
            // Create Listener and Wait
            let listener = TcpListener::bind(entity)?;
            listener.accept()?
          },
          // CONNECTION MODE: ACTIVE
          ConnectionMode::Active => {
            // Determine Socket
            let socket = entity.to_socket_addrs()?.next().ok_or(Error::from(ErrorKind::AddrNotAvailable))?;
            // Connect with Timeout
            let stream = TcpStream::connect_timeout(
              &socket, 
              t5,
            )?;
            (stream, socket)
          },
        }
      },
      // IS: CONNECTED
      _ => return Err(Error::from(ErrorKind::AlreadyExists)),
    };
    // Set Read and Write Timeouts to T8
    stream.set_read_timeout(Some(t8))?;
    stream.set_write_timeout(Some(t8))?;
    // TO: CONNECTED
    *self.connection_state.write().unwrap().deref_mut() = ConnectionState::Connected(stream);
    // Create Channels
    let (rx_sender, rx_receiver) = channel::<Message>();
    // Start RX Thread
    let rx_clone: Arc<Client> = self.clone();
    thread::spawn(move || {rx_clone.receive(rx_sender.clone())});
    // Finish
    Ok((socket, rx_receiver))
  }

  /// ### DISCONNECT PROCEDURE
  /// **Based on SEMI E37-1109§6.4-6.5**
  /// 
  /// Disconnects the [Client] from the Remote Entity.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state to use this
  /// procedure.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Upon completion of the [Disconnect Procedure], the [NOT CONNECTED] state
  /// is entered.
  /// 
  /// [Client]:               Client
  /// [Disconnect Procedure]: Client::disconnect
  /// [Connection State]:     ConnectionState
  /// [NOT CONNECTED]:        ConnectionState::NotConnected
  /// [CONNECTED]:            ConnectionState::Connected
  pub fn disconnect(
    self: &Arc<Self>
  ) -> Result<(), Error> {
    match self.connection_state.read().unwrap().deref() {
      // IS: NOT CONNECTED
      ConnectionState::NotConnected => return Err(Error::from(ErrorKind::NotConnected)),
      // IS: CONNECTED
      ConnectionState::Connected(stream) => {
        // TCP: SHUTDOWN
        let _ = stream.shutdown(Shutdown::Both);
      },
    }
    // TO: NOT CONNECTED
    *self.connection_state.write().unwrap().deref_mut() = ConnectionState::NotConnected;
    Ok(())
  }
}

/// ## MESSAGE EXCHANGE PROCEDURES
/// **Based on SEMI E37-1109§7**
/// 
/// Encapsulates the parts of the [Client]'s functionality dealing with
/// exchanging [Message]s.
/// 
/// - [Transmit Procedure] - Any [Message]
/// 
/// [Client]:             Client
/// [Transmit Procedure]: Client::transmit
/// [Message]:            Message
impl Client {
  /// ### RECEIVE PROCEDURE
  /// 
  /// A [Client] in the [CONNECTED] state will automatically receive
  /// [Message]s, and send them to the hook provided by the
  /// [Connect Procedure].
  /// 
  /// [Message]:           Message
  /// [Client]:            Client
  /// [Connect Procedure]: Client::connect
  /// [CONNECTED]:         ConnectionState::Connected
  fn receive(
    self: Arc<Self>,
    rx_sender: Sender<Message>,
  ) {
    while let ConnectionState::Connected(stream_immutable) = self.connection_state.read().unwrap().deref() {
      let res: Result<Option<Message>, Error> = 'rx: {
        let mut stream: &TcpStream = stream_immutable;
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
          break 'rx Err(Error::from(ErrorKind::TimedOut))
        }
        let length: u32 = u32::from_be_bytes(length_buffer);
        if length < 10 {
          break 'rx Err(Error::from(ErrorKind::InvalidData))
        }
        // Header + Data [Bytes 4+]
        let mut message_buffer: Vec<u8> = vec![0; length as usize];
        let message_bytes: usize = match stream.read(&mut message_buffer) {
          Ok(message_bytes) => message_bytes,
          Err(error) => break 'rx Err(error),
        };
        if message_bytes != length as usize {
          break 'rx Err(Error::from(ErrorKind::TimedOut))
        }
        // Diagnostic
        /*println!(
          "rx {: >4X} {: >3}{} {: >3} {: >2X} {: >2X} {: >8X} {:?}",
          u16::from_be_bytes(message_buffer[0..2].try_into().unwrap()),
          &message_buffer[2] & 0b0111_1111,
          if (&message_buffer[2] & 0b1000_0000) > 0 {'W'} else {' '},
          &message_buffer[3],
          &message_buffer[4],
          &message_buffer[5],
          u32::from_be_bytes(message_buffer[6..10].try_into().unwrap()),
          &message_buffer[10..],
        );// */
        // Finish
        match Message::try_from(message_buffer) {
          Ok(message) => Ok(Some(message)),
          Err(_) => break 'rx Err(Error::from(ErrorKind::InvalidData)),
        }
      };
      match res {
        // RX: SUCCESS
        Ok(optional_rx_message) => if let Some(rx_message) = optional_rx_message {
          if rx_sender.send(rx_message).is_err() {break}
        },
        // RX: FAILURE
        Err(_error) => break,
      }
    }
    //let _ = self.disconnect();
  }

  /// ### TRANSMIT PROCEDURE
  /// **Based on SEMI E37-1109§7.2**
  /// 
  /// Serializes a [Message] and transmits it over the TCP/IP connection.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state to use this
  /// procedure.
  /// 
  /// [Message]:          Message
  /// [Connection State]: ConnectionState
  /// [CONNECTED]:        ConnectionState::Connected
  pub fn transmit(
    self: &Arc<Self>,
    message: Message,
  ) -> Result<(), Error> {
    match self.connection_state.read().unwrap().deref() {
      ConnectionState::Connected(stream_immutable) => 'disconnect: {
        let mut stream: &TcpStream = stream_immutable;
        // Header + Data [Bytes 4+]
        let message_buffer: Vec<u8> = (&message).into();
        // Length [Bytes 0-3]
        let length: u32 = message_buffer.len() as u32;
        let length_buffer: [u8; 4] = length.to_be_bytes();
        // Diagnostic
        /*println!(
          "tx {: >4X} {: >3}{} {: >3} {: >2X} {: >2X} {: >8X} {:?}",
          u16::from_be_bytes(message_buffer[0..2].try_into().unwrap()),
          &message_buffer[2] & 0b0111_1111,
          if (&message_buffer[2] & 0b1000_0000) > 0 {'W'} else {' '},
          &message_buffer[3],
          &message_buffer[4],
          &message_buffer[5],
          u32::from_be_bytes(message_buffer[6..10].try_into().unwrap()),
          &message_buffer[10..],
        );// */
        // Write
        if stream.write_all(&length_buffer).is_err() {break 'disconnect};
        if stream.write_all(&message_buffer).is_err() {break 'disconnect};
        // Finish
        return Ok(())
      },
      ConnectionState::NotConnected => return Err(Error::from(ErrorKind::NotConnected)),
    };
    self.disconnect()?;
    Err(Error::from(ErrorKind::ConnectionAborted))
  }
}

/// ## CONNECTION STATE
/// **Based on SEMI E37-1109§5.4-5.5**
/// 
/// In the [HSMS] protocol, two [Connection State]s exist, [NOT CONNECTED],
/// and [CONNECTED].
/// 
/// The [Client] will move between them based on whether it has established a
/// TCP/IP connection to a Remote Entity, and the integrity of that connection.
/// 
/// [HSMS]:             crate
/// [Client]:           Client
/// [Connection State]: ConnectionState
/// [NOT CONNECTED]:    ConnectionState::NotConnected
/// [CONNECTED]:        ConnectionState::Connected
#[derive(Debug)]
pub enum ConnectionState {
  /// ### NOT CONNECTED
  /// **Based on SEMI E37-1109§5.5.1**
  /// 
  /// In this state, the [Client] is ready to initiate the [Connect Procedure]
  /// but has either not yet done so, or has terminated a previous connection.
  /// 
  /// [Client]:            Client
  /// [Connect Procedure]: Client::connect
  NotConnected,

  /// ### CONNECTED
  /// **Based on SEMI E37-1109§5.5.2**
  /// 
  /// In this state, the [Client] has successfully initiated the
  /// [Connect Procedure] and is able to send and receive data.
  /// 
  /// [Client]:            Client
  /// [Connect Procedure]: Client::connect
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
/// The [Client] must use one of two [Connection Mode]s, [PASSIVE] or [ACTIVE],
/// in order to perform the [Connect Procedure] and attain a TCP/IP connection.
/// 
/// [Client]:            Client
/// [Connect Procedure]: Client::connect
/// [Connection Mode]:   ConnectionMode
/// [PASSIVE]:           ConnectionMode::Passive
/// [ACTIVE]:            ConnectionMode::Active
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConnectionMode {
  /// ### PASSIVE
  /// **Based on SEMI E37-1109§6.3.2**
  /// 
  /// In this mode, the [Client] listens for and accepts the
  /// [Connect Procedure] when initiated by the Remote Entity.
  /// 
  /// [Client]:            Client
  /// [Connect Procedure]: Client::connect
  Passive,

  /// ### ACTIVE
  /// **Based on SEMI E37-1109§6.3.2**
  /// 
  /// In this mode, the [Client] initiates the [Connect Procedure] and waits up
  /// to the time specified by [T5] for the Remote Entity to respond.
  /// 
  /// [Client]:            Client
  /// [Connect Procedure]: Client::connect
  /// [T5]:                crate::generic::ParameterSettings::t5
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

/// ## MESSAGE
/// **Based on SEMI E37-1109§8.2**
/// 
/// Data using the [HSMS] defined structure, but not enforcing compliance
/// with the standards for how its fields are filled and what they mean.
/// 
/// Note that the Message Length field defined in the standard is not included,
/// as it is only temporarily used when a message is received or transmitted
/// by the [Client].
/// 
/// [HSMS]:   crate
/// [Client]: Client
#[derive(Clone, Debug)]
pub struct Message {
  /// ### MESSAGE HEADER
  /// 
  /// Information about the [Message] stored with the [Message Header] format.
  /// 
  /// [Message]:        Message
  /// [Message Header]: MessageHeader
  pub header: MessageHeader,

  /// ### MESSAGE TEXT
  /// 
  /// Contains the [Message]'s content, whose layout is defined by its
  /// [Presentation Type] and [Session Type].
  /// 
  /// [Message]:           Message
  /// [Presentation Type]: MessageHeader::presentation_type
  /// [Session Type]:      MessageHeader::session_type
  pub text: Vec<u8>,
}
impl From<&Message> for Vec<u8> {
  /// ### SERIALIZE MESSAGE
  /// 
  /// Converts a [Message] into raw bytes.
  /// 
  /// [Message]: Message
  fn from(val: &Message) -> Self {
    let mut vec: Vec<u8> = vec![];
    let header_bytes: [u8;10] = val.header.into();
    vec.extend(header_bytes.iter());
    vec.extend(&val.text);
    vec
  }
}
impl TryFrom<Vec<u8>> for Message {
  type Error = ();

  /// ### DESERIALIZE MESSAGE
  /// 
  /// Converts raw bytes into a [Message].
  /// 
  /// [Message]: Message
  fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
    if bytes.len() < 10 {return Err(())}
    Ok(Self {
      header: MessageHeader::from(<[u8;10]>::try_from(&bytes[0..10]).map_err(|_| ())?),
      text: bytes[10..].to_vec(),
    })
  }
}

/// ## MESSAGE HEADER
/// **Based on SEMI E37-1109§8.2.5-8.2.6**
/// 
/// A 10 byte field describing the contents of a [Message].
/// 
/// [Message]: Message
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MessageHeader {
  /// ### SESSION ID
  /// **Based on SEMI E37-1109§8.2.6.1**
  /// 
  /// Provides an association between [Message]s across multiple transactions.
  /// 
  /// [Message]: Message
  pub session_id : u16,

  /// ### HEADER BYTE 2
  /// **Based on SEMI E37-1109§8.2.6.2**
  /// 
  /// Contains information specific to the [Presentation Type] and
  /// [Session Type].
  /// 
  /// [Presentation Type]: MessageHeader::presentation_type
  /// [Session Type]:      MessageHeader::session_type
  pub byte_2 : u8,

  /// ### HEADER BYTE 3
  /// **Based on SEMI E37-1109§8.2.6.3**
  /// 
  /// Contains information specific to the [Presentation Type] and
  /// [Session Type].
  /// 
  /// [Presentation Type]: MessageHeader::presentation_type
  /// [Session Type]:      MessageHeader::session_type
  pub byte_3 : u8,

  /// ### PRESENTATION TYPE
  /// **Based on SEMI E37-1109§8.2.6.4**
  /// 
  /// An enumerated value, the [Presentation Type], defining the encoding type
  /// of the [Message Text].
  /// 
  /// [Message Text]:      Message::text
  /// [Presentation Type]: crate::PresentationType
  pub presentation_type : u8,

  /// ### SESSION TYPE
  /// **Based on SEMI E37-1109§8.2.6.5-8.2.6.6**
  /// 
  /// An enumerated value, the [Session Type] defining the specific
  /// interpreation of the [Message].
  /// 
  /// [Session Type]: crate::generic::SessionType
  /// [Message]:      Message
  pub session_type : u8,

  /// ### SYSTEM BYTES
  /// **Based on SEMI E37-1109§8.2.6.7**
  /// 
  /// Provides an association between [Message]s across single transactions.
  /// 
  /// [Message]: Message
  pub system : u32,
}
impl From<MessageHeader> for [u8;10] {
  /// ### SERIALIZE MESSAGE HEADER
  /// 
  /// Converts a [Message Header] into raw bytes.
  /// 
  /// [Message Header]: MessageHeader
  fn from(val: MessageHeader) -> Self {
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
impl From<[u8;10]> for MessageHeader {
  /// ### DESERIALIZE MESSAGE HEADER
  /// 
  /// Converts raw bytes into a [Message Header].
  /// 
  /// [Message Header]: MessageHeader
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
