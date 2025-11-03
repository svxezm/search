#!/bin/sh

read -p "Insert your search content: " CONTENT
xdg-open "https://duckduckgo.com/?q=$CONTENT"
exit
