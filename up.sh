#!/usr/bin/env bash
set -euxo pipefail

REMOTE=pi@172.31.180.12
DIAGWARE_DIR=/home/pi/Code/diagware-rs
CARGO=/home/pi/.cargo/bin/cargo
RSYNC_EXCLUDE_FILE=rsync_exclude.txt
# ENV=DATABASE_URL="sqlite://${DIAGWARE_DIR}/src/database.db"

function print_usage {
    echo "Usage: $0 <build | run>" 1>&2
    exit 1
}

function transfer {
    rsync --delete -r . ${REMOTE}:${DIAGWARE_DIR} --exclude-from=${RSYNC_EXCLUDE_FILE}
}

function run {
    ssh ${REMOTE} "export DISPLAY=:0; cd ${DIAGWARE_DIR}; ${ENV} ${CARGO} run --release --color always"
}

function build {
    ssh ${REMOTE} "cd ${DIAGWARE_DIR}; ${ENV} ${CARGO} build --release --color always"
}


[[ $# < 1 ]] && opt="run" || opt=$1

if [[ $# > 1 ]]; then
    echo "$0: too many arguments" 1>&2
    print_usage
fi


if [[ $opt == "run" ]]; then

    transfer
    run

elif [[ $opt == "xfer" ]]; then

    transfer

elif [[ $opt == "build" ]]; then

    transfer
    build

else
    echo "$0: invalid option: $opt" 1>&2
    print_usage
fi
