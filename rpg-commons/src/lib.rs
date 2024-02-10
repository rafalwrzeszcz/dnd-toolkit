/* TODO:

tests (config, rpc, audio)

campaign
game session objects
player
character

workspace:
server
cli interface repl
dioxus
    web
    desktop
    mobile
    cli
web interface (yew)
flutter ui
mobile app
tauri? instead of grpc? additionally?

audio - chromecast (spotify app ID: CC32E753 - https://github.com/azasypkin/rust-caster & https://github.com/aartek/spotify-chromecast-player)
map
tokens
log
script
scene
monster
npc
initiative list
propagating game state between nodes instead of copying config file
*/

pub mod audio;
pub mod config;
pub mod game;
pub mod rpc;
pub mod spotify;
pub mod void;
