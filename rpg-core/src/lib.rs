/* TODO:

update Makefile for mobile and web

tests (config, rpc, audio, lights)

campaign
game session objects
player
character

workspace:
unify cli and server

module | local (in-code) | active (listening) | passive (client) | logging
--- | --- | --- | --- | ---
cli | ✓ | TODO | ✓ | ✓
dioxus-desktop | ✓ | TODO | ✓ | ✓
dioxus-web | TODO | TODO | TODO | TODO
server | ✓ | ✓ | ✓ | ✓

audio - chromecast (spotify app ID: CC32E753 - https://github.com/azasypkin/rust-caster & https://github.com/aartek/spotify-chromecast-player)
audio - html5 audio player
audio - tabletop audio picker ui
map (grid, generate through AI)
tokens
log
script
scene
monster
npc
initiative list
propagating game state between nodes instead of copying config file
websockets for wasm as "reverse passive"
*/

pub mod audio;
pub mod config;
pub mod context;
pub mod game;
pub mod lights;
pub mod void;
