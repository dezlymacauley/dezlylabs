| Tag        | Meaning                     | Example Use                               |
|-------------|-----------------------------|--------------------------------------------|
| `<header>`  | Intro section of a page or article | Contains site title, logo, nav             |
| `<nav>`     | Navigation menu             | Contains links to other parts of site      |
| `<main>`    | Main content area           | The core content, one per page             |
| `<section>` | Groups related content      | A block like “Articles” or “Projects”      |
| `<article>` | Standalone content          | Blog post, comment, or news item           |
| `<aside>`   | Secondary info              | Sidebars, related links, author info       |
| `<footer>`  | Closing section             | Copyright, contact info, links             |

_______________________________________________________________________________

The flow of the document:
```
html
 └── body
      ├── header
      │     └── nav
      │
      ├── main
      │     ├── (content area)
      │     │     ├── section (Featured)
      │     │     │     └── article(s)
      │     │     └── section (Categories)
      │     │           └── article(s)
      │     │
      │     └── aside (Sidebar)
      │
      └── footer
```
_______________________________________________________________________________
