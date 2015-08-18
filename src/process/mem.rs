use core::mem;
use core::marker::PhantomData;
use core::ops::{Deref,DerefMut};
use core::ptr::Unique;
use core::raw::Slice;
use process::Process;

pub struct Private;
pub struct Shared;

pub struct AppPtr<L, T> {
    ptr: Unique<T>,
    process: *mut (),
    _phantom: PhantomData<L>
}

impl<L, T> AppPtr<L, T> {
    pub unsafe fn new(ptr: *mut T, process: *mut ()) -> AppPtr<L, T> {
        AppPtr {
            ptr: Unique::new(ptr),
            process: process,
            _phantom: PhantomData
        }
    }
}

impl<L, T> Deref for AppPtr<L, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe {
            self.ptr.get()
        }
    }
}

impl<L, T> DerefMut for AppPtr<L, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe {
            self.ptr.get_mut()
        }
    }
}

impl<L, T> Drop for AppPtr<L, T> {
    fn drop(&mut self) {
        unsafe {
            let process : &mut Process = mem::transmute(self.process);
            process.free(self.ptr.get_mut());
        }
    }
}

pub struct AppSlice<L, T> {
    ptr: AppPtr<L, T>,
    len: usize
}

impl<L, T> AppSlice<L, T> {
    pub unsafe fn new(ptr: *mut T, len: usize, process_ptr: *mut ())
            -> AppSlice<L, T> {
        AppSlice {
            ptr: AppPtr::new(ptr, process_ptr),
            len: len
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<L, T> AsRef<[T]> for AppSlice<L, T> {
    fn as_ref(&self) -> &[T] {
        unsafe {
            mem::transmute(Slice{
                data: self.ptr.ptr.get(),
                len: self.len
            })
        }
    }
}

impl<L, T> AsMut<[T]> for AppSlice<L, T> {
    fn as_mut(&mut self) -> &mut [T] {
        unsafe {
            mem::transmute(Slice{
                data: self.ptr.ptr.get(),
                len: self.len
            })
        }
    }
}

