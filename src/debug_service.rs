use crate::process::Process;
use crate::process_info::ProcessInfo;
use crate::error;

use core::ffi::c_void;
use std::collections::HashMap;

pub struct DebugService<P: Process> {
    process: P,
    breakpoint_to_value: HashMap<*mut c_void, *mut c_void>,
}

impl<P> DebugService<P> 
where P: Process
{
    pub fn new_and_attach(file_name: &str, args:Option<Vec<String>>) -> Result<Self, error::AttachError> {
        let child = P::new_and_attach(file_name, args)?;
        Ok(DebugService {
            process: child,
            breakpoint_to_value: HashMap::new(),
        })
    }

    pub fn attach(pid: i32) -> Result<Self, error::AttachError> {
        let target_process = P::attach(pid)?;
        Ok(DebugService {
            process: target_process,
            breakpoint_to_value: HashMap::new(),
        })
    }

    pub fn insert_break(&mut self, addr: *mut c_void) -> Result <(), error::InsertBreakError> {
        let value = self.process.rewrite_memory(addr, 0xcc)?;
        self.breakpoint_to_value.insert(addr, value);
        Ok(())
    }

    pub fn cont(&self) -> Result<(), error::ContError>{
        self.process.cont()?;
        Ok(())
    }

    pub fn get_info(&self) -> Result<ProcessInfo, error::GetInfoError> {
        let info = self.process.get_info()?;
        Ok(info)
    }

    pub fn detach(&self) -> Result<(), error::DetachError> {
        self.process.detach()?;
        Ok(())
    }
}
