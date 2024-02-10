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