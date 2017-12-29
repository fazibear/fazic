require 'digest'

SRC = '../fazic/target/asmjs-unknown-emscripten/release/fazic.js'
DST = 'static/'
HTML = './fazic.html'

desc 'build'
task :build do
  sh 'bundle exec inesita build -f > /dev/null'
  sh 'cp fazic.html dist/'
  sh 'cp _redirects dist/'
  sh 'cp _headers dist/'
end

task 'update' do
  hash = Digest::MD5.file(SRC)
  size = File.size(SRC).to_f / 2**20

  File.open(HTML, "r+") do |file|
    content = file.read.gsub(/fazic.js\?hash=\w+/, "fazic.js?hash=#{hash}")
    file.rewind
    file.puts content
  end

  puts 'Size: %.2f, checksum: %s' % [size, hash]
  cp SRC, DST
end
