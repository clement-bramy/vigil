default:
    just --list

deb:
    @cargo deb

deb-install:
    @sudo dpkg -i target/release/vigil*.deb

vdebug-on:
    @sudo systemctl kill --kill-who=main --signal=USR1 vigil

vdebug-off:
    @sudo systemctl kill --kill-who=main --signal=USR2 vigil
