use std::fmt;
use std::error::Error as StdError;
use nix::unistd::Pid;

use core::ffi::c_void;

macro_rules! from {
    ($f:ty, $t:ty, $r:expr) => {
        impl From<$f> for $t {
            fn from(e: $f) -> Self {
                $r(e)
            }
        }        
    }
}

#[derive(Debug)]
pub enum AllError { 
    Attacher(AttachError),
    InsertBreaker(InsertBreakError),
    Conter(ContError),
    GetInfoer(GetInfoError),
    Detacher(DetachError),
} 

from!(AttachError, AllError, AllError::Attacher);
from!(InsertBreakError, AllError, AllError::InsertBreaker);
from!(ContError, AllError, AllError::Conter);
from!(GetInfoError, AllError, AllError::GetInfoer);
from!(DetachError, AllError, AllError::Detacher);

impl fmt::Display for AllError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "running error.")
    }
}
impl StdError for AllError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            AllError::Attacher(err) => Some(err),
            AllError::InsertBreaker(err) => Some(err),
            AllError::Conter(err) => Some(err),
            AllError::GetInfoer(err) => Some(err),
            AllError::Detacher(err) => Some(err),
        }
    }
}


#[derive(Debug)]
pub enum AttachError { //used DebugService::attach func.
    AttachError(RawAttachError),
}

from!(RawAttachError, AttachError, AttachError::AttachError);

impl fmt::Display for AttachError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AttachError::AttachError(_) => {
                write!(f, "Failed to attach.")
            },
        }
    }
}
impl StdError for AttachError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            AttachError::AttachError(err) => Some(err),
        }
    }
}

#[derive(Debug)]
pub enum RawAttachError {
    RawAttachError(i32),
}
impl RawAttachError {
    pub fn fail_raw_attach(pid: i32) -> Self {
        RawAttachError::RawAttachError(pid)
    }
}
impl fmt::Display for RawAttachError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RawAttachError::RawAttachError(pid) => {
                write!(f, "pid is {}", pid)
            },
        }
    }
}
impl StdError for RawAttachError {}



#[derive(Debug)]
pub enum InsertBreakError { //used DebugService::insert_break func.
    RWError(RawRWMemoryError),
}
from!(RawRWMemoryError, InsertBreakError, InsertBreakError::RWError);
impl fmt::Display for InsertBreakError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InsertBreakError::RWError(_) => {
                write!(f, "Failed to insert break point.")
            },
        }
    }
}
impl StdError for InsertBreakError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            InsertBreakError::RWError(e) => Some(e),
        }
    }
}

#[derive(Debug)]
pub enum RawRWMemoryError {
    WriteMemoryError(*mut c_void, *mut c_void),
    ReadMemoryError(*mut c_void),
}
impl RawRWMemoryError {
    pub fn write_error(addr: *mut c_void, value: *mut c_void) -> Self {
        RawRWMemoryError::WriteMemoryError(addr, value)
    }
    pub fn read_error(addr: *mut c_void) -> Self {
        RawRWMemoryError::ReadMemoryError(addr)
    }
}
impl fmt::Display for RawRWMemoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RawRWMemoryError::WriteMemoryError(addr, value) => {
                write!(f, "at:{:?}, written_value:{:?}", addr, value)
            },
            RawRWMemoryError::ReadMemoryError(addr) => {
                write!(f, "at:{:?}", addr)
            },
        }
    }
}
impl StdError for RawRWMemoryError {}



#[derive(Debug)]
pub enum ContError { // used DebugService::cont func.
    ContError(RawContError),
}
from!(RawContError, ContError, ContError::ContError);
impl fmt::Display for ContError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContError::ContError(_) => {
                write!(f, "Failed to cont.")
            },
        }
    }
}
impl StdError for ContError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            ContError::ContError(err) => Some(err),
        }
    }
}

#[derive(Debug)]
pub enum RawContError {
    RawContError(Pid),
} 
impl RawContError {
    pub fn fail_raw_cont(pid: Pid) -> Self {
        RawContError::RawContError(pid)
    }
}
impl fmt::Display for RawContError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RawContError::RawContError(pid) => {
                write!(f, "pid is {}", pid)
            },
        }
    }
}
impl StdError for RawContError {}


#[derive(Debug)]
pub enum GetInfoError { //used DebugService::get_info func.
    GetInfoError(RawGetInfoError),
}
from!(RawGetInfoError, GetInfoError, GetInfoError::GetInfoError);
impl fmt::Display for GetInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GetInfoError::GetInfoError(_) => {
                write!(f, "Failed to get information.")
            },
        }
    }
}
impl StdError for GetInfoError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            GetInfoError::GetInfoError(err) => Some(err),
        }
    }
}
#[derive(Debug)]
pub enum RawGetInfoError {
    GetRegsError,
    ReadStackMemoryError(*mut c_void),
    ReadTextMemoryError(*mut c_void),
}
impl RawGetInfoError {
    pub fn fail_get_regs() -> Self {
        RawGetInfoError::GetRegsError
    }
    pub fn fail_read_stack(addr: *mut c_void) -> Self {
        RawGetInfoError::ReadStackMemoryError(addr)
    }
    pub fn fail_read_text(addr: *mut c_void) -> Self {
        RawGetInfoError::ReadTextMemoryError(addr)
    }
}
impl fmt::Display for RawGetInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RawGetInfoError::GetRegsError => {
                write!(f, "failed to get regs")
            },
            RawGetInfoError::ReadStackMemoryError(addr) => {
                write!(f, "failed to read stack memory:{:?}", addr)
            },
            RawGetInfoError::ReadTextMemoryError(addr) => {
                write!(f, "failed to get text memory:{:?}", addr)
            },
        }
    }
}
impl StdError for RawGetInfoError {}


#[derive(Debug)]
pub enum DetachError { //used DebugService::detach func.
    DetachError(RawDetachError),
}
from!(RawDetachError, DetachError, DetachError::DetachError);
impl fmt::Display for DetachError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DetachError::DetachError(_) => {
                write!(f, "Failed to detach.")
            },
        }
    }
}
impl StdError for DetachError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            DetachError::DetachError(err) => Some(err),
        }
    }
}
#[derive(Debug)]
pub enum RawDetachError {
    RawDetachError(Pid),
}
impl RawDetachError {
    pub fn fail_detach(pid: Pid) -> Self {
        RawDetachError::RawDetachError(pid)
    }
}
impl fmt::Display for RawDetachError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RawDetachError::RawDetachError(pid) => {
                write!(f, "pid: {}", pid)
            },
        }
    }
}
impl StdError for RawDetachError {}

