pub enum InstructionType {
    Noop,
    Addx,
}

pub fn get_instruction_type(input_line: &String) -> InstructionType {
    if input_line.starts_with("noop") {
        return InstructionType::Noop;
    }

    return InstructionType::Addx;
}

#[derive(Debug)]
pub struct CpuState {
    pub register_x: i32,
    pub cycle_count: usize,
}

impl CpuState {
    pub fn get_signal_strength(&self) -> i32 {
        return self.register_x * (self.cycle_count as i32);
    }
}
