# mcprocess

A tool to manage minecraft servers on unix systems.
Built for people who want more utility than `screen`, but don't want to deploy a full-blown solution with a web interface like multicraft or pterodactyl.
This tool does not need any other software to run, no database, no redis, it manages it's own data.

## Features roadmap

- CLI
  - [x] Import existing server
  - [ ] Create new server
  - [ ] Start/stop servers
  - [ ] Hook into console
- General
  - [ ] Backups
  - [ ] JAR management, e.g. update paper
  - [ ] Plugin management
  - [ ] Automatic system firewall configuration
    - [ ] Also automatic bungeeguard configuration
  - [ ] Crash detection
- Daemon
  - [x] Automatically start servers on boot
  - [ ] Automatically restart servers on crash
