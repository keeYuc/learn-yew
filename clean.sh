for item in $(ls); do
    if [[ -d $item ]]; then
	cd $item&& cargo clean;
	cd ..;
    fi
done
