module Kernel
  unless defined?(original_require)
    alias_method :original_require, :require
    private :original_require
  end

  def require(path)
    Kompo.context do
      original_require(path)
    end
  rescue LoadError
    original_require(path)
  end

  private :require
end

p 'test1'
# require './sample/hello2'
# Kompo.context do
  # p __dir__
  # p __FILE__
  # require File.expand_path('test2', File.dirname(__FILE__))
  require_relative 'test2'
  require_relative '../hello2'
# end
