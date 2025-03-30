OUTPUT = bin
OBJS = *.o
FS_O = fs.o
FS_CLI = target/debug/kompo-cli
SAMPLE_DIR = sample
RUBY_HDRS = $(shell pkg-config --cflags dest_dir/lib/pkgconfig/ruby.pc)
RUBY_LIB = dest_dir/lib/libruby-static.a
RUBY_CONF = ruby/configure
FS_LIB = target/debug/libkompo_fs.a
RUBY_SRC = $(SAMPLE_DIR)/test.rb
EXTS = $(shell ruby -e'puts Dir.glob("ruby/ext/**/extconf.rb").reject { _1 =~ /-test-/ }.reject { _1 =~ /win32/ }.reject { _1 =~ /io\/console/ }.map { File.dirname(_1) }.map { _1.split("ruby/ext/")[1] }.join(",")')
EXT_OBJS = $(shell ruby -e'puts ["ruby/ext/extinit.o", "ruby/enc/encinit.o", *Dir.glob("ruby/ext/**/*.a"), *Dir.glob("ruby/enc/**/*.a")].join(" ")')
LOAD_PATHS = $(shell dest_dir/bin/ruby -e 'require "./bundle/bundler/setup.rb";puts $$LOAD_PATH.join(" ")')
AUTOXXX = $(shell ruby -e'puts File.exist?("ruby/autogen.sh") ? "./autogen.sh" : "autoconf"')
MAINLIB = $(shell pkg-config --variable=MAINLIBS dest_dir/lib/pkgconfig/ruby.pc)
EXTLIBS = $(shell ruby -e'puts Dir.glob("ruby/ext/**/exts.mk").flat_map { File.read(_1).scan(/EXTLIBS = (.*)/) }.join(" ")')
GEMLIBS = $(shell dest_dir/bin/ruby -e'puts Dir.glob("bundle/ruby/#{RbConfig::CONFIG["ruby_version"]}/gems/*/ext/*/Makefile").flat_map{ File.read(_1).scan(/LIBS = (.*)/)}.join(" ")')
LIBS = $(shell ruby -e'dyn,static=%w[$(MAINLIB) $(EXTLIBS) $(GEMLIBS)].uniq.select { _1.start_with?("-l") }.partition { _1 == "-lpthread" || _1 == "-ldl" || _1 == "-lm" || _1 == "-lc" };dyn.unshift "-Wl,-Bdynamic";static.unshift "-Wl,-Bstatic";puts static.join(" ") + " " + dyn.join(" ")')

all: $(OUTPUT)

$(OUTPUT): $(RUBY_LIB) $(FS_O) $(FS_LIB) main.c
		gcc -O3 -Wall main.c -Ldest_dir/lib exts/**/*.o $(OBJS) $(RUBY_HDRS) $(EXT_OBJS) -lruby-static $(FS_LIB) $(LIBS) -o $@

main.c exts/**/*.o: Gemfile
		dest_dir/bin/ruby make_main.rb

$(FS_O): $(PWD)/bundle/bundler/setup.rb $(FS_CLI) $(RUBY_SRC)
		$(FS_CLI) $(PWD) $(SAMPLE_DIR)/ $(LOAD_PATHS) -- --start=$(RUBY_SRC) --ruby-static=$(RUBY_LIB)

$(FS_CLI) $(FS_LIB): ./kompo_cli/src/*.rs ./kompo_cli/src/*.rb ./kompo_fs/src/*.rs ./kompo_wrap/src/*.rs
		cargo build -v

$(PWD)/bundle/bundler/setup.rb: Gemfile
		cd test_rails && \
		../dest_dir/bin/ruby -rbundler -rbundler/installer/standalone -e 'system ["../dest_dir/bin/bundler", "install", "--standalone"].join(" ")'
$(RUBY_LIB): ruby/*.c
$(RUBY_LIB): $(RUBY_CONF)
		$(MAKE) V=1 -C ruby -i install

$(RUBY_CONF):
		cd ruby && $(AUTOXXX) && ./configure --prefix=$(PWD)/dest_dir --disable-install-doc --disable-install-rdoc --disable-install-capi --with-static-linked-ext --with-ext=$(EXTS) --with-ruby-pc=ruby.pc

a.out: /workspaces/ruby_packager/kompo_storage/src/lib.rs /workspaces/ruby_packager/kompo_wrap/src/lib.rs /workspaces/ruby_packager/kompo_fs/**/*.rs $(RUBY_LIB) $(PWD)/bundle/bundler/setup.rb main.c fs.c
		cargo build --release
		gcc -g $(shell pkg-config --cflags dest_dir/lib/pkgconfig/ruby.pc) -Iruby -mbranch-protection=pac-ret -fstack-protector-strong -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2  -O3 -fno-fast-math -ggdb3 -Wall -Wextra -Wdeprecated-declarations -Wdiv-by-zero -Wduplicated-cond -Wimplicit-function-declaration -Wimplicit-int -Wpointer-arith -Wwrite-strings -Wold-style-definition -Wimplicit-fallthrough=0 -Wmissing-noreturn -Wno-cast-function-type -Wno-constant-logical-operand -Wno-long-long -Wno-missing-field-initializers -Wno-overlength-strings -Wno-packed-bitfield-compat -Wno-parentheses-equality -Wno-self-assign -Wno-tautological-compare -Wno-unused-parameter -Wno-unused-value -Wsuggest-attribute=format -Wsuggest-attribute=noreturn -Wunused-variable -Wmisleading-indentation -Wundef   -L. -fstack-protector-strong -rdynamic -Wl,-export-dynamic -fstack-protector-strong -Wl,--compress-debug-sections=zlib -L. -fstack-protector-strong -rdynamic -Wl,-export-dynamic \
		main.c fs.c ruby/ext/extinit.o ruby/ext/cgi/escape/escape.a ruby/ext/continuation/continuation.a ruby/ext/coverage/coverage.a ruby/ext/date/date_core.a ruby/ext/digest/digest.a ruby/ext/digest/bubblebabble/bubblebabble.a ruby/ext/digest/md5/md5.a ruby/ext/digest/rmd160/rmd160.a ruby/ext/digest/sha1/sha1.a ruby/ext/digest/sha2/sha2.a ruby/ext/erb/escape/escape.a ruby/ext/etc/etc.a ruby/ext/fcntl/fcntl.a ruby/ext/io/nonblock/nonblock.a ruby/ext/io/wait/wait.a ruby/ext/json/generator/generator.a ruby/ext/json/parser/parser.a ruby/ext/monitor/monitor.a ruby/ext/objspace/objspace.a ruby/ext/openssl/openssl.a ruby/ext/pathname/pathname.a ruby/ext/psych/psych.a ruby/ext/pty/pty.a ruby/ext/rbconfig/sizeof/sizeof.a ruby/ext/ripper/ripper.a ruby/ext/socket/socket.a ruby/ext/stringio/stringio.a ruby/ext/strscan/strscan.a ruby/ext/zlib/zlib.a ruby/enc/encinit.o ruby/enc/libenc.a ruby/enc/libtrans.a -Wl,-rpath,/workspaces/ruby_packager/dest_dir/lib -L/workspaces/ruby_packager/dest_dir/lib \
		-L/workspaces/ruby_packager/target/release \
		exts/nio4r/*.o exts/sqlite3/*.o exts/puma_http11/*.o \
		exts/websocket-driver/websocket_mask.o exts/nokogiri/*.o exts/cparse/cparse.o exts/console/console.o exts/bigdecimal/*.o exts/debug/*.o exts/parser/*.o exts/bootsnap/*.o exts/msgpack/*.o exts/generator/*.o exts/mri/*.o exts/skiptrace/*.o \
		exts/ed25519_ref10/ed25519_ref10.o exts/ed25519_ref10/fe.o exts/ed25519_ref10/ge.o exts/ed25519_ref10/keypair.o exts/ed25519_ref10/open.o exts/ed25519_ref10/sc_muladd.o exts/ed25519_ref10/sc_reduce.o exts/ed25519_ref10/sign.o exts/ed25519_ref10/verify.o \
		/workspaces/ruby_packager/test_rails/bundle/ruby/3.5.0+0/gems/nokogiri-1.18.3/ports/aarch64-linux/libxml2/2.13.6/lib/libxml2.a \
		/workspaces/ruby_packager/test_rails/bundle/ruby/3.5.0+0/gems/nokogiri-1.18.3/ports/aarch64-linux/libxslt/1.1.42/lib/libexslt.a \
		/workspaces/ruby_packager/test_rails/bundle/ruby/3.5.0+0/gems/nokogiri-1.18.3/ports/aarch64-linux/libxslt/1.1.42/lib/libxslt.a \
		/workspaces/ruby_packager/test_rails/bundle/ruby/3.5.0+0/gems/nokogiri-1.18.3/ext/nokogiri/ports/aarch64-linux/libgumbo/1.0.0-nokogiri/lib/libgumbo.a \
		/workspaces/ruby_packager/test_rails/bundle/ruby/3.5.0+0/gems/sqlite3-2.6.0/ports/aarch64-linux-gnu/sqlite3/3.49.1/lib/libsqlite3.a \
		-lruby-static -lrt -lgmp -lcrypt -lm -ldl -lffi -lssl -lcrypto -lyaml -lz -lkompo_fs -lkompo_wrap -lpthread -lc
# test_rails dest_dir/lib/ruby/3.5.0+0 dest_dir/lib/ruby/gems/3.5.0+0/specifications dest_dir/lib/ruby
fs.c: make_fs_data.rb sample/test.rb sample/dir/test1.rb test_rails
		ruby make_fs_data.rb ./main.rb main.rb test_rails dest_dir/lib/ruby

# test_rails:
# 	cp -r test_rails_tmp test_rails
# /workspaces/ruby_packager/test_rails/bundle/ruby/3.5.0+0/gems/nokogiri-1.18.1/ports/aarch64-linux/libxslt/1.1.42/lib/libexslt.a \
# 		/workspaces/ruby_packager/test_rails/bundle/ruby/3.5.0+0/gems/nokogiri-1.18.1/ports/aarch64-linux/libxslt/1.1.42/lib/libxslt.a \
# 		/workspaces/ruby_packager/test_rails/bundle/ruby/3.5.0+0/gems/nokogiri-1.18.1/ports/aarch64-linux/libxml2/2.13.5/lib/libxml2.a \
# 	  /workspaces/ruby_packager/test_rails/bundle/ruby/3.5.0+0/gems/nokogiri-1.18.1/ext/nokogiri/ports/aarch64-linux/libgumbo/1.0.0-nokogiri/lib/libgumbo.a \
# 		/workspaces/ruby_packager/test_rails/bundle/ruby/3.5.0+0/gems/sqlite3-2.5.0/ports/aarch64-linux-gnu/sqlite3/3.47.2/lib/libsqlite3.a \

test: a.out
	./a.out -e"p File.read('./main.c');p Dir.getwd; p Dir.open('dest_dir'){_1.fileno;_1.each{|d|p _1.tell};_1.seek(0)};Dir.delete('hoge');Dir.mkdir('hoge');p Dir.exist?('hoge');File.delete('fuga');File.symlink('hoge', 'fuga');File.readlink('fuga');Dir.chdir('hoge');exec('ls');"

PHONY: clean clear
clean:
		$(RM) a.out
		$(RM) main.c
		$(RM) $(OUTPUT)
		$(RM) $(OBJS)
		$(RM) -r exts/
		$(RM) -r bundle/
		cargo clean

clear: clean
		$(RM) -r dest_dir
		$(RM) $(RUBY_CONF)
		$(MAKE) -C ruby clean

#  /workspaces/ruby_packager/ruby/ext/extinit.c /workspaces/ruby_packager/ruby/ext/monitor/monitor.o

# gcc -mbranch-protection=pac-ret -fstack-protector-strong -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2  -O3 -fno-fast-math -ggdb3 -Wall -Wextra -Wdeprecated-declarations -Wdiv-by-zero -Wduplicated-cond -Wimplicit-function-declaration -Wimplicit-int -Wpointer-arith -Wwrite-strings -Wold-style-definition -Wimplicit-fallthrough=0 -Wmissing-noreturn -Wno-cast-function-type -Wno-constant-logical-operand -Wno-long-long -Wno-missing-field-initializers -Wno-overlength-strings -Wno-packed-bitfield-compat -Wno-parentheses-equality -Wno-self-assign -Wno-tautological-compare -Wno-unused-parameter -Wno-unused-value -Wsuggest-attribute=format -Wsuggest-attribute=noreturn -Wunused-variable -Wmisleading-indentation -Wundef   -L. -fstack-protector-strong -rdynamic -Wl,-export-dynamic -fstack-protector-strong -Wl,--compress-debug-sections=zlib -L. -fstack-protector-strong -rdynamic -Wl,-export-dynamic ../main.c ext/extinit.o ext/cgi/escape/escape.a ext/continuation/continuation.a ext/coverage/coverage.a ext/date/date_core.a ext/digest/digest.a ext/digest/bubblebabble/bubblebabble.a ext/digest/md5/md5.a ext/digest/rmd160/rmd160.a ext/digest/sha1/sha1.a ext/digest/sha2/sha2.a ext/erb/escape/escape.a ext/etc/etc.a ext/fcntl/fcntl.a ext/fiddle/fiddle.a ext/io/console/console.a ext/io/nonblock/nonblock.a ext/io/wait/wait.a ext/json/generator/generator.a ext/json/parser/parser.a ext/monitor/monitor.a ext/objspace/objspace.a ext/openssl/openssl.a ext/pathname/pathname.a ext/psych/psych.a ext/pty/pty.a ext/rbconfig/sizeof/sizeof.a ext/ripper/ripper.a ext/socket/socket.a ext/stringio/stringio.a ext/strscan/strscan.a ext/zlib/zlib.a enc/encinit.o enc/libenc.a enc/libtrans.a -Wl,-rpath,/workspaces/ruby_packager/dest_dir/lib -L/workspaces/ruby_packager/dest_dir/lib -lruby-static -lz -lrt -lrt -lgmp -ldl -lcrypt -lm -lpthread  -lz -lrt -lrt -lgmp -ldl -lcrypt -lm -lpthread  -ldl -lffi -lssl -lcrypto -lyaml -lz
# gcc $(pkg-config --cflags ruby.pc) -mbranch-protection=pac-ret -fstack-protector-strong -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2  -O3 -fno-fast-math -ggdb3 -Wall -Wextra -Wdeprecated-declarations -Wdiv-by-zero -Wduplicated-cond -Wimplicit-function-declaration -Wimplicit-int -Wpointer-arith -Wwrite-strings -Wold-style-definition -Wimplicit-fallthrough=0 -Wmissing-noreturn -Wno-cast-function-type -Wno-constant-logical-operand -Wno-long-long -Wno-missing-field-initializers -Wno-overlength-strings -Wno-packed-bitfield-compat -Wno-parentheses-equality -Wno-self-assign -Wno-tautological-compare -Wno-unused-parameter -Wno-unused-value -Wsuggest-attribute=format -Wsuggest-attribute=noreturn -Wunused-variable -Wmisleading-indentation -Wundef   -L. -fstack-protector-strong -rdynamic -Wl,-export-dynamic -fstack-protector-strong -Wl,--compress-debug-sections=zlib -L. -fstack-protector-strong -rdynamic -Wl,-export-dynamic ../main.c ext/extinit.o ext/cgi/escape/escape.a ext/continuation/continuation.a ext/coverage/coverage.a ext/date/date_core.a ext/digest/digest.a ext/digest/bubblebabble/bubblebabble.a ext/digest/md5/md5.a ext/digest/rmd160/rmd160.a ext/digest/sha1/sha1.a ext/digest/sha2/sha2.a ext/erb/escape/escape.a ext/etc/etc.a ext/fcntl/fcntl.a ext/fiddle/fiddle.a ext/io/console/console.a ext/io/nonblock/nonblock.a ext/io/wait/wait.a ext/json/generator/generator.a ext/json/parser/parser.a ext/monitor/monitor.a ext/objspace/objspace.a ext/openssl/openssl.a ext/pathname/pathname.a ext/psych/psych.a ext/pty/pty.a ext/rbconfig/sizeof/sizeof.a ext/ripper/ripper.a ext/socket/socket.a ext/stringio/stringio.a ext/strscan/strscan.a ext/zlib/zlib.a enc/encinit.o enc/libenc.a enc/libtrans.a -Wl,-rpath,/workspaces/ruby_packager/dest_dir/lib -L/workspaces/ruby_packager/dest_dir/lib -L/workspaces/ruby_packager/target/debug /workspaces/ruby_packager/exts/sqlite3/*.o  -lsqlite3 -lruby-static -lz -lrt -lrt -lgmp -ldl -lcrypt -lm -lpthread  -lz -lrt -lrt -lgmp -ldl -lcrypt -lm -lpthread  -ldl -lffi -lssl -lcrypto -lyaml -lz -lkompo_wrap -lc

# gcc main.c -Wall -O3 -Wextra -rdynamic -Wl,-export-dynamic $(shell pkg-config --cflags /workspaces/ruby_packager/dest_dir/lib/pkgconfig/ruby.pc) -L/workspaces/ruby_packager/target/debug -L/workspaces/ruby_packager/dest_dir/lib -Wl,--start-group exts/**/*.o $(EXT_OBJS) -Wl,-Bstatic -lsqlite3 -lruby-static -lffi -lssl -lcrypto -lyaml -lgmp -lcrypt -lz -lkompo_wrap -Wl,-Bdynamic -lm -lc -Wl,--end-group
