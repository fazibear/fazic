class Store
  module Commands
    COMMANDS = [
      {name: "ABS", description: "Evaluates to the absolute value (value without the sign) of the given numeric term.", code: %Q{PRINT ABS(-10)\n10}},
      {name: "AND", description: "Boolean AND or bitwise AND operation depending on arguments", code: %Q{PRINT 3>2 AND 5<6\ntrue\nPRINT 199 AND 64\n64}},
      {name: "ASC", description: "Evaluates the first char of a string as a numeric index.", code: %Q{PRINT ASC("A")\n65}},
      {name: "ATN", description: "A mathematical function that returns the arc tangent of a numeric value (the inverse function of TAN). The resulting value is the angle in radians of the given tangent.", code: %{PRINT ATN(1)\n0.7854}},
      {name: "CHR", description: "Can be used to convert a number between 0 and 255 into an ASCII-char and is the inverse of the function ASC.", code: %Q{PRINT CHR(49)\n1}},
      {name: "CLR", description: "Deletes all variables.", code: %Q{A=10\nPRINT A\n10\nCLR\nPRINT A\nnull}},
      {name: "COS", description: "Is a mathematical function which evaluates to the cosine for a given angle, a number regarded as being in radians.", code: %Q{PRINT COS(0)\n1.0000}},
      {name: "DOT", description: "Drapws a pixel on x, y position", code: %Q{DOT 1,5}},
      {name: "END", description: "Ends a BASIC-program and the BASIC-Interpreter writes READY. ", code: %Q{10 PRINT "START"\n20 END\n30 PRINT "NEVER HAPPENED"}},
      {name: "EXP", description: "Is a mathemathical function that evaluates the inverse natural LOG of the argument.", code: %Q{PRINT EXP(1)\n2.7183}},
      {name: "FLIP", description: "When MODE 1 is used, draw current screen buffer", code: %Q{FLIP}},
      {name: "FOR", description: "is the start command of a FOR…TO…STEP…NEXT loop. This FOR...NEXT loop is executed until counter variable equals the value in the TO clause. With the step-size-number, the counter variable value is either increased (positive) or decreased (negative). When the STEP command isn't used then the step-size-number defaults to 1.", code: %Q{FOR X=1 TO 5:PRINT X:NEXT\n1\n2\n3\n4\n5}},
      {name: "GOSUB", description: "Jumps to a subroutine at the indicated line number. The subroutine finalizes using a RETURN command.", code: %Q{10 GOSUB 40\n20 PRINT "HELLO"\n30 END\n40 PRINT "HELLO GOSUB"\n50 RETURN}},
      {name: "GOTO", description: "Makes the BASIC interpreter jump to the indicated line and the execution of the BASIC program is continued at that line.", code: %Q{10 PRINT "HELLO"\n20 GOTO 10}},
      {name: "IF", description: "Is used together with the BASIC command THEN or with the BASIC command GOTO by condition.", code: %{IF A>0 THEN PRINT "A is positive"}},
      {name: "INT", description: "Is used to round numbers, whereas rounding is different from its common mathematical definition.", code: %Q{PRINT INT(1.53)\n1}},
      {name: "LEN", description: "Views the number of all chars in a string.", code: %Q{PRINT LEN("just and example")\n16}},
      {name: "LET", description: "Is for assign numerical values or chars in the right type of variable. Assignment of variables can also be done without LET.", code: %Q{LET A="TEST"\nB="COMMODORE"}},
      {name: "LIST", description: "Displays the BASIC program currently in memory.", code: "LIST"},
      {name: "LOAD", description: "Load a program into memory", code: %Q{LOAD "MYPROG"}},
      {name: "LOG", description: "Is a natural logarithm with the basis e(E).", code: %Q{PRINT LOG(1)\n0}},
      {name: "MODE", description: "Sets display mode: 0 - text mode, 1 - Graphic mode with frame buffer, use FLIP to show, 2 - Graphic direct mode", code: %Q{MODE 1}},
      {name: "NEXT", description: "Is used with the BASIC-Command FOR.", code: %Q{}},
      {name: "NOT", description: "Reverse the boolean true into false.", code: %Q{}},
      {name: "OR", description: "Boolean OR or bitwise OR operation depending on arguments", code: %Q{PRINT 3<2 OR 5<6\ntrue\nPRINT 0 OR 0\n0}},
      {name: "PRINT", description: "Is used to print data onto the screen.", code: %Q{PRINT 12+2\n14}},
      {name: "REM", description: "Is used to place remarks into BASIC-programs.", code: %Q{10 REM NICE COMMENT}},
      {name: "RETURN", description: "Finishes a subroutine, which is called with the BASIC-command GOSUB.", code: %Q{}},
      {name: "RUN", description: "Starts a BASIC program.", code: %Q{RUN}},
      {name: "SAVE", description: "Save current program", code: %Q{SAVE}},
      {name: "SGN", description: "Gives autonomous of the algebraic sign the number (-1; 0; 1) of a numerical argument.", code: %Q{PRINT SGN(-11)\n-1}},
      {name: "SIN", description: "Is a mathematical function which evaluates to the sine for a given angle, a number regarded as being in radians.", code: %Q{PRINT SIN(1)}},
      {name: "SQR", description: " Is a mathemathical function for square root of a number.", code: %Q{PRINT SQL(4)\n2}},
      {name: "STOP", description: "Breaks a program.", code: %Q{STOP}},
      {name: "STR", description: "Is for converting numerical values or varibales into a string.", code: %Q{PRINT STR(12)\n12}},
      {name: "TAN", description: "Is a mathematical function which evaluates to the tangent for a given angle, a number regarded as being in radians.", code: %Q{PRINT TAN(1)}},
      {name: "VAL", description: "Finds a numerical value in a string.", code: %Q{PRINT VAL("10")\n10}},
    ]
  end
end
