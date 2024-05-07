use std::{
    io,
    ops::{Deref, DerefMut},
    slice::from_raw_parts_mut,
};

use log::error;

use crate::types::{Pmtu, PAGE_SIZE};

/// Get the length of the first packet.
///
/// A buffer will be divided into multiple packets if any slice is crossed the boundary of pmtu
/// For example, if pmtu = 256 and va = 254, then the first packet can be at most 2 bytes.
/// If pmtu = 256 and va = 256, then the first packet can be at most 256 bytes.
#[inline]
#[allow(clippy::arithmetic_side_effects)]
pub(crate) fn get_first_packet_max_length(va: u64, pmtu: u32) -> u32 {
    // The offset is smaller than pmtu, which is smaller than 4096 currently.
    #[allow(clippy::cast_possible_truncation)]
    let offset = (va.wrapping_rem(u64::from(pmtu))) as u32;

    pmtu - offset
}

#[allow(clippy::arithmetic_side_effects)] // total_len must be greater or equal than first_pkt_len
pub(crate) fn calculate_packet_cnt(pmtu: Pmtu, raddr: u64, total_len: u32) -> u32 {
    let first_pkt_max_len = get_first_packet_max_length(raddr, u32::from(&pmtu));
    let first_pkt_len = total_len.min(first_pkt_max_len);

    1 + (total_len - first_pkt_len).div_ceil(u32::from(&pmtu))
}

#[allow(clippy::arithmetic_side_effects)]
pub(crate) fn u8_slice_to_u64(slice: &[u8]) -> u64 {
    // this operation convert a [u8;8] to a u64. So it's safe to left shift
    slice.iter().fold(0, |a, b| (a << 8_i32) + u64::from(*b))
}

#[allow(clippy::arithmetic_side_effects)]
pub(crate) fn align_up<const PAGE: usize>(addr: usize) -> usize {
    (((addr) + ((PAGE) - 1)) / PAGE) * PAGE
}

/// A struct to manage hugepage memory
#[derive(Debug)]
pub struct HugePage {
    size: usize,
    addr: usize,
}

impl HugePage {
    /// size of the huge page
    pub const HUGE_PAGE_SIZE: usize = PAGE_SIZE;

    /// # Errors
    ///
    pub fn new(size: usize) -> io::Result<Self> {
        let size = align_up::<{ Self::HUGE_PAGE_SIZE }>(size);
        let buffer = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                size,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_ANONYMOUS | libc::MAP_PRIVATE | libc::MAP_HUGETLB,
                -1,
                0,
            )
        };
        if buffer == libc::MAP_FAILED {
            return Err(io::Error::last_os_error());
        }

        let ret = unsafe { libc::mlock(buffer, size) };
        if ret != 0_i32 {
            return Err(io::Error::last_os_error());
        }
        Ok(HugePage {
            size,
            addr: buffer as usize,
        })
    }

    /// get raw pointer of huge page buffer
    #[must_use]
    pub fn as_ptr(&self) -> *const u8 {
        self.addr as *const u8
    }

    /// get size of the huge page buffer
    #[must_use]
    pub fn size(&self) -> usize {
        self.size
    }

    pub(crate) fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { from_raw_parts_mut(self.addr as *mut u8, self.size) }
    }

    pub(crate) fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.addr as *const u8, self.size) }
    }
}

impl Deref for HugePage {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl DerefMut for HugePage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl Drop for HugePage {
    fn drop(&mut self) {
        let result = unsafe { libc::munmap(self.addr as *mut libc::c_void, self.size) };
        if result != 0_i32 {
            error!("drop huge page failed: {result}");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::Pmtu;

    use super::align_up;

    #[test]
    fn test_calculate_packet_cnt() {
        let raddr = 0;
        let total_len = 4096;
        let packet_cnt = super::calculate_packet_cnt(Pmtu::Mtu1024, raddr, total_len);
        assert_eq!(packet_cnt, 4);

        for raddr in 1..1023 {
            let packet_cnt = super::calculate_packet_cnt(Pmtu::Mtu1024, raddr, total_len);
            assert_eq!(packet_cnt, 5);
        }
    }

    #[test]
    fn align_up_test() {
        let a = align_up::<{ 1024 * 1024 * 2 }>(1024);
        let b = align_up::<{ 1024 * 1024 * 2 }>(1024 * 1024 * 2 + 1);

        assert_eq!(a, 1024 * 1024 * 2);
        assert_eq!(b, 1024 * 1024 * 4);
    }
}
