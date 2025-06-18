# bs-socket

A Rust-based implementation meant for interpreting events from Beat Saber websockets.

Has support for [WentTheFox's BSDataPuller](https://github.com/WentTheFox/BSDataPuller), 
[HardCPP's BeatSaberPlus-SongOverlay](https://github.com/hardcpp/BeatSaberPlus/), 
and [denpadokei's HTTPSiraStatus](https://github.com/denpadokei/HttpSiraStatus/).

There are too many websocket client libraries to count, so this library plans to let the developer use whatever websocket framework they want. It does include configurations for each socket mod though, so developers can focus on getting the websocket libraries to work!