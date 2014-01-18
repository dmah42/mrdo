func funcstar()
  printd(42);
done;

func varstar()
  var a = 42;
  printd(a);
done;

func forstar(n)
  for i = 0, i lt n do
    printd(42+i);
    var b = 20;
    printd(b);
  done;
done;

func ifstar(a)
  if a eq 42 do
    printd(42);
  done;
done;

func ifelsestar(a)
  if a eq 42 do
    printd(20);
  else
    printd(42);
  done;
done;

printd(42);
funcstar();
forstar(3);
ifstar(42);
ifelsestar(41);

done;
