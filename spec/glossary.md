# Glossary

### Guest
A program that interfaces with the Daku API.

### Host
The program that embeds the guest.

### Command
A command is a data structure that is sent from the guest to the host.
All commands execute once and complete asynchronously.  Any command may be
canceled.

### Channel
Channels receieve commands from the guest.  For the guest to receive data, the
guest must send a memory address of a buffer to the host; The buffer then gets
overwritten upon completion of the command.

### Portal
Portals are channels that get opened before execution of the guest WebAssembly
module begins.  A custom command portal is always opened at channel 0.  Other
portals can be opened using the portal extension, which provides
system-interface APIs similar to what WASI does.
