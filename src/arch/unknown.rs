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

//! This module is used to provide stubs for the architecture layer.

use volatile::Volatile;
use task::args::Args;
use alloc::boxed::Box;

extern "Rust" {
    // Give up remaining CPU time to the scheduler, usually done through some inerrupt call
    fn __yield_cpu();

    // Initialize the stack with the given arguments, `stack_ptr` is the initial stack pointer,
    // `code_ptr` is a pointer to the function to run, `args_ptr` is a pointer to the arguments
    // that should be placed in the correct register for the architecture's calling convention.
    //
    // Must return the updated stack pointer
    fn __initialize_stack(stack_ptr: usize, code_ptr: usize, args_ptr: usize) -> usize;

    // Start the first task, assuming that `CURRENT_TASK` has been selected by the scheduler and
    // now just needs its context loaded into the CPU
    fn __start_first_task();

    // Check if the code is running in kernel mode, return `true` if it is. This is generally just
    // a convenience method, and can be stubbed out to return only `true` if needed.
    fn __in_kernel_mode() -> bool;

    // Begin a critical section, disabling interrupts.
    //
    // Return a value that will be used in a future `end_critical` call, which may be useful for
    // restoring some state. If it is unneccessary, returning `0` is allowed.
    fn __begin_critical() -> usize;

    // End a critical section, re-enabling interrupts.
    //
    // `mask` is the value returned from the matching `begin_critical` call, use it to restore some
    // state of the processor or ignore the value is unneeded.
    fn __end_critical(mask: usize);

    // Initiate a system call with 0 arguments, return the result of that system call as a pointer
    // width integer.
    fn __syscall0(call: u32) -> usize;

    // Initiate a system call with 1 argument, return the result of that system call as a pointer
    // width integer.
    fn __syscall1(call: u32, arg1: usize) -> usize;

    // Initiate a system call with 2 arguments, return the result of that system call as a pointer
    // width integer.
    fn __syscall2(call: u32, arg1: usize, arg2: usize) -> usize;
}

pub fn yield_cpu() {
    unsafe { __yield_cpu() };
}

pub fn initialize_stack(stack_ptr: Volatile<usize>, code: fn(&mut Args), args: &Box<Args>) -> usize {
    unsafe {
        __initialize_stack(stack_ptr.as_ptr() as usize, code as usize, &**args as *const _ as usize)
    }
}

pub fn start_first_task() {
    unsafe { __start_first_task() };
}

pub fn in_kernel_mode() -> bool {
    unsafe { __in_kernel_mode() }
}

pub fn begin_critical() -> usize {
    unsafe { __begin_critical() }
}

pub fn end_critical(mask: usize) {
    unsafe { __end_critical(mask) };
}

pub fn syscall0(call: u32) -> usize {
    unsafe { __syscall0(call) }
}

pub fn syscall1(call: u32, arg1: usize) -> usize {
    unsafe { __syscall1(call, arg1) }
}

pub fn syscall2(call: u32, arg1: usize, arg2: usize) -> usize {
    unsafe { __syscall2(call, arg1, arg2) }
}
