#!/bin/sh

sh -c "./build_skript.sh"
CMD="target/debug/vertexC build testingCode/function_scope.vtx scopes.out -d"

while true
do
    clear

    sh -c "$CMD"
    EXIT_CODE=$?


    echo ""
    echo "Exit code: $EXIT_CODE"

    if [ "$EXIT_CODE" -eq 161 ]; then
        echo "Exit code -95 detected. Call instruction detected."
        break
    fi

    sleep 1
done
