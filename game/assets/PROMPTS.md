# Juokseva Johtaja Game Visual Style Guide & AI Generation Prompts

This guide defines the visual aesthetic and provides production-ready prompts for generating game assets using AI generators (Midjourney v6, DALL-E 3, or Stable Diffusion XL).

---

## 1. Visual Style & Art Direction

The art style is modeled after **Max Garkavyi's 2020game.io** and **Kenney's Toon/Platformer Characters**:
- **Geometry**: Clean 2D side-scrolling platformer vector art, bold clean silhouettes.
- **Outlines**: Thick, uniform dark borders (2-3px line weight).
- **Shading**: Cel-shaded / flat vector with minimal gradients (1-2 shadow tones max).
- **Color Palette**: Vibrant, high-contrast, friendly Nordic daylight outdoors:
  - Sky: Soft morning cyan `#87CEEB` to warm sunrise `#FFD180`
  - Asphalt / Track: Neutral charcoal `#374151` with clean white running distance markers
  - Host runner: Safety neon orange `#FF6B35` / runner blue `#2563EB`
  - Nature / foliage: Lush Finnish birch green `#48BB78` to autumn amber `#F6AD55`
- **Output Requirements**:
  - Transparent PNG background (or pure solid white `#FFFFFF` for automatic background removal).
  - Orthographic side-profile view (facing right).

---

## 2. Character Generation Prompts

### Host: Jukka Paasonen ("Juokseva Johtaja")
*The host runs alongside the player, wearing athletic running gear, headphones, and holding a small podcast recorder or coffee mug.*

#### Midjourney v6 Prompt:
```text
2D game sprite sheet of an athletic male marathon runner in his 30s with short neat hair and running headphones, wearing a bright orange athletic runner t-shirt with a small microphone badge, navy blue running shorts, and modern running shoes. Side-profile view facing right, orthographic view. Poses on a single sheet: 1) idle breathing pose, 2) 4-frame dynamic running sprint cycle, 3) jumping mid-air pose with knees bent, 4) cheerful thumbs-up pose holding a takeaway coffee cup, 5) comical scheming face waving a calendar invite obstacle. Clean vector illustration, bold outlines, flat colors, Kenney game assets style, 2020game.io style, solid clean white background, high contrast --no 3d, realistic, photographic, shading, shadows, gradients --v 6.0 --ar 16:9
```

#### DALL-E 3 Prompt:
```text
A 2D vector video game character sheet in the clean flat cartoon style of Kenney.nl and 2020game.io. The character is a male podcast host running leader, wearing an orange running jersey with a subtle microphone logo, dark blue shorts, and running sneakers. Show 4 poses side-by-side facing right: running cycle, jumping over a hurdle, cheering with a takeaway coffee mug, and tripping. Flat 2D vector art, bold black outlines, bright solid colors, isolated on a pure white background.
```

---

### Episode Guest: Customizable Template
*Since each episode features a different interviewee, adapt the bracketed tags to match the specific guest's appearance, industry, and role.*

#### Master Prompt Template:
```text
2D side-scrolling video game character sprite sheet of [INTERVIEWEE_DESCRIPTION, e.g. a smiling woman tech executive with blonde hair in a ponytail], wearing [RUNNING_OUTFIT, e.g. a teal athletic windbreaker jacket, black running leggings, and lime-green running shoes], wearing a sports smartwatch. Side-profile perspective facing to the right. 
Poses included on sheet:
1. Idle running stance
2. 4-frame running animation cycle
3. Jumping into the air
4. Thinking pose with lightbulb above head (solving question)
5. Victorious celebration cross finish line
Style: Clean 2D vector cartoon platformer art, bold solid outlines, Kenney toon style, 2020game.io aesthetic, vibrant flat colors, isolated on solid white background --no 3d, realistic, gradients, complex background, text
```

#### Example 1: Season 1, Episode 1 ("Undisclosed" Tech Leader)
```text
2D side-scrolling video game character sprite sheet of an approachable tech CTO in his 40s with glasses and stubble, wearing a sleek black running long-sleeve with subtle circuit pattern, gray athletic tights, and vibrant cyan running shoes. Side-profile facing right. Running cycle, jumping, thinking pose, celebratory fist pump. Flat vector art, thick clean outlines, Kenney platformer character style, isolated on clean white background.
```

---

## 3. Parallax Backgrounds (Episode Settings)

Each episode can represent a running path corresponding to where the conversation was held (e.g., Helsinki Töölönlahti, Central Park, Silicon Valley, Forest trail).

### Helsinki Urban Run (Töölönlahti / Oodi)
```text
Seamless 2D horizontal side-scroller parallax game background layers, Nordic Helsinki urban running trail. Divided into 4 distinct horizontal strips:
- Layer 1 (Distant Sky): Soft morning Scandinavian sunrise with gentle gradient clouds.
- Layer 2 (Far Skyline): Minimalist flat vector silhouettes of Helsinki landmarks (Oodi library, modern glass office hubs, distant trees).
- Layer 3 (Midground): Autumn Finnish birch trees with yellow-orange leaves, wooden park benches, street lamps.
- Layer 4 (Foreground Ground): Clean asphalt running pavement with crisp white kilometer line markings (0.5km, 1.0km, 1.5km).
Vector illustration, flat design, 2020game.io background style, seamless horizontal tileable pattern --ar 16:9
```

---

## 4. Obstacles, Sabotage Items & Checkpoints

```text
2D side-scroller video game collectible icons and obstacles sprite sheet, cartoon vector style:
1. Steaming takeaway coffee cup with golden speed-lines (energy pickup)
2. Broadcast podcast microphone on a podium with glowing sound waves (checkpoint marker)
3. Giant red Google Calendar "URGENT MEETING" popup box hurdle (sabotage obstacle)
4. Stack of red Jira bug tickets with angry spider bug icons (platform hurdle)
5. Finnish wooden trail roadblock gate with "KYSYMYS / QUESTION" banner
6. Finish line banner ribbon with confetti bursts
Kenney game assets style, flat vector colors, thick black outlines, isolated on clean white background.
```

---

## 5. Integrating Generated Sprites into the Game

1. Remove the white background using ImageMagick, remove.bg, or Photoshop:
   ```bash
   magick convert input_sprite.png -fuzz 10% -transparent white output_sprite.png
   ```
2. Place the frames into `game/assets/characters/<guest_id>/`
3. Reference the guest's sprite and parameters in `game/assets/levels/<episode>.json`.
