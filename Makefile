# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: jnovotny <jnovotny@student.hive.fi>        +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2022/06/28 09:32:05 by jnovotny          #+#    #+#              #
#    Updated: 2022/06/28 13:27:41 by jnovotny         ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

MAKEOPTS=--no-print-directory

BIN_DIR=bins/

TARGETS=target/release/simple \
		target/release/opt_01 \
		target/release/opt_02


all:
	cargo test
	make folders $(MAKEOPTS)
	cargo build --release
	cp $(TARGETS) $(BIN_DIR)

folders:
	@mkdir -p $(BIN_DIR)

run-all:
	make all $(MAKEOPTS)
	(cd resources; ./test.sh ../$(BIN_DIR)/simple)
	(cd resources; ./test.sh ../$(BIN_DIR)/opt_01)
	(cd resources; ./test.sh ../$(BIN_DIR)/opt_02)

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

run-opt_02:
	cargo test -p opt_02
	make folders $(MAKEOPTS)
	cargo build -p opt_02 --release
	cp target/release/opt_02 $(BIN_DIR)
	(cd resources; ./test.sh ../$(BIN_DIR)/opt_02)

clean:
	cargo clean
	rm -r $(BIN_DIR)