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

//! # STREAM 1: EQUIPMENT STATUS
//! **Based on SEMI E5§10.5**
//! 
//! ---------------------------------------------------------------------------
//! 
//! [Message]s which deal with exchanging information about the status of the
//! equipment, including its current mode, depletion of various consumable
//! items, and the status of transfer operations.
//! 
//! [Message]: crate::Message

use crate::*;
use crate::Error::*;
use crate::items::*;

/// ## S1F0
/// 
/// **Abort Transaction**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <-> EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Used in lieu of an expected reply to abort a transaction.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// Header only.
pub struct Abort;
message_headeronly!{Abort, false, 1, 0}

/// ## S1F1
/// 
/// **Are You There Request (R)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <-> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Establishes if the equipment is on-line. A function 0 response to this
/// message is equivalent to receiving a timeout on the receive timer.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// Header only.
pub struct AreYouThere;
message_headeronly!{AreYouThere, true, 1, 1}

/// ## S1F2H
/// 
/// **On Line Data (D)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Data signifying the equipment is alive.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 0
pub struct OnLineDataHost(pub ());
message_data!{OnLineDataHost, false, 1, 2}

/// ## S1F2E
/// 
/// **On Line Data (D)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Data signifying the equipment is alive.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [MDLN]
///    2. [SOFTREV]
/// 
/// [MDLN]:    ModelName
/// [SOFTREV]: SoftwareRevision
pub struct OnLineDataEquipment(pub (ModelName, SoftwareRevision));
message_data!{OnLineDataEquipment, false, 1, 2}

/// ## S1F3
/// 
/// **Selected Equipment Status Request (SSR)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// A request to the equipment to report selected values of its status.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - n
///    - [SVID]
/// 
/// A zero-length list means to report all SVIDs.
/// 
/// [SVID]: StatusVariableID
pub struct SelectedEquipmentStatusRequest(pub VecList<StatusVariableID>);
message_data!{SelectedEquipmentStatusRequest, true, 1, 3}

/// ## S1F4
/// 
/// **Selected Equipment Status Data (SSD)**
/// 
/// - **MULTI-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// The equipment reports the value of each [SVID] requested in the order
/// requested.
/// 
/// The host must remember the names of the values it requested.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - n
///    - [SV]
/// 
/// A zero-length item for a given [SV] means that the [SVID] does not
/// exist.
/// 
/// [SV]:   StatusVariableValue
/// [SVID]: StatusVariableID
pub struct SelectedEquipmentStatusData(pub VecList<StatusVariableValue>);
message_data!{SelectedEquipmentStatusData, false, 1, 4}

/// ## S1F5
/// 
/// **Formatted Status Request (FSR)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// A request for the equipment to report the status according to a
/// predefined fixed format.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [SFCD]
/// 
/// [SFCD]: StatusFormCode
pub struct FormattedStatusRequest(pub StatusFormCode);
message_data!{FormattedStatusRequest, true, 1, 5}

/// ## S1F6
/// 
/// **Formatted Status Data (FSD)**
/// 
/// - **MULTI-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// The value of status variables according to the [SFCD].
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// Depends on the structure specified by the status form.
/// 
/// A zero-length item means that no report can be made.
/// 
/// [SFCD]: StatusFormCode
pub struct FormattedStatusData(pub Item);
message_item!{FormattedStatusData, false, 1, 6}

/// ## S1F7
/// 
/// **Fixed Form Request (FFR)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// A request for the form used in [S1F6].
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [SFCD]
/// 
/// [S1F6]: FormattedStatusData
/// [SFCD]: StatusFormCode
pub struct FixedFormRequest(pub StatusFormCode);
message_data!{FixedFormRequest, true, 1, 7}

/// ## S1F8
/// 
/// **Fixed Form Data (FFD)**
/// 
/// - **MULTI-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// The form is returned with the name of each value and the data format
/// item having a zero length as a two-element list in the place of each
/// single item to be returned in [S1F6].
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// Depends on the form being specified.
/// 
/// A zero-length item means the form is unavailable.
/// 
/// [S1F6]: FormattedStatusData
pub struct FixedFormData(pub Item);
message_item!{FixedFormData, false, 1, 8}

/// ## S1F9
/// 
/// **Material Transfer Status Request (TSR)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// A request to report the status of all material ports.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// Header only.
pub struct MaterialTransferStatusRequest;
message_headeronly!{MaterialTransferStatusRequest, true, 1, 9}

/// ## S1F10
/// 
/// **Material Transfer Status data (TSD)**
/// 
/// - **MULTI-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// The transfer status of all material ports.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. <[TSIP]...>
///    2. <[TSOP]...>
/// 
/// A zero length item means there are no such ports.
/// A zero length list means there are no ports.
/// 
/// [TSIP]: TransferStatusInputPort
/// [TSOP]: TransferStatusOutputPort
pub struct MaterialTransferStatusData(pub OptionList<(Vec<TransferStatusInputPort>, Vec<TransferStatusOutputPort>)>);
message_data!{MaterialTransferStatusData, false, 1, 10}

/// ## S1F11
/// 
/// **Status Variable Namelist Request (SVNR)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// A request to identify certain status variables.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - N
///    - [SVID]
/// 
/// N is the number of status variables requested.
/// Zero-length N is a request to report all [SVID]s.
/// 
/// [SVID]: StatusVariableID
pub struct StatusVariableNamelistRequest(pub VecList<StatusVariableID>);
message_data!{StatusVariableNamelistRequest, true, 1, 11}

/// ## S1F12
/// 
/// **Status Variable Namelist Reply (SVNRR)**
/// 
/// - **MULTI-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// The name and units of the requested status variables.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - N
///    - List - 3
///       1. [SVID]
///       2. [SVNAME]
///       3. [UNITS]
/// 
/// N is the number of status variables requested.
/// Zero length items for both [SVNAME] and [UNITS] indicates that the
/// [SVID] does not exist.
/// 
/// [SVID]:   StatusVariableID
/// [SVNAME]: StatusVariableName
/// [UNITS]:  Units
pub struct StatusVariableNamelistReply(pub VecList<(StatusVariableID, StatusVariableName, Units)>);
message_data!{StatusVariableNamelistReply, false, 1, 12}

/// ## S1F13H
/// 
/// **Establish Communications Request (CR)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// The purpose of this message is to provide a formal means of
/// initializing communications at a logical application level both on
/// power-up and following a break in communications.
/// 
/// It should follow any period where host and equipment SECS applications
/// are unable to communicate.
/// 
/// An attempt to send an Establish Communications Request ([S1F13])
/// should be repeated at programmable intervals until an Establish
/// Communications Acknowledge ([S1F14]) is received within the
/// transaction timeout period with an acknowledgement code accepting the
/// establishment.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 0
/// 
/// [S1F13]: HostCR
/// [S1F14]: EquipmentCRA
pub struct HostCR(pub ());
message_data!{HostCR, true, 1, 13}

/// ## S1F13E
/// 
/// **Establish Communications Request (CR)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// The purpose of this message is to provide a formal means of
/// initializing communications at a logical application level both on
/// power-up and following a break in communications.
/// 
/// It should follow any period where host and equipment SECS applications
/// are unable to communicate.
/// 
/// An attempt to send an Establish Communications Request ([S1F13])
/// should be repeated at programmable intervals until an Establish
/// Communications Acknowledge ([S1F14]) is received within the
/// transaction timeout period with an acknowledgement code accepting the
/// establishment.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    - [MDLN]
///    - [SOFTREV]
/// 
/// [S1F13]:   EquipmentCR
/// [S1F14]:   HostCRA
/// [MDLN]:    ModelName
/// [SOFTREV]: SoftwareRevision
pub struct EquipmentCR(pub (ModelName, SoftwareRevision));
message_data!{EquipmentCR, true, 1, 13}

/// ## S1F14H
/// 
/// **Establish Communications Request Acknowledge (CRA)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Accept or deny Establish Communications Request ([S1F13]).
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [COMMACK]
///    2. List - 0
/// 
/// [S1F13]:   EquipmentCR
/// [COMMACK]: CommAck
pub struct HostCRA(pub (CommAck, ()));
message_data!{HostCRA, false, 1, 14}

/// ## S1F14E
/// 
/// **Establish Communications Request Acknowledge (CRA)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Accept or deny Establish Communications Request ([S1F13]).
/// 
/// [MDLN] and [SOFTREV] are on-line data and are valid only if
/// [COMMACK] = 0.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [COMMACK]
///    2. List - 2
///       1. [MDLN]
///       2. [SOFTREV]
/// 
/// [S1F13]:   HostCR
/// [COMMACK]: CommAck
/// [MDLN]:    ModelName
/// [SOFTREV]: SoftwareRevision
pub struct EquipmentCRA(pub (CommAck, (ModelName, SoftwareRevision)));
message_data!{EquipmentCRA, false, 1, 14}

/// ## S1F15
/// 
/// **Request OFF-LINE (ROFL)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// The host requirests that the equipment transition to the OFF-LINE
/// state.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// Header only.
pub struct RequestOffLine;
message_headeronly!{RequestOffLine, true, 1, 15}

/// ## S1F16
/// 
/// **OFF-LINE Acknowledge (OFLA)**
///  
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Acknowledge or error.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// [OFLACK]
/// 
/// [OFLACK]: OffLineAcknowledge
pub struct OffLineAck(pub OffLineAcknowledge);
message_data!{OffLineAck, false, 1, 16}

/// ## S1F17
/// 
/// **Request ON-LINE (RONL)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// The host requests that the equipment transition to the ON-LINE state.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// Header only.
pub struct RequestOnLine;
message_headeronly!{RequestOnLine, true, 1, 17}

/// ## S1F18
/// 
/// **ON-LINE Acknowledge (ONLA)**
///  
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Acknowledge or error.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// [ONLACK]
/// 
/// [ONLACK]: OnLineAcknowledge
pub struct OnLineAck(pub OnLineAcknowledge);
message_data!{OnLineAck, false, 1, 16}

/// ## S1F19
/// 
/// **Get Attribute (GA)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <-> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Request for attribute data relating to the specified object or entity
/// within the equipment.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 3
///    1. [OBJTYPE]
///    2. List - M
///       - [OBJID]
///    3. List - N
///       - [ATTRID]
/// 
/// M is the number of objects for which attributes are requested.
/// Zero-length M is a request for all objects of the specified type.
/// 
/// N is the number of attributes requested for each object.
/// Zero-length N is a request for all attributes.
/// 
/// [OBJTYPE]: ObjectType
/// [OBJID]:   ObjectID
/// [ATTRID]:  AttributeID
pub struct GetAttribute(pub (ObjectType, VecList<ObjectID>, VecList<AttributeID>));
message_data!{GetAttribute, true, 1, 19}

/// ## S1F20
/// 
/// **Attribute Data (AD)**
/// 
/// - **MULTI-BLOCK**
/// - **HOST <-> EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Requested object attributes.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. List - M
///       - List - N
///          - [ATTRDATA]
///    2. List - P
///       - List - 2
///          1. [ERRCODE]
///          2. [ERRTEXT]
/// 
/// M is the number of objects for which attributes are being returned.
/// Zero-length M indicates that the specified [OBJTYPE] is unknown.
/// 
/// N is the number of attributes requested for each object.
/// Zero-length N indicates that the corresponding object was not found.
/// 
/// Zero-length [ATTRDATA] item indicates that the specified [ATTRID] is
/// unknown.
/// 
/// P is the number of errors reported.
/// Zero-length P indicates no errors were found.
/// 
/// [ATTRDATA]: AttributeValue
/// [ERRCODE]:  ErrorCode
/// [ERRTEXT]:  ErrorText
/// [OBJTYPE]:  ObjectType
/// [ATTRID]:   AttributeID
pub struct AttributeData(pub (VecList<VecList<AttributeValue>>, VecList<(ErrorCode, ErrorText)>));
message_data!{AttributeData, false, 1, 20}

/// ## S1F21
/// 
/// **Data Variable Namelist Request (DVNR)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Request basic information about data variables.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - N
///    - [VID]
/// 
/// N is the number of requested data variables.
/// 
/// [VID]s are limited to those of 'DVVAL' class variables only.
/// 
/// [VID]: VariableID
pub struct DataVariableNamelistRequest(pub VecList<VariableID>);
message_data!{DataVariableNamelistRequest, true, 1, 21}

/// ## S1F22
/// 
/// **Data Variable Namelist (DVN)**
/// 
/// - **MULTI-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Report information of the [VID]s requested by [S1F21].
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - N
///    - List - 3
///       1. [VID]
///       2. [DVVALNAME]
///       3. [UNITS]
/// 
/// N is the number of requested data variables.
/// 
/// [VID]s are limited to those of 'DVVAL' class variables only.
/// 
/// Zero-length ASCII items for [DVVALNAME] and [UNITS] indicates that the
/// [VID] does not exist or is not the identifier of a 'DVVAL' class
/// variable.
/// 
/// [S1F21]:     DataVariableNamelistRequest
/// [VID]:       VariableID
/// [DVVALNAME]: DataVariableValueName
/// [UNITS]:     Units
pub struct DataVariableNamelist(pub VecList<(VariableID, DataVariableValueName, Units)>);
message_data!{DataVariableNamelist, false, 1, 22}

/// ## S1F23
/// 
/// **Collection Event Namelist Request (CENR)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Retrieve information about what collection event IDs are available and
/// which data values are valid for each collection event.
/// 
/// ---------------------------------------------------------------------------
/// 
/// - List - N
///    - [CEID]
/// 
/// N is the number of requested [CEID]s.
/// Zero-length N is a request for to send information for all [CEID]s.
/// 
/// [CEID]: CollectionEventID
pub struct CollectionEventNamelistRequest(pub VecList<CollectionEventID>);
message_data!{CollectionEventNamelistRequest, true, 1, 23}

/// ## S1F24
/// 
/// **Collection Event Namelist (CEN)**
/// 
/// - **MULTI-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Information of the collection events and associated [VID]s of the
/// [CEID]s. A listed [VID] can be conditionally or unconditionally
/// associated with the [CEID]; it is the responsibility of the equipment
/// supplier to document whether conditional [VID]s are reported.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - N
///    - List - 3
///       1. [CEID]
///       2. [CENAME]
///       3. List - A
///          - [VID]
/// 
/// N is the number of requested [CEID]s.
/// 
/// A is the number of associated [VID]s.
/// 
/// [VID]s are limited to those of 'DVVAL' class variables only.
/// 
/// When both [CENAME] and the list of associated [VID]s are zero-length,
/// this indicates that the [CEID] does not exist.
/// 
/// [CEID]:   CollectionEventID
/// [CENAME]: CollectionEventName
/// [VID]:    VariableID
pub struct CollectionEventNamelist(pub VecList<(CollectionEventID, CollectionEventName, VecList<VariableID>)>);
message_data!{CollectionEventNamelist, false, 1, 24}
