class Disc
  include Inesita::Component

  def toggle(program)
    program[:show] = !program[:show]
    render!
  end

  def go_to_program(name)
    name = "program_#{name}".to_n
    `window.location.hash = #{name}`
  end

  def render
    h3 "Your private floppy disc"
    if store.programs.any?
      p "List of all your saved programs. Click to show the code."
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
    else
      p do
        span "You don't have any programs. You can copy from "
        a href: router.url_for(:examples), class: "#{"active" if router.current_url?(:examples)}" do
          'EXAMPLES'
        end
        span " section, or save you own using save command!"
      end
    end
  end
end
