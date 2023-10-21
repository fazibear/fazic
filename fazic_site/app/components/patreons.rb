class Patreons
  include Inesita::Component

  def render
    h3 "Patreon"
    p "If you like it. Want more. You can support me on patreon!"
    a href: "https://www.patreon.com/fazibear" do
      "https://www.patreon.com/fazibear"
    end
    p "Thanks!"
    h3 "itch.io"
    p "You can also check out itch.io page."
    a href: "https://fazibear.itch.io/fazic" do
      "https://fazibear.itch.io/fazic"
    end
    p "Thanks!"
    h3 "Who support this project?"
  end
end
