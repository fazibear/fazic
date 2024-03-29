peg::parser!(
    pub grammar parser() for str {

        use fazic::enums::*;
        use fazic::nodes::*;

        pub rule parse_all() -> Entry
            = i:integer() _ a:(all() ++ ":") { entry_node(&Some(i), a) }
            / a:(all() ++ ":") { entry_node(&None, a) }
            / i:integer() _ { entry_node(&Some(i), vec![]) }

        rule all() -> Vec<NodeElement>
            = _ c:command() _ { c }
            // _ e:expression() _ { vec![e] }

        rule command() -> Vec<NodeElement>
            = v:variable() _ "=" _ e:expression() { nodes("let", vec![v, e]) }
            / ("LET" / "let") _ v:variable() _ "=" _ e:expression() { nodes("let", vec![v, e]) }
            / ("?" / "PRINT" / "print") _ e:expression() { nodes("print", vec![e]) }

            / ("GOTO" / "goto") _ i:integer() { nodes("goto", vec![i]) }
            / ("GOSUB" / "gosub") _ i:integer() { nodes("gosub", vec![i]) }

            / ("FOR" / "for") _ v:variable() _ "=" _ e:expression() _ ("TO" / "to") _ t:expression() _ ("STEP" / "step") _ s:expression() { nodes("for", vec![v, e, t, s]) }
            / ("FOR" / "for") _ v:variable() _ "=" _ e:expression() _ ("TO" / "to") _ t:expression() { nodes("for", vec![v, e, t, number_node("1")]) }

            / ("IF" / "if") _ e:expression() _ ("GOTO" / "goto") _ i:integer() {
                vec![node("if", vec![e]), node("goto", vec![i])]
            }
            / ("IF" / "if") _ e:expression() _ ("GOSUB" / "gosub") _ i:integer() {
                vec![node("if", vec![e]), node("gosub", vec![i])]
            }
            / ("IF" / "if") _ e:expression() _ ("THEN" / "then") _ i:integer() {
                vec![node("if", vec![e]), node("goto", vec![i])]
            }
            / ("IF" / "if") _ e:expression() _ ("THEN" / "then") _ a:all() {
                let mut vec = nodes("if", vec![e]);
                vec.extend(a);
                vec
            }
            / ("DOT" / "dot") _ x:expression() _ "," _ y:expression() { nodes("dot", vec![x, y]) }
            / ("LINE" / "line") _ x:expression() _ "," _ y:expression() _ "," _ x2:expression() _ "," _ y2:expression() { nodes("line", vec![x, y, x2, y2]) }
            / ("CIRCLE" / "circle") _ x:expression() _ "," _ y:expression() _ "," _ r:expression() { nodes("circle", vec![x, y, r]) }
            / ("CLEAR" / "clear") _ color:expression() { nodes("clear", vec![color]) }
            / ("SRAND" / "srand") _ x:expression() { nodes("srand", vec![x]) }

            / ("NEXT" / "next" ) _ v:(variable() ++ (_ "," _) ) {
                let mut vec = vec![];
                for variable in v.into_iter() {
                    vec.extend(nodes("next", vec![variable]));
                };
                vec
            }

            / ("LOAD" / "load") _ s:string() { nodes("load", vec![s]) }
            / ("SAVE" / "save") _ s:string() { nodes("save", vec![s]) }
            / ("DIR" / "dir") { nodes("dir", vec![]) }

            / ("NEXT" / "next" ) { nodes("next", vec![]) }

            / ("COLOR" / "color") _ e:expression() { nodes("color", vec![e]) }
            / ("FLIP" / "flip") { nodes("flip", vec![]) }
            / ("MODE" / "mode") _ i:integer() { nodes("mode", vec![i]) }

            / ("RETURN" / "return") { nodes("return", vec![]) }
            / ("LIST" / "list")  { nodes("list", vec![]) }
            / ("CLR" / "clr")  { nodes("clr", vec![]) }
            / ("CONT" / "cont")  { nodes("cont", vec![]) }
            / ("STOP" / "stop")  { nodes("stop", vec![]) }
            / ("NEW" / "new")  { nodes("new", vec![]) }
            / ("RUN" / "run")  { nodes("run", vec![]) }
            / ("END" / "end")  { nodes("end", vec![]) }
            / ("REM" / "rem") * { nodes("rem", vec![]) }

        rule function() -> NodeElement
            = ("ABS(" / "abs(") _ e:expression() _ ")" { node("abs", vec![e]) }
            / ("RND(" / "rnd(") _ e:expression() _ ")" { node("rnd", vec![e]) }
            / ("SIN(" / "sin(") _ e:expression() _ ")" { node("sin", vec![e]) }
            / ("COS(" / "cos(") _ e:expression() _ ")" { node("cos", vec![e]) }
            / ("TAN(" / "tan(") _ e:expression() _ ")" { node("tan", vec![e]) }
            / ("ATN(" / "atn(") _ e:expression() _ ")" { node("atn", vec![e]) }
            / ("EXP(" / "exp(") _ e:expression() _ ")" { node("exp", vec![e]) }
            / ("LOG(" / "log(") _ e:expression() _ ")" { node("log", vec![e]) }
            / ("SQR(" / "sqr(") _ e:expression() _ ")" { node("sqr", vec![e]) }
            / ("SGN(" / "sgn(") _ e:expression() _ ")" { node("sgn", vec![e]) }
            / ("LEN(" / "len(") _ e:expression() _ ")" { node("len", vec![e]) }
            / ("CHR(" / "chr(") _ e:expression() _ ")" { node("chr", vec![e]) }
            / ("ASC(" / "asc(") _ e:expression() _ ")" { node("asc", vec![e]) }
            / ("INT(" / "int(") _ e:expression() _ ")" { node("int", vec![e]) }
            / ("VAL(" / "val(") _ e:expression() _ ")" { node("val", vec![e]) }
            / ("STR(" / "str(") _ e:expression() _ ")" { node("str", vec![e]) }
            / ("TIME()" / "time()") { node("time", vec![]) }

        rule expression() -> NodeElement=precedence! {
            l:(@) ("AND" / "and") r:@ { node("and", vec![l, r]) }
            l:(@) ("OR" / "or") r:@ { node("or", vec![l, r]) }
            --
            l:(@) "="  r:@ { node("eq", vec![l, r]) }
            l:(@) "==" r:@ { node("eq", vec![l, r]) }
            l:(@) "!=" r:@ { node("neq", vec![l, r]) }
            l:(@) "<>" r:@ { node("neq", vec![l, r]) }
            l:(@) "<"  r:@ { node("lt", vec![l, r]) }
            l:(@) ">"  r:@ { node("gt", vec![l, r]) }
            l:(@) "<=" r:@ { node("lteq", vec![l, r]) }
            l:(@) ">=" r:@ { node("gteq", vec![l, r]) }
            l:(@) "=<" r:@ { node("lteq", vec![l, r]) }
            l:(@) "=>" r:@ { node("gteq", vec![l, r]) }
            --
            l:(@) "+"  r:@ { node("add", vec![l, r]) }
            l:(@) "-"  r:@ { node("sub", vec![l, r]) }
            --
            l:(@) "*"  r:@ { node("mul", vec![l, r]) }
            l:(@) "/"  r:@ { node("div", vec![l, r]) }
            l:(@) "%"  r:@ { node("mod", vec![l, r]) }
            --
            l:@ "^"  r:(@) { node("pow", vec![l, r]) }
            --
            t:term() { t }
        }

        rule term() -> NodeElement
            = _ f:float() _ { f }
            / _ i:integer() _ { i }
            / _ s:string() _ { s }
            / _ f:function() _ { f }
            / _ "(" _ e:expression() _ ")" _ { e }
            / _ "-" _ t:term() _ { node("neg", vec![t]) }
            / _ ("NOT" / "not") _ e:expression() _ { node("not", vec![e]) }
            / _ v:variable() _ { v }

        rule float() -> NodeElement
            = f:$(['0'..='9']* "." ['0'..='9']+) { number_node(f) }

        rule integer() -> NodeElement
            = i:$(['0'..='9']+) { number_node(i) }

        rule string() -> NodeElement
            = "\"" s:$([^'"']*) "\"" { string_node(s) }
            / "'" s:$([^'\'']*) "'" { string_node(s) }

        rule variable() -> NodeElement
            = v:$(['a'..='z'|'A'..='Z']+) { variable_node(v) }

        rule _ = quiet!{" "*}
    }
);
