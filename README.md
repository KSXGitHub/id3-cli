# id3-cli

## Some CLI examples

### `id3 get`

#### `id3 get <text-field>`

```sh
id3 get title /path/to/my/audio.mp3
```

```sh
id3 get artist /path/to/my/audio.mp3
```

```sh
id3 get album /path/to/my/audio.mp3
```

```sh
id3 get album-artist /path/to/my/audio.mp3
```

```sh
id3 get genre /path/to/my/audio.mp3
```

#### `id3 get comment`

##### Print the only comment

```sh
id3 get comment /path/to/my/audio.mp3
```

##### List multiple comments

```sh
id3 get comment --format=json /path/to/my/audio.mp3
```

#### `id3 get picture`

##### List pictures

```sh
id3 get picture list --format=json /path/to/my/audio.mp3
```

##### Export the only picture

```sh
id3 get picture file /path/to/my/audio.mp3 /path/to/the/exported/picture.jpg
```

##### Export the first picture of many

```sh
id3 get picture file --id=0 /path/to/my/audio.mp3 /path/to/the/exported/picture.jpg
```

##### Export all pictures to a directory

```sh
id3 get picture dir /path/to/my/audio.mp3 /path/to/the/output/directory/
```

## License

[MIT](https://github.com/KSXGitHub/id3-cli/blob/master/LICENSE.md) @ [Hoàng Văn Khải](https://ksxgithub.github.io/).
