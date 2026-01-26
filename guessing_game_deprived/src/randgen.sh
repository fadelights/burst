#!/usr/bin/env bash

if [[ $(echo $1 | wc -c) -gt 10 || $(echo $2 | wc -c) -gt 10 ]]
then
    echo "Big ass number."
    exit 1
fi

START_OF_RANGE=${1}
END_OF_RANGE=${2}

if [[ $1 -ge $2 ]]
then
    START_OF_RANGE=$(( ${START_OF_RANGE} + ${END_OF_RANGE} ))
    END_OF_RANGE=$(( ${START_OF_RANGE} - ${END_OF_RANGE} ))
    START_OF_RANGE=$(( ${START_OF_RANGE} - ${END_OF_RANGE} ))
fi

function normalize() {
    echo $(( ${1} % (${END_OF_RANGE} - ${START_OF_RANGE}) + ${START_OF_RANGE} ))
}

RANDOM_NUMBER=$(head /dev/urandom | od | head -1 | cut -f 8- -d ' ' | tr -d ' ')
normalize $RANDOM_NUMBER
