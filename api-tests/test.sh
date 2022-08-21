#!/usr/bin/env bash
set -e

inform () {
    printf "  [ \033[00;34m..\033[0m ] $1\n"
}

test () {
    inform 'Starting test...';
    START=95001
    END=285400
    for i in $(seq $START $END)
    do
        URL="http://0.0.0.0:8080/rss/anilist/staff/$i"
        STATUS_CODE=$(curl --silent --show-error --output /dev/null -w "%{http_code}" $URL)
        if [ $STATUS_CODE != "200" ]
        then
            echo  "$URL $STATUS_CODE"
        fi


        if [ $i%50 == "0" ]
        then
            sleep 55
        fi
    done
}

test
