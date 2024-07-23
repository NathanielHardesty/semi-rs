
mod temp {
//! ## Data Item Restrictions
//! 
//! | Item       | Formats            |
//! | :--        |                --: |
//! | ACKC7A     | 51                 |
//! | ALCD       | ONLY BIT 8         |
//! | ALID       | 5()                |
//! | CCODE      | 20, 32, 34, 52, 54 |
//! | CEID       | 5()                |
//! | CPNAME     | 20                 |
//! | DATAID     | 5()                |
//! | DATALENGTH | 5()                |
//! | ECID       | 5()                |
//! | LENGTH     | 5()                |
//! | PPID       | 20                 |
//! | RCMD       | 20                 |
//! | REPGSZ     | 5()                |
//! | RPTID      | 5()                |
//! | SEQNUM     | 52                 |
//! | SMPLN      | 5()                |
//! | SVID       | 5()                |
//! | TEXT       | 20                 |
//! | TOTSMP     | 5()                |
//! | TRID       | 5()                |
//! | VID        | 5()                |
//! 
//! ---------------------------------------------------------------------------
//! 
//! ## SECS-II Message Subset
//! 
//! Lists the required set of SECS-II messages as referenced.
//! 
//! Definitions for these messages can be found in SEMI E5.
//! 
//! All primary messages should have replies available, but are required or
//! optional as specified in SEMI E5.
//! 
//! 148 TOTAL MESSAGES ARE REQUIRED
//! 
//! ---------------------------------------------------------------------------
//! 
//! - S1:  Equipment Status
//!   - F1/2:   Are You There Request/On-Line Data               (R/D)
//!   - F3/4:   Selected Equipment Status Request/Data           (SSR/SSD)
//!   - F11/12: Status Variable Namelist Request/Reply           (SVNR/SVNRR)
//!   - F13/14: Establish Communications Request/Acknowledge     (CR/CRA)
//!   - F15/16: Request OFF-LINE/Acknowledge                     (ROFL/OFLA)
//!   - F17/18: Request ON-LINE/Acknowledge                      (RONL/ONLA)
//! 
//! ---------------------------------------------------------------------------
//! 
//! - S2:  Equipment Control and Diagnostics
//!   - F13/14: Equipment Constant Request/Data                  (ECR/ECD)
//!   - F15/16: New Equipment Constant Send/Acknowledge          (ECS/ECA)
//!   - F17/18: Date and Time Request/Data                       (DTR/DTD)
//!   - F23/24: Trace Initialize Send/Acknowledge                (TIS/TIA)
//!   - F29/30: Equipment Constant Namelist/Request              (ECNR/ECN)
//!   - F31/32: Date and Time Send/Acknowledge                   (DTS/DTA)
//!   - F33/34: Define Report/Acknowledge                        (DR/DRA)
//!   - F35/36: Link Event Report/Acknowledge                    (LER/LERA)
//!   - F37/38: Enable/Disable Event Report/Acknowledge          (EDER/EDEA)
//!   - F39/40: Multi-Block Inquire/Grant                        (DMBI/DMBG)
//!   - F41/42: Host Command Send/Acknowledge                    (HCS/HCA)
//!   - F43/44: Reset Spooling Streams and Functions/Acknowledge (RSSF/RSA)
//!   - F45/46: Define Variable Limit Attributes/Acknowledge     (DVLA/VLAA)
//!   - F47/48: Variable Limit Attribute Request/Send            (VLAR/VLAS)
//!   - F49/50: Enhanced Remote Command/Acknowledge
//! 
//! ---------------------------------------------------------------------------
//! 
//! - S5:  Exception Handling
//!   - F1/2:   Alarm Report Send/Acknowledge                    (ARS/ARA)
//!   - F3/4:   Enable/Disable Alarm Send/Acknowledge            (EAS/EAA)
//!   - F5/6:   List Alarms Request/Data                         (LAR/LAD)
//! 
//! ---------------------------------------------------------------------------
//! 
//! - S6:  Data Collection
//!   - F1/2:   Trace Data Send/Acknowledge                      (TDS/TDA)
//!   - F5/6:   Multi-block Data Send Inquire/Grant              (MBI/MBG)
//!   - F11/12: Event Report Send/Acknowledge                    (ERS/ERA)
//!   - F15/16: Event Report Request/Data                        (ERR/ERD)
//!   - F19/20: Individual Report Request/Data                   (IRR/IRD)
//!   - F23/24: Request Spooled Data/Acknowledgement Send        (RSD/RSDAS)
//! 
//! ---------------------------------------------------------------------------
//! 
//! - S7:  Process Program Load
//! (Required Only if Equipment Implements Process Programs)
//!   - F1/2:   Process Program Load Inquire/Grant               (PPI/PPG)
//!   - F3/4:   Process Program Send/Acknowledge                 (PPS/PPA)
//!   - F5/6:   Process Program Request/Data                     (PPR/PPD)
//!   - F17/18: Delete Process Program Send/Acknowledge          (DPS/DPA)
//!   - F19/20: Current EEPD Request/Data                        (RER/RED)
//!   - F23/24: Formatted Process Program Send/Acknowledge       (FPS/FPA)
//!   - F25/26: Formatted Process Program Request/Data           (FPR/FPD)
//!   - F27/28: Process Program Verification Send/Acknowledge    (PVS/PVA)
//!   - F29/30: Process Program Verification Inquire/Grant       (PVI/PVG)
//!   - F37/38: Large Process Program Send/Acknowledge
//!   - F39/40: Large Formatted Process Program Send/Acknowledge
//!   - F41/42: Large Process Program Request/Acknowledge
//!   - F43/44: Large Formatted Process Program Request/Acknowledge
//! 
//! ---------------------------------------------------------------------------
//! 
//! - S9:  System Errors
//!   - F1:     Unrecognized Device ID                           (UDN)
//!   - F3:     Unrecognized Stream Type                         (USN)
//!   - F5:     Unrecognized Function Type                       (UFN)
//!   - F7:     Illegal Data                                     (IDN)
//!   - F9:     Transaction Timer Timeout                        (TTN)
//!   - F11:    Data Too Long                                    (DLN)
//!   - F13:    Conversation Timeout                             (CTN)
//! 
//! ---------------------------------------------------------------------------
//! 
//! - S10: Terminal Services
//!   - F1/2:   Terminal Request/Acknowledge                     (TRN/TRA)
//!   - F3/4:   Terminal Display, Single/Acknowledge             (VTN/VTA)
//!   - F5/6:   Terminal Display, Multi-block/Acknowledge        (VMN/VMA)
//!   - F7:     Multi-block Not Allowed                          (MNN)
//! 
//! ---------------------------------------------------------------------------
//! 
//! - S13: Data Set Transfers
//! (Required Only if Equipment Implements E139 Recipes, Large E42 Recipes,
//! or Large Process Programs)
//!   - F1/2:   Send Data Set Send/Acknowledge                   (DSSS/DSSA)
//!   - F3/4:   Open Data Set Request/Data                       (DSOR/DSOD)
//!   - F5/6:   Read Data Set Request/Data                       (DSSR/DSRD)
//!   - F7/8:   Close Data Set Send/Acknowledge                  (DSCS/DSCA)
//!   - F9/10:  Reset Data Set Send/Acknowledge                  (DSRS/DSRA)
//! 
//! ---------------------------------------------------------------------------
//! 
//! - S14: Object Services
//!   - F1/2:   GetAttr Request/Data
//! 
//! ---------------------------------------------------------------------------
//! 
//! - S15: Recipe Management
//! (Required Only if Equipment Implements E42 Recipes)
//!   - F1/2:   Recipe Management Multi-block Inquire/Grant
//!   - F21/22: Recipe Action Request/Acknowledge
//!   - F27/28: Recipe Download Request/Acknowledge
//!   - F29/30: Recipe Verify Request/Data
//!   - F31/32: Recipe Upload Request/Data
//!   - F35/36: Recipe Delete Request/Acknowledge
//!   - F49/50: Large Recipe Download Request/Acknowledge
//!   - F51/52: Large Recipe Upload Request/Acknowledge
//!   - F53/54: Recipe Verification Send/Acknowledge
//! 
//! ---------------------------------------------------------------------------
//! 
//! - S19: Recipe and Parameter Management
//! (Required Only if E139 Recipes are Implemented)
//!   - F1/2:   Get PDE Directory/Data                           (GPD/PDD)
//!   - F3/4:   Delete PDE/Acknowledge                           (DPDE/DPDEA)
//!   - F5/6:   Get PDE Header/Data                              (GPH/PHD)
//!   - F7/8:   Get PDE/Data                                     (GPDE/PDED)
//!   - F9/10:  Request to Send PDE/Grant                        (RTSP/SPDEG)
//!   - F11/12: Send PDE/Acknowledge                             (SPDE/SPDEA)
//!   - F13/14: TransferContainer Report/Acknowledge             (TR/TA)
//!   - F15/16: Resolve PDE Request/Data                         (RPR/RPD)
//!   - F17/18: Verify PDE/Data                                  (VP/VPD)
}
