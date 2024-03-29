#!/bin/bash
container_name="redis-cluster" # Name of the container
start_port=7001                # Start port
end_port=7006                  # End port

# If the container already exists, remove it
if docker ps -a --format '{{.Names}}' | grep -q "^$container_name$"; then
    echo "Removing existing container: $container_name"
    docker rm -f $container_name
fi

# Start the Redis cluster
docker run \
    -dt \
    -e "IP=0.0.0.0" \
    -e "BIND_ADDRESS=0.0.0.0" \
    -e "INITIAL_PORT=$start_port" \
    -e "MASTERS=3" \
    -e "SLAVES_PER_MASTER=1" \
    -p $start_port-$end_port:$start_port-$end_port \
    --name $container_name \
    grokzen/redis-cluster:latest

# Wait for the container to be ready
sleep 10

# Run the command for each port in the range
for ((port = $start_port; port <= $end_port; port++)); do
    echo "Running command for Port $port:"
    docker exec $container_name sh -c "redis-cli -c -p $port CONFIG SET protected-mode no"
    sleep 3
done