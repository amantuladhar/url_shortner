#!/bin/sh

# Create a log directory 
mkdir -p /home/ubuntu/applogs
touch /home/ubuntu/applogs/url_shortner.log

# Run the application
# export DATABASE_URL=sqlite:./test_123123123.db?mode=rwc
__DATABASE_URL__
/home/ubuntu/url_shortner/url_shortner >> /home/ubuntu/applogs/url_shortner.log 2>&1 &