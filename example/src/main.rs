#![feature(ascii_char)]
#![feature(ascii_char_variants)]

use std::{ascii::Char::*, sync::{mpsc::Receiver, Arc}, thread::{self, JoinHandle}, time::Duration};
use semi_e5::{items::StatusVariableValue, messages::s1::SelectedEquipmentStatusData, Item, Message};
use semi_e37::{ConnectionMode, ConnectionStateTransition, HsmsClient, HsmsMessageID, ParameterSettings};

fn main() {
  test_data();
  let equipment = thread::spawn(|| {test_equipment();});
  let host = thread::spawn(|| {test_host();});
  let _ = equipment.join();
  let _ = host.join();
}

fn test_data() {
  println!("{:?}", Item::try_from(vec![1, 1, 177, 4, 0, 0, 7, 237]))
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
          match semi_e5::messages::s1::SelectedEquipmentStatusRequest::try_from(message) {
            Ok(s1f3) => {
              let mut vec = vec![];
              for _status_variable in s1f3.0.0 {
                vec.push(StatusVariableValue::List(vec![Item::Unsigned4(vec![10])]));
              }
              rx_client.data(
                id,
                SelectedEquipmentStatusData(semi_e5::items::VecList(vec)).into()
              ).join().unwrap().unwrap();
            },
            Err(_) => todo!(),
          }
        },
        (true, 1, 11) => {
          rx_client.data(
            id,
            Message {
              stream: 1,
              function: 12,
              w: false,
              text: Some(Item::List(vec![])),
            }
          ).join().unwrap().unwrap();
        },
        (true, 1, 13) => {
          rx_client.data(
            id,
            Message {
              w: false,
              stream: 1,
              function: 14,
              text: Some(Item::List(vec![
                Item::Binary(vec![0]),
                Item::List(vec![
                  Item::Ascii(vec![CapitalT, SmallE, SmallS, SmallT]),
                  Item::Ascii(vec![Digit0, Digit1, Digit0]),
                ]),
              ])),
            }
          ).join().unwrap().unwrap();
        },
        (true, 1, 17) => {
          rx_client.data(
            id,
            Message {
              w: false,
              stream: 1,
              function: 18,
              text: Some(Item::Binary(vec![0])),
            }
          ).join().unwrap().unwrap();
        },
        (true, 1, 21) => {
          rx_client.data(
            id,
            Message {
              w: false,
              stream: 1,
              function: 22,
              text: Some(Item::List(vec![])),
            }
          ).join().unwrap().unwrap();
        },
        (true, 1, 23) => {
          rx_client.data(
            id,
            Message {
              w: false,
              stream: 1,
              function: 24,
              text: Some(Item::List(vec![])),
            }
          ).join().unwrap().unwrap();
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
