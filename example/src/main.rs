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

#![feature(ascii_char)]
#![feature(ascii_char_variants)]

use std::{ascii::Char::*, sync::{mpsc::Receiver, Arc}, thread::{self, JoinHandle}, time::Duration};
use semi_e5::{Item, Message, items::*, messages::*};
use semi_e37::{ConnectionMode, ConnectionStateTransition, HsmsClient, HsmsMessageID, ParameterSettings};

fn main() {
  test_data();
  let equipment = thread::spawn(|| {test_equipment();});
  let host = thread::spawn(|| {test_host();});
  let _ = equipment.join();
  let _ = host.join();
}

fn test_data() {
  println!("{:?}", Item::try_from(vec![1, 1, 177, 4, 0, 0, 7, 237]));
  let a: semi_e5::items::ErrorText = semi_e5::items::ErrorText::new(vec![CapitalA]).unwrap();
  println!("{:?}", a);
  println!("{:?}", a.read()[0])
}

fn test_equipment() {
  // CLIENT
  let parameter_settings: ParameterSettings = ParameterSettings::default();
  let client: Arc<HsmsClient> = HsmsClient::new(parameter_settings);
  // RX
  let rx_message: Receiver<(HsmsMessageID, Message)> = client.connect("127.0.0.1:5000").unwrap();
  let rx_client: Arc<HsmsClient> = client.clone();
  let rx_thread: JoinHandle<()> = thread::spawn(move || {
    for (id, message) in rx_message {
      println!("EQUIPMENT DATA RX {:?}", message);
      match (message.w, message.stream, message.function) {
        (true, 1, 3) => {
          match s1::SelectedEquipmentStatusRequest::try_from(message) {
            Ok(s1f3) => {
              let mut vec = vec![];
              for _svid in s1f3.0.0 {
                vec.push(StatusVariableValue::List(vec![Item::u4(10)]));
              }
              rx_client.data(
                id,
                s1::SelectedEquipmentStatusData(VecList(vec)).into()
              ).join().unwrap().unwrap();
            },
            Err(_) => {
              rx_client.data(id, s1::Abort.into()).join().unwrap().unwrap();
            },
          }
        },
        (true, 1, 11) => {
          match s1::StatusVariableNamelistRequest::try_from(message) {
            Ok(s1f11) => {
              let mut vec = vec![];
              for svid in s1f11.0.0 {
                vec.push((svid, StatusVariableName(vec![]), Units(vec![])));
              }
              rx_client.data(
                id,
                s1::StatusVariableNamelistReply(VecList(vec)).into()
              ).join().unwrap().unwrap();
            },
            Err(_) => {
              rx_client.data(id, s1::Abort.into()).join().unwrap().unwrap();
            },
          }
        },
        (true, 1, 13) => {
          match s1::HostCR::try_from(message) {
            Ok(_s1f13) => {
              rx_client.data(
                id,
                s1::EquipmentCRA((
                  CommAck::Accepted, (
                    ModelName::new(vec![CapitalT, SmallE, SmallS, SmallT]).unwrap(),
                    SoftwareRevision::new(vec![Digit0, Digit1, Digit0]).unwrap(),
                  )
                )).into()
              ).join().unwrap().unwrap();
            },
            Err(_) => {
              rx_client.data(id, s1::Abort.into()).join().unwrap().unwrap();
            }
          }
        },
        (true, 1, 17) => {
          match s1::RequestOnLine::try_from(message) {
            Ok(_s1f17) => {
              rx_client.data(
                id,
                s1::OnLineAck(OnLineAcknowledge::Accepted).into()
              ).join().unwrap().unwrap();
            },
            Err(_) => {
              rx_client.data(id, s1::Abort.into()).join().unwrap().unwrap();
            }
          }
        },
        (true, 1, 21) => {
          match s1::DataVariableNamelistRequest::try_from(message) {
            Ok(s1f21) => {
              let mut vec = vec![];
              for vid in s1f21.0.0 {
                vec.push((vid, DataVariableValueName(vec![]), Units(vec![])));
              }
              rx_client.data(
                id,
                s1::DataVariableNamelist(VecList(vec)).into()
              ).join().unwrap().unwrap();
            },
            Err(_) => {
              rx_client.data(id, s1::Abort.into()).join().unwrap().unwrap();
            },
          }
        },
        (true, 1, 23) => {
          match s1::CollectionEventNamelistRequest::try_from(message) {
            Ok(s1f11) => {
              let mut vec = vec![];
              for ceid in s1f11.0.0 {
                vec.push((ceid, CollectionEventName(vec![]), VecList(vec![])));
              }
              rx_client.data(
                id,
                s1::CollectionEventNamelist(VecList(vec)).into()
              ).join().unwrap().unwrap();
            },
            Err(_) => {
              rx_client.data(id, s1::Abort.into()).join().unwrap().unwrap();
            },
          }
        },
        (true, 2, 13) => {
          rx_client.data(
            id,
            Message {
              w: false,
              stream: 2,
              function: 14,
              text: Some(Item::List(vec![])),
            }
          ).join().unwrap().unwrap();
        },
        (true, 2, 29) => {
          rx_client.data(
            id,
            Message {
              w: false,
              stream: 2,
              function: 30,
              text: Some(Item::List(vec![])),
            }
          ).join().unwrap().unwrap();
        },
        (true, 5, 5) => {
          rx_client.data(
            id,
            Message {
              w: false,
              stream: 5,
              function: 6,
              text: Some(Item::List(vec![])),
            }
          ).join().unwrap().unwrap();
        },
        (true, 5, 7) => {
          rx_client.data(
            id,
            Message {
              w: false,
              stream: 5,
              function: 8,
              text: Some(Item::List(vec![])),
            }
          ).join().unwrap().unwrap();
        },
        (true, 7, 19) => {
          rx_client.data(
            id,
            Message {
              w: false,
              stream: 7,
              function: 20,
              text: Some(Item::List(vec![])),
            }
          ).join().unwrap().unwrap();
        },
        _ => {break},
      }
    }
  });
  // TX
  let tx_client: Arc<HsmsClient> = client.clone();
  let tx_thread: JoinHandle<()> = thread::spawn(move || {
    let mut system: u32 = 0;
    loop {
      //LINK TEST
      let link_result: Result<(), ConnectionStateTransition> = tx_client.linktest(system).join().unwrap();
      system += 1;
      println!("EQUIPMENT LINK TEST {:?}", link_result);
      if link_result.is_err() {break}
      if system == 10 || link_result.is_err() {break}
      thread::sleep(Duration::from_secs(1));
    }
    tx_client.disconnect();
  });
  rx_thread.join().unwrap();
  tx_thread.join().unwrap();
}

fn test_host() {
  // CLIENT
  let parameter_settings: ParameterSettings = ParameterSettings {
    connect_mode: ConnectionMode::Active,
    ..Default::default()
  };
  let client: Arc<HsmsClient> = HsmsClient::new(parameter_settings);
  let _ = client.connect("127.0.0.1:5000").unwrap();
  thread::sleep(Duration::from_millis(2000));
  let mut system: u32 = 0;
  client.select(HsmsMessageID{session: 0, system}).join().unwrap().unwrap();
  system += 1;
  loop {
    let data_result: Result<Option<Message>, ConnectionStateTransition> = client.data(
      HsmsMessageID {
        session: 0,
        system,
      },
      Message {
        stream: 1,
        function: 13,
        w: true,
        text: None,
      }
    ).join().unwrap();
    println!("HOST DATA TEST {:?}", data_result);
    if data_result.is_err() {break}
    system += 1;
    thread::sleep(Duration::from_secs(1));
  }
  client.disconnect();
}
