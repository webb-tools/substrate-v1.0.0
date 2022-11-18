// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate archive specification API.
//!
//! The *archive* functions inspect the history of the chain.
//!
//! They can be used to access recent information as well,
//! but JSON-RPC clients should keep in mind that the chainHead
//! functions could be more appropriate.
//!
//! # Note
//!
//! Methods are prefixed by `archive`.

pub mod api;
pub mod event;

pub use api::ArchiveApiServer;
pub use event::{ArchiveEvent, ArchiveResult, ErrorEvent, NetworkConfig};