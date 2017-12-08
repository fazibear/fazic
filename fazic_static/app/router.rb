class Router
  include Inesita::Router

  def routes
    route '/', to: Home
    route '/commands', to: Commands
    route '/programs', to: Programs
  end
end
