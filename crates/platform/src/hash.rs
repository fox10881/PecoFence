//! SHA-256 via Windows CNG (`bcrypt.dll`) — no hashing dependency needed. Used to verify
//! the downloaded update setup against the release's `.sha256` file before it runs.

windows_core::link!("bcrypt.dll" "system" fn BCryptOpenAlgorithmProvider(phalgorithm: *mut isize, pszalgid: windows_core::PCWSTR, pszimplementation: windows_core::PCWSTR, dwflags: u32) -> i32);
windows_core::link!("bcrypt.dll" "system" fn BCryptCreateHash(halgorithm: isize, phhash: *mut isize, pbhashobject: *mut u8, cbhashobject: u32, pbsecret: *const u8, cbsecret: u32, dwflags: u32) -> i32);
windows_core::link!("bcrypt.dll" "system" fn BCryptHashData(hhash: isize, pbinput: *const u8, cbinput: u32, dwflags: u32) -> i32);
windows_core::link!("bcrypt.dll" "system" fn BCryptFinishHash(hhash: isize, pboutput: *mut u8, cboutput: u32, dwflags: u32) -> i32);
windows_core::link!("bcrypt.dll" "system" fn BCryptDestroyHash(hhash: isize) -> i32);
windows_core::link!("bcrypt.dll" "system" fn BCryptCloseAlgorithmProvider(halgorithm: isize, dwflags: u32) -> i32);

const BCRYPT_SHA256_ALGORITHM: &[u16] = &[
    b'S' as u16,
    b'H' as u16,
    b'A' as u16,
    b'2' as u16,
    b'5' as u16,
    b'6' as u16,
    0,
];

fn check(status: i32, what: &str) -> Result<(), String> {
    if status == 0 {
        Ok(())
    } else {
        Err(format!("{what} failed: NTSTATUS {status:#010x}"))
    }
}

/// Lowercase hex SHA-256 of `data`.
pub fn sha256_hex(data: &[u8]) -> Result<String, String> {
    let mut provider: isize = 0;
    check(
        unsafe {
            BCryptOpenAlgorithmProvider(
                &mut provider,
                windows_core::PCWSTR(BCRYPT_SHA256_ALGORITHM.as_ptr()),
                windows_core::PCWSTR::null(),
                0,
            )
        },
        "BCryptOpenAlgorithmProvider",
    )?;
    let result = hash_with_provider(provider, data);
    unsafe {
        let _ = BCryptCloseAlgorithmProvider(provider, 0);
    }
    result
}

fn hash_with_provider(provider: isize, data: &[u8]) -> Result<String, String> {
    let mut hash: isize = 0;
    check(
        unsafe {
            BCryptCreateHash(
                provider,
                &mut hash,
                core::ptr::null_mut(),
                0,
                core::ptr::null(),
                0,
                0,
            )
        },
        "BCryptCreateHash",
    )?;
    let result: Result<[u8; 32], String> = (|| {
        // CNG rejects a null input pointer even for empty input.
        let (ptr, len) = if data.is_empty() {
            ([0u8; 1].as_ptr(), 0)
        } else {
            (data.as_ptr(), data.len() as u32)
        };
        check(
            unsafe { BCryptHashData(hash, ptr, len, 0) },
            "BCryptHashData",
        )?;
        let mut digest = [0u8; 32];
        check(
            unsafe { BCryptFinishHash(hash, digest.as_mut_ptr(), 32, 0) },
            "BCryptFinishHash",
        )?;
        Ok(digest)
    })();
    unsafe {
        let _ = BCryptDestroyHash(hash);
    }
    let digest = result?;
    Ok(digest.iter().map(|b| format!("{b:02x}")).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_matches_known_vectors() {
        assert_eq!(
            sha256_hex(b"").unwrap(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            sha256_hex(b"abc").unwrap(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_eq!(
            sha256_hex(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq").unwrap(),
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
        );
    }
}
