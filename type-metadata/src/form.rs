// Copyright 2019 Parity Technologies (UK) Ltd.
// This file is part of type-metadata.
//
// type-metadata is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// type-metadata is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with type-metadata.  If not, see <http://www.gnu.org/licenses/>.

use crate::{
	interner::{UntrackedStringSymbol, UntrackedTypeIdSymbol},
	TypeId,
};
use serde::Serialize;

/// Trait to control the internal structures of type identifiers and definitions.
///
/// This allows for type-level separation between free forms that can be instantiated
/// out of the flux and compact forms that require some sort of interning data structures.
pub trait Form {
	/// The string type.
	type String: Serialize + PartialEq + Eq + PartialOrd + Ord + Clone + core::fmt::Debug;
	/// The type identifier type.
	type TypeId: Serialize + PartialEq + Eq + PartialOrd + Ord + Clone + core::fmt::Debug;
}

/// Free form is not depending on any interner data structure
/// to compact symbols and type identifiers. This means it requires
/// more space but can also be created in flux.
///
/// # Note
///
/// The free form is the default for all type identifiers and definitions.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Debug)]
pub enum FreeForm {}

/// Compact form is depending on interning data structures
/// and generally requires less space. However, due to the dependency
/// on some interning data structures it cannot be created without them.
///
/// # Note
///
/// The compact form is only in use by the type registry which itself
/// owns the interning data structures.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Debug)]
pub enum CompactForm {}

impl Form for FreeForm {
	type String = &'static str;
	type TypeId = TypeId;
}

impl Form for CompactForm {
	/// Untracked here means that we don't track lifetimes.
	/// We use the untracked form to avoid lifetime issues within the later-to-be
	/// self-referential registry data structure.
	type String = UntrackedStringSymbol;
	/// See above why we use the untracked symbol type.
	type TypeId = UntrackedTypeIdSymbol;
}
