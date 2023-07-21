/*
 * Copyright 2023 Oxide Computer Company
 */

#![allow(non_camel_case_types)]

use std::{
    marker::{PhantomData, PhantomPinned},
    os::raw::{c_char, c_int, c_uchar, c_uint, c_void},
};

macro_rules! opaque_handle {
    ($type_name:ident) => {
        #[repr(C)]
        pub struct $type_name {
            _data: [u8; 0],
            /*
             * See https://doc.rust-lang.org/nomicon/ffi.html; this marker
             * guarantees our type does not implement `Send`, `Sync`, or
             * `Unpin`.
             */
            _marker: PhantomData<(*mut u8, PhantomPinned)>,
        }
        impl Copy for $type_name {}
        impl Clone for $type_name {
            fn clone(&self) -> $type_name {
                *self
            }
        }
    };
}

opaque_handle!(di_node_t);
opaque_handle!(di_prop_t);
opaque_handle!(di_minor_t);
opaque_handle!(di_devlink_handle_t);
opaque_handle!(di_devlink_t);

pub const DI_NODE_NIL: *mut di_node_t = std::ptr::null_mut();
pub const DI_PROP_NIL: *mut di_prop_t = std::ptr::null_mut();
pub const DI_MINOR_NIL: *mut di_minor_t = std::ptr::null_mut();
pub const DI_LINK_NIL: *mut di_devlink_handle_t = std::ptr::null_mut();

pub const DIIOC: c_uint = 0xDF << 8;
pub const DINFOSUBTREE: c_uint = DIIOC | 0x01; /* include subtree */
pub const DINFOMINOR: c_uint = DIIOC | 0x02; /* include minor data */
pub const DINFOPROP: c_uint = DIIOC | 0x04; /* include properties */
pub const DINFOPATH: c_uint = DIIOC | 0x08; /* include multipath node data */
pub const DINFOLYR: c_uint = DIIOC | 0x40; /* include device layering data */
pub const DINFOHP: c_uint = DIIOC | 0x400000; /* include hotplug info (?) */

pub const DINFOCPYONE: c_uint = DIIOC; /* just a single node */
pub const DINFOCPYALL: c_uint = DINFOSUBTREE | DINFOPROP | DINFOMINOR;

/*
 * These flags are Private:
 */
#[cfg(feature = "private")]
pub const DINFOPRIVDATA: c_uint = DIIOC | 0x10; /* include private data */
#[cfg(feature = "private")]
pub const DINFOFORCE: c_uint = DIIOC | 0x20; /* force load all drivers */
#[cfg(feature = "private")]
pub const DINFOCACHE: c_uint = DIIOC | 0x100000; /* use cached data  */
#[cfg(feature = "private")]
pub const DINFOCLEANUP: c_uint = DIIOC | 0x200000; /* cleanup /etc files */

pub const DI_MAKE_LINK: c_uint = 0x01;

pub const DI_PRIMARY_LINK: c_uint = 0x01;
pub const DI_SECONDARY_LINK: c_uint = 0x02;
pub const DI_LINK_TYPES: c_uint = 0x03;

pub const DI_WALK_CONTINUE: c_int = 0;
pub const DI_WALK_PRUNESIB: c_int = -1;
pub const DI_WALK_PRUNECHILD: c_int = -2;
pub const DI_WALK_TERMINATE: c_int = -3;

pub const DI_PROP_TYPE_BOOLEAN: c_int = 0;
pub const DI_PROP_TYPE_INT: c_int = 1;
pub const DI_PROP_TYPE_STRING: c_int = 2;
pub const DI_PROP_TYPE_BYTE: c_int = 3;
pub const DI_PROP_TYPE_UNKNOWN: c_int = 4;
pub const DI_PROP_TYPE_UNDEF_IT: c_int = 5;
pub const DI_PROP_TYPE_INT64: c_int = 6;

#[link(name = "devinfo")]
extern "C" {
    pub fn di_init(phys_path: *const c_char, flag: c_uint) -> *mut di_node_t;
    pub fn di_fini(root: *mut di_node_t);

    pub fn di_drv_first_node(
        drv_name: *const c_char,
        root: *mut di_node_t,
    ) -> *mut di_node_t;
    pub fn di_drv_next_node(node: *mut di_node_t) -> *mut di_node_t;

    pub fn di_parent_node(node: *mut di_node_t) -> *mut di_node_t;
    pub fn di_sibling_node(node: *mut di_node_t) -> *mut di_node_t;
    pub fn di_child_node(node: *mut di_node_t) -> *mut di_node_t;

    pub fn di_node_name(node: *mut di_node_t) -> *const c_char;
    pub fn di_driver_name(node: *mut di_node_t) -> *const c_char;
    pub fn di_instance(node: *mut di_node_t) -> c_int;

    pub fn di_prop_next(
        node: *mut di_node_t,
        prop: *mut di_prop_t,
    ) -> *mut di_prop_t;

    pub fn di_prop_name(prop: *mut di_prop_t) -> *const c_char;
    pub fn di_prop_type(prop: *mut di_prop_t) -> c_int;

    pub fn di_prop_strings(
        prop: *mut di_prop_t,
        data: *const *const c_char,
    ) -> c_int;
    pub fn di_prop_bytes(
        prop: *mut di_prop_t,
        data: *const *const c_uchar,
    ) -> c_int;
    pub fn di_prop_ints(
        prop: *mut di_prop_t,
        data: *const *const c_int,
    ) -> c_int;
    pub fn di_prop_int64(
        prop: *mut di_prop_t,
        data: *const *const i64,
    ) -> c_int;

    pub fn di_devfs_path(node: *mut di_node_t) -> *mut c_char;
    pub fn di_devfs_minor_path(minor: *mut di_minor_t) -> *mut c_char;
    pub fn di_devfs_path_free(path_buf: *mut c_char);

    pub fn di_minor_next(
        node: *mut di_node_t,
        minor: *mut di_minor_t,
    ) -> *mut di_minor_t;
    pub fn di_minor_name(minor: *mut di_minor_t) -> *const c_char;
    pub fn di_minor_nodetype(minor: *mut di_minor_t) -> *const c_char;
    pub fn di_minor_spectype(minor: *mut di_minor_t) -> c_int;

    pub fn di_devlink_init(
        name: *const c_char,
        flags: c_uint,
    ) -> *mut di_devlink_handle_t;
    pub fn di_devlink_fini(hdlp: *mut *mut di_devlink_handle_t) -> c_int;

    pub fn di_devlink_walk(
        hdl: *mut di_devlink_handle_t,
        re: *const c_char,
        mpath: *const c_char,
        flags: c_uint,
        arg: *mut c_void,
        devlink_callback: unsafe extern "C" fn(
            *const di_devlink_t,
            *mut c_void,
        ) -> c_int,
    ) -> c_int;

    pub fn di_devlink_path(devlink: *const di_devlink_t) -> *const c_char;
    pub fn di_devlink_content(devlink: *const di_devlink_t) -> *const c_char;
    pub fn di_devlink_type(devlink: *const di_devlink_t) -> c_int;
}
