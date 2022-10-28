use alloc::vec::Vec;
use core::mem;

use crate::{
    sys::{self, Command, Connect, Portal},
    tls::Local,
};

// 1kB list of up to 256 ready channels
#[allow(unused)] // For when no portals are enabled via feature flags
const READY_LIST_CAPACITY: usize = 256;

#[cfg(feature = "log")]
static LOG: Local<mem::MaybeUninit<u32>> =
    Local::new(mem::MaybeUninit::uninit());

#[cfg(feature = "prompt")]
static PROMPT: Local<mem::MaybeUninit<u32>> =
    Local::new(mem::MaybeUninit::uninit());

static READY_LIST: Local<Option<Vec<usize>>> = Local::new(None);

#[inline(always)]
pub(crate) unsafe fn ready_list<R>(
    size: usize,
    f: impl FnOnce(&[usize]) -> R,
) -> R {
    READY_LIST.with(|ready_list| {
        let ready_list = ready_list.as_mut().unwrap_unchecked();

        ready_list.set_len(size);
        f(ready_list.as_slice())
    })
}

#[inline(never)]
#[allow(dead_code)] // For when no portals are enabled via feature flags
fn init() {
    READY_LIST.with(|state| {
        if state.is_some() {
            return;
        };

        let mut ready_list = Vec::with_capacity(READY_LIST_CAPACITY);
        let mut portals: Vec<u32> = Vec::new();

        if cfg!(feature = "log") {
            portals.push(Portal::Log as u32);
        }
        if cfg!(feature = "prompt") {
            portals.push(Portal::Prompt as u32);
        }

        let connect = &Connect {
            ready_capacity: READY_LIST_CAPACITY,
            ready_data: ready_list.as_mut_ptr(),
            portals_size: portals.len(),
            portals_data: portals.as_mut_ptr(),
        };
        let connect: *const _ = connect;

        let commands = [Command {
            ready: usize::MAX,
            channel: 0,
            size: mem::size_of::<Connect>(),
            data: connect.cast(),
        }];

        unsafe {
            sys::ar(commands.len(), commands.as_ptr());
        }

        *state = Some(ready_list);

        #[cfg(feature = "prompt")]
        PROMPT.with(|prompt| {
            *prompt = mem::MaybeUninit::new(portals.pop().unwrap())
        });
        #[cfg(feature = "log")]
        LOG.with(|log| *log = mem::MaybeUninit::new(portals.pop().unwrap()));
    });
}

/// Get the log channel
#[cfg(feature = "log")]
#[inline(always)]
pub(crate) fn log() -> u32 {
    init();
    LOG.with(|log| unsafe { log.assume_init() })
}

/// Get the prompt channel
#[cfg(feature = "prompt")]
#[inline(always)]
pub(crate) fn prompt() -> u32 {
    init();
    PROMPT.with(|prompt| unsafe { prompt.assume_init() })
}
