# HIGH-SPEED SECS MESSAGE SERVICES (HSMS)
**Based on:**
- **[SEMI E37]-1109**
- **[SEMI E37].1-0702**

This third-party codebase will be updated to reflect more up-to-date SEMI
standards if/when they can be acquired for this purpose.

-------------------------------------------------------------------------------

[HSMS] is a Session Protocol designed to facilitate communications between
semiconductor equipment over TCP/IP, particularly for sending data
formatted with the SECS-II ([SEMI E5]) Presentation Protocol and
understood by the GEM ([SEMI E30]) Application Protocol (together known as
SECS/GEM).

For ease of programming and extension, the functionality of [HSMS] has been
divided into a few subsets, the Primitive Services, the HSMS Generic
Services, and the HSMS Single Selected Session Services.

-------------------------------------------------------------------------------

## Primitive Services

Defines the most agnostic form in which data can be exchanged persuant to
the HSMS protocol and any subsidary protocols. This is not necessarily
outlined by the standard, but is an important piece of establishing and
maintaining proper communications.

To use the Primitive Services:
- Build [Primitive Message]s which use [Primitive Message Header]s.
- Create a [Primitive Client] with the [New Primitive Client] function.
- Manage the [Connection State] with the [Primitive Connect Procedure]
  and [Primitive Disconnect Procedure].
- Receive [Primitive Message]s with the hook provided by the
  [Primitive Connect Procedure].
- Transmit [Primitive Message]s with the [Primitive Transmit Procedure].

-------------------------------------------------------------------------------

## HSMS Generic Services

Defines the full functionality of the [HSMS] protocol without modification
by any subsidiary standards.

To use the HSMS Generic Services:
- Build [HSMS Message]s which use an [HSMS Message ID] and
  [HSMS Message Contents]:
  - [Data Message]
  - [Select.req]
  - [Select.rsp]
  - [Deselect.req]
  - [Deselect.rsp]
  - [Linktest.req]
  - [Linktest.rsp]
  - [Reject.req]
  - [Separate.req]
- Create an [HSMS Client] by providing the [New HSMS Client] function
  with [Parameter Settings].
- Manage the [Connection State] with the [HSMS Connect Procedure] and
  [HSMS Disconnect Procedure].
- Manage the [Selection State] with the [HSMS Select Procedure],
  [HSMS Deselect Procedure], and [HSMS Separate Procedure].
- Receive [Data Message]s with the hook provided by the
  [HSMS Connect Procedure].
- Test connection integrity with the [HSMS Linktest Procedure].
- Send [Data Message]s with the [HSMS Data Procedure].
- Send [Reject.req] messages [HSMS Reject Procedure].

-------------------------------------------------------------------------------

## HSMS Single Selected-Session Services

Not yet implemented.

-------------------------------------------------------------------------------

## TO BE DONE

- [HSMS Client] - [HSMS Deselect Procedure]
- [HSMS Client] - "Simultaneous Deselect Procedure"
- [HSMS Client] - [HSMS Separate Procedure]
- [HSMS Client] - [HSMS Reject Procedure]
- HSMS-SS/SEMI E37.1

[SEMI E5]:  https://store-us.semi.org/products/e00500-semi-e5-specification-for-semi-equipment-communications-standard-2-message-content-secs-ii
[SEMI E30]: https://store-us.semi.org/products/e03000-semi-e30-specification-for-the-generic-model-for-communications-and-control-of-manufacturing-equipment-gem
[SEMI E37]: https://store-us.semi.org/products/e03700-semi-e37-high-speed-secs-message-services-hsms-generic-services

[HSMS]:                           https://docs.rs/semi_e37/0.1.1/semi_e37/index.html
[Primitive Message]:              https://docs.rs/semi_e37/0.1.1/semi_e37/struct.PrimitiveMessage.html
[Primitive Message Header]:       https://docs.rs/semi_e37/0.1.1/semi_e37/struct.PrimitiveMessageHeader.html
[Primitive Client]:               https://docs.rs/semi_e37/0.1.1/semi_e37/struct.PrimitiveClient.html
[New Primitive Client]:           https://docs.rs/semi_e37/0.1.1/semi_e37/struct.PrimitiveClient.html#method.new
[Primitive Connect Procedure]:    https://docs.rs/semi_e37/0.1.1/semi_e37/struct.PrimitiveClient.html#method.connect
[Primitive Disconnect Procedure]: https://docs.rs/semi_e37/0.1.1/semi_e37/struct.PrimitiveClient.html#method.disconnect
[Primitive Transmit Procedure]:   https://docs.rs/semi_e37/0.1.1/semi_e37/struct.PrimitiveClient.html#method.transmit
[Connection State]:               https://docs.rs/semi_e37/0.1.1/semi_e37/enum.ConnectionState.html
[Connection Mode]:                https://docs.rs/semi_e37/0.1.1/semi_e37/enum.ConnectionMode.html

[HSMS Message]:                   https://docs.rs/semi_e37/0.1.1/semi_e37/struct.HsmsMessage.html
[HSMS Message ID]:                https://docs.rs/semi_e37/0.1.1/semi_e37/struct.HsmsMessageID.html
[HSMS Message Contents]:          https://docs.rs/semi_e37/0.1.1/semi_e37/enum.HsmsMessageContents.html
[Data Message]:                   https://docs.rs/semi_e37/0.1.1/semi_e37/enum.HsmsMessageContents.html#variant.DataMessage
[Select.req]:                     https://docs.rs/semi_e37/0.1.1/semi_e37/enum.HsmsMessageContents.html#variant.SelectRequest
[Select.rsp]:                     https://docs.rs/semi_e37/0.1.1/semi_e37/enum.HsmsMessageContents.html#variant.SelectResponse
[Deselect.req]:                   https://docs.rs/semi_e37/0.1.1/semi_e37/enum.HsmsMessageContents.html#variant.DeselectRequest
[Deselect.rsp]:                   https://docs.rs/semi_e37/0.1.1/semi_e37/enum.HsmsMessageContents.html#variant.DeselectResponse
[Linktest.req]:                   https://docs.rs/semi_e37/0.1.1/semi_e37/enum.HsmsMessageContents.html#variant.LinktestRequest
[Linktest.rsp]:                   https://docs.rs/semi_e37/0.1.1/semi_e37/enum.HsmsMessageContents.html#variant.LinktestResponse
[Reject.req]:                     https://docs.rs/semi_e37/0.1.1/semi_e37/enum.HsmsMessageContents.html#variant.RejectRequest
[Separate.req]:                   https://docs.rs/semi_e37/0.1.1/semi_e37/enum.HsmsMessageContents.html#variant.SeparateRequest
[HSMS Client]:                    https://docs.rs/semi_e37/0.1.1/semi_e37/struct.HsmsClient.html
[New HSMS Client]:                https://docs.rs/semi_e37/0.1.1/semi_e37/struct.HsmsClient.html#method.new
[HSMS Connect Procedure]:         https://docs.rs/semi_e37/0.1.1/semi_e37/struct.HsmsClient.html#method.connect
[HSMS Disconnect Procedure]:      https://docs.rs/semi_e37/0.1.1/semi_e37/struct.HsmsClient.html#method.disconnect
[HSMS Data Procedure]:            https://docs.rs/semi_e37/0.1.1/semi_e37/struct.HsmsClient.html#method.data
[HSMS Select Procedure]:          https://docs.rs/semi_e37/0.1.1/semi_e37/struct.HsmsClient.html#method.select
[HSMS Deselect Procedure]:        https://docs.rs/semi_e37/0.1.1/semi_e37/struct.HsmsClient.html#method.deselect
[HSMS Linktest Procedure]:        https://docs.rs/semi_e37/0.1.1/semi_e37/struct.HsmsClient.html#method.linktest
[HSMS Separate Procedure]:        https://docs.rs/semi_e37/0.1.1/semi_e37/struct.HsmsClient.html#method.separate
[HSMS Reject Procedure]:          https://docs.rs/semi_e37/0.1.1/semi_e37/struct.HsmsClient.html#method.reject
[Selection State]:                https://docs.rs/semi_e37/0.1.1/semi_e37/enum.SelectionState.html
[Parameter Settings]:             https://docs.rs/semi_e37/0.1.1/semi_e37/struct.ParameterSettings.html

[Connection State Transition]:    https://docs.rs/semi_e37/0.1.1/semi_e37/enum.ConnectionStateTransition.html
[Presentation Type]:              https://docs.rs/semi_e37/0.1.1/semi_e37/enum.PresentationType.html
[Session Type]:                   https://docs.rs/semi_e37/0.1.1/semi_e37/enum.SessionType.html
[Select Status]:                  https://docs.rs/semi_e37/0.1.1/semi_e37/enum.SelectStatus.html
[Deselect Status]:                https://docs.rs/semi_e37/0.1.1/semi_e37/enum.DeselectStatus.html
[Reject Reason]:                  https://docs.rs/semi_e37/0.1.1/semi_e37/enum.RejectReason.html
