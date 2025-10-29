<!-- 

  https://tailwindcss.com/docs/responsive-design

  There are five breakpoints by default, inspired by common device resolutions:
  Breakpoint prefix	Minimum width	CSS
  sm	40rem (640px)	@media (width >= 40rem) { ... }
  md	48rem (768px)	@media (width >= 48rem) { ... }
  lg	64rem (1024px)	@media (width >= 64rem) { ... }
  xl	80rem (1280px)	@media (width >= 80rem) { ... }
  2xl	96rem (1536px)	@media (width >= 96rem) { ... }

  `grid` enables the grid layout inside a container

  Tailwind uses “mobile-first,” 
  so the styles for sm apply to anything above 640px 
  unless overridden by a larger breakpoint.

1 column → fits small phones perfectly

2 columns (sm) → good for larger phones / small tablets

3 columns (md) → ideal for tablets and small laptops

4 columns (lg) → looks great on desktop monitors

gap-4 → nice balanced spacing between items (not too tight, not too loose)

This is pretty much the industry standard approach and works well for 90% of use cases. 

-->
<div class="
  grid grid-cols-1 

  sm:grid-cols-2 
  md:grid-cols-3 
  lg:grid-cols-4 

  gap-4"
>
  <div class="bg-purple-500">
    <p>Item 1</p>
  </div>

  <div class="bg-amber-500">
    <p>Item 2</p>
  </div>

  <div class="bg-red-500">
    <p>Item 3</p>
  </div>

  <div class="bg-blue-500">
    <p>Item 4</p>
  </div>

  <div class="bg-green-500">
    <p>Item 5</p>
  </div>

  <div class="bg-pink-500">
    <p>Item 6</p>
  </div>
</div>
