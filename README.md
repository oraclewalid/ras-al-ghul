# Ras al ghul
KV database

#### Roadmap
- [x] Partial Redis protocol support
- [x] Config file and arguments parsing
- [x] Support of basic functions (INC, ADD ...)
- [x] Simple snapshot on disk
- [ ] TTL
- [ ] Benchmark (vs Redis)
    +  [x] Support redis-benchmark
    +  [ ] Performance tunning
- [ ] Docker image
    +  [ ] Simple docker image
    +  [ ] Build on docker
    +  [ ] Docker-compose with clustering support
- [ ] Monitoring
    +  [ ] logging and tracing
    +  [ ] Prometheus
- [ ] Persistence
    +  [ ] Rocksdb support as a backend
    +  [ ] Persistence via LSM tree
- [ ] SQL support
- [ ] Versioning
- [ ] RAFT
  + [ ] RAFT Leader election
  + [ ] RAFT replication
- [ ] Complex data structures 
  + [ ] List
  + [ ] Set
  + [ ] Map
  + [ ] HyperLogLog
  + [ ] Bloom filter