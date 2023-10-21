class Router
  include Inesita::Router

  def routes
    route '/', to: About
    route '/help', to: Help
    route '/disc', to: Disc
    route '/examples', to: Examples
    route '/roadmap', to: Roadmap
    route '/patreons', to: Patreons
  end
end
