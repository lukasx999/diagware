#!/usr/bin/env bash
set -euxo pipefail


USER=pi
IP=10.0.0.253
REMOTE=${USER}@${IP}
DIAGWARE_DIR=/home/pi/Code/diagware-rs
CARGO=/home/pi/.cargo/bin/cargo
RSYNC_EXCLUDE_FILE=rsync_exclude.txt


function print_usage {
    echo "Usage: $0 <build | run>" 1>&2
    exit 1
}

function run {
    ssh ${REMOTE} "cd ${DIAGWARE_DIR}; ${CARGO} r -r --color always"
}

function build {
    rsync -r . ${REMOTE}:${DIAGWARE_DIR} --exclude-from=${RSYNC_EXCLUDE_FILE}
    ssh ${REMOTE} "cd ${DIAGWARE_DIR}; DATABASE_URL="sqlite://${DIAGWARE_DIR}/src/database.db" ${CARGO} b -r --color always"
}


[[ $# < 1 ]] && opt="build_and_run" || opt=$1

if [[ $# > 1 ]]; then
    echo "$0: too many arguments" 1>&2
    print_usage
fi


if [[ $opt == "run" ]]; then

    run

elif [[ $opt == "build" ]]; then

    build

elif [[ $opt == "build_and_run" ]]; then

    build
    run

else
    echo "$0: invalid option: $opt" 1>&2
    print_usage
fi
