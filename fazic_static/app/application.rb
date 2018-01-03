# require Inesita
require 'inesita'
require 'inesita-router'

# require main parts of application
require 'router'
require 'store'

# require all components
require_tree './components'
require_tree './store'

# when document is ready render application to <body>

class Application
  include Inesita::Component

  inject Router
  inject Store

  def render
    component NavBar
    div.container do
      component Stripes
      div.content do
        h1 "FAZIC"
        h2 "fantasy retro computer"
        div.social do
          a class: "fa fa-twitter", href: "https://twitter.com/fazic1"
          a class: "bold", href: "https://www.patreon.com/fazic" do
            "P"
          end
          a class: "bold", href: "https://fazibear.itch.io/fazic" do
            "i"
          end
          a class: "fa fa-github", href: "https://github.com/fazibear/fazic"
        end
        component router
      end
    end
    component Footer
  end
end


Inesita::Browser.ready? do
  Application.mount_to(Inesita::Browser.body)
end
