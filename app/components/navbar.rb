class NavBar
  include Inesita::Component

  def render
    nav do
      ul do
        li do
          a href: "/fazic" do
            'Fazic'
          end
        end
        li do
          a href: router.url_for(:home), class: "#{"active" if router.current_url?(:home)}" do
            'Home'
          end
        end
        li do
          a href: router.url_for(:commands), class: "#{"active" if router.current_url?(:commands)}" do
            'Commands'
          end
        end
        li do
          a href: router.url_for(:programs), class: "#{"active" if router.current_url?(:programs)}" do
            'Programs'
          end
        end
      end
    end
  end
end
