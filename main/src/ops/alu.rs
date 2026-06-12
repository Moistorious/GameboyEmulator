use crate::gameboy::Gameboy;
use crate::cpu::AluOp;

impl Gameboy {
    fn alu_op<F>(&mut self, value: u8, op: AluOp, f: F)
    where
        F: Fn(u8, u8) -> u8,
    {
        let a_before = self.cpu.a;
        let result = f(self.cpu.a, value);
        if let AluOp::Cp = op {
            self.cpu.set_flags(result == 0,false, false);
        }else {
            self.cpu.a = result;
           // self.cpu.set_flags()
        }
    }

}