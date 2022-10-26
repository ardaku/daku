use alloc::vec::Vec;

use crate::sys::{self, Command, Connect, Portal};

#[cfg(feature = "log")]
static mut LOG: core::mem::MaybeUninit<u32> = core::mem::MaybeUninit::uninit();

#[cfg(feature = "prompt")]
static mut PROMPT: core::mem::MaybeUninit<u32> =
    core::mem::MaybeUninit::uninit();

// 1kB list of up to 256 ready channels
const READY_LIST_CAPACITY: usize = 256;

static mut READY_LIST: Vec<usize> = Vec::new();

static mut INIT: bool = false;

pub(crate) unsafe fn ready_list(size: usize) -> &'static [usize] {
    READY_LIST.set_len(size);
    READY_LIST.as_slice()
}

async fn init() {
    if unsafe { INIT } {
        return;
    };

    unsafe {
        INIT = true;
        READY_LIST.reserve_exact(READY_LIST_CAPACITY);
    }

    unsafe {
        let ptr = READY_LIST.as_ptr();
        let len = READY_LIST.len();
        let cap = READY_LIST.capacity();
        let text = alloc::format!("Ready List: {ptr:p}, {len}/{cap}");
        sys::dbg(text.len(), text.as_ptr());
    }

    let mut portals: Vec<u32> = Vec::new();

    if cfg!(feature = "log") {
        portals.push(Portal::Log as u32);
    }
    if cfg!(feature = "prompt") {
        portals.push(Portal::Prompt as u32);
    }

    let connect = &Connect {
        ready_capacity: READY_LIST_CAPACITY,
        ready_data: unsafe { READY_LIST.as_mut_ptr() },
        portals_size: portals.len(),
        portals_data: portals.as_mut_ptr(),
    };
    let connect: *const _ = connect;

    let commands = [Command {
        ready: usize::MAX,
        channel: 0,
        size: core::mem::size_of::<Connect>(),
        data: connect.cast(),
    }];

    unsafe {
        sys::ar(commands.len(), commands.as_ptr());
    }

    #[cfg(feature = "prompt")]
    unsafe {
        PROMPT = core::mem::MaybeUninit::new(portals.pop().unwrap())
    };
    #[cfg(feature = "log")]
    unsafe {
        LOG = core::mem::MaybeUninit::new(portals.pop().unwrap())
    };
}

/// Get the log channel
#[cfg(feature = "log")]
pub(crate) async fn log() -> u32 {
    unsafe {
        init().await;
        *LOG.assume_init_ref()
    }
}

/// Get the prompt channel
#[cfg(feature = "prompt")]
pub(crate) async fn prompt() -> u32 {
    unsafe {
        init().await;
        *PROMPT.assume_init_ref()
    }
}
