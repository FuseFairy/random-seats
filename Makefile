.PHONY: all build strip clean

all: build strip

build:
	cargo build --release --target x86_64-pc-windows-gnu

strip:
	strip target/x86_64-pc-windows-gnu/release/random_seats.exe || echo "Strip failed or not needed."

clean:
	rm -rf target
