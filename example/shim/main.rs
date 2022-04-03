// Shim for providing async main

#[allow(unused_imports)]
use main::*;

mod main {
    include!("../src/main.rs");

    pub(super) mod main {
        pub(in crate) async fn main() {
            super::main().await
        }
    }
}

fn main() {
    daku::block_on(self::main::main::main());
}
