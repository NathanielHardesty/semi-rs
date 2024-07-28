#![feature(ascii_char)]
#![feature(ascii_char_variants)]

use std::{ascii::Char::*, sync::{mpsc::Receiver, Arc}, thread::{self, JoinHandle}, time::Duration};
use hsms::{ConnectionMode, ConnectionStateTransition, HsmsClient, HsmsMessageID, ParameterSettings};
use secs_ii::{Message, Item};

fn main() {
  let equipment = thread::spawn(|| {test_equipment();});
  let host = thread::spawn(|| {test_host();});
  let _ = equipment.join();
  let _ = host.join();
}

fn test_equipment() {
  // CLIENT
  let parameter_settings: ParameterSettings = ParameterSettings::default();
  let client: Arc<HsmsClient> = HsmsClient::new(parameter_settings);
  // RX
  let rx_message: Receiver<(HsmsMessageID, Message)> = client.connect("127.0.0.1:5000").unwrap();
  let rx_client: Arc<HsmsClient> = client.clone();
  let rx_thread: JoinHandle<()> = thread::spawn(move || {  
    for (id, data) in rx_message {
      match (data.w, data.stream, data.function) {
        (true, 1, 3) => {
          rx_client.data(
            id,
            Message {
              stream: 1,
              function: 4,
              w: false,
              text: Some(Item::List(vec![])),
            }
          ).join().unwrap().unwrap();
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
      //DATA TEST
      /*thread::sleep(Duration::from_secs(5));
      let data_result: Result<Option<HsmsMessage>, ConnectionStateTransition> = tx_client.data(DataMessage {
        session_id: 0,
        w: true,
        stream: 1,
        function: 1,
        system: 0xFFFF,
        text: vec![],
      }).join().unwrap();
      println!("DATA TEST {:?}", data_result);
      if let Err(_) = data_result {break}*/
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
  thread::sleep(Duration::from_millis(5000));
  let mut system: u32 = 0;
  client.select(HsmsMessageID{session: 0, system}).join().unwrap().unwrap();
  system += 1;
  loop {
    let link_result: Result<(), ConnectionStateTransition> = client.linktest(system).join().unwrap();
    system += 1;
    println!("HOST LINK TEST {:?}", link_result);
    if link_result.is_err() {break}
    //if system == 10 || link_result.is_err() {break}
    thread::sleep(Duration::from_secs(1));
  }
  client.disconnect();
}
