if [ -d ../headers ]; then
    if [ -f ../headers/obs.h ]; then 
        find ../headers -type f ! \( -name "*.h" -o -name "*.hpp" \) -exec rm -rf {} \;
    else
        echo "could not find obs.h in directory!"
    fi
else
    echo "could not find headers directory!"
fi