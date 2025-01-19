# scranner

# DESCRIPTION

A really ugly hobby packet sniffer which is still a WIP, and mostly a play around with Rust, pnet and Rust flakes. 
At the time of writing the nic name is hard coded into main.rs


# SYNOPSIS: Building and Running

To build and run the nix binary pkg:


## NixOS

Swap out eth0 to your nic
```
sudo -E nix run -- eth0
```

## Dev Env

```
nix develop

```

To test as root (required for sniffing):
```
sudo -E nix develop --command cargo run -- <nic_name eg. eth0>
```

## Other Nix

You should really be able to `sudo cargo run`, and I'd imagine `setcap cap_net_raw|cap_net_admin+eip./target/debug/scranner`
This caused a few issues in nixos due to flake environmentals not being propagated.


