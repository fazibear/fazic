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
      h1 "FAZIC"
      h2 "fantasy retro computer"
      div.content do
        component router
      end
    end
    component Footer
  end
end


Inesita::Browser.ready? do
  Application.mount_to(Inesita::Browser.body)
end
