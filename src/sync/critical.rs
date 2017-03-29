/*
* Copyright (C) 2017 AltOS-Rust Team
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program. If not, see <http://www.gnu.org/licenses/>.
*/

use core::ops::Drop;
use arch;

/// A marker for a critical region of code.
///
/// This struct marks the beginning of a critical section, returning a `CriticalSectionGuard` that
/// lives for the scope of the function or until it is explicitly dropped. Marking a section as
/// critical means that, while executing in that region, a thread can not be preempted.
///
/// # Examples
///
/// ```rust,no_run
/// use altos_core::sync::CriticalSection;
///
/// let critical_guard = CriticalSection::begin();
///
/// // Do some critical work here...
/// // we can not be preempted in this region
///
/// drop(critical_guard); // Could also just let it drop out of scope
/// ```
pub struct CriticalSection;

impl CriticalSection {
    /// Marks the beginning of a critical section, returning a `CriticalSectionGuard` that will
    /// end the critical section when it falls out of scope.
    pub fn begin() -> CriticalSectionGuard {
        CriticalSectionGuard(arch::begin_critical())
    }
}

/// Tracks the lifetime of a critical section.
///
/// Can only be generated by the `begin()` function on `CriticalSection`. When this falls out of
/// scope, it will automatically re-enable preemption.
#[must_use]
pub struct CriticalSectionGuard(usize);

impl Drop for CriticalSectionGuard {
    fn drop(&mut self) {
        arch::end_critical(self.0);
    }
}