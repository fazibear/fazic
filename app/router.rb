class Router
  include Inesita::Router

  def routes
    route '/', to: Home
  end
end
