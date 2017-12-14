class Programs
  include Inesita::Component

  def toggle(program)
    program[:show] = !program[:show]
    render!
  end

  def go_to_program(name)
    $console.log(name)
    name = "program_#{name}".to_n
    `window.location.hash = #{name}`
  end

  def render
    h3 "Your programs"
    ul.programs do
      store.programs.each do |program|
        li.program id: "program_#{program[:id]}" do
          a href: "#", onclick: -> { toggle(program) } do
            program[:name]
          end
          pre program[:code] if program[:show]
        end
      end
    end
  end
end
