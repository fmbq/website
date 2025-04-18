.PHONY: run
run:
	cargo watch -i src/web/scss -x run

.PHONY: images
images: wwwroot/favicon.ico wwwroot/icon.svg wwwroot/apple-touch-icon.png

wwwroot/favicon.ico: Makefile
	inkscape --export-type=png --export-filename=- --export-width=32 --export-area-page design-sources/logo/FMBQ-logo-no-text.svg | magick - -format ico $@

wwwroot/icon.svg: Makefile
	inkscape --export-type=svg --export-filename=$@ --export-area-page design-sources/logo/FMBQ-logo-no-text.svg

wwwroot/apple-touch-icon.png: Makefile
	inkscape --export-type=png --export-filename=$@ --export-width=180 --export-area-page design-sources/logo/FMBQ-logo-no-text.svg
