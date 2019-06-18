#!/bin/bash
cd /home/foos/foosball/
filename=$(date +%s)
cp foosball.db dbbackups/$filename
