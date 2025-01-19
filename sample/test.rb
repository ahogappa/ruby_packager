# require "json"

# # JSON文字列をRubyのオブジェクトに変換する
# json_str = '{"name": "Ruby", "age": 30}'
# p JSON.parse(json_str)

# # RubyのオブジェクトをJSON文字列に変換する
# data = {"name" => "Ruby", "age" => 30}
# p JSON.dump(data)

# require 'sqlite3'

# File.delete("test.db") if File.exist?("test.db")

# db = SQLite3::Database.new "test.db"

# # Create a table
# rows = db.execute <<-SQL
#   create table numbers (
#     name varchar(30),
#     val int
#   );
# SQL

# {
#   "one" => 1,
#   "two" => 2,
# }.each do |pair|
#   db.execute "insert into numbers values ( ?, ? )", pair
# end

# require 'sinatra'

# set :bind, '0.0.0.0'

# get '/' do
#   rows = db.execute <<-SQL
#     select * from numbers;
# SQL
# rows.to_s
# end
