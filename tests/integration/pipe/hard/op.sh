true && echo T1
false && echo NO1
false || echo O1
true  || echo NO2

true && false && echo NO3
true && false || echo O2
false || true && echo T2

true
echo "$?"
false
echo "$?"
true && true
echo "$?"
true && false
echo "$?"
false || true
echo "$?"
false || false
echo "$?"
echo "s=$?"
