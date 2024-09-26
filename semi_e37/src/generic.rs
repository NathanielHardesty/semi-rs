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
//! - Create a [Client] by providing the [New Client] function with
//!   [Parameter Settings] and [Procedure Callbacks].
//! - Manage the [Connection State] with the [Connect Procedure] and
//!   [Disconnect Procedure].
//! - Manage the [Selection State] with the [Select Procedure],
//!   [Deselect Procedure], and [Separate Procedure].
//! - Receive [Data Message]s with the hook provided by the
//!   [Connect Procedure].
//! - Test connection integrity with the [Linktest Procedure].
//! - Send [Data Message]s with the [Data Procedure].
//! - Send [Reject.req] messages with the [Reject Procedure].
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
//! [Procedure Callbacks]:  ProcedureCallbacks

pub use crate::primitive::ConnectionMode;

use crate::{
  PresentationType,
  primitive,
};
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

/// ## CLIENT
/// 
/// Encapsulates the full functionality of the [HSMS] protocol without
/// reference to any subsidiary standards, known as the [Generic Services].
/// 
/// [HSMS]:             crate
/// [Generic Services]: crate::generic
pub struct Client {
  /// ### PARAMETER SETTINGS
  /// 
  /// Stores immutable [Parameter Settings] provided with the [New Client]
  /// function.
  /// 
  /// [New Client]:         Client::new
  /// [Parameter Settings]: ParameterSettings
  parameter_settings: ParameterSettings,

  /// ### PROCEDURE CALLBACKS
  /// 
  /// Stores [Procedure Callbacks] used when acting as the responding entity in
  /// the [Select Procedure], [Deselect Procedure], and [Separate Procedure].
  /// 
  /// [Procedure Callbacks]: ProcedureCallbacks
  /// [Select Procedure]:    Client::select
  /// [Deselect Procedure]:  Client::deselect
  /// [Separate Procedure]:  Client::separate
  procedure_callbacks: ProcedureCallbacks,

  /// ### PRIMITIVE CLIENT
  /// 
  /// The [Primitive Client] responsible for handling the [Connection State] by
  /// undertaking part of the [Connect Procedure] and [Disconnect Procedure],
  /// and for providing and transmitting [Primitive Message]s.
  /// 
  /// [Primitive Client]:     primitive::Client
  /// [Connection State]:     primitive::ConnectionState
  /// [Primitive Message]:    primitive::Message
  /// [Connect Procedure]:    Client::connect
  /// [Disconnect Procedure]: Client::disconnect
  primitive_client: Arc<primitive::Client>,

  /// ### SELECTION MUTEX
  /// 
  /// Locks the editing of the [Selection State] and the Selection Count for
  /// critical sections involving the [Select Procedure], [Deselect Procedure],
  /// [Separate Procedure], and [Disconnect Procedure].
  /// 
  /// [Disconnect Procedure]: Client::disconnect
  /// [Select Procedure]:     Client::select
  /// [Deselect Procedure]:   Client::deselect
  /// [Separate Procedure]:   Client::separate
  /// [Selection State]:      SelectionState
  selection_mutex: Mutex<()>,

  /// ### SELECTION COUNT
  /// 
  /// Provides flexibility in determining when to move between the [SELECTED]
  /// and [NOT SELECTED] states, by using a reference count of the number of
  /// selections which have successfully completed.
  /// 
  /// [NOT SELECTED]: SelectionState::NotSelected
  /// [SELECTED]:     SelectionState::Selected
  selection_count: Atomic<u16>,

  /// ### SELECTION STATE
  /// 
  /// The current [Selection State].
  /// 
  /// [Selection State]: SelectionState
  selection_state: Atomic<SelectionState>,

  /// ### OUTBOX
  /// 
  /// The list of open transactions initiated client-side which have not yet
  /// received a reply or timed out.
  outbox: Mutex<HashMap<MessageID, SendOnce<Option<Message>>>>,
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
  /// [NOT CONNECTED]:     primitive::ConnectionState::NotConnected
  pub fn new(
    parameter_settings: ParameterSettings,
    procedure_callbacks: ProcedureCallbacks,
  ) -> Arc<Self> {
    Arc::new(Client {
      parameter_settings,
      procedure_callbacks,
      primitive_client: primitive::Client::new(),
      selection_mutex: Default::default(),
      selection_count: Default::default(),
      selection_state: Default::default(),
      outbox: Default::default(),
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
    self.selection_state.store(SelectionState::NotSelected, Relaxed);
    self.selection_count.store(0, Relaxed);
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
  ///   completing the [Reject Procedure].
  /// - [SELECTED], Primary [Data Message] - The [Client] will send the
  ///   [Data Message] to the hook provided by the [Connect Procedure].
  /// - [SELECTED], Response [Data Message] - The [Client] will respond by
  ///   correllating the message to a previously sent Primary [Data Message],
  ///   finishing a previously initiated [Data Procedure] if successful,
  ///   or by transmitting a [Reject.req] message, rejecting the
  ///   [Data Procedure] and completing the [Reject Procedure] if unsuccessful.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### [Select.req]:
  /// 
  /// - The [Client] will respond by calling the [Select Procedure Callback].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### [Select.rsp]:
  /// 
  /// - The [Client] will respond by correllating the message to a previously
  ///   send [Select.req], finishing a previously initiated [Select Procedure]
  ///   if successful, or by transmitting a [Reject.req] message, rejecting the
  ///   [Select Procedure] and completing the [Reject Procedure] if
  ///   unsuccessful.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### [Deselect.req]:
  /// 
  /// - The [Client] will respond by calling the [Deselect Procedure Callback].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### [Deselect.rsp]:
  /// 
  /// - The [Client] will respond by correllating the message to a previously
  ///   send [Deselect.req], finishing a previously initiated
  ///   [Deselect Procedure] if successful, or by transmitting a [Reject.req]
  ///   message, rejecting the [Deselect Procedure] and completing the
  ///   [Reject Procedure] if unsuccessful.
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
  /// - The [Client] will respond by correlating the message to a previously
  ///   sent message which is awaiting a reply, aborting a previously initiated
  ///   [Data Procedure], [Select Procedure], [Deselect Procedure], or
  ///   [Linktest Procedure], and completing the [Reject Procedure].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### [Separate.req]:
  /// 
  /// - The [Client] will respond by calling the [Separate Procedure Callback].
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// #### Unknown [Primitive Message]:
  /// 
  /// - The [Client] will respond by transmitting a [Reject.req] message,
  ///   completing the [Reject Procedure]. 
  /// 
  /// [Primitive Message]:           primitive::Message
  /// [Connection State]:            primitive::ConnectionState
  /// [NOT CONNECTED]:               primitive::ConnectionState::NotConnected
  /// [CONNECTED]:                   primitive::ConnectionState::Connected
  /// [Client]:                      Client
  /// [Connect Procedure]:           Client::connect
  /// [Select Procedure]:            Client::select
  /// [Data Procedure]:              Client::data
  /// [Deselect Procedure]:          Client::deselect
  /// [Linktest Procedure]:          Client::linktest
  /// [Separate Procedure]:          Client::separate
  /// [Reject Procedure]:            Client::reject
  /// [Selection State]:             SelectionState
  /// [NOT SELECTED]:                SelectionState::NotSelected
  /// [SELECTED]:                    SelectionState::Selected
  /// [SELECT INITIATED]:            SelectionState::SelectInitiated
  /// [DESELECT INITIATED]:          SelectionState::DeselectInitiated
  /// [Select Procedure Callback]:   ProcedureCallbacks::select
  /// [Deselect Procedure Callback]: ProcedureCallbacks::deselect
  /// [Separate Procedure Callback]: ProcedureCallbacks::separate
  /// [Message]:                     Message
  /// [Message Contents]:            MessageContents
  /// [Data Message]:                MessageContents::DataMessage
  /// [Select.req]:                  MessageContents::SelectRequest
  /// [Select.rsp]:                  MessageContents::SelectResponse
  /// [Deselect.req]:                MessageContents::DeselectRequest
  /// [Deselect.rsp]:                MessageContents::DeselectResponse
  /// [Linktest.req]:                MessageContents::LinktestRequest
  /// [Linktest.rsp]:                MessageContents::LinktestResponse
  /// [Reject.req]:                  MessageContents::RejectRequest
  /// [Separate.req]:                MessageContents::SeparateRequest
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
                  match self.outbox.lock().unwrap().deref_mut().remove(&rx_message.id) {
                    // OUTBOX: Transaction Not Found
                    None => {
                      // TX: Reject.req 
                      if self.primitive_client.transmit(Message {
                        id: rx_message.id,
                        contents: MessageContents::RejectRequest(0, RejectReason::TransactionNotOpen as u8)
                      }.into()).is_err() {break}
                    }
                    // OUTBOX: Transaction Found
                    Some(sender) => {
                      // OUTBOX: Complete Transaction
                      sender.send(Some(Message {
                        id: rx_message.id,
                        contents: MessageContents::DataMessage(data),
                      })).unwrap();
                    }
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
          }
          // RX: Select.req
          MessageContents::SelectRequest => {
            match self.selection_mutex.try_lock() {
              Ok(_guard) => {
                // CALLBACK
                let selection_count = self.selection_count.load(Relaxed);
                let select_status = (self.procedure_callbacks.select)(rx_message.id.session, selection_count);
                // CALLBACK: SUCCESS
                if let SelectStatus::Ok = select_status {
                  // SELECTION COUNT + 1
                  self.selection_count.store(selection_count + 1, Relaxed);
                  // TO: SELECTED
                  self.selection_state.store(SelectionState::Selected, Relaxed);
                }
                // TX: Select.rsp
                if self.primitive_client.transmit(Message {
                  id: rx_message.id,
                  contents: MessageContents::SelectResponse(select_status as u8),
                }.into()).is_err() {break};
              },
              Err(_) => {
                // TODO: probably appropriate to put something here, maybe to do with the simulatenous select procedure?
              },
            }
          }
          // RX: Select.rsp
          MessageContents::SelectResponse(select_status) => {
            // OUTBOX: Find Transaction
            match self.outbox.lock().unwrap().deref_mut().remove(&rx_message.id) {
              // OUTBOX: Transaction Not Found
              None => {
                // TX: Reject.req 
                if self.primitive_client.transmit(Message {
                  id: rx_message.id,
                  contents: MessageContents::RejectRequest(0, RejectReason::TransactionNotOpen as u8)
                }.into()).is_err() {break}
              }
              // OUTBOX: Transaction Found
              Some(sender) => {
                // OUTBOX: Complete Transaction
                sender.send(Some(Message {
                  id: rx_message.id,
                  contents: MessageContents::SelectResponse(select_status),
                })).unwrap();
              }
            }
          }
          // RX: Deselect.req
          MessageContents::DeselectRequest => {
            match self.selection_mutex.try_lock() {
              Ok(_guard) => {
                // CALLBACK
                let selection_count = self.selection_count.load(Relaxed);
                if selection_count > 0 {
                  let deselect_status = (self.procedure_callbacks.deselect)(rx_message.id.session, selection_count);
                  // CALLBACK: SUCCESS
                  if let DeselectStatus::Ok = deselect_status {
                    // SELECTION COUNT - 1
                    self.selection_count.store(selection_count - 1, Relaxed);
                    // TO: NOT SELECTED
                    if self.selection_count.load(Relaxed) == 0 {
                      self.selection_state.store(SelectionState::NotSelected, Relaxed);
                    }
                  }
                  // TX: Deselect.rsp
                  if self.primitive_client.transmit(Message {
                    id: rx_message.id,
                    contents: MessageContents::DeselectResponse(deselect_status as u8),
                  }.into()).is_err() {break};
                } else {
                  // TX: Deselect.rsp
                  if self.primitive_client.transmit(Message {
                    id: rx_message.id,
                    contents: MessageContents::SelectResponse(DeselectStatus::NotEstablished as u8),
                  }.into()).is_err() {break};
                }
              },
              Err(_) => {
                // TODO: probably appropriate to put something here, maybe to do with the simulatenous deselect procedure?
              },
            }
          }
          // RX: Deselect.rsp
          MessageContents::DeselectResponse(deselect_status) => {
            // OUTBOX: Find Transaction
            match self.outbox.lock().unwrap().deref_mut().remove(&rx_message.id) {
              // OUTBOX: Transaction Not Found
              None => {
                // TX: Reject.req 
                if self.primitive_client.transmit(Message {
                  id: rx_message.id,
                  contents: MessageContents::RejectRequest(0, RejectReason::TransactionNotOpen as u8)
                }.into()).is_err() {break}
              }
              // OUTBOX: Transaction Found
              Some(sender) => {
                // OUTBOX: Complete Transaction
                sender.send(Some(Message {
                  id: rx_message.id,
                  contents: MessageContents::DeselectResponse(deselect_status),
                })).unwrap();
              }
            }
          }
          // RX: Linktest.req
          MessageContents::LinktestRequest => {
            // TX: Linktest.rsp
            if self.primitive_client.transmit(Message{
              id: rx_message.id,
              contents: MessageContents::LinktestResponse,
            }.into()).is_err() {break};
          }
          // RX: Linktest.rsp
          MessageContents::LinktestResponse => {
            // OUTBOX: Find Transaction
            match self.outbox.lock().unwrap().deref_mut().remove(&rx_message.id) {
              // OUTBOX: Transaction Not Found
              None => {
                // TX: Reject.req 
                if self.primitive_client.transmit(Message {
                  id: rx_message.id,
                  contents: MessageContents::RejectRequest(0, RejectReason::TransactionNotOpen as u8)
                }.into()).is_err() {break}
              }
              // OUTBOX: Transaction Found
              Some(sender) => {
                // OUTBOX: Complete Transaction
                sender.send(Some(Message {
                  id: rx_message.id,
                  contents: MessageContents::LinktestResponse,
                })).unwrap();
              }
            }
          }
          // RX: Reject.req
          MessageContents::RejectRequest(ps_type, reason_code) => {
            // OUTBOX: Find Transaction
            match self.outbox.lock().unwrap().deref_mut().remove(&rx_message.id) {
              // OUTBOX: Transaction Not Found
              None => {
                // TX: Reject.req 
                if self.primitive_client.transmit(Message {
                  id: rx_message.id,
                  contents: MessageContents::RejectRequest(0, RejectReason::TransactionNotOpen as u8)
                }.into()).is_err() {break}
              }
              // OUTBOX: Transaction Found
              Some(sender) => {
                // OUTBOX: Complete Transaction
                sender.send(Some(Message {
                  id: rx_message.id,
                  contents: MessageContents::RejectRequest(ps_type, reason_code),
                })).unwrap();
              }
            }
          }
          // RX: Separate.req
          MessageContents::SeparateRequest => {
            let _guard: std::sync::MutexGuard<'_, ()> = self.selection_mutex.lock().unwrap();
            // CALLBACK
            let selection_count = self.selection_count.load(Relaxed);
            if selection_count > 0 {
              let decrement = (self.procedure_callbacks.separate)(rx_message.id.session, selection_count);
              // CALLBACK: SUCCESS
              if decrement {
                // SELECTION COUNT - 1
                self.selection_count.store(selection_count - 1, Relaxed);
                // TO: NOT SELECTED
                if self.selection_count.load(Relaxed) == 0 {
                  self.selection_state.store(SelectionState::NotSelected, Relaxed);
                }
              }
            }
          }
        }
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
        }
      }
    }
    // OUTBOX: CLEAR
    for (_, sender) in self.outbox.lock().unwrap().deref_mut().drain() {
      let _ = sender.send(None);
    }
  }

  /// ### TRANSMIT PROCEDURE
  /// **Based on SEMI E37-1109§7.2**
  /// 
  /// Serializes a [Message] and transmits it over the TCP/IP connection and
  /// waiting up to the time specified for the requisite response [Message] to
  /// be recieved if it is necessary to do so.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// The [Connection State] must be in the [CONNECTED] state to use this
  /// procedure.
  /// 
  /// If the transmission of the message over the TCP/IP connection fails, the
  /// [Client] will consider it a communications failure and initiate the
  /// [Disconnect Procedure].
  /// 
  /// [Connection State]:     primitive::ConnectionState
  /// [NOT CONNECTED]:        primitive::ConnectionState::NotConnected
  /// [CONNECTED]:            primitive::ConnectionState::Connected
  /// [Client]:               Client
  /// [Disconnect Procedure]: Client::disconnect
  /// [Message]:              Message
  fn transmit(
    self: &Arc<Self>,
    message: Message,
    reply_expected: bool,
    delay: Duration,
  ) -> Result<Option<Message>, Error> {
    let message_id: MessageID = message.id;
    let receiver: oneshot::Receiver<Option<Message>> = {
      // OUTBOX: LOCK
      let outbox_lock = if reply_expected {Some(self.deref().outbox.lock().unwrap())} else {None};
      // TX
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
              match outbox.deref_mut().try_insert(message_id, sender) {
                Err(_error) => return Err(Error::from(ErrorKind::AlreadyExists)),
                Ok(_sender) => receiver,
              }
            }
          }
        },
        // TX: Failure
        Err(error) => {
          // DISCONNECT
          let _ = self.disconnect();
          return Err(error)
        },
      }
    };
    // RX
    let rx_result: Result<Option<Message>, _> = receiver.recv_timeout(delay);
    // OUTBOX: Remove Transaction
    let mut outbox = self.outbox.lock().unwrap();
    match rx_result {
      // RX: Success
      Ok(rx_message) => Ok(rx_message),
      // RX: Failure
      Err(_e) => {
        outbox.deref_mut().remove(&message_id);
        Ok(None)
      }
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
  /// state will respond to having received a [Data Message] based on its
  /// contents and the current [Selection State]:
  /// - [NOT SELECTED] - The [Client] will respond by transmitting a
  ///   [Reject.req] message, rejecting the [Data Procedure] and
  ///   completing the [Reject Procedure].
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
  /// The [Connection State] must be in the [CONNECTED] state to use this
  /// procedure.
  /// 
  /// The [Client] will wait to receive the [Select.rsp] for the amount
  /// of time specified by [T6] before it will consider it a communications
  /// failure and initiate the [Disconnect Procedure].
  /// 
  /// Upon completion of the [Select Procedure], the [SELECTED] state is
  /// entered, and the Selection Count is incremented by one.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Although not done within this function, a [Client] in the [CONNECTED]
  /// state will respond to having received a [Select.req] message by calling
  /// the [Select Procedure Callback].
  /// 
  /// [Connection State]:          primitive::ConnectionState
  /// [CONNECTED]:                 primitive::ConnectionState::Connected
  /// [Selection State]:           SelectionState
  /// [NOT SELECTED]:              SelectionState::NotSelected
  /// [SELECTED]:                  SelectionState::Selected
  /// [T6]:                        ParameterSettings::t6
  /// [Client]:                    Client
  /// [Disconnect Procedure]:      Client::disconnect
  /// [Select Procedure]:          Client::select
  /// [Select Procedure Callback]: ProcedureCallbacks::select
  /// [Select.req]:                MessageContents::SelectRequest
  /// [Select.rsp]:                MessageContents::SelectResponse
  pub fn select(
    self: &Arc<Self>,
    id: MessageID,
  ) -> JoinHandle<Result<(), Error>> {
    let clone: Arc<Client> = self.clone();
    thread::spawn(move || {
      let _guard = clone.selection_mutex.lock().unwrap();
      // TX: Select.req
      match clone.transmit(
        Message {
          id,
          contents: MessageContents::SelectRequest,
        },
        true,
        clone.parameter_settings.t6,
      )?{
        // RX: No Response
        None => {
          // DISCONNECT
          clone.disconnect()?;
          Err(Error::from(ErrorKind::ConnectionAborted))
        }
        // RX: Response
        Some(rx_message) => {
          match rx_message.contents {
            // RX: Select.rsp
            MessageContents::SelectResponse(select_status) => {
              // RX: Select.rsp Success
              if select_status == SelectStatus::Ok as u8 {
                // TO: SELECTED
                clone.selection_state.store(SelectionState::Selected, Relaxed);
                // SELECTION COUNT + 1
                clone.selection_count.store(clone.selection_count.load(Relaxed) + 1, Relaxed);
                // FINISH
                Ok(())
              }
              // RX: Select.rsp Failure
              else {
                Err(Error::from(ErrorKind::PermissionDenied))
              }
            }
            // RX: Reject.req
            MessageContents::RejectRequest(_type, _reason) => Err(Error::from(ErrorKind::PermissionDenied)),
            // RX: Unknown
            _ => Err(Error::from(ErrorKind::InvalidData)),
          }
        }
      }
    })
  }

  /// ### DESELECT PROCEDURE
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
  /// Upon completion of the [Deselect Procedure], the Selection Count is
  /// decremented. If the Selection Count becomes zero, the [NOT SELECTED]
  /// state is entered.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Although not done within this function, a [Client] in the [CONNECTED]
  /// state will respond to having received a [Deselect.req] message by calling
  /// the [Deselect Procedure Callback].
  /// 
  /// [Connection State]:            primitive::ConnectionState
  /// [CONNECTED]:                   primitive::ConnectionState::Connected
  /// [Client]:                      Client
  /// [Disconnect Procedure]:        Client::disconnect
  /// [Deselect Procedure]:          Client::deselect
  /// [Selection State]:             SelectionState
  /// [NOT SELECTED]:                SelectionState::NotSelected
  /// [SELECTED]:                    SelectionState::Selected
  /// [T6]:                          ParameterSettings::t6
  /// [Deselect Procedure Callback]: ProcedureCallbacks::deselect
  /// [Deselect.req]:                MessageContents::DeselectRequest
  /// [Deselect.rsp]:                MessageContents::DeselectResponse
  pub fn deselect(
    self: &Arc<Self>,
    id: MessageID,
  ) -> JoinHandle<Result<(), Error>> {
    let clone: Arc<Client> = self.clone();
    thread::spawn(move || {
      match clone.selection_state.load(Relaxed) {
        // NOT SELECTED
        SelectionState::NotSelected => Err(Error::from(ErrorKind::AlreadyExists)),
        // SELECTED
        SelectionState::Selected => {
          let _guard = clone.selection_mutex.lock().unwrap();
          // TX: Deselect.req
          match clone.transmit(
            Message {
              id,
              contents: MessageContents::DeselectRequest,
            },
            true,
            clone.parameter_settings.t6,
          )?{
            // RX: Response
            Some(rx_message) => {
              match rx_message.contents {
                // RX: Deselect.rsp
                MessageContents::DeselectResponse(deselect_status) => {
                  // RX: Deselect.rsp Success
                  if deselect_status == DeselectStatus::Ok as u8 {
                    // SELECTION COUNT - 1
                    clone.selection_count.store(clone.selection_count.load(Relaxed) - 1, Relaxed);
                    // TO: NOT SELECTED
                    if clone.selection_count.load(Relaxed) == 0 {
                      clone.selection_state.store(SelectionState::NotSelected, Relaxed);
                    }
                    // FINISH
                    Ok(())
                  }
                  // RX: Deselect.rsp Failure
                  else {
                    Err(Error::from(ErrorKind::PermissionDenied))
                  }
                },
                // RX: Reject.req
                MessageContents::RejectRequest(_type, _reason) => Err(Error::from(ErrorKind::PermissionDenied)),
                // RX: Unknown
                _ => Err(Error::from(ErrorKind::InvalidData)),
              }
            },
            // RX: No Response
            None => {
              // DISCONNECT
              clone.disconnect()?;
              Err(Error::from(ErrorKind::ConnectionAborted))
            },
          }
        },
      }
    })
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
  /// [CONNECTED] state will respond to having received a [Linktest.req]
  /// message with a [Linktest.rsp], completing the [Linktest Procedure].
  /// 
  /// [Connection State]:     primitive::ConnectionState
  /// [CONNECTED]:            primitive::ConnectionState::Connected
  /// [Client]:               Client
  /// [Disconnect Procedure]: Client::disconnect
  /// [Linktest Procedure]:   Client::linktest
  /// [Selection State]:      SelectionState
  /// [NOT SELECTED]:         SelectionState::NotSelected
  /// [SELECTED]:             SelectionState::Selected
  /// [T6]:                   ParameterSettings::t6
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
          // DISCONNECT
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
  /// Upon completion of the [Separate Procedure], the Selection Count is
  /// decremented by one. If the Selection Count becomes zero, the
  /// [NOT SELECTED] state is entered.
  /// 
  /// -------------------------------------------------------------------------
  /// 
  /// Although not done within this function, a [Client] in the [CONNECTED]
  /// state will respond to having received a [Separate.req] message by calling
  /// the [Separate Procedure Callback].
  /// 
  /// [Connection State]:            primitive::ConnectionState
  /// [CONNECTED]:                   primitive::ConnectionState::Connected
  /// [Selection State]:             SelectionState
  /// [NOT SELECTED]:                SelectionState::NotSelected
  /// [SELECTED]:                    SelectionState::Selected
  /// [Client]:                      Client
  /// [Separate Procedure]:          Client::separate
  /// [Separate Procedure Callback]: ProcedureCallbacks::separate
  /// [Separate.req]:                MessageContents::SeparateRequest
  pub fn separate(
    self: &Arc<Self>,
    id: MessageID,
  ) -> JoinHandle<Result<(), Error>> {
    let clone: Arc<Client> = self.clone();
    thread::spawn(move || {
      match clone.selection_state.load(Relaxed) {
        // NOT SELECTED: ERROR
        SelectionState::NotSelected => Err(Error::from(ErrorKind::AlreadyExists)),
        // SELECTED
        SelectionState::Selected => {
          let _guard = clone.selection_mutex.lock().unwrap();
          // TX: Separate.req
          clone.transmit(
            Message {
              id,
              contents: MessageContents::SeparateRequest,
            },
            false,
            clone.parameter_settings.t6,
          )?;
          // SELECTION COUNT - 1
          clone.selection_count.store(clone.selection_count.load(Relaxed) - 1, Relaxed);
          // TO: NOT SELECTED
          if clone.selection_count.load(Relaxed) == 0 {
            clone.selection_state.store(SelectionState::NotSelected, Relaxed);
          }
          // FINISH
          Ok(())
        }
      }
    })
  }

  /// ### REJECT PROCEDURE
  /// **Based on SEMI E37-1109§7.10**
  /// 
  /// Asks the [Client] to complete the [Reject Procedure] by transmitting a
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
  /// state will respond to having received a [Reject.req] by correlating the
  /// message to a previously sent message which is awaiting a reply, aborting
  /// a previously initiated [Data Procedure], [Select Procedure],
  /// [Deselect Procedure], or [Linktest Procedure], and completing the
  /// [Reject Procedure].
  /// 
  /// [Connection State]:   primitive::ConnectionState
  /// [CONNECTED]:          primitive::ConnectionState::Connected
  /// [Client]:             Client
  /// [Data Procedure]:     Client::data
  /// [Select Procedure]:   Client::select
  /// [Deselect Procedure]: Client::deselect
  /// [Linktest Procedure]: Client::linktest
  /// [Reject Procedure]:   Client::reject
  /// [Selection State]:    SelectionState
  /// [NOT SELECTED]:       SelectionState::NotSelected
  /// [SELECTED]:           SelectionState::Selected
  /// [Reject.req]:         MessageContents::RejectRequest
  pub fn reject(
    self: &Arc<Self>,
    id: MessageID,
    ps_type: u8,
    reason: RejectReason,
  ) -> JoinHandle<Result<(), Error>> {
    let clone: Arc<Client> = self.clone();
    thread::spawn(move || {
      // TX: Reject.req
      clone.transmit(
        Message {
          id,
          contents: MessageContents::RejectRequest(ps_type, reason as u8)
        },
        false,
        clone.parameter_settings.t6,
      )?;
      Ok(())
    })
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

/// ## PROCEDURE CALLBACKS
/// **Based on SEMI E37-1109§7**
/// 
/// In the [Generic Services], the responding entity is given the option of how
/// to respond to the [Select Procedure], [Deselect Procedure], and
/// [Separate Procedure] without futher definition by the standard.
/// 
/// In order to provide a mechanism for subsidiary standards and third-party
/// users to define this behavior, the use of callbacks is required.
/// 
/// [Generic Services]:   crate::generic
/// [Select Procedure]:   Client::select
/// [Deselect Procedure]: Client::deselect
/// [Separate Procedure]: Client::separate
#[derive(Clone)]
pub struct ProcedureCallbacks {
  /// ### SELECT PROCEDURE CALLBACK
  /// **Based on SEMI E37-1109§7.4.2**
  /// 
  /// Called when a [Select.req] is received, thus making the [Client] the
  /// responding entity in the [Select Procedure].
  /// 
  /// The [Session ID] of the [Select.req] and the current Selection Count are
  /// provided as arguments, and a [Select Status] must be provided.
  /// 
  /// The [Client] will proceed to respond with a [Select.rsp] containing the
  /// provided [Select Status], thus completing the [Select Procedure], and if
  /// a [Select Status] of [COMMUNICATION ESTABLISHED] is provided, the
  /// Selection Count will be incremented by one.
  /// 
  /// [Client]:                    Client
  /// [Select Procedure]:          Client::select
  /// [Session ID]:                MessageID::session
  /// [Select.req]:                MessageContents::SelectRequest
  /// [Select.rsp]:                MessageContents::SelectResponse
  /// [Select Status]:             SelectStatus
  /// [COMMUNICATION ESTABLISHED]: SelectStatus::Ok
  pub select: Arc<dyn Fn(u16, u16) -> SelectStatus + Sync + Send>,

  /// ### DESELECT PROCEDURE CALLBACK
  /// **Based on SEMI E37-1109§7.7.2**
  /// 
  /// Called when a [Deselect.req] is received, thus making the [Client] the
  /// responding entity in the [Deselect Procedure], and the current Selection
  /// Count is greater than zero.
  /// 
  /// The [Session ID] of the [Deselect.req] and the current Selection Count
  /// are provided as arguments, and a [Deselect Status] must be provided.
  /// 
  /// The [Client] will proceed to respond with a [Deselect.rsp] containing the
  /// provided [Deselect Status], thus completing the [Deselect Procedure], and
  /// if a [Deselect Status] of [COMMUNICATION ENDED] is provided, the
  /// Selection Count will be decremented by one.
  /// 
  /// [Client]:              Client
  /// [Deselect Procedure]:  Client::deselect
  /// [Session ID]:          MessageID::session
  /// [Deselect.req]:        MessageContents::DeselectRequest
  /// [Deselect.rsp]:        MessageContents::DeselectResponse
  /// [Deselect Status]:     DeselectStatus
  /// [COMMUNICATION ENDED]: DeselectStatus::Ok
  pub deselect: Arc<dyn Fn(u16, u16) -> DeselectStatus + Sync + Send>,

  /// ### SEPARATE PROCEDURE CALLBACK
  /// **Based on SEMI E37-1109§7.9**
  /// 
  /// Called when a [Separate.req] is received, thus making the [Client] the
  /// responding entity in the [Separate Procedure], and the current Selection
  /// Count is greater than zero.
  /// 
  /// The [Session ID] of the [Separate.req] and the current Selection Count
  /// are provided as arguments, and a boolean value indicating whether to
  /// decrement the Selection Count must be provided.
  /// 
  /// If a value of true is provided, the Selection Count will be decremented
  /// by one.
  /// 
  /// [Client]:             Client
  /// [Separate Procedure]: Client::separate
  /// [Session ID]:         MessageID::session
  /// [Separate.req]:       MessageContents::SeparateRequest
  pub separate: Arc<dyn Fn(u16, u16) -> bool + Sync + Send>,
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
  /// ### COMMUNICATION ESTABLISHED
  /// 
  /// Select was successfully completed.
  Ok = 0,

  /// ### COMMUNICATION ALREADY ACTIVE
  /// 
  /// A previous select has already established communications to the entity
  /// being selected.
  AlreadyActive = 1,

  /// ### CONNECTION NOT READY
  /// 
  /// The connection is not yet ready to accept select requests.
  NotReady = 2,

  /// ### CONNECTION EXHAUSTED
  /// 
  /// The entity is already servicing a separate TCP/IP connection and is
  /// unable to service more than one at a given time.
  Exhausted = 3,

  /// ### NO SUCH ENTITY (HSMS-GS)
  /// 
  /// The Session ID does not correspond to any Session Entity ID available at
  /// this connection.
  NoSuchEntity = 4,

  /// ### ENTITY IN USE (HSMS-GS)
  /// 
  /// The Session Entity corresponding to the Session ID is not shareable by
  /// multiple connections and is already selected by another connection.
  EntityInUse = 5,

  /// ### ENTITY SELECTED (HSMS-GS)
  /// 
  /// The Session Entity corresponding to the Session ID is already selected on
  /// the current connection.
  EntitySelected = 6,
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
  /// ### COMMUNICATION ENDED
  /// 
  /// The deselect completed successfully.
  Ok = 0,

  /// ### COMMUNICATION NOT ESTABLISHED
  /// 
  /// Communication has not been established with a prior select, or has
  /// already been ended with a previous deselect.
  NotEstablished = 1,

  /// ### COMMUNICATION BUSY
  /// 
  /// The session is still in use by the responding entity, and so it cannot
  /// relinquish it gracefully. If the initiator must still terminate
  /// communications, seprate must be used.
  Busy = 2,
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
