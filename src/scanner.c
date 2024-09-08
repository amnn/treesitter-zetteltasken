/** Re-use the markdown external scanner for zetteltasken. */
#define tree_sitter_markdown_external_scanner_create                           \
  tree_sitter_zetteltasken_external_scanner_create
#define tree_sitter_markdown_external_scanner_destroy                          \
  tree_sitter_zetteltasken_external_scanner_destroy
#define tree_sitter_markdown_external_scanner_serialize                        \
  tree_sitter_zetteltasken_external_scanner_serialize
#define tree_sitter_markdown_external_scanner_deserialize                      \
  tree_sitter_zetteltasken_external_scanner_deserialize
#define tree_sitter_markdown_external_scanner_scan                             \
  tree_sitter_zetteltasken_external_scanner_scan

#include "../third-party/tree-sitter-markdown/tree-sitter-markdown/src/scanner.c"
