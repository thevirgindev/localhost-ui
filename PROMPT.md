# Prompt for Gemini (paste this into AI Studio along with the attached files)

You are a senior frontend engineer working on "localhost", an existing Tauri 2 desktop anime app. The frontend is Svelte 5 (runes) + TypeScript strict + Vite, with all data coming from typed Tauri commands via `src/lib/api.ts`. I have attached the full `src/` tree and root config files. Read them first and internalize the patterns before writing a single line.

## Mission
Restyle and restructure the UI/UX of this app so it is a LITERAL clone of crunchyroll.com's web UI — the real site's look, layout, and interactions, pixel-level: dark #0d0d0d surfaces, CR orange #F47521 accents, pill-shaped buttons and filter chips, uppercase micro-labels with wide letter-spacing, Inter typography with tight -0.02em headings, hero billboard with left-aligned title block, horizontal carousel rows with circular chevron arrows, hover-lift cards with an orange "Watch" pill overlay, tabbed details page over a blurred backdrop, episode rows with progress bars, orange dot for "airing now". Match crunchyroll.com's actual spacing rhythm and hover behavior — if you have not seen the real site recently, reason from the existing components, which are already 80% there. This is a UI/UX pass only. Zero backend work.

## Hard boundaries (violating any of these = failed task)
1. UI/UX ONLY. Every piece of data comes from the existing `api` object in src/lib/api.ts and navigation from `src/lib/router.ts`. Do NOT invent new Tauri commands, do NOT touch `src-tauri/`, `ipc.ts`, `api.ts` command names, or `main.ts`. If a Crunchyroll UI element needs data that doesn't exist in `types.ts`, omit that element or degrade it gracefully — never fake data, never stub a command.
2. NO new dependencies. package.json stays untouched. No Tailwind, no component libraries, no icon packages, no utility libs. Icons are inline SVG exactly like the existing code does.
3. NO new design system. Reuse the tokens and global classes in `src/app.css` (`.btn.primary/.secondary/.outline/.ghost`, `.chip.active`, `.page`, `.page-title`, `.page-sub`, `.section-title`, `.grid`, `.spinner`, `.skeleton`, `.error-box`, `.empty-state`, `.badge`, `.format-tag`, and the `--bg/--surface/--accent/--radius` variables). Page-specific styles go in each component's scoped `<style>` block, like the existing files. Only edit `app.css` to add genuinely new shared tokens if a CR element truly needs one.
4. Keep the existing architecture: same routes in `router.ts` (home, browse, season, details, watch, list, library, settings), same file layout (`pages/`, `components/`, `lib/`), same component names where possible. Improve how pages look and behave; do not re-architect.

## Anti-slop checklist (every output file is checked against this)
- No emoji used as UI icons. Inline SVG only, `stroke="currentColor"`, matching existing icon style.
- No purple/blue gradient "AI aesthetic", no glassmorphism cards, no neon glows, no fake 3D. The only blur is the existing navbar backdrop-filter.
- No lorem ipsum, no placeholder images, no "Example Title", no dead buttons. Every visible control is wired to real state or removed.
- No invented settings, fake user avatars, fake notifications, fake premium badges, or features the backend can't serve.
- No comments narrating obvious code. Comments only where the existing files use them: a short `// purpose — why` note at the top of a file or above a genuinely non-obvious block.
- No defensive over-engineering: no extra config files, no abstraction layers, no "reusable" components with 10 props used once, no try/catch that swallows errors silently except the existing `reportError` pattern for page loads.
- No TypeScript heroics: no generics gymnastics, no new utility types. Mirror the plain interface style in `types.ts`.

## Bloat limits
- Fewest possible changed files. Do not rewrite files you don't need to touch — assume everything unmentioned stays as-is.
- Reuse existing components (`Row`, `AnimeCard`, `Hero`, `Navbar`) instead of creating near-duplicates. New components only for genuinely new CR patterns (e.g. an episode thumbnail card if the details page needs one).
- Each file stays focused and under ~450 lines. Split only when a page genuinely outgrows that.
- Svelte 5 runes exactly as the existing code: `let { x }: { x: T } = $props()`, `$state`, `$derived`, `$derived.by`, `$effect`, keyed `{#each items as item (item.id)}`, `onMount` async with `loading`/`error` state, `reportError("PageName.load", e)` in catch blocks. No legacy `export let`, no `svelte:component` hacks, no stores beyond the existing router.
- Code style: 2-space indent, double quotes, semicolons, `$derived` for computed values, strict-mode clean (the tsconfig has `noUnusedLocals`/`noUnusedParameters` — dead code will fail the build).

## Specific surfaces to nail (crunchyroll.com reference)
- Navbar: CR logo-style wordmark left, nav links, center search field with rounded-full input, account/settings icon right. Keep current route names.
- Home: full-bleed hero billboard (5-slide crossfade, left-bottom text block, orange Start Watching pill + outline Details pill, dash-style slide indicators), Continue Watching cards with orange progress bars, carousel rows with circular chevron buttons top-right.
- Browse: CR filter bar — pill chips for format/status/genres, dark rounded-full search input, sort/year dropdowns styled dark, responsive card grid, infinite scroll sentinel (already exists — keep it).
- Details: tall banner backdrop with bottom gradient, overlapping 2:3 poster, title block with meta line "★ 8.5 · 12 episodes · 24m · 2024", genre pills, orange Start Watching pill + My List secondary + trailer outline, tabs (Episodes / Characters / Information) with orange underline, episode rows with number, title, filler tag, per-episode progress bar.
- Watch: CR-style dark player page — minimal top bar with back chevron and "Episode X of Y", episode strip below, prev/next buttons. Keep hls.js wiring and progress saving exactly as-is; restyle only.
- List / Season / Library / Settings: same CR treatment — chip tab filters, panel cards, consistent empty states using the existing `.empty-state` pattern.

## Output format
For every file you change or create, output the COMPLETE final file content under a header with its path relative to project root (e.g. `### src/pages/HomePage.svelte`). No diffs, no snippets, no "rest unchanged". Files not listed are untouched. End with a one-paragraph summary listing what changed and why, nothing else. Before finishing, re-read your output against the anti-slop checklist and the boundaries above and fix violations silently.
