class Router
  include Inesita::Router

  def routes
    route '/', to: About
    route '/help', to: Help
    route '/programs', to: Programs
    route '/roadmap', to: Roadmap
    route '/patreons', to: Patreons
  end
end
