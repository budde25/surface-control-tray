# Surface Control Tray

Tray program for controlling various aspects of the Mircrosoft Surface Book 2 from the tray.  
Thanks to the incredible work https://github.com/linux-surface/surface-control for the surface control command line tool. This tray would not be possible without it, as most of the code for actually controlling the device is from the CLI, I simply put a GUI over it.  
Note: As I only have a surface book 2 that is the only device that I have been able to test it on, Also detach funtionality is not working at the moment.

### TODO
Add an icon.  
detach functionality.  

## Insallation 
To install you can either grab a precompiled binary from releases or build it yourself.  
To build it yourself clone the repository and run `cargo build --release`, or `cargo install`. make sure to add it to the PATH.  
Then copy the Sysetmd Service file from the repository or below and add it to  
`~/.config/systemd/user/surface-control-tray.service`  
Edit the ExecStart line to pointer to where your binary is install. (you can use `which surface-control-tray`) assuming its in your path.  
Now just run `systemctl --user enable --now surface-control-tray` This will enable it to run at startup.

```
[Unit]
Description=Surface Control Tray

[Service]
Type=simple
ExecStart=<BIN PATH GOES HERE>
Restart=on-failure

[Install]
WantedBy=multi-user.target
```
