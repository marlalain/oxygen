# Oxygen
# downloading and building dash
git clone git://git.kernel.org/pub/scm/utils/dash/dash.git dash
cd dash
sh autogen.sh
sh configure.sh --enable-static
make
# creating an enviroment for dash
mkdir -p ../oxygen_dash/bin
cp src/dash ../oxygen_dash/bin
cd ..
# now we download and install oxygen
git clone https://github.com/paulo-e/oxygen oxygen
cd oxygen
RUSTFLAGS='-C target-feature=+crt-static' cargo install --path . --root=../oxygen_dash/
cd ..
sudo chroot oxygen_dash /bin/dash

[![Rust](https://github.com/paulo-e/oxygen/actions/workflows/rust.yml/badge.svg)](https://github.com/paulo-e/oxygen/actions/workflows/rust.yml)

## Installation

Do as follows:

```sh
git clone https://github.com/paulo-e/oxygen oxygen
cd oxygen
cargo build --release
# now all the programs are in ./target/release
ls ./target/release
# you can also install the files (default path is $HOME/.cargo/bin) with
cargo install --path .
```

## Testing
Optionally, you can test the userspace on a controlled enviroment

``` sh
# downloading and building dash git clone git://git.kernel.org/pub/scm/utils/dash/dash.git dash
cd dash
sh autogen.sh
sh configure.sh --enable-static
make
# creating an enviroment for dash
mkdir -p ../oxygen_dash/bin
cp src/dash ../oxygen_dash/bin
cd ..
# now we download and install oxygen
git clone https://github.com/paulo-e/oxygen oxygen
cd oxygen
RUSTFLAGS='-C target-feature=+crt-static' cargo install --path . --root=../oxygen_dash/
cd ..
sudo chroot oxygen_dash /bin/dash
# now you can try all the installed programs
```

## License
[It's the BSD 3-clause license](https://github.com/paulo-e/oxygen/blob/master/LICENSE)
