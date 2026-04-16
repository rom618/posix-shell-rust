if true; then
  echo T
else
  echo F
fi

if false; then
  echo NO
else
  echo YES
fi


if false; then
  echo A
elif false; then
  echo B
elif true; then
  echo C
else
  echo D
fi

if true; then echo YES; else echo NO; fi
if false; then echo NO; else echo YES; fi

if if true; then true; else false; fi; then echo T2; else echo F2; fi

n=0
if [ $n -eq 0 ]; then echo "eq fonctionne"; else echo "eq fonctionne pas"; fi
if [ $n -le 0 ]; then echo "le fonctionne"; else echo "le fonctionne pas"; fi
if [ $n -ge 0 ]; then echo "ge fonctionne"; else echo "ge fonctionne pas"; fi
n=-1
if [ $n -ne 0 ]; then echo "ne fonctionne"; else echo "ne fonctionne pas"; fi
if [ $n -lt 0 ]; then echo "lt fonctionne"; else echo "lt fonctionne pas"; fi
n=1
if [ $n -gt 0 ]; then echo "gt fonctionne"; else echo "gt fonctionne pas"; fi

if true; then for i in 1 2 3 4; do echo toto; done; elif ! true; then echo not true; else echo tu rentres pas la; fi

if ! false | true; then echo ok; fi
