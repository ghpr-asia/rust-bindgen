#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

impl PartialEq for AssumePartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.bar == other.bar
    }
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AssumePartialEq {
    pub bar: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_AssumePartialEq() {
    const UNINIT: ::std::mem::MaybeUninit<AssumePartialEq> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<AssumePartialEq>(),
        4usize,
        concat!("Size of: ", stringify!(AssumePartialEq))
    );
    assert_eq!(
        ::std::mem::align_of::<AssumePartialEq>(),
        4usize,
        concat!("Alignment of ", stringify!(AssumePartialEq))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bar) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AssumePartialEq),
            "::",
            stringify!(bar)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct UsesAssumePartialEq {
    pub foo: AssumePartialEq,
    pub baz: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_UsesAssumePartialEq() {
    const UNINIT: ::std::mem::MaybeUninit<UsesAssumePartialEq> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<UsesAssumePartialEq>(),
        8usize,
        concat!("Size of: ", stringify!(UsesAssumePartialEq))
    );
    assert_eq!(
        ::std::mem::align_of::<UsesAssumePartialEq>(),
        4usize,
        concat!("Alignment of ", stringify!(UsesAssumePartialEq))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).foo) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(UsesAssumePartialEq),
            "::",
            stringify!(foo)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).baz) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(UsesAssumePartialEq),
            "::",
            stringify!(baz)
        )
    );
}
