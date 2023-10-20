DST_DIR = build
WASM_PACK_TARGET = web
WASM_PACK_BUILD_OPTS = --target $(WASM_PACK_TARGET) --out-dir $(DST_DIR)
STATIC_FILES = $(wildcard static/*)

all: static build

dir:
	mkdir -p $(DST_DIR)

static: dir $(STATIC_FILES)
	cp $(STATIC_FILES) $(DST_DIR)/

.PHONY: build
build:
	wasm-pack build $(WASM_PACK_BUILD_OPTS)

