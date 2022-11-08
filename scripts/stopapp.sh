#!/bin/sh

# Kill the existing app
ps aux | grep url_shortner | grep -v grep | awk '{print $2}' | xargs kill -9 || true