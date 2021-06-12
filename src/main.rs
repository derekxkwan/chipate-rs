use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

static FONTSET: [u16;80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

const W : usize = 64;
const H : usize = 32;
const MEM_SZ : usize = 4096;
const STACK_SZ: usize = 16;
const SCL : usize = 2;
const REG_SZ: usize = 16;
const REG_N: usize = 16;
const MEMOFF: usize = 512;
type Keypad = [u16;16];

struct Chip8 {
    pc: u16,
    cur_op: u16,
    i: u16,
    sp: u8,
    disp: [u8; W*H],
    stack: [u16;STACK_SZ],
    v: [[u8;REG_SZ]; REG_N],
    mem: [u8; MEM_SZ],
    deltmr: u8,
    sndtmr: u8,
    drawflag: bool
}   

impl Chip8 {
    fn new(fontset: &[u8], buf: &[u8]) -> Chip8 {
        let v = [[0u8; REG_SZ]; REG_N];
        let disp = [0u8; W*H];
        let mut mem = [0u8; MEM_SZ];
        let stack = [0u16; STACK_SZ];

        for i in 0..80 {
            mem[i] = FONTSET[i];
        };

        for i in 0..buf.len() {
            mem[i+MEMOFF] = buf[i];
        };
        Chip8{pc: 0x200, cur_op: 0, i: 0, sp: 0, disp: disp,
            stack, v: v, mem: mem, deltmr: 0, sndtmr: 0,
            drawflag: false}
    }

    fn load(&mut self, buf: &[u8]) {
        for i in 0..buf.len(){
            self.mem[i+MEMOFF] = buf[i];  
        };
    }

    fn emulate_cycle(&mut self) {
        //fetch opcode
        let cur_op1 = self.m[pc];
        let cur_op2 = self.m[pc+1];
        let cur_op = cur_op1 << 8 | cur_op2;


        //decode opcode
        //exec opcode
        self.i = cur_op;
        self.pc += 2;
        //update timers
        if self.deltmr > 0 {
            self.deltmr -= 1;
        };
        if self.sndtmr > 0 {
            if self.sndtmr == 1 {
                println!("beep");
            };
            self.sndtmr -= 1;
        };
    }
}

fn main() {
    let fontset = [1u8; 80];
    let buf = [1u8; 80];
    let mut chip8 = Chip8::new(&fontset, &buf);
}
/*
fn main() -> Result<(), pixels::Error> {
   let evt_loop = EventLoop::new();
   let mut ipt = WinitInputHelper::new();
   let win = {
        let size = LogicalSize::new(W as f64, H as f64);
        WindowBuilder::new()
            .with_title("Chipate")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&evt_loop)
            .unwrap()
   };
   let mut pxls = {
        let win_size = win.inner_size();
        let surf_txr = SurfaceTexture::new(win_size.width, win_size.height, &win);
        Pixels::new(W as u32,H as u32, surf_txr)?
   };
   let mut paused = false;

   let mut disp = Display::new();
   let mut draw_state: Option<bool> = None;
   evt_loop.run(move |evt, _, ctrl_flow| {

   });
}
*/
