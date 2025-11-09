

## 開発環境準備
-script-build` (exit status: 1)
  --- stdout
  cargo:rerun-if-env-changed=SYSTEMD_LIBS_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=SYSTEMD_LIBS_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_SYSTEMD_LIBS
  cargo:rerun-if-env-changed=SYSTEMD_LIBS
  cargo:rerun-if-env-changed=SYSTEMD_LIB_DIR_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=SYSTEMD_LIB_DIR_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_SYSTEMD_LIB_DIR
  cargo:rerun-if-env-changed=SYSTEMD_LIB_DIR
  cargo:rerun-if-env-changed=SYSTEMD_PKG_NAME_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=SYSTEMD_PKG_NAME_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_SYSTEMD_PKG_NAME
  cargo:rerun-if-env-changed=SYSTEMD_PKG_NAME
  cargo:rerun-if-env-changed=LIBSYSTEMD_NO_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG
  cargo:rerun-if-env-changed=LIBSYSTEMD_STATIC
  cargo:rerun-if-env-changed=LIBSYSTEMD_DYNAMIC
  cargo:rerun-if-env-changed=PKG_CONFIG_ALL_STATIC
  cargo:rerun-if-env-changed=PKG_CONFIG_ALL_DYNAMIC
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_SYSROOT_DIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR

  --- stderr
  pkg_config could not find "libsystemd":
  pkg-config exited with status code 1
  > PKG_CONFIG_ALLOW_SYSTEM_LIBS=1 PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1 pkg-config --libs --cflags libsystemd

  The system library `libsystemd` required by crate `libsystemd-sys` was not found.
  The file `libsystemd.pc` needs to be installed and the PKG_CONFIG_PATH environment variable must contain its parent directory.
  The PKG_CONFIG_PATH environment variable is not set.

  HINT: if you have installed the library, try setting PKG_CONFIG_PATH to the directory containing `libsystemd.pc`.

warning: build failed, waiting for other jobs to finish...

### 上記エラーが出た場合 lib_install.shで実行して再ビルド
```bash
./lib_install.sh
cargo r
```

## テスト実行方法

/stub配下にあるrun_test_echo.shを実行
```bash
./run_test_echo.sh
cargo r
```

使い終わったら止めること
```bash
sudo systemctl stop test_echo
```
