/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "zetteltasken",
  rules: {
    source_file: (_) => "hello",
  },
});
