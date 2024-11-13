Anstml is a library for converting strings with ANSI codes to valid HTML to show the same thing.

## todo:
- [ ] support all the codes that are of the form `^[[{n}m`
- [ ] apparently it's allowed to do multiple modifiers at once seperated by `;`, that's gonna be a doozy
- [ ] create tests to make sure everything works as expected
- [ ] allow creation of classes + css instead of inlining all the styles
- [ ] optimize the generated HTML in using various methods
  - [ ] when some color is nested in a different color, use a nested span
  - [ ] when only part of the style changes, use a nested a span
