# 0. CPU Info

## Commands
 0. Get CPU Architecture `send: (), recv: u32`
    ```rust
    #[repr(C, u32)]
    enum Arch {
        Wasm = 0,
        RiscV = 1,
        Arm = 2,
        Mips = 3,
        X86 = 4,
    }
    ```
 1. Get CPU Reference Size `send: (), recv: u32`
    ```rust
    #[repr(C, u32)]
    enum RefSize {
        Ref16 = 0,
        Ref32 = 1,
        Ref64 = 2,
        Ref128 = 3,
    }
    ```
 2. Query CPU Extensions description `send: (), recv: (Text, ...)`.
    Results should never be relied on for app features.
