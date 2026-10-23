# MEROA motion

Only movement from Material 3 Expressive is applied. MEROA's palette, typography,
surfaces, layout and components remain in place; the requested active-dot colors,
reset display and settings controls are the deliberate visual changes.

## Mapping and implementation

React 19 / CSS in the existing Tauri 2 WebView; no motion/UI dependency added.
These are app-owned CSS animations, not an official Google web implementation.

References consulted on 2026-10-23:

- [Material motion physics](https://m3.material.io/styles/motion/overview/how-it-works).
- [AndroidX ExpressiveMotionTokens, token version v0_14_0](https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/tokens/ExpressiveMotionTokens.kt).
- [Material MotionTokens, token version v0_103](https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/tokens/MotionTokens.kt).

| Use | Expressive spring (damping ratio / stiffness) | Local settling interval |
| --- | --- | --- |
| Screen entrance, ring, content position | Default spatial: 0.8 / 380 | 500 ms |
| Press/release feedback, switch thumb | Fast spatial: 0.6 / 800 | 350 ms |
| Ring/menu appearance, number changes | Default effects: 1 / 1600 | 200 ms |
| Active-dot color, control state colors | Fast effects: 1 / 3800 | 150 ms |

The damping and stiffness values above are from the referenced Expressive tokens.
Durations and distances are MEROA's local adaptations, not claimed Google tokens.
Spatial motion can slightly overshoot; critically damped effects stay in range.
CSS `linear()` samples a unit-mass spring at 41 equally spaced times. For spatial
motion, with w = sqrt(stiffness), wd = w * sqrt(1 - damping²):

    progress(t) = 1 - exp(-damping*w*t) * (cos(wd*t) + damping*w/wd*sin(wd*t))

For effects (damping = 1):

    progress(t) = 1 - (1 + w*t) * exp(-w*t)

The last sample is snapped to 1. The values are precomputed in `motion.css`, so
there is no JavaScript spring solver or frame loop. Older engines fall back to
Material's cubic-bezier(0.2, 0, 0, 1); this is an approximation, not a spring.

CSS transitions retarget when interrupted. `@starting-style` and discrete display
transitions animate menu/ring entrance and exit on supported WebViews. Older
engines show/hide immediately. Screen changes animate the incoming content only,
without delaying navigation or animating native window size. Number changes use
an opacity effect; consumed dots never animate or change their existing color.

## Preferences, accessibility and performance

`MotionProvider` owns the persistent animations preference (default ON) and one
live `prefers-reduced-motion` subscription. Its document-level gate is set before
paint. OFF or reduced motion applies `animation: none` / `transition: none`,
which cancels ongoing work instead of hiding it. No animation timers, rendering
loops, delayed state changes or spring calculations run in application code.
The refresh spin class is also omitted when effective motion is disabled.

All motion is finite and triggered by interaction/data changes. There are no
decorative idle animations or persistent compositor hints. The existing clock
and native geometry synchronization remain functional, independently of motion.
The menu becomes hidden/inert immediately on closure. The switch retains its
stored ON choice when the system overrides it, with an explanatory settings note.

Dot and animation choices use versioned localStorage keys, following existing
MEROA preferences, and remain usable for the session if storage is unavailable.
At remaining percentage <= 10, critical red overrides every chosen active color;
the original consumed-dot fill and dot-count rounding are preserved.

## Verification (2026-10-23)

- `npm test`: 20 passing tests, including preference defaults/persistence/storage
  failure, active counts at boundaries, critical override for every color,
  unchanged consumed-dot markup, reset visibility and countdown progression.
- `npm run build`: TypeScript and production Vite build pass.
- `npm run tauri:build`: Windows release build passes.
- Chromium/Playwright, Italian, 400 x 650 viewport, fixed 372px desktop popover:
  visual comparison against the original HEAD UI, unchanged hero geometry,
  all active colors, 10% override, centered reset and return, OFF/ON, reload
  persistence, live system reduced motion, menu keyboard focus and refresh pass.
  No console warnings, errors or framework overlays. OFF has zero document
  animations; idle frame/interval call counters remain unchanged. Advancing the
  browser clock by one hour still updates the reset value while OFF.

The Browser plugin/skill is not available in this session, so the bundled
Playwright runtime is used. Sandbox localhost navigation was unavailable;
the production files were served through Playwright request fulfillment at
`http://meroa.test/`, with a test-only Tauri IPC/data fixture. No test data or
browser instrumentation is included in the application. This validates the
frontend, not the native tray/IPC on a running Windows WebView. Native runtime,
screen-reader testing and measurements on low-powered hardware remain unverified.
The motion mapping is a documented CSS approximation, not Google certification.
