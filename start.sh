#!/bin/sh
set -eu

echo "Starting Xvfb"
Xvfb :99 -screen 0 1280x800x24 &
XVFB_PID=$!

echo "▶ Starting window manager"
openbox &
sleep 1

echo "Starting VNC server"
x11vnc -display :99 -nopw -forever -shared -rfbport 5900 &
VNC_PID=$!

echo "Starting noVNC"
websockify --web=/usr/share/novnc/ 6080 localhost:5900 &
NOVNC_PID=$!

echo "Starting application"
java -jar /app/app.jar &

wait

