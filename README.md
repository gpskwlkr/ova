<h1 align="center" style="color: red">
  (O)TP (VA)ULT - ova. <img alt="clipboard" src="readme_items/clipboard.svg" height="35" width="35" align="center"/>
</h1>

> <code>ova</code> is a simple CLI tool which lets you manage your TOTPs, or basically lets you get your two-way authentication code straight to your clipboard.

<div align="center">
  <a href="https://github.com/gpskwlkr/ova">
     <img alt="github" src="https://img.shields.io/badge/github-gpskwlkr/ova-50C878?style=for-the-badge&labelColor=555555&logo=github">
  </a>
</div>

<div align="center">
  <a href="https://github.com/gpskwlkr/ova/actions/workflows/ci.yml" style="text-decoration: none;">
    <img alt="build status" src="https://img.shields.io/github/actions/workflow/status/clap-rs/clap/ci.yml?branch=master&style=for-the-badge">
  </a>
  <a href="LICENSE" style="text-decoration: none;">
    <img alt="license" src="https://img.shields.io/badge/license-MIT-blue?style=for-the-badge">
  </a>
</div>

<div align="center">
  <a href="https://crates.io/crates/ova" style="text-decoration: none;">
    <img alt="" src="https://img.shields.io/crates/v/ova?style=for-the-badge">
  </a>
  
  <a href="https://crates.io/crates/ova" style="text-decoration: none;">
    <img alt="" src="https://img.shields.io/crates/d/ova?style=for-the-badge">
  </a>
  
  <a href="https://coveralls.io/github/gpskwlkr/ova?branch=master" style="text-decoration: none;">
    <img alt="" src="https://img.shields.io/coverallsCoverage/github/gpskwlkr/ova.svg?branch=master&style=for-the-badge">
  </a>
</div>


- Project is in active development state, if something's not working the way it should work, you're more than welcome to open an [issue](https://github.com/gpskwlkr/ova/issues).

# Available commands

| Command | Options | Description |
| --- | --- | --- |
| ova get | -n, --name `<name of a key>`, -c, --copy `<true/false>` | Get 2FA code for specified key, copy to clipboard determined by -c flag |
| ova add | -n, --name `<name of a key>`, -k, --key `<2FA secret>` | Store your key locally at `~/.config/ova.store` or `'%USERPROFILE%\AppData\Local'` |
| ova remove | -n, --name `<name of a key>` | Remove key from local store file |
| ova update | -n, --name `<name of a key>`, -k, --key `<new secret value>` | Update key if it exists |
| ova list | no options | List all stored keys and their secrets |
| ova help | no options | Provide description for all the commands |
  
Note that `ova help` could be executed per subcommand as well, like `ova add help` which provides a description for chosen subcommand.

# Dependencies

`ova` depends on `xclip` or `wl-copy` to use system-wide clipboard.

- Linux X11 - `xclip` should be installed
- Linux Wayland - `wl-copy` should be installed

Windows & Mac OS does not require any additional setup as they use `powershell` :persevere: and `osascript` respectively.

# Install

You can install `ova` via

`cargo install ova`

Or use precompiled binaries provided as releases.

# Currently tested with

- [x] Bitwarden
- [x] Facebook
- [ ] Twitter

# What's to do

- [x] Storing keys in a local file :tada:
- [x] Fully working `get`, `add`, `list` commands :tada:
- [x] Fully working `remove` and `update` commands :tada:
- [x] Copy to clipboard working on Windows, Mac OS and Linux
- [ ] Pretty print
- [ ] Rewrite windows copy method to winapi and throw away powershell :persevere:
- [ ] More to come...

# Credits

Icon by [BomSymbols](https://thenounproject.com/korawan_m/)
