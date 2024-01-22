```ts
const message = "type(scope): subject";
const actual = await parse(message);
const expected = {
  body: null,
  footer: null,
  header: "type(scope): subject",
  mentions: [],
  merge: null,
  notes: [],
  raw: "type(scope): subject",
  references: [],
  revert: null,
  scope: "scope",
  subject: "subject",
  type: "type",
};

expect(actual).toMatchObject(expected);
```
