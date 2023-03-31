# Simple background app that modifies the config file of windows terminal to change your background.

### Goals
Daemon runs in the back 24/7(when the machine is running) and client can connect to interact with it (modify settings, check health)

- [x] Client - Daemon comunicaton (used std::net::TcpStream and the Daemon has a std::net::TcpListener)
- [x] Daemon can get and set the WT background
- [x] Daemon logs and backup file for failsafe when setting new bg
- [x] Robust error handling.
- [x] Daemon logs it's actions
- [x] Simple client
- [x] Daemon responds to simple commands
- [x] Daemon fully independent


This could still be improved, & a real client could be made.
This version is not perfect (history & logging in general could have been better) but it's ok for now.

Feel free to use / modify it as you want.
Im open to suggestions for features to add.