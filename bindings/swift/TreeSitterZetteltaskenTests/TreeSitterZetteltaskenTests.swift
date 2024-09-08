import XCTest
import SwiftTreeSitter
import TreeSitterZetteltasken

final class TreeSitterZetteltaskenTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_zetteltasken())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Zetteltasken grammar")
    }
}
