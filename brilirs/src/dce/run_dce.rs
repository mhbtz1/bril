use std::collections::{HashSet, HashMap};
use bril_rs::{Program, Function, Code, Instruction};
use crate::{BBProgram, BBFunction};

pub fn full_dce(program: BBProgram) -> BBProgram {
    let mut new_program = program.clone();
    new_program.func_index.clear();
    for func in program.func_index{
        let upd_func = run_dce_func(&func);
        new_program.func_index.push(upd_func);
    }
    new_program
}

pub fn run_dce_func(block: &BBFunction) -> BBFunction {
    // currently only does one pass of DCE, so not fully done

    let mut new_func = block.clone();
    block.blocks.into_iter().map(|x| x.instrs.clear());
    
    let mut used = HashSet::new();
    let mut map: HashMap<String, usize> = HashMap::new();
    
    for blk in &block.blocks {
        let tup_iters = blk.instrs.clone().into_iter().enumerate().collect::<Vec<_>>();   
        for (line, instr) in tup_iters {
            if let Instruction::Value { args, dest, funcs: _, labels: _,  op: _, pos: _, op_type: _} = instr {
               map.insert(dest.clone().to_string(), *line);
            }
        }

        for (line, instr) in &tup_iters {
            match instr {
                Instruction::Value { args, dest, funcs: _, labels: _,  op: _, pos: _, op_type: _} => {
                    used.insert(dest);
                }
                _ => ()
            }
        }
        
        for (line, instr) in &tup_iters {
            match instr {
                Instruction::Value { args, dest, funcs: _, labels: _,  op: _, pos: _, op_type: _} => {
                    if let Some(&line) = map.get(dest) {
                        if used.contains(&dest) {
                            new_func.instrs.push(instr.clone());
                        }
                    }

                }
                _ => ()
            }
        }
    }
    new_func
}


