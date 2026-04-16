x=1
echo test | (
  x=2
  echo "inside=$x"
)
echo "outside=$x"
