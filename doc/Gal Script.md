# Gal Script

## Grammmar

Gal Script contains several instructions, ench instruction takes one line.

```text
[Instruction Type] [Arg 1] [Arg 2] ...
```

Instructions:

### `say` instruction

For non-character dialog:

```text
say [Text]
```

For character dialog:

```text
say [Text] [Character]
```

### `scene` instruction

Draw a scene.

```text
scene [Image Path]
```

### `label` instruction

Define a label.

```text
label [Label Name]
```

### `switch` instruction

Switch to a label.

```text
switch [Label Name]
```

External label switching.

```text
switch [Script Name]:[Label Name]
```
