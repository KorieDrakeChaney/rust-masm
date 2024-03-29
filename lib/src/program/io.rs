use miden::math::Felt;

use super::{error::MidenProgramError, Instruction, MidenProgram};

impl MidenProgram {
    /// Pushes a value onto the stack.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to push onto the stack.
    pub fn push(&mut self, value: u64) {
        self.add_instruction(Instruction::Push(Felt::from(value)));
    }
    /// Pushes the nth advice onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The index of the advice to push onto the stack.
    ///
    /// # Errors
    ///
    /// Returns `MidenProgramError::AdviceStackReadOutOfBounds` if `n` is greater than the length of the advice stack.
    pub fn adv_push(&mut self, n: usize) {
        let op: Instruction;
        if n > self.advice_stack.len() {
            op = Instruction::Error(MidenProgramError::AdviceStackReadOutOfBounds(
                n,
                self.advice_stack.len(),
            ));
        } else {
            op = Instruction::AdvPush(n);
        }
        self.add_instruction(op);
    }
    /// Stores the second value on the stack in memory at the address specified by the first value on the stack.
    pub fn mem_store(&mut self) {
        self.add_instruction(Instruction::MemStore);
    }

    /// Stores the first value in the stack in memory at the address specified by `n`.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to store the first value on the stack.
    pub fn mem_store_n(&mut self, n: u32) {
        self.add_instruction(Instruction::MemStoreImm(n));
    }

    /// Uses the first value on the stack as an address and stores the 2-5th values on the stack in memory at that address.
    pub fn mem_store_w(&mut self) {
        self.add_instruction(Instruction::MemStoreW);
    }

    /// Stores the first word in the stack in memory at the address specified by `n`.
    /// # Arguments
    /// * `n` - The address to store the first word on the stack.
    pub fn mem_store_w_n(&mut self, n: u32) {
        self.add_instruction(Instruction::MemStoreWImm(n));
    }

    /// Assumes top value on the stack is an address and pops it off, then loads the value at that address from RAM onto the stack.
    pub fn mem_load(&mut self) {
        self.add_instruction(Instruction::MemLoad);
    }

    /// Loads the value at address `n` from RAM onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to load onto the stack.
    pub fn mem_load_n(&mut self, n: u32) {
        self.add_instruction(Instruction::MemLoadImm(n));
    }

    /// Assumes top value on the stack is an address and pops it off, then loads the word at that address from RAM onto the stack.
    pub fn mem_load_w(&mut self) {
        self.add_instruction(Instruction::MemLoadW);
    }

    /// Loads the word at address `n` from RAM onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to load onto the stack.
    pub fn mem_load_w_n(&mut self, n: u32) {
        self.add_instruction(Instruction::MemLoadWImm(n));
    }
}
