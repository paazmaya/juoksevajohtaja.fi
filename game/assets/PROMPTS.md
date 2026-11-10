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
  a correctly spaced sprite sheet. Use the same square canvas (`1:1` aspect ratio,
  e.g. 1024 × 1024 px) and character scale for every frame of a character, leaving
  ~15% breathing room around limbs and maintaining a consistent foot baseline.
  Align and pack the frames in an image editor afterward.
- Generate background layers on a wide landscape canvas (`16:9`, e.g. 1920 × 1080 px,
  or `2:1`, 2048 × 1024 px) to facilitate seamless horizontal edge matching.
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
4. One cheerful celebration pose (thumbs up or fist pump).
5. One comical stumble or tripping pose (recoiling slightly, arms flailing)
   for visual feedback when hitting an obstacle during the invulnerability timer.

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

## Running environments: parallax background layers

Generate each layer as a **separate wide image**, with the same canvas size
(`16:9` at 1920 × 1080 px or `2:1` at 2048 × 1024 px), side-on orthographic
viewpoint, matching lighting, and consistent low horizon line (placed at roughly
75% down the canvas, corresponding to `GROUND_Y`).

Except for the opaque sky, request a plain pure white background above the
objects for clean extraction into transparent PNGs. Check that left and right
edges match seamlessly so the layer can repeat infinitely during running.

Stack layers from back to front:
1. **Sky**: static or 0.05× scroll
2. **Far distance**: 0.15× scroll
3. **Midground**: 0.45× scroll (where primary landmark features sit)
4. **Near foreground**: 0.70× scroll (low vegetation, edge details)
5. **Running track**: 1.00× scroll (ground surface)

---

### Environment 1: Helsinki City Center with Trams

An urban morning run through downtown Helsinki, featuring classic Art Nouveau
stone buildings, copper domes, and green-and-yellow trams.

#### 1. Sky: Morning city sky
```text
One wide seamless horizontally repeating 2D game background tile: a calm Nordic morning sky over Helsinki, soft cyan (#87CEEB) transitioning to warm sunrise gold (#FFD180) near the horizon with sparse wispy clouds. Flat cartoon platformer art, clean restrained colors, strict side-on view, no perspective or vanishing point. Fill the entire canvas with sky; left and right edges join seamlessly. No buildings, trees, ground, characters, text or borders.
```

#### 2. Far distance: Helsinki skyline and domes
```text
One wide seamless horizontally repeating 2D parallax layer for an urban Helsinki running game: distant muted city skyline silhouettes, neoclassical stone facades, the iconic green copper cathedral dome (Tuomiokirkko), and church spires, in soft hazy blue-gray tones (#7A8B99). Strict side-on orthographic view, low horizon line. Place distant buildings in a low horizontal band along the bottom third; leave the upper area plain pure white for removal. Matching left and right edges. No foreground, roads, vehicles, characters or text.
```

#### 3. Midground: City street, buildings, and green-and-yellow trams
```text
One wide seamless horizontally repeating 2D parallax layer for a Helsinki city center running game: handsome 4-story Nordic Art Nouveau stone apartment facades in pastel ochre, cream, and brick-red; street rails with a classic Helsinki tram in traditional dark bottle-green (#276749) with golden-yellow (#ECC94B) window trim; black cast-iron street lamps and granite building entrances. Strict side-on orthographic 2D view, no perspective. Bases of buildings and tram wheels aligned along the ground baseline; leave the sky area above plain pure white for removal. Left and right edges join seamlessly. No characters, speech bubbles, track or text.
```

#### 4. Near foreground: Granite curbs and street planters
```text
One wide seamless horizontally repeating 2D parallax layer for an urban street running game: a low ground-level strip of dressed gray granite curbstones, modern metal street railings, and small wooden city planters with orange and yellow marigold flowers, seen side-on. Flat cartoon platformer art, bold clean dark outlines. Keep all objects low along the bottom edge so runners are not obscured; leave the area above plain pure white for removal. Matching left and right edges. No pavement, characters or text.
```

#### 5. Track: Cobblestone city pavement
```text
One wide seamless horizontally repeating 2D side-scrolling game ground tile: a flat, level Helsinki street pavement paved with neat gray granite cobblestones, with a crisp straight curb edge and painted white bicycle/runner lane lines. Flat cartoon platformer art, clean dark outlines, strict side-on view, no perspective. Fill only the narrow bottom strip below the ground line; leave the area above plain pure white for removal. Matching left and right edges. No text or scenery.
```

---

### Environment 2: Finnish Taiga Forest

A trail run through Nordic coniferous and birch forest, with granite boulders,
blueberry undergrowth, and a soft bark-chip path (pururata).

#### 1. Sky: Forest morning canopy sky
```text
One wide seamless horizontally repeating 2D game background tile: a cool crisp Scandinavian forest morning sky, pale clear cyan-blue (#93C5FD) with soft morning mist near the horizon. Flat cartoon platformer art, minimal cel shading, strict side-on view, no perspective. Fill the entire canvas with sky; matching left and right edges. No trees, ground, characters, text or borders.
```

#### 2. Far distance: Deep pine ridges
```text
One wide seamless horizontally repeating 2D parallax layer for a Finnish forest running game: distant layers of rolling forested hills covered with dense dark-spruce and pine silhouettes in atmospheric muted blue-greens (#234E52, #285E61). Flat cartoon platformer art, simple shapes, strict side-on view, consistent low horizon. Place distant tree ridges along the bottom third; leave the area above plain pure white for removal. Left and right edges join seamlessly. No foreground trees, path, animals, characters or text.
```

#### 3. Midground: Pines, silver birches, and granite boulders
```text
One wide seamless horizontally repeating 2D parallax layer for a Finnish nature trail running game: tall Scots pine trees with textured reddish-orange upper bark (#C05621), slender white silver birches with dark bark markings and lush green leaves (#48BB78), and rounded moss-covered gray granite boulders (#718096). Flat friendly cartoon platformer art, bold clean outlines, vibrant solid colors. Tree bases and rocks resting on a consistent low ground line; leave the space between and above trunks plain pure white for removal. Matching left and right edges. No path, characters, animals or text.
```

#### 4. Near foreground: Moss, blueberry bushes, and ferns
```text
One wide seamless horizontally repeating 2D parallax layer for a Finnish forest running game: a low strip of lush green forest floor moss (#38A169), wild blueberry (mustikka) and lingonberry shrubs with small green leaves and tiny red and blue berries, a few small forest ferns, and scattered pine cones. Flat cartoon platformer art, clean dark outlines. Keep all undergrowth confined to a shallow strip along the bottom edge; leave the area above plain pure white for removal. Matching left and right edges. No trees, path, characters or text.
```

#### 5. Track: Bark-chip trail (pururata)
```text
One wide seamless horizontally repeating 2D side-scrolling game ground tile: a level Finnish sawdust / bark-chip outdoor exercise track (pururata), rich warm cedar-brown woodchips (#854D0E, #9A3412) with a neat natural grass border along the top edge. Flat cartoon platformer art, clean dark outlines, strict side-on view, consistent ground height. Fill only the narrow bottom strip below the ground line; leave the area above plain pure white for removal. Matching left and right edges. No text or obstacles.
```

---

### Environment 3: Rural Fields with Small Country Church in Far Distance

An open summer run across rolling Finnish agricultural plains, with ripening
grain fields, rustic wooden fences, and a traditional parish church steeple
on the distant horizon.

#### 1. Sky: Open agricultural summer sky
```text
One wide seamless horizontally repeating 2D game background tile: a wide-open Nordic summer sky over rural fields, sunny pale blue with a few puffy white cumulus clouds drifting high up and warm golden sunlight (#FEF08A). Flat cartoon platformer art, clean bright colors, strict side-on view, no perspective. Fill the entire canvas with sky; matching left and right edges. No ground, buildings, trees, characters or text.
```

#### 2. Far distance: Country church steeple and distant farm horizon
```text
One wide seamless horizontally repeating 2D parallax layer for a Finnish countryside running game: a distant flat rural horizon with a classic Finnish whitewashed countryside stone parish church featuring a steep dark-shingled roof and separate wooden bell tower (tapuli) silhouette, framed by a faraway dark pine treeline and low distant barns. Soft muted pastel colors (#64748B, #475569). Strict side-on view, low horizon along the bottom quarter; leave the sky above plain pure white for removal. Matching left and right edges. No foreground, roads, crops, characters or text.
```

#### 3. Midground: Golden grain fields and split-rail fence
```text
One wide seamless horizontally repeating 2D parallax layer for a Finnish farmland running game: waving fields of golden ripe oats and wheat (#EAB308, #FACC15), an authentic Finnish rustic wooden split-rail pole fence (riukuaita / seiväsaita) running along the field edge, and occasional neat cylindrical rolled hay bales wrapped in pale plastic. Flat friendly cartoon platformer art, bold clean dark outlines, bright warm colors. Base of fence and crops aligned along the ground line; leave the area above plain pure white for removal. Matching left and right edges. No church, roads, characters or text.
```

#### 4. Near foreground: Roadside wildflowers and tall grasses
```text
One wide seamless horizontally repeating 2D parallax layer for a rural Finnish road running game: a low roadside ditch strip with nodding bluebells (kissankello), white yarrow, yellow buttercups, and tall stalks of wild timothy grass. Flat cartoon platformer art, clean dark outlines, bright saturated colors. Keep all wild plants low along the bottom margin; leave the area above plain pure white for removal. Matching left and right edges. No pavement, fence, buildings, characters or text.
```

#### 5. Track: Rural gravel country road
```text
One wide seamless horizontally repeating 2D side-scrolling game ground tile: a firm, level Finnish country gravel road (hiekkatie) in warm sandy-gray (#D1D5DB, #A8A29E) with fine pebble textures and a soft grassy verge. Flat cartoon platformer art, clean dark outlines, strict side-on view, no perspective. Fill only the narrow bottom strip below the ground line; leave the area above plain pure white for removal. Matching left and right edges. No markings, text or obstacles.
```

---

### Environment 4: Abandoned House with Broken Tractor Next to It

A rustic, slightly weathered Finnish countryside scene featuring a derelict
wooden farmhouse (*autiotalo*) and an overgrown vintage tractor.

#### 1. Sky: Overcast Nordic autumn sky
```text
One wide seamless horizontally repeating 2D game background tile: an atmospheric, slightly overcast Nordic autumn sky, cool silver-gray (#CBD5E1) with pale breaking sunbeams and soft drifting slate clouds. Flat cartoon platformer art, restrained color palette, strict side-on view, no perspective. Fill the entire canvas with sky; matching left and right edges. No buildings, trees, ground, characters or text.
```

#### 2. Far distance: Autumn woodland ridge
```text
One wide seamless horizontally repeating 2D parallax layer for a rural autumn running game: a distant low ridge of mixed autumn woods with golden-yellow birches, orange aspens, and dark evergreen spruces in soft muted tones (#78716C, #854D0E). Strict side-on view, low horizon line along the bottom quarter; leave the sky above plain pure white for removal. Left and right edges join seamlessly. No houses, tractors, roads, characters or text.
```

#### 3. Midground: Abandoned red farmhouse and broken vintage tractor
```text
One wide seamless horizontally repeating 2D parallax layer for a Finnish rural running game: a weathered 1.5-story traditional Finnish wooden farmhouse in faded peeling Falun red paint (punamulta, #991B1B) with a slightly sagging shingle roof, weathered gray wooden window frames, and wild vines climbing one wall. Parked beside the house in overgrown weeds is a broken vintage tractor (classic boxy Finnish Valmet style) with rusted orange patina (#EA580C), a missing exhaust pipe, faded yellow wheel rims, and a flat rear tire. A leaning wooden outbuilding sits nearby. Strict side-on orthographic 2D view, no perspective. House and tractor wheels resting on the ground line; leave the sky above plain pure white for removal. Left and right edges join seamlessly. No characters, speech bubbles, roads or text.
```

#### 4. Near foreground: Overgrown tall weeds and rusted barrel
```text
One wide seamless horizontally repeating 2D parallax layer for a rustic countryside running game: a low strip of overgrown wild tall weeds, stinging nettles, dry golden dandelion seed heads, weathered broken fence rails lying on the ground, and a small rusted iron barrel half-buried in the grass. Flat cartoon platformer art, bold clean dark outlines. Keep all items low along the bottom margin; leave the area above plain pure white for removal. Matching left and right edges. No vehicles, house, road, characters or text.
```

#### 5. Track: Overgrown dirt farm track
```text
One wide seamless horizontally repeating 2D side-scrolling game ground tile: a rural Finnish dirt cart lane with two parallel packed-earth wheel ruts in warm dark soil (#78350F) and a thin strip of hardy green grass growing down the middle. Flat cartoon platformer art, clean dark outlines, strict side-on view, consistent ground height. Fill only the narrow bottom strip below the ground line; leave the area above plain pure white for removal. Matching left and right edges. No text or scenery.
```

---

## Obstacles, pickups, and track props

Generate these separately as full, isolated sprites using the same style and
white-background cleanup workflow:

### Pickups

```text
One isolated 2D side-scrolling game pickup sprite: a cheerful steaming takeaway coffee cup, side-on, centered on a plain pure white background. Friendly Nordic cartoon platformer art, bold dark outlines, simple flat colors, minimal cel shading. No text, scenery, ground or cast shadow.
```

```text
One isolated 2D side-scrolling game pickup sprite: a small podcast microphone with simple sound-wave shapes, side-on, centered on a plain pure white background. Friendly Nordic cartoon platformer art, bold dark outlines, simple flat colors, minimal cel shading. No text, scenery, ground or cast shadow.
```

### Obstacles

```text
One isolated 2D side-scrolling game obstacle sprite: a clean athletic running track hurdle with red-and-white striped crossbar and lightweight silver support legs, side-on, centered on a plain pure white background. Friendly Nordic cartoon platformer art, bold dark outlines, simple flat colors, minimal cel shading. No text, scenery, ground or cast shadow.
```

```text
One isolated 2D side-scrolling game obstacle sprite: a comically oversized red calendar notification shaped like a hurdle, side-on, centered on a plain pure white background. Friendly Nordic cartoon platformer art, bold dark outlines, simple flat colors, minimal cel shading. No text, logos, scenery, ground or cast shadow.
```

```text
One isolated 2D side-scrolling game obstacle sprite: a comical software bug obstacle shaped like a red Jira bug ticket or a stylized round bug with antennae, side-on, centered on a plain pure white background. Friendly Nordic cartoon platformer art, bold dark outlines, simple flat colors, minimal cel shading. No text, scenery, ground or cast shadow.
```

```text
One isolated 2D side-scrolling game obstacle sprite for a forest trail: a moss-covered fallen silver birch tree log hurdle with peeling white-and-black paper bark and small bracket fungi, side-on, centered on a plain pure white background. Friendly Nordic cartoon platformer art, bold dark outlines, simple flat colors, minimal cel shading. No ground, characters, text or cast shadow.
```

```text
One isolated 2D side-scrolling game obstacle sprite for rural fields: a cylindrical agricultural rolled hay bale hurdle wrapped in pale plastic or tied with rustic twine, side-on, centered on a plain pure white background. Friendly Nordic cartoon platformer art, bold dark outlines, simple flat colors, minimal cel shading. No ground, characters, text or cast shadow.
```

```text
One isolated 2D side-scrolling game obstacle sprite for an abandoned farmstead: a rusted vintage steel milk can and leaning weathered wooden crate hurdle, side-on, centered on a plain pure white background. Friendly Nordic cartoon platformer art, bold dark outlines, simple flat colors, minimal cel shading. No ground, characters, text or cast shadow.
```

### Track props

```text
One isolated 2D side-scrolling game checkpoint gate sprite: a welcoming Finnish wooden trail gate with a hanging orange banner, side-on, centered on a plain pure white background. Friendly Nordic cartoon platformer art, bold dark outlines, simple flat colors, minimal cel shading. Leave the center of the arch open; no ground, characters or cast shadow.
```

```text
One isolated 2D side-scrolling game finish line prop sprite: two checkered finish line posts with a bright ribbon banner, side-on, centered on a plain pure white background. Friendly Nordic cartoon platformer art, bold dark outlines, simple flat colors, minimal cel shading. No text, characters, ground or cast shadow.
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
4. Store the finished files under `game/assets/backgrounds/`, `game/assets/obstacles/`,
   and `game/assets/characters/`. The game renderer automatically detects and
   renders these image layers and character sprites when present, while retaining
   procedural drawing as a fallback when images are loading or absent.
