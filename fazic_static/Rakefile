SRC = '../fazic/target/asmjs-unknown-emscripten/release/fazic.js'
DST = 'static/'

desc 'build'
task :build do
  sh 'bundle exec inesita build -f > /dev/null'
  sh 'cp fazic.html dist/fazic.html'
  sh 'cp _redirects dist/_redirects'
end

task 'update' do
  size = File.size(SRC).to_f / 2**20
  puts 'Size: %.2f' % size
  cp SRC, DST
end
