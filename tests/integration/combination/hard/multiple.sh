echo A; echo B; echo C
echo D
echo E; echo F

echo "hello world"
echo 'hello world'
echo "a  b   c"
echo 'a  b   c'

x=OK
echo "X=$x"
echo 'X=$x'
echo "A'$x'B"
echo 'A' "$x" 'B'

echo "argc=$#"
for v in A B C do
  echo "[$v]"
done

{ echo A; echo B; echo C; }
echo D

a=hello b=world echo "$a $b"
a=foo
b=bar
a=xx b=yy echo "$a-$b"

a=1
echo $a
a=2 echo $a
echo $a

VAR=base
echo "$VAR"
VAR=tmp sh -c 'echo "$VAR"'
echo "$VAR"

VAR=base
echo '$VAR'
VAR=tmp sh -c 'echo "$VAR"'
echo "$VAR"
