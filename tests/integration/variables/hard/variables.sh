a=hello
b=world
echo $a $b

name=ok
echo X${name}Y

echo $PWD
echo ${PWD}123
echo $OLDPWD
#echo $RANDOM #non deterministe
#echo ${RANDOM}42 #non deterministe

echo $*
echo $#
#echo $$
echo $?
echo $UID
#echo $IFS

var=1
var=2
echo $var
var=3
var=5
echo $var
