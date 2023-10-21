class Examples
  include Inesita::Component

  def copy(program)
    store.copy_program(program[:name], program[:code])
    store.fetch_programs
    render!
    false
  end

  def render
    h3 "Program Examples"
    p "If you want to try few programs, on this page, you can find some of basic programs. Click disc icon to copy into your disc."
    ul.examples do
      store.examples.each do |program|
        li.command id: "program_#{program[:name]}" do
          a class: "copy", href: "#", onclick: -> { copy(program) } do
            store.exist?(program[:name]) ? "✅" : "💾"
          end
          span.name program[:name]
          div.description program[:description]
          pre program[:code]
        end
      end
    end
  end
end
