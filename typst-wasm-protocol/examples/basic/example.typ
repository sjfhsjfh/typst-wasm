#let p = plugin("target/wasm32-unknown-unknown/release/basic.wasm")

#assert.eq(
  str(p.to_uppercase(bytes("Hello, Typst!"))),
  "HELLO, TYPST!",
)

#assert.eq(str(p.shout(bytes("hello"))), "HELLO")

#assert.eq(str(p.echo(bytes("raw bytes"))), "raw bytes")

#assert.eq(str(p.count_chars(bytes("hello"))), "5")

#let json_out = json(
  p.transform_json(bytes(json.encode((text: "ab", repeat: 2)))),
)
#assert.eq(json_out.text, "abab")
#assert.eq(json_out.bytes, 4)
#assert.eq(json_out.repeat, 2)

#let cbor_out = cbor(
  p.transform_cbor(cbor.encode((text: "ab", repeat: 3))),
)
#assert.eq(cbor_out.text, "ababab")
#assert.eq(cbor_out.bytes, 6)
#assert.eq(cbor_out.repeat, 3)

#assert.eq(str(p.checked_add(bytes("2"), bytes("3"))), "5")

#text("All examples passed.")
