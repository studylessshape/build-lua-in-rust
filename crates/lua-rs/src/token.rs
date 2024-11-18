#[derive(Debug, PartialEq)]
pub enum Token {
    // keywords
    And, Break, Do, Else, ElseIf, End,
    False, For, Function, Goto, If, In,
    Local, Nil, Not, Or, Repeat, Return,
    Then, True, Until, While,
    //+  -    *    /    %    ^    #
    Add, Sub, Mul, Div, Mod, Pow, Len,
    //&     ~       |      <<      >>      //
    BitAnd, BitXor, BitOr, ShiftL, ShiftR, Idiv,
    //==   ~=     <=     >=     <     >        =
    Equal, NotEq, LesEq, GreEq, Less, Greater, Assign,
    //(   )     {       }       [      ]      ::
    ParL, ParR, CurlyL, CurlyR, SqurL, SqurR, DoubleColon,
    //;        :      ,      .    ..      ...
    SemiColon, Colon, Comma, Dot, Concat, Dots,

    Integer(i64),
    Float(f64),
    Name(String),
    String(String),
    EOF,
}