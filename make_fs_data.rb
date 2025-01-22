require 'erb'
require 'find'
require 'thread'
require 'async'

start_file_path = ARGV[0]
args = ARGV[1..]
paths = []
files = []

def add(path)
  ret = Async do
    file = File.read(path).bytes
    raise "null byte is included in #{path}" if file.include?(0)

    file
  end.wait

  ret << 0
  path = (path.bytes << "\0".ord)

  return [ret, path]
end

Async do
args.each do |arg_path|
  expand_path = File.expand_path(arg_path)
  if File.directory?(expand_path)
    Async do
      Find.find(expand_path) do |path|
        Find.prune if path.end_with?('.git') || path.end_with?('/ports') || path.end_with?('/logs') || path.end_with?('/tmp') || path.end_with?('/extensions') || path.end_with?('/test') || path.end_with?('/spec') || path.end_with?('.github') || path.end_with?('/docs') || path.end_with?('/exe')
        next if path.end_with?('.so') || path.end_with?('.c') || path.end_with?('.h') || path.end_with?('.o') || path.end_with?('.java') || path.end_with?('.jar') || path.end_with?('.gz') || path.end_with?('.dat') || path.end_with?('.sqlite3') || path.end_with?('.exe')
        next if path.end_with?('.gem') || path.end_with?('.png') || path.end_with?('.jpg') || path.end_with?('.jpeg') || path.end_with?('.gif') || path.end_with?('.bmp') || path.end_with?('.ico') || path.end_with?('.svg') || path.end_with?('.webp') || path.end_with?('.ttf') || path.end_with?('.data')
        next if path.end_with?('selenium-manager')
        next if File.directory?(path)

        ret = add(path)
        files += ret[0]
        paths += ret[1]
      end
    end
  else
    ret = add(expand_path)
    files += ret[0]
    paths += ret[1]
  end
end
end.wait

File.write('fs.c', ERB.new(DATA.read).result(binding))

__END__
const char FILES[] = {<%= files.join(',') %>};
const int FILES_SIZE = <%= files.size %>;
const char PATHS[] = {<%= paths.join(',') %>};
const int PATHS_SIZE = <%= paths.size %>;
const char WD[] = {<%= Dir.pwd.bytes.join(',') %>,0};
const char START_FILE_PATH[] = {<%= File.expand_path(start_file_path).bytes.join(',') %>,0};
