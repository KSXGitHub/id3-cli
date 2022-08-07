# id3-cli

## Some CLI examples

### `id3 get`

#### `id3 get <text-field>`

```sh
id3 get title ~/Music/target-audio.mp3
```

```sh
id3 get artist ~/Music/target-audio.mp3
```

```sh
id3 get album ~/Music/target-audio.mp3
```

```sh
id3 get album-artist ~/Music/target-audio.mp3
```

```sh
id3 get genre-name ~/Music/target-audio.mp3
```

```sh
id3 get genre-code ~/Music/target-audio.mp3
```

#### `id3 get comment`

##### Print the only comment

```sh
id3 get comment ~/Music/target-audio.mp3
```

##### List multiple comments

```sh
id3 get comment --format=json ~/Music/target-audio.mp3
```

#### `id3 get picture`

##### List pictures

```sh
id3 get picture list --format=json ~/Music/target-audio.mp3
```

##### Export the only picture

```sh
id3 get picture file ~/Music/target-audio.mp3 ~/Pictures/output-picture.jpg
```

##### Export the a front cover picture amongst multiple pictures

```sh
id3 get picture file ~/Music/target-audio.mp3 ~/Pictures/exported-picture.jpg CoverFront
```

##### Export all pictures to a directory

```sh
id3 get picture dir ~/Music/target-audio.mp3 ~/Pictures/exported-pictures/
```

### `id3 set`

#### `id3 set <text-field>`

```sh
id3 set title ~/Music/target-audio.mp3 'Song title'
```

```sh
id3 set artist ~/Music/target-audio.mp3 'Song artist'
```

```sh
id3 set album ~/Music/target-audio.mp3 'Album'
```

```sh
id3 set album-artist ~/Music/target-audio.mp3 'Album Artist'
```

```sh
id3 set genre ~/Music/target-audio.mp3 '(10)'
```

#### `id3 set comment`

```sh
id3 set comment --language=eng ~/Music/target-audio.mp3 'My comment'
```

#### `id3 set picture`

```sh
id3 set picture ~/Music/target-audio.mp3 ~/Pictures/front-cover.jpg CoverFront
```

### `id3 delete`

#### `id3 delete all`

```sh
id3 delete all ~/Music/target-audio.mp3
```

#### `id3 delete <text-field>`

```sh
id3 delete title ~/Music/target-audio.mp3
```

```sh
id3 delete artist ~/Music/target-audio.mp3
```

```sh
id3 delete album ~/Music/target-audio.mp3
```

```sh
id3 delete album-artist ~/Music/target-audio.mp3
```

```sh
id3 delete genre ~/Music/target-audio.mp3
```

#### `id3 delete comment`

```sh
id3 delete comment ~/Music/target-audio.mp3
```

#### `id3 delete picture`

##### Delete all pictures

```sh
id3 delete picture ~/Music/target-audio.mp3 all
```

##### Delete just the front cover

```sh
id3 delete picture ~/Music/target-audio.mp3 CoverFront
```

## License

[MIT](https://github.com/KSXGitHub/id3-cli/blob/master/LICENSE.md) @ [Hoàng Văn Khải](https://ksxgithub.github.io/).
