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
  md5 = Digest::MD5.file(SRC)
  size = File.size(SRC).to_f / 2**20

  File.open(HTML, "r+") do |file|
    content = file.read.gsub(/fazic.js\?md5=\w+/, "fazic.js?md5=#{md5}")
    file.rewind
    file.puts content
  end

  puts 'Size: %.2f, checksum: %s' % [size, md5]
  cp SRC, DST
end
