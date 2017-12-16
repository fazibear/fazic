class About
  include Inesita::Component

  def render
    h3 "What is this?"
    p "FAZIC is a fantasy retro computer. You can create, share, and play tiny games or programs. You have build in BASIC interpreter, so you can start now like it's the 80s."
    h3 do
      a href: "/fazic" do
        'Click here to start'
      end
    end
  end
end
