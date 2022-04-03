use daku::log::{self, Target};

async fn main() {
    let info = Target::new("Info").await;

    log::info!(info, "Hello, world!").await;
}
