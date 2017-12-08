class Programs
  include Inesita::Component

  def render
    h3 "Your programs:"
    p.programs do
      store.programs.each do |program|
        a href: "#program-#{program[:name]}" do
          program[:name]
        end
      end
    end
    ul.programs do
      store.programs.each do |program|
        li.program id: "program-#{program[:name]}" do
          div.name program[:name]
          div.code program[:code]
        end
      end
    end
  end
end
