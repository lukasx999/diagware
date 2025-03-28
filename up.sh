#!/usr/bin/env bash
set -euxo pipefail

REMOTE=pi@172.31.180.113
DIAGWARE_DIR=/home/pi/Code/diagware
RSYNC_EXCLUDE_FILE=rsync_exclude.txt

function print_usage {
    echo "Usage: $0 <build | run | stop>" 1>&2
    exit 1
}

function transfer {
    rsync --delete -r . ${REMOTE}:${DIAGWARE_DIR} --exclude-from=${RSYNC_EXCLUDE_FILE}
}

function stop {
    ssh ${REMOTE} "sudo pkill diagware"
}

function run {
    ssh ${REMOTE} "export DISPLAY=:0; cd ${DIAGWARE_DIR} && ./install.sh && sudo xinit ./diagware"
}

function build {
    ssh ${REMOTE} "export DISPLAY=:0; cd ${DIAGWARE_DIR} && ./install.sh"
}

[[ $# < 1 ]] && opt="run" || opt=$1

if [[ $# > 1 ]]; then
    echo "$0: too many arguments" 1>&2
    print_usage
fi

if [[ $opt == "run" ]]; then
    transfer
    run

elif [[ $opt == "stop" ]]; then
    stop

elif [[ $opt == "build" ]]; then
    transfer
    build

else
    echo "$0: invalid option: $opt" 1>&2
    print_usage
fi
