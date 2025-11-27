1. Pull the kafka image
```
  docker-compose pull
```
2. Start the kafka server
```
  docker exec -it kafka-broker /bin/bash
```
## ! cd to kafka bin directory
```
  cd /opt/kafka/bin
```
3. Create a topic
```
  ./kafka-topics.sh --create --topic test-topic --bootstrap-server localhost:9092 --partitions 1 --replication-factor 1
```

4. List the topics
```
  ./kafka-topics.sh --list --bootstrap-server localhost:9092
```

5. Produce messages to the topic
```
  ./kafka-console-producer.sh --topic test-topic --bootstrap-server localhost:9092
```
Now type messages in terminal 
   <br> > message 1
   <br> > message 2
   <br> > message 3


6. Consume messages from the topic
```
  ./kafka-console-consumer.sh --topic test-topic --bootstrap-server localhost:9092 --from-beginning
```


Now we can we same using rust code to produce and consume messages from the topic 
7. Produce messages using rust code
```
  cargo run --bin producer
```

8. Consume messages using rust code
```
  cargo run --bin consumer
```