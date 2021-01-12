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

#### Basic Svelte-like compilier.

- based off of 'joshnuss' tiny compilier [repo](https://github.com/joshnuss/micro-svelte-compiler)

### In sequential order fallow the fallow the cli.js.

```
    #!/usr/bin/env babel-node
import FileParser from './src/FileParser'
import ScriptParser from './src/ScriptParser'
import TagParser from './src/TagParser'
import ComponentGenerator from './src/ComponentGenerator'
import CodeFormatter from './src/CodeFormatter'

const { code, tags } = FileParser.readFile(process.argv[2])
const { props, rest } = ScriptParser.parse(code)
const { nodes, listeners } = TagParser.parse(tags)
const output = ComponentGenerator.generate(props, nodes, listeners, rest)
const formatted = CodeFormatter.format(output)

console.log(formatted)


```

- #### Step One.
  - uses parse5: Parses HTML tags.
  - use the fs module to extract out the command like arguments which specify the file path to parse.
  - While reading the file run it throught the parse5.parseFragment() method. I believe this will provide all the data between the <> tags.
  - With all the parsed fragments you can now strip out(separate) anything that is a \<script> tag and standard \<html> tag by looping through...

```
Example from Docs...

    const documentFragment = parse5.parseFragment('<table></table>');

    console.log(documentFragment.childNodes[0].tagName); //> 'table'

// Parses the html fragment in the context of the parsed <table> element.
    const trFragment = parser.parseFragment(documentFragment.childNodes[0], '<tr><td>Shake it, baby</td></tr>');

    console.log(trFragment.childNodes[0].childNodes[0].tagName); //> 'td'
```

- output of this step is:

  1. a 'code' === string of the script tag data...
  2. a 'tags' === array of the rest of the HTMl tag identifiers.

  ```
    this is what parse5 and the Svelte compilier will output in a simplified way...it seperates out the code from inbetween the \<script> tags and the rest of the html tags in the parse5 parse objects.

        code:: let count = 0;

        const increment = () => {
                count += 1;
        };
  tags:: [
  {
    nodeName: 'h1',
    tagName: 'h1',
    attrs: [],
    namespaceURI: 'http://www.w3.org/1999/xhtml',
    childNodes: [ [Object] ],
    parentNode: { nodeName: '#document-fragment', childNodes: [Array] }
  },
  {
    nodeName: 'p',
    tagName: 'p',
    attrs: [],
    namespaceURI: 'http://www.w3.org/1999/xhtml',
    childNodes: [ [Object] ],
    parentNode: { nodeName: '#document-fragment', childNodes: [Array] }
  }
  ]
  ```

#### Step Two.

[use acorn to parse script tags](https://github.com/joshnuss/micro-svelte-compiler/blob/master/src/ScriptParser.js)

- use 'acorn.parse' to parse the code that was associted with \<script> tags. (formatted as a string) into an AST.
- parse(input, options) is the main interface to the library. The input parameter is a string, options must be an object setting some of the options listed below. The return value will be an **abstract syntax tree object** as specified by the ESTree spec. In this instance they use sourceType: 'module'
  - sourceType: Indicate the mode the code should be parsed in. Can be either "script" or "module". This influences global strict mode and parsing of import and export declarations.
- Once parsed you then 'walk the AST' ..

```
    walk (ast) {
    const props = []
    const rest = []

    ast.body.forEach(declaration => {
      if (declaration.type === 'ExportNamedDeclaration') {
        this.addExport(props, declaration.declaration)
      } else {
        rest.push(declaration)
      }
    })

    return { props, rest }
  },

 addExport (props, variableDeclaration) {
    variableDeclaration.declarations.forEach(decl => {
      props.push(decl.id.name)
    })
  }

```

- this step looks for 'types' in the AST tree that are 'ExportNamedDeclaration' which is anything that is using the 'export'....like in 'export default App'. There is another field labeled 'declaration' on the AST.

```
AstExplorer....acorn.


   {
      "type": "ExportNamedDeclaration",
      "start": 0,
      "end": 16,
      "declaration": {
        "type": "VariableDeclaration",
        "start": 7,
        "end": 16,
        "declarations": [
          {
            "type": "VariableDeclarator",
            "start": 11,
            "end": 15,
            "id": {
              "type": "Identifier",
              "start": 11,
              "end": 15,
              "name": "poop"
            },
            "init": null
          }
        ],
        "kind": "let"
      },
      "specifiers": [],
      "source": null
    },
```

- this step creates:
  1. 'props': an array that is all of the export variable names. In this case it's 'poop'.
  2. 'rest': an array with everything else. All other declarations...whole object I think.

#### Step Three

- parse the 'tags' from Step One...all the HTML identifiers.
