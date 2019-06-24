# README #

Foosball table

## Dev env setup
- install rust (if you don't like this... sorry)
- install sqlite (good luck on windows)
- install diesel_cli
```bash
	cargo install diesel_cli --no-default-features --features sqlite
```
- run migrations 
```
	diesel migration run
```
- run the application
```
	cargo run --bin foosball_server
```

## Configuration for nginx as reverse proxy
Add the following snippet of config to the server

``` nginx
location /api {
                 proxy_pass http://localhost:12346;
                 proxy_set_header Upgrade $http_upgrade;
                 proxy_set_header Connection 'upgrade';
                 proxy_set_header Host $host;
                 proxy_cache_bypass $http_upgrade;
        }
```


### Features
- Camera behind dinging goal plate
- Speed meter
- Live camera stream
- Capacitive surface for trajectory marking
- Pressure sensitive Foospeople (measuring striking power)
- Speakers that announce things like what type of shot