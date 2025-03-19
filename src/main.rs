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
    Syscall,
}

enum Register {
    EAX,
    EBX,
    ECX,
    EDX,
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

                let value = instruction
                    .args
                    .get(1)
                    .ok_or("too few arguments for mov opcode: missing value")?;

                self.set_register(register, *value);
            }

            Opcode::Syscall => {
                let raw_syscall = instruction
                    .args
                    .get(0)
                    .ok_or("too few arguments for syscall opcode: missing syscall value")?;

                let syscall = Syscall::try_from(*raw_syscall).unwrap();

                match syscall {
                    Syscall::Exit => {
                        self.exit_code = self.eax as u8;
                        self.running = false;
                    }
                }
            }
        }

        Ok(())
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
        Instruction {
            opcode: Opcode::Move,
            args: vec![Register::EAX as usize, 42],
        },
        Instruction {
            opcode: Opcode::Syscall,
            args: vec![Syscall::Exit as usize],
        },
    ];

    while cpu.running {
        cpu.execute()?;
        cpu.increment_isp();
    }

    println!("Exit code: {}", cpu.exit_code);

    Ok(())
}
