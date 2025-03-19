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
    Eax = 0xff,
    Ebx = 0xfe,
    Ecx = 0xfd,
    Edx = 0xfc,
}

impl TryFrom<usize> for Register {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            x if x == Self::Eax as usize => Ok(Self::Eax),
            x if x == Self::Ebx as usize => Ok(Self::Ebx),
            x if x == Self::Ecx as usize => Ok(Self::Ecx),
            x if x == Self::Edx as usize => Ok(Self::Edx),
            _ => Err(()),
        }
    }
}

struct Instruction {
    opcode: Opcode,
    args: Vec<usize>,
}

#[derive(Default)]
struct Cpu {
    isp: usize,
    program: Vec<Instruction>,
    running: bool,
    exit_code: u8,

    eax: usize,
    ebx: usize,
    ecx: usize,
    edx: usize,
}

impl Cpu {
    fn execute(&mut self) -> Result<(), &'static str> {
        let instruction = self.program.get(self.isp).ok_or("isp overflow")?;

        match instruction.opcode {
            Opcode::Move => {
                let raw_register = instruction
                    .args
                    .first()
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
                    .first()
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
            Register::Eax => &self.eax,
            Register::Ebx => &self.ebx,
            Register::Ecx => &self.ecx,
            Register::Edx => &self.edx,
        }
    }

    fn set_register(&mut self, register: Register, value: usize) {
        match register {
            Register::Eax => self.eax = value,
            Register::Ebx => self.ebx = value,
            Register::Ecx => self.ecx = value,
            Register::Edx => self.edx = value,
        };
    }

    fn increment_isp(&mut self) {
        self.isp += 1;
    }
}

fn main() -> Result<(), &'static str> {
    let mut cpu = Cpu {
        running: true,
        program: vec![
            // mov ebx, 34
            Instruction {
                opcode: Opcode::Move,
                args: vec![Register::Ebx as usize, 34],
            },
            // mov eax, ebx
            Instruction {
                opcode: Opcode::Move,
                args: vec![Register::Eax as usize, Register::Ebx as usize],
            },
            // add eax, 35
            Instruction {
                opcode: Opcode::Add,
                args: vec![35],
            },
            // mov ebx, eax
            Instruction {
                opcode: Opcode::Move,
                args: vec![Register::Ebx as usize, Register::Eax as usize],
            },
            // mov eax, 0
            Instruction {
                opcode: Opcode::Move,
                args: vec![Register::Eax as usize, Syscall::Exit as usize],
            },
            // syscall
            Instruction {
                opcode: Opcode::Syscall,
                args: vec![],
            },
        ],
        ..Default::default()
    };

    while cpu.running {
        cpu.execute()?;
        cpu.increment_isp();
    }

    println!("Exit code: {}", cpu.exit_code);

    Ok(())
}
