enum Syscall {
    Exit,
}

impl TryFrom<usize> for Syscall {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            x if x == Self::Exit as usize => Ok(Self::Exit),
            _ => Err(()),
        }
    }
}

enum Opcode {
    Move,
    Add,
    Syscall,
}

enum Register {
    // TODO: Add a better way to detect use of registers.
    EAX = 0xff,
    EBX = 0xfe,
    ECX = 0xfd,
    EDX = 0xfc,
}

impl TryFrom<usize> for Register {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            x if x == Self::EAX as usize => Ok(Self::EAX),
            x if x == Self::EBX as usize => Ok(Self::EBX),
            x if x == Self::ECX as usize => Ok(Self::ECX),
            x if x == Self::EDX as usize => Ok(Self::EDX),
            _ => Err(()),
        }
    }
}

struct Instruction {
    opcode: Opcode,
    args: Vec<usize>,
}

#[derive(Default)]
struct CPU {
    isp: usize,
    program: Vec<Instruction>,
    running: bool,
    exit_code: u8,

    eax: usize,
    ebx: usize,
    ecx: usize,
    edx: usize,
}

impl CPU {
    fn execute(&mut self) -> Result<(), &'static str> {
        let instruction = self.program.get(self.isp).ok_or("isp overflow")?;

        match instruction.opcode {
            Opcode::Move => {
                let raw_register = instruction
                    .args
                    .get(0)
                    .ok_or("too few arguments for mov opcode: missing register")?;

                let register =
                    Register::try_from(*raw_register).or(Err("invalid register for mov opcode"))?;

                let mut value = instruction
                    .args
                    .get(1)
                    .ok_or("too few arguments for mov opcode: missing value")?;

                if let Ok(reg) = Register::try_from(*value) {
                    value = self.get_register(reg);
                }

                self.set_register(register, *value);
            }

            Opcode::Add => {
                // TODO: Add second argument.
                let value = instruction
                    .args
                    .get(0)
                    .ok_or("too few arguments for add opcode: missing value")?;

                self.eax += value;
            }

            Opcode::Syscall => {
                let syscall = Syscall::try_from(self.eax).or(Err("invalid syscall"))?;

                match syscall {
                    Syscall::Exit => {
                        self.exit_code = self.ebx as u8;
                        self.running = false;
                    }
                }
            }
        }

        Ok(())
    }

    fn get_register(&self, register: Register) -> &usize {
        match register {
            Register::EAX => &self.eax,
            Register::EBX => &self.ebx,
            Register::ECX => &self.ecx,
            Register::EDX => &self.edx,
        }
    }

    fn set_register(&mut self, register: Register, value: usize) {
        match register {
            Register::EAX => self.eax = value,
            Register::EBX => self.ebx = value,
            Register::ECX => self.ecx = value,
            Register::EDX => self.edx = value,
        };
    }

    fn increment_isp(&mut self) {
        self.isp += 1;
    }
}

fn main() -> Result<(), &'static str> {
    let mut cpu = CPU::default();
    cpu.running = true;

    cpu.program = vec![
        // mov ebx, 34
        Instruction {
            opcode: Opcode::Move,
            args: vec![Register::EBX as usize, 34],
        },
        // mov eax, ebx
        Instruction {
            opcode: Opcode::Move,
            args: vec![Register::EAX as usize, Register::EBX as usize],
        },
        // add eax, 35
        Instruction {
            opcode: Opcode::Add,
            args: vec![35],
        },
        // mov ebx, eax
        Instruction {
            opcode: Opcode::Move,
            args: vec![Register::EBX as usize, Register::EAX as usize],
        },
        // mov eax, 0
        Instruction {
            opcode: Opcode::Move,
            args: vec![Register::EAX as usize, Syscall::Exit as usize],
        },
        // syscall
        Instruction {
            opcode: Opcode::Syscall,
            args: vec![],
        },
    ];

    while cpu.running {
        cpu.execute()?;
        cpu.increment_isp();
    }

    println!("Exit code: {}", cpu.exit_code);

    Ok(())
}
