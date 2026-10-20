//! RFC 4648 base64 (standard alphabet, `=` padding) — enough for the snapshot wallpaper
//! payloads embedded in exported configuration files.

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let a = chunk[0] as u32;
        let b = *chunk.get(1).unwrap_or(&0) as u32;
        let c = *chunk.get(2).unwrap_or(&0) as u32;
        let n = (a << 16) | (b << 8) | c;
        out.push(ALPHABET[(n >> 18) as usize & 63] as char);
        out.push(ALPHABET[(n >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 {
            ALPHABET[(n >> 6) as usize & 63] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            ALPHABET[n as usize & 63] as char
        } else {
            '='
        });
    }
    out
}

/// Strict decode: rejects invalid characters, misplaced `=` padding and truncated groups.
/// ASCII whitespace between groups is tolerated (some exporters wrap base64 at 76 columns).
pub fn decode(text: &str) -> Option<Vec<u8>> {
    let mut table = [u8::MAX; 256];
    for (i, &c) in ALPHABET.iter().enumerate() {
        table[c as usize] = i as u8;
    }
    let bytes: Vec<u8> = text.bytes().filter(|b| !b.is_ascii_whitespace()).collect();
    if bytes.is_empty() {
        return Some(Vec::new());
    }
    if bytes.len() % 4 != 0 {
        return None;
    }
    let last = bytes.len() / 4 - 1;
    let mut out = Vec::with_capacity(last * 3 + 3);
    for (ci, chunk) in bytes.chunks(4).enumerate() {
        let pad = chunk.iter().filter(|&&b| b == b'=').count();
        // '=' may only pad the tail of the final group, one or two of them.
        if pad > 2 || (pad > 0 && (ci != last || !chunk[4 - pad..].iter().all(|&b| b == b'='))) {
            return None;
        }
        let mut n = 0u32;
        for &c in chunk {
            n <<= 6;
            if c == b'=' {
                continue;
            }
            let v = table[c as usize];
            if v == u8::MAX {
                return None;
            }
            n |= v as u32;
        }
        out.push((n >> 16) as u8);
        if pad < 2 {
            out.push((n >> 8) as u8);
        }
        if pad < 1 {
            out.push(n as u8);
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_all_lengths() {
        for len in 0..64 {
            let data: Vec<u8> = (0..len).map(|i| (i * 37 + len) as u8).collect();
            assert_eq!(decode(&encode(&data)).as_deref(), Some(data.as_slice()));
        }
    }

    #[test]
    fn known_vectors() {
        assert_eq!(encode(b"f"), "Zg==");
        assert_eq!(encode(b"fo"), "Zm8=");
        assert_eq!(encode(b"foo"), "Zm9v");
        assert_eq!(decode("Zm9v"), Some(b"foo".to_vec()));
        assert_eq!(decode("Zg=="), Some(b"f".to_vec()));
        assert_eq!(decode(" Zm9v\n"), Some(b"foo".to_vec()));
    }

    #[test]
    fn rejects_malformed() {
        for bad in ["Zg=", "Zg===", "====", "Z==g", "Zm 9", "!!9v"] {
            assert_eq!(decode(bad), None, "{bad:?}");
        }
    }
}
