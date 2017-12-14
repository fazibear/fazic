class Store
  include Inesita::Injection

  attr_accessor :commands, :programs

  def init
    fetch_commnads
    fetch_programs
  end

  def fetch_commnads
    @commands = []
  end

  def fetch_programs
    @programs = []

    (0...`localStorage.length`).each do |i|
      name = Native(`localStorage.key(i)`)
      if name.end_with?(".bas")
        code = Native(`localStorage.getItem(localStorage.key(i))`)
        @programs << {
          id: i,
          name: name[0..-5],
          code: code,
          show: false,
        }
      end
    end
  end
end
