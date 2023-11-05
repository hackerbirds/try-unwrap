# try-unwrap

A revolutionary new crate that allows you to unwrap() without making your stupid software panic.

Do you ever wish "man, I wish I could use unwrap() everywhere and not have my code panic"? Boy do we just have the answer for you! This revolutionary crate adds `try_unwrap()` to `Result` and `Option` types, which allow you to safely `unwrap()`!

How does it work? It's quite shrimple really. If your type is `Ok`, then try-unwrap will `unwrap()` successfully and return the type. If it ends up being an `Err`, then it will unwrap the error instead and return that for you! And don't worry about type strictness either: we return the value wrapped in a `Result` that supports both the `Ok` and `Err`!!

### Examples

Whether your type is `Result` or `Option`, using try-unwrap is super easy!

```
let ok: Result<i32, ()> = Ok(3);
assert_eq!(ok.try_unwrap(), Ok(3));

let err: Result<(), i32> = Err(2);
assert_eq!(err.try_unwrap(), Err(2));
```

```
let some: Option<i32> = Some(4);
assert_eq!(some.try_unwrap(), Some(4));

let none: Option<i32> = None;
assert_eq!(none.try_unwrap(), None);
```

### Wait, what?

We know! It's crazy that no one has thought of it before. But don't worry no more, because we did it.

### Isn't that, like, useless?

What? Of course not!

### How did you do it?

The answer to all your burning questions are here: https://hackerbirds.neocities.org/try-unwrap/
