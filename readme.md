# Simple background app that modifies the config file of windows terminal to change your background.

### Goals
Daemon runs in the back 24/7(when the machine is running) and client can connect to interact with it (modify settings, check health)

- [x] Client - Daemon comunicaton (used std::net::TcpStream and the Daemon has a std::net::TcpListener)
- [x] Daemon can get and set the WT background
- [x] Daemon logs and backup file for failsafe when setting new bg
- [x] Robust error handling.
- [ ] Daemon logs it's actions
- [ ] Cool client