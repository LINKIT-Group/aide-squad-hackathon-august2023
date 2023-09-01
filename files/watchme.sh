#!/bin/sh

APP_DIRECTORY=${APP_DIRECTORY:-`pwd`}
APP_NAME=`\
    grep "^name =" ${APP_DIRECTORY}/Cargo.toml |\
    grep -o "\".*\"" |tr -d "\"" || exit 1`
HTML_DIRECTORY=/html
PORT=8080

if [ "$1" = "serve" ];then
    cd "${HTML_DIRECTORY}" || exit 1
    cp -r "${APP_DIRECTORY}"/public/* . || exit 1
    basic-http-server -a 0.0.0.0:${PORT} .
elif [ "$1" = "update" ];then
    HASH=$(cat /dev/urandom | tr -dc 'a-zA-Z0-9' | fold -w 10 | head -n 1)
    rm "${HTML_DIRECTORY}"/*.wasm 2>/dev/null
    cp "${APP_DIRECTORY}"/target/wasm32-unknown-unknown/release/"${APP_NAME}".wasm \
        "${HTML_DIRECTORY}"/${HASH}.wasm || exit 1
    sed "\
        s:{{ *WASM_FILE *}}:${HASH}.wasm:g;\
        s:{{ *APP_NAME *}}:${APP_NAME}:g" \
        "${APP_DIRECTORY}"/public/index.html > ${HTML_DIRECTORY}/index.html \
        || exit 1
    echo "${APP_NAME}  updated succesfully refresh browser"
else
    cd "${APP_DIRECTORY}" || exit 1
    # start http server in background
    sh $0 serve &
    # on change: compile and update html directory with new wasm file
    cargo watch -x "\
        build --release --target wasm32-unknown-unknown && sh $0 update"
fi

exit 0
