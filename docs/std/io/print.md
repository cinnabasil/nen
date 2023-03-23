# Function std.io.print

```nen
impure func print(input: string);
```

Prints to the standard output.

```nen
from io include print;

impure func main() {
    print("Hello, world!");
}
```

## Errors

`std.io.print` can fail if writing to `stdout` fails.