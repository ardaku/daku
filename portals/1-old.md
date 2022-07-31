# 1. CPU Settings

## Commands
 0. Get overclocking setting `send: (), recv: Overclock`
    ```rust
    #[repr(C, packed)]
    enum Overclock {
        Off = 0,
    }
    ```
 1. Set overclocking setting `send: Overclock, recv: ()`
