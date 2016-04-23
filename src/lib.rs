// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A low-dimensional linear algebra library, targeted at computer graphics.
//!
//! # Trait overview
//!
//! In order to make a clean, composable API, we divide operations into traits
//! that are roughly based on mathematical properties. The main ones that we
//! concern ourselves with are listed below:
//!
//! - `VectorSpace`: Specifies the main operators for vectors, quaternions, and
//!    matrices.
//! - `InnerSpace`: For types that have a dot (or inner) product - ie. vectors or
//!    quaternions. This also allows for the definition of operations that are
//!    based on the dot product, like finding the magnitude or normalizing.
//! - `EuclideanSpace`: Points in euclidean space, with an associated space of
//!   displacement vectors.
//! - `Matrix`: Common operations for matrices of arbitrary dimensions.
//! - `SquareMatrix`: A special trait for matrices where the number of columns
//!   equal the number of rows.
//!
//! Other traits are included for practical convenience, for example:
//!
//! - `Array`: For contiguous, indexable arrays of elements, specifically
//!   vectors.
//! - `ElementWise`: For element-wise addition, subtraction, multiplication,
//!   division, and remainder operations.
//!
//! # The prelude
//!
//! Importing each trait individually can become a chore, so we provide a
//! `prelude` module to allow you to import the main trait all at once. For
//! example:
//!
//! ```rust
//! use cgmath::prelude::*;
//! ```

extern crate num_traits;
extern crate rustc_serialize;
extern crate rand;

// Re-exports

pub use approx::*;
pub use num::*;
pub use structure::*;

pub use matrix::{Matrix2, Matrix3, Matrix4};
pub use quaternion::Quaternion;
pub use vector::{Vector2, Vector3, Vector4, dot, vec2, vec3, vec4};

pub use angle::{Deg, Rad};
pub use euler::Euler;
pub use point::{Point2, Point3};
pub use rotation::*;
pub use transform::*;

pub use projection::*;

pub use num_traits::{One, Zero};

// Modules

pub mod conv;
pub mod prelude;

mod macros;

mod approx;
mod num;
mod structure;

mod matrix;
mod quaternion;
mod vector;

mod angle;
mod euler;
mod point;
mod rotation;
mod transform;

mod projection;
