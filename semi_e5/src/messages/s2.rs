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

//! # STREAM 2: EQUIPMENT CONTROL AND DIAGNOSTICS
//! **Based on SEMI E5§10.6**
//! 
//! ---------------------------------------------------------------------------
//! 
//! [Message]s which deal with control of the equipment from the host.
//! 
//! This includes all remote operations and equipment self-diagnostics and
//! calibration but specifically excluses:
//! 
//! - Control operations associated with material transfer ([Stream 4]).
//! - Loading of executive and boot programs ([Stream 8]).
//! - File and operating system calls ([Stream 10], [Stream 13]).
//! 
//! ---------------------------------------------------------------------------
//! 
//! This functionality continues in [Stream 17].
//! 
//! ---------------------------------------------------------------------------
//! 
//! ## TO BE DONE
//! 
//! - Fill out stream contents
//! 
//! [Message]:   crate::Message
//! [Stream 4]:  crate::messages::s4
//! [Stream 8]:  crate::messages::s8
//! [Stream 10]: crate::messages::s10
//! [Stream 13]: crate::messages::s13
//! [Stream 17]: crate::messages::s17

use crate::*;
use crate::Error::*;
use crate::items::*;

/// ## S2F0
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
message_headeronly!{Abort, false, 2, 0}

/// ## S2F1
/// 
/// **Service Program Load Inquire (SPI)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <-> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Request to send the specified service program.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [SPID]
///    2. [LENGTH]
/// 
/// [SPID]:   ServiceProgramID
/// [LENGTH]: Length
pub struct ServiceProgramLoadInquire(pub (ServiceProgramID, Length));
message_data!{ServiceProgramLoadInquire, true, 2, 1}

/// ## S2F2
/// 
/// **Service Program Load Grant (SPG)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <-> EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Permission to send the service program.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [GRANT]
/// 
/// [GRANT]: Grant
pub struct ServiceProgramLoadGrant(pub Grant);
message_data!{ServiceProgramLoadGrant, false, 2, 2}

/// ## S2F3
/// 
/// **Service Program Send (SPS)**
/// 
/// - **MULTI-BLOCK**
/// - **HOST <-> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Data associated with prior [S2F1] inquire.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [SPD]
/// 
/// [S2F1]: ServiceProgramLoadInquire
/// [SPD]:  ServiceProgramData
pub struct ServiceProgramSend(pub ServiceProgramData);
message_data!{ServiceProgramSend, true, 2, 3}

/// ## S2F4
/// 
/// **Service Program Send Acknowledge (SPA)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <-> EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Acknowledgement of [S2F3].
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [SPAACK]
/// 
/// [S2F3]:   ServiceProgramSend
/// [SPAACK]: ServiceProgramAcknowledge
pub struct ServiceProgramSendAcknowledge(pub ServiceProgramAcknowledge);
message_data!{ServiceProgramSendAcknowledge, false, 2, 4}

/// ## S2F5
/// 
/// **Service Program Load Request (SPR)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <-> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Request to be sent service program.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [SPID]
/// 
/// [SPID]: ServiceProgramID
pub struct ServiceProgramLoadRequest(pub ServiceProgramID);
message_data!{ServiceProgramLoadRequest, true, 2, 5}

/// ## S2F6
/// 
/// **Service Program Load Data (SPD)**
/// 
/// - **MULTI-BLOCK**
/// - **HOST <-> EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Service program data.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [SPD]
/// 
/// Zero-length item means that the service program cannot be returned.
/// 
/// [SPD]: ServiceProgramData
pub struct ServiceProgramLoadData(pub ServiceProgramData);
message_data!{ServiceProgramLoadData, false, 2, 6}

/// ## S2F7
/// 
/// **Service Program Run Send (CSS)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Request to start service program.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [SPID]
/// 
/// [SPID]: ServiceProgramID
pub struct ServiceProgramRunSend(pub ServiceProgramID);
message_data!{ServiceProgramRunSend, true, 2, 7}

/// ## S2F8
/// 
/// **Service Program Run Acknowledge (CSA)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Acknowledgement of [S2F7].
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [CSAACK]
/// 
/// [S2F7]:   ServiceProgramRunSend
/// [CSAACK]: ServiceAcknowledgeCode
pub struct ServiceProgramRunAcknowledge(pub ServiceAcknowledgeCode);
message_data!{ServiceProgramRunAcknowledge, false, 2, 8}

/// ## S2F9
/// 
/// **Service Program Results Request (SRR)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Request for results of service program.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [SPID]
/// 
/// [SPID]: ServiceProgramID
pub struct ServiceProgramResultsRequest(pub ServiceProgramID);
message_data!{ServiceProgramResultsRequest, true, 2, 9}

/// ## S2F10
/// 
/// **Service Program Results Data (SRD)**
/// 
/// - **MULTI-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Service program results.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [SPR]
/// 
/// Zero-length item means [SPR] does not exist.
/// 
/// [SPR]: ServiceProgramResults
pub struct ServiceProgramResultsData(pub ServiceProgramResults);
message_item!{ServiceProgramResultsData, false, 2, 10}

/// ## S2F11
/// 
/// **Service Program Directory Request (SDR)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <-> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Request service program list.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// Header only.
pub struct ServiceProgramDirectoryRequest;
message_headeronly!{ServiceProgramDirectoryRequest, true, 2, 11}

/// ## S2F12
/// 
/// **Service Program Directory Data (SDD)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <-> EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Service program list.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - N
///    - [SPID]
/// 
/// N is the number of service programs.
/// 
/// [SPID]: ServiceProgramID
pub struct ServiceProgramDirectoryData(pub VecList<ServiceProgramID>);
message_data!{ServiceProgramDirectoryData, false, 2, 12}

/// ## S2F13
/// 
/// **Equipment Calibration Request (ECR)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Request equipment constant values, which are changed infrequently.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - N
///    - [ECID]
/// 
/// N is the number of requested [ECV]s.
/// Zero-length N means report all [ECV]s.
/// 
/// [ECID]: EquipmentConstantID
/// [ECV]:  EquipmentConstantValue
pub struct EquipmentConstantRequest(pub VecList<EquipmentConstantID>);
message_data!{EquipmentConstantRequest, true, 2, 13}

/// ## S2F14
/// 
/// **Equipment Constant Data (ECD)**
/// 
/// - **MULTI-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Equipment constant values.
/// 
/// TODO: Implement zero-length list item exception.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - N
///    - [ECV]
/// 
/// N is the number of [ECV]s.
/// 
/// Zero-length list item [ECV] means that the corresponding [ECID] does
/// not exist. The list format is not allowed for [ECV] except in this
/// case.
/// 
/// [ECID]: EquipmentConstantID
/// [ECV]:  EquipmentConstantValue
pub struct EquipmentConstantData(pub VecList<EquipmentConstantValue>);
message_data!{EquipmentConstantData, false, 2, 14}

/// ## S2F15
/// 
/// **New Equipment Constant Send (ECS)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Change equipment constants.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - N
///    - List - 2
///       1. [ECID]
///       2. [ECV]
/// 
/// N is the number of equipment constants to be changed.
/// 
/// [ECID]: EquipmentConstantID
/// [ECV]:  EquipmentConstantValue
pub struct NewEquipmentConstantSend(pub VecList<(EquipmentConstantID, EquipmentConstantValue)>);
message_data!{NewEquipmentConstantSend, true, 2, 15}

/// ## S2F16
/// 
/// **New Equipment Constant Acknowledge (ECA)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Acknowledge equipment constant changes.
/// 
/// If error, the equipment should not update any of the constants
/// specified by [S2F15].
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [EAC]
/// 
/// [EAC]:   EquipmentAcknowledgeCode
/// [S2F15]: NewEquipmentConstantSend
pub struct NewEquipmentConstantAcknowledge(pub EquipmentAcknowledgeCode);
message_data!{NewEquipmentConstantAcknowledge, false, 2, 16}

/// ## S2F17
/// 
/// **Date and Time Request (DTR)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <-> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Check time base or synchronize clocks.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// Header only.
pub struct DateTimeRequest;
message_headeronly!{DateTimeRequest, true, 2, 17}

/// ## S2F18
/// 
/// **Date and Time Data (DTD)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <-> EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Time data.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [TIME]
/// 
/// Zero-length [TIME] item means no time data exists.
/// 
/// [TIME]: Time
pub struct DateTimeData(pub Time);
message_data!{DateTimeData, false, 2, 18}

/// ## S2F19
/// 
/// **Reset/Initialize Send (RIS)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Cause equipment to reach one of several predetermined conditions.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [RIC]
/// 
/// [RIC]: ResetCode
pub struct ResetInitializeSend(pub ResetCode);
message_data!{ResetInitializeSend, true, 2, 19}

/// ## S2F20
/// 
/// **Reset Acknowledge (RIA)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Acknowledge reset.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [RAC]
/// 
/// [RAC]: ResetAcknowledgeCode
pub struct ResetAcknowledge(pub ResetAcknowledgeCode);
message_data!{ResetAcknowledge, false, 2, 20}

/// ## S2F21
/// 
/// **Remote Command Send (RCS)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY OPTIONAL**
/// 
/// TODO: Implement optional reply.
/// 
/// ---------------------------------------------------------------------------
/// 
/// Cause activity on equipment to commence or cease.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [RCMD]
/// 
/// [RCMD]: RemoteCommand
pub struct RemoteCommandSend(pub RemoteCommand);
message_data!{RemoteCommandSend, true, 2, 21}

/// ## S2F22
/// 
/// **Remote Command Acknowledge (RCA)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Acknowledge remote command.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [CMDA]
/// 
/// [CMDA]: CommandAcknowledge
pub struct RemoteCommandAcknowledge(pub CommandAcknowledge);
message_data!{RemoteCommandAcknowledge, false, 2, 22}

/// ## S2F23
/// 
/// **Trace Initialize Send (TIS)**
/// 
/// - **MULTI-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Status variables exist at all times. Request that a subset of said
/// status variables be reported back to the host as a function of time.
/// 
/// The trace data is returned by S6F1, and is related to the original
/// request by the [TRID].
/// 
/// Multiple trace requests may be made.
/// 
/// If equipment receives a trace request with the same [TRID] as an
/// existing trace request, it should terminate the old trace request
/// before beginning the new one.
/// 
/// A trace in progress may be terminated by sending a trace request with
/// the same [TRID] and a [TOTSMP] of zero.
/// 
/// If this message is multi-block, it must be preceded by an S2F39/S2F40
/// transaction.
/// 
/// Some equipment may support only single-block S6F1 and may refuse this
/// message if it would cause a multi-block S6F1.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 5
///    1. [TRID]
///    2. [DSPER]
///    3. [TOTSMP]
///    4. [REPGSZ]
///    5. List - N
///       - [SVID]
/// 
/// N is the number of status variables to become a part of the trace.
/// 
/// [TRID]:   TraceRequestID
/// [DSPER]:  DataSamplePeriod
/// [TOTSMP]: TotalSamples
/// [REPGSZ]: ReportingGroupSize
/// [SVID]:   StatusVariableID
pub struct TraceInitializeSend(pub (TraceRequestID, DataSamplePeriod, TotalSamples, ReportingGroupSize, VecList<StatusVariableID>));
message_data!{TraceInitializeSend, true, 2, 23}

/// ## S2F24
/// 
/// **Trace Initialize Acknowledge (TIA)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Acknowledge trace initialize.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [TIAACK]
/// 
/// [TIAACK]: TraceInitializeAcknowledgeCode
pub struct TraceInitializeAcknowledge(pub TraceInitializeAcknowledgeCode);
message_data!{TraceInitializeAcknowledge, false, 2, 24}

/// ## S2F25
/// 
/// **Loopback Diagnostic Request (LDR)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <-> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Diagnostic message to test protocol and communication circuits.
/// 
/// Binary string is echoed back verbatim.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [ABS]
/// 
/// [ABS]: AnyBinaryString
pub struct LoopbackDiagnosticRequest(pub AnyBinaryString);
message_data!{LoopbackDiagnosticRequest, true, 2, 25}

/// ## S2F26
/// 
/// **Loopback Diagnostic Data (LDD)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <-> EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Echo binary string.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [ABS]
/// 
/// [ABS]: AnyBinaryString
pub struct LoopbackDiagnosticData(pub AnyBinaryString);
message_data!{LoopbackDiagnosticData, false, 2, 26}
