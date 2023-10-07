# Activate Xbox Controller on Linux

This is a simple program that activate xbox controller when it is plugged into a Linux device.

It is rust version of the python script below:

```python
#!/usr/bin/env python3

import usb.core

dev = usb.core.find(idVendor=0x045e, idProduct=0x028e)

if dev is None:
    raise ValueError('Device not found')
else:
    dev.ctrl_transfer(0xc1, 0x01, 0x0100, 0x00, 0x14) 
```

Why rust? Because I don't want to mess up with my system. It doesn't make much sense if you need to install python, then libusb just to do a simple thing.

With this rust version, you can compile once (using a docker container or a VM), then run without any dependency.

Or even easier, just download the pre-built binary in **Release** section of this repo.

To run it:

```bash
./activate-xbox-controller-linux
```

## Download pre-built version

Please go to: [Release](https://github.com/ngxson/activate-xbox-controller-linux/releases)
