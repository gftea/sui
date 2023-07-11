// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use move_binary_format::{errors::PartialVMResult, file_format::Bytecode};
use move_vm_types::{loaded_data::runtime_types::Type, values::Locals};

use crate::{
    interpreter::{FrameInterface, InstrRet, InterpreterInterface},
    loader::{Function, Resolver},
};

#[derive(Clone, Copy)]
pub enum Severity {
    Critical,
    NonCritical,
}

pub(crate) trait Plugin {
    fn get_severity(&self) -> Severity;

    fn pre_hook_entrypoint(
        &mut self,
        function: &Arc<Function>,
        ty_args: &[Type],
        resolver: &Resolver,
    ) -> PartialVMResult<()>;

    fn pre_hook_fn(
        &mut self,
        interpreter: &dyn InterpreterInterface,
        current_frame: &dyn FrameInterface,
        function: &Arc<Function>,
        ty_args: &[Type],
        resolver: &Resolver,
    ) -> PartialVMResult<()>;

    fn post_hook_fn(
        &mut self,
        // gas_meter: &mut impl GasMeter, TODO(wlmyng): GasMeter has a bunch of generic types that are incompatible with trait objects
        function: &Arc<Function>,
    ) -> ();

    fn pre_hook_instr(
        &mut self,
        interpreter: &dyn InterpreterInterface,
        // gas_meter: &mut impl GasMeter,
        function: &Arc<Function>,
        instruction: &Bytecode,
        locals: &Locals,
        ty_args: &[Type],
        resolver: &Resolver,
    ) -> PartialVMResult<()>;

    fn post_hook_instr(
        &mut self,
        interpreter: &dyn InterpreterInterface,
        // gas_meter: &mut impl GasMeter,
        function: &Arc<Function>,
        instruction: &Bytecode,
        ty_args: &[Type],
        resolver: &Resolver,
        r: &InstrRet,
    ) -> PartialVMResult<()>;
}
