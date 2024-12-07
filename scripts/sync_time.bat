@echo off

net session >nul 2>&1
if %errorlevel% neq 0 (
    echo This script must be run as Administrator!
    pause
    exit /b
)

echo Checking Windows Time service status...

sc query w32time | find "RUNNING" > nul
if %errorlevel%==0 (
    echo Windows Time service is running.
) else (
    echo Windows Time service is not running. Attempting to start it...
    net start w32time >nul 2>&1
    if %errorlevel%==0 (
        echo Windows Time service started successfully.
    ) else (
        echo Failed to start Windows Time service. Exiting...
        pause
        exit /b
    )
)

echo Synchronizing time...
w32tm /resync >nul 2>&1
if %errorlevel%==0 (
    echo Success!
) else (
    echo Error while time synchronization.
)

pause
