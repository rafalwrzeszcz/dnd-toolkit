/* TODO:

tests (config, audio, lights)

campaign
game session objects
player
character

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
separate app config from game session config
*/

pub mod audio;
pub mod blebox;
pub mod config;
pub mod context;
pub mod game;
pub mod lights;
pub mod spotify;
pub mod void;
