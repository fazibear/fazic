class Store
  include Inesita::Injection

  attr_accessor :commands, :programs

  def commands
    Commands::COMMANDS
  end

  def examples
    Examples::EXAMPLES
  end

  def init
    fetch_commnads
    fetch_programs
  end

  def fetch_commnads
    @commands = []
  end

  def fetch_programs
    @programs = []

    (0...storage_length).each do |i|
      name = storage_key(i)
      if name.end_with?(".bas")
        code = storage_value(i)
        @programs << {
          id: i,
          name: name[0..-5],
          code: code,
          show: false,
        }
      end
    end
  end

  def storage_length
    `localStorage.length`
  end

  def storage_key(i)
    Native(`localStorage.key(i)`)
  end

  def storage_value(i)
    Native(`localStorage.getItem(localStorage.key(i))`)
  end
end
