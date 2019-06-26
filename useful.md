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
```
docker build -t foosball:latest .
```

### Running through docker
```
docker run -v C:\dev\foosball\data:/data -v  C:\dev\foosball\templates:/templates -v C:\dev\foosball\static_files:/static_files -p 12346:12346 foosball:latest
```

### Docker tag foosball
```
docker tag foosball danwilkins8/foosball:0.1.1
```

### Docker push
```
docker push danwilkins8/foosball:0.1.1
```