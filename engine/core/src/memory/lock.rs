use crate::errors::internal::ResourceError;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum LockStrategy {
    None,
    BestEffort,
    Required,
}

#[cfg(unix)]
fn os_mlock(buf: &mut [u8]) -> Result<(), i32> {
    if buf.is_empty() {
        return Ok(());
    }

    use libc::{c_void, mlock};

    let ptr = buf.as_mut_ptr() as *const c_void;
    let len = buf.len();
    let result = unsafe { mlock(ptr, len) };

    if result != 0 {
        let err = unsafe { *libc::__errno_location() };
        Err(err)
    } else {
        Ok(())
    }
}

#[cfg(windows)]
fn os_mlock(buf: &mut [u8]) -> Result<(), u32> {
    if buf.is_empty() {
        Ok(())
    }

    use windows_sys::Win32::{Foundation::GetLastError, System::Memory::VirtualLock};

    let ptr = buf.as_mut_ptr();
    let len = buf.len();
    let result = unsafe { VirtualLock(ptr as *mut _, len) };

    if result == 0 {
        let err = unsafe { GetLastError() };
        Err(err)
    } else {
        Ok(())
    }
}

pub(crate) fn lock_memory(buf: &mut [u8], strategy: LockStrategy) -> Result<(), ResourceError> {
    use LockStrategy::*;

    match strategy {
        None => Ok(()),
        BestEffort => {
            if let Err(e) = os_mlock(buf) {
                log::warn!("Best-effor lock failed: {:?}", e);
            }
            Ok(())
        }
        Required => os_mlock(buf).map_err(|_| ResourceError::SystemFailure {
            context: "Memory lock failed",
        }),
    }
}

#[cfg(unix)]
pub(crate) fn unlock_memory(buf: &mut [u8]) {
    use libc::{c_void, munlock};
    unsafe {
        munlock(buf.as_ptr() as *const c_void, buf.len());
    };
}

#[cfg(windows)]
pub(crate) fn unlock_memory(buf: &mut [u8]) {
    use windows_sys::Win32::System::Memory::VirtualUnlock;
    unsafe {
        VirtualUnlock(buf.as_ptr() as *const _, buf.len());
    };
}
