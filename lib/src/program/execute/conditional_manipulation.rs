use math::{fields::f64::BaseElement, FieldElement};

use crate::{MidenProgram, Operand};

pub fn execute_conditional(program: &mut MidenProgram, operand: &Operand) {
    match operand {
        Operand::CSwap => {
            if let Some(c) = program.stack.pop_front() {
                if c == BaseElement::ONE {
                    program.stack.swap(0, 1);
                }
            }
        }
        Operand::CSwapW => {
            if let Some(c) = program.stack.pop_front() {
                if c == BaseElement::ONE {
                    program.stack.swap(0, 4);
                    program.stack.swap(1, 5);
                    program.stack.swap(2, 6);
                    program.stack.swap(3, 7);
                }
            }
        }
        Operand::CDrop => {
            if let (Some(c), Some(b), Some(a)) = (
                program.stack.pop_front(),
                program.stack.pop_front(),
                program.stack.pop_front(),
            ) {
                if c == BaseElement::ZERO {
                    program.stack.push_front(a);
                } else {
                    program.stack.push_front(b);
                }
            }
        }
        Operand::CDropW => {
            if let (Some(c), b, a) = (
                program.stack.pop_front(),
                (
                    program.stack.pop_front(),
                    program.stack.pop_front(),
                    program.stack.pop_front(),
                    program.stack.pop_front(),
                ),
                (
                    program.stack.pop_front(),
                    program.stack.pop_front(),
                    program.stack.pop_front(),
                    program.stack.pop_front(),
                ),
            ) {
                if c == BaseElement::ZERO {
                    if let (Some(a3), Some(a2), Some(a1), Some(a0)) = a {
                        program.stack.push_front(a0);
                        program.stack.push_front(a1);
                        program.stack.push_front(a2);
                        program.stack.push_front(a3);
                    }
                } else {
                    if let (Some(b3), Some(b2), Some(b1), Some(b0)) = b {
                        program.stack.push_front(b0);
                        program.stack.push_front(b1);
                        program.stack.push_front(b2);
                        program.stack.push_front(b3);
                    }
                }
            }
        }

        _ => {}
    }
}
