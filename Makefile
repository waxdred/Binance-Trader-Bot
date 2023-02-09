install:
	cargo build
	mv target/debug/Binance-Trader-Bot .

build:
	cargo build

run:
	./Binance-Trader-Bot

clean:
	rm -rf Binance-Trader-Bot
