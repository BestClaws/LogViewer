import XCTest
import SwiftTreeSitter
import TreeSitterLoguage

final class TreeSitterLoguageTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_loguage())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Loguage grammar")
    }
}
