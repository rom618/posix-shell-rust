val="test"

case "$val" in
  [a-z) )
    echo "This should never run" ;;
esac
