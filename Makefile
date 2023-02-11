EXEC=Binance-Trader-Bot
RS_FILES := ./src/$(wildcard *.rs)

all: build run

build: ${RS_FILES}
	cargo build
	mv target/debug/Binance-Trader-Bot .

run:
	./Binance-Trader-Bot

clean:
	rm -rf ./Cargo.lock
	cargo clean
	rm -rf $(EXEC)

re: clean all

.PHONY: all
