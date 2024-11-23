#!/usr/bin/env bash
set -euxo pipefail


USER=pi
IP=10.0.0.253
REMOTE=${USER}@${IP}
DIAGWARE_DIR=/home/pi/Code/diagware-rs
CARGO=/home/pi/.cargo/bin/cargo
RSYNC_EXCLUDE_FILE=rsync_exclude.txt
ENV=DATABASE_URL="sqlite://${DIAGWARE_DIR}/src/database.db"


function print_usage {
    echo "Usage: $0 <build | run>" 1>&2
    exit 1
}

function transfer {
    rsync --delete -r . ${REMOTE}:${DIAGWARE_DIR} --exclude-from=${RSYNC_EXCLUDE_FILE}
}

function run {
    ssh ${REMOTE} "cd ${DIAGWARE_DIR}; ${ENV} ${CARGO} r -r --color always"
}

function build {
    ssh ${REMOTE} "cd ${DIAGWARE_DIR}; ${ENV} ${CARGO} b -r --color always"
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
