if [ -d ../headers/obs ]; then
    if [ -f ../headers/obs/obs.h ]; then 
        find ../headers -type f ! \( -name "*.h" -o -name "*.hpp" \) -exec rm -rf {} \;
    else
        echo "could not find obs.h in directory!"
    fi
else
    echo "could not find headers directory!"
fi