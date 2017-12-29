class Store
  include Inesita::Injection

  attr_accessor :commands, :programs

  BAS_EXT = '.bas'

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
      if name.end_with?(BAS_EXT)
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

  def exist?(name)
    name = filename(name)
    !Native(`localStorage.getItem(#{name})`).nil?
  end

  def copy_program(name, code)
    name = filename(name)
    `localStorage.setItem(name, code)`
  end

  def filename(name)
    "#{name}#{BAS_EXT}"
  end
end
