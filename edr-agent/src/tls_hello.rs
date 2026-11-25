// src/tls_hello.rs
// Reads the server name (SNI) and ALPN protocols from the start of a TLS ClientHello.
//
// net_flow.bpf.c copies the first write a process makes on a socket it connected to
// port 443 or 8443 (up to 1024 bytes). A TLS client sends its ClientHello in the clear
// before any key exists, so the name of the server it asked for is readable here
// without a key or a hook in the TLS library. Encrypted Client Hello hides the real
// name behind a public one, and this parser then reports the public name.
//
// Layout (RFC 8446 section 4.1.2, RFC 6066 section 3, RFC 7301 section 3.1):
//   record:    type 0x16 | version (2) | length (2)
//   handshake: type 0x01 | length (3) | client_version (2) | random (32)
//              | session_id (1 + n) | cipher_suites (2 + n) | compression (1 + n)
//              | extensions (2 + n), each: type (2) | length (2) | data
//   server_name (type 0): list length (2) | name_type 0 | name length (2) | name
//   ALPN (type 16): list length (2) | (length (1) | protocol)...

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Hello {
    /// The host name the client asked for.
    pub sni: Option<String>,
    /// Protocols offered with ALPN, such as h2 and http/1.1.
    pub alpn: Vec<String>,
    /// legacy_version from the handshake: 0x0303 for TLS 1.2 and 1.3.
    pub version: u16,
}

struct Cursor<'a> {
    b: &'a [u8],
    at: usize,
}

impl<'a> Cursor<'a> {
    fn u8(&mut self) -> Option<u8> {
        let v = *self.b.get(self.at)?;
        self.at += 1;
        Some(v)
    }
    fn u16(&mut self) -> Option<u16> {
        let v = u16::from_be_bytes([*self.b.get(self.at)?, *self.b.get(self.at + 1)?]);
        self.at += 2;
        Some(v)
    }
    fn skip(&mut self, n: usize) -> Option<()> {
        if self.at + n > self.b.len() {
            return None;
        }
        self.at += n;
        Some(())
    }
    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let s = self.b.get(self.at..self.at + n)?;
        self.at += n;
        Some(s)
    }
}

fn host_name(raw: &[u8]) -> Option<String> {
    let s = std::str::from_utf8(raw).ok()?;
    let ok = !s.is_empty()
        && s.len() <= 253
        && s.bytes().all(|c| c.is_ascii_alphanumeric() || matches!(c, b'.' | b'-' | b'_'));
    ok.then(|| s.to_ascii_lowercase())
}

/// The ClientHello at the start of `data`, or None when `data` does not start with
/// one. A copy cut short still gives the fields that fit.
pub fn parse_client_hello(data: &[u8]) -> Option<Hello> {
    let mut c = Cursor { b: data, at: 0 };
    if c.u8()? != 0x16 {
        return None;
    }
    let _record_version = c.u16()?;
    let _record_len = c.u16()?;
    if c.u8()? != 0x01 {
        return None;
    }
    c.skip(3)?; // handshake length
    let mut hello = Hello { version: c.u16()?, ..Default::default() };
    extensions(&mut c, &mut hello);
    Some(hello)
}

/// Skip to the extensions and read the two this module wants. Stops quietly where
/// the copy ends.
fn extensions(c: &mut Cursor, hello: &mut Hello) -> Option<()> {
    c.skip(32)?; // random
    let sid = c.u8()? as usize;
    c.skip(sid)?;
    let suites = c.u16()? as usize;
    c.skip(suites)?;
    let comp = c.u8()? as usize;
    c.skip(comp)?;
    let ext_len = c.u16()?;
    let end = (c.at + ext_len as usize).min(c.b.len());
    while c.at + 4 <= end {
        let (Some(ty), Some(len)) = (c.u16(), c.u16()) else {
            break;
        };
        let Some(body) = c.take(len as usize) else {
            break; // cut short by the copy
        };
        let mut e = Cursor { b: body, at: 0 };
        match ty {
            0 => {
                let _list = e.u16();
                if e.u8() == Some(0) {
                    if let Some(n) = e.u16() {
                        if let Some(raw) = e.take(n as usize) {
                            hello.sni = host_name(raw);
                        }
                    }
                }
            }
            16 => {
                let _list = e.u16();
                while let Some(n) = e.u8() {
                    match e.take(n as usize) {
                        Some(p) => hello.alpn.push(String::from_utf8_lossy(p).into_owned()),
                        None => break,
                    }
                }
            }
            _ => {}
        }
    }
    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Read, Write};

    /// A stream that keeps what the TLS client writes and never answers.
    #[derive(Debug)]
    struct Capture(Vec<u8>);
    impl Read for Capture {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Err(io::ErrorKind::WouldBlock.into())
        }
    }
    impl Write for Capture {
        fn write(&mut self, b: &[u8]) -> io::Result<usize> {
            self.0.extend_from_slice(b);
            Ok(b.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    /// The bytes OpenSSL sends first when it connects to `host`.
    fn openssl_hello(host: &str, alpn: &[u8]) -> Vec<u8> {
        use openssl::ssl::{SslConnector, SslMethod};
        let mut b = SslConnector::builder(SslMethod::tls()).unwrap();
        b.set_alpn_protos(alpn).unwrap();
        let conf = b.build().configure().unwrap();
        let stream = match conf.connect(host, Capture(Vec::new())) {
            Err(openssl::ssl::HandshakeError::WouldBlock(mid)) => mid,
            other => panic!("expected the handshake to wait for the server: {:?}", other.err().map(|e| e.to_string())),
        };
        stream.get_ref().0.clone()
    }

    #[test]
    fn reads_the_name_from_a_real_openssl_hello() {
        let b = openssl_hello("Bucket-X.s3.amazonaws.com", b"\x02h2\x08http/1.1");
        // The sensor copies at most the first 1024 bytes. OpenSSL 3.0 fits in that,
        // OpenSSL 3.5 does not (1,585 bytes, most of it a post-quantum key share), so
        // the parser reads what the sensor would have: the first 1024 bytes. OpenSSL
        // puts server_name and ALPN ahead of the key share, so both are in the cut.
        let seen = &b[..b.len().min(1024)];
        let h = parse_client_hello(seen).unwrap();
        assert_eq!(h.sni.as_deref(), Some("bucket-x.s3.amazonaws.com"), "hello of {} bytes", b.len());
        assert_eq!(h.alpn, vec!["h2".to_string(), "http/1.1".to_string()]);
        assert_eq!(h.version, 0x0303);
        // The whole hello parses to the same.
        assert_eq!(parse_client_hello(&b).unwrap().sni, h.sni);
    }

    #[test]
    fn short_or_foreign_data_is_handled() {
        let b = openssl_hello("pool.example.test", b"\x08http/1.1");
        // Cut before the extensions: a hello, but no name.
        let h = parse_client_hello(&b[..60]).unwrap();
        assert_eq!(h.sni, None);
        for junk in [&b"GET / HTTP/1.1\r\n"[..], &b"\x16\x03\x01"[..], &b""[..], &[0x16, 3, 1, 0, 5, 2, 0, 0, 1, 0][..]] {
            assert!(parse_client_hello(junk).map(|h| h.sni).unwrap_or(None).is_none());
        }
        // A name with characters no host name has is not reported.
        let mut bad = b.clone();
        if let Some(i) = bad.windows(4).position(|w| w == b"pool") {
            bad[i] = b'/';
        }
        assert_eq!(parse_client_hello(&bad).unwrap().sni, None);
    }
}
