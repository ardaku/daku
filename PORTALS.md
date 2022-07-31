# Portals List
Portals are consecutively numbered, and can be used to open channels.  Any
number of channels can be opened on the same portal.  Each portal matches an
app permission.

 0. [Spawn tasks](portals/0.md)
 1. [Wake tasks](portals/1.md)
 2. [Write logs](portals/2.md)



 0. [CPU Info](portals/0.md)
 1. [CPU Settings](portals/1.md)
 2. [Environment Info](portals/2.md)
 3. [Environment Settings](portals/3.md)
 5. [Read serial commands](portals/5.md)
 6. [Spawn sandboxed child tasks](portals/6.md)
 7. [Install apps](portals/7.md)
 8. [Persistent File Storage](portals/8.md)
 9. [Removable Drive Raw Access](portals/9.md)
 10. [HTTPS Server](portals/10.md)
 11. [HTTPS Client](portals/11.md)
 12. [Bluetooth Server](portals/12.md)
 13. [Bluetooth Client](portals/13.md)
 14. [WebRTC Server](portals/14.md)
 15. [WebRTC Client](portals/15.md)
 16. [Set GUI "headerbar" / "hud"](portals/16.md)
 17. [Get GUI events](portals/17.md)
 18. [Send/Receive HID Events (Rumble/Input)](portals/18.md)
 19. [Get Environment Status (Battery/Signal Strength)](portals/19.md)
 20. [Camera](portals/20.md)
 21. [Framebuffer](portals/21.md)
 22. [Graphics Acceleration Wgpu](portals/22.md)
 23. [Neural Network Acceleration](portals/23.md)
 24. [Microphone](portals/24.md)
 25. [Speaker](portals/25.md)
 26. [GPIO](portals/26.md)
 27. [Serial Bus (USB/I2C/etc)](portals/27.md)
 28. [Spawn Background Tasks](portals/28.md)
 29. [Send Notifications](portals/29.md)
 30. [Handle (Send/Receive) Phone Calls](portals/30.md)
 31. [Handle (Send/Receive) Texts](portals/31.md)
 32. [Get GPS Location](portals/32.md)
 33. [Light Level](portals/33.md)
 34. [Get Acceleration](portals/34.md)
 35. [Get Orientation](portals/35.md)
 36. [Get Timezone](portals/36.md)
 37. [Set Timezone](portals/37.md)
 38. [Get Current Time](portals/38.md)
 39. [Set Interval Timer](portals/39.md)

## Services
Services are custom portals.  Unlike portals, they are not I/O bound and can use
only a handful of portals.  Services can be shared between apps, so they are
useful for large functionality shared across many apps, such as multimedia
encoding/decoding.  Services start at 2^32 and can go up to 2^64.

Allowed Portals:
 - (22) [Graphics Acceleration Wgpu](portals/22.md)
 - (23) [Neural Network Acceleration](portals/23.md)
