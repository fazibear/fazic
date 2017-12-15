class Commands
  include Inesita::Component

  def render
    h3 "Available Commands"
    p "List of all commands with example that are supported."
    ul.commands_toc do
      store.commands.each do |command|
        li.command do
          a href: "#", onclick: -> {`window.location.hash = #{"command_#{command[:name].to_n}"}`; false} do
            command[:name]
          end
        end
      end
    end
    ul.commands do
      store.commands.each do |command|
        li.command id: "command_#{command[:name]}" do
          div.name command[:name]
          div.description command[:description]
          pre command[:code]
        end
      end
    end
  end
end
