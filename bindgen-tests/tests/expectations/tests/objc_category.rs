#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]
#![cfg(target_os = "macos")]

use objc::{self, class, msg_send, sel, sel_impl};
#[allow(non_camel_case_types)]
pub type id = *mut objc::runtime::Object;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Foo(pub id);
impl std::ops::Deref for Foo {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for Foo {}
impl Foo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(class!(Foo), alloc) })
    }
}
impl IFoo for Foo {}
pub trait IFoo: Sized + std::ops::Deref {
    unsafe fn method(&self)
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        unsafe { msg_send!(*self, method) }
    }
}
impl Foo_BarCategory for Foo {}
pub trait Foo_BarCategory: Sized + std::ops::Deref {
    unsafe fn categoryMethod(&self)
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        unsafe { msg_send!(*self, categoryMethod) }
    }
}
