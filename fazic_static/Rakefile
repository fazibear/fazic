SRC = '../fazic/target/asmjs-unknown-emscripten/release/fazic.js'
DST = 'static/'

task 'update' do
  size = File.size(SRC).to_f / 2**20
  puts 'Size: %.2f' % size
  cp SRC, DST
end
