false && echo NO
true  && echo YES
false || echo ALT
true  || echo NO

[ 1 -eq 1 ] && echo EQUAL
[ 1 -eq 2 ] || echo NOT EQUAL

echo START && echo MIDDLE && echo END
echo FIRST || echo "SHOULD NOT PRINT" || echo SECOND
false && echo SHOULD NOT PRINT || echo PRINTED
true  || echo SHOULD NOT PRINT && echo ALSO NOT PRINTED
( false && echo SUBSHELL NO ) || echo SUBSHELL YES
( true  || echo SUBSHELL NO ) && echo SUBSHELL YES
echo A && false || echo B && true || echo C
echo X || true && echo Y || false && echo Z
echo P && ( false || echo Q ) && echo R
echo L || ( true && echo M ) || echo N
echo U && ( true && echo V ) && echo W
echo D || ( false || echo E ) || echo F
