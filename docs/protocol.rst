Protocol Specification
======================

.. |MHP| replace:: Metarhia Protocol

Introduction
------------

|MHP| is an RPC, session and binary data transfer protocol that provides
two-way asynchronous data transfer, multiplexing several RPC channels, event
streams and binary data transmissions over one socket, and graceful handling of
short-time connection losses due to network errors with full and transparent
session restoration.  It also provides authentication mechanisms, offers data
compression and supports multiple serialization formats with each of those
being more appropriate or efficient for different kinds of data.

Terms
-----

.. important::

   The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
   "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this
   document are to be interpreted as described in `RFC 2119`_.

**Transport** refers to a network protocol or another communication mechanism
that provides full-duplex communication channel with ordered and reliable data
flow, to which |MHP| delegates transmission of raw data streams.

**Transport connection** refers to an underlying transport socket, connection
or other transport-specific communication channel, over which a |MHP|
connection transmits data.

**Metarhia Protocol connection** (or just **connection** without additional
adjectives) refers to an abstraction over an instance of transport connection
managed by a |MHP| implementation, that hides implementation details of the
transport and provides the functionality of |MHP| to user applications.  |MHP|
connections and transport connections have one-to-one correspondence.

**Chunk** refers to a data unit of |MHP| consisting of headers and optional
payload.  All data transmitted via |MHP| are split into chunks.

**Channel** refers to a set of chunks transmitted over the same connection,
identified among other ones using a number that is unique throughout the
currently active channels connection-wise.  Channels provide multiplexing
capabilities for connections.

**Message** refers to a channel that is characterized by a short lifetime and
small number of chunks (only one chunk in most cases, with the maximal amount
specified in section ???) that are buffered in memory until the message is
received, and then processed as a single unit by an application.

**Stream** refers to a channel that is characterized by arbitrarily long
lifetime (up to existing as long as the connection exists) and indefinite
number of chunks, which may be processed by an application one by one
immediately after they become available.

**Session** is a persistent association between applications on both sides of a
connection.  Sessions may be anonymous and authenticated.  If a connection is
lost because of network failure, its sessions can be restored after
reconnection, either all of them or only those a client wants to.  Channels are
bound to corresponding sessions.

**RPC** is an acronym for remote procedure calls.

**AEAD** is an acronym for Authenticated Encryption with Associated Data.

.. _RFC 2119: https://tools.ietf.org/html/rfc2119

Transport Support
-----------------

|MHP| implementations primarily targeted to be used in server-side environments
MUST support `TCP`_, `TLS`_, `WebSocket`_ and WebSocket tunneled over TLS
protocols as transports.  |MHP| implementations primarily targeted to be used
in server-side environments MAY support additional transports, for example,
Unix domain sockets.

|MHP| implementations designed specifically to be used in client-side
environments SHOULD support TCP and TLS transports.  In those cases where it is
not possible (for example, in implementations for Web browsers), such
implementations MUST support WebSocket and WebSocket tunneled over TLS as
transports, and MAY support them otherwise.  In other words, at least one of
"TCP and TLS" or "WebSocket and WebSocket over TLS" pairs of transports MUST be
supported, with preference towards TCP and TLS.  Client-side implementations
MAY support other transports, if their implementors find it reasonable.

Connection States
-----------------

``AWAITING_HANDSHAKE``
^^^^^^^^^^^^^^^^^^^^^^

A transport connection has opened, but handshake hasn't been performed
and there is no session established.

A client sends a `Protocol Handshake`_ chunk.  When the handshake is
performed successfully, the connection transitions into `AWAITING_SESSION`_
state.

A server waits a `Protocol Handshake`_ chunk from the client.  When the
handshake is performed successfully, the connection transitions into
`AWAITING_SESSION`_ state.

``AWAITING_SESSION``
^^^^^^^^^^^^^^^^^^^^

The client sends a `Session Establishment Request`_ chunk.  The server responds
either with a `New Session Establishment Response`_ chunk, or, in the case when
an existing session is being restored, with a Ping chunk, to which the client
responds with a Pong chunk, and both sides re-send all the chunks they did not
receive.  After that, both connections transition into `NORMAL`_ state.

``NORMAL``
^^^^^^^^^^

This is the main mode of operation.  All the communication is performed using
channels and ping/pong chunks.  On network error, the connection transitions
into `NETWORK_CONN_LOST`_ state.

``NETWORK_CONN_LOST``
^^^^^^^^^^^^^^^^^^^^^

The client buffers all outgoing chunks and tries to reconnect to the server.
On success, the connection transitions into `AWAITING_HANDSHAKE`_ state.

The server buffers all outgoing chunks and awaits a new connection from the
client.

Chunk Formats
-------------

.. note::

   |MHP| uses little-endian byte order.

Protocol Handshake
^^^^^^^^^^^^^^^^^^

+-----------------------------+------+
| Field                       | Bits |
+=============================+======+
| ``Version``                 | 16   |
+-----------------------------+------+
| ``Encryption``              | 16   |
+-----------------------------+------+
| ``Payload``                        |
+------------------------------------+

``Version`` field indicates the version of the protocol to use.  This document
describes |MHP| version ``1``.

Currently, the only possible value of ``Encryption`` is ``0`` and the payload
is empty.

When new possible values of ``Encryption`` are added, they may require adding
new handshake chunks to implement, e.g., key exchange.  When ``Encryption`` is
``0``, no additional data is required for the protocol handshake, and |MHP|
sessions may be opened or restored over the connection immediately.

Session Establishment Request
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

+-----------------------------+------+
| Field                       | Bits |
+=============================+======+
| ``Token``                   | 256  |
+-----------------------------+------+

``Token`` is a 32-byte session ID and session secret key.  ``0`` is a special
value reserved to indicate that a new session must be created, instead of
restoring an existing one.

New Session Establishment Response
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

+-----------------------------+------+
| Field                       | Bits |
+=============================+======+
| ``Token``                   | 256  |
+-----------------------------+------+

``Token`` is a 32-byte random string, obtained from a cryptographically secure
source.  It serves both as a session ID and a session secret key.  ``Token``
must not be equal to ``0``.

Channel Preamble
^^^^^^^^^^^^^^^^

+-----------------------------+------+
| Field                       | Bits |
+=============================+======+
| ``Id``                      | 32   |
+-----------------------------+------+
| ``Type``                    | 8    |
+-----------------------------+------+
| ``Compression``             | 8    |
+-----------------------------+------+
| ``MessagePreambleReserved`` | 16   |
+-----------------------------+------+

To avoid collisions because of unsynchronized channel counters on the sides of
a connection, the most significant bit of the ``Id`` field is masked to be
always equal to 0 for channels initiated by clients and 1 for channels
initiated by servers by making the field an signed integer, two's complement.
In other words, the valid values of the ``Id`` field of client-initiated
channels are :math:`{[0, 2^{31} - 1]}` and the valid values of the ``Id`` field
of server-initiated channels are :math:`{[{-2^{31}}, -1]}`.  The ``Id`` value
MUST be unique throughout the currently active channels.

The ``Type`` field MUST be equal to one of the following values:

+-------------+
| Type        |
+===+=========+
| 0 | Message |
+---+---------+
| 1 | Stream  |
+---+---------+

If ``Type`` is ``0``, then the chunk is a `Message Preamble`_.

The ``Compression`` field indicates if the payload of subsequent `data
chunks`__ in this channel is compressed.  The field MUST be set to one of the
following values:

+----------------------+
| Compression          |
+===+==================+
| 0 | No compression   |
+---+------------------+
| 1 | Gzip compression |
+---+------------------+

__ `Data Chunk`_

Message Preamble
^^^^^^^^^^^^^^^^

See `Channel Preamble`_.

+-----------------------------+------+
| Field                       | Bits |
+=============================+======+
| ``Id``                      | 32   |
+-----------------------------+------+
| ``ChannelType``             | 8    |
+-----------------------------+------+
| ``Compression``             | 8    |
+-----------------------------+------+
| ``Encoding``                | 8    |
+-----------------------------+------+
| ``MessageType``             | 8    |
+-----------------------------+------+

This chunk type extends the generic `Channel Preamble`_, adding two new fields
instead of the ``MessagePreambleReserved`` field, namely, ``Encoding`` and
``MessageType``.

The ``Encoding`` field specifies the format used to encode the payload fields
of messages that require passing arbitrary data (e.g., arguments of RPC methods
in ``Call`` messages).  It MUST be set to one of the following values:

+----------+
| Encoding |
+===+======+
| 0 | JSTP |
+---+------+
| 1 | JSON |
+---+------+

The ``MessageType`` field MUST be set to one of the following values:

+-----------------------------+
| MessageType                 |
+====+========================+
| 0  | ``HandshakeRequest``   |
+----+------------------------+
| 1  | ``HandshakeResponse``  |
+----+------------------------+
| 2  | ``Event``              |
+----+------------------------+
| 3  | ``Call``               |
+----+------------------------+
| 4  | ``Callback``           |
+----+------------------------+
| 5  | ``Inspect``            |
+----+------------------------+
| 6  | ``InspectCallback``    |
+----+------------------------+
| 7  | ``Ping``               |
+----+------------------------+
| 8  | ``Pong``               |
+----+------------------------+

Data Chunk
^^^^^^^^^^

+-----------------------------+------+
| Field                       | Bits |
+=============================+======+
| ``ChannelId``               | 32   |
+-----------------------------+------+
| ``Length``                  | 16   |
+-----------------------------+------+
| ``Flags``                   | 16   |
+-----------------------------+------+
| ``Payload``                        |
+------------------------------------+

The ``ChannelId`` field specifies a channel the chunk belongs to.  The
``Length`` field contains the size of the payload in bytes.  The ``Flags``
field has the following structure:

+-----------------------+
| Flags                 |
+============+==========+
| Bits 15--1 | Reserved |
+------------+----------+
| Bit 0      | ``More`` |
+------------+----------+

Flag ``More`` specifies if the channel has more chunks.  Reserved flags MUST be
set to ``0``.

.. _TCP: https://tools.ietf.org/html/rfc793
.. _TLS: https://tools.ietf.org/html/rfc5246
.. _WebSocket: https://tools.ietf.org/html/rfc6455

"Fast" UDP Events Encryption (ignore this for now)
--------------------------------------------------

.. note::

   I drafted this section while I was in context so as to not forget about all
   I thought about this; the things this would be needed for aren't quite there
   yet ;)

AEAD Algorithm Requirements and Motivation
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

For chunks that use symmetric encryption (for example, "fast" UDP events), AEAD
based on the `ChaCha20`_ stream cipher and `Poly1305`_ message authentication
code algorithm with modifications from IETF (`RFC 7539`_) MUST be used.

ChaCha20 and Poly1305 are modern, secure, high-speed algorithms developed by
Daniel J. Berstein, that have undergone scrupulous analysis in multiple
scientific papers and are under constantly growing adoption now.  As some
examples:

* Google has used their implementation of these algorithms for TLS traffic
  between Google Chrome on Android and Google's servers since 2014.

* `TLS 1.3`_ draft has ``TLS_CHACHA20_POLY1305_SHA256`` cipher suite, and
  recommends implementing it.

IETF versions of ChaCha20, ChaCha20-Poly1305 and ChaCha20-Poly1305 AEAD
specified in `RFC 7539`_ modify Berstein's algorithm by changing 64-bit nonce
to 96-bit nonce, so 64-bit block counter is reduced 32-bit block counter,
effectively limiting the size of a message to 256 GB (instead of 2\ :sup:`64`
bytes).

Poly1305 is proved to be secure using the same key for at least 2\ :sup:`64`
messages, provided that nonces are never reused.

.. _ChaCha20: https://cr.yp.to/chacha.html
.. _Poly1305: https://cr.yp.to/mac.html
.. _RFC 7539: https://tools.ietf.org/html/rfc7539
.. _TLS 1.3: https://tools.ietf.org/html/draft-ietf-tls-tls13-21

Symmetric Encryption Implementation
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Upon creation of a ``Session`` structure instance, the following fields
relevant to the symmetric encryption facilities (with one of them not being
limited to this scope only) are initialized:

- ``secret`` — a 32-byte unsigned integer value
- ``nonce`` — a 12-byte unsigned integer value

``nonce`` value MUST be initialized with random data from a cryptographically
secure source.

If the ``Session`` structure is created on the side of a client, the least
significant bit of ``nonce`` MUST be set to 0.  If the ``Session`` structure is
created on the side of a server, the least significant bit of ``nonce`` MUST be
set to 1.

If the ``Session`` structure is created on the side of a server, ``secret``
value MUST be initialized with random data from a cryptographically secure
source.

The server shares this value with the client during the handshake, as described
in section ???.  When the client receives this value, it MUST initialize the
``secret`` field of its ``Session`` structure with the received value.

.. DANGER::

   This procedure MAY be conducted over a connection that is not secured using
   TLS or other method of asymmetric encryption and server authentication in a
   local or trusted environment, or on a single machine during testing, but one
   SHOULD NOT do so over a publicly accessible network.  Security may be
   compromised in such case.  Only connections secured with TLS (or an
   alternative method) SHOULD be used with |MHP| in public networks.

When symmetric encryption of a chunk is requested, |MHP| implementations MUST
follow the next algorithm:

1. **Let** *secret* := **Get** *secret* from *Session*.
2. **Let** *nonce* := **Get** *nonce* from *Session*.
3. **Let** *data* := **Input**.
4. **Let** *result* := AEAD\_ChaCha20\_Poly1305\_IETF\_Encrypt(*data*, *secret*,
   *nonce*).
5. **Set** *nonce* in *Session* := *nonce* + 2.
6. **Output** := *result*.

When symmetric decryption of a chunk is requested, |MHP| implementations MUST
follow the next algorithm:

1. **Let** *secret* := **Get** *secret* from *Session*.
2. **Let** *data* := **Input**.
3. **Let** *result* := AEAD\_ChaCha20\_Poly1305\_IETF\_Decrypt(*data*,
   *secret*).
4. **Output** := *result*.
