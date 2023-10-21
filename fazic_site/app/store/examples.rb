class Store
  module Examples
    EXAMPLES = [
      {
        name: "HELLO",
        description: "Simple 'hello world' basic program",
        code: <<~EOP
          10 PRINT "Hello"
          20 GOTO 10
        EOP
      },
      {
        name: "COLORS",
        description: "Fill all screen with pixel and show it.",
        code: <<~EOP
          5 mode 1
          10 for c=0 to 15
          20 for y=0 to 240
          30 for x=0 to 320
          40 dot x,y
          50 next
          60 next
          70 flip
          80 color c
          90 next
        EOP
      },
    ]
  end
end
