//! # GENERIC SERVICES
//! 
//! Defines the full functionality of the [HSMS] protocol without modification
//! by any subsidiary standards. This involves the sending of messages of
//! particular types and at particular times as allowed by the protocol.
//! 
//! ---------------------------------------------------------------------------
//! 
//! To use the [Generic Services]:
//! 
//! - Build [Message]s which use a [Message ID] and [Message Contents]:
//!   - [Data Message]
//!   - [Select.req]
//!   - [Select.rsp]
//!   - [Deselect.req]
//!   - [Deselect.rsp]
//!   - [Linktest.req]
//!   - [Linktest.rsp]
//!   - [Reject.req]
//!   - [Separate.req]
//! - Create an [Client] by providing the [New Client] function with
//!   [Parameter Settings].
//! - Manage the [Connection State] with the [Connect Procedure] and
//!   [Disconnect Procedure].
//! - Manage the [Selection State] with the [Select Procedure],
//!   [Deselect Procedure], and [Separate Procedure].
//! - Receive [Data Message]s with the hook provided by the
//!   [Connect Procedure].
//! - Test connection integrity with the [Linktest Procedure].
//! - Send [Data Message]s with the [Data Procedure].
//! - Send [Reject.req] messages [Reject Procedure].
//! 
//! [HSMS]:                 crate
//! [Generic Services]:     crate::generic
//! [Client]:               Client
//! [New Client]:           Client::new
//! [Connect Procedure]:    Client::connect
//! [Disconnect Procedure]: Client::disconnect
//! [Select Procedure]:     Client::select
//! [Deselect Procedure]:   Client::deselect
//! [Separate Procedure]:   Client::separate
//! [Linktest Procedure]:   Client::linktest
//! [Data Procedure]:       Client::data
//! [Reject Procedure]:     Client::reject
//! [Message]:              Message
//! [Message ID]:           MessageID
//! [Message Contents]:     MessageContents
//! [Data Message]:         MessageContents::DataMessage
//! [Select.req]:           MessageContents::SelectRequest
//! [Select.rsp]:           MessageContents::SelectResponse
//! [Deselect.req]:         MessageContents::DeselectRequest
//! [Deselect.rsp]:         MessageContents::DeselectResponse
//! [Linktest.req]:         MessageContents::LinktestRequest
//! [Linktest.rsp]:         MessageContents::LinktestResponse
//! [Reject.req]:           MessageContents::RejectRequest
//! [Separate.req]:         MessageContents::SeparateRequest
//! [Connection State]:     crate::primitive::ConnectionState
//! [Selection State]:      SelectionState
//! [Parameter Settings]:   ParameterSettings

use std::{
  collections::HashMap,
  io::{
    Error,
    ErrorKind,
  },
  net::SocketAddr,
  ops::{
    Deref,
    DerefMut,
  },
  sync::{
    atomic::Ordering::Relaxed,
    Arc,
    Mutex,
    mpsc::{
      channel,
      Receiver,
      Sender,
    },
  },
  thread::{
    self,
    JoinHandle,
  },
  time::Duration,
};
use atomic::Atomic;
use bytemuck::NoUninit;
use oneshot::Sender as SendOnce;
use crate::{
  PresentationType,
  primitive,
};

pub use crate::primitive::ConnectionMode;

/// ## CLIENT
/// 
/// Encapsulates the full functionality of the [HSMS] protocol without
/// reference to any subsidiary standards, known as the [Generic Services].
/// 
/// [HSMS]:             crate
/// [Generic Services]: crate::generic
pub struct Client {
  parameter_settings: ParameterSettings,
  primitive_client: Arc<primitive::Client>,
  selection_state: Atomic<SelectionState>,
  selection_mutex: Mutex<()>,
  outbox: Mutex<HashMap<u32, (MessageID, SendOnce<Option<Message>>)>>,
  system: Mutex<u32>,
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
  /// 
  /// Creates a [Client] in the [NOT CONNECTED] state, ready to initiate the
  /// [Connect Procedure].
  /// 
  /// [Client]:            Client
  /// [Connect Procedure]: Client::connect
  /// [NOT CONNECTED]:     primitive::ConnectionState::NotConnected
  pub fn new(
    parameter_settings: ParameterSettings
  ) -> Arc<Self> {
    Arc::new(Client {
      parameter_settings,
      primitive_client: primitive::Client::new(),
      selection_state:  Default::default(),
      selection_mutex:  Default::default(),
      outbox:           Default::default(),
      system:           Default::default(),
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
  /// [Connection State]:  primitive::ConnectionState
  /// [NOT CONNECTED]:     primitive::ConnectionState::NotConnected
  /// [CONNECTED]:         primitive::ConnectionState::Connected
  /// [Connection Mode]:   primitive::ConnectionMode
  /// [PASSIVE]:           primitive::ConnectionMode::Passive
  /// [ACTIVE]:            primitive::ConnectionMode::Active
  /// [Client]:            Client
  /// [Connect Procedure]: Client::connect
  /// [T5]:                ParameterSettings::t5
  /// [T8]:                ParameterSettings::t8
  pub fn connect(
    self: &Arc<Self>,
    entity: &str,
  ) -> Result<(SocketAddr, Receiver<(MessageID, semi_e5::Message)>), Error> {
    // Connect Primitive Client
    let (socket, rx_receiver) = self.primitive_client.connect(entity, self.parameter_settings.connect_mode, self.parameter_settings.t5, self.parameter_settings.t8)?;
    // Create Channel
    let (data_sender, data_receiver) = channel::<(MessageID, semi_e5::Message)>();
    // Start RX Thread
    let clone: Arc<Client> = self.clone();
    thread::spawn(move || {clone.receive(rx_receiver, data_sender)});
    // Finish
    Ok((socket, data_receiver))
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
  /// [Connection State]:     primitive::ConnectionState
  /// [NOT CONNECTED]:        primitive::ConnectionState::NotConnected
  /// [CONNECTED]:            primitive::ConnectionState::Connected
  /// [Client]:               Client
  /// [Disconnect Procedure]: Client::disconnect
  pub fn disconnect(
    self: &Arc<Self>,
  ) -> Result<(), Error> {
    // TO: NOT CONNECTED
    let result: Result<(), Error> = self.primitive_client.disconnect();
    // TO: NOT SELECTED
    let _guard = self.selection_mutex.lock().unwrap();
    if let SelectionState::Selected = self.selection_state.load(Relaxed) {
      self.selection_state.store(SelectionState::NotSelected, Relaxed);
    }
    // Finish
    result
  }
}

/// ## MESSAGE EXCHANGE PROCEDURES
/// **Based on SEMI E37-1109§7**
/// 
/// Encapsulates the parts of the [Client]'s functionality dealing with
/// exchanging [Message]s.
/// 
/// - [Data Procedure] - [Data Message]s
/// - [Select Procedure] - [Select.req] and [Select.rsp]
/// - [Deselect Procedure] - [Deselect.req] and [Deselect.rsp]
/// - [Linktest Procedure] - [Linktest.req] and [Linktest.rsp]
/// - [Separate Procedure] - [Separate.req]
/// - [Reject Procedure] - [Reject.req]
/// 
/// [Message]:            Message
/// [Client]:             Client
/// [Select Procedure]:   Client::select
/// [Data Procedure]:     Client::data
/// [Deselect Procedure]: Client::deselect
/// [Linktest Procedure]: Client::linktest
/// [Separate Procedure]: Client::separate
/// [Reject Procedure]:   Client::reject
/// [Data Message]:       MessageContents::DataMessage
/// [Select.req]:         MessageContents::SelectRequest
/// [Select.rsp]:         MessageContents::SelectResponse
/// [Deselect.req]:       MessageContents::DeselectRequest
/// [Deselect.rsp]:       MessageContents::DeselectResponse
/// [Linktest.req]:       MessageContents::LinktestRequest
/// [Linktest.rsp]:       MessageContents::LinktestResponse
/// [Reject.req]:         MessageContents::RejectRequest
/// [Separate.req]:       MessageContents::SeparateRequest
impl Client {
  /// ### RECEIVE PROCEDURE
  /// 
  /// An [Client] in the [CONNECTED] state will automatically receive
  /// [Message]s and respond based on their [Message Contents] and the current
  /// [Selection State].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### [Data Message]
  /// 
  /// - [NOT SELECTED] - The [Client] will respond by transmitting a
  ///   [Reject.req] message, rejecting the [HSMS Data Procedure] and
  ///   completing the [HSMS Reject Procedure].
  /// - [SELECTED], Primary [Data Message] - The [Client] will send the
  ///   [Data Message] to the hook provided by the [Connect Procedure].
  /// - [SELECTED], Response [Data Message] - The [Client] will respond by
  ///   correllating the message to a previously sent Primary [Data Message],
  ///   finishing a previously initiated [Data Procedure] if successful,
  ///   or if unsuccessful by transmitting a [Reject.req] message, rejecting
  ///   the [Data Procedure] and completing the [Reject Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### [Select.req]:
  /// 
  /// - [NOT SELECTED] - The [Client] will respond with a [Select.rsp]
  ///   accepting and completing the [Select Procedure].
  /// - [SELECTED] - The [Client] will respond with a [Select.rsp] message
  ///   rejecting the [Select Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### [Select.rsp]:
  /// 
  /// - [NOT SELECTED] - The [Client] will complete the [Select Procedure].
  /// - [SELECTED] - The [Client] will respond with a [Reject.req] message,
  ///   completing the [Reject Procedure].
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
  /// - The [Client] will respond with a [Linktest.rsp], completing the
  ///   [Linktest Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### [Linktest.rsp]:
  /// 
  /// - The [Client] will respond by correllating the message to a previously
  ///   sent [Linktest.req] message, finishing a previously initiated
  ///   [Linktest Procedure] if successful, or if unsuccessful by transmitting
  ///   a [Reject.req] message, completing the [Reject Procedure].
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
  /// - [NOT SELECTED] - The [Client] will not do anything.
  /// - [SELECTED] - The [Client] will complete the [Separate Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### Unknown [Primitive Message]:
  /// 
  /// - The [Client] will respond by transmitting a [Reject.req] message,
  ///   completing the [Reject Procedure]. 
  /// 
  /// [Primitive Message]:  primitive::Message
  /// [Connection State]:   primitive::ConnectionState
  /// [NOT CONNECTED]:      primitive::ConnectionState::NotConnected
  /// [CONNECTED]:          primitive::ConnectionState::Connected
  /// [Message]:            Message
  /// [Message Contents]:   MessageContents
  /// [Data Message]:       MessageContents::DataMessage
  /// [Select.req]:         MessageContents::SelectRequest
  /// [Select.rsp]:         MessageContents::SelectResponse
  /// [Deselect.req]:       MessageContents::DeselectRequest
  /// [Deselect.rsp]:       MessageContents::DeselectResponse
  /// [Linktest.req]:       MessageContents::LinktestRequest
  /// [Linktest.rsp]:       MessageContents::LinktestResponse
  /// [Reject.req]:         MessageContents::RejectRequest
  /// [Separate.req]:       MessageContents::SeparateRequest
  /// [Client]:             Client
  /// [Connect Procedure]:  Client::connect
  /// [Select Procedure]:   Client::select
  /// [Data Procedure]:     Client::data
  /// [Deselect Procedure]: Client::deselect
  /// [Linktest Procedure]: Client::linktest
  /// [Separate Procedure]: Client::separate
  /// [Reject Procedure]:   Client::reject
  /// [Selection State]:    SelectionState
  /// [NOT SELECTED]:       SelectionState::NotSelected
  /// [SELECTED]:           SelectionState::Selected
  /// [SELECT INITIATED]:   SelectionState::SelectInitiated
  /// [DESELECT INITIATED]: SelectionState::DeselectInitiated
  fn receive(
    self: &Arc<Self>,
    rx_receiver: Receiver<primitive::Message>,
    rx_sender: Sender<(MessageID, semi_e5::Message)>,
  ) {
    for primitive_message in rx_receiver {
      let primitive_header = primitive_message.header;
      match Message::try_from(primitive_message) {
        Ok(rx_message) => match rx_message.contents {
          // RX: Data Message
          MessageContents::DataMessage(data) => {
            match self.selection_state.load(Relaxed) {
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
                    sender.send(Some(Message{
                      id: rx_message.id,
                      contents: MessageContents::DataMessage(data),
                    })).unwrap();
                  }
                  // OUTBOX: Transaction Not Found
                  else {
                    // TX: Reject.req 
                    if self.primitive_client.transmit(Message {
                      id: rx_message.id,
                      contents: MessageContents::RejectRequest(0, RejectReason::TransactionNotOpen as u8)
                    }.into()).is_err() {break}
                  }
                }
              },
              // IS: NOT SELECTED
              _ => {
                // TX: Reject.req
                if self.primitive_client.transmit(Message {
                  id: rx_message.id,
                  contents: MessageContents::RejectRequest(0, RejectReason::EntityNotSelected as u8)
                }.into()).is_err() {break}
              },
            }
          },
          // RX: Select.req
          MessageContents::SelectRequest => {
            match self.selection_mutex.try_lock() {
              Ok(_guard) => {
                match self.selection_state.load(Relaxed) {
                  // IS: NOT SELECTED
                  SelectionState::NotSelected => {
                    // TX: Select.rsp Success
                    if self.primitive_client.transmit(Message {
                      id: rx_message.id,
                      contents: MessageContents::SelectResponse(SelectStatus::Success as u8),
                    }.into()).is_err() {break};
                    // TO: SELECTED
                    self.selection_state.store(SelectionState::Selected, Relaxed);
                  },
                  // IS: SELECTED
                  SelectionState::Selected => {
                    // TX: Select.rsp Already Active
                    if self.primitive_client.transmit(Message {
                      id: rx_message.id,
                      contents: MessageContents::SelectResponse(SelectStatus::AlreadyActive as u8),
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
          MessageContents::SelectResponse(select_status) => {
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
              sender.send(Some(Message{
                id: rx_message.id,
                contents: MessageContents::SelectResponse(select_status),
              })).unwrap();
            }
            // OUTBOX: Transaction Not Found
            else {
              // TX: Reject.req
              if self.primitive_client.transmit(Message {
                id: rx_message.id,
                contents: MessageContents::RejectRequest(0, RejectReason::TransactionNotOpen as u8)
              }.into()).is_err() {break}
            }
          },
          // RX: Deselect.req
          MessageContents::DeselectRequest => {
            todo!()
          },
          // RX: Deselect.rsp
          MessageContents::DeselectResponse(_deselect_status) => {
            todo!()
          },
          // RX: Linktest.req
          MessageContents::LinktestRequest => {
            // TX: Linktest.rsp
            if self.primitive_client.transmit(Message{
              id: rx_message.id,
              contents: MessageContents::LinktestResponse,
            }.into()).is_err() {break};
          },
          // RX: Linktest.rsp
          MessageContents::LinktestResponse => {
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
              if self.primitive_client.transmit(Message {
                id: rx_message.id,
                contents: MessageContents::RejectRequest(SessionType::LinktestRequest as u8, RejectReason::TransactionNotOpen as u8),
              }.into()).is_err() {break}
            }
          },
          // RX: Reject.req
          MessageContents::RejectRequest(_message_type, _reason_code) => {
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
          MessageContents::SeparateRequest => {
            let _guard: std::sync::MutexGuard<'_, ()> = self.selection_mutex.lock().unwrap();
            if let SelectionState::Selected = self.selection_state.load(Relaxed) {
              self.selection_state.store(SelectionState::NotSelected, Relaxed);
            }
          },
        },
        Err(reject_reason) => {
          // TX: Reject.req
          if self.primitive_client.transmit(Message {
            id: MessageID {
              session: primitive_header.session_id,
              system: primitive_header.system,
            },
            contents: MessageContents::RejectRequest(match reject_reason {
              RejectReason::UnsupportedPresentationType => primitive_header.presentation_type,
              _ => primitive_header.session_type,
            }, reject_reason as u8),
          }.into()).is_err() {break}
        },
      }
    }
    // OUTBOX: CLEAR
    for (_, (_, sender)) in self.outbox.lock().unwrap().deref_mut().drain() {
      let _ = sender.send(None);
    }
  }

  /// ### TRANSMIT PROCEDURE
  /// **Based on SEMI E37-1109§7.2**
  /// 
  /// Serializes a [Message] and transmits it over the TCP/IP connection.
  /// If a reply is expected, this function will then wait up to the time
  /// specified for the requisite response [Message] to be recieved.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state to use this
  /// procedure.
  /// 
  /// [Message]:          Message
  /// [Connection State]: primitive::ConnectionState
  /// [NOT CONNECTED]:    primitive::ConnectionState::NotConnected
  /// [CONNECTED]:        primitive::ConnectionState::Connected
  fn transmit(
    self: &Arc<Self>,
    message: Message,
    reply_expected: bool,
    delay: Duration,
  ) -> Result<Option<Message>, Error> {
    let (receiver, system) = {
      // OUTBOX: LOCK
      let outbox_lock = if reply_expected {Some(self.deref().outbox.lock().unwrap())} else {None};
      // TX
      let message_id = message.id;
      match self.primitive_client.transmit(message.into()) {
        // TX: Success
        Ok(()) => {
          match outbox_lock {
            // REPLY NOT EXPECTED: Finish
            None => return Ok(None),
            // REPLY EXPECTED
            Some(mut outbox) => {
              // OUTBOX: Create Transaction
              let (sender, receiver) = oneshot::channel::<Option<Message>>();
              let system = {
                let mut system_guard = self.deref().system.lock().unwrap();
                let system_counter = system_guard.deref_mut();
                let system = *system_counter;
                *system_counter += 1;
                system
              };
              outbox.deref_mut().insert(system, (message_id, sender));
              (receiver, system)
            }
          }
        },
        // TX: Failure
        Err(error) => {
          // TO: NOT CONNECTED, NOT SELECTED
          let _ = self.disconnect();
          return Err(error)
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

  /// ### DATA PROCEDURE
  /// **Based on SEMI E37-1109§7.5-7.6**
  /// 
  /// Asks the [Client] to initiate the [Data Procedure] by transmitting a
  /// [Data Message] and waiting for the corresponding response to be received
  /// if it is necessary to do so.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state and the
  /// [Selection State] must be in the [SELECTED] state to use this procedure.
  /// 
  /// When a Response [Data Message] is necessary, the [Client] will wait
  /// to receive it for the amount of time specified by [T3] before it will
  /// consider it a communications failure and initiate the
  /// [Disconnect Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Although not done within this function, a [Client] in the [CONNECTED]
  /// state will automatically respond to having received a [Data Message]
  /// based on its contents and the current [Selection State]:
  /// - [NOT SELECTED] - The [Client] will respond by transmitting a
  ///   [Reject.req] message, rejecting the [HSMS Data Procedure] and
  ///   completing the [HSMS Reject Procedure].
  /// - [SELECTED], Primary [Data Message] - The [Client] will send the
  ///   [Data Message] to the hook provided by the [Connect Procedure].
  /// - [SELECTED], Response [Data Message] - The [Client] will respond by
  ///   correllating the message to a previously sent Primary [Data Message],
  ///   finishing a previously initiated [Data Procedure] if successful,
  ///   or if unsuccessful by transmitting a [Reject.req] message, rejecting
  ///   the [Data Procedure] and completing the [Reject Procedure].
  /// 
  /// [Connection State]:     primitive::ConnectionState
  /// [CONNECTED]:            primitive::ConnectionState::Connected
  /// [Selection State]:      SelectionState
  /// [NOT SELECTED]:         SelectionState::NotSelected
  /// [SELECTED]:             SelectionState::Selected
  /// [T3]:                   ParameterSettings::t3
  /// [Client]:               Client
  /// [Connect Procedure]:    Client::connect
  /// [Disconnect Procedure]: Client::disconnect
  /// [Data Procedure]:       Client::data
  /// [Reject Procedure]:     Client::reject
  /// [Data Message]:         MessageContents::DataMessage
  /// [Reject.req]:           MessageContents::RejectRequest
  pub fn data(
    self: &Arc<Self>,
    id: MessageID,
    message: semi_e5::Message,
  ) -> JoinHandle<Result<Option<semi_e5::Message>, Error>> {
    let clone: Arc<Client> = self.clone();
    let reply_expected: bool = message.function % 2 == 1 && message.w;
    thread::spawn(move || {
      match clone.selection_state.load(Relaxed) {
        // IS: NOT SELECTED
        SelectionState::NotSelected => return Err(Error::from(ErrorKind::AlreadyExists)),
        // IS: SELECTED
        SelectionState::Selected => {
          // TX: Data Message
          match clone.transmit(
            Message {
              id,
              contents: MessageContents::DataMessage(message),
            },
            reply_expected,
            clone.parameter_settings.t3,
          )?{
            // RX: Response
            Some(rx_message) => {
              match rx_message.contents {
                // RX: Data
                MessageContents::DataMessage(data_message) => return Ok(Some(data_message)),
                // RX: Reject.req
                MessageContents::RejectRequest(_type, _reason) => return Err(Error::from(ErrorKind::PermissionDenied)),
                // RX: Unknown
                _ => return Err(Error::from(ErrorKind::InvalidData)),
              }
            },
            // RX: No Response
            None => {
              // REPLY EXPECTED
              if reply_expected {
                // TO: NOT CONNECTED
                clone.disconnect()?;
                Err(Error::from(ErrorKind::ConnectionAborted))
                // TODO: HSMS-SS does NOT disconnect when the Data Procedure fails, may require this behavior to be optional.
              }
              // REPLY NOT EXPECTED
              else {
                return Ok(None);
              }
            },
          }
        },
      }
    })
  }

  /// ### SELECT PROCEDURE
  /// **Based on SEMI E37-1109§7.3-7.4**
  /// 
  /// Asks the [Client] to initiate the [Select Procedure] by transmitting a
  /// [Select.req] message and waiting for the corresponding [Select.rsp]
  /// message to be received.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state and the
  /// [Selection State] must be in the [NOT SELECTED] state to use this
  /// procedure.
  /// 
  /// The [Client] will wait to receive the [Select.rsp] for the amount
  /// of time specified by [T6] before it will consider it a communications
  /// failure and initiate the [Disconnect Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Although not done within this function, a [Client] in the [CONNECTED]
  /// state will automatically respond to having received a [Select.req]
  /// message based on its current [Selection State]:
  /// - [NOT SELECTED] - The [Client] will respond with a [Select.rsp]
  ///   accepting and completing the [Select Procedure].
  /// - [SELECTED] - The [Client] will respond with a [Select.rsp] message
  ///   rejecting the [Select Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Upon completion of the [Select Procedure], the [SELECTED] state
  /// is entered.
  /// 
  /// [Connection State]:     primitive::ConnectionState
  /// [CONNECTED]:            primitive::ConnectionState::Connected
  /// [Selection State]:      SelectionState
  /// [NOT SELECTED]:         SelectionState::NotSelected
  /// [SELECTED]:             SelectionState::Selected
  /// [T6]:                   ParameterSettings::t6
  /// [Client]:               Client
  /// [Disconnect Procedure]: Client::disconnect
  /// [Select Procedure]:     Client::select
  /// [Select.req]:           MessageContents::SelectRequest
  /// [Select.rsp]:           MessageContents::SelectResponse
  pub fn select(
    self: &Arc<Self>,
    id: MessageID,
  ) -> JoinHandle<Result<(), Error>> {
    let clone: Arc<Client> = self.clone();
    thread::spawn(move || {
      'disconnect: {
        let _guard = clone.selection_mutex.lock();
        match clone.selection_state.load(Relaxed) {
          SelectionState::NotSelected => {
            // TX: Select.req
            match clone.transmit(
              Message {
                id,
                contents: MessageContents::SelectRequest,
              },
              true,
              clone.parameter_settings.t6,
            )?{
              // RX: Response
              Some(rx_message) => {
                match rx_message.contents {
                  // RX: Select.rsp
                  MessageContents::SelectResponse(select_status) => {
                    // RX: Select.rsp Success
                    if select_status == SelectStatus::Success as u8 {
                      // TO: SELECTED
                      clone.selection_state.store(SelectionState::Selected, Relaxed);
                      return Ok(())
                    }
                    // RX: Select.rsp Failure
                    else {
                      return Err(Error::from(ErrorKind::PermissionDenied))
                    }
                  },
                  // RX: Reject.req
                  MessageContents::RejectRequest(_type, _reason) => return Err(Error::from(ErrorKind::PermissionDenied)),
                  // RX: Unknown
                  _ => return Err(Error::from(ErrorKind::InvalidData)),
                }
              },
              // RX: No Response
              None => {
                // TO: NOT CONNECTED, NOT SELECTED
                break 'disconnect;
              },
            }
          },
          SelectionState::Selected => {
            return Err(Error::from(ErrorKind::AlreadyExists))
          },
        }
      }
      clone.disconnect()?;
      Err(Error::from(ErrorKind::ConnectionAborted))
    })
  }

  /// ### DESELECT PROCEDURE (TODO)
  /// **Based on SEMI E37-1109§7.7**
  /// 
  /// Asks the [Client] to initiate the [Deselect Procedure] by transmitting a
  /// [Deselect.req] message and waiting for the corresponding [Deselect.rsp]
  /// message to be received.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state and the
  /// [Selection State] must be in the [SELECTED] state to use this procedure.
  /// 
  /// The [Client] will wait to receive the [Deselect.rsp] for the amount of
  /// time specified by [T6] before it will consider it a communications
  /// failure and initiate the [Disconnect Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Although not done within this function, a [Client] in the [CONNECTED]
  /// state will automatically respond to having received a [Deselect.req]
  /// message based on its current [Selection State]:
  /// - [NOT SELECTED] - The [Client] will respond with a [Deselect.rsp]
  ///   rejecting the [Deselect Procedure].
  /// - [SELECTED] - The [Client] will respond with a [Deselect.rsp] accepting
  ///   and completing the [Deselect Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Upon completion of the [Deselect Procedure], the [NOT SELECTED] state is
  /// entered.
  /// 
  /// [Connection State]:     primitive::ConnectionState
  /// [CONNECTED]:            primitive::ConnectionState::Connected
  /// [Selection State]:      SelectionState
  /// [NOT SELECTED]:         SelectionState::NotSelected
  /// [SELECTED]:             SelectionState::Selected
  /// [T6]:                   ParameterSettings::t6
  /// [Client]:               Client
  /// [Disconnect Procedure]: Client::disconnect
  /// [Deselect Procedure]:   Client::deselect
  /// [Deselect.req]:         MessageContents::DeselectRequest
  /// [Deselect.rsp]:         MessageContents::DeselectResponse
  pub fn deselect(
    self: &Arc<Self>,
  ) -> Result<(), Error> {
    todo!()
  }

  /// ### LINKTEST PROCEDURE
  /// **Based on SEMI E37-1109§7.8**
  /// 
  /// Asks the [Client] to initiate the [Linktest Procedure] by transmitting a
  /// [Linktest.req] message and waiting for the corresponding [Linktest.rsp]
  /// message to be received.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state to use this
  /// procedure.
  /// 
  /// The [Client] will wait to receive the [Linktest.rsp] for the amount of
  /// time specified by [T6] before it will consider it a communications
  /// failure and initiate the [Disconnect Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Although not done within this function, a [Client] in the
  /// [CONNECTED] state will automatically respond to having received a
  /// [Linktest.req] message:
  /// - The [Client] will respond with a [Linktest.rsp], completing the
  ///   [Linktest Procedure].
  /// 
  /// [Connection State]:     primitive::ConnectionState
  /// [CONNECTED]:            primitive::ConnectionState::Connected
  /// [Selection State]:      SelectionState
  /// [NOT SELECTED]:         SelectionState::NotSelected
  /// [SELECTED]:             SelectionState::Selected
  /// [T6]:                   ParameterSettings::t6
  /// [Client]:               Client
  /// [Disconnect Procedure]: Client::disconnect
  /// [Linktest Procedure]:   Client::linktest
  /// [Linktest.req]:         MessageContents::LinktestRequest
  /// [Linktest.rsp]:         MessageContents::LinktestResponse
  pub fn linktest(
    self: &Arc<Self>,
    system: u32,
  ) -> JoinHandle<Result<(), Error>> {
    let clone: Arc<Client> = self.clone();
    thread::spawn(move || {
      // TX: Linktest.req
      match clone.transmit(
        Message {
          id: MessageID {
            session: 0xFFFF,
            system,
          },
          contents: MessageContents::LinktestRequest,
        },
        true,
        clone.parameter_settings.t6,
      )?{
        // RX: Response
        Some(rx_message) => {
          match rx_message.contents {
            // RX: Linktest.rsp
            MessageContents::LinktestResponse => Ok(()),
            // RX: Reject.req
            MessageContents::RejectRequest(_type, _reason) => Err(Error::from(ErrorKind::PermissionDenied)),
            // RX: Unknown
            _ => Err(Error::from(ErrorKind::InvalidData)),
          }
        },
        // RX: No Response
        None => {
          // TO: NOT CONNECTED, NOT SELECTED
          clone.disconnect()?;
          Err(Error::from(ErrorKind::ConnectionAborted))
        },
      }
    })
  }

  /// ### SEPARATE PROCEDURE
  /// **Based on SEMI E37-1109§7.9**
  /// 
  /// Asks the [Client] to initiate the [Separate Procedure] by transmitting a
  /// [Separate.req] message.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state and the
  /// [Selection State] must be in the [SELECTED] state to use this procedure.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Although not done within this function, a [Client] in the [CONNECTED]
  /// state will automatically respond to having received a [Separate.req]
  /// message based on its current [Selection State]:
  /// - [NOT SELECTED] - The [Client] will not do anything.
  /// - [SELECTED] - The [Client] will complete the [Separate Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Upon completion of the [Separate Procedure], the [NOT SELECTED] state is
  /// entered.
  /// 
  /// [Connection State]:   primitive::ConnectionState
  /// [CONNECTED]:          primitive::ConnectionState::Connected
  /// [Selection State]:    SelectionState
  /// [NOT SELECTED]:       SelectionState::NotSelected
  /// [SELECTED]:           SelectionState::Selected
  /// [Client]:             Client
  /// [Separate Procedure]: Client::separate
  /// [Separate.req]:       MessageContents::SeparateRequest
  pub fn separate(
    self: &Arc<Self>,
    id: MessageID,
  ) -> JoinHandle<Result<(), Error>> {
    let clone: Arc<Client> = self.clone();
    thread::spawn(move || {
      let _guard = clone.selection_mutex.lock().unwrap();
      match clone.selection_state.load(Relaxed) {
        // IS: NOT SELECTED
        SelectionState::NotSelected => {
          Err(Error::from(ErrorKind::PermissionDenied))
        },
        // IS: SELECTED
        SelectionState::Selected => {
          // TX: Separate.req
          clone.transmit(
            Message {
              id,
              contents: MessageContents::SeparateRequest,
            },
            false,
            clone.parameter_settings.t6,
          )?;
          // TO: NOT SELECTED
          clone.selection_state.store(SelectionState::NotSelected, Relaxed);
          Ok(())
        },
      }
    })
  }

  /// ### REJECT PROCEDURE (TODO)
  /// **Based on SEMI E37-1109§7.10**
  /// 
  /// Asks the [Client] to initiate the [Reject Procedure] by transmitting a
  /// [Reject.req] message.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state to use this
  /// procedure.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Although not done within this function, a [Client] in the [CONNECTED]
  /// state will automatically respond to having received a [Reject.req]:
  /// - Not yet implemented.
  /// 
  /// [Connection State]: primitive::ConnectionState
  /// [CONNECTED]:        primitive::ConnectionState::Connected
  /// [Selection State]:  SelectionState
  /// [NOT SELECTED]:     SelectionState::NotSelected
  /// [SELECTED]:         SelectionState::Selected
  /// [Client]:           Client
  /// [Reject Procedure]: Client::reject
  /// [Reject.req]:       MessageContents::RejectRequest
  pub fn reject(
    self: &Arc<Self>,
    _reason: RejectReason,
  ) -> Result<(), Error> {
    todo!()
  }
}

/// ## SELECTION STATE
/// **Based on SEMI E37-1109§5.5.2**
/// 
/// The [CONNECTED] state has two substates, [NOT SELECTED] and [SELECTED].
/// 
/// The [Client] moves between them based on whether it has established
/// a session with another entity according to the [Select Procedure],
/// [Deselect Procedure], and [Separate Procedure].
/// 
/// [CONNECTED]:          primitive::ConnectionState::Connected
/// [NOT SELECTED]:       SelectionState::NotSelected
/// [SELECTED]:           SelectionState::Selected
/// [Client]:             Client
/// [Select Procedure]:   Client::select
/// [Deselect Procedure]: Client::deselect
/// [Separate Procedure]: Client::separate
#[derive(Clone, Copy, Debug, PartialEq, NoUninit)]
#[repr(u8)]
pub enum SelectionState {
  /// ### NOT SELECTED
  /// **Based on SEMI E37-1109§5.5.2.1**
  /// 
  /// In this state, the [Client] is ready to initiate the [Select Procedure]
  /// but has either not yet done so, or has terminated a previous session.
  /// 
  /// [Client]:           Client
  /// [Select Procedure]: Client::select
  NotSelected,

  /// ### SELECTED
  /// **Based on SEMI E37-1109§5.5.2.2**
  /// 
  /// In this state, the [Client] has successfully initiated the
  /// [Select Procedure] and is able to send and receive [Data Message]s.
  /// 
  /// [Client]:           Client
  /// [Select Procedure]: Client::select
  /// [Data Message]:     MessageContents::DataMessage
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
/// and which the [Client] will abide by.
/// 
/// [HSMS]:   crate
/// [Client]: Client
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParameterSettings {
  /// ### CONNECT MODE
  /// 
  /// Specifies the [Connection Mode] the [Client] will provide to
  /// the [Primitive Client] to use when performing the [Connect Procedure]:
  /// [PASSIVE] to wait for an incoming connection, or [ACTIVE] to initiate
  /// an outgoing connection.
  /// 
  /// [Primitive Client]:  primitive::Client
  /// [Client]:            Client
  /// [Connect Procedure]: Client::connect
  /// [Connection Mode]:   ConnectionMode
  /// [PASSIVE]:           ConnectionMode::Passive
  /// [ACTIVE]:            ConnectionMode::Active
  pub connect_mode: ConnectionMode,

  /// ### T3: REPLY TIMEOUT
  /// 
  /// The maximum amount of time that the [Client] will wait after sending
  /// a Primary [Data Message] to receive the appropriate Response
  /// [Data Message] before it must initiate the [Disconnect Procedure].
  /// 
  /// [Client]:               Client
  /// [Disconnect Procedure]: Client::disconnect
  /// [Data Message]:         MessageContents::DataMessage
  pub t3: Duration,

  /// ### T5: CONNECTION SEPARATION TIMEOUT
  /// 
  /// The minimum amount of time that the [Client] must wait between successive
  /// attempts to initiate the [Connect Procedure] with a [Connect Mode] of
  /// [ACTIVE].
  /// 
  /// [Client]:            Client
  /// [Connect Procedure]: Client::connect
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
  /// [Client]:               Client
  /// [Disconnect Procedure]: Client::disconnect
  /// [Select Request]:       MessageContents::SelectRequest
  /// [Select Response]:      MessageContents::SelectResponse
  /// [Deselect Request]:     MessageContents::DeselectRequest
  /// [Deselect Response]:    MessageContents::DeselectResponse
  /// [Linktest Request]:     MessageContents::LinktestRequest
  /// [Linktest Response]:    MessageContents::LinktestResponse
  pub t6: Duration,

  /// ### T7: NOT SELECTED TIMEOUT
  /// 
  /// The maximum amount of time that the [Client] will wait after being
  /// placed in the [NOT SELECTED] state before it must initiate the
  /// [Disconnect Procedure].
  /// 
  /// [Client]:               Client
  /// [Disconnect Procedure]: Client::disconnect
  /// [NOT SELECTED]:         SelectionState::NotSelected
  pub t7: Duration,

  /// ### T8: NETWORK INTERCHARACTER TIMEOUT
  /// 
  /// The amount of time that the [Client] will provide to the
  /// [Primitive Client] to use as the maximum amount of time it may wait while
  /// sending or receiving data between successive characters in the same
  /// [Primitive Message] before it must initiate the [Disconnect Procedure].
  /// 
  /// [Primitive Client]:     primitive::Client
  /// [Disconnect Procedure]: primitive::Client::disconnect
  /// [Primitive Message]:    primitive::Message
  /// [Client]:               Client
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
  /// [Parameter Settings]: ParameterSettings
  /// [PASSIVE]:            ConnectionMode::Passive
  /// [Connect Mode]:       ParameterSettings::connect_mode
  /// [T3]:                 ParameterSettings::t3
  /// [T5]:                 ParameterSettings::t5
  /// [T6]:                 ParameterSettings::t6
  /// [T7]:                 ParameterSettings::t7
  /// [T8]:                 ParameterSettings::t8
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

/// ## MESSAGE
/// **Based on SEMI E37-1109§8.2-8.3**
/// 
/// Data using the structure defined by the [Generic Services], enforcing
/// compliance as determined by a [Presentation Type] of 0, broken down into
/// its [Message ID] and [Message Contents].
/// 
/// [Generic Services]:  crate::generic
/// [Presentation Type]: PresentationType
/// [Message ID]:        MessageID
/// [Message Contents]:  MessageContents
#[derive(Clone, Debug)]
pub struct Message {
  pub id: MessageID,
  pub contents: MessageContents,
}
impl From<Message> for primitive::Message {
  /// ### PRIMITIVE MESSAGE FROM GENERIC MESSAGE
  /// 
  /// Due to the fact that valid [Generic Message]s are a subset of valid
  /// [Primitive Message]s, this operation is infallible.
  /// 
  /// [Generic Message]:   Message
  /// [Primitive Message]: primitive::Message
  fn from(message: Message) -> Self {
    match message.contents {
      MessageContents::DataMessage(e5_message) => {
        primitive::Message {
          header: primitive::MessageHeader {
            session_id        : message.id.session,
            byte_2            : ((e5_message.w as u8) << 7) | e5_message.stream,
            byte_3            : e5_message.function,
            presentation_type : PresentationType::SecsII as u8,
            session_type      : SessionType::DataMessage as u8,
            system            : message.id.system,
          },
          text: match e5_message.text {
            Some(item) => Vec::<u8>::from(item),
            None => vec![],
          },
        }
      },
      MessageContents::SelectRequest => {
        primitive::Message {
          header: primitive::MessageHeader {
            session_id        : message.id.session,
            byte_2            : 0,
            byte_3            : 0,
            presentation_type : PresentationType::SecsII as u8,
            session_type      : SessionType::SelectRequest as u8,
            system            : message.id.system,
          },
          text: vec![],
        }
      },
      MessageContents::SelectResponse(select_status) => {
        primitive::Message {
          header: primitive::MessageHeader {
            session_id        : message.id.session,
            byte_2            : 0,
            byte_3            : select_status,
            presentation_type : PresentationType::SecsII as u8,
            session_type      : SessionType::SelectResponse as u8,
            system            : message.id.system,
          },
          text: vec![],
        }
      },
      MessageContents::DeselectRequest => {
        primitive::Message {
          header: primitive::MessageHeader {
            session_id        : message.id.session,
            byte_2            : 0,
            byte_3            : 0,
            presentation_type : PresentationType::SecsII as u8,
            session_type      : SessionType::DeselectRequest as u8,
            system            : message.id.system,
          },
          text: vec![],
        }
      },
      MessageContents::DeselectResponse(deselect_status) => {
        primitive::Message {
          header: primitive::MessageHeader {
            session_id        : message.id.session,
            byte_2            : 0,
            byte_3            : deselect_status,
            presentation_type : PresentationType::SecsII as u8,
            session_type      : SessionType::DeselectResponse as u8,
            system            : message.id.system,
          },
          text: vec![],
        }
      },
      MessageContents::LinktestRequest => {
        primitive::Message {
          header: primitive::MessageHeader {
            session_id        : 0xFFFF,
            byte_2            : 0,
            byte_3            : 0,
            presentation_type : PresentationType::SecsII as u8,
            session_type      : SessionType::LinktestRequest as u8,
            system            : message.id.system,
          },
          text: vec![],
        }
      },
      MessageContents::LinktestResponse => {
        primitive::Message {
          header: primitive::MessageHeader {
            session_id        : 0xFFFF,
            byte_2            : 0,
            byte_3            : 0,
            presentation_type : PresentationType::SecsII as u8,
            session_type      : SessionType::LinktestResponse as u8,
            system            : message.id.system,
          },
          text: vec![],
        }
      },
      MessageContents::RejectRequest(message_type, reason_code) => {
        primitive::Message {
          header: primitive::MessageHeader {
            session_id        : message.id.session,
            byte_2            : message_type,
            byte_3            : reason_code,
            presentation_type : PresentationType::SecsII as u8,
            session_type      : SessionType::RejectRequest as u8,
            system            : message.id.system,
          },
          text: vec![],
        }
      },
      MessageContents::SeparateRequest => {
        primitive::Message {
          header: primitive::MessageHeader {
            session_id        : message.id.session,
            byte_2            : 0,
            byte_3            : 0,
            presentation_type : PresentationType::SecsII as u8,
            session_type      : SessionType::SeparateRequest as u8,
            system            : message.id.system,
          },
          text: vec![],
        }
      },
    }
  }
}
impl TryFrom<primitive::Message> for Message {
  type Error = RejectReason;

  /// ## GENERIC MESSAGE FROM PRIMITIVE MESSAGE
  /// 
  /// Due to the fact that valid [Generic Message]s are a subset of valid
  /// [Primitive Message]s, this operation is fallable when the
  /// [Primitive Message] is not a [Generic Message].
  /// 
  /// [Generic Message]:   Message
  /// [Primitive Message]: primitive::Message
  fn try_from(message: primitive::Message) -> Result<Self, Self::Error> {
    if message.header.presentation_type != 0 {return Err(RejectReason::UnsupportedPresentationType)}
    Ok(Message {
      id: MessageID {
        session: message.header.session_id,
        system: message.header.system,
      },
      contents: match message.header.session_type {
        0 => {
          MessageContents::DataMessage(semi_e5::Message{
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
          MessageContents::SelectRequest
        },
        2 => {
          if message.header.byte_2 != 0 {return Err(RejectReason::MalformedData)}
          if !message.text.is_empty()   {return Err(RejectReason::MalformedData)}
          MessageContents::SelectResponse(message.header.byte_3)
        },
        3 => {
          if message.header.byte_2 != 0 {return Err(RejectReason::MalformedData)}
          if message.header.byte_3 != 0 {return Err(RejectReason::MalformedData)}
          if !message.text.is_empty()   {return Err(RejectReason::MalformedData)}
          MessageContents::DeselectRequest
        },
        4 => {
          if message.header.byte_2 != 0 {return Err(RejectReason::MalformedData)}
          if !message.text.is_empty()   {return Err(RejectReason::MalformedData)}
          MessageContents::DeselectResponse(message.header.byte_3)
        },
        5 => {
          if message.header.session_id != 0xFFFF {return Err(RejectReason::MalformedData)}
          if message.header.byte_2     != 0      {return Err(RejectReason::MalformedData)}
          if message.header.byte_3     != 0      {return Err(RejectReason::MalformedData)}
          if !message.text.is_empty()            {return Err(RejectReason::MalformedData)}
          MessageContents::LinktestRequest
        },
        6 => {
          if message.header.session_id != 0xFFFF {return Err(RejectReason::MalformedData)}
          if message.header.byte_2     != 0      {return Err(RejectReason::MalformedData)}
          if message.header.byte_3     != 0      {return Err(RejectReason::MalformedData)}
          if !message.text.is_empty()            {return Err(RejectReason::MalformedData)}
          MessageContents::LinktestResponse
        },
        7 => {
          if !message.text.is_empty() {return Err(RejectReason::MalformedData)}
          MessageContents::RejectRequest(message.header.byte_2, message.header.byte_3)
        },
        9 => {
          if message.header.byte_2 != 0 {return Err(RejectReason::MalformedData)}
          if message.header.byte_3 != 0 {return Err(RejectReason::MalformedData)}
          if !message.text.is_empty()   {return Err(RejectReason::MalformedData)}
          MessageContents::SeparateRequest
        },
        _ => {return Err(RejectReason::UnsupportedSessionType)}
      },
    })
  }
}

/// ## MESSAGE ID
/// **Based on SEMI E37-1109§8.2**
/// 
/// The uniquely identifying components of a [Message] in forming a valid
/// transaction, including the [Session ID] and [System Bytes].
/// 
/// [Message]:      Message
/// [Session ID]:   MessageID::session
/// [System Bytes]: MessageID::system
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MessageID {
  /// ### SESSION ID
  /// **Based on SEMI E37-1109§8.2.6.1**
  /// 
  /// Provides an association between [Message]s across multiple
  /// transactions, particularly to link the [Select Procedure] and
  /// [Deselect Procedure] to subsequent [Data Message]s.
  /// 
  /// [Select Procedure]:   Client::select
  /// [Deselect Procedure]: Client::deselect
  /// [Message]:            Message
  /// [Data Message]:       MessageContents::DataMessage
  pub session: u16,

  /// ### SYSTEM BYTES
  /// **Based on SEMI E37-1109§8.2.6.7**
  /// 
  /// Identifies a transaction uniquely among the set of open transactions.
  pub system: u32,
}

/// ## MESSAGE CONTENTS
/// **Based on SEMI E37-1109§8.3.1-8.3.21**
/// 
/// The contents of a [Message], broken down by its [Session Type]:
/// 
/// - [SECS-II] formatted [Data Message]
/// - [Select.req]
/// - [Select.rsp]
/// - [Deselect.req]
/// - [Deselect.rsp]
/// - [Linktest.req]
/// - [Linktest.rsp]
/// - [Reject.req]
/// - [Separate.req]
/// 
/// [SECS-II]:      semi_e5
/// [Message]:      Message
/// [Session Type]: SessionType
/// [Data Message]: MessageContents::DataMessage
/// [Select.req]:   MessageContents::SelectRequest
/// [Select.rsp]:   MessageContents::SelectResponse
/// [Deselect.req]: MessageContents::DeselectRequest
/// [Deselect.rsp]: MessageContents::DeselectResponse
/// [Linktest.req]: MessageContents::LinktestRequest
/// [Linktest.rsp]: MessageContents::LinktestResponse
/// [Reject.req]:   MessageContents::RejectRequest
/// [Separate.req]: MessageContents::SeparateRequest
#[repr(u8)]
#[derive(Clone, Debug)]
pub enum MessageContents {
  /// ## DATA MESSAGE
  /// **Based on SEMI E37-1109§8.3.1-8.3.3**
  /// 
  /// A [Message] with a [Session Type] of 0, used by the initiator of or
  /// responding entity in the [Data Procedure] to send data.
  /// 
  /// Contains [SECS-II] formatted data.
  /// 
  /// [SECS-II]:        semi_e5
  /// [Message]:        Message
  /// [Session Type]:   SessionType
  /// [Data Procedure]: Client::data
  DataMessage(semi_e5::Message) = SessionType::DataMessage as u8,

  /// ## SELECT REQUEST
  /// **Based on SEMI E37-1109§8.3.4**
  /// 
  /// A [Message] with a [Session Type] of 1, used by the initiator of the
  /// [Select Procedure] for establishing communications.
  /// 
  /// [Message]:          Message
  /// [Select Procedure]: Client::select
  /// [Session Type]:     SessionType
  SelectRequest = SessionType::SelectRequest as u8,

  /// ## SELECT RESPONSE
  /// **Based on SEMI E37-1109§8.3.5-8.3.7**
  /// 
  /// A [Message] with a [Session Type] of 2, used by the responding
  /// entity in the [Select Procedure].
  /// 
  /// Contains a [Select Status], indicating the success or failure mode of
  /// the [Select Procedure].
  /// 
  /// [Message]:          Message
  /// [Select Procedure]: Client::select
  /// [Session Type]:     SessionType
  /// [Select Status]:    SelectStatus
  SelectResponse(u8) = SessionType::SelectResponse as u8,

  /// ## DESELECT REQUEST
  /// **Based on SEMI E37-1109§8.3.8-8.3.10**
  /// 
  /// A [Message] with a [Session Type] of 3, used by the initiator of the
  /// [Deselect Procedure] for breaking communications.
  /// 
  /// [Message]:            Message
  /// [Deselect Procedure]: Client::deselect
  /// [Session Type]:       SessionType
  DeselectRequest = SessionType::DeselectRequest as u8,

  /// ## DESELECT RESPONSE
  /// **Based on SEMI E37-1109§8.3.11-8.3.13**
  /// 
  /// An [Message] with a [Session Type] of 4, used by the responding entity
  /// in the [Deselect Procedure].
  /// 
  /// Contains a [Deselect Status], indicating the success or failure mode of
  /// the [Deselect Procedure].
  /// 
  /// [Message]:            Message
  /// [Deselect Procedure]: Client::deselect
  /// [Session Type]:       SessionType
  /// [Deselect Status]:    DeselectStatus
  DeselectResponse(u8) = SessionType::DeselectResponse as u8,

  /// ## LINKTEST REQUEST
  /// **Based on SEMI E37-1109§8.3.14-8.3.16**
  /// 
  /// A [Message] with a [Session Type] of 5, used by the initiator of the
  /// [Linktest Procedure] for checking communications stability.
  /// 
  /// [Message]:            Message
  /// [Session Type]:       SessionType
  /// [Linktest Procedure]: Client::linktest
  LinktestRequest = SessionType::LinktestRequest as u8,

  /// ## LINKTEST RESPONSE
  /// **Based on SEMI E37-1109§8.3.17-8.3.19**
  /// 
  /// A [Message] with a [Session Type] of 6, used by the responding entity
  /// in the [Linktest Procedure].
  /// 
  /// [Message]:            Message
  /// [Session Type]:       SessionType
  /// [Linktest Procedure]: Client::linktest
  LinktestResponse = SessionType::LinktestResponse as u8,

  /// ## REJECT REQUEST
  /// **Based on SEMI E37-1109§8.3.20-8.3.21**
  /// 
  /// A [Message] with a [Session Type] of 7, used by the responding entity
  /// in the [Reject Procedure].
  /// 
  /// Contains the [Presentation Type] or [Session Type] of the [Message] being
  /// rejected, and the [Reason Code] indicating why the message was rejected.
  /// 
  /// [Message]:           Message
  /// [Reject Procedure]:  Client::reject
  /// [Presentation Type]: PresentationType
  /// [Session Type]:      SessionType
  /// [Reason Code]:       RejectReason
  RejectRequest(u8, u8) = SessionType::RejectRequest as u8,

  /// ## SEPARATE REQUEST
  /// **Based on SEMI E37-1109§8.3.22**
  /// 
  /// A [Message] with a [Session Type] of 9, used by the initiator of the
  /// [Separate Procedure] for breaking communications.
  /// 
  /// [Message]:            Message
  /// [Separate Procedure]: Client::separate
  /// [Session Type]:       SessionType
  SeparateRequest = SessionType::SeparateRequest as u8,
}

/// ## SESSION TYPE
/// **Based on SEMI E37-1109§8.2.6.5-8.2.6.6**
/// 
/// Defines the type of [Message] being sent.
/// 
/// Values 11-127 are reserved for Subsidiary Standards.
/// 
/// Values 8, 10, and 128-255 are reserved and may not be used.
/// 
/// [Message]: Message
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SessionType {
  /// ### DATA MESSAGE
  /// 
  /// Denotes a [SECS-II] formatted [Data Message].
  /// 
  /// [SECS-II]:      semi_e5
  /// [Data Message]: MessageContents::DataMessage 
  DataMessage = 0,

  /// ### SELECT REQUEST
  /// 
  /// Denotes a [Select.req] message.
  /// 
  /// [Select.req]: MessageContents::SelectRequest
  SelectRequest = 1,

  /// ### SELECT RESPONSE
  /// 
  /// Denotes a [Select.rsp] message.
  /// 
  /// [Select.rsp]: MessageContents::SelectResponse
  SelectResponse = 2,

  /// ### DESELECT REQUEST
  /// 
  /// Denotes a [Deselect.req] message.
  /// 
  /// [Deselect.req]: MessageContents::DeselectRequest
  DeselectRequest = 3,

  /// ### DESELECT RESPONSE
  /// 
  /// Denotes a [Deselect.rsp] message.
  /// 
  /// [Deselect.rsp]: MessageContents::DeselectResponse
  DeselectResponse = 4,

  /// ### LINKTEST REQUEST
  /// 
  /// Denotes a [Linktest.req] message.
  /// 
  /// [Linktest.req]: MessageContents::LinktestRequest
  LinktestRequest = 5,

  /// ### LINKTEST RESPONSE
  /// 
  /// Denotes a [Linktest.rsp] message.
  /// 
  /// [Linktest.rsp]: MessageContents::LinktestResponse
  LinktestResponse = 6,

  /// ### REJECT REQUEST
  /// 
  /// Denotes a [Reject.req] message.
  /// 
  /// [Reject.req]: MessageContents::RejectRequest
  RejectRequest = 7,

  /// ### SEPARATE REQUEST
  /// 
  /// Denotes a [Separate.req] message.
  /// 
  /// [Separate.req]: MessageContents::SeparateRequest
  SeparateRequest = 9,
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
/// [Byte 3]:           primitive::MessageHeader::byte_3
/// [Deselect.rsp]:     MessageContents::DeselectResponse
/// [Select Procedure]: Client::select
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
/// reason for failure of the [Deselect Procedure].
/// 
/// Values 3-127 are reserved for Subsidiary Standards.
/// 
/// Values 128-255 are reserved for the Local Entity.
/// 
/// [Byte 3]:             primitive::MessageHeader::byte_3
/// [Deselect.rsp]:       MessageContents::DeselectResponse
/// [Deselect Procedure]: Client::deselect
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
/// been rejected in the [Reject Procedure].
/// 
/// Values 4-127 are reserved for Subsidiary Standards.
/// 
/// Values 0, and 128-255 are reserved for the Local Entity.
/// 
/// [Byte 3]:           primitive::MessageHeader::byte_3
/// [Reject.req]:       MessageContents::RejectRequest
/// [Reject Procedure]: Client::reject
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RejectReason {
  /// ### MALFORMED DATA
  /// **Local Entity Specific Reason**
  /// 
  /// A [Message] was recieved which was valid according to the
  /// [Primitive Services] but invalid according to the [Generic Services].
  /// 
  /// [Message]:            primitive::Message
  /// [Primitive Services]: primitive
  /// [Generic Services]:   crate::generic
  MalformedData = 0,

  /// ### SESSION TYPE NOT SUPPORTED
  /// 
  /// A [Message] was received whose [Session Type] value is not allowed.
  /// 
  /// [Message]:      primitive::Message
  /// [Session Type]: SessionType
  UnsupportedSessionType = 1,

  /// ### PRESENTATION TYPE NOT SUPPORTED
  /// 
  /// A [Message] was received whose [Presentation Type] value is not allowed.
  /// 
  /// [Message]:           primitive::Message
  /// [Presentation Type]: crate::PresentationType
  UnsupportedPresentationType = 2,

  /// ### TRANSACTION NOT OPEN
  /// 
  /// A [Select.rsp], [Deselect.rsp], or [Linktest.rsp] was recieved when there
  /// was no outstanding [Select.req], [Deselect.req], or [Linktest.req] which
  /// corresponded to it.
  /// 
  /// [Select.req]:   MessageContents::SelectRequest
  /// [Select.rsp]:   MessageContents::SelectResponse
  /// [Deselect.req]: MessageContents::DeselectRequest
  /// [Deselect.rsp]: MessageContents::DeselectResponse
  /// [Linktest.req]: MessageContents::LinktestRequest
  /// [Linktest.rsp]: MessageContents::LinktestResponse
  TransactionNotOpen = 3,

  /// ### ENTITY NOT SELECTED
  /// 
  /// A [Data Message] was recieved when not in the [SELECTED] state.
  /// 
  /// [Data Message]: MessageContents::DataMessage
  /// [SELECTED]:     SelectionState::Selected
  EntityNotSelected = 4,
}
