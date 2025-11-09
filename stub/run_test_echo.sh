#!/bin/bash
sudo cp ./test_echo.sh /usr/local/bin/

if [ $1 == "-u" ]; then
    sudo cp -f test_echo.service /home/dev/.config/systemd/user/test_echo.service
    sudo systemctl --user daemon-reexec
    sudo systemctl --user daemon-reload
    sudo systemctl --user enable --now test_echo.service
else
    sudo cp -f test_echo.service /etc/systemd/system/test_echo.service
    sudo systemctl daemon-reexec
    sudo systemctl daemon-reload
    sudo systemctl enable --now test_echo.service
fi