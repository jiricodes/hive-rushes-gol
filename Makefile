# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: jnovotny <jnovotny@student.hive.fi>        +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2022/06/28 09:32:05 by jnovotny          #+#    #+#              #
#    Updated: 2022/06/28 11:41:53 by jnovotny         ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

MAKEOPTS=--no-print-directory

BIN_DIR=bins/

TARGETS=target/release/simple


all:
	cargo test
	make folders $(MAKEOPTS)
	cargo build --release
	cp $(TARGETS) $(BIN_DIR)

folders:
	@mkdir -p $(BIN_DIR)


run-simple:
	cargo test -p simple
	make folders $(MAKEOPTS)
	cargo build -p simple --release
	cp target/release/simple $(BIN_DIR)
	(cd resources; ./test.sh ../$(BIN_DIR)/simple)

run-opt_01:
	cargo test -p opt_01
	make folders $(MAKEOPTS)
	cargo build -p opt_01 --release
	cp target/release/opt_01 $(BIN_DIR)
	(cd resources; ./test.sh ../$(BIN_DIR)/opt_01)

clean:
	cargo clean
	rm -r $(BIN_DIR)