# IP v6


## SLAAC

In IPv4, there are two common ways to provide addressing:
- Static addressing
- Dynamic address configuration via DHCP

DHCP (Dynamic Host Configuration Protocol) provides a method to dynamically assign IP addresses, but it can also give out other service information like DNS server addresses, domain names, and custom information.

IPv6 address configuration methods include:
- Static addressing
- Static addressing with DHCPv6 (Stateless)
- Dynamic addressing via DHCPv6 (Stateful)
- SLAAC alone
- SLAAC with DHCPv6 (Stateless)

IPv6 static addressing works the same as in IPv4; implementing DHCP can be stateful and stateless.

**Stateful DHCP** server tracks the IP addresses it leases.

**Stateless DHCP** provides DNS server information (it doesn't lease IPv6 addresses and it doesn't track the advertised info) and it is paired with another mechanism (with **Static addressing**, or with SLAAC) to complete IPv6 address assignment.

**SLAAC** provides the ability to address a host based on a network prefix that is advertised from a local network router via RA.

**Router Advertisements (RA)** messages, by default periodically sent by most IPV6 routers, contain information such as:
- one or more IPv6 prefixes (link-local scope)
- prefix lifetime information
- flags
- default device information (default router to use and its lifetime)

SLAAC is implemented on the IPv6 client by listening for these local RA’s and then taking the prefix that is advertised to form a unique address that can be used on the network. For this to work, the prefix that is advertised must advertise a prefix length of 64 bits (i.e., /64); SLAAC will then dynamically form a host identifier that is 64 bits long and will be suffixed to the end of the advertised prefix to form an IPv6 address. Originally, the host identifier was formed using the EUI-64 rules (the same that are used to form link local addresses) and many devices still use this method. However, some Microsoft operating systems by default do not use this original method. Instead, they take advantage of some additional privacy extensions that were defined in RFC4941.