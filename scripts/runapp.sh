#!/bin/sh

# Create a log directory 
mkdir -p /home/ubuntu/applogs
touch /home/ubuntu/applogs/url_shortner.log

# Run the application
/home/ubuntu/url_shortner/url_shortner >> /home/ubuntu/applogs/url_shortner.log 2>&1 &