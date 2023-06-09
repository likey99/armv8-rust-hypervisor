#![allow(unused_macros)]
use core::arch::asm;
use aarch64_cpu::asm::barrier;
use aarch64_cpu::registers::ESR_EL2::EC::Value;
use aarch64_cpu::registers::*;
use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};

const SAVED_LINUX_REGS: usize = 31;

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct GeneralRegisters {
    pub exit_reason: u64,
    pub usr: [u64; 31],
}

macro_rules! save_regs_to_stack {
    () => {
        "
        sub     sp, sp, 34 * 8
        stp     x0, x1, [sp]
        stp     x2, x3, [sp, 2 * 8]
        stp     x4, x5, [sp, 4 * 8]
        stp     x6, x7, [sp, 6 * 8]
        stp     x8, x9, [sp, 8 * 8]
        stp     x10, x11, [sp, 10 * 8]
        stp     x12, x13, [sp, 12 * 8]
        stp     x14, x15, [sp, 14 * 8]
        stp     x16, x17, [sp, 16 * 8]
        stp     x18, x19, [sp, 18 * 8]
        stp     x20, x21, [sp, 20 * 8]
        stp     x22, x23, [sp, 22 * 8]
        stp     x24, x25, [sp, 24 * 8]
        stp     x26, x27, [sp, 26 * 8]
        stp     x28, x29, [sp, 28 * 8]
        mrs     x9, sp_el1
        mrs     x10, elr_el2
        mrs     x11, spsr_el2
        stp     x30, x9, [sp, 30 * 8]
        stp     x10, x11, [sp, 32 * 8]"
    };
}

macro_rules! restore_regs_from_stack {
    () => {
        "
        ldp     x10, x11, [sp, 32 * 8]
        ldp     x30, x9, [sp, 30 * 8]
        msr     sp_el1, x9
        msr     elr_el2, x10
        msr     spsr_el2, x11
    
        ldp     x28, x29, [sp, 28 * 8]
        ldp     x26, x27, [sp, 26 * 8]
        ldp     x24, x25, [sp, 24 * 8]
        ldp     x22, x23, [sp, 22 * 8]
        ldp     x20, x21, [sp, 20 * 8]
        ldp     x18, x19, [sp, 18 * 8]
        ldp     x16, x17, [sp, 16 * 8]
        ldp     x14, x15, [sp, 14 * 8]
        ldp     x12, x13, [sp, 12 * 8]
        ldp     x10, x11, [sp, 10 * 8]
        ldp     x8, x9, [sp, 8 * 8]
        ldp     x6, x7, [sp, 6 * 8]
        ldp     x4, x5, [sp, 4 * 8]
        ldp     x2, x3, [sp, 2 * 8]
        ldp     x0, x1, [sp]
        add     sp, sp, 34 * 8"
    };
}

#[derive(Debug)]
pub struct LinuxContext {
    pub usr: [u64; 31],
    pub spsr: u64,
    pub elr: u64,
    pub sctlr: u64,
    pub sp: u64,
}

#[allow(unused_unsafe)]
impl LinuxContext {
    pub fn new() -> Self {
        Self {
            usr: [0; 31],
            spsr: (SPSR_EL2::M::EL1h
                + SPSR_EL2::I::Masked
                + SPSR_EL2::F::Masked
                + SPSR_EL2::A::Masked
                + SPSR_EL2::D::Masked)
                .value as u64,
            elr: 0,
            sctlr: 0,
            sp: 0,
        }
    }
    pub fn load_from(linux_sp: usize) -> Self {
        let regs = unsafe { core::slice::from_raw_parts(linux_sp as *const u64, SAVED_LINUX_REGS) };
        let mut ret = Self {
            usr: [0; 31],
            spsr: SPSR_EL2.get(),
            elr: ELR_EL2.get(),
            sctlr: SCTLR_EL2.get(),
            sp: SP.get(),
        };
        for i in 0..31 {
            ret.usr[i] = regs[i];
        }
        ret
    }

    /// Restore system registers.
    pub fn restore(&self) {
        unsafe {
            SPSR_EL2.set(self.spsr);
            ELR_EL2.set(self.elr);
            SCTLR_EL2.set(self.sctlr);
            SP.set(self.sp);
        }        
    }
}