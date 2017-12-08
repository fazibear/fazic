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
        li class: "#{"active" if router.current_url?(:home)}" do
          a href: router.url_for(:home) do
            'Home'
          end
        end
        li class: "#{"active" if router.current_url?(:home)}" do
          a href: router.url_for(:home) do
            'Basic'
          end
        end
      end
    end
  end
end
