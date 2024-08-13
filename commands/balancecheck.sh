#!/bin/bash

#Duration in seconds (20 minutes)

duration=$((20 * 60))

# End Time
start_time=$(date +%s)

# End Time

end_time=$((start_time + duration))

# Loop until the end time is reached

while [ $(date +%s) -lt $end_time ]; do
	#run the nomic balance command
	nomic balance
	#wait for 30 seconds befoe next iteration

	sleep 5
done
