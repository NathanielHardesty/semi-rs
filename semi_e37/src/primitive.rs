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

use std::io::{Error, ErrorKind, Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

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
  /// ### CONNECTION STATE
  /// 
  /// The current [Connection State].
  /// 
  /// [Connection State]: ConnectionState
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
    // CONNECT
    //
    // We attempt here to derive a TCP connection, first by inspecting the
    // current connection state.
    let (stream, socket) = match self.connection_state.read().unwrap().deref() {
      // CONNECTED
      //
      // Two connections cannot be managed by the same client, so the function
      // exits early by returning an error.
      ConnectionState::Connected(_) => return Err(Error::new(ErrorKind::AlreadyExists, "semi_e37::primitive::Client::connect")),

      // NOT CONNECTED
      //
      // Because the client is not currently managing another connection, the
      // connect procedure may continue.
      ConnectionState::NotConnected => {
        // CONNECT
        //
        // Here we attempt to derive a TCP connection, and branch based on the
        // specified connection mode.
        match connection_mode {
          // PASSIVE CONNECTION MODE
          //
          // The passive connection mode requires the client to:
          // - Obtain a connection endpoint.
          // - Listen for an incoming connect request to the published port.
          // - Upon recept of a connect request, acknowledge and accept it.
          ConnectionMode::Passive => {
            // Obtain a connection endpoint.
            let listener: TcpListener = TcpListener::bind(entity)?;
            // Listen for an incoming connect request, and proceed to accept
            // and acknowledge it automatically.
            listener.accept()?
          }

          // ACTIVE CONNECTION MODE
          //
          // The active connection mode requires the client to:
          // - Obtain a connection endpoint.
          // - Initiate a connection to a published port.
          // - Wait for the other side to acknowledge and accept the connect.
          ConnectionMode::Active => {
            // Because the function intakes a string for the address to connect
            // to, the string must be converted into a socket address; this
            // operation is fallable.
            let socket: SocketAddr = entity.to_socket_addrs()?.next().ok_or(Error::new(ErrorKind::AddrNotAvailable, "semi_e37::primitive::Client::connect"))?;
            // Obtain and initiate a connection, with possible timeout.
            let stream: TcpStream = TcpStream::connect_timeout(
              &socket, 
              t5,
            )?;
            (stream, socket)
          }
        }
      }
    };

    // CONNECT SUCCESSFUL
    //
    // Now that a TCP stream has been connected, it is properly initiated with
    // respect to the protocol by setting the read and write timeouts to the
    // provided T8 value.
    stream.set_read_timeout(Some(t8))?;
    stream.set_write_timeout(Some(t8))?;

    // MOVE TO CONNECTED STATE
    //
    // Now that a TCP stream has been connected and initiated, it is safe to
    // declare that the connection is live and may be used by the receive and
    // transmit protocols.
    //
    // TODO: This particular usage of guards may cause issues if this function
    //       is called twice in rapid succession, a solution like that for the
    //       selection state in the generic client may be required?
    *self.connection_state.write().unwrap().deref_mut() = ConnectionState::Connected(stream);

    // CREATE MESSAGE CHANNEL
    //
    // A new channel is created for received messages to be provided through.
    let (rx_sender, rx_receiver) = channel::<Message>();

    // START RECEIVE PROCEDURE
    //
    // The receive procedure is not called externally, instead upon connection
    // a new thread which runs automatically is started. It is provided with
    // the sending end of the message channel.
    let rx_clone: Arc<Client> = self.clone();
    thread::spawn(move || {rx_clone.receive(rx_sender.clone())});

    // FINISH
    //
    // The caller is now provided with the socket address and receiving end of
    // the message channel.
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
    // DISCONNECT
    //
    // We seek to close an exiting TCP connection, first by inspecting the
    // current connection state.
    match self.connection_state.read().unwrap().deref() {
      // NOT CONNECTED
      //
      // If no connection is established, there is nothing to do, so the
      // function exits early by returning an error.
      ConnectionState::NotConnected => return Err(Error::new(ErrorKind::NotConnected, "semi_e37::primitive::Client::disconnect")),

      // CONNECTED
      //
      // Because the client is currently managing a connection, the function
      // may proceed normally.
      //
      // TODO: Perhaps a JoinHandle for the receive thread should be stored,
      //       and have join called on it here?
      ConnectionState::Connected(stream) => {
        // SHUTDOWN TCP
        //
        // The TCP connection is shut down in order to inform the other end
        // that communications are no longer occuring. This should also cause
        // the receive thread to error out and quit execution if it has not
        // already done so.
        let _result: Result<(), Error> = stream.shutdown(Shutdown::Both);
      }
    }

    // MOVE TO NOT CONNECTED
    //
    // Now that the TCP connection has been shut down, it is safe to declare
    // that the connection is no longer live.
    //
    // TODO: This particular usage of guards may cause issues if this function
    //       is called twice in rapid succession, a solution like that for the
    //       selection state in the generic client may be required?
    *self.connection_state.write().unwrap().deref_mut() = ConnectionState::NotConnected;

    // FINISH
    //
    // At this point, we are assured that no errors have occurred.
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
    // VERIFY CONNECTION
    //
    // Because receptions may time out periodically during lulls in
    // communication activity, the connection is verified to still exist each
    // time this occurs. If the connection no longer exists, the thread closes
    // and no further action is taken.
    while let ConnectionState::Connected(stream_immutable) = self.connection_state.read().unwrap().deref() {
      // ATTEMPT RECEPTION
      //
      // Because receptions may time out periodically during lulls in
      // communication activity, but other errors which require disconnection
      // or halting of this thread may occur, these two scenarios are
      // represented separately. An Err means that the thread must close, an
      // Ok(None) means that the timeout occurred at an allowable time, and
      // an Ok(Some) means that a message has been received successfully.
      let res: Result<Option<Message>, Error> = 'rx: {
        // Due to some kind of odd behavior expected by the read and write
        // functions, a mut& TcpStream is allowed to be used for achieving
        // the required mutability, rather than a &mut TcpStream. The latter
        // is more difficult to attain, though I do not remember why that was
        // the case.
        let mut stream: &TcpStream = stream_immutable;

        // RECEIVE LENGTH BYTES
        //
        // Since the length bytes are the first part of a message, and are
        // deterministically 4 bytes long according to the protocol, this is
        // where the distinction between allowable and disallowed timeouts is
        // made.
        let mut length_buffer: [u8;4] = [0;4];
        let length_bytes: usize = match stream.read(&mut length_buffer) {
          // A reception was made, but possibly not one which was 4 bytes long.
          Ok(length_received) => length_received,
          // No reception was made.
          Err(error) => match error.kind() {
            // No reception was made, due to a timeout error, which is allowed
            // here, so an Ok(None) is returned.
            ErrorKind::TimedOut => break 'rx Ok(None),
            // No reception was made, due to an error which is not acceptable,
            // so an Err is returned.
            _ => break 'rx Err(error),
          }
        };
        // If 4 bytes for the message length were not properly received, then
        // a timeout has occured at an unacceptable time, so an Err is
        // returned.
        if length_bytes != 4 {break 'rx Err(Error::from(ErrorKind::TimedOut))}
        // Now that we are assured that we have received the length bytes, we
        // we can turn them into an actual value.
        let length: u32 = u32::from_be_bytes(length_buffer);
        // According to the protocol, it is unacceptable for the length bytes
        // to have a value less than 10, as that is required in order to parse
        // the message header. If that is the case, then an Err is returned.
        if length < 10 {break 'rx Err(Error::from(ErrorKind::InvalidData))}

        // RECEIVE MESSAGE HEADER
        //
        // Now that we are assured that the length field is valid, we proceed
        // to receive the actual message header.
        let mut header_buffer: [u8; 10] = [0; 10];
        let header_bytes: usize = match stream.read(&mut header_buffer[0..]) {
          // A reception was made, but possibly not all of the bytes needed to
          // for the header.
          Ok(header_received) => header_received,
          // No reception was made, due to an error which is not acceptable,
          // so an Err is returned.
          Err(error) => break 'rx Err(error),
        };
        if header_bytes != 10 {break 'rx Err(Error::from(ErrorKind::TimedOut))}

        // RECEIVE MESSAGE DATA
        //, and any further data.
        let data_length: u32 = length - 10;
        let mut data_buffer: Vec<u8> = vec![0; data_length as usize];
        if data_length > 0 {
          let data_bytes: usize = match stream.read(&mut data_buffer) {
            // A reception was made, but possibly not all of the bytes needed
            // to comply with the length bytes.
            Ok(data_received) => data_received,
            // No reception was made, due to an error which is not acceptable,
            // so an Err is returned.
            Err(error) => break 'rx Err(error),
          };
          // It is unacceptable to have at this point not received all of the
          // requisite data, so an Err is returned.
          if data_bytes != data_length as usize {break 'rx Err(Error::from(ErrorKind::TimedOut))}
        }

        // PRINT DIAGNOSTIC
        //
        // This vestigial diagnostic remains useful for debugging purposes, as
        // it prints all messages received. It will be commented out in all
        // published versions.
        /*println!(
          "rx {: >4X} {: >3}{} {: >3} {: >2X} {: >2X} {: >8X} {:?}",
          u16::from_be_bytes(header_buffer[0..2].try_into().unwrap()),
          &header_buffer[2] & 0b0111_1111,
          if (&header_buffer[2] & 0b1000_0000) > 0 {'W'} else {' '},
          &header_buffer[3],
          &header_buffer[4],
          &header_buffer[5],
          u32::from_be_bytes(header_buffer[6..10].try_into().unwrap()),
          &data_buffer[0..],
        );// */

        // FINISH RECEPTION
        //
        // At this point the derivation of a message is infallable and it can
        // be returned.
        Ok(Some(Message {
          header: MessageHeader::from(header_buffer),
          text: data_buffer,
        }))
      };
      match res {
        // RECEPTION SUCCESS
        //
        // When this branch is reached, it means that a message was received,
        // or that a timeout occured at an acceptable time. If a timeout
        // has occurred, no action is taken.
        Ok(optional_rx_message) => if let Some(rx_message) = optional_rx_message {
          // SEND MESSAGE
          //
          // The message is now provided to the other end of the message
          // channel.
          if rx_sender.send(rx_message).is_err() {
            // If the other end of the message channel has hung up, there is no
            // point in continuing to receive messages, so the thread stops
            // here.
            break
          }
        }

        // RECEPTION FAILURE
        //
        // When this branch is reached, it means that the other end of the TCP
        // connection has closed the connection, or some other TCP error
        // preventing further communication has occurred.
        Err(_error) => {
          // TCP SHUTDOWN
          //
          // Only the read side of a TCP connection is guaranteed to be
          // informed of dropped communications, so shutdown is called in order
          // for the transmit function to throw an error the next time it
          // transmits, without having to reach a timeout.
          let _result: Result<(), Error> = stream_immutable.shutdown(Shutdown::Both);
          break
        }
      }
    }
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
    // VERIFY CONNECTION
    //
    // First, it is checked whether or not a connection has been established.
    match self.connection_state.read().unwrap().deref() {
      // NOT CONNECTED
      //
      // If a valid connection has not been established, it is impossible to
      // transmit the message, so the function exits early by returning an
      // error.
      ConnectionState::NotConnected => return Err(Error::new(ErrorKind::NotConnected, "semi_e37::primitive::Client::transmit")),

      // CONNECTED
      //
      // If a valid connection has been established, the function may proceed.
      ConnectionState::Connected(stream_immutable) => 'disconnect: {
        // Due to some kind of odd behavior expected by the read and write
        // functions, a mut& TcpStream is allowed to be used for achieving
        // the required mutability, rather than a &mut TcpStream. The latter
        // is more difficult to attain, though I do not remember why that was
        // the case.
        let mut stream: &TcpStream = stream_immutable;

        // SERALIZE MESSAGE LENGTH BYTES
        //
        // All messages must begin with a 4 byte field declaring the length of
        // the following data, so that is encoded here.
        //
        // TODO: It is possible (though unlikely) that a message longer than it
        //       is possible to represent with a u32 could be transmitted.
        //       Perhaps this edge case should be handled here?
        let length: u32 = (message.text.len() + 10) as u32;
        let length_buffer: [u8; 4] = length.to_be_bytes();

        // SERALIZE MESSAGE HEADER
        //
        // All messages contain at least 10 bytes sent, the header, so that is
        // encoded here.
        let header_buffer: [u8; 10] = message.header.into();

        // PRINT DIAGNOSTIC
        //
        // This vestigial diagnostic remains useful for debugging purposes, as
        // it prints all messages transmitted. It will be commented out in all
        // published versions.
        /*println!(
          "tx {: >4X} {: >3}{} {: >3} {: >2X} {: >2X} {: >8X} {:?}",
          u16::from_be_bytes(header_buffer[0..2].try_into().unwrap()),
          &header_buffer[2] & 0b0111_1111,
          if (&header_buffer[2] & 0b1000_0000) > 0 {'W'} else {' '},
          &header_buffer[3],
          &header_buffer[4],
          &header_buffer[5],
          u32::from_be_bytes(header_buffer[6..10].try_into().unwrap()),
          &message.text[0..],
        );// */

        // TRANSMIT MESSAGE
        //
        // The length bytes, followed by the message data, are now transmitted.
        // This operation is fallable, and failing to transmit due to a timeout
        // or other reason requires the client to disconnect.
        if stream.write_all(&length_buffer).is_err() {break 'disconnect};
        if stream.write_all(&header_buffer).is_err() {break 'disconnect};
        if stream.write_all(&message.text).is_err() {break 'disconnect};

        // FINISH
        //
        // At this point, we are assured that no errors have occurred.
        return Ok(())
      }
    };

    // DISCONNECT
    //
    // If a transmission error occurs, the client must now disconnect. If the
    // disconnect succeeds, it means that this is the first transmission to
    // fail after a connection has been established, so an error is returned
    // indicating that.
    self.disconnect()?;
    Err(Error::new(ErrorKind::ConnectionAborted, "semi_e37::primitive::Client::transmit"))
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
    // SERALIZE
    //
    // A new array to be returned is now created.
    let mut bytes: [u8;10] = [0;10];
    // The fields which must be broken down into multiple bytes in network
    // order are now seralized.
    let session_id_bytes: [u8;2] = val.session_id.to_be_bytes();
    let system_bytes: [u8;4] = val.system.to_be_bytes();
    // All bytes of all fields are now placed into the array.
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

    // FINISH
    //
    // Because this operation is infallable, the serialized version of the
    // header is now provided.
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
    // DESERALIZE
    //
    // The deserialization of a message header from an array of the correct
    // length is an infallable operation.
    Self {
      session_id: u16::from_be_bytes(bytes[0..2].try_into().unwrap()),
      byte_2: bytes[2],
      byte_3: bytes[3],
      presentation_type: bytes[4],
      session_type: bytes[5],
      system: u32::from_be_bytes(bytes[6..10].try_into().unwrap()),
    }
  }
}
