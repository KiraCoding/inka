use core::fmt::{Debug, Formatter, Pointer, Result};
use core::ptr::NonNull;
use windows::core::PCWSTR;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

/// Thread-safe `Base` pointer.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Base {
    ptr: NonNull<u8>,
}

impl Base {
    pub fn program() -> Self {
        let raw_base = unsafe { GetModuleHandleW(PCWSTR::null()).unwrap_unchecked().0.cast() };

        // SAFETY: todo!()
        unsafe { Self::new_unchecked(raw_base) }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.ptr.as_ptr()
    }

    #[inline]
    pub unsafe fn add(&self, count: usize) -> NonNull<u8> {
        unsafe { self.ptr.add(count) }
    }

    pub(crate) unsafe fn new_unchecked(ptr: *mut u8) -> Self {
        // SAFETY: todo!()
        let ptr = unsafe { NonNull::new_unchecked(ptr) };

        Self { ptr }
    }
}

impl Debug for Base {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Pointer::fmt(&self.ptr, f)
    }
}

impl Pointer for Base {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Pointer::fmt(&self.ptr, f)
    }
}

unsafe impl Sync for Base {}
unsafe impl Send for Base {}
