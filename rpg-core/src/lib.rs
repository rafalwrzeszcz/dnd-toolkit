/* TODO:

update Makefile for mobile and web

tests (config, rpc, audio)

campaign
game session objects
player
character

workspace:
unify cli, server and dioxus-cli; yew and dioxus-web

audio - chromecast (spotify app ID: CC32E753 - https://github.com/azasypkin/rust-caster & https://github.com/aartek/spotify-chromecast-player)
audio - html5 audio player
map
tokens
log
script
scene
monster
npc
initiative list
propagating game state between nodes instead of copying config file
websockets for wasm as "reverse passive"

ensure each platform has option to receive message
*/

pub mod audio;
pub mod config;
pub mod context;
pub mod game;
pub mod void;
