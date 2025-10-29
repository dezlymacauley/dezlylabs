<!-- 

  `grid` enables the grid layout inside a container
  
  WARNING: Ensure that the items inside the container do not have a 
  fixed width or height like `w-20` or `h-20`.

  You wan't the grid property to dynamically control the width 
  and height of the items inside the container,
  as it uses those items to fill up the grid.

  `grid-cols-4` 

  This creates four colomns and will fill in the items from left to right

  `gap-x-2`. This adds a gap of 2 Tailwind CSS units of horizontal spacing 
  between each column.

  1 Tailwind CSS unit = 0.25 rem (4px), so `gap-x-2` is 0.5 rem (8px)

  rem = "root em" = relative to the root font size of the document
  By default in most browsers: 1rem = 16px

  16 px * 0.5 = 8 px

  `gap-y-2`. This adds a gap of 2 Tailwind CSS units of vertical spacing 
  between each column.
-->
<div class="grid grid-cols-4 gap-x-2 gap-y-2">
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
