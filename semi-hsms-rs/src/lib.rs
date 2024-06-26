//! # HIGH-SPEED SECS MESSAGE SERVICES (HSMS)
//! **Based on:**
//! - **[SEMI E37]-1109**
//! - **[SEMI E37].1-0702**
//! 
//! Codebase will be updated to reflect more up-to-date SEMI standards if/when
//! they can be acquired for this purpose.
//! 
//! ---------------------------------------------------------------------------
//! 
//! [HSMS] is a Session Protocol designed to facilitate communications between
//! semiconductor equipment over TCP/IP, particularly for sending data
//! formatted with the SECS-II ([SEMI E5]) Presentation Protocol and understood
//! by the GEM ([SEMI E30]) Application Protocol (together known as SECS/GEM).
//! 
//! ---------------------------------------------------------------------------
//! 
//! ## HSMS Generic Services
//! 
//! [HSMS] defines a set of behaviors without modification by any subsidiary
//! standards.
//! 
//! To use the [HSMS] protocol:
//! - Create a [Generic Client] with the [New Generic Client] function.
//! - Manage the connection with the [Generic Connect Procedure] and
//! [Generic Disconnect Procedure].
//! - Send  [Data Message]s with the [Generic Data Procedure].
//! - Control the connection with the [Generic Select Procedure],
//! [Generic Deselect Procedure], [Generic Separate Procedure],
//! [Generic Linktest Procedure], and [Generic Reject Procedure].
//! 
//! ---------------------------------------------------------------------------
//! 
//! ## HSMS Single Selected-Session Mode
//! 
//! [HSMS-SS] is a subsidiary standard of [HSMS] intended to directly replace
//! both the RS-232 based SECS-I ([SEMI E4]) and the OSI based SECS Message
//! Services (SEMI E13) by simplifying the overall behavior of [HSMS] for
//! point-to-point communications.
//! 
//! ---------------------------------------------------------------------------
//! 
//! ## Extension
//! 
//! To build off of the [HSMS] protocol in a manner not defined by this crate:
//! - [Transmit] and [Receive] [Message]s directly with the provided functions.
//! - Create a [Primitive Client] with the [New Primitive Client] function.
//! - Manage the connection with the [Primitive Connect Procedure] and
//! [Primitive Disconnect Procedure].
//! - Transmit and Receive [HSMS Message]s with the hooks provided by the
//! [Primitive Connect Procedure].
//! 
//! ---------------------------------------------------------------------------
//! 
//! ## TO BE DONE
//! 
//! - [Generic Client] - [Generic Select Procedure]
//! - [Generic Client] - "Simultaneous Select Procedure"
//! - [Generic Client] - [Generic Deselect Procedure]
//! - [Generic Client] - "Simultaneous Deselect Procedure"
//! - [Generic Client] - [Generic Separate Procedure]
//! - [Generic Client] - [Generic Reject Procedure]
//! - [HSMS-SS]
//! 
//! [SEMI E4]:  https://store-us.semi.org/products/e00400-semi-e4-specification-for-semi-equipment-communications-standard-1-message-transfer-secs-i
//! [SEMI E5]:  https://store-us.semi.org/products/e00500-semi-e5-specification-for-semi-equipment-communications-standard-2-message-content-secs-ii
//! [SEMI E30]: https://store-us.semi.org/products/e03000-semi-e30-specification-for-the-generic-model-for-communications-and-control-of-manufacturing-equipment-gem
//! [SEMI E37]: https://store-us.semi.org/products/e03700-semi-e37-high-speed-secs-message-services-hsms-generic-services
//! 
//! [HSMS]:                           crate
//! [HSMS-SS]:                        single_selected_session
//! [Connection State]:               ConnectionState
//! [NOT CONNECTED]:                  ConnectionState::NotConnected
//! [CONNECTED]:                      ConnectionState::Connected
//! [Selection State]:                SelectionState
//! [NOT SELECTED]:                   SelectionState::NotSelected
//! [SELECTED]:                       SelectionState::Selected
//! [Connection Mode]:                ConnectionMode
//! [PASSIVE]:                        ConnectionMode::Passive
//! [ACTIVE]:                         ConnectionMode::Active
//! [Receive]:                        rx
//! [Transmit]:                       tx
//! [Primitive Client]:               PrimitiveClient
//! [New Primitive Client]:           PrimitiveClient::new
//! [Primitive Connect Procedure]:    PrimitiveClient::connect
//! [Primitive Disconnect Procedure]: PrimitiveClient::disconnect
//! [Generic Client]:                 GenericClient
//! [New Generic Client]:             GenericClient::new
//! [Generic Connect Procedure]:      GenericClient::connect
//! [Generic Disconnect Procedure]:   GenericClient::disconnect
//! [Generic Data Procedure]:         GenericClient::data
//! [Generic Select Procedure]:       GenericClient::select
//! [Generic Deselect Procedure]:     GenericClient::deselect
//! [Generic Linktest Procedure]:     GenericClient::linktest
//! [Generic Separate Procedure]:     GenericClient::separate
//! [Generic Reject Procedure]:       GenericClient::reject
//! [Message]:                        Message
//! [Message Text]:                   Message::text
//! [Message Header]:                 MessageHeader
//! [Session ID]:                     MessageHeader::session_id
//! [Byte 2]:                         MessageHeader::byte_2
//! [Byte 3]:                         MessageHeader::byte_3
//! [System Bytes]:                   MessageHeader::system
//! [Presentation Type]:              PresentationType
//! [Session Type]:                   SessionType
//! [HSMS Message]:                   HsmsMessage
//! [Data Message]:                   DataMessage
//! [Select.req]:                     SelectRequest
//! [Select.rsp]:                     SelectResponse
//! [Deselect.req]:                   DeselectRequest
//! [Deselect.rsp]:                   DeselectResponse
//! [Linktest.req]:                   LinktestRequest
//! [Linktest.rsp]:                   LinktestResponse
//! [Reject.req]:                     RejectRequest
//! [Separate.req]:                   SeparateRequest
//! [Parameter Settings]:             ParameterSettings
//! [Connect Mode]:                   ParameterSettings::connect_mode
//! [T3]:                             ParameterSettings::t3
//! [T5]:                             ParameterSettings::t5
//! [T6]:                             ParameterSettings::t6
//! [T7]:                             ParameterSettings::t7
//! [T8]:                             ParameterSettings::t8

#![crate_name = "hsms"]
#![crate_type = "lib"]

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

// SECTION 5: OVERVIEW AND STATE DIAGRAM

/// ## CONNECTION STATE
/// **Based on SEMI E37-1109§5.4-5.5**
/// 
/// In HSMS, two [Connection State]s exist, [NOT CONNECTED], and [CONNECTED].
/// 
/// The [Client] will move between them based on whether it has established
/// a TCP/IP connection to a Remote Entity, and the integrity of that
/// connection.
/// 
/// [Connection State]: ConnectionState
/// [NOT CONNECTED]:    ConnectionState::NotConnected
/// [CONNECTED]:        ConnectionState::Connected
/// [Client]:           GenericClient
#[derive(Debug)]
pub enum ConnectionState {
  /// ### NOT CONNECTED
  /// **Based on SEMI E37-1109§5.5.1**
  /// 
  /// In this state, the [Client] is ready to initiate the [Connect Procedure]
  /// but has either not yet done so, or has terminated a previous connection.
  /// 
  /// [Client]:            GenericClient
  /// [Connect Procedure]: GenericClient::connect
  NotConnected,

  /// ### CONNECTED
  /// **Based on SEMI E37-1109§5.5.2**
  /// 
  /// In this state, the [Client] has successfully initiated the
  /// [Connect Procedure] and is able to send and receive data. This state has
  /// two [Selection State]s, [NOT SELECTED], and [SELECTED].
  /// 
  /// [Client]:            GenericClient
  /// [Connect Procedure]: GenericClient::connect
  /// [Selection State]:   SelectionState
  /// [NOT SELECTED]:      SelectionState::NotSelected
  /// [SELECTED]:          SelectionState::Selected
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

/// ## SELECTION STATE
/// **Based on SEMI E37-1109§5.5.2**
/// 
/// The [CONNECTED] state has two substates, [NOT SELECTED] and [SELECTED].
/// 
/// The [Client] moves between them based on whether it has established an HSMS
/// session with another entity according to the [Select Procedure].
/// 
/// [Client]:           GenericClient
/// [Select Procedure]: GenericClient::select
/// [CONNECTED]:        ConnectionState::Connected
/// [NOT SELECTED]:     SelectionState::NotSelected
/// [SELECTED]:         SelectionState::Selected
#[derive(Clone, Copy, Debug)]
pub enum SelectionState {
  /// ### NOT SELECTED
  /// **Based on SEMI E37-1109§5.5.2.1**
  /// 
  /// In this state, the [Client] is ready to initiate the [Select Procedure]
  /// but has either not yet done so, or has terminated a previous session.
  /// 
  /// [Client]:           GenericClient
  /// [Select Procedure]: GenericClient::select
  NotSelected,

  /// ### SELECTED
  /// **Based on SEMI E37-1109§5.5.2.2**
  /// 
  /// In this state, the [Client] has successfully initiated the
  /// [Select Procedure] and is able to send and receive [Data Message]s.
  /// 
  /// [Client]:           GenericClient
  /// [Select Procedure]: GenericClient::select
  /// [Data Message]:     DataMessage
  Selected(u16),
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
  /// Used to indicate that the [Client] has failed to perform a certain
  /// procedure, but that this failure has not changed its [Connection State]
  /// or [Selection State].
  /// 
  /// [Client]:           GenericClient
  /// [Connection State]: ConnectionState
  /// [Selection State]:  SelectionState
  None = 0,

  /// ### NOT CONNECTED
  /// **Based on SEMI E37-1109§5.6**
  /// 
  /// Used to indicate that the [Client] has failed to perform a certain
  /// procedure because the [NOT CONNECTED] state is currently active.
  /// 
  /// [Client]:        GenericClient
  /// [NOT CONNECTED]: ConnectionState::NotConnected
  NotConnected = 1,

  /// ### NOT CONNECTED TO NOT SELECTED
  /// **Based on SEMI E37-1109§5.6**
  /// 
  /// TCP/IP connection has been established.
  NotConnectedToNotSelected = 2,

  /// ### CONNECTED TO NOT CONNECTED
  /// **Based on SEMI E37-1109§5.6**
  /// 
  /// Used to indicate that the [Client] has initiated the
  /// [Disconnect Procedure] at an unexpected time, moving it from the
  /// [CONNECTED] to the [NOT CONNECTED] state.
  /// 
  /// [Client]:               GenericClient
  /// [Disconnect Procedure]: GenericClient::disconnect
  /// [NOT CONNECTED]:        ConnectionState::NotConnected
  /// [CONNECTED]:            ConnectionState::Connected
  ConnectedToNotConnected = 3,

  /// ### NOT SELECTED TO SELECTED
  /// **Based on SEMI E37-1109§5.6**
  /// 
  /// Used to indicate that the [Client] has responded to the
  /// [Select Procedure] being initated by the other entity, moving it from the
  /// [NOT SELECTED] to the [SELECTED] state.
  /// 
  /// [Client]:           GenericClient
  /// [Select Procedure]: GenericClient::select
  /// [NOT SELECTED]:     SelectionState::NotSelected
  /// [SELECTED]:         SelectionState::Selected
  NotSelectedToSelected = 4,

  /// ### SELECTED TO NOT SELECTED
  /// **Based on SEMI E37-1109§5.6**
  /// 
  /// Used to indicate that the [Client] has responded to the
  /// [Deselect Procedure] or the [Separate Procedure] being initiated by the
  /// other entity, moving it from the [SELECTED] to the [NOT SELECTED] state.
  /// 
  /// [Client]:             GenericClient
  /// [Deselect Procedure]: GenericClient::deselect
  /// [Separate Procedure]: GenericClient::separate
  /// [NOT SELECTED]:       SelectionState::NotSelected
  /// [SELECTED]:           SelectionState::Selected
  SelectedToNotSelected = 5,

  /// ### NOT SELECTED TO NOT CONNECTED
  /// **Based on SEMI E37-1109§5.6**
  /// 
  /// Used to indicate that the [Client] remained in the [NOT SELECTED] state
  /// for longer than the amount of time specified by [T7], moving it from the
  /// [CONNECTED] state to the [NOT CONNECTED] state.
  /// 
  /// [Client]:        GenericClient
  /// [NOT CONNECTED]: ConnectionState::NotConnected
  /// [CONNECTED]:     ConnectionState::Connected
  /// [NOT SELECTED]:  SelectionState::NotSelected
  /// [T7]:            ParameterSettings::t7
  NotSelectedToNotConnected = 6,
}

// SECTION 6: USE OF TCP/IP

/// ## CONNECTION MODE
/// **Based on SEMI E37-1109§6.3.2**
/// 
/// The [Client] must use one of two [Connection Mode]s, [PASSIVE] or [ACTIVE],
/// in order to perform the [Connect Procedure] and attain a TCP/IP connection.
/// 
/// [Client]:            GenericClient
/// [Connect Procedure]: GenericClient::connect
/// [Connection Mode]:   ConnectionMode
/// [PASSIVE]:           ConnectionMode::Passive
/// [ACTIVE]:            ConnectionMode::Active
#[derive(Clone, Copy, Debug)]
pub enum ConnectionMode {
  /// ### PASSIVE
  /// **Based on SEMI E37-1109§6.3.2**
  /// 
  /// In this mode, the [Client] listens for and accepts a the
  /// [Connect Procedure] when initiated by another entity.
  /// 
  /// [Client]:            GenericClient
  /// [Connect Procedure]: GenericClient::connect
  Passive,

  /// ### ACTIVE
  /// **Based on SEMI E37-1109§6.3.2**
  /// 
  /// In this mode, the [Client] initiates the [Connect Procedure] and waits
  /// up to the time specified by [T5] for the other entity to respond.
  /// 
  /// [Client]:            GenericClient
  /// [Connect Procedure]: GenericClient::connect
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

/// ## PRIMITIVE CLIENT
/// 
/// Encapsulates a limited set of functionality of the [HSMS] protocol, dealing
/// with the [Connect Procedure], [Disconnect Procedure], and basic exchange of
/// [HSMS Message]s.
/// 
/// [HSMS]:                 crate
/// [Connect Procedure]:    PrimitiveClient::connect
/// [Disconnect Procedure]: PrimitiveClient::disconnect
/// [HSMS Message]:         HsmsMessage
pub struct PrimitiveClient {
  pub(self) parameter_settings: ParameterSettings,
  connection_state: RwLock<ConnectionState>,
}

/// ## GENERIC CLIENT
/// 
/// Encapsulates the full functionality of the [HSMS] protocol.
/// 
/// [HSMS]: crate
pub struct GenericClient {
  primitive_client: Arc<PrimitiveClient>,
  selection_state: RwLock<SelectionState>,
  tx_sender: Mutex<Option<Sender<HsmsMessage>>>,
  outbox: Mutex<HashMap<u32, (u32, SessionType, SendOnce<Option<HsmsMessage>>)>>,
  system: Mutex<u32>,
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
  /// ### NEW CLIENT
  /// 
  /// Creates a [Client] in the [NOT CONNECTED] state, ready to initiate the
  /// [Connect Procedure].
  /// 
  /// [NOT CONNECTED]:     ConnectionState::NotConnected
  /// [Client]:            PrimitiveClient
  /// [Connect Procedure]: PrimitiveClient::connect
  pub fn new(parameter_settings: ParameterSettings) -> Arc<Self> {
    Arc::new(Self {
      parameter_settings,
      connection_state: Default::default(),
    })
  }

  /// ### CONNECT PROCEDURE
  /// **Based on SEMI E37-1109§6.3.4-6.3.7**
  /// 
  /// Connects the [Client] to another entity.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [NOT CONNECTED] to use this
  /// procedure.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connect Procedure] has two different behaviors based on the
  /// [Connect Mode], [PASSIVE] or [ACTIVE].
  /// 
  /// - [PASSIVE] - The socket address of the Local Entity must be
  /// provided, and the [Client] listens for and accepts a the
  /// [Connect Procedure] when initiated by another entity.
  /// 
  /// - [ACTIVE] - The socket address of the Remote Entity must be
  /// provided, and the [Client] initiates the [Connect Procedure] and waits
  /// up to the time specified by [T5] for the other entity to respond.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Upon completion of the [Connect Procedure], the [CONNECTED] state is
  /// entered.
  /// 
  /// [Client]:            PrimitiveClient
  /// [Connect Procedure]: PrimitiveClient::connect
  /// [PASSIVE]:           ConnectionMode::Passive
  /// [ACTIVE]:            ConnectionMode::Active
  /// [Connection State]:  ConnectionState
  /// [NOT CONNECTED]:     ConnectionState::NotConnected
  /// [CONNECTED]:         ConnectionState::Connected
  /// [Connect Mode]:      ParameterSettings::connect_mode
  /// [T5]:                ParameterSettings::t5
  pub fn connect(
    self: &Arc<Self>,
    entity: &str
  ) -> Result<(Receiver<HsmsMessage>, Sender<HsmsMessage>), Error> {
    //Derive TCP/IP Connection
    let stream = match self.connection_state.read().unwrap().deref() {
      //NOT CONNECTED
      ConnectionState::NotConnected => {
        //Connect Mode Switch
        match self.parameter_settings.connect_mode {
          //PASSIVE: Create Listener and Wait
          ConnectionMode::Passive => {
            let listener = TcpListener::bind(entity)?;
            let (stream, socket) = listener.accept()?;
            println!("PrimitiveClient::connect {:?}", socket);
            stream
          },
          //ACTIVE: Connect with Timeout
          ConnectionMode::Active => {
            let stream = TcpStream::connect_timeout(
              &entity.to_socket_addrs()?.next().ok_or(Error::new(ErrorKind::AddrNotAvailable, "INVALID ADDRESS"))?, 
              self.parameter_settings.t5,
            )?;
            println!("PrimitiveClient::connect {:?}", entity);
            stream
          },
        }
      },
      //CONNECTED: Error
      _ => return Err(Error::new(ErrorKind::AlreadyExists, "ALREADY CONNECTED")),
    };
    //Set Read and Write Timeouts to T8
    stream.set_read_timeout(Some(self.parameter_settings.t8))?;
    stream.set_write_timeout(Some(self.parameter_settings.t8))?;
    //Change Connection State
    *self.connection_state.write().unwrap().deref_mut() = ConnectionState::Connected(stream);
    //Create Channels
    let (rx_sender, rx_receiver) = channel::<HsmsMessage>();
    let (tx_sender, tx_receiver) = channel::<HsmsMessage>();
    //Start RX Thread
    let rx_clone: Arc<PrimitiveClient> = self.clone();
    let tx_sender_clone = tx_sender.clone();
    thread::spawn(move || {rx_clone.rx_handle(rx_sender.clone(), tx_sender_clone)});
    //Start TX Thread
    let tx_clone: Arc<PrimitiveClient> = self.clone();
    thread::spawn(move || {tx_clone.tx_handle(tx_receiver)});
    //Finish
    Ok((rx_receiver, tx_sender))
  }

  /// ### DISCONNECT PROCEDURE
  /// **Based on SEMI E37-1109§6.4-6.5**
  /// 
  /// Disconnects the [Client] from the other entity.
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
  /// [Client]:               PrimitiveClient
  /// [Disconnect Procedure]: PrimitiveClient::disconnect
  /// [Connection State]:     ConnectionState
  /// [NOT CONNECTED]:        ConnectionState::NotConnected
  /// [CONNECTED]:            ConnectionState::Connected
  /// [Selection State]:      SelectionState
  /// [SELECTED]:             SelectionState::Selected
  pub fn disconnect(
    self: &Arc<Self>
  ) -> Result<(), ConnectionStateTransition> {
    println!("PrimitiveClient::disconnect");
    //Read Connection State
    {
      let connection_state = self.connection_state.read().unwrap();
      match connection_state.deref() {
        //CONNECTED
        ConnectionState::Connected(stream) => {
          //Shutdown TCP/IP Connection
          //This should cause all read locks on the connection state to release.
          let _ = stream.shutdown(Shutdown::Both);
        },
        //NOT CONNECTED: Error
        _ => return Err(ConnectionStateTransition::None),
      }
    }
    //Change Connection State
    {
      let mut connection_state = self.connection_state.write().unwrap();
      *connection_state.deref_mut() = ConnectionState::NotConnected;
    }
    //TODO: Clear Outbox
    Ok(())
  }
}

/// ## GENERIC CLIENT: CONNECTION PROCEDURES
/// **Based on SEMI E37-1109§6.3-6.5**
/// 
/// Encapsulates the parts of the [Client]'s functionality dealing with
/// establishing and breaking a TCP/IP connection.
/// 
/// - [New Client]
/// - [Connect Procedure]
/// - [Disconnect Procedure]
/// 
/// [Client]:               GenericClient
/// [New Client]:           GenericClient::new
/// [Connect Procedure]:    GenericClient::connect
/// [Disconnect Procedure]: GenericClient::disconnect
impl GenericClient {
  /// ### NEW CLIENT
  /// 
  /// Creates a [Client] in the [NOT CONNECTED] state, ready to initiate the
  /// [Connect Procedure].
  /// 
  /// [NOT CONNECTED]:     ConnectionState::NotConnected
  /// [Client]:            GenericClient
  /// [Connect Procedure]: GenericClient::connect
  pub fn new(
    parameter_settings: ParameterSettings
  ) -> Arc<Self> {
    Arc::new(GenericClient {
      primitive_client: PrimitiveClient::new(parameter_settings),
      selection_state:  Default::default(),
      tx_sender:        Default::default(),
      outbox:           Default::default(),
      system:           Default::default(),
    })
  }

  /// ### CONNECT PROCEDURE
  /// **Based on SEMI E37-1109§6.3.4-6.3.7**
  /// 
  /// Connects the [Client] to another entity.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [NOT CONNECTED] to use this
  /// procedure.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connect Procedure] has two different behaviors based on the
  /// [Connect Mode], [PASSIVE] or [ACTIVE].
  /// 
  /// - [PASSIVE] - The socket address of the Local Entity must be
  /// provided, and the [Client] listens for and accepts a the
  /// [Connect Procedure] when initiated by another entity.
  /// 
  /// - [ACTIVE] - The socket address of the Remote Entity must be
  /// provided, and the [Client] initiates the [Connect Procedure] and waits
  /// up to the time specified by [T5] for the other entity to respond.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Upon completion of the [Connect Procedure], the [CONNECTED] state is
  /// entered.
  /// 
  /// [Client]:            GenericClient
  /// [Connect Procedure]: GenericClient::connect
  /// [PASSIVE]:           ConnectionMode::Passive
  /// [ACTIVE]:            ConnectionMode::Active
  /// [Connection State]:  ConnectionState
  /// [NOT CONNECTED]:     ConnectionState::NotConnected
  /// [CONNECTED]:         ConnectionState::Connected
  /// [Connect Mode]:      ParameterSettings::connect_mode
  /// [T5]:                ParameterSettings::t5
  pub fn connect(
    self: &Arc<Self>,
    entity: &str
  ) -> Result<Receiver<DataMessage>, Error> {
    println!("GenericClient::connect");
    //Connect Primitive
    let (rx_receiver, tx_sender) = self.primitive_client.connect(entity)?;
    *self.tx_sender.lock().unwrap().deref_mut() = Some(tx_sender.clone());
    //Create Channel
    let (data_sender, data_receiver) = channel::<DataMessage>();
    //Start RX Thread
    let clone: Arc<GenericClient> = self.clone();
    thread::spawn(move || {clone.rx_handle(rx_receiver, tx_sender, data_sender)});
    Ok(data_receiver)
  }

  /// ### DISCONNECT PROCEDURE
  /// **Based on SEMI E37-1109§6.4-6.5**
  /// 
  /// Disconnects the [Client] from the other entity.
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
  /// [Client]:               GenericClient
  /// [Disconnect Procedure]: GenericClient::disconnect
  /// [Separate Procedure]:   GenericClient::separate
  /// [Connection State]:     ConnectionState
  /// [NOT CONNECTED]:        ConnectionState::NotConnected
  /// [CONNECTED]:            ConnectionState::Connected
  /// [Selection State]:      SelectionState
  /// [SELECTED]:             SelectionState::Selected
  pub fn disconnect(
    self: &Arc<Self>,
  ) {
    println!("GenericClient::disconnect");
    *self.tx_sender.lock().unwrap().deref_mut() = None;
    *self.selection_state.write().unwrap().deref_mut() = SelectionState::NotSelected;
    for (_, (_, _, sender)) in self.outbox.lock().unwrap().deref_mut().drain() {
      let _ = sender.send(None);
    }
    let _ = self.primitive_client.disconnect();
  }
}

// SECTION 7: HSMS MESSAGE EXCHANGE PROCEDURES

/// ### RECEIVE MESSAGE
/// **Based on SEMI E37-1109§7.2**
/// 
/// Waits for a [Message] to be recieved over the TCP/IP connection, and
/// deserializes it.
pub fn rx(
  mut stream: &TcpStream,
) -> Result<Option<Message>, Error> {
  //Length [Bytes 0-3]
  let mut length_buffer: [u8;4] = [0;4];
  let length_bytes: usize = match stream.read(&mut length_buffer) {
    Ok(l) => l,
    Err(error) => match error.kind() {
      ErrorKind::TimedOut => {
        return Ok(None)
      },
      _ => {
        return Err(error)
      },
    }
  };
  if length_bytes != 4 {
    return Err(Error::new(ErrorKind::TimedOut, "T8"))
  }
  let length: u32 = u32::from_be_bytes(length_buffer);
  if length < 10 {
    return Err(Error::new(ErrorKind::InvalidData, "INVALID MESSAGE"))
  }
  //Header + Data [Bytes 4+]
  let mut message_buffer: Vec<u8> = vec![0; length as usize];
  let message_bytes: usize = stream.read(&mut message_buffer)?;
  if message_bytes != length as usize {
    return Err(Error::new(ErrorKind::TimedOut, "T8"))
  }
  //Finish
  println!(
    "rx {: >5} {: >3}{} {: >3} {: >3} {: >3} {: >10} {:?}",
    u16::from_be_bytes(message_buffer[0..2].try_into().unwrap()),
    &message_buffer[2] & 0b0111_1111,
    if (&message_buffer[2] & 0b1000_0000) > 0 {'W'} else {' '},
    &message_buffer[3],
    &message_buffer[4],
    &message_buffer[5],
    u32::from_be_bytes(message_buffer[6..10].try_into().unwrap()),
    &message_buffer[10..],
  );
  Ok(Some(Message::try_from(message_buffer).map_err(|()| -> Error {Error::new(ErrorKind::InvalidData, "INVALID MESSAGE")})?))
}

/// ### TRANSMIT MESSAGE
/// **Based on SEMI E37-1109§7.2**
/// 
/// Serializes a [Message] and transmits it over the TCP/IP connection.
pub fn tx(
  mut stream: &TcpStream,
  message: Message,
) -> Result<(), Error> {
  //Header + Data [Bytes 4+]
  let message_buffer: Vec<u8> = (&message).into();
  //Length [Bytes 0-3]
  let length: u32 = message_buffer.len() as u32;
  let length_buffer: [u8; 4] = length.to_be_bytes();
  //Finish
  stream.write_all(&length_buffer)?;
  stream.write_all(&message_buffer)?;
  println!(
    "tx {: >5} {: >3}{} {: >3} {: >3} {: >3} {: >10} {:?}",
    u16::from_be_bytes(message_buffer[0..2].try_into().unwrap()),
    &message_buffer[2] & 0b0111_1111,
    if (&message_buffer[2] & 0b1000_0000) > 0 {'W'} else {' '},
    &message_buffer[3],
    &message_buffer[4],
    &message_buffer[5],
    u32::from_be_bytes(message_buffer[6..10].try_into().unwrap()),
    &message_buffer[10..],
  );
  Ok(())
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
  /// A [Client] in the [CONNECTED] state will automatically [Receive]
  /// [Message]s and respond based on its contents.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### Valid [HSMS Message]
  /// 
  /// - The [Client] will send them to the Receiver provided by the
  /// [Connect Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### Invalid [Message]
  /// 
  /// - The [Client] will respond by [Transmit]ting a [Reject.req] message,
  /// completing the [Reject Procedure].
  /// 
  /// [Connection State]:  ConnectionState
  /// [CONNECTED]:         ConnectionState::Connected
  /// [Receive]:           rx
  /// [Transmit]:          tx
  /// [Client]:            PrimitiveClient
  /// [Connect Procedure]: PrimitiveClient::connect
  /// [Reject Procedure]:  GenericClient::reject
  /// [Message]:           Message
  /// [HSMS Message]:      HsmsMessage
  /// [Reject.req]:        RejectRequest
  fn rx_handle(
    self: Arc<Self>,
    rx_sender: Sender<HsmsMessage>,
    tx_sender: Sender<HsmsMessage>,
  ) {
    println!("PrimitiveClient::rx_handle start");
    while let ConnectionState::Connected(stream) = self.connection_state.read().unwrap().deref() {
      match rx(stream) {
        //RX Success
        Ok(optional_rx_message) => if let Some(rx_message) = optional_rx_message {
          let rx_header = rx_message.header;
          let optional_hsms_message: Result<HsmsMessage, RejectReason> = HsmsMessage::try_from(rx_message);
          match optional_hsms_message {
            //Known Message Type
            Ok(hsms_message) => {
              if rx_sender.send(hsms_message).is_err() {break}
            },
            //Unknown Message Type
            Err(reject_reason) => {
              //Reject.req
              if tx_sender.send(
                HsmsMessage::RejectRequest(RejectRequest{
                  session_id   : rx_header.session_id,
                  message_type : rx_header.session_type,
                  reason_code  : reject_reason as u8,
                  system       : rx_header.system,
                })
              ).is_err() {break}
            },
          }
        },
        //RX Failure
        Err(_error) => break,
      }
    }
    let _ = self.disconnect();
    println!("PrimitiveClient::rx_handle end");
  }

  /// ### TRANSMISSION HANDLER
  /// 
  /// A [Client] in the [CONNECTED] state will automatically [Transmit]
  /// [Message]s sent to it by the sender provided by the
  /// [Connect Procedure].
  /// 
  /// [Connection State]:  ConnectionState
  /// [CONNECTED]:         ConnectionState::Connected
  /// [Transmit]:          tx
  /// [Client]:            PrimitiveClient
  /// [Connect Procedure]: PrimitiveClient::connect
  fn tx_handle(
    self: Arc<Self>,
    tx_receiver: Receiver<HsmsMessage>
  ) {
    println!("PrimitiveClient::tx_handle start");
    for message in tx_receiver {
      match self.connection_state.read().unwrap().deref() {
        ConnectionState::Connected(stream) => {
          if tx(stream, message.into()).is_err() {break}
        },
        _ => break,
      }
    }
    let _ = self.disconnect();
    println!("PrimitiveClient::tx_handle end");
  }
}

/// ## GENERIC CLIENT: MESSAGE EXCHANGE PROCEDURES
/// **Based on SEMI E37-1109§7**
/// 
/// Encapsulates the parts of the [Client]'s functionality dealing with
/// exchanging [HSMS Message]s.
/// 
/// - [Data Procedure] - [Data Message]s
/// - [Select Procedure] - [Select.req] and [Select.rsp]
/// - [Deselect Procedure] - [Deselect.req] and [Deselect.rsp]
/// - [Linktest Procedure] - [Linktest.req] and [Linktest.rsp]
/// - [Separate Procedure] - [Separate.req]
/// - [Reject Procedure] - [Reject.req]
/// 
/// [HSMS]:               crate
/// [Client]:             GenericClient
/// [Inbox]:              GenericClient::inbox
/// [Select Procedure]:   GenericClient::select
/// [Data Procedure]:     GenericClient::data
/// [Deselect Procedure]: GenericClient::deselect
/// [Linktest Procedure]: GenericClient::linktest
/// [Separate Procedure]: GenericClient::separate
/// [Reject Procedure]:   GenericClient::reject
/// [HSMS Message]:       HsmsMessage
/// [Data Message]:       DataMessage
/// [Select.req]:         SelectRequest
/// [Select.rsp]:         SelectResponse
/// [Deselect.req]:       DeselectRequest
/// [Deselect.rsp]:       DeselectResponse
/// [Linktest.req]:       LinktestRequest
/// [Linktest.rsp]:       LinktestResponse
/// [Reject.req]:         RejectRequest
/// [Separate.req]:       SeparateRequest
impl GenericClient {
  /// ### RECEPTION HANDLER
  /// 
  /// A [Client] in the [CONNECTED] state will automatically [Receive]
  /// [Message]s and respond based on its contents and the current
  /// [Selection State].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### [Data Message]
  /// 
  /// - [NOT SELECTED] - The [Client] will respond by [Transmit]ting a
  /// [Reject.req] message, rejecting the [Data Procedure] and completing
  /// the [Reject Procedure].
  /// 
  /// - [SELECTED], Primary [Data Message] - The [Client] will respond by
  /// adding the message to the [Inbox].
  /// 
  /// - [SELECTED], Response [Data Message] - The [Client] will respond by
  /// correllating the message to a previously sent Primary [Data Message],
  /// finishing a previously initiated [Data Procedure] if successful, or
  /// if unsuccessful by [Transmit]ting a [Reject.req] message, rejecting the
  /// [Data Procedure] and completing the [Reject Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Select.req]:
  /// 
  /// - [NOT SELECTED] - The [Client] will respond with a [Select.rsp]
  /// accepting and completing the [Select Procedure].
  /// 
  /// - [SELECTED] - The [Client] will respond with a [Select.rsp] rejecting
  /// the [Select Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Select.rsp]:
  /// 
  /// - Not yet implemented.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Deselect.req]:
  /// 
  /// - Not yet implemented.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Deselect.rsp]:
  /// 
  /// - Not yet implemented.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Linktest.req]:
  /// 
  /// - The [Client] will respond with a [Linktest.rsp], completing the
  /// [Linktest Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Linktest.rsp]:
  /// 
  /// - The [Client] will respond by correllating the message to a previously
  /// sent [Linktest.req] message, finishing a previously initiated
  /// [Linktest Procedure] if successful, or if unsuccessful by [Transmit]ting
  /// a [Reject.req] message, completing the [Reject Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Reject.req]:
  /// 
  /// - Not yet implemented.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// [Separate.req]:
  /// 
  /// - [NOT SELECTED] - The [Client] will not do anything.
  /// 
  /// - [SELECTED] - The [Client] will complete the [Separate Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Unknown [Message]:
  /// 
  /// - The [Client] will respond by [Transmit]ting a [Reject.req] message,
  /// completing the [Reject Procedure]. 
  /// 
  /// [Client]:             GenericClient
  /// [Connection State]:   ConnectionState
  /// [NOT CONNECTED]:      ConnectionState::NotConnected
  /// [CONNECTED]:          ConnectionState::Connected
  /// [Selection State]:    SelectionState
  /// [NOT SELECTED]:       SelectionState::NotSelected
  /// [SELECTED]:           SelectionState::Selected
  /// [Receive]:            rx
  /// [Transmit]:           tx
  /// [Inbox]:              GenericClient::inbox
  /// [Select Procedure]:   GenericClient::select
  /// [Data Procedure]:     GenericClient::data
  /// [Deselect Procedure]: GenericClient::deselect
  /// [Linktest Procedure]: GenericClient::linktest
  /// [Separate Procedure]: GenericClient::separate
  /// [Reject Procedure]:   GenericClient::reject
  /// [Message]:            Message
  /// [Data Message]:       DataMessage
  /// [Select.req]:         SelectRequest
  /// [Select.rsp]:         SelectResponse
  /// [Deselect.req]:       DeselectRequest
  /// [Deselect.rsp]:       DeselectResponse
  /// [Linktest.req]:       LinktestRequest
  /// [Linktest.rsp]:       LinktestResponse
  /// [Reject.req]:         RejectRequest
  /// [Separate.req]:       SeparateRequest
  fn rx_handle(
    self: &Arc<Self>,
    rx_receiver: Receiver<HsmsMessage>,
    tx_sender: Sender<HsmsMessage>,
    rx_sender: Sender<DataMessage>,
  ) {
    println!("GenericClient::rx_handle start");
    for hsms_message in rx_receiver {
      match hsms_message {
        //Data Message
        HsmsMessage::DataMessage(data_message) => {
          match self.selection_state.read().unwrap().deref() {
            //SELECTED
            SelectionState::Selected(_session_id) => {
              if data_message.function % 2 == 1 { //Primary Data Message
                if rx_sender.send(data_message).is_err() {break}
              } else { //Response Data Message
                let mut outbox = self.outbox.lock().unwrap();
                let mut optional_transaction: Option<u32> = None;
                for (outbox_id, (system, session_type, _)) in outbox.deref() {
                  if *system == data_message.system && *session_type == SessionType::DataMessage {
                    optional_transaction = Some(*outbox_id);
                    break;
                  }
                }
                if let Some(transaction) = optional_transaction {
                  //Outbox
                  let (_, _, sender) = outbox.deref_mut().remove(&transaction).unwrap();
                  sender.send(Some(HsmsMessage::DataMessage(data_message))).unwrap();
                } else {
                  //Reject.req
                  if tx_sender.send(HsmsMessage::RejectRequest(RejectRequest {
                    session_id: data_message.session_id,
                    message_type: 0,
                    reason_code: RejectReason::TransactionNotOpen as u8,
                    system: data_message.system
                  })).is_err() {break}
                }
              }
            },
            //NOT SELECTED
            SelectionState::NotSelected => {
              //Reject.req
              if tx_sender.send(HsmsMessage::RejectRequest(RejectRequest {
                session_id: data_message.session_id,
                message_type: 0,
                reason_code: RejectReason::EntityNotSelected as u8,
                system: data_message.system
              })).is_err() {break}
            },
          }
        },
        //Select.req
        HsmsMessage::SelectRequest(select_request) => {
          let mut select = self.selection_state.write().unwrap();
          if let SelectionState::Selected(_session_id) = select.deref() {
            //Select.rsp: Already Active
            if tx_sender.send(HsmsMessage::SelectResponse(SelectResponse {
              session_id: select_request.session_id,
              status: SelectStatus::AlreadyActive as u8,
              system: select_request.system,
            })).is_err() {break};
          } else {
            //Select.rsp: Success
            if tx_sender.send(HsmsMessage::SelectResponse(SelectResponse {
              session_id: select_request.session_id,
              status: SelectStatus::Success as u8,
              system: select_request.system,
            })).is_err() {break};
            *select.deref_mut() = SelectionState::Selected(select_request.session_id)
          }
        },
        //Select.rsp
        HsmsMessage::SelectResponse(_select_response) => {
          todo!()
        },
        //Deselect.req
        HsmsMessage::DeselectRequest(_deselect_request) => {
          todo!()
        },
        //Deselect.rsp
        HsmsMessage::DeselectResponse(_deselect_response) => {
          todo!()
        },
        //Linktest.req
        HsmsMessage::LinktestRequest(linktest_request) => {
          //Linktest.rsp
          if tx_sender.send(HsmsMessage::LinktestResponse(LinktestResponse {
            system: linktest_request.system
          })).is_err() {break};
        },
        //Linktest.rsp
        HsmsMessage::LinktestResponse(linktest_response) => {
          let mut outbox = self.outbox.lock().unwrap();
          let mut optional_transaction: Option<u32> = None;
          for (outbox_id, (system, session_type, _)) in outbox.deref() {
            if *system == linktest_response.system && *session_type == SessionType::LinktestRequest {
              optional_transaction = Some(*outbox_id);
              break;
            }
          }
          if let Some(transaction) = optional_transaction {
            //Outbox
            let (_, _, sender) = outbox.deref_mut().remove(&transaction).unwrap();
            sender.send(Some(HsmsMessage::LinktestResponse(linktest_response))).unwrap();
          } else {
            //Reject.req
            if tx_sender.send(HsmsMessage::RejectRequest(RejectRequest {
              session_id: 0xFFFF,
              message_type: 0,
              reason_code: RejectReason::TransactionNotOpen as u8,
              system: linktest_response.system
            })).is_err() {break}
          }
        },
        //Reject.req
        HsmsMessage::RejectRequest(_reject_request) => {
          todo!()
        },
        //Separate.req
        HsmsMessage::SeparateRequest(separate_request) => {
          let mut select = self.selection_state.write().unwrap();
          if let SelectionState::Selected(session_id) = select.deref() {
            if *session_id == separate_request.session_id {
              *select.deref_mut() = SelectionState::NotSelected;
            }
          }
        },
      }
    }
    self.disconnect();
    println!("GenericClient::rx_handle end");
  }

  /// ### TRANSMISSION HANDLER
  fn tx_handle(
    self: &Arc<Self>,
    message: HsmsMessage,
    reply_expected: bool,
    delay: Duration,
  ) -> Option<HsmsMessage> {
    println!("GenericClient::tx_handle");
    let (system_bytes, session_type) = (message.system_bytes(), message.session_type());
    match self.tx_sender.lock().unwrap().deref() {
      Some(tx_sender) => {
        if reply_expected {
          let res = {
            //Acquire Locks
            let mut outbox = self.deref().outbox.lock().unwrap();
            //Attempt TX
            match tx_sender.send(message) {
              //TX Success
              Ok(_) => {
                //Spawn Oneshot
                let (sender, receiver) = oneshot::channel::<Option<HsmsMessage>>();
                let system = {
                  let mut system_guard = self.deref().system.lock().unwrap();
                  let system_counter = system_guard.deref_mut();
                  let system = *system_counter;
                  *system_counter += 1;
                  system
                };
                outbox.deref_mut().insert(system, (system_bytes, session_type, sender));
                Some((receiver, system))
              },
              //TX Failure
              Err(_) => {
                self.disconnect();
                None
              },
            }
          };
          match res {
            Some((receiver, system)) => {
              let rx_result = receiver.recv_timeout(delay);
              let mut outbox = self.outbox.lock().unwrap();
              outbox.deref_mut().remove(&system);
              match rx_result {
                Ok(rx_message) => rx_message,
                Err(_e) => None,
              }
            },
            None => None,
          }
        } else {
          if tx_sender.send(message).is_err() {self.disconnect()};
          None
        }
      },
      None => None,
    }
  }

  /// ### DATA PROCEDURE
  /// **Based on SEMI E37-1109§7.5-7.6**
  /// 
  /// Asks the [Client] to initiate the [Data Procedure] by [Transmit]ting a
  /// Primary [Data Message] and waiting for the corresponding Response
  /// [Data Message] to be [Receive]d if it is necessary to do so.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state and the
  /// [Selection State] must be in the [SELECTED] state to use this procedure.
  /// 
  /// When a Response [Data Message] is necessary, the [Client] will wait
  /// to [Receive] it for the amount of time specified by [T3] before it will
  /// consider it a communications failure and initiate the
  /// [Disconnect Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Although not done within this function, a [Client] in the [CONNECTED]
  /// state will automatically respond to having [Receive]d a [Data Message]
  /// based on its contents and the current [Selection State]:
  /// 
  /// - [NOT SELECTED] - The [Client] will respond by [Transmit]ting a
  /// [Reject.req] message, rejecting the [Data Procedure] and completing
  /// the [Reject Procedure].
  /// 
  /// - [SELECTED], Primary [Data Message] - The [Client] will send the message
  /// to the receiver provided after the [Connect Procedure] succeeded.
  /// 
  /// - [SELECTED], Response [Data Message] - The [Client] will respond by
  /// correllating the message to a previously sent Primary [Data Message],
  /// finishing a previously initiated [Data Procedure] if successful, or
  /// if unsuccessful by [Transmit]ting a [Reject.req] message, rejecting the
  /// [Data Procedure] and completing the [Reject Procedure].
  /// 
  /// [Client]:               GenericClient
  /// [Connection State]:     ConnectionState
  /// [CONNECTED]:            ConnectionState::Connected
  /// [Selection State]:      SelectionState
  /// [NOT SELECTED]:         SelectionState::NotSelected
  /// [SELECTED]:             SelectionState::Selected
  /// [Connect Procedure]:    GenericClient::connect
  /// [Disconnect Procedure]: GenericClient::disconnect
  /// [Receive]:              rx
  /// [Transmit]:             tx
  /// [Data Procedure]:       GenericClient::data
  /// [Reject Procedure]:     GenericClient::reject
  /// [Data Message]:         DataMessage
  /// [Reject.req]:           RejectRequest
  /// [T3]:                   ParameterSettings::t3
  pub fn data(
    self: &Arc<Self>,
    message: DataMessage,
  ) -> JoinHandle<Result<Option<HsmsMessage>, ConnectionStateTransition>> {
    println!("GenericClient::data");
    let clone: Arc<GenericClient> = self.clone();
    let reply_expected = message.function % 2 == 1 && message.w;
    thread::spawn(move || {
      match clone.tx_handle(
        HsmsMessage::DataMessage(message),
        reply_expected,
        clone.primitive_client.parameter_settings.t3,
      ) {
        Some(rx_message) => Ok(Some(rx_message)),
        None => {
          if reply_expected {
            clone.disconnect();
            Err(ConnectionStateTransition::ConnectedToNotConnected)
          } else {
            Ok(None)
          }
        },
      }
    })
  }

  /// ### SELECT PROCEDURE (TODO)
  /// **Based on SEMI E37-1109§7.3-7.4**
  /// 
  /// Asks the [Client] to initiate the [Select Procedure] by [Transmit]ting
  /// a [Select.req] message and waiting for the corresponding [Select.rsp]
  /// message to be [Receive]d.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state and the
  /// [Selection State] must be in the [NOT SELECTED] state to use this
  /// procedure.
  /// 
  /// The [Client] will wait to [Receive] the [Select.rsp] for the amount of
  /// time specified by [T6] before it will consider it a communications
  /// failure and initiate the [Disconnect Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Although not done within this function, a [Client] in the [CONNECTED]
  /// state will automatically respond to having [Receive]d a [Select.req]
  /// message based on its current [Selection State]:
  /// 
  /// - [NOT SELECTED] - The [Client] will respond with a [Select.rsp]
  /// accepting and completing the [Select Procedure].
  /// 
  /// - [SELECTED] - The [Client] will respond with a [Select.rsp] rejecting
  /// the [Select Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Upon completion of the [Select Procedure], the [SELECTED] state
  /// is entered.
  /// 
  /// [Client]:               GenericClient
  /// [Connection State]:     ConnectionState
  /// [CONNECTED]:            ConnectionState::Connected
  /// [Selection State]:      SelectionState
  /// [NOT SELECTED]:         SelectionState::NotSelected
  /// [SELECTED]:             SelectionState::Selected
  /// [Disconnect Procedure]: GenericClient::disconnect
  /// [Receive]:              rx
  /// [Transmit]:             tx
  /// [Select Procedure]:     GenericClient::select
  /// [Select.req]:           SelectRequest
  /// [Select.rsp]:           SelectResponse
  /// [T6]:                   ParameterSettings::t6
  pub fn select(
    self: &Arc<Self>,
    _session_id: u16,
  ) -> JoinHandle<Result<(), ConnectionStateTransition>> {
    println!("GenericClient::select");
    todo!()
  }

  /// ### DESELECT PROCEDURE (TODO)
  /// **Based on SEMI E37-1109§7.7**
  /// 
  /// Asks the [Client] to initiate the [Deselect Procedure] by [Transmit]ting
  /// a [Deselect.req] message and waiting for the corresponding [Deselect.rsp]
  /// message to be [Receive]d.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state and the
  /// [Selection State] must be in the [SELECTED] state to use this procedure.
  /// 
  /// The [Client] will wait to [Receive] the [Deselect.rsp] for the amount of
  /// time specified by [T6] before it will consider it a communications
  /// failure and initiate the [Disconnect Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Although not done within this function, a [Client] in the [CONNECTED]
  /// state will automatically respond to having [Receive]d a [Deselect.req]
  /// message based on its current [Selection State]:
  /// 
  /// - [NOT SELECTED] - The [Client] will respond with a [Deselect.rsp]
  /// rejecting the [Deselect Procedure].
  /// 
  /// - [SELECTED] - The [Client] will respond with a [Deselect.rsp] accepting
  /// and completing the [Deselect Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Upon completion of the [Deselect Procedure], the [NOT SELECTED] state
  /// is entered.
  /// 
  /// [Client]:               GenericClient
  /// [Connection State]:     ConnectionState
  /// [CONNECTED]:            ConnectionState::Connected
  /// [Selection State]:      SelectionState
  /// [NOT SELECTED]:         SelectionState::NotSelected
  /// [SELECTED]:             SelectionState::Selected
  /// [Disconnect Procedure]: GenericClient::disconnect
  /// [Receive]:              rx
  /// [Transmit]:             tx
  /// [Deselect Procedure]:   GenericClient::deselect
  /// [Deselect.req]:         DeselectRequest
  /// [Deselect.rsp]:         DeselectResponse
  /// [T6]:                   ParameterSettings::t6
  pub fn deselect(
    self: &Arc<Self>,
  ) -> Result<(), ConnectionStateTransition> {
    println!("GenericClient::deselect");
    todo!()
  }

  /// ### LINKTEST PROCEDURE
  /// **Based on SEMI E37-1109§7.8**
  /// 
  /// Asks the [Client] to initiate the [Linktest Procedure] by [Transmit]ting
  /// a [Linktest.req] message and waiting for the corresponding [Linktest.rsp]
  /// message to be [Receive]d.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state to use this
  /// procedure.
  /// 
  /// The [Client] will wait to [Receive] the [Linktest.rsp] for the amount of
  /// time specified by [T6] before it will consider it a communications
  /// failure and initiate the [Disconnect Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Although not done within this function, a [Client] in the [CONNECTED]
  /// state will automatically respond to having [Receive]d a [Linktest.req]
  /// message:
  /// 
  /// - The [Client] will respond with a [Linktest.rsp], completing the
  /// [Linktest Procedure].
  /// 
  /// [Client]:               GenericClient
  /// [Connection State]:     ConnectionState
  /// [CONNECTED]:            ConnectionState::Connected
  /// [Selection State]:      SelectionState
  /// [NOT SELECTED]:         SelectionState::NotSelected
  /// [SELECTED]:             SelectionState::Selected
  /// [Disconnect Procedure]: GenericClient::disconnect
  /// [Receive]:              rx
  /// [Transmit]:             tx
  /// [Linktest Procedure]:   GenericClient::linktest
  /// [Linktest.req]:         LinktestRequest
  /// [Linktest.rsp]:         LinktestResponse
  /// [T6]:                   ParameterSettings::t6
  pub fn linktest(
    self: &Arc<Self>,
  ) -> JoinHandle<Result<(), ConnectionStateTransition>> {
    println!("GenericClient::linktest");
    let clone: Arc<GenericClient> = self.clone();
    thread::spawn(move || {
      match clone.tx_handle(
        HsmsMessage::LinktestRequest(LinktestRequest { system: 0xFFFF }),
        true,
        clone.primitive_client.parameter_settings.t6,
      ) {
        Some(rx_message) => {
          if let HsmsMessage::LinktestResponse(_linktest_response) = rx_message {
            Ok(())
          } else {
            Err(ConnectionStateTransition::None)
          }
        },
        None => {
          clone.disconnect();
          Err(ConnectionStateTransition::ConnectedToNotConnected)
        },
      }
    })
  }

  /// ### SEPARATE PROCEDURE (TODO)
  /// **Based on SEMI E37-1109§7.9**
  /// 
  /// Asks the [Client] to initiate the [Separate Procedure] by [Transmit]ting
  /// a [Separate.req] message.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state and the
  /// [Selection State] must be in the [SELECTED] state to use this procedure.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Although not done within this function, a [Client] in the [CONNECTED]
  /// state will automatically respond to having [Receive]d a [Separate.req]
  /// message based on its current [Selection State]:
  /// 
  /// - [NOT SELECTED] - The [Client] will not do anything.
  /// 
  /// - [SELECTED] - The [Client] will complete the [Separate Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Upon completion of the [Separate Procedure], the [NOT SELECTED] state
  /// is entered.
  /// 
  /// [Client]:             GenericClient
  /// [Connection State]:   ConnectionState
  /// [CONNECTED]:          ConnectionState::Connected
  /// [Selection State]:    SelectionState
  /// [NOT SELECTED]:       SelectionState::NotSelected
  /// [SELECTED]:           SelectionState::Selected
  /// [Receive]:            rx
  /// [Transmit]:           tx
  /// [Separate Procedure]: GenericClient::separate
  /// [Separate.req]:       SeparateRequest
  pub fn separate(
    self: &Arc<Self>,
  ) -> Result<(), ConnectionStateTransition> {
    println!("GenericClient::separate");
    todo!()
  }

  /// ### REJECT PROCEDURE (TODO)
  /// **Based on SEMI E37-1109§7.10**
  /// 
  /// Asks the [Client] to initiate the [Reject Procedure] by [Transmit]ting
  /// a [Reject.req] message.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state to use this
  /// procedure.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Although not done within this function, a [Client] in the [CONNECTED]
  /// state will automatically respond to having [Receive]d a [Reject.req]:
  /// 
  /// - Not yet implemented.
  /// 
  /// [Client]:           GenericClient
  /// [Connection State]: ConnectionState
  /// [CONNECTED]:        ConnectionState::Connected
  /// [Selection State]:  SelectionState
  /// [NOT SELECTED]:     SelectionState::NotSelected
  /// [SELECTED]:         SelectionState::Selected
  /// [Receive]:          rx
  /// [Transmit]:         tx
  /// [Reject Procedure]: GenericClient::reject
  /// [Reject.req]:       RejectRequest
  pub fn reject(
    self: &Arc<Self>,
    _header: MessageHeader,
    _reason: RejectReason,
  ) -> Result<(), ConnectionStateTransition> {
    println!("GenericClient::reject");
    todo!()
  }
}

// SECTION 8: HSMS MESSAGE FORMAT

/// ## MESSAGE
/// **Based on SEMI E37-1109§8.2**
/// 
/// Data using the [HSMS] defined structure, but not enforcing compliance
/// with the standards for how its fields are filled and what they mean.
/// 
/// Note that the Message Length field defined in the standard is not included,
/// as it is only temporarily used when a message is [Receive]d or [Transmit]ted
/// by the [Client].
/// 
/// [HSMS]:     crate
/// [Client]:   GenericClient
/// [Receive]:  rx
/// [Transmit]: tx
#[derive(Clone, Debug)]
pub struct Message {
  /// ### MESSAGE HEADER
  /// 
  /// Contains information about the [Message] using the standard-defined
  /// [Message Header] format.
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
  /// [Presentation Type]: PresentationType
  /// [Session Type]:      SessionType
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
#[derive(Clone, Copy, Debug)]
pub struct MessageHeader {
  /// ### SESSION ID
  /// **Based on SEMI E37-1109§8.2.6.1**
  /// 
  /// Provides by association a reference between the [Select Procedure]
  /// and subsequent other [Message]s.
  pub session_id : u16,

  /// ### HEADER BYTE 2
  /// **Based on SEMI E37-1109§8.2.6.2**
  /// 
  /// Contains information specific to the [Presentation Type] and
  /// [Session Type].
  /// 
  /// [Presentation Type]: PresentationType
  /// [Session Type]:      SessionType
  pub byte_2 : u8,

  /// ### HEADER BYTE 3
  /// **Based on SEMI E37-1109§8.2.6.3**
  /// 
  /// Contains information specific to the [Presentation Type] and
  /// [Session Type].
  /// 
  /// [Presentation Type]: PresentationType
  /// [Session Type]:      SessionType
  pub byte_3 : u8,

  /// ### PRESENTATION TYPE
  /// **Based on SEMI E37-1109§8.2.6.4**
  /// 
  /// An enumerated value, the [Presentation Type], defining the
  /// Presentation Layer content of the [Message Text].
  /// 
  /// [Message Text]:      Message::text
  /// [Presentation Type]: PresentationType
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

/// ## PRESENTATION TYPE
/// **Based on SEMI E37-1109§8.2.6.4**
/// 
/// Defines the Presentation Layer content of [Message Text].
/// 
/// Values 1-127 are reserved for Subsidiary Standards.
/// 
/// Values 128-255 are reserved and may not be used.
/// 
/// [Message Text]: Message::text
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PresentationType {
  /// ### SECS II ENCODING
  /// 
  /// Denotes an [HSMS Message], which is often a SECS-II formatted
  /// [Data Message].
  /// 
  /// [HSMS Message]: HsmsMessage
  /// [Data Message]: DataMessage
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
  /// [Data Message]: DataMessage 
  DataMessage = 0,

  /// ### SELECT REQUEST
  /// 
  /// Denotes a [Select.req] message.
  /// 
  /// [Select.req]: SelectRequest
  SelectRequest = 1,

  /// ### SELECT RESPONSE
  /// 
  /// Denotes a [Select.rsp] message.
  /// 
  /// [Select.rsp]: SelectResponse
  SelectResponse = 2,

  /// ### DESELECT REQUEST
  /// 
  /// Denotes a [Deselect.req] message.
  /// 
  /// [Deselect.req]: DeselectRequest
  DeselectRequest = 3,

  /// ### DESELECT RESPONSE
  /// 
  /// Denotes a [Deselect.rsp] message.
  /// 
  /// [Deselect.rsp]: DeselectResponse
  DeselectResponse = 4,

  /// ### LINKTEST REQUEST
  /// 
  /// Denotes a [Linktest.req] message.
  /// 
  /// [Linktest.req]: LinktestRequest
  LinktestRequest = 5,

  /// ### LINKTEST RESPONSE
  /// 
  /// Denotes a [Linktest.rsp] message.
  /// 
  /// [Linktest.rsp]: LinktestResponse
  LinktestResponse = 6,

  /// ### REJECT REQUEST
  /// 
  /// Denotes a [Reject.req] message.
  /// 
  /// [Reject.req]: RejectRequest
  RejectRequest = 7,

  /// ### SEPARATE REQUEST
  /// 
  /// Denotes a [Separate.req] message.
  /// 
  /// [Separate.req]: SeparateRequest
  SeparateRequest = 9,
}

/// ## HSMS MESSAGE
/// **Based on SEMI E37-1109§8.3**
/// 
/// A [Message] with a [Presentation Type] of 0, interpreted as one of
/// the following based on the [Session Type]:
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
/// [Message]:           Message
/// [Presentation Type]: PresentationType
/// [Session Type]:      SessionType
/// [HSMS Message]:      HsmsMessage
/// [Data Message]:      DataMessage
/// [Select.req]:        SelectRequest
/// [Select.rsp]:        SelectResponse
/// [Deselect.req]:      DeselectRequest
/// [Deselect.rsp]:      DeselectResponse
/// [Linktest.req]:      LinktestRequest
/// [Linktest.rsp]:      LinktestResponse
/// [Reject.req]:        RejectRequest
/// [Separate.req]:      SeparateRequest
#[repr(u8)]
#[derive(Clone, Debug)]
pub enum HsmsMessage {
  DataMessage      (DataMessage)      = SessionType::DataMessage      as u8,
  SelectRequest    (SelectRequest)    = SessionType::SelectRequest    as u8,
  SelectResponse   (SelectResponse)   = SessionType::SelectResponse   as u8,
  DeselectRequest  (DeselectRequest)  = SessionType::DeselectRequest  as u8,
  DeselectResponse (DeselectResponse) = SessionType::DeselectResponse as u8,
  LinktestRequest  (LinktestRequest)  = SessionType::LinktestRequest  as u8,
  LinktestResponse (LinktestResponse) = SessionType::LinktestResponse as u8,
  RejectRequest    (RejectRequest)    = SessionType::RejectRequest    as u8,
  SeparateRequest  (SeparateRequest)  = SessionType::SeparateRequest  as u8,
}
impl From<HsmsMessage> for Message {
  /// ### HSMS MESSAGE INTO GENERIC MESSAGE
  /// 
  /// Due to the fact that valid HSMS Messages are a subset of valid Messages,
  /// this operation is infallible.
  fn from(val: HsmsMessage) -> Self {
    match val {
      HsmsMessage::DataMessage(message) => {
        Message {
          header: MessageHeader {
            session_id        : message.session_id,
            byte_2            : ((message.w as u8) << 7) | message.stream,
            byte_3            : message.function,
            presentation_type : PresentationType::SecsII as u8,
            session_type      : SessionType::DataMessage as u8,
            system            : message.system,
          },
          text: message.text,
        }
      },
      HsmsMessage::SelectRequest(message) => {
        Message {
          header: MessageHeader {
            session_id        : message.session_id,
            byte_2            : 0,
            byte_3            : 0,
            presentation_type : PresentationType::SecsII as u8,
            session_type      : SessionType::SelectRequest as u8,
            system            : message.system,
          },
          text: vec![],
        }
      },
      HsmsMessage::SelectResponse(message) => {
        Message {
          header: MessageHeader {
            session_id        : message.session_id,
            byte_2            : 0,
            byte_3            : message.status,
            presentation_type : PresentationType::SecsII as u8,
            session_type      : SessionType::SelectResponse as u8,
            system            : message.system,
          },
          text: vec![],
        }
      },
      HsmsMessage::DeselectRequest(message) => {
        Message {
          header: MessageHeader {
            session_id        : message.session_id,
            byte_2            : 0,
            byte_3            : 0,
            presentation_type : PresentationType::SecsII as u8,
            session_type      : SessionType::DeselectRequest as u8,
            system            : message.system,
          },
          text: vec![],
        }
      },
      HsmsMessage::DeselectResponse(message) => {
        Message {
          header: MessageHeader {
            session_id        : message.session_id,
            byte_2            : 0,
            byte_3            : message.status,
            presentation_type : PresentationType::SecsII as u8,
            session_type      : SessionType::DeselectResponse as u8,
            system            : message.system,
          },
          text: vec![],
        }
      },
      HsmsMessage::LinktestRequest(message) => {
        Message {
          header: MessageHeader {
            session_id        : 0xFFFF,
            byte_2            : 0,
            byte_3            : 0,
            presentation_type : PresentationType::SecsII as u8,
            session_type      : SessionType::LinktestRequest as u8,
            system            : message.system,
          },
          text: vec![],
        }
      },
      HsmsMessage::LinktestResponse(message) => {
        Message {
          header: MessageHeader {
            session_id        : 0xFFFF,
            byte_2            : 0,
            byte_3            : 0,
            presentation_type : PresentationType::SecsII as u8,
            session_type      : SessionType::LinktestResponse as u8,
            system            : message.system,
          },
          text: vec![],
        }
      },
      HsmsMessage::RejectRequest(message) => {
        Message {
          header: MessageHeader {
            session_id        : message.session_id,
            byte_2            : message.message_type,
            byte_3            : message.reason_code,
            presentation_type : PresentationType::SecsII as u8,
            session_type      : SessionType::RejectRequest as u8,
            system            : message.system,
          },
          text: vec![],
        }
      },
      HsmsMessage::SeparateRequest(message) => {
        Message {
          header: MessageHeader {
            session_id        : message.session_id,
            byte_2            : 0,
            byte_3            : 0,
            presentation_type : PresentationType::SecsII as u8,
            session_type      : SessionType::SeparateRequest as u8,
            system            : message.system,
          },
          text: vec![],
        }
      },
    }
  }
}
impl TryFrom<Message> for HsmsMessage {
  type Error = RejectReason;

  /// ## HSMS MESSAGE FROM GENERIC MESSAGE
  /// 
  /// Due to the fact that valid HSMS Messages are a subset of valid Messages,
  /// this operation is fallable when the Message is not an HSMS message.
  fn try_from(message: Message) -> Result<Self, Self::Error> {
    if message.header.presentation_type != 0 {return Err(RejectReason::UnsupportedPresentationType)}
    match message.header.session_type {
      0 => {
        Ok(HsmsMessage::DataMessage(DataMessage {
          session_id : message.header.session_id,
          w          : message.header.byte_2 & 0b1000_0000 > 0,
          stream     : message.header.byte_2 & 0b0111_1111,
          function   : message.header.byte_3,
          system     : message.header.system,
          text       : message.text,
        }))
      },
      1 => {
        if message.header.byte_2 != 0 {return Err(RejectReason::MalformedData)}
        if message.header.byte_3 != 0 {return Err(RejectReason::MalformedData)}
        if !message.text.is_empty()   {return Err(RejectReason::MalformedData)}
        Ok(HsmsMessage::SelectRequest(SelectRequest {
          session_id : message.header.session_id,
          system     : message.header.system,
        }))
      },
      2 => {
        if message.header.byte_2 != 0 {return Err(RejectReason::MalformedData)}
        if !message.text.is_empty()   {return Err(RejectReason::MalformedData)}
        Ok(HsmsMessage::SelectResponse(SelectResponse {
          session_id : message.header.session_id,
          status     : message.header.byte_3,
          system     : message.header.system,
        }))
      },
      3 => {
        if message.header.byte_2 != 0 {return Err(RejectReason::MalformedData)}
        if message.header.byte_3 != 0 {return Err(RejectReason::MalformedData)}
        if !message.text.is_empty()   {return Err(RejectReason::MalformedData)}
        Ok(HsmsMessage::DeselectRequest(DeselectRequest {
          session_id : message.header.session_id,
          system     : message.header.system,
        }))
      },
      4 => {
        if message.header.byte_2 != 0 {return Err(RejectReason::MalformedData)}
        if !message.text.is_empty()   {return Err(RejectReason::MalformedData)}
        Ok(HsmsMessage::DeselectResponse(DeselectResponse {
          session_id : message.header.session_id,
          status     : message.header.byte_3,
          system     : message.header.system,
        }))
      },
      5 => {
        if message.header.session_id != 0xFFFF {return Err(RejectReason::MalformedData)}
        if message.header.byte_2     != 0      {return Err(RejectReason::MalformedData)}
        if message.header.byte_3     != 0      {return Err(RejectReason::MalformedData)}
        if !message.text.is_empty()            {return Err(RejectReason::MalformedData)}
        Ok(HsmsMessage::LinktestRequest(LinktestRequest {
          system : message.header.system,
        }))
      },
      6 => {
        if message.header.session_id != 0xFFFF {return Err(RejectReason::MalformedData)}
        if message.header.byte_2     != 0      {return Err(RejectReason::MalformedData)}
        if message.header.byte_3     != 0      {return Err(RejectReason::MalformedData)}
        if !message.text.is_empty()            {return Err(RejectReason::MalformedData)}
        Ok(HsmsMessage::LinktestResponse(LinktestResponse {
          system : message.header.system,
        }))
      },
      7 => {
        if !message.text.is_empty() {return Err(RejectReason::MalformedData)}
        Ok(HsmsMessage::RejectRequest(RejectRequest {
          session_id   : message.header.session_id,
          message_type : message.header.byte_2,
          reason_code  : message.header.byte_3,
          system       : message.header.system,
        }))
      },
      9 => {
        if message.header.byte_2 != 0 {return Err(RejectReason::MalformedData)}
        if message.header.byte_3 != 0 {return Err(RejectReason::MalformedData)}
        if !message.text.is_empty()   {return Err(RejectReason::MalformedData)}
        Ok(HsmsMessage::SeparateRequest(SeparateRequest {
          session_id : message.header.session_id,
          system     : message.header.system,
        }))
      },
      _ => Err(RejectReason::UnsupportedSessionType)
    }
  }
}
impl HsmsMessage {
  pub fn session_type(&self) -> SessionType {
    match self {
      HsmsMessage::DataMessage      (_) => SessionType::DataMessage     ,
      HsmsMessage::SelectRequest    (_) => SessionType::SelectRequest   ,
      HsmsMessage::SelectResponse   (_) => SessionType::SelectResponse  ,
      HsmsMessage::DeselectRequest  (_) => SessionType::DeselectRequest ,
      HsmsMessage::DeselectResponse (_) => SessionType::DeselectResponse,
      HsmsMessage::LinktestRequest  (_) => SessionType::LinktestRequest ,
      HsmsMessage::LinktestResponse (_) => SessionType::LinktestResponse,
      HsmsMessage::RejectRequest    (_) => SessionType::RejectRequest   ,
      HsmsMessage::SeparateRequest  (_) => SessionType::SeparateRequest ,
    }
  }

  pub fn system_bytes(&self) -> u32 {
    match self {
      HsmsMessage::DataMessage      (data_message)      => data_message.system,
      HsmsMessage::SelectRequest    (select_request)    => select_request.system,
      HsmsMessage::SelectResponse   (select_response)   => select_response.system,
      HsmsMessage::DeselectRequest  (deselect_request)  => deselect_request.system,
      HsmsMessage::DeselectResponse (deselect_response) => deselect_response.system,
      HsmsMessage::LinktestRequest  (linktest_request)  => linktest_request.system,
      HsmsMessage::LinktestResponse (linktest_response) => linktest_response.system,
      HsmsMessage::RejectRequest    (reject_request)    => reject_request.system,
      HsmsMessage::SeparateRequest  (separate_request)  => separate_request.system,
    }
  }
}

/// ## DATA MESSAGE
/// **Based on SEMI E37-1109§8.3.1-8.3.3**
/// 
/// An [HSMS Message] with a [Session Type] of 0, used by the initiator of or
/// responding entity in the [Data Procedure] to send data.
/// 
/// [HSMS Message]:   HsmsMessage
/// [Session Type]:   SessionType
/// [Data Procedure]: GenericClient::data
#[derive(Clone, Debug)]
pub struct DataMessage {
  /// ### SESSION ID
  /// **Based on SEMI E37-1109§8.3.3.1**
  /// 
  /// The specific value is subject to Subsidiary Standards.
  pub session_id : u16,

  /// ### W-BIT
  /// **Based on SEMI E37-1109§8.3.3.3**
  /// 
  /// In a Primary Data Message, the W-Bit indicates whether a
  /// Reply Data Message is expected.
  pub w : bool,

  /// ### STREAM
  /// **Based on SEMI E37-1109§8.3.3.3**
  /// 
  /// Identifies the major topic of the message according to SECS-II.
  pub stream : u8,

  /// ### FUNCTION
  /// **Based on SEMI E37-1109§8.3.3.4**
  /// 
  /// Identifies the minor topic of the message according to SECS-II.
  pub function : u8,

  /// ### SYSTEM BYTES
  /// **Based on SEMI E37-1109§8.3.3.6**
  /// 
  /// Identifies a transaction uniquely among the set of open transactions.
  pub system : u32,

  /// ### MESSAGE TEXT
  /// **Based on SEMI E37-1109§8.3.3.7**
  /// 
  /// Contains the Text of the Data Message.
  pub text : Vec<u8>,
}

/// ## SELECT REQUEST
/// **Based on SEMI E37-1109§8.3.4**
/// 
/// An [HSMS Message] with a [Session Type] of 1, used by the initiator of the
/// [Select Procedure] for establishing communications.
/// 
/// [HSMS Message]:     HsmsMessage
/// [Session Type]:     SessionType
/// [Select Procedure]: GenericClient::select
#[derive(Clone, Copy, Debug)]
pub struct SelectRequest {
  /// ### SESSION ID
  /// **Based on SEMI E37-1109§8.3.4.3**
  /// 
  /// The specific value is subject to Subsidiary Standards.
  pub session_id: u16,

  /// ### SYSTEM BYTES
  /// **Based on SEMI E37-1109§8.3.4.4**
  /// 
  /// Identifies a transaction uniquely among the set of open transactions.
  pub system: u32,
}

/// ## SELECT RESPONSE
/// **Based on SEMI E37-1109§8.3.5-8.3.7**
/// 
/// An [HSMS Message] with a [Session Type] of 2, used by the responding entity
/// in the [Select Procedure].
/// 
/// [HSMS Message]:     HsmsMessage
/// [Session Type]:     SessionType
/// [Select Procedure]: GenericClient::select
#[derive(Clone, Copy, Debug)]
pub struct SelectResponse {
  /// ### SESSION ID
  /// **Based on SEMI E37-1109§8.3.7.1**
  /// 
  /// Must equal the Session ID of the corresponding Select Request.
  pub session_id: u16,

  /// ### SELECT STATUS
  /// **Based on SEMI E37-1109§8.3.7.2**
  /// 
  /// Indicates the success or failure mode of the Select Procedure.
  pub status: u8,

  /// ### SYSTEM BYTES
  /// **Based on SEMI E37-1109§8.3.3.6**
  /// 
  /// Must equal the System Bytes of the corresponding Select Request.
  pub system: u32,
}

/// ## SELECT STATUS
/// **Based on SEMI E37-1109§8.3.7.2**
/// 
/// [Byte 3] of a [Deselect.rsp] message, used as the indication of success or
/// reason for failure of the [Select Procedure].
/// 
/// Values 4-127 are reserved for Subsidiary Standards.
/// 
/// Values 128-255 are reserved for the Local Entity.
/// 
/// [Select Procedure]: GenericClient::select
/// [Byte 3]:           MessageHeader::byte_3
/// [Deselect.rsp]:     DeselectResponse
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SelectStatus {
  Success       = 0,
  AlreadyActive = 1,
  NotReady      = 2,
  Exhausted     = 3,
}

/// ## DESELECT REQUEST
/// **Based on SEMI E37-1109§8.3.8-8.3.10**
/// 
/// An [HSMS Message] with a [Session Type] of 3, used by the initiator of the
/// [Deselect Procedure] for breaking communications.
/// 
/// [HSMS Message]:       HsmsMessage
/// [Session Type]:       SessionType
/// [Deselect Procedure]: GenericClient::deselect
#[derive(Clone, Copy, Debug)]
pub struct DeselectRequest {
  /// ### SESSION ID
  /// **Based on SEMI E37-1109§8.3.10.1**
  /// 
  /// Must equal a previous Session ID used in a Select Procedure.
  pub session_id: u16,

  /// ### SYSTEM BYTES
  /// **Based on SEMI E37-1109§8.3.10.2**
  /// 
  /// Identifies a transaction uniquely among the set of open transactions.
  pub system: u32,
}

/// ## DESELECT RESPONSE
/// **Based on SEMI E37-1109§8.3.11-8.3.13**
/// 
/// An [HSMS Message] with a [Session Type] of 4, used by the responding entity
/// in the [Deselect Procedure].
/// 
/// [HSMS Message]:       HsmsMessage
/// [Session Type]:       SessionType
/// [Deselect Procedure]: GenericClient::deselect
#[derive(Clone, Copy, Debug)]
pub struct DeselectResponse {
  pub session_id : u16, //Same as Request
  pub status     : u8,  //Deselect Status
  pub system     : u32, //Same as Request
}

/// ## DESELECT STATUS
/// **Based on SEMI E37-1109§8.3.13.2**
/// 
/// [Byte 3] of a [Deselect.rsp] message, used as the indication of success or
/// reason for failure of the [Deselect Procedure].
/// 
/// Values 3-127 are reserved for Subsidiary Standards.
/// 
/// Values 128-255 are reserved for the Local Entity.
/// 
/// [Deselect Procedure]: GenericClient::deselect
/// [Byte 3]:             MessageHeader::byte_3
/// [Deselect.rsp]:       DeselectResponse
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeselectStatus {
  Success        = 0,
  NotEstablished = 1,
  Busy           = 2,
}

/// ## LINKTEST REQUEST
/// **Based on SEMI E37-1109§8.3.14-8.3.16**
/// 
/// An [HSMS Message] with a [Session Type] of 5, used by the initiator of the
/// [Linktest Procedure] for checking communications stability.
/// 
/// [HSMS Message]:       HsmsMessage
/// [Session Type]:       SessionType
/// [Linktest Procedure]: GenericClient::linktest
#[derive(Clone, Copy, Debug)]
pub struct LinktestRequest {
  pub system : u32, //Unique
}

/// ## LINKTEST RESPONSE
/// **Based on SEMI E37-1109§8.3.17-8.3.19**
/// 
/// An [HSMS Message] with a [Session Type] of 6, used by the responding entity
/// in the [Linktest Procedure].
/// 
/// [HSMS Message]:       HsmsMessage
/// [Session Type]:       SessionType
/// [Linktest Procedure]: GenericClient::linktest
#[derive(Clone, Copy, Debug)]
pub struct LinktestResponse {
  pub system : u32, //Same as Request
}

/// ## REJECT REQUEST
/// **Based on SEMI E37-1109§8.3.20-8.3.21**
/// 
/// An [HSMS Message] with a [Session Type] of 7, used by the responding entity
/// in the [Reject Procedure].
/// 
/// [HSMS Message]:     HsmsMessage
/// [Session Type]:     SessionType
/// [Reject Procedure]: GenericClient::reject
#[derive(Clone, Copy, Debug)]
pub struct RejectRequest {
  pub session_id   : u16, //Same as Rejected Message
  pub message_type : u8,  //PType or SType of Rejected Message
  pub reason_code  : u8,  //Reason Code
  pub system       : u32, //Same as Rejected Message
}

/// ## REJECT REASON
/// **Based on SEMI E37-1109§8.3.21.3**
/// 
/// [Byte 3] of a [Reject.req] message, specifying the reason a message has
/// been rejected in the [Reject Procedure].
/// 
/// Values 4-127 are reserved for Subsidiary Standards.
/// 
/// Values 0, and 128-255 are reserved for the Local Entity.
/// 
/// [Reject Procedure]: GenericClient::reject
/// [Byte 3]:           MessageHeader::byte_3
/// [Reject.req]:       RejectRequest
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

/// ## SEPARATE REQUEST
/// **Based on SEMI E37-1109§8.3.22**
/// 
/// An [HSMS Message] with a [Session Type] of 9, used by the initiator of the
/// [Separate Procedure] for breaking communications.
/// 
/// [HSMS Message]:       HsmsMessage
/// [Session Type]:       SessionType
/// [Separate Procedure]: GenericClient::separate
#[derive(Clone, Copy, Debug)]
pub struct SeparateRequest {
  pub session_id : u16, //Further Defined
  pub system     : u32, //Unique
}

// SECTION 9: SPECIAL CONSIDERATIONS

// SECTION 10: HSMS DOCUMENTATION

/// ## PARAMETER SETTINGS
/// **Based on SEMI E37-1109§10.2**
/// 
/// The required set of paramters which an [HSMS] implementation must provide.
/// 
/// [HSMS]: crate
#[derive(Clone, Copy, Debug)]
pub struct ParameterSettings {
  /// ### CONNECT MODE
  /// 
  /// Specifies the [Connection Mode] the [Client] will use when performing the
  /// [Connect Procedure], [PASSIVE] to wait for an incoming connection, or
  /// [ACTIVE] to initiate an outgoing connection.
  /// 
  /// [Connection Mode]:   ConnectionMode
  /// [PASSIVE]:           ConnectionMode::Passive
  /// [ACTIVE]:            ConnectionMode::Active
  /// [Client]:            GenericClient
  /// [Connect Procedure]: GenericClient::connect
  pub connect_mode: ConnectionMode,

  /// ### T3: REPLY TIMEOUT
  /// 
  /// The maximum amount of time that the [Client] will wait after sending a
  /// Primary [Data Message] to receive the appropriate Response [Data Message]
  /// before it must initiate the [Disconnect Procedure].
  /// 
  /// [Client]:               GenericClient
  /// [Data Message]:         DataMessage
  /// [Disconnect Procedure]: GenericClient::disconnect
  pub t3: Duration,

  /// ### T5: CONNECTION SEPARATION TIMEOUT
  /// 
  /// The minimum amount of time that the [Client] must wait between successive
  /// attempts to initiate the [Connect Procedure] with a [Connect Mode] of
  /// [ACTIVE].
  /// 
  /// [Client]:            GenericClient
  /// [Connect Procedure]: GenericClient::connect
  /// [Connect Mode]:      ParameterSettings::connect_mode
  /// [ACTIVE]:            ConnectionMode::Active
  pub t5: Duration,

  /// ### T6: CONTROL TRANSACTION TIMEOUT
  /// 
  /// The maximum amount of time that the [Client] will wait after sending a
  /// [Select Request], [Deselect Request], or [Linktest Request] to receive
  /// the appropriate [Select Response], [Deselect Response], or
  /// [Linktest Response] before it must initiate the [Disconnect Procedure].
  /// 
  /// [Client]:               GenericClient
  /// [Disconnect Procedure]: GenericClient::disconnect
  /// [Select Request]:       SelectRequest
  /// [Select Response]:      SelectResponse
  /// [Deselect Request]:     DeselectRequest
  /// [Deselect Response]:    DeselectResponse
  /// [Linktest Request]:     LinktestRequest
  /// [Linktest Response]:    LinktestResponse
  pub t6: Duration,

  /// ### T7: NOT SELECTED TIMEOUT
  /// 
  /// The maximum amount of time that the [Client] will wait after being placed
  /// in the [NOT SELECTED] state before it must initiate the
  /// [Disconnect Procedure].
  /// 
  /// [NOT SELECTED]:         SelectionState::NotSelected
  /// [Client]:               GenericClient
  /// [Disconnect Procedure]: GenericClient::disconnect
  pub t7: Duration,

  /// ### T8: NETWORK INTERCHARACTER TIMEOUT
  /// 
  /// The maximum amount of time that may elapse when the [Client] is sending
  /// or receiving data between successive characters in the same [Message]
  /// before it must initiate the [Disconnect Procedure].
  /// 
  /// [Client]:               GenericClient
  /// [Disconnect Procedure]: GenericClient::disconnect
  /// [Message]:              Message
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

/// # HSMS SINGLE SELECTED-SESSION MODE (HSMS-SS)
/// **Based on SEMI E37.1**
pub mod single_selected_session {
  pub struct SingleSessionClient {

  }
  impl SingleSessionClient {
    
  }
}
