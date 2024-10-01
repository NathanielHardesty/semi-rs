# HIGH-SPEED SECS MESSAGE SERVICES (HSMS)

Copyright © 2024 Nathaniel Hardesty, Licensed under the [MIT License](../license.md)

This software is created by a third-party and not endorsed or supported by SEMI.

The codebase will be updated to reflect more up-to-date SEMI standards if/when they can be acquired for this purpose.

[![crates.io](https://img.shields.io/crates/v/semi_e37.svg)](https://crates.io/crates/semi_e37)
[![crates.io](https://img.shields.io/crates/dv/semi_e37/0.2.0.svg)](https://crates.io/crates/semi_e37/0.2.0)

--------------------------------------------------------------------------------

**Based on:**

- **[SEMI E37]-1109**
- **[SEMI E37].1-0702**

[HSMS] is a protocol designed to facilitate the reliable transmission of
messages between semiconductor equipment over TCP/IP.

Most commonly, exchanged messages are encoded with the [SECS-II] ([SEMI E5])
protocol.

--------------------------------------------------------------------------------

For ease of programming and extension, the functionality of the protocol
has been divided into a few subsets:

- [Primitive Services] - Manages the TCP/IP connection and the sending of
  messages with proper headers.
- [Generic Services] - Manages the sending of messages of particular types
  and at particular times as allowed by the protocol.
- Single Selected Session Services - Manages the restriction of the
  protocol to scenarios involving a single host/equipment pair in
  communication.
  - Not yet implemented.

[HSMS]:               https://docs.rs/semi_e37/0.2./semi_e37/index.html
[Primitive Services]: https://docs.rs/semi_e37/0.2.0/semi_e37/primitive/index.html
[Generic Services]:   https://docs.rs/semi_e37/0.2.0/semi_e37/generic/index.html

[SECS-II]: ../semi_e5/readme.md

[SEMI E5]:  https://store-us.semi.org/products/e00500-semi-e5-specification-for-semi-equipment-communications-standard-2-message-content-secs-ii
[SEMI E37]: https://store-us.semi.org/products/e03700-semi-e37-high-speed-secs-message-services-hsms-generic-services
