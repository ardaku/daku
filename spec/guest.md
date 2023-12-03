# Guest Exports

 - Export 32-bit memory as `memory`
 - Export "main" function as `run`
 - Export Daku global pointer as `16` for a size 16 ready list

## *Type*: `Daku[N]`

Commands are the way the Daku application sends messages to the environment.

### Fields

 - `command: [opt[T]; N]` Ready list of commands
