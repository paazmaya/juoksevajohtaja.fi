# juoksevajohtaja.fi

[![Node.js CI](https://github.com/paazmaya/juoksevajohtaja.fi/actions/workflows/linting-and-unit-testing.yml/badge.svg)](https://github.com/paazmaya/juoksevajohtaja.fi/actions/workflows/linting-and-unit-testing.yml)

Web site of the "running leader" podcast, available at [`juoksevajohtaja.fi`](https://juoksevajohtaja.fi),
hosted at [render](https://render.com/), and DNS provided for free by [1984](https://1984hosting.com/).

The whole site is written in Rust and runs as WebAssembly:

- [Leptos](https://leptos.dev/) single-page application, built with [Trunk](https://trunk-rs.github.io/trunk/)
- every route is pre-rendered to static HTML at build time for search engines, social previews and deep links
- one podcast player that keeps playing while browsing, including while playing the game
- the running game is a separate Trunk build, downloaded only when the game page is opened

## Project layout

```
episodes.yaml              podcast and episode metadata (audio URL, date, guest, highlights)
content/episode-N.md       episode texts: "## Tiivistelmä" (summary) and "## Vieras" (guest introduction)
crates/content/            content model, YAML/markdown parsing, RSS feed, sitemap and SEO tags
site/                      Leptos application
  style/_tokens.scss       the design system: all colors, spacing, typography, sizes, motion
  src/components/<name>/   reusable UI components, each with its own <name>.scss
  src/pages/<name>/        routed pages, each with its own <name>.scss
  src/bin/prerender.rs     writes pre-rendered HTML, feed.xml, sitemap.xml and robots.txt
  public/                  icons, images and other static files
game/                      the running game (separate Trunk build, served at /peli-app/)
e2e/                       Playwright end-to-end tests
```

## Adding an episode

1. Add an entry to `episodes.yaml` (see the comments at the top of the file).
2. Write `content/episode-N.md` with the two sections `## Tiivistelmä` and `## Vieras`.
3. Put the cover image under `site/public/images/episodes/`. Use JPEG, PNG or WebP for the real covers so that
   social media previews work (SVG is a placeholder only).

The build fails with a clear message when a field, a section or an image is missing. The audio file is hosted
elsewhere; its server must support HTTP range requests (all podcast hosts and CDNs do) so listeners can seek.

## Development

Requirements: Rust with the `wasm32-unknown-unknown` target, [Trunk](https://trunk-rs.github.io/trunk/),
Node.js, and [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) for coverage.

```sh
npm ci
npm start             # site at http://localhost:8080 with live reload
npm run serve:game    # optional, in another terminal: game for the /peli page
npm run build         # site + game + pre-render into dist/
npm run serve         # serve dist/ like the production host
npm test              # unit tests with >= 95 % line coverage, then end-to-end tests
npm run lint
```

Code that only runs in the browser (`site/src/web.rs`, the wasm entry points, the game's DOM wiring) is kept thin
and is covered by the Playwright tests instead of the coverage report.

## How it works

- **Persistent player**: the router only swaps the page content; the header, footer and `<audio>` player stay
  mounted, so navigation never interrupts the podcast. The game page embeds the game in an iframe, which keeps
  the player running around it.
- **Deep links and SEO**: `npm run build` writes a real HTML file for every route
  (`dist/jaksot/<slug>/index.html` etc.) with the page title, description, canonical URL, Open Graph tags and
  `PodcastEpisode` structured data. The WebAssembly then takes over on the same page. Unknown paths get
  `dist/404.html`. The build also publishes the podcast RSS feed at `/feed.xml`.
- **Loading**: while the WebAssembly loads, the host runs along the bottom of the screen and the pre-rendered
  content is already readable.

## Hosting

Render static site: build command `npm ci && npm run build`, publish directory `dist`. Every existing route is a
real file, so no SPA rewrite is needed for them. Unknown paths should get `404.html`; if the host does not serve it
on its own, add a rewrite rule `/*` → `/404.html`. The game moved from `/game/` to `/peli`; add a redirect rule
`/game/*` → `/peli`.

## Running Game

2D tile platform game, where two characters run towards challenges, as we all do in life, specially if you are a leader.

Technologies used while creating the game:

- Rust, https://www.rust-lang.org/, programming language
- Bevy, https://bevyengine.org/, game engine
- Rapier, https://rapier.rs/, physics engine
- Kenney, https://www.kenney.nl/, game assets such as images
  - https://www.kenney.nl/assets/platformer-characters
  - https://www.kenney.nl/assets/toon-characters-1
  - https://www.kenney.nl/assets/platformer-bricks
- LDtk level editor, https://ldtk.io/
- WebAssembly, to run fast on a web browser
