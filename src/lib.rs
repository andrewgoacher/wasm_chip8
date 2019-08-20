extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate lib_chip;
use lib_chip::state::{State, delay_timer, sound_timer};
use lib_chip::memory::Memory;
use lib_chip::rom::Rom;
use lib_chip::opcode::{OpCode, AddOp, JumpOp, LoadOp};

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Stack(i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32);

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Registers(i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32);

#[wasm_bindgen]
pub struct Chip8 {
    memory: Ram,
    state: State,
    screen: Vec<u8>,
    keys: Keys
}

#[wasm_bindgen]
pub struct Keys {
    keys:Vec<u8>
}

#[wasm_bindgen]
impl Keys {
    pub fn new() -> Keys {
        Keys {
            keys: Vec::new()
        }
    }

    pub fn pressed(keys: Keys, key: u8) -> Keys {
        let mut vec = keys.keys;
        vec.push(key);
        Keys {
            keys: vec
        }
    }
}

#[wasm_bindgen]
pub struct Ram {
    memory: Memory
}

#[wasm_bindgen]
impl Ram {
    fn new() -> Ram {
        Ram {
            memory: Memory::new()
        }
    }
}

#[wasm_bindgen]
impl Chip8 {
    pub fn new() -> Chip8 {
        let state:State = Default::default();
        let screen = state.create_buffer();
        Chip8 {
            memory: Ram::new(),
            state,
            screen,
            keys: Keys::new()
        }
    }

    pub fn reset(&mut self) {
        self.memory.memory.reset();
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.memory.memory.read(addr)
    }

    pub fn should_clear(&self) -> bool {
        self.state.clear_flag
    }

    pub fn should_draw(&self) -> bool {
        self.state.draw_flag
    }

    pub fn set(self, addr: u16, val: u8) -> Chip8{
        let mut ram = self.memory;
        ram.memory.set(addr as usize, val);

        Chip8 {
            memory: ram,
            ..self
        }
    }

    pub fn get_opcode(&self) -> String {
        match self.state.last_opcode {
            OpCode::Unknown(op) => format!("Unknown({})", op),
            OpCode::XOR(x,y) => format!("XOR({},{})", x, y),
            OpCode::ADD(op) => {
                match op {
                    AddOp::ADD(vx, n) => format!("ADD {} to v[{}]", n, vx),
                    AddOp::ADDI(vx) => format!("ADD v[{}] to I", vx),
                    AddOp::ADDREG(vx, vy) => format!("ADD v[{}] to v[{}]", vy, vx)
                }
            },
            OpCode::AND(x,y) => format!("AND({},{})", x, y),
            OpCode::CALL(nnn) => format!("CALL({})", nnn),
            OpCode::CLS => String::from("Clear Screen"),
            OpCode::DRW(x,y,n) => format!("DRAW({},{},{})", x, y, n),
            OpCode::JP(op) => {
                match op {
                    JumpOp::JP(nnn) => format!("JUMP({})", nnn),
                    JumpOp::JPV0(nnn) => format!("JUMP(V0 + {})", nnn)
                }
            },
            OpCode::LD(op) => {
                match op {
                    LoadOp::LD(vx, n) => format!("LD {}, {}", vx, n),
                    LoadOp::LDB(vx) => format!("BCD v[{}]", vx),
                    LoadOp::LDDTVX(vx) => format!("delay_timer = v[{}]", vx),
                    LoadOp::LDF(vx) => format!("I = v[{}]", vx),
                    LoadOp::LDI(nnn) => format!("I = {}", nnn),
                    LoadOp::LDIV0X(vx) => format!("Store V0 to v[{}] to I++", vx),
                    LoadOp::LDKEY(vx) => format!("LDKEY(v[{}]", vx),
                    LoadOp::LDSTVX(vx) => format!("sound_timer = v[{}]", vx),
                    LoadOp::LDV0XI(vx) => format!("Store I from V0 to V[{}]", vx),
                    LoadOp::LDVXDT(vx) => format!("Set v[{}] = delay_timer", vx),
                    LoadOp::LDXY(vx, vy) => format!("v[{}] = v[{}]", vy, vx) 
                }
            },
            OpCode::OR(x,y) => format!("OR({},{})", x, y),
            OpCode::RET => String::from("RETURN"),
            OpCode::RND(vx, b) => format!("RANDOM(v[{}], {})", vx, b),
            OpCode::SHIFT(_) => String::from("SHIFT"),
            OpCode::SKIP(_) => String::from("SKIP"),
            OpCode::SUB(vx, vy) => format!("SUB(v[{}], v[{}])", vx, vy),
            OpCode::SUBN(vx, vy) => format!("SUBN(v[{}], v[{}])", vx, vy)
        }
    }

    pub fn timer(ch8:Chip8) -> Chip8 {
        let mut state = ch8.state;
        let d = delay_timer(&state);
        let s = sound_timer(&state);

        state = State {
            delay_timer: d,
            sound_timer: s,
            ..state
        };

        Chip8 {
            state,
            ..ch8
        }
    }

    pub fn step(ch8: Chip8) -> Chip8 {
        let mut state = ch8.state;
        let mut screen:Vec<u8> = ch8.screen;
        let mut ram = ch8.memory;
        let keys = ch8.keys;

        state = state.step(&mut ram.memory, &keys.keys[..], &mut screen);

        Chip8 {
            memory: ram,
            state,
            screen: screen,
            keys: Keys::new()
        }
    }

    pub fn clear_flags(self) -> Chip8 {
        let state = self.state;
        let new_state = State {
            draw_flag: false,
            clear_flag: false,
            ..state
        };

        Chip8 {
            state: new_state,
            ..self
        }
    }

    pub fn get_pixel(&self, x: u8, y: u8) -> u8 {
        // let w = ch8.state.width;
        let h = self.state.height;

        let idx = (u32::from(y) * h) + u32::from(x);

        self.screen[idx as usize]
    }
}