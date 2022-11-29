use core::{arch::asm, mem::size_of, marker::PhantomData};

pub struct IdtDescriptor<'a>(u64, PhantomData<&'a ()>);

impl IdtDescriptor<'static> {
    pub unsafe fn load(&self) {
        asm!("lidt [{}]", in(reg) self, options(nostack));
    }
}

#[repr(C)]
#[repr(align(16))]
pub struct Idt {
    table: [GateDescriptor; 256],
}

impl Idt {
    pub const fn new() -> Self {
        Idt { table: [GateDescriptor::empty(); 256] }
    }

    pub fn descriptor<'a>(&'a self) -> IdtDescriptor<'a> {
        IdtDescriptor((size_of::<Self>() as u16 as u64) | (self as *const Idt as u64) << 16, PhantomData)
    }
}

pub static mut IDT: Idt = Idt::new();

use lazy_static::lazy_static;
lazy_static! {
    pub static ref IDT_DESCRIPTOR: IdtDescriptor<'static> = unsafe { IDT.descriptor() };
}

/// from https://www.amd.com/system/files/TechDocs/24593.pdf Table 8-1
#[repr(u8)]
pub enum Vector {
    DivideByZero = 0,
    Debug,
    NonMaskableInterrupt,
    Breakpoint,
    Overflow,
    BoundRange,
    InvalidOpcode,
    DeviceNotAvailable,
    DoubleFault,

    InvalidTss = 10,
    SegmentNotPresent,
    Stack,
    GeneralProtection,
    PageFault,

    X87FloatingPointExceptionPending = 16,
    AlignmentCheck,
    MachineCheck,
    SimdFloatingPoint,

    ControlProtectionException = 21,

    HypervisorInjectionException = 28,
    VmmCommunicationException,
    SecurityException,
}

#[derive(Copy, Clone)]
#[repr(C)]
#[repr(align(16))]
pub struct GateDescriptor {
    offset_0_15: u16,
    selector: u16,
    ist: u8,
    flags: u8,
    offset_16_31: u16,
    offset_32_63: u32,
    res: u32,
}

#[repr(u8)]
pub enum GateType {
    Interrupt = 0xE,
    Trap = 0xF,
}

impl GateDescriptor {
    pub const fn new(
        offset: u64,
        selector: u16,
        ist: u8,
        gate_type: GateType,
        privilege_level: u8,
        present: bool,
    ) -> Self {
        GateDescriptor {
            offset_0_15: offset as u16,
            selector,
            ist: ist | 7,
            flags: gate_type as u8 | privilege_level << 5 | (present as u8) << 7,
            offset_16_31: (offset >> 16) as u16,
            offset_32_63: (offset >> 32) as u32,
            res: 0,
        }
    }

    pub const fn empty() -> Self {
        Self::new(0, 0, 0, GateType::Interrupt, 0, false)
    }

    pub fn set_offset(&mut self, offset: u64) {
        self.offset_0_15 = offset as u16;
        self.offset_16_31 = (offset >> 16) as u16;
        self.offset_32_63 = (offset >> 32) as u32;
    }

    pub fn set_selector(&mut self, selector: u16) {
        self.selector = selector
    }

    pub fn set_ist(&mut self, ist: u8) {
        self.ist = ist | 7
    }

    pub fn set_type(&mut self, gate_type: GateType) {
        self.flags = self.flags & !1 | gate_type as u8;
    }

    pub fn set_privilege(&mut self, privilege_level: u8) {
        self.flags = self.flags & !(1 << 5) | privilege_level << 5;
    }

    pub fn set_present(&mut self, present: bool) {
        self.flags = self.flags & !(1 << 7) | (present as u8) << 7;
    }
}
