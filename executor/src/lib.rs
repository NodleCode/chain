/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2020  Nodle International
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
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

//! A `CodeExecutor` specialization which uses natively compiled runtime when the wasm to be
//! executed is equivalent to the natively compiled code.

// use sc_executor::native_executor_instance;
// pub use sc_executor::NativeExecutor;
pub use sc_executor::NativeElseWasmExecutor;

// Declare an instance of the native executor named `Executor`. Include the wasm binary as the
// equivalent wasm code.
// native_executor_instance!(
//     pub Executor,
//     nodle_chain_runtime::api::dispatch,
//     nodle_chain_runtime::native_version,
//     frame_benchmarking::benchmarking::HostFunctions,
// );

// Declare an instance of the native executor named `Executor`. Include the wasm binary as the
// equivalent wasm code.
pub struct Executor;

impl sc_executor::NativeExecutionDispatch for Executor {
    type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;

    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        nodle_chain_runtime::api::dispatch(method, data)
    }

    fn native_version() -> sc_executor::NativeVersion {
        nodle_chain_runtime::native_version()
    }
}
