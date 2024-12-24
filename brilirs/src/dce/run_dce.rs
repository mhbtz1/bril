use std::collections::{HashSet, HashMap};
use bril_rs::{Program, Function, Code, Instruction};


pub fn full_dce(program: Program) -> Program {
    let mut new_program = program.clone();
    new_program.functions.clear();
    for func in program.functions {
        let upd_func = run_dce_func(&func);
        new_program.functions.push(upd_func);
    }
    new_program
}

pub fn run_dce_func(block: &Function) -> Function {
    // currently only does one pass of DCE, so not fully done

    let mut new_func = Function {
        name: block.name.clone(),
        args: block.args.clone(),
        instrs: Vec::new(),
        pos: block.pos.clone(),
        return_type: block.return_type.clone()
    };
    
    let mut used = HashSet::new();
    let mut map: HashMap<String, usize> = HashMap::new();
    let tup_iters = block.instrs.clone().into_iter().enumerate().collect::<Vec<(usize, Code)>>();   
    for (line, instr) in &tup_iters{
        if let Code::Instruction(Instruction::Value { args, dest, funcs: _, labels: _,  op: _, pos: _, op_type: _}) = instr {
            map.insert(dest.clone().to_string(), *line);
        }
    }

    for (line, instr) in &tup_iters {
        match instr {
            Code::Instruction(Instruction::Value { args, dest, funcs: _, labels: _,  op: _, pos: _, op_type: _}) => {
                used.insert(dest);
            }
            _ => ()
        }
    }
        
    for (line, instr) in &tup_iters {
        match instr {
            Code::Instruction(Instruction::Value { args, dest, funcs: _, labels: _,  op: _, pos: _, op_type: _}) => {
                if let Some(&line) = map.get(dest) {
                    if used.contains(&dest) {
                        new_func.instrs.push(instr.clone());
                    }
                }

            }
            _ => ()
        }
    }
    new_func
}


