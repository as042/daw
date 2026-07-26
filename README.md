# daw

Simple tool for generating .wav files. A song is described entirely in plain-text TOML: a
`.project` file lists the `.track` and `.effect` files that make it up plus the output encoding,
each `.track` file is a sequence of notes with pitch, timing, dynamic, and channel, and each
`.effect` file applies a fade or reverb to chosen tracks over a time range. Running the binary and
giving it a path to a `.project` file synthesizes every note, applies the effects, and writes the
mixed result to a `.wav` next to the project. Notes are rendered by an internal subtractive synth,
so there are no samples or external audio libraries involved.

## Example

Given `Projects/TheLick/lick.project`:

```toml
tracks = ["lick.track"]

[settings]
num_channels = 2
sample_rate = 44100
bytes_per_sample = 2
```

and a `lick.track` alongside it:

```toml
start = 0
duration = 2
note = "C4"
dynamic = "MF"
instrument = "SubtractiveSynth"
channels = "All"

start = 2
duration = 2
notes = ["E3", "E4", "E5"]
```

run the binary and paste in the path to the project file:

```
cargo run --release
Projects/TheLick/lick.project
```

It prints progress as it goes and writes `lick.wav`. `Projects/ExampleProject` is a fuller example
with both effect types attached, and every field in these files is documented by comments inside the
example files themselves.