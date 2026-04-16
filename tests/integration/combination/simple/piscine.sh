#!/bin/sh

echo "My frais is $MY_ENV_FRAIS"

my_local_frais="Javotte"
echo "My frais is $my_local_frais"
my_local_frais="Pulpa"
echo "My frais is now $my_local_frais"

my_local_frais="Javotte"
echo "My frais is $my_local_frais"

[ $# -ne 1 ] && echo "Sorry, expected 1 argument but $# were passed" && exit 1
[ ! -f ${1} ] && echo "${1}:" && echo "\tis not a valid file" && exit 2
cat ${1}

if [ $# -ne 1 ]
then
    echo "Sorry, expected 1 argument but $# were passed"
    exit 1
else
    argu=${1}
    if [ -f $argu ]
    then
        cat $argu
    else
        echo "${1}:" && echo "\tis not a valid file"
        exit 2
    fi
fi

