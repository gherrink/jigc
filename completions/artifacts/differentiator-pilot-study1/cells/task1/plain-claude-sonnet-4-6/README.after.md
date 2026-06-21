# UI-Doc Core

This is the heart of UI-Doc. The following steps will be performed:

- Take source text
- identify blocks and parse them
- transform the blocks into a UI-Doc context
- use a renderer to output the context

This is how your dock blocks can look like. The syntax is similar to JS-Documentation blocks.

```js
/**
 * Will be interpreted as a page.
 *
 * @page typography Typography
 */

/**
 * Will be interpreted as a section on the page `typography`. Giving an example of default typography formats.
 *
 * @location typography.format Format
 * @example
 * <span>Normal Text</span><br>
 * <strong>Strong Text</strong><br>
 * <small>Small Text</small><br>
 * <em>Emphasis Text</em><br>
 * <i>Italic Text</i>
 */
```

You will use different tags to define how and where your block will be displayed in the documentation. Tags look like the following `@tag-name[ {your-type}][ name][ description]`.

## Available Tags

Here is a list of all pre existing tags. If you like you can add your custom tags.

The tags are separated into two different roles:

- **placement**: where in the documentation should your block be displayed (one kind of placement is always required)
- **display**: what/how should your block be displayed

| Tag       | Role      | Description                                                       |
| --------- | --------- | ----------------------------------------------------------------- |
| @code     | display   | specify the code that will be displayed                           |
| @example  | display   | add example and code to block. The example will show how it looks |
| @hideCode | display   | remove code from block                                            |
| @location | placement | combine tag page and section                                      |
| @order    | placement | define sorting between pages/sections                             |
| @page     | placement | create page or give page where your block should be shown         |
| @section  | placement | define give section where your block should be shown              |
| @color    | display   | define a color that is used in your style                         |
| @space    | display   | define a space variable used in your style                        |
| @icon     | display   | define a icon used in your style                                  |

Please note that you can combine tags to get an different outcome. Please see the documentation of the different tags for more details.

### @example

The example code will add an example and code. The example will show the viewer how your component looks like. The code will be displayed as copyable and readable code, so the viewer can see what to do to get the example outcome.

```js
/**
 * Will display code and example
 *
 * @example
 * <span>Normal Text</span><br>
 * <strong>Strong Text</strong><br>
 * <small>Small Text</small><br>
 * <em>Emphasis Text</em><br>
 * <i>Italic Text</i>
 */

// TODO @example {modifier} change modifier class
// TODO @example {modifier|js} change code type (default html)
```

![Code and it's output](../../doc-assets/core/example.png)

### @code

The code tag will add readable code.

```js
/**
 * Will display code.
 *
 * @code
 * <div class="code-example"></div>
 */
```

![Code tag result](../../doc-assets/core/code.png)

```js
/**
 * If used in combination with `@example` the code from `@code` will override the example code.
 * Comes in handy if you like to hide some inline styling or additional html-tags that are only for better displaying.
 *
 * @example
 * <div class="code-example" style="max-width: 200px"></div>
 *
 * @code
 * <div class="code-example"></div>
 */

// TODO @code {js}
// TODO @code {html} is default
```

![Code with example tag result](../../doc-assets/core/code_with_example.png)

### @hideCode

Can be used to hide code if you only want to add showcase but no code block.

```js
/**
 * Only add showcase of different typos.
 *
 * @example
 * <span>Normal Text</span><br>
 * <strong>Strong Text</strong><br>
 * <small>Small Text</small><br>
 * <em>Emphasis Text</em><br>
 * <i>Italic Text</i>
 * @hideCode
 */
```

![example with hideCode tag result](../../doc-assets/core/code_with_example.png)

### @page

Create a page or give the pace on witch your block should be shown on.

```js
/**
 * Will create a `Typography` page with the key `typo`. The key can be used in other placements for reference.
 *
 * @page typo Typography
 */

/**
 * Will display the section `sizes` on the page with the key `typo`.
 *
 * @page typo
 * @section sizes Different Font Sizes
 */

/**
 * Same as above. Will display the section `sizes` on the page with the key `typo`.
 *
 * @location typo.sizes Different Font Sizes
 */
```

### @section

Create a documentation section.

```js
/**
 * Will display the section `sizes` on the page with the key `typo`.
 *
 * @page typo
 * @section sizes Different Font Sizes
 */

/**
 * You can nest sections in sections by using points and section keys.
 * Will display the section `small` inside the section `sizes` on the page `typo`.
 *
 * @page typo
 * @section sizes.small A small typo variation
 */
```

### @location

A combination of `@page` and `@section`.

```js
/**
 * Will display the section `sizes` on the page with the key `typo`.
 *
 * @location typo.sizes Different Font Sizes
 */

/**
 * You can nest sections in sections by using points and section keys.
 * Will display the section `small` inside the section `sizes` on the page `typo`.
 *
 * @location typo.sizes.small A small typo variation
 */
```

### @order

Define the order who pages or sections should be ordered by giving a number. If the order is equal an alphabetic order will be used.

```js
/**
 * @page bar Bar
 * @order 2
 */

/**
 * @page foo Foo
 * @order 1
 */

// order will be Foo | Bar

/**
 * @location test.bar Bar
 * @order 2
 */

/**
 * @location test.foo Foo
 * @order 1
 */

// order will be Foo | Bar. Equal to the example above
```

### @color

Define colors you are using in your layout.

```js
/**
 * The colors used in this style.
 *
 * @color {0 0 0|255 255 255} --color-black | black
 * @color {20 33 61|255 255 255} --color-blue | blue
 * @color {252 163 17} --color-yellow | yellow
 * @color {229 229 229} --color-gray | gray
 * @color {255 255 255} --color-white-rgb - white rgb
 * @color {#fff} --color-white-hex white hex
 * @color {--color-white} --color-white-var white var
 * @color --color-white white var without value
 */
```

![color tag result](../../doc-assets/core/color.png)

You can define multiple colors in on codeblock the colors will then be displayed together. The color tag expects as type the rgb or hex value of the color you are going to use. As a second type you can define the font color that should be used for this color. A variable name and description also need to be given, both can be separated by `|`, `-` or just whitespace.

If you provide a custom style sheet with custom properties for variables you can use these directly.

```css
:root {
  --color-white: 255 255 255;
}
```

### @space

Define spacings you are using in your layout.

```js
/**
 * The space definitions.
 *
 * @space {0.5} --space-xs | XS
 * @space {0.8} --space-sm | SM
 * @space {1} --space-normal | Normal
 * @space {1.2} --space-md | MD
 * @space {1.6} --space-lg - LG
 * @space {2.4} --space-xl XL
 */
```

![space tag result](../../doc-assets/core/space.png)

You can define multiple spaces in on codeblock the paces will then be displayed together. The space tag expects as type a spacing value, the variable name and description. Like in the color tag you can separate variable name and description by `|`, `-` or just whitespace. The given spacing value will be used when displaying, to make the targeted space visible.

Note: Depending on your renderer the spacing value will be used differently. When using the default renderer the spacing value will be multiplied with the spacing unit.

### @icon

Define icons you are using in your icon font.

```js
/**
 * Some icons used.
 *
 * @icon {e900} --icon-chevron-down - chevron-down
 * @icon {e901} --icon-chevron-left - chevron-left
 * @icon {--icon-chevron-right} --icon-chevron-right | chevron-right
 * @icon --icon-chevron-up | chevron-up
 */
```

![icon tag result](../../doc-assets/core/icon.png)

You can define multiple icons in on codeblock the icons will then be displayed together. As type a css variable or the char code can be given. Variable name and description can be separated by `|`, `-` or just whitespace.

Please not that you will require a custom style to define the `@font-face` for your icon font and set the `--icons-font-family` variable to your icon font name.

```css
@font-face {
  font-family: icons;
  font-weight: normal;
  font-style: normal;
  font-display: block;
  src: url('fonts/icons.eot');
  src:
    url('fonts/icons.eot#iefix') format('embedded-opentype'),
    url('fonts/icons.ttf') format('truetype'),
    url('fonts/icons.woff') format('woff'),
    url('fonts/icons.svg#icons') format('svg');
}

:root {
  --icons-font-family: icons;
}
```

## Integration

UI-Doc can be used in any context you want it to run. There are already integrations for:

- [Rollup](../rollup/README.md)
- [Vite](../vite/README.md)

But you can write your own integration or just write a node script.

### Short example

To use UI-Doc in a node script your code need something like this

```js
import fs from 'node:fs/promises'
import path from 'node:path'
import { UIDoc } from '@ui-doc/core'

// const renderer = ... // create a renderer instance, this depends on the renderer you want to use
const outputDir = './dist'
const filePath = path.resolve('./my-css-file-to-source.css')

// create a UI-Doc instance
const uidoc = new UIDoc({
  renderer,
  // ... other options
})

// read the file content
const content = await fs.readFile(path.resolve(filePath), 'utf8')

// create a new source, by giving the filename and the file content
uidoc.sourceCreate(filePath, content)

// use the output function to get all files that should be created
await uidoc.output(async (fileName, content) => {
  await fs.writeFile(`${outputDir}/${fileName}`, content, 'utf8')
})
```

## Options

| Name | Required | Type | Description |
| --- | --- | --- | --- |
| blockParser | no | BlockParser | Change implementation of the parses that interprets the source and creates blocks for the UI-Doc. |
| generate | no | object of functions | Functions that will generate content for the renderer. |
| renderer | yes | Renderer | The renderer that should be used to generate the output. |
| texts | no | object of texts | Texts used by the default generate functions |

### Texts

| Name      | Description                                           |
| --------- | ----------------------------------------------------- |
| copyright | Used in footer text to display copyright information. |
| title     | Title of your UI-Doc                                  |

### Generate functions

| Name | Return Type | Params | Description |
| --- | --- | --- | --- |
| exampleTitle | string | ExampleContext | Create page title for examples |
| footerText | string |  | Text for the footer |
| homeLink | string |  | Link to the homepage (frontpage) |
| logo | string |  | Logo you want to display, give text, html or an svg |
| menu | {active: boolean, href: string, order: number, text: string}[] | menu array, pages array | Create/manipulate the menu |
| name | string |  | Name of your UI-Doc |
| pageLink | string | page context | Link to a page |
| pageTitle | string | page context | Title of a page |
| resolve | string | uri: string, context: string | Change/manipulate a uri |

You can change generate functions in two ways:

```ts
// set over options
const uidoc = new UIDoc({
  generate: {
    footerText: () => 'Custom Footer Text',
  },
})

// using a function
uidoc.replaceGenerate('name', () => 'MyUIDoc')
```

## Events

UI-Doc provides functionality to register events.

| Name          | Params            | When                                                   |
| ------------- | ----------------- | ------------------------------------------------------ |
| context-entry | ContextEntryEvent | Before context entry gets created, updated or deleted. |
| example       | ExampleEvent      | Before an example gets outputted.                      |
| output        | OutputEvent       | Before the complete documentation gets outputted.      |
| page          | PageEvent         | Before a page gets outputted.                          |
| source        | SourceEvent       | Before a source gets created, updated or deleted.      |

```ts
// change the order on every entry to 200
const onContextEntry = ({ entry }) => {
  entry.order = 200
}

// register your listener
uidoc.on('context-entry', onContextEntry)

// unregister your listener
uidoc.off('context-entry', onContextEntry)
```

## CommentBlockParser

The default CommentBlockParser extracts comment blocks from the source using comment tags to create the context.

```
/**
 * Will be interpreted as a page.
 *
 * @page typography Typography
 */

/**
 * Will be interpreted as a section on the page `typography`. Giving an example of default typography formats.
 *
 * @location typography.format Format
 * @example
 * <span>Normal Text</span><br>
 * <strong>Strong Text</strong><br>
 * <small>Small Text</small><br>
 * <em>Emphasis Text</em><br>
 * <i>Italic Text</i>
 */
```

Per default the `CommentBlockParser` will use the `MarkdownDescriptionParser` to parse descriptions from markdown into HTML.

### Events

| Name   | Params | When                      |
| ------ | ------ | ------------------------- |
| parsed | Block  | After a block got parsed. |

### Custom Tags

You can add your custom tags to transform the block to your needs. Let's say you want to create a `author` tag that defines an author.

```ts
// @author author-key Author Name

const commentBlockParser = new CommentBlockParser(new MarkdownDescriptionParser())

commentBlockParser.registerTagTransformer({
  name: 'author', // name of your tag
  transform(block, spec) {
    // we will add the author to the block (the block get's currently transformed by the parser)
    // inside the spec we will find the parsed information of your tag
    block.author = {
      key: spec.name, // author-key
      name: spec.description, // Author Name
    }
  },
})
```
