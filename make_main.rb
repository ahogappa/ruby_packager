require 'fileutils'
require 'erb'

# for nokogiri
ENV['NOKOGIRI_USE_CANONICAL_GNOME_SOURCE'] = '1'

exts=[]
exts_libs=[]
Dir.glob("test_rails/bundle/ruby/#{RbConfig::CONFIG['ruby_version']}/gems/**/extconf.rb").each do |makefile_dir|
  # dir_name = File.dirname(makefile_dir)
  # makefile = File.join(dir_name, 'Makefile')
  # next unless File.exist?(makefile)
  # objs = File.read(File.join(dir_name, 'Makefile')).scan(/OBJS = (.*\.o)/).join(' ')
  # system ['make', '-C', dir_name, objs, '--always-make'].join(' ')
  # dir = FileUtils.mkdir_p('exts/' + File.basename(dir_name))
  # FileUtils.cp(objs.split(' ').map{File.join(dir_name, _1)}, "#{dir[0]}")
  # prefix = File.read(File.join(dir_name, 'Makefile')).scan(/target_prefix = (.*)/).join.delete_prefix('/')
  # target_name = File.read(File.join(dir_name, 'Makefile')).scan(/TARGET_NAME = (.*)/).join
  # exts << [File.join(prefix, "#{target_name}.so").delete_prefix('/'), "Init_#{target_name}"]

  dir_name = File.dirname(makefile_dir)
  makefile = File.join(dir_name, 'Makefile')
  copy_targets = []
  Dir.chdir(dir_name) do |path|
    command = [
      'ruby',
      'extconf.rb',
    ].join(' ')
    system command
    objs = File.read('./Makefile').match(/OBJS = (.*\.o)/)[1]
    command = ['make', objs, '--always-make'].join(' ')
    system command
    exts_libs += File.read('./Makefile').match(/^libpath = (.*)/)[1].split(' ')
    copy_targets = objs.split(' ').map { File.join(dir_name, _1) }
  end
  dir = FileUtils.mkdir_p('exts/' + File.basename(dir_name)).first
  FileUtils.cp(copy_targets, dir)
  prefix = File.read(makefile).scan(/target_prefix = (.*)/).join.delete_prefix('/')
  target_name = File.read(makefile).scan(/TARGET_NAME = (.*)/).join
  exts << [File.join(prefix, "#{target_name}.so").delete_prefix('/'), "Init_#{target_name}"]
end

File.write("main.c", ERB.new(File.read("main.c.erb")).result)
