# Matte — context.md

**Document purpose.** Establish a clear, concrete, and realistic foundation for building *Matte*, a fully custom, multi‑platform web browser with original engines (HTML/CSS layout, JavaScript, networking, graphics, and UI). This doc aligns product vision with the technical plan, scope, risks, milestones, engineering practices, and org shape.

**Audience.** Founders, engineering leads, security, product/design, legal.

**Status.** Draft v0.1 — living document. Update as decisions land; record changes via ADRs (Architecture Decision Records).

---

## 1) Vision & Strategy

### 1.1 Vision

Matte is a fast, private, and developer‑friendly browser that is *not* a Chromium or Gecko derivative. It advances the state of the art in safety (memory‑safe by default), compatibility (laser‑focused spec compliance subset → iterative expansion), and performance (predictable frame pacing, low power).

### 1.2 Product Pillars

1. **Safety first**: memory‑safe engines where feasible (Rust‑first), strong sandboxing, exploit mitigations.
2. **Performance & power**: predictable 60/120Hz compositing; low wakeups; minimal background work.
3. **Privacy by default**: strict tracking protection, storage partitioning, minimal telemetry.
4. **Developer joy**: modern DevTools, clean remote debugging protocol, excellent diagnostics.
5. **Clarity over scope**: ship a *subset* of the web well, broaden with telemetry‑guided demand.

### 1.3 Non‑Goals (Phase 0–2)

* No legacy plugins/NPAPI/ActiveX.
* No DRM/EME or proprietary CDM in early phases.
* No H.264/HEVC in early phases (codec patents). Prefer royalty‑free first.
* No full WebGL2/WebGPU initially; start with static 2D/animation + later incremental 3D.
* No broad extension compatibility at first; small, safe Matte Extension API (MEA) only.

---

## 2) Success Metrics

* **Compatibility**: % pass rate on targeted Web Platform Tests (WPT) subset; top 100 target sites functional (internal list).
* **Performance**: FCP/LCP/INP targets vs. reference baselines on a fixed corpus; cold start < 500ms (desktop, release build).
* **Stability**: < 1 crash / 10,000 hours; zero known RCEs in stable.
* **Power**: background tabs < 1% CPU on idle pages; video playback power draw within 10% of OS media apps (when implemented).
* **Privacy**: storage partitioning on by default; fingerprinting surface audit score (internal rubric) ≥ 90/100.

---

## 3) Platform & Policy Considerations

* **Desktop**: Windows 11+, macOS 13+, Linux (Wayland primary; X11 fallback). Tier‑1 support.
* **Mobile**:

  * Android: tier‑2 after desktop MVP.
  * iOS/iPadOS: engine policy constraints vary by region; treat as tier‑3 and revisit after MVP.
* **Distribution**: Signed installers, delta updates, crash reporting opt‑in, privacy‑preserving telemetry.

---

## 4) Architecture Overview

```
+-----------------+     +----------------------+     +------------------+
|   Browser UI    |<--->|  Browser Process     |<--->|  GPU/Compositor  |
+-----------------+     |  (privileged)        |     +------------------+
        ^               |  - Tab mgr            |              ^
        |               |  - Profile/Settings   |              |
        v               |  - Extension host     |              v
+-----------------+     +----------------------+     +------------------+
|  Renderer Proc  |<--->|  Network Process     |<--->| OS / Drivers     |
|  (per-site)     |     |  (TLS, cache, HTTP)  |     +------------------+
|  - Parser/DOM   |     +----------------------+
|  - Style/Layout |
|  - JS VM        |
|  - Scheduler    |
+-----------------+
```

**Process model.** Multi‑process, site‑isolated renderers; dedicated network and GPU processes; optional utility processes (font, audio, spellcheck) to reduce attack surface.

**Language choices.** Rust for engines (parser, DOM, layout, networking, scheduler, parts of JS runtime), C/C++ **only** where platform APIs require it (e.g., macOS Objective‑C bridges, Windows COM).

**Sandboxing.** Platform sandboxes per OS (Win: AppContainer + Win32k lockdown; macOS: sandboxd + hardened runtime; Linux: namespaces/seccomp/bpf). Brokering via a privilege boundary IPC.

**IPC.** Typed message schema (Cap’n Proto/FlatBuffers or custom minimal) with versioning; backpressure & prioritization (input > decode > paint > preload > idle).

---

## 5) Engines & Subsystems (Deep‑Dive)

### 5.1 HTML Parser & DOM

* **Tokenizer**: spec‑driven states; error‑tolerant; locations for source mapping.
* **Tree builder**: insertion modes; foster parenting; templates; document.write support (Phase 2).
* **DOM**: Shadow DOM (closed in Phase 1, open later), Custom Elements (Phase 2), DOM Events, MutationObserver.
* **Navigation**: BFCache (back/forward cache) later; start with classic unload/reload.
* **History**: pushState/replaceState; fragment/nav timing hooks.

### 5.2 CSS & Style System

* **CSSOM** with cascade/inheritance/computed values.
* **Selector engine**: O(N) matching with bloom‑filter style ancestor hints; \:hover/\:active/\:focus states; \:has() later.
* **Layout**: Block & inline formatting contexts, floats, absolute/fixed positioning, Flexbox (Phase 1), Grid (Phase 2+), fragmentation later.
* **Typography**: font fallback, text shaping (HarfBuzz‑class functionality; evaluate build vs buy), bidi/line‑break rules (Unicode), kerning/ligatures, hyphenation.
* **Compositing**: layers tree, stacking contexts, transforms, opacity; sticky/position\:sticky.
* **Painting**: retained display lists; partial invalidation; sub‑pixel AA; color‑managed (sRGB first, P3 later).

### 5.3 Graphics & Compositor

* **Abstraction**: GPU‑agnostic backend (wgpu/Vulkan/Metal/D3D12), fallback to software raster (llvmpipe‑class) for headless and VMs.
* **Scheduler**: V‑sync aligned; separate input, animation, and raster/compose phases; frame budget accounting.
* **Effects**: clip, scroll, opacity, transforms; filters later; SVG support basic -> advanced.

### 5.4 JavaScript Engine ("MatteJS")

* **Parser**: hand‑rolled Pratt or LR; source maps; UTF‑8.
* **Bytecode VM**: register‑based; inline caching; baseline JIT optional (later) with tiering (interp → baseline → optimizing). Start interpreter‑only for portability.
* **GC**: generational, moving, incremental + concurrent marking; nursery for short‑lived DOM wrappers.
* **ES Features**: Target ES2021+ core; TypedArrays; Intl later; BigInt in Phase 2; Proxy in Phase 2.
* **Bindings**: WebIDL generator to create fast DOM bindings; microtask queue integration; structured clone.
* **Wasm**: minimal interpreter (no JIT) in Phase 2.

### 5.5 Networking Stack ("MatteNet")

* **Protocols**: DNS, HTTP/1.1 and HTTP/2 in MVP; HTTP/3/QUIC Phase 2.
* **TLS**: TLS 1.3, OCSP stapling, certificate pinning list, HSTS preload list ingestion.
* **Cache**: HTTP cache (disk + memory), cache partitioned by top‑level site; validation & revalidation policy.
* **Fetch/XHR**: fetch algorithm, CORS, CORP, COOP/COEP; service workers Phase 2.
* **Sockets**: non‑blocking I/O; evented reactor (epoll/kqueue/IOCP).

### 5.6 Storage & Persistence

* **Cookies**: same‑site defaults (Lax), partitioned by top‑level site; cookie jar encryption at rest.
* **Web Storage**: localStorage/sessionStorage MVP; IndexedDB Phase 2; CacheStorage with SWs in Phase 2.
* **Permissions**: unified prompt system; sane defaults; durable decisions persisted per site + expiry.

### 5.7 Media

* **Audio**: Audio output (WASAPI/CoreAudio/PulseAudio); WebAudio Phase 2.
* **Video**: Software decode for royalty‑free first (VP9/AV1/Opus inside WebM/Matroska). Hardware decode later via platform decoders. No EME initially.
* **Text tracks**: WebVTT later; Picture‑in‑Picture later.

### 5.8 Accessibility (A11y)

* **Tree**: platform accessibility tree export (UIA/AXAPI/AT‑SPI); focus navigation; caret browsing; zoom; high contrast.
* **ARIA**: core roles/states supported in Phase 1.

### 5.9 Security & Privacy

* **Security model**: strict same‑origin policy; site isolation; CSP; mixed‑content blocking; CORB.
* **Sandbox**: per‑renderer least privilege; brokered file/network access; content filters.
* **Mitigations**: CFI, ASLR, CET (Windows), hardened runtime (macOS), seccomp (Linux). Prefer memory‑safe code.
* **Fuzzing**: coverage‑guided fuzzers for parser/DOM/JS/bindings; WPT fuzz harness; continuous.
* **Privacy**: DoH/ECH later; GPC signal; fingerprint surface minimization; prefetching off by default.

### 5.10 DevTools & Diagnostics

* **Inspector**: DOM/CSS inspector, box model, computed styles, layout overlays.
* **Console**: runtime eval in isolated world; source maps; stack traces with async cause links.
* **Network**: waterfall, headers, timing, HAR export.
* **Performance**: flamegraphs of JS + layout/composite; FPS meter; memory snapshots.
* **Remote debugging**: Matte DevTools Protocol (MDP) over WebSocket/pipe; headless mode.

### 5.11 Extensions (MEA)

* **Model**: minimal permissioned APIs: tabs, storage (scoped), network hooks (limited), content scripts in isolated worlds.
* **Packaging**: signed zips; manifest v1; watch for future compatibility bridges to WebExtensions.
* **Security**: review + store signing; granular prompts.

---

## 6) Product Scope: MVP → Beta → 1.0

### 6.1 MVP (Desktop) — "Alpha"

**Goal**: Render a curated corpus of modern content with strong stability & privacy; developers can inspect and debug.

**Feature subset**

* HTML: parser, core DOM, forms (basic), navigation, History API.
* CSS: selectors up to level 4 subset (no \:has initially), block/inline/Flexbox, transforms, opacity, sticky, media queries.
* JS: ES modules, promises/microtasks, fetch/XHR, timers, events, workers (dedicated) later.
* Net: HTTP/1.1, HTTP/2, TLS 1.3; cache; CORS; HSTS.
* Storage: cookies (partitioned), localStorage.
* Graphics: GPU compositing; SVG basic; canvas 2D.
* DevTools: Elements/Styles, Console, Network.
* A11y: platform trees; keyboard nav; zoom.
* Privacy: tracker blocklist (curated), GPC, partitioned storage.

**Out of scope for MVP**: service workers, push, notifications, IndexedDB, WebRTC, WebGL/WebGPU, EME, clipboard write, screen capture.

### 6.2 Beta

* Add Grid, WebFonts full (unicode‑range), font loading API, IndexedDB, dedicated workers, service workers (fetch only), HTTP/3, WebSocket, WebVTT, picture decoding, lazy image decode; Wasm interpreter; WebGL 1 minimal.

### 6.3 1.0

* Stabilize performance/power; add site isolation hardening, basic extensions store, crash‑free rate targets, localization, updater GA.

---

## 7) Milestones & Timeline (illustrative)

> **Reality check**: A from‑scratch browser is a *large, multi‑year* effort. Below is an aggressive but staged plan for a 25–40 person core team.

* **M0 (0–3 months)** Foundation

  * Bootstrapping repo, build, CI/CD, style/lints; scaffolding for processes (browser/net/gpu/renderer).
  * Minimal window + renderer embedding + clear/composited background; IPC skeleton; basic crash reporter.
  * HTML tokenizer, tiny DOM; CSS parser; primitive block layout; software raster; simple HTTP client.
  * ADR‑000..00N capturing foundational choices.

* **M1 (4–9 months)** First pixels & navigation

  * Functional HTML parser/tree‑builder; CSS cascade/computed values; inline/block layout; events; links/forms.
  * GPU compositor, display lists, tiled raster; TLS 1.3; HTTP cache; cookies.
  * JS interpreter (modules, promises); fetch; CORS; localStorage.
  * Elements/Styles/Console DevTools; basic A11y export.

* **M2 (10–15 months)** Developer Preview

  * Flexbox; transforms; SVG basic; canvas 2D; input/IME; text shaping; font fallback; bidi.
  * HTTP/2; HSTS preload ingestion; partitioned storage; tracker blocking.
  * Stability hardening; fuzzing across parser/DOM/JS.
  * Site corpus v1 functional; dogfood builds.

* **M3 (16–24 months)** Beta

  * Grid; WebFonts full; IndexedDB; service workers (fetch); WebSocket; Wasm interp; WebVTT; initial WebGL 1.
  * HTTP/3; power optimization; crash rate targets; extensions v1; updater GA; localization.

* **M4 (24–30 months)** 1.0

  * Performance polish; a11y conformance drive; docs; release.

---

## 8) Engineering Practices

* **Repo**: monorepo; cargo workspaces (engines) + CMake/Bazel for platform glue as needed.
* **CI**: tiered (presubmit linters/tests; postsubmit perf/regression; nightly fuzzing; WPT runners).
* **Testing**: unit → integration → WPT subset; capture minimal repros for all crashes; golden image tests for layout.
* **Code health**: rustfmt, clippy, clang‑tidy; sanitizers in debug; pre‑commit hooks; mandatory code review.
* **Perf discipline**: perf budgets per patch; traces archived; bisection bots; perf sheriff rotation.
* **Security**: threat modeling (STRIDE) per feature; code review checklist; embargo protocol.

---

## 9) Spec & Compatibility Strategy

* **Spec tracking**: map MVP features to WHATWG/ECMA/TC39 specs; maintain a living *Compat Matrix* for top sites.
* **WPT**: fork + pin a snapshot; enable only targeted directories initially; track pass/fail/skipped with bug links.
* **Interop posture**: no vendor prefixes; behind flags for experimental features; origin trials *later*.

---

## 10) Updater, Telemetry, and Crash Reporting

* **Updater**: signed, delta, background; channels: Nightly/Dev/Beta/Stable; key rotation plan; rollback.
* **Telemetry**: opt‑in; coarse metrics only (startup time, crashes, WPT pass %, page load timing on our test corpus); no URL/path collection without explicit research builds.
* **Crash**: minidumps with symbol server; privacy scrubber; SOC alerting for spikes.

---

## 11) Privacy Model

* Storage partitioning by top‑level site (cookies, cache, storage).
* Block third‑party cookies by default.
* Resist fingerprinting: reduce high‑entropy surfaces (canvas readback prompts, font enumeration limits, UA string frozen + client hints minimal).
* Private windows: ephemeral storage; no history/cache writes; stricter API availability.
* Network: preconnect/prefetch off by default; proxy support; DoH/ECH later; GPC signal.

---

## 12) Security Model

* **Isolation**: renderer per site instance; strict origin checks; COOP/COEP; CORB.
* **Sandbox**: deny‑by‑default syscalls; filesystem brokerage; clipboard/camera/mic prompts (later).
* **Supply chain**: vendored third‑party code, reproducible builds, SBOM per release.
* **Fuzzing**: continuous corpus growth; test case minimization; crash triage SLA.

---

## 13) UI/UX Principles

* Native platform look with light theming; no heavy chrome.
* Tab model: compact tabs, quick‑switcher (Ctrl/Cmd+K), vertical tabs optional.
* Omnibox: local history/bookmarks suggestions; *no* online suggestions by default.
* Downloads panel; permissions prompts; site info panel with clear storage/permission toggles.
* Accessibility: keyboard‑first, descriptive labels, ARIA in UI.

---

## 14) Legal & Licensing Notes (non‑exhaustive)

* **Codecs**: Start with royalty‑free (Opus, Vorbis, VP9, AV1) to avoid licensing exposure; consider H.264/MP3 later with counsel.
* **Trademarks**: file for *Matte*, iconography, wordmarks.
* **Patents**: defensive publication for novel engine components; consult on web standards IPR policies.
* **Privacy**: adopt clear privacy policy; GDPR/CCPA assessments; data minimization.
* **Third‑party deps**: prefer permissive OSS; track licenses in SBOM.

---

## 15) Team & Roles (initial)

* **Founding Eng Leads**:

  * Rendering/Layout, JavaScript VM, Networking, Graphics/Compositor, Security, DevTools, Platform (Windows/macOS/Linux), Build/Release.
* **Product/Design**: PM, UX, Visual/Brand.
* **QA/Perf**: test infra, WPT lead, perf sheriffs.
* **Security/Privacy**: threat modeling, fuzzing, incident response.
* **Legal/Compliance**: licensing, policy.
* **Developer Relations**: docs, samples, site‑compat outreach.

---

## 16) Risk Register & Mitigations

* **Scope risk**: a custom engine is multi‑year. → Scope MVP tightly; stage features; dogfood on a corpus.
* **Compat risk**: the modern web relies on rarely‑documented edge cases. → WPT + real‑world top sites + compat fixes queue.
* **Performance risk**: GC pauses, layout thrash, jank. → Structured schedulers, incremental GC, layout invalidation discipline.
* **Security risk**: browser engines are attack magnets. → Memory‑safe code, sandboxing, fuzzing, bug bounty (later).
* **Legal risk**: codecs/DRM distribution. → Royalty‑free first; counsel review for proprietary additions.
* **Hiring risk**: niche expertise required. → Advisory board, contractors for subsystems, grow juniors with strong code review.

---

## 17) Build & Release Machinery

* **Build**: cargo + cmake glue; hermetic toolchains; PGO/LTO for release; symbol servers per channel.
* **CI**: GitHub Actions/Buildkite; Windows/macOS/Linux runners; artifact store; notarization (macOS).
* **Packaging**:

  * Windows: MSIX/winget;
  * macOS: notarized .pkg;
  * Linux: Flatpak first, then DEB/RPM.
* **Channels**: Nightly (auto‑update daily), Dev (weekly), Beta (6‑8 weeks), Stable (8–10 weeks).

---

## 18) Developer Ecosystem

* **Docs**: docs.matte/ with architecture, embedding, devtools protocol, extension API.
* **Samples**: minimal browser embedding sample, extension samples, devtools client reference.
* **Bug/Feature process**: public tracker; triage rotations; compat shims policy.
* **Contribution**: coding standards; CLA if needed; code of conduct.

---

## 19) Internal Test Corpus (initial idea)

* 100–200 pages covering: news, retail, docs, social embeds, maps (static), code hosting, docs sites, dashboards.
* Include pathological cases: deeply nested tables, long text, huge images, CSS stressors, JS frameworks (React/Vue minimal), WASM hello‑world (later).
* Curate *no‑JavaScript* versions to isolate layout from script issues.

---

## 20) Component Backlog (MVP‑critical)

* [ ] HTML tokenizer/tree builder complete for core modes
* [ ] DOM Core + Events + MutationObserver
* [ ] CSS parser + cascade + selectors
* [ ] Layout: block/inline + Flexbox + transforms + sticky
* [ ] Text: shaping, bidi, fallback
* [ ] Display lists + tiled raster + GPU compositor
* [ ] JS interpreter + modules + promises
* [ ] DOM bindings + WebIDL generator
* [ ] Fetch/XHR + CORS + cache + cookies
* [ ] TLS 1.3 + HTTP/2
* [ ] Cookies (partitioned) + localStorage
* [ ] DevTools: Elements/Styles/Console/Network
* [ ] A11y tree export (Win/macOS/Linux)
* [ ] Crash reporter + updater (Dev channel)
* [ ] Site corpus harness + WPT subset runner

---

## 21) Naming & Brand Notes

* **Name**: *Matte* — connotes non‑glossy, minimal glare, clarity over shine.
* **Tone**: calm, careful, technical.
* **Icon**: matte surface/flat disc motif; high‑contrast variants for a11y.

---

## 22) Open Questions (to resolve via ADRs)

1. **Graphics backend**: adopt *wgpu* vs. custom thin backends per OS?
2. **Text shaping**: build in‑house vs. depend on HarfBuzz (permissive but external)?
3. **JS JIT**: interpreter‑only first 12–18 months? How/when to ship JIT per OS policy?
4. **Service workers**: when to enable; what perf budget and cache limits?
5. **Extension model**: MEA forever or a WebExtensions‑compat layer later?
6. **Mobile**: is Android a priority before 1.0 desktop?
7. **Crash reporting**: self‑host vs. third‑party.

---

## 23) Appendices

### 23.1 ADR Template

* **Context**: What problem are we solving?
* **Decision**: What did we choose and why?
* **Alternatives**: Other options considered.
* **Consequences**: Trade‑offs/risks.
* **Links**: Issues, prototypes, benchmarks.

### 23.2 Definition of Done (per feature)

* Spec mapped + tests written/passing (WPT where applicable).
* Perf budget met (numbers documented).
* Security review complete; threat model doc updated.
* Telemetry hooks added (if any) and documented.
* Docs written; DevTools supports debugging the feature.

### 23.3 Coding Standards (excerpt)

* Rust: 2021 edition+, deny(unsafe\_code) except audited modules; clippy pedantic enabled.
* C/C++ bridges: modern C++17+, RAII, no raw new/delete, sanitizers on in CI.
* IPC messages versioned; backward compatibility for ≥ 2 stable releases.

---

*End of v0.1.*
