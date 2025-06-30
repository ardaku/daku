# Command
 
## *Type*: `Command`

Command sent from the guest to the host

### Fields

 - `channel: int` (in) Which channel is being used, (out) number of channels
   opened
 - `capacity: int` Capacity of `buffer`
 - `buffer: Buffer` Data buffer
