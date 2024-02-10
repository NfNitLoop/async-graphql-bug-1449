Bug
===

This demonstrates the bug from
<https://github.com/async-graphql/async-graphql/issues/1449>

`cargo run` this code, then enter the query:

```graphql
query {
    foo
    bar
}
```

The result does not include the result of "foo", even though it succeeded. Instead, we get:

```json
{
  "data": null,
  "errors": [
    {
      "message": "Some error",
      "locations": [
        {
          "line": 3,
          "column": 3
        }
      ],
      "path": [
        "bar"
      ]
    }
  ]
}
```


Workaround
-----------

Aha! If you return an `Option<Result<T>>`, things work as expected:

```graphql
query {
  foo
  # bar
  inverted
}
```

```json
{
  "data": {
    "foo": "Hello from foo()",
    "inverted": null
  },
  "errors": [
    {
      "message": "Some error",
      "locations": [
        {
          "line": 4,
          "column": 3
        }
      ],
      "path": [
        "inverted"
      ]
    }
  ]
}
```

But this seems to be the wrong type, IMO. My function should return either:
 * An `Err`
 * A nullable value (`Option<T>`, via the `Ok` value.)

Which would be a `Result<Option<T>>`

The type `Option<Result<T>>` seems to return either:
 * no value at all. (Which is neither an `Err` or an `Ok`!)
 * A `Result`, whose inner `Ok` variant is non-nillable.

Either way, it's very surprising that both `Result<Option<T>>` and `Option<Result<T>>`
both result in declaring a nullable field in the resulting graphql schema,
but only one of them handles errors as expected.
