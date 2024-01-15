# rcss
CSS in Rust


# struct

## declaration:
- selectors
- rules or declarations

```scss
h1,h2 {
    font-family: sans-serif;
    color: red;
    div {
      color: blue;
    }
}
```

https://www.w3.org/TR/CSS2/selector.html#q5.0
### selector:
Type selectors: `h1`
Descendant selectors: `h1 em`
Child selectors:`body>P`
Adjacent sibling selectors: `h1,h2,body>P,h1 + h2,h1.opener`
Attribute selectors 1 (Matching attributes and attribute values): `[att=val]`
Attribute selectors 2 (Default attribute values in DTDs): `EXAMPLE[notation=decimal]`
Attribute selectors 2 (Class selectors): `H1.pastoral`
H1.pastoral: `h1#chapter1`