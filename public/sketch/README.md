# public/sketch/

Vendored JavaScript for the nine p5.js sketches that project posts embed.
`src/sketch.rs` serves the page that loads them, and its allowlist is the one
place a tenth sketch has to be registered.

- `lib/p5.min.js`, p5.js 1.11.13 from the npm registry. Held at 1.x because the
  sketches were written against ^1.4.1 and 2.x renamed enough of the API to
  break them. `lib/p5-LICENSE.txt` is its licence, shipped alongside.
- `js/*.js`, one IIFE bundle per sketch, publishing its p5 instance-mode
  function as `__sketch.default`.
- `lib/roboto-regular.ttf`, because WebGL text needs a real font file and
  `lorenz-attractor` is the one sketch that draws any. p5's `loadFont` reads
  TrueType and OpenType, so the woff2 faces in `public/fonts/` are no use to it.

## Regenerating a bundle

The TypeScript sources are not in the working tree. They live in git, at the
last commit before the Leptos port:

```sh
git show 'd82ff04^:web/src/components/sketch/matrix.ts'
```

There is no npm project here and adding one for a build that runs about once a
year is not worth it. Pull the sources into a scratch directory, transpile, and
copy the output back:

```sh
cd "$(mktemp -d)" && mkdir src
for f in asteroids fireworks fluid-simulation fourier lorenz-attractor \
         matrix maze-astar pong raycasting-2d utils; do
  git -C ~/dev/lukeworks show "d82ff04^:web/src/components/sketch/$f.ts" > "src/$f.ts"
done

# The sketches import p5 for its types. It loads as a global <script> instead,
# so the import resolves to a shim rather than pulling 1MB into every bundle.
echo 'export default globalThis.p5;' > src/p5-shim.js

for f in asteroids fireworks fluid-simulation fourier lorenz-attractor \
         matrix maze-astar pong raycasting-2d; do
  npx esbuild@0.25 "src/$f.ts" --bundle --format=iife --global-name=__sketch \
    --alias:p5=./src/p5-shim.js --minify --target=es2020 --outfile="out/$f.js"
done
```

One source needs an edit before it builds usefully. The lorenz-attractor source
loads `/fonts/noto_sans_regular.ttf`, which never existed in the Next.js repo
either, so that sketch failed in `preload` there too. Point it at
`/sketch/lib/roboto-regular.ttf`.

A Rust and `web-sys` rewrite of the sketches is still on the board. This
vendoring exists to unblock the nine posts until then.
