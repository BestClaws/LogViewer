/**
 * @file LogViewer language parser
 * @author Syed Liyakhath
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "loguage",

  rules: {
    query: $ => seq(
      '[',
      $.operation,
      repeat(seq('|', $.operation)),
      ']'
    ),

    operation: $ => seq(
      $.operation_name,
      $.operation_arguments
    ),

    operation_name: $ => /\w+/, // Matches operations like 'search', 'fields', etc.
    
    operation_arguments: $ => choice(
      $.expression,
      $.lucene_query
    ),

    subquery: $ => $.query, // Nested queries inside arguments

    lucene_query: $ => seq(
      '`',
      /[^``]+/,  // Match everything inside parentheses
      '`'
    ),

 

    expression: $ => prec.right(seq(
      $.term,
      repeat(seq($.logical_operator, $.term))
    )),

    term: $ => choice(
      $.subquery,
      /\w+/
    ),

    logical_operator: $ => prec.left(choice(
      'AND',
      'OR',
      'NOT'
    )),

    _whitespace: $ => /\s+/
  }
});
