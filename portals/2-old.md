# 2. Environment Info 

## Commands
 0. Get User:Username `send: (), recv: Text`
 1. Get User:FullName `send: (), recv: Text`
 2. Get User:Language `send: (), recv: Lang`
 3. Get OS Name `send: (), recv: Text`
 4. Get Kernel Type `send: (), recv: Platform`
    ```rust
    #[repr(C, u32)]
    enum Platform {
        Linux = 0,
        Bsd = 1,
        Windows = 2,
        MacOS = 3,
        Ios = 4,
        Android = 5,
        Nintendo = 6,
        Xbox = 7,
        PlayStation = 8,
        Fuchsia = 9,
        Redox = 10,
        Novusk = 11,
        Unknown = u32::MAX,
    }
    ```
 5. Get (Desktop) Environment `send: (), recv: Environment`
    ```rust
    #[repr(C, u32)]
    enum Environment {
        Gnome = 0,
        Windows = 1,
        Lxde = 2,
        Openbox = 3,
        Mate = 4,
        Xfce = 5,
        Kde = 6,
        Cinnamon = 7,
        I3 = 8,
        Aqua = 9,
        Ios = 10,
        Android = 11,
        WebBrowser = 12,
        Console = 13,
        Ubuntu = 14,
        Ermine = 15,
        Orbital = 16,
       _quantii_env = 17,
        Unknown = u32::MAX,
    }
    ```
 6. Get Wasm Runtime `send: (), recv: Runtime`
    ```rust
    #[repr(C, u32)]
    enum Runtime {
        Wasmtime = 0,
        Wasmer = 1,
        Wasmi = 2,
        Wari = 3,
        Unknown = u32::MAX,
    }
    ```
