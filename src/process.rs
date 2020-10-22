
use core::ffi::c_void;
use std::process::{Command, Child};
use nix::unistd::Pid;
use nix::sys::ptrace;
use nix::sys::wait::waitpid;

use crate::process_info::{ProcessInfo, STACKLEN, TEXTLEN};
use crate::error;

pub trait Process {
    fn new_and_attach(file_name: &str, args:Option<Vec<String>>) -> Result<Self, error::RawAttachError> where Self: Sized;
    fn attach(pid: i32) -> Result<Self, error::RawAttachError> where Self: Sized;
    fn rewrite_memory(&mut self, addr: *mut c_void, value: u64) -> Result<*mut c_void, error::RawRWMemoryError>;
    fn cont(&self) -> Result<(), error::RawContError>;
    fn get_info(&self) -> Result<ProcessInfo, error::RawGetInfoError>;
    fn detach(&self) -> Result<(), error::RawDetachError>;
}

pub struct UnixProcess {
    process: Option<Child>,
    pid: nix::unistd::Pid,
}

impl Process for UnixProcess {
    fn new_and_attach(file_name: &str, args:Option<Vec<String>>) -> Result<Self, error::RawAttachError> {
        let mut p = Command::new(file_name); 
        match args {
            Some(all) => {
                let proc = p.args(all).spawn().unwrap();
                let proc_id = Pid::from_raw(proc.id() as i32);
                match ptrace::attach(proc_id) {
                    Ok(_) => Ok(UnixProcess{
                        process: Some(proc),
                        pid: proc_id,
                    }),
                    Err(_) => Err(error::RawAttachError::fail_raw_attach(proc.id() as i32)),
                }
            },
            None => {
                let proc = p.spawn().unwrap();
                let proc_id = Pid::from_raw(proc.id() as i32);
                match ptrace::attach(proc_id) {
                    Ok(_) => Ok(UnixProcess{
                        process: Some(proc),
                        pid: proc_id,
                    }),
                    Err(_) => Err(error::RawAttachError::fail_raw_attach(proc.id() as i32)),
                }
            }, 
        }
    }

    fn attach(pid: i32) -> Result<Self, error::RawAttachError> {
        let t_pid = Pid::from_raw(pid);
        match ptrace::attach(t_pid) {
            Ok(_) => Ok(UnixProcess{process: None, pid: t_pid}),
            Err(_) => Err(error::RawAttachError::fail_raw_attach(pid)),
        }
    }

    fn rewrite_memory(&mut self, addr: *mut c_void, value: u64) -> Result<*mut c_void, error::RawRWMemoryError>{
        let original_text = ptrace::read(self.pid, addr);
        let t = match original_text {
            Ok(p) => p,
            Err(_) => return Err(error::RawRWMemoryError::read_error(addr)),
        };
        let f = (t as u64 & 0xffffffffffffff00) | value;
        let r = f as *mut c_void;
        match ptrace::write(self.pid, addr, r) {
            Ok(_) => {},
            Err(_) => return Err(error::RawRWMemoryError::write_error(addr, r)),
        }
        Ok(t as *mut c_void)
    }

    fn cont(&self) -> Result<(), error::RawContError>{
        match ptrace::cont(self.pid, None) {
            Ok(_) => {},
            Err(_) => return Err(error::RawContError::fail_raw_cont(self.pid)),
        }
        waitpid(self.pid, None);
        Ok(())
    }

    fn detach(&self) -> Result<(), error::RawDetachError>{
        match ptrace::detach(self.pid, None) {
            Ok(_) => Ok(()),
            Err(_) => Err(error::RawDetachError::fail_detach(self.pid)),
        }
    }

    fn get_info(&self) -> Result<ProcessInfo, error::RawGetInfoError> {
        let regs = ptrace::getregs(self.pid);
        match regs {
            Err(_) => return Err(error::RawGetInfoError::fail_get_regs()),
            _ => {},
        }
        let regs = regs.unwrap();
        let init = 0 as *mut c_void;
        let mut stack: [(*mut c_void, *mut c_void); STACKLEN] = [(init, init); STACKLEN];
        let mut text: [(*mut c_void, *mut c_void); TEXTLEN] = [(init, init); TEXTLEN];
        let mut rsp = regs.rsp;
        for number in 0..STACKLEN {
            let value = ptrace::read(self.pid, rsp as *mut c_void);
            match value {
                Err(_) => return Err(error::RawGetInfoError::fail_read_stack(rsp as *mut c_void)),
                _ => {}
            }
            stack[number] = (rsp as *mut c_void, value.unwrap() as *mut c_void);
            rsp += 8;
        }
        let mut rip = regs.rip;
        for number in 0..TEXTLEN {
            let value = ptrace::read(self.pid, rip as *mut c_void);
            match value {
                Err(_) => return Err(error::RawGetInfoError::fail_read_text(rip as *mut c_void)),
                _ => {},
            }
            text[number] = (rip as *mut c_void, value.unwrap() as *mut c_void);
            rip += 8;
        }
        Ok(ProcessInfo::new(regs, stack, text))
    }
}
