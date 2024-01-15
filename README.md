# juoksevajohtaja.fi

[![Node.js v20 CI](https://github.com/paazmaya/juoksevajohtaja.fi/actions/workflows/linting-and-unit-testing.yml/badge.svg)](https://github.com/paazmaya/juoksevajohtaja.fi/actions/workflows/linting-and-unit-testing.yml)

Static web site available at [`juoksevajohtaja.fi`](https://juoksevajohtaja.fi),
made for the "running leader" podcast, hosted at [render](https://render.com/),
build with [11ty.js](https://www.11ty.dev/),
and DNS provided for free by [1984](https://1984hosting.com/)

## Running Game

2D tile platform game, where two characters run towards challenges, as we all do in life, specially if you are a leader.

Technologies used while creating the game:

* Rust, https://www.rust-lang.org/, programming language
* Bevy, https://bevyengine.org/, game engine
* Rapier, https://rapier.rs/, physics engine
* Kenney, https://www.kenney.nl/, game assets such as images
  * https://www.kenney.nl/assets/platformer-characters
  * https://www.kenney.nl/assets/toon-characters-1
  * https://www.kenney.nl/assets/platformer-bricks
* LDtk level editor, https://ldtk.io/
* WebAssembly, to run fast on a web browser
  