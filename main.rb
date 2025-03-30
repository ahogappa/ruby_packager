Dir.chdir('/workspaces/ruby_packager/test_rails')
APP_PATH = '/workspaces/ruby_packager/test_rails/config/application'
require '/workspaces/ruby_packager/test_rails/config/boot.rb'
require 'rails/commands'

Rails::Command.invoke('server', ['-P', '/tmp/server.pid', '-p', '3000', '-b', '0.0.0.0'])
