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
pub struct EquipmentConstantData(pub VecList<OptionItem<EquipmentConstantValue>>);
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

/// ## S2F27
/// 
/// **Initiate Processing Request (IPR)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Request equipment to initiate processing of material at a location in the
/// machine using a process program.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 3
///    1. [LOC]
///    2. [PPID]
///    3. List - N
///       - [MID]
/// 
/// N is the number of materials.
/// 
/// Zero-length [PPID] means no process program is specified and the equipment
/// determines what action to take.
/// 
/// Zero-length N means no [MID] is associated with the material.
/// 
/// [LOC]:  LocationCode
/// [PPID]: ProcessProgramID
/// [MID]:  MaterialID
pub struct InitiateProcessingRequest(pub (LocationCode, ProcessProgramID, VecList<MaterialID>));
message_data!{InitiateProcessingRequest, true, 2, 27}

/// ## S2F28
/// 
/// **Initiate Processing Acknowledge (IPA)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Whether or not the request was honored.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [CMDA]
/// 
/// [CMDA]: CommandAcknowledge
pub struct InitiateProcessingAcknowledge(pub CommandAcknowledge);
message_data!{InitiateProcessingAcknowledge, false, 2, 28}

/// ## S2F29
/// 
/// **Equipment Constant Namelist Request (ECNR)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Retrieve basic information about what equipment constants are available.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - N
///    - [ECID]
/// 
/// N is the number of equipment constants.
/// 
/// Zero-length N means to request information about all equipment constants.
/// 
/// [ECID]: EquipmentConstantID
pub struct EquipmentConstantNamelistRequest(pub VecList<EquipmentConstantID>);
message_data!{EquipmentConstantNamelistRequest, true, 2, 29}

/// ## S2F30
/// 
/// **Equipment Constant Namelist (ECN)**
/// 
/// - **MULTI-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// List of requested equipment constants.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - N
///    - List - 6
///       1. [ECID]
///       2. [ECNAME]
///       3. [ECMIN]
///       4. [ECMAX]
///       5. [ECDEF]
///       6. [UNITS]
/// 
/// N is the number of equipment constants.
/// 
/// Zero-length [ECNAME], [ECMIN], [ECMAX], [ECDEF], and [UNITS] means that the
/// equipment constant does not exist.
/// 
/// [ECID]:   EquipmentConstantID
/// [ECNAME]: EquipmentConstantName
/// [ECMIN]:  EquipmentConstantMinimumValue
/// [ECMAX]:  EquipmentConstantMaximumValue
/// [ECDEF]:  EquipmentConstantDefaultValue
/// [UNITS]:  Units
pub struct EquipmentConstantNamelist(pub VecList<(EquipmentConstantID, EquipmentConstantName, EquipmentConstantMinimumValue, EquipmentConstantMaximumValue, EquipmentConstantDefaultValue, Units)>);
message_data!{EquipmentConstantNamelist, false, 2, 30}

/// ## S2F31
/// 
/// **Date and Time Set Request (DTS)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Synchronize equipment time with host time base.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [TIME]
/// 
/// [TIME]: Time
pub struct DateTimeSetRequest(pub Time);
message_data!{DateTimeSetRequest, true, 2, 31}

/// ## S2F32
/// 
/// **Date and Time Set Acknowledge (DTA)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Acknowledge receipt of time and date.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [TIACK]
/// 
/// [TIACK]: TimeAcknowledgeCode
pub struct DateTimeSetAcknowledge(pub TimeAcknowledgeCode);
message_data!{DateTimeSetAcknowledge, false, 2, 32}

/// ## S2F33
/// 
/// **Define Report (DR)**
/// 
/// - **MULTI-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Define a group of reports for the equipment.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [DATAID]
///    2. List - M
///       - List - 2
///          1. [RPTID]
///          2. List - N
///             - [VID]
/// 
/// M is the number of reports to be defined.
/// 
/// Zero-length M means to delete all report definitions and associated links.
/// 
/// N is the number of [VID]s in a report.
/// 
/// Zero-length N means to delete the [RPTID] and any [CEID]s linked to it.
/// 
/// [DATAID]: DataID
/// [RPTID]:  ReportID
/// [VID]:    VariableID
/// [CEID]:   CollectionEventID
pub struct DefineReport(pub (DataID, VecList<(ReportID, VecList<VariableID>)>));
message_data!{DefineReport, true, 2, 33}

/// ## S2F34
/// 
/// **Define Report Acknowledge (DRA)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Acknowledge, or error if any error condition is detected. In the latter
/// case, the entire message is rejected, partial changes are not allowed.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [DRACK]
/// 
/// [DRACK]: DefineReportAcknowledgeCode
pub struct DefineReportAcknowledge(pub DefineReportAcknowledgeCode);
message_data!{DefineReportAcknowledge, false, 2, 34}

/// ## S2F35
/// 
/// **Link Event Report (LER)**
/// 
/// - **MULTI-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Link reports to events.
/// 
/// Linked reports will be disabled by default.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [DATAID]
///    2. List - M
///       - List - 2
///          1. [CEID]
///          2. List - N
///             - [RPTID]
/// 
/// M is the number of collection events.
/// 
/// N is the number of reports to link to a collection event.
/// 
/// Zero-length N means to delete all reports associated with a collection
/// event.
/// 
/// [DATAID]: DataID
/// [CEID]:   CollectionEventID
/// [RPTID]:  ReportID
pub struct LinkEventReport(pub (DataID, VecList<(CollectionEventID, VecList<ReportID>)>));
message_data!{LinkEventReport, true, 2, 35}

/// ## S2F36
/// 
/// **Link Event Report Acknowledge (LERA)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Acknowledge, or error if any error condition is detected. In the latter
/// case, the entire message is rejected, partial changes are not allowed.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [LRACK]
/// 
/// [LRACK]: LinkReportAcknowledgeCode
pub struct LinkEventReportAcknowledge(pub LinkReportAcknowledgeCode);
message_data!{LinkEventReportAcknowledge, false, 2, 36}

/// ## S2F37
/// 
/// **Enable/Disable Event Report (EDER)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Enable or disable reporting for collection events.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [CEED]
///    2. List - N
///       - [CEID]
/// 
/// N is the number of collection events.
/// 
/// Zero-length N means to enable or disable all collection events.
/// 
/// [CEED]: CollectionEventEnableDisable
/// [CEID]: CollectionEventID
pub struct EnableDisableEventReport(pub (CollectionEventEnableDisable, VecList<CollectionEventID>));
message_data!{EnableDisableEventReport, true, 2, 37}

/// ## S2F38
/// 
/// **Enable/Disable Event Report Acknowledge**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Acknowledge, or error if any error condition is detected. In the latter
/// case, the entire message is rejected, partial changes are not allowed.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [ERACK]
/// 
/// [ERACK]: EnableDisableEventReportAcknowledgeCode
pub struct EnableDisableEventReportAcknowledge(pub EnableDisableEventReportAcknowledgeCode);
message_data!{EnableDisableEventReportAcknowledge, false, 2, 38}

/// ## S2F39
/// 
/// **Multi-Block Inquire (DMBI)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Establish that sending a multi-block message is allowed prior to sending
/// [S2F23], [S2F33], [S2F35], [S2F45], or [S2F49].
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [DATAID]
///    2. [DATALENGTH]
/// 
/// [DATAID]:     DataID
/// [DATALENGTH]: DataLength
/// [S2F23]:      TraceInitializeSend
/// [S2F33]:      DefineReport
/// [S2F35]:      LinkEventReport
/// [S2F45]:      DefineVariableLimitAttributes
/// [S2F49]:      EnhancedRemoteCommand
pub struct MultiBlockInquire(pub (DataID, DataLength));
message_data!{MultiBlockInquire, true, 2, 39}

/// ## S2F40
/// 
/// **Multi-Block Grant (DMBG)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Grant permission to send a multi-block message.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - [GRANT]
/// 
/// [GRANT]: Grant
pub struct MultiBlockGrant(pub Grant);
message_data!{MultiBlockGrant, false, 2, 40}

/// ## S2F41
/// 
/// **Host Command Send (HCS)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Request equipment to perform the specified remote command with the
/// associated parameters.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [RCMD]
///    2. List - N
///       - List - 2
///          1. [CPNAME]
///          2. [CPVAL]
/// 
/// [RCMD]:   RemoteCommand
/// [CPNAME]: CommandParameterName
/// [CPVAL]:  CommandParameterValue
pub struct HostCommandSend(pub (RemoteCommand, VecList<(CommandParameterName, CommandParameterValue)>));
message_data!{HostCommandSend, true, 2, 41}

/// ## S2F42
/// 
/// **Host Command Acknowledge (HCA)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Acknowledge, or error if the command cannot be accepted. If the command is
/// not accepted due to one or more invalid parameters, a list of invalid
/// parameters and the associated reasons for rejection is provided.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [HCACK]
///    2. List - N
///       - List - 2
///          1. [CPNAME]
///          2. [CPACK]
/// 
/// [HCACK]:  HostCommandAcknowledgeCode
/// [CPNAME]: CommandParameterName
/// [CPACK]:  CommandParameterAcknowledgeCode
pub struct HostCommandAcknowledge(pub (HostCommandAcknowledgeCode, VecList<(CommandParameterName, CommandParameterAcknowledgeCode)>));
message_data!{HostCommandAcknowledge, false, 2, 42}

/// ## S2F43
/// 
/// **Reset Spooling Streams and Functions (RSSF)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Select specific streams and functions to be spooled whenever spooling is
/// active.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - M
///    - List - 2
///       1. [STRID]
///       2. List - N
///          - [FCNID]
/// 
/// M is the number of streams.
/// 
/// Zero-length M turns off spooling for all streams and functions.
/// 
/// N is the number of functions in a stream.
/// 
/// Zero-length N turns on spooling for all functions in the stream.
/// 
/// Turning off spooling for all functions for a specific stream is achieved by
/// omitting reference to the stream from this message.
/// 
/// Spooling for Stream 1 is not allowed.
/// 
/// Equipment must allow the host to spool all primary messages for a stream.
/// 
/// A defined list of functions for a stream in this message will replace any
/// previously selected functions.
/// 
/// [STRID]: StreamID
/// [FCNID]: FunctionID
pub struct ResetSpoolingStreamsAndFunctions(pub VecList<(StreamID, VecList<FunctionID>)>);
message_data!{ResetSpoolingStreamsAndFunctions, true, 2, 43}

/// ## S2F44
/// 
/// **Reset Spooling Acknowledge (RSA)**
/// 
/// - **MULTI-BLOCK**
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
/// - List - 2
///    1. [RSPACK]
///    2. List - M
///       - List - 3
///          1. [STRID]
///          2. [STRACK]
///          3. List - N
///             - [FCNID]
/// 
/// M is the number of streams in error.
/// 
/// Zero-length M means no streams in error.
/// 
/// N is the number of functions in error in a stream.
/// 
/// Zero-length N means no functions in error for stream.
/// 
/// [RSPACK]: ResetSpoolingAcknowledgeCode
/// [STRID]:  StreamID
/// [STRACK]: SpoolStreamAcknowledgeCode
/// [FCNID]:  FunctionID
pub struct ResetSpoolingAcknowledge(pub (ResetSpoolingAcknowledgeCode, VecList<(StreamID, SpoolStreamAcknowledgeCode, VecList<FunctionID>)>));
message_data!{ResetSpoolingAcknowledge, false, 2, 44}

/// ## S2F45
/// 
/// **Define Variable Limit Attributes (DVLA)**
/// 
/// - **MULTI-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [DATAID]
///    2. List - M
///       - List - 2
///          1. [VID]
///          2. List - N
///             - List - 2
///                1. [LIMITID]
///                2. List - 0 or 2
///                   1. [UPPERDB]
///                   2. [LOWERDB]
/// 
/// M is the number of variables whose limits are being defined.
/// 
/// Zero-length M means to set all limit values for all monitored [VID]s to
/// undefined.
/// 
/// N is the number of limits being defined or changed.
/// 
/// Zero-length N means to set all limit values for that [VID] to undefined.
/// 
/// Zero-length list after [LIMITID] means to set that limit to undefined.
/// 
/// [DATAID]:  DataID
/// [VID]:     VariableID
/// [LIMITID]: LimitID
/// [UPPERDB]: UpperDeadband
/// [LOWERDB]: LowerDeadband
pub struct DefineVariableLimitAttributes(pub (DataID, VecList<(VariableID, VecList<(LimitID, OptionItem<(UpperDeadband, LowerDeadband)>)>)>));
message_data!{DefineVariableLimitAttributes, true, 2, 45}

/// ## S2F46
/// 
/// **Variable Limit Attribute Acknowledge (VLAA)**
/// 
/// - **MULTI-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Acknowledge definition of variable limit attributes. If the command is not
/// accepted due to one or more invalid parameters, then a list of invalid
/// parameters is returned containing the variable limit attribute and reason
/// for rejection. In the case that the command is rejected, the entire message
/// is rejected, partial changes are not allowed.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [VLAACK]
///    2. List - M
///       - List - 3
///          1. [VID]
///          2. [LVACK]
///          3. List - 0 or 2
///             1. [LIMITID]
///             2. [LIMITACK]
/// 
/// M is the number of invalid parameters.
/// 
/// Zero-length M means no invalid variable limit attributes.
/// 
/// Zero-length list after [LVACK] means no invalid limit values for that
/// variable.
/// 
/// [VLAACK]:   VariableLimitAttributeAcknowledgeCode
/// [VID]:      VariableID
/// [LVACK]:    VariableLimitDefinitonAcknowledgeCode
/// [LIMITID]:  LimitID
/// [LIMITACK]: VariableLimitAttributeSetAcknowledgeCode
pub struct VariableLimitAttributeAcknowledge(pub (VariableLimitAttributeAcknowledgeCode, VecList<(VariableID, VariableLimitDefinitonAcknowledgeCode, OptionItem<(LimitID, VariableLimitAttributeSetAcknowledgeCode)>)>));
message_data!{VariableLimitAttributeAcknowledge, false, 2, 46}

/// ## S2F47
/// 
/// **Variable Limit Attribute Request (VLAR)**
/// 
/// - **SINGLE-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Query current variable limit attribute definitions.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - N
///    - [VID]
/// 
/// N is the number of variables being requested.
/// 
/// Zero-length N means to report all variables that can have variable limit
/// attributes.
/// 
/// [VID]: VariableID
pub struct VariableLimitAttributeRequest(pub VecList<VariableID>);
message_data!{VariableLimitAttributeRequest, true, 2, 47}

/// ## S2F48
/// 
/// **Variable Limit Attribute Send (VLAS)**
/// 
/// - **MULTI-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Send values of requested variable limit attribute definitions.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - M
///    - List - 2
///       1. [VID]
///       2. List - 0 or 4
///          1. [UNITS]
///          2. [LIMITMIN]
///          3. [LIMITMAX]
///          4. List - N
///             - List - 3
///                1. [LIMITID]
///                2. [UPPERDB]
///                3. [LOWERDB]
/// 
/// M is the number of requested variables.
/// 
/// N is the number of limits defined for a variable.
/// 
/// Zero-length N is the means no limits are defined for the variable.
/// 
/// Zero-length list after [VID] means limits are not supported.
/// 
/// [VID]:      VariableID
/// [UNITS]:    Units
/// [LIMITMIN]: LimitMinimum
/// [LIMITMAX]: LimitMaximum
/// [LIMITID]:  LimitID
/// [UPPERDB]:  UpperDeadband
/// [LOWERDB]:  LowerDeadband
pub struct VariableLimitAttributeSend(pub VecList<(VariableID, OptionItem<(Units, LimitMinimum, LimitMaximum, VecList<(LimitID, UpperDeadband, LowerDeadband)>)>)>);
message_data!{VariableLimitAttributeSend, false, 2, 48}

/// ## S2F49
/// 
/// **Enhanced Remote Command**
/// 
/// - **MULTI-BLOCK**
/// - **HOST -> EQUIPMENT**
/// - **REPLY REQUIRED**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Request an object to perform the specified remote command with its
/// associated parameters.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 4
///    1. [DATAID]
///    2. [OBJSPEC]
///    3. [RCMD]
///    4. List - N
///       - List - 2
///          1. [CPNAME]
///          2. [CEPVAL]
/// 
/// N is the number of parameters being passed.
/// 
/// [DATAID]:  DataID
/// [OBJSPEC]: ObjectSpecifier
/// [RCMD]:    RemoteCommand
/// [CPNAME]:  CommandParameterName
/// [CEPVAL]:  CommandEnhancedParameterValue
pub struct EnhancedRemoteCommand(pub (DataID, ObjectSpecifier, RemoteCommand, VecList<(CommandParameterName, CommandEnhancedParameterValue)>));
message_data!{EnhancedRemoteCommand, true, 2, 49}

/// ## S2F50
/// 
/// **Enhanced Remote Command Acknowledge**
/// 
/// - **MULTI-BLOCK**
/// - **HOST <- EQUIPMENT**
/// - **REPLY FORBIDDEN**
/// 
/// ---------------------------------------------------------------------------
/// 
/// Acknowledge enhanced remote command or report any errors. If the command is
/// not accepted due to one or more invalid parameters, then a list of invalid
/// parameters will be returned containing the parameter name and reason for
/// being invalid.
/// 
/// ---------------------------------------------------------------------------
/// 
/// #### Structure
/// 
/// - List - 2
///    1. [HCACK]
///    2. List - N
///       - List - 2
///          1. [CPNAME]
///          2. [CEPACK]
/// 
/// N is the number of parameters in error.
/// 
/// [HCACK]:  HostCommandAcknowledgeCode
/// [CPNAME]: CommandParameterName
/// [CEPACK]: CommandEnhancedParameterAcknowledgeCode
pub struct EnhancedRemoteCommandAcknowledge(pub (HostCommandAcknowledgeCode, VecList<(CommandParameterName, CommandParameterAcknowledgeCode)>));
message_data!{EnhancedRemoteCommandAcknowledge, false, 2, 50}
