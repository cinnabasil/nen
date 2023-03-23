# Function \_call\_extern

```
impure func _call_extern(func: string);
```

Call an external libc function.

## Errors

`_call_extern()` can fail if:

- The function specifies does not exist.
- The program could not be linked with libc.
- The libc function itself fails, in which case the error will be propagated.
