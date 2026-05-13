# docs/ — GitHub Pages landing page

This directory is served verbatim at
<https://twarga.github.io/RocmTop/> once GitHub Pages is pointed at it
(Repository Settings → Pages → Branch: `main`, folder: `/docs`).

## Layout

- `index.html` — single-page landing with hero, features, install, FAQ.
- `styles.css` — dark theme matching the app.
- `icon.svg`, `icon-512.png` — logo assets reused from the app.
- `screenshots/` — drop PNG/GIF captures of the app here. The README in
  the repo root has a placeholder comment that will show them once added.
- `.nojekyll` — disables Jekyll so filenames starting with `_` are served.

## Enabling the page

On GitHub:

1. Repo → **Settings** → **Pages**.
2. **Source**: "Deploy from a branch".
3. **Branch**: `main`, folder: `/docs`.
4. Save. The site is usually live within a minute.

There is no build step — committing HTML/CSS here is enough.
