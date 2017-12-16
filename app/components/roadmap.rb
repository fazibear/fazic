class Roadmap
  include Inesita::Component

  def render
    h3 "Things to do!"
    h4 "BASIC"
    ul do
      li "Graphic"
      li "Sprites"
      li "Sound"
      li "Arrays"
      li "Hashes"
      li "Networking"
    end
    h4 "Infrastructure"
    ul do
      li "Save in the cloud"
      li "Share programs"
      li "Load other user programs"
    end
    h4 "Native clients"
    ul do
      li "OSX"
      li "Windows"
      li "Linux"
      li "iOS"
      li "Android"
    end
  end
end
