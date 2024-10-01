# SEMI EQUIPMENT COMMUNICATIONS STANDARD 2 MESSAGE CONTENT (SECS-II)

Copyright © 2024 Nathaniel Hardesty, Licensed under the [MIT License](../license.md)

This software is created by a third-party and not endorsed or supported by SEMI.

The codebase will be updated to reflect more up-to-date SEMI standards if/when they can be acquired for this purpose.

[![crates.io](https://img.shields.io/crates/v/semi_e5.svg)](https://crates.io/crates/semi_e5)
[![crates.io](https://img.shields.io/crates/dv/semi_e5/0.2.0.svg)](https://crates.io/crates/semi_e5/0.2.0)

--------------------------------------------------------------------------------

**Based on:**

- **[SEMI E5]-0813**

[SECS-II] is a protocol designed to facilitate a common syntactic and semantic
message structure used in communications between semiconductor equipment.

Most commonly, the [HSMS] ([SEMI E37]) or SECS-I ([SEMI E4]) protocols are used
to transmit SECS-II formatted messages.

Most commonly, the GEM ([SEMI E30]) and OSS ([SEMI E39]) protocols are used to
integrate host/equipment communication and behavior using SECS-II messages.

[SECS-II]: https://docs.rs/semi_e5/0.2.0/semi_e5/index.html

[HSMS]: ../semi_e37/readme.md

[SEMI E4]:  https://store-us.semi.org/products/e00400-semi-e4-specification-for-semi-equipment-communications-standard-1-message-transfer-secs-i
[SEMI E5]:  https://store-us.semi.org/products/e00500-semi-e5-specification-for-semi-equipment-communications-standard-2-message-content-secs-ii
[SEMI E30]: https://store-us.semi.org/products/e03000-semi-e30-specification-for-the-generic-model-for-communications-and-control-of-manufacturing-equipment-gem
[SEMI E37]: https://store-us.semi.org/products/e03700-semi-e37-high-speed-secs-message-services-hsms-generic-services
[SEMI E39]: https://store-us.semi.org/products/e03900-semi-e39-specification-for-object-services-concepts-behavior-and-services
