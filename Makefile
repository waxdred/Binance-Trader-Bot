EXEC=binance-trader-bot
RS_FILES := ./src/$(wildcard *.rs)

all: build run

build: ${RS_FILES}
	cargo build
	mv target/debug/binance-trader-bot .

run:
	./binance-trader-bot

clean:
	rm -rf ./Cargo.lock
	cargo clean
	rm -rf $(EXEC)

re: clean all

.PHONY: all
