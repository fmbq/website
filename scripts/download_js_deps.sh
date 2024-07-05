#!/bin/sh
set -eu

cd js/vendor
rm -r ./*

mkdir -p htmx/1.9.3
mkdir -p htmx/1.9.3/ext
curl -L -o htmx/1.9.3/htmx.min.js https://unpkg.com/htmx.org@1.9.3/dist/htmx.min.js
curl -L -o htmx/1.9.3/ext/sse.js https://unpkg.com/htmx.org@1.9.3/dist/ext/sse.js

mkdir -p hyperscript/0.9.12
curl -L -o hyperscript/0.9.12/_hyperscript.min.js https://unpkg.com/hyperscript.org@0.9.12/dist/_hyperscript.min.js

mkdir -p editorjs/2.29.1
curl -L -o editorjs/2.29.1/editorjs.js https://cdn.jsdelivr.net/npm/@editorjs/editorjs@2.29.1/dist/editorjs.umd.min.js

mkdir -p editorjs-header/2.8.5
curl -L -o editorjs-header/2.8.5/header.js https://cdn.jsdelivr.net/npm/@editorjs/header@2.8.5/dist/header.umd.js
