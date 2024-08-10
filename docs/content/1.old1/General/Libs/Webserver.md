# Webserver

Is there a std::net for the arm target we're working with?

  * https://doc.rust-lang.org/std/net/index.html

which web server to use?

  * https://www.arewewebyet.org/topics/frameworks/#pkg-actix-web
  * https://github.com/actix/actix-web
  * https://github.com/gotham-rs/gotham
  * https://rocket.rs/

## Links

  * https://www.youtube.com/watch?v=Fz084MUqR7g
  * https://github.com/smoltcp-rs/smoltcp
  * https://github.com/rust-embedded-community/embedded-nal
  * https://github.com/ninjasource/led-display-websocket-demo - websockets
  * https://github.com/bzf/rust-embedded-web-app/tree/master/src

rota appears to use actix_web which is a rust http library
but I think this is desgined to serve the ESP32 not run on it

  * https://github.com/eolder/rota
  * https://github.com/actix/actix-web

This seems the most promising

  * https://github.com/ctron/rust-esp32-hono
