EXEC=Binance-Trader-Bot

all: check run

check:
	@if [ -f $(EXEC) ]; then \
		echo "$(EXEC) found"; \
		make run; \
	else \
		echo "$(EXEC) not found"; \
		make install; \
	fi

install: clean
	cargo build
	mv target/debug/Binance-Trader-Bot .

build:
	cargo build

run:
	./Binance-Trader-Bot

clean:
	cargo clean
	rm -rf $(EXEC)

re: clean all
