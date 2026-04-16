IFS=:
x="a:b::c"
for i in $x; do
  echo ">$i<"
done
