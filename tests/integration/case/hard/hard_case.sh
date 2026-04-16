val1="abc123"
val2="file.TXT"
val3="!special?"

echo "Testing tricky case statements..."

case "$val1" in
  a*b[0-9])
    echo "val1 matches a*b[0-9]" ;;
  *)
    echo "val1 no match" ;;
esac

case "$val2" in
  *.txt|*.TXT)
    echo "val2 is text file" ;;
  *)
    echo "val2 no match" ;;
esac

case "$val3" in
  [!\a-zA-Z0-9]*)
    echo "val3 starts with a special character" ;;
  *)
    echo "val3 normal" ;;
esac

case "$val1$val2" in
  abc*|file.*)
    echo "val1+val2 matched multiple patterns" ;;
esac
val4="hello world"
case "$val4" in
  "hello world") echo "val4 exact match" ;;
esac
