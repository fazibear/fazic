class Stripes
  include Inesita::Component

  def render
    div.stripes do
      div.s1l ""
      div.s2l ""
      div.s3l ""
      div.s3r ""
      div.s2r ""
      div.s1r ""
    end
  end
end
