# cham

the name 'cham' is from 'Chameleon'  

I sometimes type `make run` in a Cargo project, or `cargo run` in a C++ Makefile project by mistake.  

`cham` changes to another command depending on `$HOME/.config/cham.conf` and (current and) parents's directories.  

example of `$HOME/.config/cham.conf`
```
Makefile => make
Cargo.toml => cargo
```

then, `cham run` changes to `cargo run` in Cargo projects, and `cham run` changes to `make run` in C++ Makefile projects.  

Happy!!!

# How to build

run `cargo build --release`

# Copyright
Copyright (C) 2017 akitsu sanae.  
Distributed under the Boost Software License, Version 1.0. 
(See accompanying file LICENSE_1_0.txt or copy at http://www.boost/org/LICENSE_1_0.txt)  

