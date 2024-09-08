package tree_sitter_zetteltasken_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_zetteltasken "github.com/tree-sitter/tree-sitter-zetteltasken/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_zetteltasken.Language())
	if language == nil {
		t.Errorf("Error loading Zetteltasken grammar")
	}
}
