// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! New recursive solver modeled on Chalk's recursive solver. Most of
//! the guts are broken up into modules; see the comments in those modules.

#![deny(warnings)]

#![feature(crate_visibility_modifier)]
#![feature(match_default_bindings)]
#![feature(underscore_lifetimes)]
#![cfg_attr(stage0, feature(use_nested_groups))]

#[macro_use]
extern crate log;
extern crate rustc;
extern crate rustc_data_structures;
extern crate syntax;
extern crate syntax_pos;

mod dropck_outlives;
mod normalize_projection_ty;
mod util;

use rustc::ty::maps::Providers;

pub fn provide(p: &mut Providers) {
    *p = Providers {
        dropck_outlives: dropck_outlives::dropck_outlives,
        adt_dtorck_constraint: dropck_outlives::adt_dtorck_constraint,
        normalize_projection_ty: normalize_projection_ty::normalize_projection_ty,
        ..*p
    };
}
