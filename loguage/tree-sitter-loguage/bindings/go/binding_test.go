package tree_sitter_loguage_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_loguage "github.com/tree-sitter/tree-sitter-loguage/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_loguage.Language())
	if language == nil {
		t.Errorf("Error loading Loguage grammar")
	}
}
