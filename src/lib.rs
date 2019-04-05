#![no_std]
#![feature(alloc)]
#![feature(concat_idents)]
#![feature(slice_concat_ext)]
#![feature(stdsimd)]
#![feature(try_reserve)]

extern crate alloc;

// Macro re-exports
pub use core::{assert_eq, assert_ne, debug_assert, debug_assert_eq, debug_assert_ne};
pub use core::{unreachable, unimplemented, write, writeln, r#try};
pub use core::*;

// Data structure re-exports
pub use crate::alloc::{vec, format};
pub use crate::alloc::*;

pub use std_detect::is_x86_feature_detected;

pub mod collections {
    pub use crate::alloc::collections::*;

    pub use crate::alloc::collections::BTreeMap as HashMap;
    pub use crate::alloc::collections::BTreeSet as HashSet;

    pub mod hash_map {
        pub use crate::alloc::collections::BTreeMap as HashMap;
        pub use crate::alloc::collections::btree_map::*;
    }

    pub mod hash_set {
        pub use crate::alloc::collections::BTreeSet as HashSet;
        pub use crate::alloc::collections::btree_set::*;
    }
}

pub mod prelude {
    // Re-exported from libcore
    pub use core::prelude::*;

    // Re-export from liballoc
    pub use crate::alloc::prelude::*;
}

pub use std_detect::detect;

