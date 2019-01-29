mod klass_parser;
mod opcode;
mod runtime;

use opcode::*;
// use runtime::*;

pub fn exec_method2(
    context: &mut runtime::VmContext,
    meth: runtime::OtMethod,
) -> Option<runtime::JvmValue> {
    let mut vars = runtime::InterpLocalVars::of();
    exec_method(context, meth.get_klass_name(), &meth.get_code(), &mut vars)
}

pub fn exec_method(
    context: &mut runtime::VmContext,
    klass_name: String,
    instr: &Vec<u8>,
    lvt: &mut runtime::InterpLocalVars,
) -> Option<runtime::JvmValue> {
    let mut current = 0;
    let mut eval = runtime::InterpEvalStack::of();

    // dbg!(instr);
    loop {
        let repo = context.get_repo().clone();
        let my_klass_name = klass_name.clone();
        let opt_ins = instr.get(current);
        let ins: u8 = match opt_ins {
            Some(value) => *value,
            None => panic!("Byte {} has no value", current),
        };
        current += 1;

        match ins {
            Opcode::ACONST_NULL => eval.aconst_null(),

            Opcode::ALOAD => {
                eval.push(lvt.load(instr[current]));
                current += 1;
            }
            Opcode::ALOAD_0 => eval.push(lvt.load(0)),

            Opcode::ALOAD_1 => eval.push(lvt.load(1)),

            Opcode::ARETURN => break Some(eval.pop()),
            Opcode::ASTORE => {
                lvt.store(instr[current], eval.pop());
                current += 1;
            }
            Opcode::ASTORE_0 => lvt.store(0, eval.pop()),

            Opcode::ASTORE_1 => lvt.store(1, eval.pop()),

            Opcode::BIPUSH => {
                eval.iconst(instr[current] as i32);
                current += 1;
            }
            Opcode::DADD => eval.dadd(),

            Opcode::DCONST_0 => eval.dconst(0.0),

            Opcode::DCONST_1 => eval.dconst(1.0),

            Opcode::DLOAD => {
                eval.push(lvt.load(instr[current]));
                current += 1;
            }

            Opcode::DLOAD_0 => eval.push(lvt.load(0)),

            Opcode::DLOAD_1 => eval.push(lvt.load(1)),

            Opcode::DLOAD_2 => eval.push(lvt.load(2)),

            Opcode::DLOAD_3 => eval.push(lvt.load(3)),

            Opcode::DRETURN => break Some(eval.pop()),
            Opcode::DSTORE => {
                lvt.store(instr[current], eval.pop());
                current += 1;
            }
            Opcode::DSTORE_0 => lvt.store(0, eval.pop()),

            Opcode::DSTORE_1 => lvt.store(1, eval.pop()),

            Opcode::DSTORE_2 => lvt.store(2, eval.pop()),

            Opcode::DSTORE_3 => lvt.store(3, eval.pop()),

            Opcode::DSUB => eval.dsub(),

            Opcode::DUP => eval.dup(),

            Opcode::DUP_X1 => eval.dupX1(),

            // GETFIELD => {
            //     let cp_lookup = ((int) instr[current++] << 8) + (int) instr[current++];
            //     runtime::OtField field = context.get_repo().lookupField(klass_name, (short) cp_lookup);
            //     runtime::JvmValue receiver = eval.pop();
            //     // VERIFY: Should check to make sure receiver is an Opcode::A
            //     runtime::OtObj obj = heap.findObject(receiver.value);
            //     eval.push(obj.getField(field));
            // },
            // GETSTATIC => {
            //     let cp_lookup = ((int) instr[current++] << 8) + (int) instr[current++];
            //     runtime::OtField f = context.get_repo().lookupField(klass_name, (short) cp_lookup);
            //     runtime::OtKlass fgKlass = f.getKlass();
            //     eval.push(fgKlass.getStaticField(f));
            // },
            Opcode::GOTO => {
                current += ((instr[current] as usize) << 8) + instr[current + 1] as usize
            }

            Opcode::I2D => eval.i2d(),

            Opcode::IADD => eval.iadd(),

            Opcode::IAND => eval.iand(),

            Opcode::ICONST_0 => eval.iconst(0),

            Opcode::ICONST_1 => eval.iconst(1),

            Opcode::ICONST_2 => eval.iconst(2),

            Opcode::ICONST_3 => eval.iconst(3),

            Opcode::ICONST_4 => eval.iconst(4),

            Opcode::ICONST_5 => eval.iconst(5),

            Opcode::ICONST_M1 => eval.iconst(-1),

            Opcode::IDIV => eval.idiv(),

            Opcode::IF_ICMPEQ => {
                let jumpTo = (instr[current] as usize) << 8 + instr[current + 1] as usize;
                if massage_to_jvm_int_and_equate(eval.pop(), eval.pop()) {
                    current += jumpTo;
                } else {
                    current += 2;
                }
            }
            Opcode::IF_ICMPNE => {
                let jumpTo = (instr[current] as usize) << 8 + instr[current + 1] as usize;
                if massage_to_jvm_int_and_equate(eval.pop(), eval.pop()) {
                    current += 2;
                } else {
                    current += jumpTo;
                }
            }

            // Opcode::IFEQ => {
            //     let jumpTo = (instr[current] as usize) << 8 + instr[current + 1] as usize;
            //     let v1 = match eval.pop() {
            //         runtime::JvmValue::ObjRef { val: v } => v,
            //         _ => panic!("Value not of reference type found for IFEQ"),
            //     };
            //     let v2 = match eval.pop() {
            //         runtime::JvmValue::ObjRef { val: v } => v,
            //         _ => panic!("Value not of reference type found for IFEQ"),
            //     };
            //     if v1 == v2 {
            //         current += jumpTo; // - 1; // The -1 is necessary as we've already inc'd current
            //     }
            // }    ,
            // Opcode::IFGE => {
            //     v = eval.pop();
            //     jumpTo = ((int) instr[current++] << 8) + (int) instr[current++];
            //     if (v.value >= 0L) {
            //         current += jumpTo - 1; // The -1 is necessary as we've already inc'd current
            //     }
            // } ,
            // Opcode::IFGT => {
            //     v = eval.pop();
            //     jumpTo = ((int) instr[current++] << 8) + (int) instr[current++];
            //     if (v.value > 0L) {
            //         current += jumpTo - 1; // The -1 is necessary as we've already inc'd current
            //     }
            // },
            // Opcode::IFLE => {
            //     v = eval.pop();
            //     jumpTo = ((int) instr[current++] << 8) + (int) instr[current++];
            //     if (v.value <= 0L) {
            //         current += jumpTo - 1; // The -1 is necessary as we've already inc'd current
            //     }
            // },
            // Opcode::IFLT => {
            //     v = eval.pop();
            //     jumpTo = ((int) instr[current++] << 8) + (int) instr[current++];
            //     if (v.value < 0L) {
            //         current += jumpTo - 1; // The -1 is necessary as we've already inc'd current
            //     }
            // },
            // Opcode::IFNE => {
            //     v = eval.pop();
            //     jumpTo = ((int) instr[current] << 8) + (int) instr[current + 1];
            //     if (v.value != 0L) {
            //         current += jumpTo - 1;  // The -1 is necessary as we've already inc'd current
            //     }
            // },
            Opcode::IFNONNULL => {
                let jumpTo = ((instr[current] as usize) << 8) + instr[current + 1] as usize;

                match eval.pop() {
                    runtime::JvmValue::ObjRef { val: v } => {
                        if !v.is_null() {
                            current += jumpTo;
                        } else {
                            current += 2;
                        }
                    }
                    _ => panic!(
                        "Value not of reference type found for IFNULL at {}",
                        (current - 1)
                    ),
                };
            }
            Opcode::IFNULL => {
                let jumpTo = ((instr[current] as usize) << 8) + instr[current + 1] as usize;

                match eval.pop() {
                    runtime::JvmValue::ObjRef { val: v } => {
                        if v.is_null() {
                            // println!("Ins[curr]: {} and {}", instr[current], instr[current + 1]);
                            // println!("Attempting to jump by: {} from {}", jumpTo, current);
                            current += jumpTo;
                        } else {
                            current += 2;
                        }
                    }
                    _ => panic!(
                        "Value not of reference type found for IFNULL at {}",
                        (current - 1)
                    ),
                };
            }
            Opcode::IINC => {
                lvt.iinc(instr[current], instr[current + 1]);
                current += 2;
            }

            Opcode::ILOAD => {
                eval.push(lvt.load(instr[current]));
                current += 1
            }

            Opcode::ILOAD_0 => eval.push(lvt.load(0)),

            Opcode::ILOAD_1 => eval.push(lvt.load(1)),

            Opcode::ILOAD_2 => eval.push(lvt.load(2)),

            Opcode::ILOAD_3 => eval.push(lvt.load(3)),

            Opcode::IMUL => eval.imul(),

            Opcode::INEG => eval.ineg(),

            Opcode::INVOKESPECIAL => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;
                let current_klass = repo.lookup_klass(klass_name.clone()).clone();
                dbg!(current_klass.clone());
                // FIXME DOES NOT HANDLE CALL ARGS YET
                dispatch_invoke(context, current_klass, cp_lookup, &mut eval, 1);
            }
            Opcode::INVOKESTATIC => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;
                let current_klass = repo.lookup_klass(klass_name.clone()).clone();
                dbg!(current_klass.clone());
                // FIXME DOES NOT HANDLE CALL ARGS YET
                dispatch_invoke(context, current_klass, cp_lookup, &mut eval, 0);
            }
            // FIXME DOES NOT ACTUALLY DO VIRTUAL LOOKUP YET
            Opcode::INVOKEVIRTUAL => {
                // let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                // current += 2;
                // dispatch_invoke(context.get_repo().lookup_method_virtual(&klass_name, cp_lookup), &eval);
            }
            Opcode::IOR => eval.ior(),

            Opcode::IREM => eval.irem(),

            Opcode::IRETURN => break Some(eval.pop()),
            Opcode::ISTORE => {
                lvt.store(instr[current], eval.pop());
                current += 1;
            }
            Opcode::ISTORE_0 => lvt.store(0, eval.pop()),

            Opcode::ISTORE_1 => lvt.store(1, eval.pop()),

            Opcode::ISTORE_2 => lvt.store(2, eval.pop()),

            Opcode::ISTORE_3 => lvt.store(3, eval.pop()),

            Opcode::ISUB => eval.isub(),
            // Dummy implementation
            Opcode::LDC => {
                // System.out.print("Executing " + op + " with param bytes: ");
                // for (int i = current; i < current + num; i++) {
                //     System.out.print(instr[i] + " ");
                // }
                // current += num;
                // System.out.println();
            }

            // FIXME TEMP
            Opcode::MONITORENTER => {
                eval.pop();
            }
            Opcode::MONITOREXIT => {
                eval.pop();
            }

            Opcode::NEW => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;
                let current_klass = repo.lookup_klass(klass_name.clone()).clone();

                let klass_name = match current_klass.lookup_cp(cp_lookup) {
                    runtime::CpEntry::class { idx } => "DUMMY_CLASS".to_string(), // FIXME
                    _ => panic!(
                        "Non-class found in {} at CP index {}",
                        current_klass.get_name(),
                        cp_lookup
                    ),
                };

                let klass = repo.lookup_klass(klass_name);
                // let heap = context.get_heap();
                eval.push(runtime::JvmValue::ObjRef {
                    // FIXME
                    // val: runtime::OtObj::get_null(),
                    val: context.allocate_obj(klass),
                });
            }
            Opcode::NOP => {
                ();
            }

            Opcode::POP => {
                eval.pop();
            }
            Opcode::POP2 => {
                let _discard: runtime::JvmValue = eval.pop();
                // FIXME Change to type match
                // if (discard.type == JVMType.J || discard.type == JVMType.D) {

                // }
                eval.pop();
            }
            Opcode::PUTFIELD => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;

                let putf: runtime::OtField = repo.lookup_field(my_klass_name.clone(), cp_lookup);
                let val: runtime::JvmValue = eval.pop();

                let recvp: runtime::JvmValue = eval.pop();
                // VERIFY: Should check to make sure receiver is an A
                // FIXME Match expression & destructure for recvp
                let obj = match recvp {
                    runtime::JvmValue::ObjRef { val: v } => v,
                    _ => panic!("Not an object ref at {}", (current - 1)),
                };

                obj.put_field(putf, val);
            }
            Opcode::PUTSTATIC => {
                let cp_lookup = ((instr[current] as u16) << 8) + instr[current + 1] as u16;
                current += 2;

                let puts = repo.lookup_field(my_klass_name.clone(), cp_lookup);
                let f_klass = puts.get_klass();
                f_klass.set_static_field(puts.get_name(), eval.pop());
            }
            Opcode::RETURN => break None,
            Opcode::SIPUSH => {
                let vtmp = ((instr[current] as i32) << 8) + instr[current + 1] as i32;
                eval.iconst(vtmp);
                current += 2;
            }
            Opcode::SWAP => {
                let val1 = eval.pop();
                let val2 = eval.pop();
                eval.push(val1);
                eval.push(val2);
            }
            // Disallowed opcodes
            Opcode::BREAKPOINT => break Some(runtime::JvmValue::Boolean { val: false }),
            Opcode::IMPDEP1 => break Some(runtime::JvmValue::Boolean { val: false }),
            Opcode::IMPDEP2 => break Some(runtime::JvmValue::Boolean { val: false }),
            Opcode::JSR => break Some(runtime::JvmValue::Boolean { val: false }),
            Opcode::JSR_W => break Some(runtime::JvmValue::Boolean { val: false }),
            Opcode::RET => break Some(runtime::JvmValue::Boolean { val: false }),

            _ => panic!(
                "Illegal opcode byte: {} encountered at position {}. Stopping.",
                ins,
                (current - 1)
            ),
        }
    }
}

fn massage_to_jvm_int_and_equate(v1: runtime::JvmValue, v2: runtime::JvmValue) -> bool {
    match v1 {
        runtime::JvmValue::Boolean { val: b } => match v2 {
            runtime::JvmValue::Boolean { val: b1 } => b == b1,
            _ => panic!("Value not of reference type found for IF_ICMPEQ"),
        },
        // Byte { val: i8 },
        // Short { val: i16 },
        runtime::JvmValue::Int { val: i } => match v2 {
            runtime::JvmValue::Int { val: i1 } => i == i1,
            _ => panic!("Value not of reference type found for IF_ICMPEQ"),
        },

        // Long { val: i64 },
        // Float { val: f32 },
        // Double { val: f64 },
        // Char { val: char },
        runtime::JvmValue::ObjRef { val: v } => {
            panic!("Value should not be reference type for IF_ICMPEQ")
        }
        _ => panic!("Value not implemented for IF_ICMPEQ"),
    }
}

fn dispatch_invoke(
    context: &mut runtime::VmContext,
    current_klass: runtime::OtKlass,
    cp_lookup: u16,
    eval: &mut runtime::InterpEvalStack,
    to_read: u8,
) -> () {
    let fq_name_desc = current_klass.cp_as_string(cp_lookup);
    let klz_idx = match current_klass.lookup_cp(cp_lookup) {
        runtime::CpEntry::methodref { clz_idx, nt_idx } => clz_idx,
        _ => panic!(
            "Non-methodref found in {} at CP index {}",
            current_klass.get_name(),
            cp_lookup
        ),
    };
    let dispatch_klass_name = current_klass.cp_as_string(klz_idx);
    let repo = context.get_repo().clone();
    let callee = repo.lookup_method_exact(&dispatch_klass_name, fq_name_desc);

    // FIXME - General setup requires call args
    match exec_method2(context, callee) {
        Some(val) => eval.push(val),
        None => (),
    }
}

fn parse_class(bytes: Vec<u8>, fname: String) -> runtime::OtKlass {
    let mut parser = klass_parser::oc_parser::new(bytes, fname);
    parser.parse();
    parser.klass()
}

#[cfg(test)]
mod tests;
