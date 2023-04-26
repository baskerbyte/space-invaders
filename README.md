<h1 align="center" id="title">🚀 Space Invaders</h1>
<p align="center"><img  src="https://img.shields.io/github/repo-size/oluiss/space-invaders?style=flat-square" alt="shields"></p>
<p id="description">Terminal Invaders is a retro-style arcade game recreated in the terminal with Rust. Move your ship and shoot down the invading aliens to prevent them from reaching your planet!</p>

<h2>🎮 Gameplay</h2>
<p align="center">
<img src="https://i.imgur.com/7ymxtbp.gif" alt="project-gameplay" width="400" height="400/">
</p>

<h2>🕹 Controls</h2>
<p>Use the arrow keys to move your ship left (←) and right (→), and the space bar to shoot.</p>

<h2>📦 Dependencies</h2>
<h3>Linux</h3>
<p>
To compile applications written using `ruscii` in Linux, you must have the X11 development libraries installed.

If your system does not have them, you can use the following commands in the terminal to install them according to your specific Linux distribution.</p>

- In Ubuntu or Debian:
  ```sh
  sudo apt install libx11-dev
  ```
- In Fedora, RHEL, or CentOS:
  ```sh
  sudo dnf install xorg-x11-server-devel
  ```

### Windows and macOS

Windows and macOS have no special dependencies.

<h2>👾 Installation</h2>
<p>First, clone the repository:</p>

```sh
git clone https://github.com/oluiss/space-invaders.git
```
<p>Then, navigate to the project directory and run the following command:</p>

```sh
cargo run --release
```

<h2>🏅 Credits</h2>
<p>This game is a recreation of the classic arcade game <i>Space Invaders</i>, originally developed by Tomohiro Nishikado and released by Taito Corporation in 1978.</p>