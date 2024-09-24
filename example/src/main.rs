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

use std::{ascii::Char::*, io::Error, sync::Arc, thread::{self, JoinHandle}, time::Duration};
use semi_e5::{Item, Message, items::*, messages::*};
use semi_e37::generic::{ConnectionMode, Client, MessageID, ParameterSettings};

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
  let equipment_client: Arc<Client> = Client::new(parameter_settings);
  // MAIN LOOP
  let mut system: u32 = 0x1000;
  loop {
    // LINK TEST
    let link_result: Result<(), Error> = equipment_client.linktest(system).join().unwrap();
    println!("equipment_client.linktest({:>8X}) : {:?}", system, link_result);
    if link_result.is_err() {
      // CONNECT
      let (socket, rx_message) = equipment_client.connect("127.0.0.1:5000").unwrap();
      println!("equipment_client.connect            : {:?}", socket);
      // SPAWN RX THREAD
      let equipment_rx: Arc<Client> = equipment_client.clone();
      let _rx_thread: JoinHandle<()> = thread::spawn(move || {
        for (id, request) in rx_message {
          println!("equipment_rx request                : {:?}", request);
          let response: Message = match (request.w, request.stream, request.function) {
            (true, 1, 1) => {
              match s1::AreYouThere::try_from(request) {
                Ok(_) => {
                  s1::OnLineDataEquipment((
                    ModelName::new(b"SEMI-RS".as_ascii().unwrap().to_vec()).unwrap(),
                    SoftwareRevision::new(b"010".as_ascii().unwrap().to_vec()).unwrap(),
                  )).into()
                },
                Err(_) => s1::Abort.into(),
              }
            }
            (true, 1, 3) => {
              match s1::SelectedEquipmentStatusRequest::try_from(request) {
                Ok(s1f3) => {
                  let mut vec = vec![];
                  for _svid in s1f3.0.0 {
                    vec.push(StatusVariableValue::List(vec![Item::u4(10)]));
                  }
                  s1::SelectedEquipmentStatusData(VecList(vec)).into()
                },
                Err(_) => s1::Abort.into(),
              }
            }
            (true, 1, 11) => {
              match s1::StatusVariableNamelistRequest::try_from(request) {
                Ok(s1f11) => {
                  let mut vec = vec![];
                  for svid in s1f11.0.0 {
                    vec.push((svid, StatusVariableName(vec![]), Units(vec![])));
                  }
                  s1::StatusVariableNamelistReply(VecList(vec)).into()
                },
                Err(_) => s1::Abort.into(),
              }
            }
            (true, 1, 13) => {
              match s1::HostCR::try_from(request) {
                Ok(_s1f13) => {
                  s1::EquipmentCRA((
                    CommAck::Accepted, (
                      ModelName::new(b"SEMI-RS".as_ascii().unwrap().to_vec()).unwrap(),
                      SoftwareRevision::new(b"010".as_ascii().unwrap().to_vec()).unwrap(),
                    )
                  )).into()
                },
                Err(_) => s1::Abort.into(),
              }
            }
            (true, 1, 17) => {
              match s1::RequestOnLine::try_from(request) {
                Ok(_s1f17) => {
                  s1::OnLineAck(OnLineAcknowledge::Accepted).into()
                },
                Err(_) => s1::Abort.into(),
              }
            }
            (true, 1, 21) => {
              match s1::DataVariableNamelistRequest::try_from(request) {
                Ok(s1f21) => {
                  let mut vec = vec![];
                  for vid in s1f21.0.0 {
                    vec.push((vid, DataVariableValueName(vec![]), Units(vec![])));
                  }
                  s1::DataVariableNamelist(VecList(vec)).into()
                },
                Err(_) => s1::Abort.into(),
              }
            }
            (true, 1, 23) => {
              match s1::CollectionEventNamelistRequest::try_from(request) {
                Ok(s1f11) => {
                  let mut vec = vec![];
                  for ceid in s1f11.0.0 {
                    vec.push((ceid, CollectionEventName(vec![]), VecList(vec![])));
                  }
                  s1::CollectionEventNamelist(VecList(vec)).into()
                },
                Err(_) => s1::Abort.into(),
              }
            }
            (true, 2, 13) => {
              match s2::EquipmentConstantRequest::try_from(request) {
                Ok(s2f13) => {
                  let mut vec = vec![];
                  for _ecid in s2f13.0.0 {
                    vec.push(OptionItem::<EquipmentConstantValue>(None));
                  }
                  s2::EquipmentConstantData(VecList(vec)).into()
                }
                Err(_) => s2::Abort.into(),
              }
            }
            (true, 2, 29) => {
              match s2::EquipmentConstantNamelistRequest::try_from(request) {
                Ok(s2f29) => {
                  let mut vec = vec![];
                  for ecid in s2f29.0.0 {
                    vec.push((
                      ecid,
                      EquipmentConstantName(vec![]),
                      EquipmentConstantMinimumValue::Ascii(vec![]),
                      EquipmentConstantMaximumValue::Ascii(vec![]),
                      EquipmentConstantDefaultValue::Ascii(vec![]),
                      Units(vec![])
                    ))
                  }
                  s2::EquipmentConstantNamelist(VecList(vec)).into()
                },
                Err(_) => s2::Abort.into(),
              }
            }
            (true, 5, 5) => {
              Message {
                w: false,
                stream: 5,
                function: 6,
                text: Some(Item::List(vec![])),
              }
            }
            (true, 5, 7) => {
              Message {
                w: false,
                stream: 5,
                function: 8,
                text: Some(Item::List(vec![])),
              }
            }
            (true, 7, 19) => {
              Message {
                w: false,
                stream: 7,
                function: 20,
                text: Some(Item::List(vec![])),
              }
            }
            _ => {break}
          };
          println!("equipment_rx response               : {:?}", response.clone());
          println!("equipment_rx.data                   : {:?}", equipment_rx.data(id, response).join().unwrap());
        }
      });
    }
    system += 1;
    if system == 0x1020 {break}
    thread::sleep(Duration::from_secs(1));
  }
  println!("equipment_client.separate           : {:?}", equipment_client.separate(MessageID {system, session: 0xFFFF}).join().unwrap());
  println!("equipment_client.disconnect         : {:?}", equipment_client.disconnect());
}

fn test_host() {
  // CLIENT
  let parameter_settings: ParameterSettings = ParameterSettings {
    connect_mode: ConnectionMode::Active,
    ..Default::default()
  };
  let host_client: Arc<Client> = Client::new(parameter_settings);
  // CONNECT
  let (socket, _) = host_client.connect("127.0.0.1:5000").unwrap();
  println!("host_client.connect                 : {:?}", socket);
  thread::sleep(Duration::from_millis(2000));
  let mut system: u32 = 0;
  // SELECT
  println!("host_client.select                  : {:?}", host_client.select(MessageID{session: 0, system}).join().unwrap());
  system += 1;
  // DATA LOOP
  loop {
    let data_result: Result<Option<Message>, Error> = host_client.data(
      MessageID {
        session: 0,
        system,
      },
      Message {
        stream: 1,
        function: 1,
        w: true,
        text: None,
      }
    ).join().unwrap();
    println!("host_client.data                    : {:?}", data_result);
    if data_result.is_err() {break}
    system += 1;
    thread::sleep(Duration::from_secs(1));
  }
  println!("host_client.linktest                : {:?}", host_client.linktest(system).join().unwrap());
  println!("host_client.disconnect              : {:?}", host_client.disconnect());
}
