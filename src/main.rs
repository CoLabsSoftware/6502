use std::iter::Cycle;

type Byte = u8;
type Word = u16;



struct CPU6502 {
    pc: Word,
    sp: Word,
    a: Byte,
    x: Byte,
    y: Byte,
    c: Byte,
    z: Byte,
    i: Byte,
    d: Byte,
    b: Byte,
    v: Byte,
    n: Byte,
}

impl CPU6502 {
    fn new() -> Self {
        // Initialize your CPU6502 instance here
        CPU6502 {
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
            c: 1,
            z: 1,
            i: 1,
            d: 1,
            b: 1,
            v: 1,
            n: 1,
        }
    }

    //opcodes
    const INS_LDA_IM:Byte = 0xA9;
    fn perform_ins_lda_im(&mut self, Cycles: &mut u32, memory: &mut Memory){
        let value: Byte = self.fetch_byte(Cycles, memory);
        self.a = value;

        if self.a == 0
        {
            self.z = 1
        } else
        {
            self.z = 0
        }

        if (self.a & 0b10000000) > 0
        {
            self.n = 1
        } else
        {
            self.n = 0
        }
    }

    fn reset(&mut self, memory: &mut Memory)
    {
        self.pc = 0xFFFC;
        self.sp = 0x0100;
        self.c = 0;
        self.z = 0;
        self.i = 0;
        self.d = 0;
        self.b = 0;
        self.v = 0;
        self.n = 0;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        memory.initialize();
    }

    fn fetch_byte(&mut self, cycles: &mut u32, memory: &mut Memory,) -> Byte
    {

        if *cycles == 0 {
            return 0; // or some other default value
        }

        let data:Byte = memory.data[self.pc as usize];
        self.pc+= 1;
        *cycles -= 1;
        return data;
    }
    fn execute(&mut self, Cycles: &mut u32, memory: &mut Memory)
    {
        let mut i: u32 = *Cycles;
        println!("{}", i);
        while i > 0
        {
            let Instruction: Byte = self.fetch_byte(Cycles, memory);
            println!("instruction is: ");
            println!("{}", Instruction);
            match Instruction
            {
                INS_LDA_IM=> self.perform_ins_lda_im(Cycles, memory),


            }
            i = i - 1;
        }


    }
}

struct Memory {
    data: [Byte; Memory::MAX_MEMORY as usize],
}

impl Memory {
    const MAX_MEMORY: u32 = 1024 * 64;

    fn new() -> Self {
        Memory {
            data: [0; Memory::MAX_MEMORY as usize],
        }
    }

    fn initialize(&mut self) {
        let mut i = 0;
        while i < Memory::MAX_MEMORY as usize {
            self.data[i] = 0;
            i += 1;
        }
    }

    fn operator(&self, address: usize) -> &Byte {
        //put assert here
        return &self.data[address];
    }

    fn set(&mut self, address: usize, value: Byte) {
        if address < Memory::MAX_MEMORY as usize {
            self.data[address] = value;
        }
    }


}

fn main() {
    let mut cpu = CPU6502::new();
    let mut memory = Memory::new();
    cpu.reset(&mut memory);
    //in line program start
    memory.set(0xFFFC, 0xA9);
    memory.set(0xFFFD, 0x42);
    //inline program end
    cpu.execute( &mut 2, &mut memory);
    println!("done");
}