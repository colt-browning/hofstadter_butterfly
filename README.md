Draw [Hofstadter's butterfly](https://en.wikipedia.org/wiki/Hofstadter%27s_butterfly).

To reproduce [the vector plot](https://commons.wikimedia.org/wiki/File:Hofstadter%27s_butterfly_vector_70.svg):
`cargo run --release --bin --main d intervals_upto 70 >out.txt`
`cat out.txt | cargo run --release --bin --txt2img svg`

Licensed under [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0) or [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/) at your option.
