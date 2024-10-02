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

//! # SINGLE SELECTED SESSION SERVICES

pub use crate::primitive::ConnectionMode;
pub use crate::generic::ParameterSettings;
pub use crate::generic::Message;
pub use crate::generic::MessageID;
pub use crate::generic::MessageContents;
pub use crate::generic::RejectReason;

use crate::generic;
use crate::generic::SelectStatus;
use crate::generic::DeselectStatus;
use crate::generic::ProcedureCallbacks;
use std::io::Error;
use std::net::SocketAddr;
use std::sync::mpsc::Receiver;
use std::sync::Arc;

pub struct Client {
  generic_client: Arc<generic::Client>,
}

// Connection procedures
impl Client {
  pub fn new(
    parameter_settings: ParameterSettings,
  ) -> Arc<Self> {
    let passive_callbacks: ProcedureCallbacks = ProcedureCallbacks {
      select: Arc::new(|session_id, selection_count| -> SelectStatus {
        // In HSMS-SS, only a single session may be initiated.
        if selection_count == 0 {
          // In HSMS-SS, only a Session ID of 0xFFFF is valid.
          if session_id == 0xFFFF {
            SelectStatus::Ok
          } else {
            SelectStatus::NotReady
          }
        } else {
          SelectStatus::AlreadyActive
        }
      }),
      deselect: Arc::new(|_session_id, _selection_count| -> DeselectStatus {
        // In HSMS-SS, the Deselect Procedure is forbidden.
        DeselectStatus::Busy
      }),
      separate: Arc::new(|session_id, _selection_count| -> bool {
        // In HSMS-SS, only a Session ID of 0xFFFF is valid.
        session_id == 0xFFFF
      }),
    };
    let active_callbacks: ProcedureCallbacks = ProcedureCallbacks {
      select: Arc::new(|_session_id, _selection_count| -> SelectStatus {
        // In HSMS-SS, only the Host may initiate the Select Procedure.
        SelectStatus::NotReady
      }),
      deselect: Arc::new(|_session_id, _selection_count| -> DeselectStatus {
        // In HSMS-SS, the Deselect Procedure is forbidden.
        DeselectStatus::Busy
      }),
      separate: Arc::new(|session_id, _selection_count| -> bool {
        // In HSMS-SS, only a Session ID of 0xFFFF is valid.
        session_id == 0xFFFF
      }),
    };
    Arc::new(Client {
      generic_client: generic::Client::new(
        parameter_settings,
        match parameter_settings.connect_mode {
          ConnectionMode::Passive => passive_callbacks,
          ConnectionMode::Active => active_callbacks,
        },
      )
    })
  }

  pub fn connect(
    self: &Arc<Self>,
    entity: &str,
  ) -> Result<(SocketAddr, Receiver<(MessageID, semi_e5::Message)>), Error> {
    let connection = self.generic_client.connect(entity)?;
    match self.generic_client.parameter_settings.connect_mode {
      ConnectionMode::Passive => todo!(),
      ConnectionMode::Active => {
        self.generic_client.select(MessageID {
          session: 0xFFFF,
          system: 0,
        }).join().unwrap()?;
      },
    }

    Ok(connection)
  }
}
