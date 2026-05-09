TARGET := mapfile

CC := /opt/riscv64-linux-musl-cross/bin/riscv64-linux-musl-gcc
STRIP := /opt/riscv64-linux-musl-cross/bin/riscv64-linux-musl-strip

all: $(TARGET)

%: %.c
	$(CC) -static $< -o $@
	$(STRIP) $@

clean:
	@rm -rf ./$(TARGET)
