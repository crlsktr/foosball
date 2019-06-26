# Copy Down the database in powershell
```
scp foos@foos:foosball/foosball.db foosball_real.db
```

# Sqlite database option

## set the mode to columns

```
.mode column
```

## turn on the headers
```
.headers on
```

## quit sqlite
```
.quit
```


## Docker

All of these Docker commands might not be quite right I need to test them....

### Docker build foosball
change the version 
```
docker build -t foosball:0.1.1 .
```

use no cache to build without the cache
```
docker build --no-cache -t foosball:0.1.1 .
```

### Running through docker
change the version
```
docker run -v C:\dev\foosball\data:/data -v  C:\dev\foosball\templates:/templates -v C:\dev\foosball\static_files:/static_files -p 12346:12346 foosball:0.1.1
```

### Docker tag foosball
change the version
```
docker tag foosball danwilkins8/foosball:0.1.1
```

### Docker push
change the version
```
docker push danwilkins8/foosball:0.1.1
```


### Docker Deploy
According to chris this is how to do it until the swarm is created.
```
 docker login
 docker-compose -f /mnt/inf/foos/foos-compose.yml pull
 docker-compose -f /mnt/inf/foos/foos-compose.yml up -d
```

### Info on docker file

here is the source of how I got started with the docker file
https://github.com/emk/rust-musl-builder/blob/master/examples/using-diesel/Dockerfile
https://medium.com/@gruberbastian/rust-for-the-web-02-x-deploy-your-first-app-51d1ed69cbe3