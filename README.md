# melodic track mixing
**Melodic track mixing** is a library that provides tools for the track keys analysis and the melodic sort that follows the (Camelot Wheel Rules)[https://mixedinkey.com/camelot-wheel/].

## Installation
1. First, you will need to install [FFTW3](http://www.fftw.org/download.html):

* Fedora: `$ sudo dnf install cmake fftw-devel catch2-devel`
* Debian & Ubuntu: `$ sudo apt install cmake libfftw3-dev`
* Arch Linux: `$ sudo pacman -S cmake fftw catch2`
* MacOS (via [Homebrew](https://brew.sh/)): `$ brew install cmake fftw catch2`
* Windows: `> vcpkg install fftw3 catch2`

2. `git clone https://github.com/DobbiKov/melodic-track-mixing-lib`
3. `cd melodic-track-mixing-lib`
4. `git clone https://github.com/DobbiKov/libkeyfinder`
5. `cd libkeyfinder`

6. Remove old lib:
```sh
sudo rm /usr/local/lib/libkeyfinder*.dylib                                                                                                              ─╯
sudo rm -r /usr/local/include/keyfinder    # headers
sudo rm /usr/local/lib/pkgconfig/libkeyfinder.pc
sudo rm -r /usr/local/lib/cmake/KeyFinder  # CMake package files
```

7. Build the cpp lib:
```sh
cmake -B build \                                                                                                                                        ─╯
      -DCMAKE_BUILD_TYPE=Release \
      -DCMAKE_OSX_ARCHITECTURES=arm64 \ #or your architecture (arm64 if you're on arm (macbooks with m chips for example))
      -DBUILD_SHARED_LIBS=ON          # OFF → static libKeyFinder.a
cmake --build build --config Release
sudo cmake --install build
```

8. `cd ..`
9. `cargo build`
