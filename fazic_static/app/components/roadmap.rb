class Roadmap
  include Inesita::Component

  def render
    h3 "Things to do!"
    h4 "BASIC"
    ul do
      li "PET ASCII Support"
      li "User Input Commands"
      li "Graphic Commands"
      li "Sprites Support"
      li "Sound"
      li "Arrays Support"
      li "Hashes Support"
      li "Networking Support"
    end
    h4 "Infrastructure"
    ul do
      li "Upload you programs"
      li "Save programs in the cloud"
      li "Share programs with other clients"
      li "Share programs with other users"
      li "Load other users programs"
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
