# # $LOAD_PATH = paths
# # paths.each do |path|
# #   $LOAD_PATH.unshift(path) unless $LOAD_PATH.include?(path)
# # end

# module Kernel
#   unless defined?(original_require)
#     alias_method :original_require, :require
#     private :original_require

#      alias_method :original_require_relative, :require_relative
#     private :original_require_relative
#   end

#   def require(path)
#     Kompo.context do
#       p path
#       p method(:original_require).source_location
#       original_require(path)
#     end
#   rescue LoadError => e
#     p "LoadError: #{path}, #{e}"
#     original_require(path)
#   end

#   def require_relative(path)
#     Kompo.context do
#       p caller_locations
#       # p File.dirname(caller_locations[2].path)
#       # p path if path.include?('puma')
#       # p method(:original_require_relative).source_location
#       p File.dirname(caller_locations[2].path)
#       original_require(File.expand_path(path, File.dirname(caller_locations[2].path)))
#     end
#   # rescue LoadError => e
#   #   p "LoadError: #{path}, #{e}"
#   #   original_require_relative(path)
#   end

#   private :require
#   private :require_relative
# end

# module Kernel
#   remove_method(:gem) if private_method_defined?(:gem)

#   def gem(*)
#   end

#   private :gem
# end
# unless defined?(Gem)
#   module Gem
#     def self.ruby_api_version
#       "3.5.0+0"
#     end

#     def self.extension_api_version
#       "#{ruby_api_version}-static"
#     end
#   end
# end
# if Gem.respond_to?(:discover_gems_on_require=)
#   Gem.discover_gems_on_require = false
# else
#   [::Kernel.singleton_class, ::Kernel].each do |k|
#     if k.private_method_defined?(:gem_original_require)
#       private_require = k.private_method_defined?(:require)
#       k.send(:remove_method, :require)
#       k.send(:define_method, :require, k.instance_method(:gem_original_require))
#       k.send(:private, :require) if private_require
#     end
#   end
# end

# # require '/workspaces/ruby_packager/dest_dir/lib/ruby/3.5.0+0/pathname.rb'

# Kompo.context do
# # p File.read('/workspaces/ruby_packager/dest_dir/lib/ruby/3.5.0+0/pathname.rb')
# # p File.read('/workspaces/ruby_packager/sample/dir/test1.rb')
# # p File.read('/workspaces/ruby_packager/main.rb')
# end
# # # require 'pathname'

# # # p Pathname.new('Gemfile').expand_path.parent
# $LOADED_FEATURES.unshift('/workspaces/ruby_packager/test_rails/bundle/ruby/3.5.0+0/extensions/aarch64-linux/3.5.0+0-static/bootsnap-1.18.4/bootsnap/bootsnap.so')
# # Kompo.context do
  # require 'puma/puma_http11'
# # end

# APP_PATH = File.expand_path("test_rails/config/application", __dir__)
# # # # # # p "app path #{APP_PATH}"
# Kompo.context do
# p File.read('/workspaces/ruby_packager/test_rails/Gemfile')
# end
# Kompo.context do
#   eval File.read('/workspaces/ruby_packager/test_rails/config/application.rb')
# end
# require '/workspaces/ruby_packager/test_rails/config/application.rb'
# require_relative 'test_rails/config/boot.rb'
# require "rails/command"
# $LOADED_FEATURES.unshift('puma/puma_http11.so')
# p $"
# require APP_PATH
# p $LOAD_PATH
#  Kompo.context do
#   p File.read('/workspaces/ruby_packager/main.rb')
# p $LOAD_PATH.resolve_feature_path('main.rb')
#  end
# # # # p "hgoeo: #{Rails.application.root}"

# Kompo.context do
# Rails::Command.invoke 'server'
# end


# APP_PATH = File.expand_path("test_rails_tmp/config/application", __dir__)
# require_relative './test_rails_tmp/config/boot.rb'
# require "rails/command"
# Rails::Command.invoke 'server'

# export GEM_PATH=/usr/local/rvm/gems/ruby-3.3.5:/usr/local/rvm/rubies/ruby-3.3.5/lib/ruby/gems/3.3.0/usr/local/rvm/rubies/ruby-3.3.5/lib/ruby/gems/3.3.0d
# export GEM_PATH=/workspaces/ruby_packager/test_rails/bundle/ruby/3.5.0+0:/workspaces/ruby_packager/dest_dir_tmp/lib/ruby/gems/3.5.0+0
# export GEM_HOME=/workspaces/ruby_packager/dest_dir/lib/ruby/gems/3.5.0+0
# export RUBYLIB=/workspaces/ruby_packager/dest_dir/lib/ruby/3.5.0+0:/workspaces/ruby_packager/dest_dir/lib/ruby/3.5.0+0/aarch64-linux

Kompo.context do
  require 'rubygems'
  # p Gem
  # p File.read('/workspaces/ruby_packager/dest_dir/lib/ruby/gems/3.5.0+0/specifications/error_highlight-0.7.0.gemspec')
  # require 'bundler'
  # Dir.chdir('/workspaces/ruby_packager/test_rails')
  # Bundler.ui.silence { Bundler.setup }
  # p Gem.extension_api_version
#   require 'pathname'
#   # require 'bundler'
#   # p Pathname.new('./test_rails/Gemfile').expand_path
#   # b = Bundler::Definition.build('./test_rails/Gemfile', './test_rails/Gemfile.lock', true)
#   # p b
#   # p File.read('/workspaces/ruby_packager/dest_dir/lib/ruby/gems/3.5.0+0/specifications/default/bundler-2.7.0.dev.gemspec')
#   # require File.expand_path('test_rails/config/boot.rb', File.dirname(__FILE__))
#  p File.read('/workspaces/ruby_packager/dest_dir/lib/ruby/3.5.0+0/rubygems.rb')
# p File.exist?('/workspaces/ruby_packager/dest_dir/lib/ruby/site_ruby')
  require '/workspaces/ruby_packager/test_rails/config/boot.rb'
end

