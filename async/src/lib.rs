//! A capability-based API modeled after `std`.
//!
//! This corresponds to [`std`].
//!
//! Capability-based APIs represent access to external resources as
//! objects which can be passed around between different parts of a
//! program.
//!
//! Two notable features are the [`Dir`] and [`Catalog`] types:
//!  - `Dir` represents an open directory in a filesystem. Instead of
//!    opening files by absolute paths or paths relative to the current
//!    working directory, files are opened via paths relative to a
//!    `Dir`. The concepts of a process-wide "current working directory"
//!    and a single global filesystem namespace are de-emphasized.
//!  - `Catalog` represents a set of network addresses. Instead of
//!    allowing applications to request access to any address and then
//!    applying process-wide filtering rules, filtering rules are
//!    built into catalogs which may be passed through the program.
//!
//! On WASI, use of this library closely reflects the underlying system
//! API, so it avoids compatibility layers.
//!
//! [`std`]: https://doc.rust-lang.org/std/index.html
//! [`Dir`]: struct.Dir.html
//! [`Catalog`]: struct.Catalog.html

#![allow(dead_code, unused_variables)] // TODO: When more things are implemented, remove these.
#![deny(missing_docs)]

mod sys;

pub mod fs;
pub mod net;
pub mod os;