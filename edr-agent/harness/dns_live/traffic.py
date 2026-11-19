#!/usr/bin/env python3
# harness/dns_live/traffic.py
# Sends the DNS traffic for the live test while dns_live is attached.
# No DNS server has to answer. The kprobes see each datagram as it is sent,
# so sends to a closed port on 127.0.0.1 are enough.

import ctypes
import os
import socket
import struct

A = 1
CNAME = 5
TXT = 16
AAAA = 28
BASE32 = "abcdefghijklmnopqrstuvwxyz234567"


def build_query(name, qtype, arcount=0):
    # 12-byte header (RD set, one question), then the name, qtype, class IN.
    header = struct.pack("!HHHHHH", 0x1234, 0x0100, 1, 0, 0, arcount)
    body = b""
    for label in name.split("."):
        body += bytes([len(label)]) + label.encode()
    return header + body + b"\x00" + struct.pack("!HH", qtype, 1)


# EDNS(0) OPT record for the additional section: root name, type 41, 1232-byte UDP size.
EDNS_OPT = b"\x00" + struct.pack("!HHIH", 41, 1232, 0, 0)


def step(text):
    print("step: " + text, flush=True)


def send_udp(family, host, port, payload):
    sock = socket.socket(family, socket.SOCK_DGRAM)
    try:
        sock.sendto(payload, (host, port))
    except OSError as err:
        print("    send error (the probe still fires on entry): " + str(err), flush=True)
    sock.close()


def pseudo_random(alphabet, length, seed):
    # Same xorshift as the Rust tests, so runs are repeatable.
    x = seed
    out = ""
    for _ in range(length):
        x ^= (x << 13) & 0xFFFFFFFFFFFFFFFF
        x ^= x >> 7
        x ^= (x << 17) & 0xFFFFFFFFFFFFFFFF
        out += alphabet[x % len(alphabet)]
    return out


def split_labels(data, size):
    parts = []
    i = 0
    while i < len(data):
        parts.append(data[i:i + size])
        i += size
    return ".".join(parts)


print("traffic pid=" + str(os.getpid()), flush=True)

# 0. A real lookup through the glibc stub resolver (whatever /etc/resolv.conf names).
step("0 getaddrinfo(www.example.com) through glibc")
os.environ["RES_OPTIONS"] = "timeout:1 attempts:1"
try:
    socket.getaddrinfo("www.example.com", 80, proto=socket.IPPROTO_UDP)
except OSError as err:
    print("    resolver error: " + str(err), flush=True)

# 1. Normal query, unconnected socket, sendto().
step("1 sendto www.example.com A -> 127.0.0.1:53")
send_udp(socket.AF_INET, "127.0.0.1", 53, build_query("www.example.com", A))

# 2. Long tunnel-shaped TXT query, iodine style: base32 data in 57-character labels.
iodine_name = split_labels(pseudo_random(BASE32, 170, 7), 57) + ".t.tunnel-test.net"
step("2 sendto iodine-style TXT (" + str(len(iodine_name)) + " chars) -> 127.0.0.1:53")
send_udp(socket.AF_INET, "127.0.0.1", 53, build_query(iodine_name, TXT))

# 2b. dnscat2 style: hex data in full 63-character labels, CNAME.
hex_name = split_labels(pseudo_random("0123456789abcdef", 189, 11), 63) + ".dnscat-test.net"
step("2b sendto dnscat-style CNAME (" + str(len(hex_name)) + " chars) -> 127.0.0.1:53")
send_udp(socket.AF_INET, "127.0.0.1", 53, build_query(hex_name, CNAME))

# 3. Burst of 30 short unique subdomains of one base domain. No single query looks odd.
step("3 burst of 30 unique subdomains of burst-test.net, A")
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
for i in range(30):
    name = "%02d%s.burst-test.net" % (i, pseudo_random("0123456789abcdef", 8, i + 1))
    sock.sendto(build_query(name, A), ("127.0.0.1", 53))
sock.close()

# 4a. Connected socket: connect() then send(), so msg_name is NULL in the kernel.
step("4a connect + send connected-path.example.org A")
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock.connect(("127.0.0.1", 53))
sock.send(build_query("connected-path.example.org", A))
sock.close()

# 4b. sendmsg() with one buffer (ITER_UBUF on 6.4 and later).
step("4b sendmsg one buffer sendmsg-one.example.org A")
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock.sendmsg([build_query("sendmsg-one.example.org", A)], [], 0, ("127.0.0.1", 53))
sock.close()

# 4c. sendmsg() with two buffers (ITER_IOVEC): header and question, then an EDNS OPT record.
step("4c sendmsg two buffers sendmsg-two.example.org A plus EDNS")
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock.sendmsg([build_query("sendmsg-two.example.org", A, arcount=1), EDNS_OPT], [], 0, ("127.0.0.1", 53))
sock.close()


# 4d. sendmmsg() with two messages, through libc since Python has no wrapper.
class IoVec(ctypes.Structure):
    _fields_ = [("iov_base", ctypes.c_void_p), ("iov_len", ctypes.c_size_t)]


class MsgHdr(ctypes.Structure):
    _fields_ = [
        ("msg_name", ctypes.c_void_p),
        ("msg_namelen", ctypes.c_uint32),
        ("msg_iov", ctypes.POINTER(IoVec)),
        ("msg_iovlen", ctypes.c_size_t),
        ("msg_control", ctypes.c_void_p),
        ("msg_controllen", ctypes.c_size_t),
        ("msg_flags", ctypes.c_int),
    ]


class MMsgHdr(ctypes.Structure):
    _fields_ = [("msg_hdr", MsgHdr), ("msg_len", ctypes.c_uint)]


step("4d sendmmsg two messages sendmmsg-1.example.org and sendmmsg-2.example.org A")
libc = ctypes.CDLL(None, use_errno=True)
addr = ctypes.create_string_buffer(struct.pack("=H", socket.AF_INET) + struct.pack("!H", 53) + socket.inet_aton("127.0.0.1") + b"\x00" * 8)
buffers = [ctypes.create_string_buffer(build_query("sendmmsg-%d.example.org" % n, A)) for n in (1, 2)]
iovecs = (IoVec * 2)()
msgs = (MMsgHdr * 2)()
for n in range(2):
    iovecs[n].iov_base = ctypes.cast(buffers[n], ctypes.c_void_p)
    iovecs[n].iov_len = len(buffers[n].raw) - 1  # create_string_buffer adds a trailing NUL
    msgs[n].msg_hdr.msg_name = ctypes.cast(addr, ctypes.c_void_p)
    msgs[n].msg_hdr.msg_namelen = 16
    msgs[n].msg_hdr.msg_iov = ctypes.pointer(iovecs[n])
    msgs[n].msg_hdr.msg_iovlen = 1
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sent = libc.sendmmsg(sock.fileno(), msgs, 2, 0)
print("    sendmmsg returned " + str(sent), flush=True)
sock.close()

# 5. IPv6 to ::1, unconnected and connected.
step("5a sendto ipv6-path.example.net AAAA -> [::1]:53")
send_udp(socket.AF_INET6, "::1", 53, build_query("ipv6-path.example.net", AAAA))
step("5b connect + send ipv6-connected.example.net AAAA -> [::1]:53")
sock = socket.socket(socket.AF_INET6, socket.SOCK_DGRAM)
try:
    sock.connect(("::1", 53))
    sock.send(build_query("ipv6-connected.example.net", AAAA))
except OSError as err:
    print("    send error: " + str(err), flush=True)
sock.close()

# 5c. IPv6 socket to a v4-mapped address. The kernel hands this to udp_sendmsg,
# so it must show up once, as AF_INET.
step("5c sendto v4mapped-path.example.net A -> [::ffff:127.0.0.1]:53 from an AF_INET6 socket")
send_udp(socket.AF_INET6, "::ffff:127.0.0.1", 53, build_query("v4mapped-path.example.net", A))

# 6. Same DNS payload to other ports. None of these may appear.
for port in (5353, 9999):
    step("6 sendto should-not-appear.example.com A -> 127.0.0.1:" + str(port))
    send_udp(socket.AF_INET, "127.0.0.1", port, build_query("should-not-appear.example.com", A))
step("6 sendto should-not-appear.example.com A -> [::1]:9999")
send_udp(socket.AF_INET6, "::1", 9999, build_query("should-not-appear.example.com", A))

# 7. The same name 25 times is not volume.
step("7 sendto repeat.example.com A, 25 times")
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
for i in range(25):
    sock.sendto(build_query("repeat.example.com", A), ("127.0.0.1", 53))
sock.close()

# 8. Garbage to port 53 must be captured but not parsed.
step("8 sendto 20 bytes of garbage -> 127.0.0.1:53")
send_udp(socket.AF_INET, "127.0.0.1", 53, b"\xff" * 20)

# Stop marker for dns_live --stop-on.
step("end sendto end-of-test.invalid A -> 127.0.0.1:53")
send_udp(socket.AF_INET, "127.0.0.1", 53, build_query("end-of-test.invalid", A))
print("traffic done", flush=True)
