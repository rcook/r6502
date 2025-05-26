@echo off
setlocal

set BEEBASM_PATH=%~dp0tools\beebasm.exe

for %%i in (%~dp0examples\*.asm) do (
    call :assemble %%i
)

goto :eof

:assemble
pushd "%~dp1"
"%BEEBASM_PATH%" -i "%~1"
popd
goto :eof
