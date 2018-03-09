# net module

- Module `std::net`, 1.0.0
- Networking primitives for the TCP/UDP; IP v4/v6 types; socket addresses.
- `TcpListener` and `TcpStream` provide functionality for TCP communication.
- `UdpSocket` provides functionality for communication over UDP.
- `IpAddr` represents `Ipv4Addr` and `Ipv6Addr`.
- `SocketAddr` represents `SocketAddrV4` and `SocketAddrV6` socket addresses.
- `ToSocketAddrs` trait used for generic address resolution when interacting with networking objects like `TcpListener`, `TcpStream`, `UdpSocket`.
- Other types are return or parameter types for various methods in the module.


## Structs
- `AddrParseError` error from parsing IP or socket address.
- `Incoming` iterator that infinitely accepts connections on a TcpListener.
- `Ipv4Addr` IPv4 address.
- `Ipv6Addr` IPv6 address.
- `SocketAddrV4` IPv4 socket address.
- `SocketAddrV6` IPv6 socket address.
- `TcpListener` TCP socket server, listening for connections.
- `TcpStream` TCP stream between a local and a remote socket.
- `UdpSocket` UDP socket.
- ~~`LookupHost` iterator over `SocketAddr` values returned from a host lookup operation. (Deprecated, Experimental)~~

## Enums
- `IpAddr` IPv4 or IPv6 address.
- `Shutdown` Possible values passed to the `shutdown` method of `TcpStream`.
- `SocketAddr` An internet socket address, either IPv4 or IPv6.
- `Ipv6MulticastScope` (Experimental).

## Traits
- `ToSocketAddrs` convertable or resolvable objects to SocketAddr.

## Functions
- ~~`lookup_host` Resolve the host specified by host as a number of `SocketAddr` instances. (Deprecated, Experimental)~~
