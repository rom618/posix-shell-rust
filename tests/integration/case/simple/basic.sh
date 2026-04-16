val="apple"

case "$val" in
  apple)
    echo "It is apple" ;;
  banana)
    echo "It is banana" ;;
  *)
    echo "It is something else" ;;
esac

val="123"

case "$val" in
  [0-9]*)
    echo "Number detected" ;;
  *)
    echo "Not a number" ;;
esac
