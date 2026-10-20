# Personal Liquid Glass renderer (we.9)

This is a Windows Direct2D approximation of Apple's material, not Apple's shader
or a demonstrated pixel-perfect reproduction. The current renderer was rebuilt
after the earlier broad highlights, face magnification and circular corners did
not give the desired appearance. We have not run a paired native iOS image test.

## Geometry and optics

- The fence uses a rounded rectangle with order-2.5 superellipse corners. Each
  quarter has 64 segments; optical distance is measured against that polygon.
  Analytic normals smooth out facet highlights. This is our own approximation,
  rather than a port of Flutter's rounded-superellipse implementation. During
  folding, the order smoothly decreases to 2 as opposing corners meet, producing
  circular capsule ends without a sudden shape change. The material mask, Windows
  Composition path clip, outer shadow and selection ring share the same geometry,
  including the folded fence's vertical offset.
- The corner radius is 42 DIP. The optical bevel is 20 DIP, reduced for small
  controls. The center is flat: there is no whole-face magnification.
- For inward depth `s` and bevel width `b`, edge displacement is proportional to
  `1 - sqrt(1 - (1 - clamp(s / b, 0, 1))^2)`. Its maximum is 2.2 times the bevel
  width, limited to half the short side. Samples move inward along the normal.
  This produces a curved shoulder with displacement returning to zero at the
  flat face. Chromatic dispersion is disabled.
- The interior mixes in a 1.15 DIP Gaussian blur, fading toward the bevel where
  sharper wallpaper detail makes refraction visible. Small controls reduce this
  blur. This is subtle frost, not an opaque dark tint.
- A 1.2 DIP directional glint follows the shoulder, with 21% inward bleed over
  four times that width. It fades at light-tangent edges. Glint color derives
  from the displaced wallpaper, with a limited lift. A separate contour and
  short inner shadow are limited by a 5.5% opacity; there is no full-face black
  overlay. Existing user opacity/tint and floating-fence backing still apply.

Geometry, displacement and coverage/frost/light masks are cached by size, radius
and DPI. Wallpaper uploads, Gaussian blur, displacement, masks and compositing
run on Direct2D effects. Geometry maps are not recomputed per wallpaper frame.
The CPU compatibility renderer is retained and does not have these new optics;
verification images must explicitly select the GPU renderer.

Map generation skips the flat interior and reuses precomputed polygon segments.
The CPU shadow obtains antialiased coverage from the same corner polygon using
a binary sector lookup. Liquid Glass labels use adaptive foreground colors
without the former four-offset white/black halo.
The single-title hover state and fold glyph no longer add an independent glass
capsule or circular rim across the fence corners. Hover/press feedback remains
in the outer material and glyph color; multi-tab lenses retain their tab feedback.

For large dimensions with settled order-2.5 corners, raster coverage/frost/light
masks reuse a bounded set of CPU corner templates. Smaller fold heights still use
their exact morphed geometry. Tests compare stretched masks with direct generation
at fractional DPI and odd dimensions within one alpha value. Shadow templates are
large enough to reach the settled corner order too, and are compared with direct
rendering at full, intermediate and capsule heights.

Optical strength scales GPU displacement, contour and glints without rebuilding
geometry or replacing the real wallpaper. In we.9, strength also controls actual
material alpha. Zero strength skips glass drawing and uploads entirely: Windows
composites the real animated desktop through the window. Only a 3.5% white wash
remains under the title and icons. The we.8 frozen Gaussian/CrossFade branch was
removed. Full-strength glass retains its original optical model.
After focus loss capture stops immediately, while the material drains smoothly over
the existing compositor clock. Once settled there are no recurring material draws.
Restoration waits for the first fresh capture. Critically damped transitions use
1.8 second incoming / 1.6 second outgoing response parameters (omega = 6/response),
settling in roughly 2.6/2.3 seconds. Reversal inherits current strength and velocity.
The expanded
content raster is retained while folded and repainted only when invalidated or its
adaptive ink changes; folding no longer reallocates it as a one-pixel surface.

In we.9, expansion starts its 320 ms finite Hermite height motion only after content
raster preparation; collapse takes 240 ms. A fresh motion responds with a finite
initial velocity and decelerates to zero, while reversal inherits the sampled height
velocity. Chevron motion uses the same interpolation and duration. Content fade
uses the same time window instead of an independent 83 ms flash. Unchanged rounded
pixel heights do not send SetWindowPos. During height motion, incoming live frames
and optical strength update the stored material without immediately drawing it a
second time; the height pass presents it on its next frame.

## Sources and attribution

The bevel displacement model was adapted from Kyant0's Apache-2.0 licensed
[AndroidLiquidGlass / Backdrop shaders](https://github.com/Kyant0/AndroidLiquidGlass/blob/kmp/backdrop/src/commonMain/kotlin/com/kyant/backdrop/internal/Shaders.kt).
Its [continuous corner discussion](https://kyant.gitbook.io/backdrop/tutorials/smoother-rounded-corners)
also informed the shape choice.

The flat face, narrow directional glint, tangency fade and small inward bleed
were informed by whynotmake-it's Apache-2.0 licensed
[liquid_glass_renderer](https://github.com/whynotmake-it/flutter_liquid_glass/tree/e4086e5e38151fe65172225d79fe4a516585a933/packages/liquid_glass_renderer),
especially its `geometry_fragment.glsl` and `liquid_glass_final_render_core.glsl`.
That revision targets iOS 27 and contains a native Apple comparison harness.
Its test results do not establish fidelity for this Windows implementation.
We adapted formulas and material structure, not the entire Flutter renderer.

Both projects are Apache-2.0 licensed, as is PecoFence. The PecoFence source
license remains in the package. These modifications and the sources above are
recorded here; upstream authors do not endorse this personal build.

Apple's [Meet Liquid Glass](https://developer.apple.com/videos/play/wwdc2025/219/)
describes the intended adaptive behavior. It does not provide a portable public
shader for Windows.

## Validation

`cargo test -p pecofence-render --lib` covers the common silhouette, signed edge
distance, tangent joins, small controls, DPI invariance, flat center and edge
lighting. `gpu_glass_bench` exercises actual GPU output: opacity, neutral color,
live source replacement, negative monitor coordinates, scaled source mapping,
geometry cache invalidation and control refraction. Its checkerboard test expects
neutral frosting rather than an unchanged sharp center.

`liquid_glass_wallpaper` renders actual GPU preview images using saved wallpaper
frames. Uniform gray, rain and a sunset scene are used to inspect brightness and
edge artifacts. These previews do not substitute for checking the installed
window, its compositor clip and the user's visual acceptance.

For we.5, paired native debug builds with the same optimization profile, saved
wallpaper, 800 x 1000 pixel fence and six fold/unfold cycles measured resize draw
time at median 7.51 -> 3.97 ms and p95 12.20 -> 6.88 ms. Those figures exclude
the separate CPU shadow stage, whose median was 2.04 -> 2.20 ms. They are local
stage timings, not a release FPS guarantee. Native input checks exercised
hover-open followed by arrow-close, suppression of hover reopening after manual
collapse, and reversing an in-flight fold by clicking again. Geometry tests
cover intermediate fold heights and agreement between shadow coverage and clip.

The we.6 header change was compared before/after at 125% DPI over the same saved
wallpaper using the GPU preview's `title-hover` mode. It removes the separate
header capsule and fold-button circle without changing the outer optical model.

The we.7 GPU checkerboard regression checks sharp suspended pixels, opacity and
restoring optics at strengths 0/0.25/0.5/1 with one map build and one source upload.
On this machine the 800 x 298 resize benchmark's completed GPU+readback median
was 1.26 ms (we.6: 2.37 ms), and 1152 x 1066 was 3.43 ms (we.6: 4.89 ms).
These offscreen stage timings exclude native window and layered-shadow work.

Paired we.6/we.7 release runs used the same saved wallpaper, 800 x 1000 pixel
fence, 125% DPI and six fold/unfold cycles. The summed crop/draw/shadow stage
median was 6.218 -> 4.771 ms, p95 10.575 -> 8.240 ms and maximum
12.985 -> 9.810 ms (389/446 resize samples). These measure work performed for
each resize, not end-to-end display latency. Native cache checks additionally
verified that reopening reuses the full content surface and changing width while
folded causes exactly one content redraw on expansion.

For we.8, real GPU output over 32-pixel bars produced centre samples
193/208/224/255 at strengths 0/0.25/0.5/1, returning exactly to 193 on suspension.
The same geometry and source upload were reused throughout. One-pixel checker
detail remained neutral and opaque, and the existing GPU opacity, coordinate,
multi-monitor, device-reset and control tests passed. Unit tests cover fresh-frame
gating, slower restoration, outgoing settling and continuity of strength/velocity
on reversal. Native-window checks observed incoming strengths 0.030/0.587/1.0
and outgoing strengths 0.823/0.184/0.0. After settling, successive paused dumps
had identical GPU frame and wallpaper-upload counters and no material animation.

For we.9, GPU regression samples have alpha 0/64/127/255 at strengths
0/0.25/0.5/1. At zero strength, two different underlying target colors pass through
unchanged, with no extra glass frames, maps or uploads. Native tests cover focus
restoration during expansion, interruption, settled idle counters, retained content
rasters, folded width changes and repeated fold reversal. The latter ended at the
correct 700 x 1000 window size without another content redraw. Eight motion tests
passed, including finite endpoint, immediate first-frame response and velocity
continuity at reversal. Paired release runs completed six fold/unfold cycles each;
the old open started at 46 px from a 45 px shelf, while the new first presented
heights were 55–73 px. Maximum inter-resize gaps for the new opens were 13.02–16.81
ms. These are local HWND/update timings, not display-frame or perceived-smoothness
guarantees.

In we.10, Liquid Glass keeps a full-height composition drawing surface while
folding. Only the logical viewport and continuous root clip change; the surface
brush retains its physical size, so an old folded raster cannot be stretched into
the expanded window. The new lens frame is drawn before SetWindowPos exposes its
new bounds, and the matching WM_SIZE avoids a second draw. Size notifications
are reconciled against the HWND's actual size instead of an old message payload.
The content fade uses the same client clock as the height; completion stops any
opacity animation and rests the glyph, visibility, alpha and geometry together.
Relayout during a transition retains the expanded icon raster.
The client frame clock is capped at 60 Hz, including successful but unusually
fast compositor waits. This leaves the message loop time to handle input and
timers instead of filling it with repeated glass redraws.

Native checks with five fences, live WPE capture and the user's roll-up settings
exercise twenty hover expansions, twenty mid-fold reversals and twenty injected
old size notifications. Window/viewport dimensions and settled glyph/opacity
are checked, along with stable drawing-surface allocation and retained icon
rasters. A desktop capture was inspected for complete icon rows and the expanded
outline. This does not prove that the user's previously intermittent presentation
glitch is resolved; acceptance still requires testing the installed build.

After installing we.10 the user reported that the stuck expansion no longer
occurred, but the motion frame rate was low on their 240 Hz display. In we.11,
height changes and fence dragging request up to 120 Hz while live wallpaper
capture retains its 60 Hz target. A frame remains outstanding until UI processing
has finished: the worker cannot post further frame messages while one is queued
or running. This bounds queue pressure instead of limiting every interaction to
the slower material-update cadence. At rest the client clock returns to 60 Hz,
and still disarms completely when no animation or live wallpaper needs it.
The repeated native audit again passed all 81 state checks. Across 874 consecutive
expanding height updates, the median interval was 8.85 ms, p95 12.41 ms and maximum
17.04 ms. These are local HWND/frame-update timings, not a measurement of actual
display presentation or a guarantee of sustained 120 fps.
