# SEMI NETWORKING STACK RUST LIBRARIES

Copyright © 2024 Nathaniel Hardesty, Licensed under the [MIT License](.\license.md)

This software is created by a third-party and not endorsed or supported by SEMI.

## APPLICATION LAYER

### PLANNED APPLICATION LAYER PROTOCOLS

-------------------------------------------------------------------------------

- Generic Equipment Model (GEM) - [SEMI E30]
- Object Services (OSS) - [SEMI E39]
- Processing Management (PJM) - [SEMI E40]
- Carrier Management (CMS) - [SEMI E87]
- Substrate Tracking (STS) - [SEMI E90]
- Control Job Management (CJM) - [SEMI E94]
- Equipment Performance Tracking (EPT) - [SEMI E116]
- Time Synchronization - [SEMI E148]
- Module Process Tracking (MPT) - [SEMI E157]

## PRESENTATION LAYER

### SEMI EQUIPMENT COMMUNICATIONS STANDARD 2 MESSAGE CONTENT ([SECS-II])

-------------------------------------------------------------------------------

[![crates.io](https://img.shields.io/crates/v/semi_e5.svg)](https://crates.io/crates/semi_e5)
[![crates.io](https://img.shields.io/crates/d/semi_e5.svg)](https://crates.io/crates/semi_e5)

**Based on:**

- **[SEMI E5]**

-------------------------------------------------------------------------------

[SECS-II] is a [Presentation Layer] protocol designed to facilitate a
common communications language between semiconductor equipment,
particularly as understood by the GEM ([SEMI E30]) [Application Layer]
protocol (together known as SECS/GEM). Common [Session Layer] protocols for
transporting [SECS-II] messages include SECS-I ([SEMI E4]) and [HSMS]
([SEMI E37]).

### PLANNED PRESENTATION LAYER PROTOCOLS

- XML SECS-II Message Notation (SMN) - [SEMI E173]

-------------------------------------------------------------------------------

## SESSION LAYER

### HIGH-SPEED SECS MESSAGE SERVICES ([HSMS])

-------------------------------------------------------------------------------

[![crates.io](https://img.shields.io/crates/v/semi_e37.svg)](https://crates.io/crates/semi_e37)
[![crates.io](https://img.shields.io/crates/d/semi_e37.svg)](https://crates.io/crates/semi_e37)

**Based on:**

- **[SEMI E37]**
- **[SEMI E37].1**

-------------------------------------------------------------------------------

[HSMS] is a [Session Layer] protocol designed to facilitate communications
between semiconductor equipment over TCP/IP, particularly for sending data
formatted with the [SECS-II] ([SEMI E5]) [Presentation Layer] protocol and
understood by the GEM ([SEMI E30]) [Application Layer] protocol (together
known as SECS/GEM).

### PLANNED SESSION LAYER PROTOCOLS

-------------------------------------------------------------------------------

- SEMI Equipment Communications Standard 1 Message Transfer (SECS-I) - [SEMI E4]

[SEMI E4]:  https://store-us.semi.org/products/e00400-semi-e4-specification-for-semi-equipment-communications-standard-1-message-transfer-secs-i
[SEMI E5]:  https://store-us.semi.org/products/e00500-semi-e5-specification-for-semi-equipment-communications-standard-2-message-content-secs-ii
[SEMI E30]: https://store-us.semi.org/products/e03000-semi-e30-specification-for-the-generic-model-for-communications-and-control-of-manufacturing-equipment-gem
[SEMI E37]: https://store-us.semi.org/products/e03700-semi-e37-high-speed-secs-message-services-hsms-generic-services
[SEMI E39]: https://store-us.semi.org/products/e03900-semi-e39-specification-for-object-services-concepts-behavior-and-services
[SEMI E40]: https://store-us.semi.org/products/e04000-semi-e40-specification-for-processing-management
[SEMI E87]: https://store-us.semi.org/products/e08700-semi-e87-specification-for-carrier-management-cms
[SEMI E90]: https://store-us.semi.org/products/e09000-semi-e90-specification-for-substrate-tracking
[SEMI E94]: https://store-us.semi.org/products/e09400-semi-e94-specification-for-control-job-management
[SEMI E116]: https://store-us.semi.org/products/e11600-semi-e116-specification-for-equipment-performance-tracking
[SEMI E148]: https://store-us.semi.org/products/e14800-semi-e148-specification-for-time-synchronization-and-definition-of-the-ts-clock-object
[SEMI E157]: https://store-us.semi.org/products/e15700-semi-e157-specification-for-module-process-tracking
[SEMI E173]: https://store-us.semi.org/products/e17300-semi-e173-specification-for-xml-secs-ii-message-notation-smn

[Application Layer]:  https://en.wikipedia.org/wiki/Application_layer
[Presentation Layer]: https://en.wikipedia.org/wiki/Presentation_layer
[Session Layer]:      https://en.wikipedia.org/wiki/Session_layer

[SECS-II]: .\semi_e5\readme.md
[HSMS]:    .\semi_e37\readme.md
