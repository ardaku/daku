use alloc::vec::Vec;
use core::mem;

use crate::{
    sys::{self, Command, Connect},
    tls::Local,
};

// 1kB list of up to 256 ready channels
#[allow(unused)] // For when no portals are enabled via feature flags
const READY_LIST_CAPACITY: usize = 256;

const PORTAL_LOG: usize = 0;
const PORTAL_PROMPT: usize = PORTAL_LOG + cfg!(feature = "log") as usize;
const PORTAL_COUNT: usize = PORTAL_PROMPT + cfg!(feature = "prompt") as usize;

static PORTALS: Local<[u32; PORTAL_COUNT]> = Local::new([0; PORTAL_COUNT]);

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

        PORTALS.with(|p| {
            #[cfg(feature = "log")]
            {
                p[PORTAL_LOG] = sys::Portal::Log as u32;
            }
            #[cfg(feature = "prompt")]
            {
                p[PORTAL_PROMPT] = sys::Portal::Prompt as u32;
            }

            let connect = &Connect {
                ready_capacity: READY_LIST_CAPACITY,
                ready_data: ready_list.as_mut_ptr(),
                portals_size: p.len(),
                portals_data: p.as_mut_ptr(),
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
        });

        *state = Some(ready_list);
    });
}

/// Get the log channel
#[cfg(feature = "log")]
#[inline(always)]
pub(crate) fn log() -> u32 {
    init();
    PORTALS.with(|portal| portal[PORTAL_LOG])
}

/// Get the prompt channel
#[cfg(feature = "prompt")]
#[inline(always)]
pub(crate) fn prompt() -> u32 {
    init();
    PORTALS.with(|portal| portal[PORTAL_PROMPT])
}
