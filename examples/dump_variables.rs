extern crate spirv_utils;

use std::env;

use spirv_utils::{desc, instruction};

fn main() {
    let mut args = env::args_os();
    args.next();

    let filename = args.next();

    let module = if let Some(filename) = filename {
        spirv_utils::RawModule::load_module(filename).unwrap()
    } else {
        spirv_utils::RawModule::load_module("examples/vert.spv").unwrap()
    };

    for inst in module.instructions() {
        if let instruction::Instruction::Variable { result_id, storage_class, .. } = *inst {
            if let Some(name) = get_name(&module, result_id.into()) {
                println!("Variable {: <15} {:?}", name, storage_class);
            }
        }
    }
}

fn get_name<'a>(module: &'a spirv_utils::RawModule, id: desc::Id) -> Option<&'a str> {
    module.uses(id).filter_map(|inst| {
        if let instruction::Instruction::Name { ref name, .. } = *inst {
            Some(&name[..])
        } else {
            None
        }
    }).next()
}
