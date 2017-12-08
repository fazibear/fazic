class Commands
  include Inesita::Component

  def render
    h3 "Available Commands"
    ul.basic do
      store.commands.each do |command|
        li.command do
          div.name command.name
          div.description command.description
          div.example command.example
        end
      end
    end
  end
end
