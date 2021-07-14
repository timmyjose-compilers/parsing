# Expression Parsing techniques

 * [ ] [Normal Recursive-Descent](recursive-descent/README.md)

 * [ ] [Shunting Yard](shunting-yard/README.md)

 * [ ] [Operator Precedence Parsing](operator-precedence/README.md)

 * [ ] [Pratt Parsing](pratt/README.md)


## Grammar

The specific grammar covers all four basic binary arithmetic operators (`+`, `-`, `*`, and `/`) as well as a couple of extra operators (`%`, `^`). Parentheses are
allowed to group expressions together. The grammar also includes `+` and `-` in their unary forms.

The precedence (higher numbers reflect stronger precedence) and associativity of these operators are as follows:

```
    Operator       Assocativity      Precdence
  -----------------------------------------------
      %                 L                1
      +                 L                3
      -                 L                3
      *                 L                5
      /                 L                5
      ^                 R                7
      +                 R                9 (unary)
      -                 R                9 (unary)
      (                 L                11
      )                 L                11

```

The direct grammar is as follows:

```
  expr ::= expr % expr
         | expr + expr
         | expr - expr
         | expr * expr
         | expr / expr
         | expr ^ expr
         | '+' expr 
         | '-' expr
         | '(' expr ')'

```

The same grammar, in `LL(1)` form is (TBD):

```
  expr ::= term term_tail
  term_tail ::= add_op term term_tail | epsilon
  term ::= factor factor_tail
  factor_tail ::= mult_op factor factor_tail | epsilon
  factor ::= power power_tail
  power_tail ::= power_op power power_tail | epsilon
  power ::= '(' expr ')' | number
  add_op ::= + | -
  mult_op ::= * | /
  power_op ::= ^
  number ::= ...-2 | -1 | 0 | 1 | 2 ...
  
```

Source: Adapted from [Arithmetic Evaluation in C](https://rosettacode.org/mw/index.php?title=Arithmetic_evaluation/C&redirect=no)
