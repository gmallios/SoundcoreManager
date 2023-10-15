# Important classes

- A3951CmdConstants ( Has all opcodes )
- C7577b

# ANC byte arrays from frida

```
Normal Mode                             - 8,-18,0,0,0,6,-127,14,0,2,0,0,6,-109

Transparency Mode, Fully Transparent    - 8,-18,0,0,0,6,-127,14,0,1,0,0,6,-110
Transparency Mode, Vocal Mode           - 8,-18,0,0,0,6,-127,14,0,1,0,1,6,-109

Noise Cancellation, Transport           - 8,-18,0,0,0,6,-127,14,0,0,0,1,6,-110
Noise Cancellation, Outdoor             - 8,-18,0,0,0,6,-127,14,0,0,1,1,6,-109
Noise Cancellation, Indoor              - 8,-18,0,0,0,6,-127,14,0,0,2,1,6,-108
Noise Cancellation, Custom(Max)         - 8,-18,0,0,0,6,-127,14,0,0,3,1,10,-103
Noise Cancellation, Custom(Min)         - 8,-18,0,0,0,6,-127,14,0,0,3,1,0,-113
```

# On Press MenuItem from frida

```
Sends
    8,-18,0,0,0,1,1,10,0,2
    8,-18,0,0,0,1,5,10,0,6
```

# TODO
- A3951DeviceManager Missing devices
- A3933DeviceManager