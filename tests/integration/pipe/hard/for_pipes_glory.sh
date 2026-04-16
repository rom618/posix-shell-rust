#echo a | cat -e | wc -c | grep "[0-9]+" | sed -e "s/2/4/g" | sed -e "s/3/666666/g" | sed -e "s/$/#000/g" | cat -e | wc -c
