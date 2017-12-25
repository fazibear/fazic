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
          a href: router.url_for(:disc), class: "#{"active" if router.current_url?(:disc)}" do
            'Disc'
          end
        end
        li do
          a href: router.url_for(:about), class: "#{"active" if router.current_url?(:about)}" do
            'About'
          end
        end
        li do
          a href: router.url_for(:help), class: "#{"active" if router.current_url?(:help)}" do
            'Help'
          end
        end
        li do
          a href: router.url_for(:roadmap), class: "#{"active" if router.current_url?(:roadmap)}" do
            'Roadmap'
          end
        end
        li do
          a href: router.url_for(:patreons), class: "#{"active" if router.current_url?(:patreons)}" do
            'Patreons'
          end
        end
      end
    end
  end
end
