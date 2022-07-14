# id3-cli

## Some CLI examples

### `id3 get`

#### `id3 get <text-field>`

```sh
id3 get title ~/Music/input-audio.mp3
```

```sh
id3 get artist ~/Music/input-audio.mp3
```

```sh
id3 get album ~/Music/input-audio.mp3
```

```sh
id3 get album-artist ~/Music/input-audio.mp3
```

```sh
id3 get genre ~/Music/input-audio.mp3
```

#### `id3 get comment`

##### Print the only comment

```sh
id3 get comment ~/Music/input-audio.mp3
```

##### List multiple comments

```sh
id3 get comment --format=json ~/Music/input-audio.mp3
```

#### `id3 get picture`

##### List pictures

```sh
id3 get picture list --format=json ~/Music/input-audio.mp3
```

##### Export the only picture

```sh
id3 get picture file ~/Music/input-audio.mp3 ~/Pictures/output-picture.jpg
```

##### Export the first picture of many

```sh
id3 get picture file --id=0 ~/Music/input-audio.mp3 ~/Pictures/exported-picture.jpg
```

##### Export all pictures to a directory

```sh
id3 get picture dir ~/Music/input-audio.mp3 ~/Pictures/exported-pictures/
```

## License

[MIT](https://github.com/KSXGitHub/id3-cli/blob/master/LICENSE.md) @ [Hoàng Văn Khải](https://ksxgithub.github.io/).
