
Build the cpp lib:
```sh
cmake -B build \                                                                                                                                        ─╯
      -DCMAKE_BUILD_TYPE=Release \
      -DCMAKE_OSX_ARCHITECTURES=arm64 \
      -DBUILD_SHARED_LIBS=ON          # OFF → static libKeyFinder.a
cmake --build build --config Release
sudo cmake --install build
```
