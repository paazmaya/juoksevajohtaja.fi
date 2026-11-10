# Juokseva Johtaja: Krea 2 Asset Prompts

These prompts are for creating 2D assets for the side-scrolling running and
interview game. Generate each character and background layer separately in
Krea 2; do not ask for a complete scene or a sheet of layers in one image.

## Art direction and workflow

- Friendly Nordic daylight, clean 2D cartoon platformer art, bold dark outlines,
  simple shapes, flat colors and at most one or two shadow tones.
- Palette: morning cyan `#87CEEB`, warm sunrise `#FFD180`, charcoal track
  `#374151`, host orange `#FF6B35`, runner blue `#2563EB`, birch green
  `#48BB78` and autumn amber `#F6AD55`.
- Use a consistent side-on, orthographic view, with characters facing right,
  except for the front-facing pose of each person described below. Keep the
  same proportions, colors, clothing and outline weight across assets.
- Each code block below is a standalone prompt. Generate an idle image first
  for each character, then use the approved image as a visual reference for the
  other poses if your Krea workflow supports it. Review and correct anatomy,
  silhouettes and unwanted details.
- Generate animation frames individually rather than relying on Krea to produce
  a correctly spaced sprite sheet. Use the same square canvas and character
  scale for every frame of a character, with room for limbs and a consistent
  foot baseline. Align and pack the frames in an image editor afterward.
- Request a plain white background for isolated sprites. Remove it afterward
  and export PNGs with transparency; check the edges and restore any white
  details inside the artwork. Generated transparency and perfect tiling may
  require manual cleanup.

## Character sprites

Each prompt describes an idle pose. For each approved character, repeat the
prompt with the idle pose replaced by each of the following:

1. Four distinct running frames in cycle order: right foot forward, passing
   position, left foot forward, opposite passing position. Arms swing naturally.
2. One airborne jump, knees bent and feet clear of the ground.
3. One friendly talking or listening pose suitable for an interview while
   keeping the character facing right.
4. One cheerful celebration pose.

For every person, also generate one full-body, front-facing standing pose
looking directly at the viewer. In that person's prompt, replace "idle standing
pose, strict side profile facing right" with "standing pose, orthographic
front view, face and body facing directly toward the viewer". Keep the same
identity, outfit, scale, canvas, baseline and art style as the side-view sprite.

Keep the character's identity and outfit unchanged. No text, speech bubbles,
extra characters, props outside the silhouette, ground or cast shadow. Animals
remain recognizably animals and use their natural locomotion, not human legs.

### Host: Jukka Paasonen

```text
One full-body 2D side-scrolling game character sprite, idle standing pose, strict side profile facing right. Jukka Paasonen, an approachable male podcast host and runner with short neat hair and a clearly visible short, neatly trimmed beard. He wears running headphones, a bright orange athletic shirt, navy running shorts and blue running shoes. Friendly Nordic cartoon platformer art, clean bold dark outlines, simple shapes, flat vivid colors and minimal cel shading. Center the entire character on a plain pure white background, with space around the body and feet on a consistent baseline. No text, ground, cast shadow, other characters or scenery.
```

### Guest: Sanna Marin

```text
One full-body 2D side-scrolling game character sprite, idle standing pose, strict side profile facing right. A recognizable, respectful cartoon likeness of Sanna Marin with long dark brown hair tied in a ponytail, wearing a teal running jacket, black running leggings and light-colored running shoes. Friendly Nordic cartoon platformer art, clean bold dark outlines, simple shapes, flat vivid colors and minimal cel shading. Center the entire character on a plain pure white background, with space around the body and feet on a consistent baseline. No text, ground, cast shadow, other characters or scenery.
```

### Guest: Tom Cruise as a Top Gun-inspired pilot

```text
One full-body 2D side-scrolling game character sprite, idle standing pose, strict side profile facing right. A recognizable cartoon likeness of Tom Cruise as a confident fighter pilot inspired by Top Gun, with short dark hair, aviator sunglasses and a friendly smile. He wears an olive-green flight suit with simple unmarked patches and running shoes. Friendly Nordic cartoon platformer art, clean bold dark outlines, simple shapes, flat vivid colors and minimal cel shading. Center the entire character on a plain pure white background, with space around the body and feet on a consistent baseline. No text, logos, aircraft, ground, cast shadow, other characters or scenery.
```

### Guest: Alexander Stubb

```text
One full-body 2D side-scrolling game character sprite, idle standing pose, strict side profile facing right. A recognizable, respectful cartoon likeness of Alexander Stubb, an athletic Finnish man with short light brown hair and a friendly smile, wearing a royal blue running shirt, dark running shorts and running shoes. Friendly Nordic cartoon platformer art, clean bold dark outlines, simple shapes, flat vivid colors and minimal cel shading. Center the entire character on a plain pure white background, with space around the body and feet on a consistent baseline. No text, ground, cast shadow, other characters or scenery.
```

### Guest: Gorilla

```text
One full-body 2D side-scrolling game character sprite, idle standing pose on all fours, strict side profile facing right. A friendly, expressive adult gorilla with dark charcoal fur, a broad chest and long arms, recognizably a gorilla rather than a person in a costume. Friendly Nordic cartoon platformer art, clean bold dark outlines, simple shapes, flat colors and minimal cel shading. Center the entire animal on a plain pure white background, with space around the body and feet on a consistent baseline. No clothing, text, ground, cast shadow, other characters or scenery.
```

For the running frames, use a natural quadrupedal knuckle-running gait. For the
interview pose, show the gorilla listening or gesturing while remaining
recognizably animal-like.

### Guest: Grizzly bear

```text
One full-body 2D side-scrolling game character sprite, idle standing pose on all fours, strict side profile facing right. A friendly, expressive grizzly bear with warm brown fur, a prominent shoulder hump, rounded ears and a broad muzzle, recognizably a bear rather than a person in a costume. Friendly Nordic cartoon platformer art, clean bold dark outlines, simple shapes, flat colors and minimal cel shading. Center the entire animal on a plain pure white background, with space around the body and feet on a consistent baseline. No clothing, text, ground, cast shadow, other characters or scenery.
```

For the running frames, use a natural four-legged bear gait. For the interview
pose, show the bear listening or gesturing while remaining recognizably
animal-like.

### Additional guest template

```text
One full-body 2D side-scrolling game character sprite, idle standing pose, strict side profile facing right. [Guest description and distinctive features], wearing [running outfit and shoes]. Friendly Nordic cartoon platformer art, clean bold dark outlines, simple shapes, flat vivid colors and minimal cel shading. Center the entire character on a plain pure white background, with space around the body and feet on a consistent baseline. No text, ground, cast shadow, other characters or scenery.
```

## Helsinki running route: parallax background layers

Generate each layer as a **separate wide image**, with the same canvas size,
side-on viewpoint, lighting and horizon placement. The layers are stacked in
the order below, behind the runners. Except for the opaque sky, remove the
white background and export each layer as a transparent PNG. Check that the
left and right edges join seamlessly; retouch edges if needed. Avoid placing
unique landmarks across a tile boundary.

### 1. Sky: static or slowest

```text
One wide seamless horizontally repeating 2D game background tile: a calm Nordic morning sky over Helsinki, soft cyan with a little warm sunrise color and sparse simple clouds. Flat cartoon platformer art, restrained colors, side-on view, no perspective. Fill the entire canvas with sky; matching left and right edges. No buildings, trees, ground, characters, text or borders.
```

### 2. Far distance: skyline and hills

```text
One wide seamless horizontally repeating 2D parallax layer for a Helsinki running game: distant low hills, small muted tree silhouettes and simple modern Nordic buildings, including a subtle Oodi-inspired library silhouette. Side-on view, consistent low horizon, subdued blue-green colors, flat cartoon platformer art and simple dark outlines. Place distant scenery near the bottom and leave the rest plain pure white for removal. Matching left and right edges. No sky, foreground trees, track, characters, text or borders.
```

### 3. Midground: park and trees

```text
One wide seamless horizontally repeating 2D parallax layer for a Helsinki Töölönlahti park running game: Finnish birch trees with green and autumn amber foliage, shrubs, a few simple benches and lamps, viewed side-on. Flat friendly cartoon platformer art, bold clean outlines and bright flat colors. Keep the tree bases aligned near the same low horizon as the distant layer; leave the space between and above objects plain pure white for removal. Matching left and right edges. No sky, buildings, track, characters, text or borders.
```

### 4. Near foreground: vegetation

```text
One wide seamless horizontally repeating 2D parallax layer for a Nordic park running game: a low strip of nearby grass, small leafy plants and scattered stones, seen side-on. Flat friendly cartoon platformer art, bold clean outlines and saturated green and amber colors. Keep all foliage low at the bottom of the canvas so it will not obscure runners or the track; leave the rest plain pure white for removal. Matching left and right edges. No sky, trees, buildings, pavement, characters, text or borders.
```

### 5. Running track

```text
One wide seamless horizontally repeating 2D side-scrolling game ground tile: a level charcoal asphalt running path seen from the side, with a thin pale edge and sparse simple white dash markings. Flat cartoon platformer art, clean dark outlines, no perspective, consistent ground height and matching left and right edges. Fill only a narrow strip at the bottom; leave the area above plain pure white for removal. No numbers, text, characters, scenery or borders.
```

Stack sky, far skyline, midground and foreground from back to front; draw the
track at ground level. As a starting point for implementation, move the far
layer at 0.15 times world scroll, the midground at 0.45, the near foreground at
0.7 and the track at 1.0; keep the sky static. Tune the speeds and foreground
placement in game so runners and obstacles remain visible.

## Obstacles and pickups

Generate these separately as full, isolated sprites using the same style and
white-background cleanup workflow:

```text
One isolated 2D side-scrolling game pickup sprite: a cheerful steaming takeaway coffee cup, side-on, centered on a plain pure white background. Friendly Nordic cartoon platformer art, bold dark outlines, simple flat colors, minimal cel shading. No text, scenery, ground or cast shadow.
```

```text
One isolated 2D side-scrolling game pickup sprite: a small podcast microphone with simple sound-wave shapes, side-on, centered on a plain pure white background. Friendly Nordic cartoon platformer art, bold dark outlines, simple flat colors, minimal cel shading. No text, scenery, ground or cast shadow.
```

```text
One isolated 2D side-scrolling game obstacle sprite: a comically oversized red calendar notification shaped like a hurdle, side-on, centered on a plain pure white background. Friendly Nordic cartoon platformer art, bold dark outlines, simple flat colors, minimal cel shading. No text, logos, scenery, ground or cast shadow.
```

## Preparing assets for the game

1. Remove the white backgrounds from sprites and non-sky layers in an image
   editor, checking for halos and accidentally removed white details. Export
   as transparent PNGs.
2. Align each character's frames to a common canvas, baseline and scale; pack
   them into a sprite sheet if the eventual animation implementation needs one.
   Verify that the running cycle loops smoothly.
3. Check each background tile by placing copies side by side, and composite all
   layers together to check alignment, contrast and visibility.
4. Store the finished files under `game/assets/` with descriptive names. The
   current renderer draws characters and backgrounds procedurally: loading
   these images, mapping level sprite names to files, animating the frames and
   scrolling image layers will require separate game code changes.
