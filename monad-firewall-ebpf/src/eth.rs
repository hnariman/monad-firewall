// Ethernet Frames have standard, which evolved by as tech evolved etc
// details are available here: https://en.wikipedia.org/wiki/Ethernet_frame
//
// PCI NICs and virtual eth (veth) will have this frame standard 99% of a time
// so with eBPF we're reading those frames from DMA and parsing them basically
//
// +--------------------------+----------------------+------------------------------+
// | Destination MAC (6 byes) | Source MAC (6 bytes) | Eth Type (2 byes, BigEndian) |
// +--------------------------+----------------------+------------------------------+
//
// EthType is usually one of those 3 options:
// 0x0800 -> IPv4 Header
// 0x86DD -> IPv6 Header
// 0x0806 -> ARP
//
// for VLAN and QinQ tagging sometimes additional data (tags) are added (4 bytes) before inner EthType
// +-------------------+-------------------+----------------+--------------+-------------------------+
// | Dest MAC (6 byes) | Src MAC (6 bytes) | TPID (2 bytes) | TCI (2 byes) | Inner EthType (2 bytes) |
// +-------------------+-------------------+----------------+--------------+-------------------------+
// which means we need to make additional check if vlan or double tagging is used
//
// 0x8100 -> VLAN (IEEE 802.1Q - single vlan tagging)
// 0x88A8 -> QinQ (IEEE 802.1ad - double tagging)
// (usecase: separation/isolation of LAN and nested separation inside existing VLAN)
// more about double VLAN tagging (QinQ) and eth frame shape:
// https://en.wikipedia.org/wiki/IEEE_802.1ad
//
// if EthType is any of those, we need to advance pointer offset 
// by 4 bytes for single tagging and
// by 8 bytes for double tagging
//
// ---- L2 EDGE-CASES ----
// Abovementioned is subject to standard L2 network, but sometimes there are edgecases:
//
// 1) TAP - are kernel virtual network interfaces, which operate at L2 (Data Link Layer)
//    are sending raw eth data through software instead of physical wires, will have APR as well
//    (usecases: VM's, QEMU etc), since those will have virtual MAC addresses eg 52:54:00 etc
//    more info: 
//    https://en.wikipedia.org/wiki/TUN/TAP
//    https://docs.kernel.org/networking/tuntap.html
//
//    +--------------------+----------------------+-------------------+-------------=+
//    | Dest MAC (6 bytes) | Source MAC (6 bytes) | EthType (2 bytes) | L3 Ip Header |
//    +--------------------+----------------------+-----=-------------+--------------+
//
//    ctx.data -> starts at EthHeader (Dest MAC)
//
// 2) TUN - kernel virtual network interfaces operating as L3 level 
//    this happens because those virtual interfaces don't have & intentionally don't emulate L2 (hardware) data
//    will not have any MAC address, also will not have ARP and will not respond to ARP
//    (usecases: Tunnels, Wireguard, Tailscale, OPenVPN etc)
//
//    +--------------+---------------------+
//    | L3 Ip Header | L4 Transport Header |
//    +--------------+---------------------+
//
//    on userspace there is PI (Packet Information) header added, 
//    depending on configs (IFF_NO_PI, TUN_PKT_STRIP etc) extra bytes may be the case 
//    (but on XDP/Kernel level we don't have to worry about it)
//    userspace frame representation is somethiing like:
//
//    +-----------------+--------------------+--------------------+
//    | Flags (2 bytes) | Protocol (2 bytes) | Payload (L2 or L3) |
//    +-----------------+--------------------+--------------------+
//
//    ctx.data -> starts directly at IP header (0x45/0x60)
// 
// 3) loopback - is kernels default virtual network interface, it will have standard Ethernet
//    frame, but will use dummy 00:00:00:etc for src and dest MAC addresses (as there is no hardware, and no emulation), 
//    also there is no hardware address resolution (ARP) needed 
//    (usecase: route applications on the same host to each other using common protocols, 127.0.0.1,
//    ::1, and devices ip address)
//
//    +--------------------+----------------------+-------------------+-------------=+
//    | Dest MAC (6 bytes) | Source MAC (6 bytes) | EthType (2 bytes) | L3 Ip Header |
//    | 00:00:00:00:00:00  | 00:00:00:00:00:00    | 0x0800 / 0x86DD   | 0x45 / 0x60  |
//    +--------------------+----------------------+-----=-------------+--------------+
//
// 4) 802.3 (raw/legacy) - IEEE 802.3 has different packet frame structure (so possibly using
//    strategy pattern here would be in place).
//    meanwhile to define whether it's Ethernet 2 or 802.3 package we need to check 2 byte value at
//    offset 12-13 bytes[12..14] value 
//    if value > 1500 (0x05DC) then it's EthType and we're dealing with modern Ethernet 2 frame
//    else -> we've found rare 802.3 frame and it has different frame structure
//    (it's rarely seen in modern networks, but still an edge case possible)
//    https://www.ibm.com/support/pages/ethernet-version-2-versus-ieee-8023-ethernet
//    
// PS: ethernet frame would also have preambule (7 bytes) + 1 byte delimeter, 
// but this is something NIC has to handle, so it's out of scope (it's L1 level logic)
//
//
// --- Firewall ---
// firewall will be able to filter packets starting on L2 as there is EthType available,
// which already allows to drop malformed or spoofed non-IP EthTypes, which speed is nanoseconds
// and prevents sk_buffer from getting overwhelmed early on DDOS attacks etc
//
// also L2 allows to build strategies around loopback(lo), TUN/TAP, Veth 
//

