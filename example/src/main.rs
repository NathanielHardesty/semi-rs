
use std::{sync::{mpsc::Receiver, Arc}, thread::{self, JoinHandle}, time::Duration};
use hsms::{ConnectionStateTransition, DataMessage, GenericClient, ParameterSettings};

fn main() {
  //MESSAGE TEST
  println!("{:?}", Into::<Vec<u8>>::into(
    Into::<secs_ii::Item>::into(
      secs_ii::items::AnyBinaryString::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap()
    )
  ));
  //CLIENT
  let parameter_settings: ParameterSettings = ParameterSettings::default();
  let client: Arc<GenericClient> = GenericClient::new(parameter_settings);
  //RX
  let rx_message: Receiver<DataMessage> = client.connect("127.0.0.1:5000").unwrap();
  let rx_client: Arc<GenericClient> = client.clone();
  let rx_thread: JoinHandle<()> = thread::spawn(move || {  
    for data_message in rx_message {
      match (data_message.w, data_message.stream, data_message.function) {
        (true, 1, 3) => {
          rx_client.data(DataMessage {
            session_id: data_message.session_id,
            w: false,
            stream: 1,
            function: 4,
            system: data_message.system,
            text: vec![1, 0],
          }).join().unwrap().unwrap();
        },
        (true, 1, 11) => {
          rx_client.data(DataMessage {
            session_id: data_message.session_id,
            w: false,
            stream: 1,
            function: 12,
            system: data_message.system,
            text: vec![1, 0],
          }).join().unwrap().unwrap();
        },
        (true, 1, 13) => {
          rx_client.data(DataMessage {
            session_id: data_message.session_id,
            w: false,
            stream: 1,
            function: 14,
            system: data_message.system,
            text: vec![1, 2, 33, 1, 0, 1, 2, 65, 4, b't', b'e', b's', b't', 65, 3, b'0', b'1', b'0'],
          }).join().unwrap().unwrap();
        },
        (true, 1, 17) => {
          rx_client.data(DataMessage {
            session_id: data_message.session_id,
            w: false,
            stream: 1,
            function: 18,
            system: data_message.system,
            text: vec![33, 1, 0],
          }).join().unwrap().unwrap();
        },
        (true, 1, 21) => {
          rx_client.data(DataMessage {
            session_id: data_message.session_id,
            w: false,
            stream: 1,
            function: 22,
            system: data_message.system,
            text: vec![1, 0],
          }).join().unwrap().unwrap();
        },
        (true, 1, 23) => {
          rx_client.data(DataMessage {
            session_id: data_message.session_id,
            w: false,
            stream: 1,
            function: 24,
            system: data_message.system,
            text: vec![1, 0],
          }).join().unwrap().unwrap();
        },
        (true, 2, 13) => {
          rx_client.data(DataMessage {
            session_id: data_message.session_id,
            w: false,
            stream: 2,
            function: 14,
            system: data_message.system,
            text: vec![1, 0],
          }).join().unwrap().unwrap();
        },
        (true, 2, 29) => {
          rx_client.data(DataMessage {
            session_id: data_message.session_id,
            w: false,
            stream: 2,
            function: 30,
            system: data_message.system,
            text: vec![1, 0],
          }).join().unwrap().unwrap();
        },
        (true, 5, 5) => {
          rx_client.data(DataMessage {
            session_id: data_message.session_id,
            w: false,
            stream: 5,
            function: 6,
            system: data_message.system,
            text: vec![1, 0],
          }).join().unwrap().unwrap();
        },
        (true, 5, 7) => {
          rx_client.data(DataMessage {
            session_id: data_message.session_id,
            w: false,
            stream: 5,
            function: 8,
            system: data_message.system,
            text: vec![1, 0],
          }).join().unwrap().unwrap();
        },
        (true, 7, 19) => {
          rx_client.data(DataMessage {
            session_id: data_message.session_id,
            w: false,
            stream: 7,
            function: 20,
            system: data_message.system,
            text: vec![1, 0],
          }).join().unwrap().unwrap();
        },
        _ => {
          break
        },
      }
    }
  });
  //TX
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
