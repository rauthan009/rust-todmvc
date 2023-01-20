#DEV test
```
sh 
#Test for model
cargo watch -q -c -w src/ -x 'test model_ -- --test-threads=1 --nocapture'

# run for web
cargo watch -q -c -w src/ -x 'run -- ../frontend/web-folder'

# Test for web
cargo watch -q -c -w src/ -x 'test web_ -- --test-threads=1 --nocapture'
```
#DB
```
sh
docker run --rm -p 5432:5432 -e "POSTGRES_PASSWORD=postgres" --name pg postgres:14
docker exec -it -u postgres pg psql

```
