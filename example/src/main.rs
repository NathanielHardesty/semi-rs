#![feature(ascii_char)]
#![feature(ascii_char_variants)]

use std::{ascii::Char::*, sync::{mpsc::Receiver, Arc}, thread::{self, JoinHandle}, time::Duration};
use hsms::{ConnectionMode, ConnectionStateTransition, GenericClient, ParameterSettings};
use secs_ii::{Message, Item};

fn main() {
  test_host();
}

fn test_host() {
  // CLIENT
  let mut parameter_settings: ParameterSettings = ParameterSettings::default();
  parameter_settings.connect_mode = ConnectionMode::Active;
  let client: Arc<GenericClient> = GenericClient::new(parameter_settings);
  let rx_message: Receiver<Message> = client.connect("127.0.0.1:5000").unwrap();
  thread::sleep(Duration::from_millis(5000));
  client.select(0).join().unwrap().unwrap();
  loop {
    thread::sleep(Duration::from_secs(2));
    let link_result: Result<(), ConnectionStateTransition> = client.linktest().join().unwrap();
    println!("LINK TEST {:?}", link_result);
    if link_result.is_err() {break}
  }
}

fn test_equipment() {
  // MESSAGE TEST
  println!("{:?}", Into::<Vec<u8>>::into(
    Into::<secs_ii::Item>::into(
      secs_ii::items::AnyBinaryString::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap()
    )
  ));
  println!("{:?}", Into::<secs_ii::Message>::into(
    secs_ii::messages::s1::OnLineAck(secs_ii::items::OnLineAcknowledge::Accepted)
  ));
  // CLIENT
  let parameter_settings: ParameterSettings = ParameterSettings::default();
  let client: Arc<GenericClient> = GenericClient::new(parameter_settings);
  // RX
  let rx_message: Receiver<Message> = client.connect("127.0.0.1:5000").unwrap();
  let rx_client: Arc<GenericClient> = client.clone();
  let rx_thread: JoinHandle<()> = thread::spawn(move || {  
    for data_message in rx_message {
      match (data_message.w, data_message.stream, data_message.function) {
        (true, 1, 3) => {
          rx_client.data(Message {
            stream: 1,
            function: 4,
            w: false,
            text: Some(Item::List(vec![])),
          }).join().unwrap().unwrap();
        },
        (true, 1, 11) => {
          rx_client.data(Message {
            stream: 1,
            function: 12,
            w: false,
            text: Some(Item::List(vec![])),
          }).join().unwrap().unwrap();
        },
        (true, 1, 13) => {
          rx_client.data(Message {
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
          }).join().unwrap().unwrap();
        },
        (true, 1, 17) => {
          rx_client.data(Message {
            w: false,
            stream: 1,
            function: 18,
            text: Some(Item::Binary(vec![0])),
          }).join().unwrap().unwrap();
        },
        (true, 1, 21) => {
          rx_client.data(Message {
            w: false,
            stream: 1,
            function: 22,
            text: Some(Item::List(vec![])),
          }).join().unwrap().unwrap();
        },
        (true, 1, 23) => {
          rx_client.data(Message {
            w: false,
            stream: 1,
            function: 24,
            text: Some(Item::List(vec![])),
          }).join().unwrap().unwrap();
        },
        (true, 2, 13) => {
          rx_client.data(Message {
            w: false,
            stream: 2,
            function: 14,
            text: Some(Item::List(vec![])),
          }).join().unwrap().unwrap();
        },
        (true, 2, 29) => {
          rx_client.data(Message {
            w: false,
            stream: 2,
            function: 30,
            text: Some(Item::List(vec![])),
          }).join().unwrap().unwrap();
        },
        (true, 5, 5) => {
          rx_client.data(Message {
            w: false,
            stream: 5,
            function: 6,
            text: Some(Item::List(vec![])),
          }).join().unwrap().unwrap();
        },
        (true, 5, 7) => {
          rx_client.data(Message {
            w: false,
            stream: 5,
            function: 8,
            text: Some(Item::List(vec![])),
          }).join().unwrap().unwrap();
        },
        (true, 7, 19) => {
          rx_client.data(Message {
            w: false,
            stream: 7,
            function: 20,
            text: Some(Item::List(vec![])),
          }).join().unwrap().unwrap();
        },
        _ => {break},
      }
    }
  });
  // TX
  let tx_client: Arc<GenericClient> = client.clone();
  let tx_thread: JoinHandle<()> = thread::spawn(move || {
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
      thread::sleep(Duration::from_secs(2));
      let link_result: Result<(), ConnectionStateTransition> = tx_client.linktest().join().unwrap();
      println!("LINK TEST {:?}", link_result);
      if link_result.is_err() {break}
    }
  });
  rx_thread.join().unwrap();
  tx_thread.join().unwrap();
}
