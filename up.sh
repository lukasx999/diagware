#!/usr/bin/env bash
set -euxo pipefail


USER=pi
IP=10.0.0.253
REMOTE=${USER}@${IP}
DIAGWARE_DIR=/home/pi/Code/diagware-rs
CARGO=/home/pi/.cargo/bin/cargo


function print_usage {
    echo "Usage: $0 <build | run>" 1>&2
    exit 1
}


[[ $# < 1 ]] && opt="build" || opt=$1

if [[ $# > 1 ]]; then
    echo "$0: too many arguments" 1>&2
    print_usage
fi


if [[ $opt == "run" ]]; then

    ssh ${REMOTE} "cd ${DIAGWARE_DIR}; ${CARGO} r -r --color always"

elif [[ $opt == "build" ]]; then

    rsync -r . ${REMOTE}:${DIAGWARE_DIR} --exclude-from='rsync_exclude.txt'
    ssh ${REMOTE} "cd ${DIAGWARE_DIR}; DATABASE_URL="sqlite://${DIAGWARE_DIR}/src/database.db" ${CARGO} b -r --color always"

else
    echo "$0: invalid option: $opt" 1>&2
    print_usage
fi
