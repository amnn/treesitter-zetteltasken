/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar(require("tree-sitter-markdown/grammar"), {
  name: "zetteltasken",
  rules: {},
});
