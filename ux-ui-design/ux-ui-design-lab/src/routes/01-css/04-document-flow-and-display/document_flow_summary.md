# HTML Elements Default Display Types Reference

---

## `display: none`

Takes the HTML element out of the document flow.
The page will render as if the HTML element does not exist.

---

## `display: block`

This property means that:

1. The width of the HTML element will be the same as its parent container
2. The height of the HTML element will be determined by its children.
   Aka the HTML elements inside it.
3. You can override this with `width: value` and `height: value`

### HTML elements that have this property by default

**Text/Content Elements:**

- `<div>`, `<p>`, `<h1>` through `<h6>`, `<blockquote>`, `<pre>`, `<address>`

**List Elements:**

- `<ul>`, `<ol>`, `<li>`, `<dl>`, `<dt>`, `<dd>`

**HTML5 Semantic Elements:**

- `<header>`, `<nav>`, `<main>`, `<section>`, `<article>`, `<aside>`, `<footer>`

**Form Elements:**

- `<form>`, `<fieldset>`

**Other Block Elements:**

- `<hr>`, `<figure>`, `<figcaption>`, `<details>`, `<summary>`

---

## `display: inline`

This property means that:

1. The width of the HTML element will be determined by its content.
   It will only take up as much space as it needs.
2. The width and height of the HTML element will be determined by its children.
   You can't use `display: inline` with `width: value` and `height: value`

### HTML elements that have this property by default

**Text Formatting:**

- `<span>`, `<a>`, `<strong>`, `<em>`, `<b>`, `<i>`, `<u>`, `<small>`, `<sub>`, `<sup>`

**Code Elements:**

- `<code>`, `<kbd>`, `<samp>`, `<var>`

**Other Inline Elements:**

- `<abbr>`, `<acronym>`, `<cite>`, `<dfn>`, `<q>`, `<time>`, `<mark>`, `<ins>`, `<del>`, `<s>`

---

## `display: inline-block`

This property means that:

1. The width of the HTML element will be determined by its content by default
2. You CAN override this with `width: value` and `height: value` (unlike inline)
3. Elements flow horizontally like inline elements, but can have block-like properties

### HTML elements that have this property by default

**Form Controls:**

- `<input>`, `<button>`, `<select>`, `<textarea>`

**Media Elements:**

- `<img>`, `<canvas>`, `<svg>`, `<video>`, `<audio>`

**Embedded Content:**

- `<iframe>`, `<object>`, `<embed>`

---

## Special Display Values

**`display: table`:**

- `<table>`

**`display: table-row`:**

- `<tr>`

**`display: table-cell`:**

- `<td>`, `<th>`

**`display: table-caption`:**

- `<caption>`

**`display: list-item`:**

- `<li>` (block-level with list markers)

---

**Note:** These are browser default values. Any element's display property
can be overridden with CSS.

---
