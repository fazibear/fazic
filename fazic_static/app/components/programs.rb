class Programs
  include Inesita::Component

  def go_to_program(name)
    $console.log(name)
    name = "program_#{name}".to_n
    `window.location.hash = #{name}`
  end

  def render
    h3 "Your programs"
    ul.programs_list do
      store.programs.each do |program|
        li do
          a href: "#program_#{program[:name]}", onclick: ->{ `window.location.hash = #{"program_#{program[:name].to_n}"}` } do
            program[:name]
          end
        end
      end
    end
    ul.programs do
      store.programs.each do |program|
        li.program id: "program_#{program[:name]}" do
          div.name program[:name]
          pre program[:code]
        end
      end
    end
  end
end
