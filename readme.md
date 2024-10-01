# SEMI PROTOCOL STACK RUST LIBRARIES

Copyright © 2024 Nathaniel Hardesty, Licensed under the [MIT License](./license.md)

This software is created by a third-party and not endorsed or supported by SEMI.

--------------------------------------------------------------------------------

## PROTOCOL MODEL

Although SEMI does not publish or correlate their protocol standards into a
particular model, an explanation of the role of each part of the protcol stack
is desirable. These considerations are taken into account:

- SEMI often produces graphics which place certain protocols in certain implied
  but unnamed layers in a protocol stack.
- The IETF model fails to provide explanatory power, as all but one SEMI
  protocol would fit entirely into its Application Layer.
- The OSI model, while appearing to be a decent fit for the SEMI standards
  which were developed in the 80s and early 90s, no longer provides an accurate
  model of SEMI's protocols.

Thusly, a bespoke model is used for SEMI's protocol stack. This model is
subject to change, as it is only provided for explanatory purposes.

1. Transaction Layer
   - Provides the reliable transmission of messages formed using bytes.
   - Provides the correlation of messages in two-way conversation into
     transactions, i.e. request/response pairs.
2. Encoding Layer
   - Provides a data syntax, the serialization of the syntax into bytes, and
     the deseralization of bytes into the syntax.
   - Provides a set of messages formed using the syntax with defined semantic
     meaning, and the way in which these messages can be used in transactions.
3. Integration Layer
   - Provides the correlation of transactions into conversations.
   - Provides the pragmatics by which the context of conversations is stored,
     accessed, and changed.
4. Operation Layer
   - Provides contexts to be manipulated for specific operational purposes,
     including the management and tracking of particular objects.

## TRANSACTION LAYER

### HIGH-SPEED SECS MESSAGE SERVICES ([HSMS])

[![crates.io](https://img.shields.io/crates/v/semi_e37.svg)](https://crates.io/crates/semi_e37)
[![crates.io](https://img.shields.io/crates/d/semi_e37.svg)](https://crates.io/crates/semi_e37)

Based on **[SEMI E37]**, [HSMS] is a protocol designed to facilitate the
reliable transmission of messages between semiconductor equipment over TCP/IP.

--------------------------------------------------------------------------------

### PLANNED TRANSACTION LAYER PROTOCOLS

- SEMI Equipment Communications Standard 1 Message Transfer (SECS-I) - [SEMI E4]

## ENCODING LAYER

### SEMI EQUIPMENT COMMUNICATIONS STANDARD 2 MESSAGE CONTENT ([SECS-II])

[![crates.io](https://img.shields.io/crates/v/semi_e5.svg)](https://crates.io/crates/semi_e5)
[![crates.io](https://img.shields.io/crates/d/semi_e5.svg)](https://crates.io/crates/semi_e5)

Based on **[SEMI E5]**, [SECS-II] is a protocol designed to facilitate a common
syntactic and semantic message structure used in communications between
semiconductor equipment.

--------------------------------------------------------------------------------

### PLANNED ENCODING LAYER PROTOCOLS

- XML SECS-II Message Notation (SMN) - [SEMI E173]

## INTEGRATION LAYER

### PLANNED INTEGRATION LAYER PROTOCOLS

- Generic Equipment Model (GEM) - [SEMI E30]
- Object Services (OSS) - [SEMI E39]

## OPERATION LAYER

### PLANNED OPERATION LAYER PROTOCOLS

- Processing Management (PJM) - [SEMI E40]
- Carrier Management (CMS) - [SEMI E87]
- Substrate Tracking (STS) - [SEMI E90]
- Control Job Management (CJM) - [SEMI E94]
- Equipment Performance Tracking (EPT) - [SEMI E116]
- Time Synchronization - [SEMI E148]
- Module Process Tracking (MPT) - [SEMI E157]

[SECS-II]: ./semi_e5/readme.md
[HSMS]:    ./semi_e37/readme.md

[SEMI E4]:   https://store-us.semi.org/products/e00400-semi-e4-specification-for-semi-equipment-communications-standard-1-message-transfer-secs-i
[SEMI E5]:   https://store-us.semi.org/products/e00500-semi-e5-specification-for-semi-equipment-communications-standard-2-message-content-secs-ii
[SEMI E30]:  https://store-us.semi.org/products/e03000-semi-e30-specification-for-the-generic-model-for-communications-and-control-of-manufacturing-equipment-gem
[SEMI E37]:  https://store-us.semi.org/products/e03700-semi-e37-high-speed-secs-message-services-hsms-generic-services
[SEMI E39]:  https://store-us.semi.org/products/e03900-semi-e39-specification-for-object-services-concepts-behavior-and-services
[SEMI E40]:  https://store-us.semi.org/products/e04000-semi-e40-specification-for-processing-management
[SEMI E87]:  https://store-us.semi.org/products/e08700-semi-e87-specification-for-carrier-management-cms
[SEMI E90]:  https://store-us.semi.org/products/e09000-semi-e90-specification-for-substrate-tracking
[SEMI E94]:  https://store-us.semi.org/products/e09400-semi-e94-specification-for-control-job-management
[SEMI E116]: https://store-us.semi.org/products/e11600-semi-e116-specification-for-equipment-performance-tracking
[SEMI E148]: https://store-us.semi.org/products/e14800-semi-e148-specification-for-time-synchronization-and-definition-of-the-ts-clock-object
[SEMI E157]: https://store-us.semi.org/products/e15700-semi-e157-specification-for-module-process-tracking
[SEMI E173]: https://store-us.semi.org/products/e17300-semi-e173-specification-for-xml-secs-ii-message-notation-smn
