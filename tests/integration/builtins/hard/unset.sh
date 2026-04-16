export X=42
unset X
sh -c 'echo ">$X<"'
