## Svelte compilier process

Svelte is a **compilier**...NOT a library or Framework.

'Svelte 3.0 moves reactivity out of the component API and into the language.' Harris

Svelte removes the overhead of building and diffing vdom trees. It retains references to the DOM node in variables and can update their content directly after the change.

Notes taken from ["The Svelte Compilier Handbook"]('https://lihautan.com/the-svelte-compiler-handbook/#overview')

### Svelte syntax is a "superset" of HTML

Svelte implements its own parser for the Svelte syntax.

```
HTML syntax <div>
Curly brackets { data }
Logic blocks {#each list as item}
```

The source code is compiled in a 4 step process.

1. Parsing source code into Abstract Syntax Tree (AST)
2. Tracking references and dependencies
3. Creating code blocks and fragments
4. Generate code

The Svelte parser handles specially for \<script> and \<style> tags.

When the parser encounters a \<script> tag, it uses [acorn]('https://www.npmjs.com/package/acorn') to parse the content within the tag. When the parser sees a \<style> tag, it uses [css-tree]('https://www.npmjs.com/package/css-tree') to parse the CSS content.

Compilier parses and translates everything neccessary for the Javascript DOM logic. Each component has its DOM methods.

- I believe the compiled code is constructed in such a way that the compiled DOM methods will be able to process future user input. (???)
