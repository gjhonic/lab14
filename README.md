# Laboratory work №14

**Install Projects**

```bash
git clone git@github.com:gjhonic/lab14.git
```

**Install Docker**

```bash
sudo pacman -S docker
sudo pacman -S docker-compose
```

**launch**

```bash
# Start docker (not work in windows wsl)
sudo systemctl start docker

#if you use windows wsl then need command
sudo service docker start

cd lab14

make upd
```

**Test**

```bash
$ curl http://127.0.0.1/v1/todos/{your_guid}
> {"assigned":"user@example.com","id":"{your_guid}","message":"Test message","priority":"A"}
```