#![feature(strict_provenance)]
#![warn(fuzzy_provenance_casts)]

use clap::Parser;
use std::{fs::File, ptr::copy_nonoverlapping};
use std::io::Read;
use windows::Win32::System::Memory::{self, VirtualAlloc};
use core::ffi::c_void;
use std::ptr;
use std::arch::asm;
use std::mem::transmute;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    file: String
}

fn main() {
    let args = Args::parse();
    
    let filename = args.file;

    println!("SHELLCODE DBG Ver0.1");
    println!("[*] Processing shellcode file {} ....", filename);

    let mut scfile = File::open(filename).unwrap();
    let mut buf = Vec::new();
    let _ = scfile.read_to_end(&mut buf).unwrap();
    let size = scfile.metadata().unwrap().len() as usize;
    
    let mut code: *mut c_void = ptr::null_mut();
    unsafe {
        code = VirtualAlloc(None, size, Memory::VIRTUAL_ALLOCATION_TYPE(0x3000), Memory::PAGE_PROTECTION_FLAGS(0x40));
    }

    println!("[*] Alloocated!");
    println!("[*] Base Address: 0x{:x}", code.addr());

    println!("[*] Copying shellcode ...");
    unsafe {
        let src = buf.as_ptr() as *mut c_void;
        copy_nonoverlapping(src, code, size);
    }

    println!("[*] Unleash shellcode!");
    unsafe {
        let run = transmute::<*mut c_void, fn()>(code);
        run();
    }
}
