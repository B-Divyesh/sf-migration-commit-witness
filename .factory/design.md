# Visual thesis: brutalist concrete and moss

Migration Commit Witness should feel like evidence stamped onto a load-bearing
surface: severe enough for a release gate, alive enough to imply recovery. The
site uses **brutalist concrete and moss** rather than a generic developer-tool
gradient. Large ruled slabs represent transaction boundaries; fine moss-like
marks collect only around verified edges. Decoration always explains the job:
the central witness image shows one database stratum before, after, and rolled
back around an uncompromising commit seam.

## Palette

This is an explicitly dark, single-mode product. Painting the background keeps
the concrete metaphor stable and avoids an unrequested system theme changing
the meaning of evidence states.

| Token | Hex | Use |
| --- | --- | --- |
| `--ink` | `#F3F0E7` | Primary text / chalk aggregate |
| `--soot` | `#121512` | Page background |
| `--slab` | `#20241F` | Raised concrete |
| `--slab-2` | `#2C312B` | Controls / secondary strata |
| `--ash` | `#B9BEB3` | Muted text (7.9:1 on soot) |
| `--moss` | `#B7D879` | Primary action and verified state |
| `--moss-deep` | `#29452A` | Moss backing, never body text |
| `--rust` | `#FF9B74` | Failure / broken boundary |
| `--amber` | `#F4CA72` | Warning / unexercised claim |
| `--line` | `#596157` | Rules and input borders |

Text and meaningful controls meet WCAG AA; state is always paired with a word,
symbol, or pattern rather than encoded by color alone.

## Type

- Display: `Arial Black`, `Arial Narrow Bold`, sans-serif. Its compressed,
  blunt forms resemble inspection stamps without adding a font payload.
- Working text and data: `ui-monospace`, `SFMono-Regular`, `Consolas`,
  monospace. Numbers use tabular figures; code and evidence share one voice.

No third-party fonts are requested at runtime. The scale is 14 / 16 / 20 / 28 /
clamp(44–88) px, with long-form text limited to 68 characters.

## Spacing and composition

An 8 px base rhythm with 4 px micro-spacing. Sections use 64–128 px vertical
intervals. The desktop page is an offset 12-column evidence sheet; the hero
copy occupies seven columns and the witness core five. On 390 px screens,
secondary labels and the decorative specimen index disappear while the proof
sequence stacks in reading order. Controls stay at least 44 px high.

Borders are square and 1–3 px, with occasional 10 px clipped corners. Shadows
are hard-offset, never blurred: this is poured material, not floating glass.

## Interaction grammar

- Primary actions depress by 3 px, like a physical test switch.
- Expanders reveal evidence directly beneath their trigger.
- The recorded demo advances across `before → commit → after → rollback` with
  one deliberate step at a time and remains fully operable by keyboard.
- License verification never blocks the free page; cached state appears first
  and reconciliation is announced quietly.
- Empty, offline, loading, success, warning, and invalid-license states each
  include a concrete next action.

## Motion policy

Only transform and opacity animate, 180–260 ms. The witness marker moves from
its source stage; button presses have a 100 ms physical depression. Nothing
loops. Under `prefers-reduced-motion: reduce`, transforms are removed,
transitions become effectively instant, and the recorded demo changes by text
and pattern alone.

## Asset plan and provenance

One original raster hero, `site/public/witness-core.webp`, was generated
with `/opt/fleet/lib/gen-image.sh` using the factory `factory-image` deployment,
visually inspected, then resized/optimized locally to a 122,462-byte WebP. The
generation metadata is preserved in `.factory/witness-core.provenance.json`.
Prompt:

> Use case: stylized-concept. Asset type: wide landing-page evidence
> illustration. A brutalist concrete database core in cutaway, three horizontal
> geological strata separated by one precise black transaction seam; restrained
> moss growing only along edges that survived verification; a single rust-orange
> fracture ends at the commit boundary; screen-printed editorial texture,
> graphite rubbings, aggregate grain, orthographic composition, dark soot field,
> bone-white concrete, moss green and small rust accents, generous negative
> space, no people, no logos, no interface screenshot, no letters, no words, no
> numbers, no watermark.

The image is explanatory atmosphere, not a diagram; its adjacent HTML caption
provides the exact semantic reading. Interface icons are original CSS/Unicode
marks and need no external asset license.

The 1200×630 `share.webp` and 180×180 `apple-touch-icon.png` are local crops of
that same original image, made with ImageMagick. They introduce no new source
material. Round 1 keeps the concrete-and-moss direction, while reducing the
mobile hero to the job, sample action, and three facts before any artwork.
